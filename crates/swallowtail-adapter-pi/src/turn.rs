use crate::activity::PiActivityProjection;
use crate::callback::CallbackHub;
use crate::connection::PiConnection;
use crate::failure::failure;
use crate::protocol::PiAgentEvent;
use std::collections::BTreeSet;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, Weak};
use std::task::{Context, Poll, Waker};
use swallowtail_runtime::{
    ActivityObservation, ActivityStatus, BoxEventStream, CallbackAbandonment, CallbackExchange,
    CleanupOutcome, OperationContent, RuntimeEvent, RuntimeEventKind, RuntimeFailure,
    RuntimeTurnId, TerminalOutcome, TerminalOutcomeFuture, TerminalOutcomeSender, TerminalStatus,
    TokenUsage, terminal_outcome_channel,
};

mod events;
mod scheduling;
mod ui;

pub(crate) use events::TurnEvents;
pub(crate) use ui::CallbackTimer;

const EVENT_CAPACITY: usize = 256;
const MAXIMUM_OUTPUT_BYTES: usize = 4 * 1024 * 1024;
const MAXIMUM_DIALOG_BYTES: usize = 16 * 1024;
const MAXIMUM_DIALOG_OPTIONS: usize = 32;

#[derive(Default)]
struct FinishedState {
    finished: bool,
    waiter: Option<Waker>,
}

pub(crate) struct TurnFinishedFuture(Arc<Mutex<FinishedState>>);

impl Future for TurnFinishedFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.0.lock().expect("Pi turn-finished lock poisoned");
        if state.finished {
            Poll::Ready(())
        } else {
            state.waiter = Some(context.waker().clone());
            Poll::Pending
        }
    }
}

#[derive(Clone, Copy)]
struct PendingSettled {
    aborted: Option<bool>,
}

#[derive(Default)]
struct AbortSettlementState {
    abort_in_flight: bool,
    abort_rejected: bool,
    cancelled: bool,
    pending_settled: Option<PendingSettled>,
}

pub(crate) struct ActiveTurn {
    runtime_id: RuntimeTurnId,
    events: TurnEvents,
    terminal: TerminalOutcomeSender,
    callbacks: CallbackHub,
    sequence: AtomicU64,
    activity: Mutex<PiActivityProjection>,
    output: Mutex<String>,
    usage: Mutex<Option<TokenUsage>>,
    ui_ids: Mutex<BTreeSet<String>>,
    steering_scheduled: AtomicBool,
    follow_up_scheduled: AtomicBool,
    abort_state: Mutex<AbortSettlementState>,
    timed_out: AtomicBool,
    require_aborted_settled: bool,
    finished: AtomicBool,
    completed_prompts: Arc<AtomicU32>,
    finish_signal: Arc<Mutex<FinishedState>>,
}

impl ActiveTurn {
    pub(crate) fn new(
        runtime_id: RuntimeTurnId,
        completed_prompts: Arc<AtomicU32>,
        connection: Weak<PiConnection>,
        require_aborted_settled: bool,
    ) -> Result<
        (
            Arc<Self>,
            BoxEventStream,
            CallbackExchange,
            TerminalOutcomeFuture,
        ),
        RuntimeFailure,
    > {
        let (events, stream) = TurnEvents::new(EVENT_CAPACITY)?;
        events.send(RuntimeEvent::new(0, RuntimeEventKind::Started))?;
        let (terminal, future) = terminal_outcome_channel();
        let (callbacks, exchange) = CallbackHub::new(connection);
        let activity = Mutex::new(PiActivityProjection::new(runtime_id.clone()));
        Ok((
            Arc::new(Self {
                runtime_id,
                events,
                terminal,
                callbacks,
                sequence: AtomicU64::new(1),
                activity,
                output: Mutex::new(String::new()),
                usage: Mutex::new(None),
                ui_ids: Mutex::new(BTreeSet::new()),
                steering_scheduled: AtomicBool::new(false),
                follow_up_scheduled: AtomicBool::new(false),
                abort_state: Mutex::new(AbortSettlementState::default()),
                timed_out: AtomicBool::new(false),
                require_aborted_settled,
                finished: AtomicBool::new(false),
                completed_prompts,
                finish_signal: Arc::new(Mutex::new(FinishedState::default())),
            }),
            stream,
            exchange,
            future,
        ))
    }

    pub(crate) const fn runtime_id(&self) -> &RuntimeTurnId {
        &self.runtime_id
    }

    pub(crate) fn finished_future(&self) -> TurnFinishedFuture {
        TurnFinishedFuture(Arc::clone(&self.finish_signal))
    }

    pub(crate) fn is_finished(&self) -> bool {
        self.finished.load(Ordering::SeqCst)
    }

    fn has_pending_settled(&self) -> bool {
        self.abort_state
            .lock()
            .expect("Pi abort-settlement lock poisoned")
            .pending_settled
            .is_some()
    }

    pub(crate) fn handle_event(&self, event: PiAgentEvent) -> Result<(), RuntimeFailure> {
        if self.is_finished() || self.has_pending_settled() {
            return Err(failure(
                "swallowtail.pi.rpc.event_after_terminal",
                "Pi RPC emitted an event after the active turn terminated",
            ));
        }
        self.project_activity(&event)?;
        match event {
            PiAgentEvent::Started | PiAgentEvent::Progress => self.progress(),
            PiAgentEvent::OutputDelta(delta) => self.output_delta(delta),
            PiAgentEvent::ReasoningDelta(delta) => {
                self.content_event(RuntimeEventKind::ReasoningProgress, delta)
            }
            PiAgentEvent::MessageEnded(Some(usage)) => self.add_usage(usage),
            PiAgentEvent::MessageEnded(None)
            | PiAgentEvent::MessageStarted
            | PiAgentEvent::ReasoningStarted
            | PiAgentEvent::ReasoningEnded
            | PiAgentEvent::ToolStarted { .. }
            | PiAgentEvent::ToolUpdated { .. }
            | PiAgentEvent::ToolEnded { .. }
            | PiAgentEvent::CompactionStarted
            | PiAgentEvent::CompactionEnded
            | PiAgentEvent::Unknown(_) => Ok(()),
            PiAgentEvent::ProviderFailed => {
                self.complete_activity(ActivityStatus::Failed)?;
                self.finish(TerminalStatus::ProviderFailed(
                    swallowtail_core::SafeDiagnostic::new(
                        "swallowtail.pi.rpc.provider_failed",
                        "Pi RPC reported a downstream provider failure",
                    )
                    .with_failure_classification(
                        swallowtail_core::FailureClassification::new(
                            swallowtail_core::FailureOrigin::Provider,
                            swallowtail_core::FailureKind::Unknown,
                            swallowtail_core::FailureRecovery::Unknown,
                        ),
                    ),
                ));
                Ok(())
            }
            PiAgentEvent::RetryObserved => {
                self.complete_activity(ActivityStatus::Failed)?;
                self.finish(TerminalStatus::RuntimeFailed(
                    swallowtail_core::SafeDiagnostic::new(
                        "swallowtail.pi.rpc.retry_policy_drift",
                        "Pi RPC retried despite the disabled retry policy",
                    )
                    .with_failure_classification(
                        swallowtail_core::FailureClassification::new(
                            swallowtail_core::FailureOrigin::Harness,
                            swallowtail_core::FailureKind::ProtocolIncompatible,
                            swallowtail_core::FailureRecovery::HarnessUpdateRequired,
                        ),
                    ),
                ));
                Ok(())
            }
            PiAgentEvent::Settled { aborted } => {
                if self.require_aborted_settled && aborted.is_none() {
                    return Err(failure(
                        "swallowtail.pi.rpc.settled_aborted_missing",
                        "Pi RPC omitted the qualified agent-settled abort state",
                    ));
                }
                {
                    let mut state = self
                        .abort_state
                        .lock()
                        .expect("Pi abort-settlement lock poisoned");
                    if !self.timed_out.load(Ordering::SeqCst)
                        && !state.cancelled
                        && !aborted.unwrap_or(false)
                        && state.abort_in_flight
                    {
                        state.pending_settled = Some(PendingSettled { aborted });
                        return Ok(());
                    }
                }
                self.finish_settled(aborted)
            }
        }
    }

    fn finish_settled(&self, aborted: Option<bool>) -> Result<(), RuntimeFailure> {
        let usage = *self.usage.lock().expect("Pi usage lock poisoned");
        let cancelled = self
            .abort_state
            .lock()
            .expect("Pi abort-settlement lock poisoned")
            .cancelled;
        let (status, activity_status) = if self.timed_out.load(Ordering::SeqCst) {
            (TerminalStatus::TimedOut, ActivityStatus::Failed)
        } else if cancelled || aborted.unwrap_or(false) {
            (TerminalStatus::Cancelled, ActivityStatus::Cancelled)
        } else if usage.is_none() {
            (
                TerminalStatus::RuntimeFailed(swallowtail_core::SafeDiagnostic::new(
                    "swallowtail.pi.rpc.usage_missing",
                    "Pi RPC completed without required token usage",
                )),
                ActivityStatus::Failed,
            )
        } else {
            (TerminalStatus::Completed, ActivityStatus::Completed)
        };
        self.complete_activity(activity_status)?;
        if matches!(status, TerminalStatus::Completed) {
            self.completed_prompts.fetch_add(1, Ordering::SeqCst);
        }
        self.finish_with_usage(status, usage);
        Ok(())
    }

    pub(crate) fn mark_abort_in_flight(&self) {
        self.abort_state
            .lock()
            .expect("Pi abort-settlement lock poisoned")
            .abort_in_flight = true;
        self.callbacks.abandon(CallbackAbandonment::TurnCancelled);
    }

    pub(crate) fn mark_abort_acknowledged(&self) {
        let mut state = self
            .abort_state
            .lock()
            .expect("Pi abort-settlement lock poisoned");
        state.cancelled = true;
        state.abort_in_flight = false;
        state.abort_rejected = false;
    }

    pub(crate) fn reconcile_abort_response(&self, success: bool) -> Result<(), RuntimeFailure> {
        let pending = {
            let mut state = self
                .abort_state
                .lock()
                .expect("Pi abort-settlement lock poisoned");
            if success {
                state.cancelled = true;
                state.abort_in_flight = false;
                state.abort_rejected = false;
            } else {
                state.abort_rejected = true;
                state.abort_in_flight = false;
            }
            state.pending_settled.take()
        };
        if let Some(pending) = pending
            && let Err(error) = self.finish_settled(pending.aborted)
        {
            self.finish(TerminalStatus::RuntimeFailed(error.diagnostic().clone()));
            return Err(error);
        }
        Ok(())
    }

    pub(crate) fn mark_cancelled(&self) {
        self.abort_state
            .lock()
            .expect("Pi abort-settlement lock poisoned")
            .cancelled = true;
        self.callbacks.abandon(CallbackAbandonment::TurnCancelled);
    }

    pub(crate) fn mark_timed_out(&self) {
        self.timed_out.store(true, Ordering::SeqCst);
        self.callbacks.abandon(CallbackAbandonment::TimedOut);
    }

    pub(crate) fn fail_connection(&self, diagnostic: swallowtail_core::SafeDiagnostic) {
        let (cancelled, abort_in_flight, abort_rejected) = {
            let mut state = self
                .abort_state
                .lock()
                .expect("Pi abort-settlement lock poisoned");
            state.pending_settled = None;
            (state.cancelled, state.abort_in_flight, state.abort_rejected)
        };
        let (status, activity_status) = if self.timed_out.load(Ordering::SeqCst) {
            (TerminalStatus::TimedOut, ActivityStatus::Failed)
        } else if cancelled {
            (TerminalStatus::Cancelled, ActivityStatus::Cancelled)
        } else if abort_in_flight || abort_rejected {
            (
                TerminalStatus::RuntimeFailed(swallowtail_core::SafeDiagnostic::new(
                    "swallowtail.pi.rpc.cancel_rejected",
                    "Pi could not abort the active turn",
                )),
                ActivityStatus::Failed,
            )
        } else {
            (
                TerminalStatus::RuntimeFailed(diagnostic),
                ActivityStatus::Failed,
            )
        };
        self.finish_with_activity_cleanup(status, None, Some(activity_status));
    }

    pub(crate) fn fail_rejected_abort(&self, diagnostic: swallowtail_core::SafeDiagnostic) {
        self.abort_state
            .lock()
            .expect("Pi abort-settlement lock poisoned")
            .pending_settled = None;
        let status = if self.timed_out.load(Ordering::SeqCst) {
            TerminalStatus::TimedOut
        } else {
            TerminalStatus::RuntimeFailed(diagnostic)
        };
        self.finish_with_activity_cleanup(status, None, Some(ActivityStatus::Failed));
    }

    fn output_delta(&self, delta: String) -> Result<(), RuntimeFailure> {
        {
            let mut output = self.output.lock().expect("Pi output lock poisoned");
            if output.len().saturating_add(delta.len()) > MAXIMUM_OUTPUT_BYTES {
                return Err(failure(
                    "swallowtail.pi.rpc.output_limit_exceeded",
                    "Pi RPC output exceeded the adapter limit",
                ));
            }
            output.push_str(&delta);
        }
        self.content_event(RuntimeEventKind::OutputDelta, delta)
    }

    fn project_activity(&self, event: &PiAgentEvent) -> Result<(), RuntimeFailure> {
        let observations = self
            .activity
            .lock()
            .expect("Pi activity lock poisoned")
            .project(event)?;
        self.emit_activity(observations)
    }

    fn complete_activity(&self, status: ActivityStatus) -> Result<(), RuntimeFailure> {
        let observations = self
            .activity
            .lock()
            .expect("Pi activity lock poisoned")
            .complete(status)?;
        self.emit_activity(observations)
    }

    fn emit_activity(
        &self,
        observations: impl IntoIterator<Item = ActivityObservation>,
    ) -> Result<(), RuntimeFailure> {
        for observation in observations {
            self.events.send(RuntimeEvent::new(
                self.next_sequence(),
                RuntimeEventKind::Activity(observation),
            ))?;
        }
        Ok(())
    }

    fn progress(&self) -> Result<(), RuntimeFailure> {
        let sequence = self.sequence.fetch_add(1, Ordering::SeqCst);
        self.events
            .send(RuntimeEvent::new(sequence, RuntimeEventKind::Progress))
    }

    fn add_usage(&self, usage: TokenUsage) -> Result<(), RuntimeFailure> {
        let mut aggregate = self.usage.lock().expect("Pi usage lock poisoned");
        *aggregate = Some(match *aggregate {
            Some(current) => current.checked_add_disjoint(usage).ok_or_else(|| {
                failure(
                    "swallowtail.pi.rpc.usage_overflow",
                    "Pi RPC token usage exceeded the supported range",
                )
            })?,
            None => usage,
        });
        Ok(())
    }

    fn content_event(&self, kind: RuntimeEventKind, value: String) -> Result<(), RuntimeFailure> {
        let content = OperationContent::new(value).map_err(|_| {
            failure(
                "swallowtail.pi.rpc.event_content_invalid",
                "Pi RPC event content was invalid",
            )
        })?;
        let sequence = self.next_sequence();
        self.events
            .send(RuntimeEvent::with_content(sequence, kind, content))
    }

    fn next_sequence(&self) -> u64 {
        self.sequence.fetch_add(1, Ordering::SeqCst)
    }

    fn complete_activity_best_effort(&self, status: ActivityStatus) {
        let Ok(observations) = self
            .activity
            .lock()
            .expect("Pi activity lock poisoned")
            .complete(status)
        else {
            return;
        };
        for observation in observations {
            if self
                .events
                .send(RuntimeEvent::new(
                    self.next_sequence(),
                    RuntimeEventKind::Activity(observation),
                ))
                .is_err()
            {
                break;
            }
        }
    }

    fn finish(&self, status: TerminalStatus) {
        self.finish_with_activity_cleanup(status, None, None);
    }

    fn finish_with_usage(&self, status: TerminalStatus, usage: Option<TokenUsage>) {
        self.finish_with_activity_cleanup(status, usage, None);
    }

    fn finish_with_activity_cleanup(
        &self,
        status: TerminalStatus,
        usage: Option<TokenUsage>,
        complete_activity: Option<ActivityStatus>,
    ) {
        if self.finished.swap(true, Ordering::SeqCst) {
            return;
        }
        self.callbacks.abandon(CallbackAbandonment::TurnTerminated);
        if let Some(activity_status) = complete_activity {
            self.complete_activity_best_effort(activity_status);
        }
        let output = self.output.lock().expect("Pi output lock poisoned").clone();
        let output = OperationContent::new(output).ok();
        let status = self
            .events
            .finish(status, output.as_ref(), usage, || self.next_sequence());
        let mut outcome = TerminalOutcome::new(status, CleanupOutcome::NotApplicable);
        if let Some(output) = output {
            outcome = outcome.with_output(output);
        }
        let _ = self.terminal.complete(outcome);
        let mut signal = self
            .finish_signal
            .lock()
            .expect("Pi turn-finished lock poisoned");
        signal.finished = true;
        if let Some(waiter) = signal.waiter.take() {
            waiter.wake();
        }
    }
}

fn malformed_ui_sequence() -> RuntimeFailure {
    failure(
        "swallowtail.pi.rpc.ui_request_invalid",
        "Pi RPC extension UI request was invalid",
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Weak;
    use std::sync::atomic::AtomicU32;

    #[test]
    fn failed_activity_completion_does_not_consume_a_prompt_slot() {
        let completed_prompts = Arc::new(AtomicU32::new(0));
        let (turn, _events, _callbacks, _terminal) = ActiveTurn::new(
            RuntimeTurnId::new("activity-overflow").expect("turn id is valid"),
            Arc::clone(&completed_prompts),
            Weak::new(),
            false,
        )
        .expect("turn constructs");
        turn.handle_event(PiAgentEvent::ReasoningStarted)
            .expect("reasoning starts");
        for _ in 2..EVENT_CAPACITY {
            turn.progress().expect("progress fills the event buffer");
        }
        turn.add_usage(TokenUsage::new(Some(2), Some(1)))
            .expect("usage is valid");
        let error = turn
            .handle_event(PiAgentEvent::Settled {
                aborted: Some(false),
            })
            .expect_err("activity completion overflows");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.event_buffer_rejected"
        );
        assert_eq!(completed_prompts.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn terminal_event_overflow_fails_turn_with_attached_consumer() {
        for (buffered_output, consumer_attached) in [(true, true), (false, true), (true, false)] {
            let (turn, events, _callbacks, terminal) = ActiveTurn::new(
                RuntimeTurnId::new("terminal-overflow").expect("turn id is valid"),
                Arc::new(AtomicU32::new(0)),
                Weak::new(),
                false,
            )
            .expect("turn constructs");
            if buffered_output {
                turn.output_delta("output".to_owned())
                    .expect("output is valid");
            }
            for _ in (1 + usize::from(buffered_output))..EVENT_CAPACITY {
                turn.progress().expect("progress fills the event buffer");
            }
            turn.add_usage(TokenUsage::new(Some(2), Some(1)))
                .expect("usage is valid");
            let consumer = if consumer_attached {
                Some(events)
            } else {
                drop(events);
                None
            };
            turn.handle_event(PiAgentEvent::Settled {
                aborted: Some(false),
            })
            .expect("settled publishes a terminal outcome");
            let outcome = futures_executor::block_on(terminal);
            if let Some(mut events) = consumer {
                assert!(
                    matches!(
                        outcome.status(),
                        TerminalStatus::RuntimeFailed(diagnostic)
                            if diagnostic.code() == "swallowtail.event_buffer_rejected"
                    ),
                    "output present: {buffered_output}, outcome: {outcome:?}"
                );
                let stream_error = futures_executor::block_on(async {
                    use futures_util::StreamExt;
                    while let Some(item) = events.next().await {
                        if let Err(error) = item {
                            return error;
                        }
                    }
                    panic!("the attached consumer must observe the overflow");
                });
                assert_eq!(
                    stream_error.diagnostic().code(),
                    "swallowtail.event_buffer_rejected"
                );
            } else {
                assert_eq!(outcome.status(), &TerminalStatus::Completed);
            }
        }
    }

    #[test]
    fn content_event_reports_event_content_invalid_on_empty_content() {
        let (turn, _events, _callbacks, _terminal) = ActiveTurn::new(
            RuntimeTurnId::new("pi-content-test").expect("turn id is valid"),
            Arc::new(AtomicU32::new(0)),
            Weak::new(),
            false,
        )
        .expect("active turn constructs");
        let error = turn
            .content_event(RuntimeEventKind::OutputDelta, String::new())
            .expect_err("empty content must be rejected");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.pi.rpc.event_content_invalid"
        );
    }

    #[test]
    fn fail_connection_closes_open_tool_and_reasoning_activities_before_terminal() {
        use futures_util::StreamExt;
        use swallowtail_runtime::ActivityLifecyclePhase;

        let (turn, mut events, _callbacks, terminal) = ActiveTurn::new(
            RuntimeTurnId::new("pi-disconnect-activities").expect("turn id is valid"),
            Arc::new(AtomicU32::new(0)),
            Weak::new(),
            false,
        )
        .expect("active turn constructs");
        turn.handle_event(PiAgentEvent::ReasoningStarted)
            .expect("reasoning starts");
        turn.handle_event(PiAgentEvent::ToolStarted {
            call_id: "tool-1".to_owned(),
            name: "bash".to_owned(),
        })
        .expect("tool starts");
        turn.fail_connection(swallowtail_core::SafeDiagnostic::new(
            "swallowtail.pi.rpc.disconnected",
            "Pi RPC disconnected",
        ));

        let emitted: Vec<_> = futures_executor::block_on(async move {
            let mut items = Vec::new();
            while let Some(item) = events.next().await {
                items.push(item.expect("event is valid"));
            }
            items
        });
        let completed_failed = emitted
            .iter()
            .filter_map(|event| match event.kind() {
                RuntimeEventKind::Activity(observation)
                    if observation.phase() == ActivityLifecyclePhase::Completed
                        && observation.status() == ActivityStatus::Failed =>
                {
                    Some(observation)
                }
                _ => None,
            })
            .count();
        assert_eq!(
            completed_failed, 2,
            "both open reasoning and tool activities must close as Failed before terminal"
        );
        let outcome = futures_executor::block_on(terminal);
        assert!(matches!(
            outcome.status(),
            TerminalStatus::RuntimeFailed(diagnostic)
                if diagnostic.code() == "swallowtail.pi.rpc.disconnected"
        ));
    }

    #[test]
    fn settled_without_usage_fails_open_activities_and_does_not_increment_completed_prompts() {
        use futures_util::StreamExt;
        use swallowtail_runtime::ActivityLifecyclePhase;

        let completed_prompts = Arc::new(AtomicU32::new(0));
        let (turn, mut events, _callbacks, terminal) = ActiveTurn::new(
            RuntimeTurnId::new("pi-usage-missing").expect("turn id is valid"),
            Arc::clone(&completed_prompts),
            Weak::new(),
            false,
        )
        .expect("active turn constructs");
        turn.handle_event(PiAgentEvent::ReasoningStarted)
            .expect("reasoning starts");
        turn.handle_event(PiAgentEvent::ToolStarted {
            call_id: "tool-1".to_owned(),
            name: "bash".to_owned(),
        })
        .expect("tool starts");
        turn.handle_event(PiAgentEvent::Settled {
            aborted: Some(false),
        })
        .expect("settled handles usage_missing");

        assert_eq!(
            completed_prompts.load(Ordering::SeqCst),
            0,
            "usage_missing must not consume a completed prompt slot"
        );
        let emitted: Vec<_> = futures_executor::block_on(async move {
            let mut items = Vec::new();
            while let Some(item) = events.next().await {
                items.push(item.expect("event is valid"));
            }
            items
        });
        let completed_ok = emitted
            .iter()
            .filter(|event| {
                matches!(
                    event.kind(),
                    RuntimeEventKind::Activity(observation)
                        if observation.phase() == ActivityLifecyclePhase::Completed
                            && observation.status() == ActivityStatus::Completed
                )
            })
            .count();
        let completed_failed = emitted
            .iter()
            .filter(|event| {
                matches!(
                    event.kind(),
                    RuntimeEventKind::Activity(observation)
                        if observation.phase() == ActivityLifecyclePhase::Completed
                            && observation.status() == ActivityStatus::Failed
                )
            })
            .count();
        assert_eq!(
            completed_ok, 0,
            "open activities must not close as Completed on usage_missing"
        );
        assert_eq!(
            completed_failed, 2,
            "open reasoning and tool activities must close as Failed on usage_missing"
        );
        let outcome = futures_executor::block_on(terminal);
        assert!(matches!(
            outcome.status(),
            TerminalStatus::RuntimeFailed(diagnostic)
                if diagnostic.code() == "swallowtail.pi.rpc.usage_missing"
        ));
    }

    #[test]
    fn settled_and_reconcile_abort_response_share_mutex_and_never_strand_turn() {
        use futures_util::FutureExt;
        use std::sync::Barrier;

        // 1. Deterministic lock-serialization proof: while `abort_state` is held
        // during an in-flight abort transition, a concurrent `Settled` event
        // cannot observe stale `abort_in_flight = true` and strand `pending_settled`.
        {
            let (turn, _events, _callbacks, terminal) = ActiveTurn::new(
                RuntimeTurnId::new("pi-toctou-lock").expect("turn id is valid"),
                Arc::new(AtomicU32::new(0)),
                Weak::new(),
                false,
            )
            .expect("turn constructs");
            turn.mark_abort_in_flight();

            let mut guard = turn
                .abort_state
                .lock()
                .expect("Pi abort-settlement lock poisoned");
            let entered = Arc::new(Barrier::new(2));
            let settled_turn = Arc::clone(&turn);
            let entered_worker = Arc::clone(&entered);
            let settled_thread = std::thread::spawn(move || {
                entered_worker.wait();
                settled_turn
                    .handle_event(PiAgentEvent::Settled {
                        aborted: Some(false),
                    })
                    .expect("settled handles cleanly");
            });
            entered.wait();
            // Simulate `reconcile_abort_response(true)` completing under the lock.
            guard.cancelled = true;
            guard.abort_in_flight = false;
            guard.abort_rejected = false;
            assert!(guard.pending_settled.is_none());
            drop(guard);

            settled_thread.join().expect("settled thread joins");
            assert!(turn.is_finished(), "turn must not be stranded");
            assert!(!turn.has_pending_settled());
            let outcome = terminal
                .now_or_never()
                .expect("terminal outcome must already be ready");
            assert_eq!(outcome.status(), &TerminalStatus::Cancelled);
        }

        // 2. Concurrent race between `handle_event(Settled)` and
        // `reconcile_abort_response` for both `success = true` and `false`.
        for success in [true, false] {
            for iteration in 0..64 {
                let (turn, _events, _callbacks, terminal) = ActiveTurn::new(
                    RuntimeTurnId::new(format!("pi-toctou-{success}-{iteration}"))
                        .expect("turn id is valid"),
                    Arc::new(AtomicU32::new(0)),
                    Weak::new(),
                    false,
                )
                .expect("turn constructs");
                turn.add_usage(TokenUsage::new(Some(3), Some(2)))
                    .expect("usage recorded");
                turn.mark_abort_in_flight();

                let barrier = Arc::new(Barrier::new(2));
                let t1 = Arc::clone(&turn);
                let b1 = Arc::clone(&barrier);
                let h1 = std::thread::spawn(move || {
                    b1.wait();
                    t1.handle_event(PiAgentEvent::Settled {
                        aborted: Some(false),
                    })
                    .expect("settled succeeds");
                });
                let t2 = Arc::clone(&turn);
                let b2 = Arc::clone(&barrier);
                let h2 = std::thread::spawn(move || {
                    b2.wait();
                    t2.reconcile_abort_response(success)
                        .expect("reconcile succeeds");
                });
                h1.join().expect("settled thread joins");
                h2.join().expect("reconcile thread joins");

                assert!(
                    turn.is_finished(),
                    "turn stranded on success={success}, iteration={iteration}"
                );
                assert!(!turn.has_pending_settled());
                let outcome = terminal
                    .now_or_never()
                    .expect("terminal outcome must be ready without stranding");
                let expected = if success {
                    TerminalStatus::Cancelled
                } else {
                    TerminalStatus::Completed
                };
                assert_eq!(outcome.status(), &expected);
            }
        }
    }
}
