use super::activity::SidecarActivityProjection;
use super::failure::failure;
use super::wire::PiSdkSidecarEvent;
use crate::turn::TurnEvents;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use swallowtail_core::{
    FailureClassification, FailureKind, FailureOrigin, FailureRecovery, SafeDiagnostic,
};
use swallowtail_runtime::{
    ActivityStatus, BoxEventStream, CleanupOutcome, OperationContent, RuntimeEvent,
    RuntimeEventKind, RuntimeFailure, RuntimeTurnId, TerminalOutcome, TerminalOutcomeFuture,
    TerminalOutcomeSender, TerminalStatus, TokenUsage, terminal_outcome_channel,
};

const EVENT_CAPACITY: usize = 256;
const MAXIMUM_OUTPUT_BYTES: usize = 4 * 1024 * 1024;

#[derive(Default)]
struct FinishedState {
    finished: bool,
    waiter: Option<Waker>,
}

pub(crate) struct TurnFinishedFuture(Arc<Mutex<FinishedState>>);

impl Future for TurnFinishedFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.0.lock().expect("sidecar turn-finished lock poisoned");
        if state.finished {
            Poll::Ready(())
        } else {
            state.waiter = Some(context.waker().clone());
            Poll::Pending
        }
    }
}

#[derive(Default)]
struct AbortSettlementState {
    abort_in_flight: bool,
    abort_rejected: bool,
    cancelled: bool,
    pending_settled: bool,
}

#[cfg(test)]
type ReconcileProbeHook = Arc<dyn Fn(bool) + Send + Sync>;

pub(crate) struct SidecarActiveTurn {
    runtime_id: RuntimeTurnId,
    events: TurnEvents,
    terminal: TerminalOutcomeSender,
    sequence: AtomicU64,
    activity: Mutex<SidecarActivityProjection>,
    output: Mutex<String>,
    usage: Mutex<Option<TokenUsage>>,
    steering_scheduled: AtomicBool,
    follow_up_scheduled: AtomicBool,
    abort_state: Mutex<AbortSettlementState>,
    timed_out: AtomicBool,
    finished: AtomicBool,
    completed_prompts: Arc<AtomicU32>,
    finish_signal: Arc<Mutex<FinishedState>>,
    #[cfg(test)]
    after_abort_in_flight_check: Mutex<Option<Arc<dyn Fn() + Send + Sync>>>,
    #[cfg(test)]
    before_reconcile_lock: Mutex<Option<ReconcileProbeHook>>,
}

impl SidecarActiveTurn {
    pub(crate) fn new(
        runtime_id: RuntimeTurnId,
        completed_prompts: Arc<AtomicU32>,
    ) -> Result<(Arc<Self>, BoxEventStream, TerminalOutcomeFuture), RuntimeFailure> {
        let (events, stream) = TurnEvents::new(EVENT_CAPACITY)?;
        events.send(RuntimeEvent::new(0, RuntimeEventKind::Started))?;
        let (terminal, future) = terminal_outcome_channel();
        let activity = Mutex::new(SidecarActivityProjection::new(runtime_id.clone()));
        Ok((
            Arc::new(Self {
                runtime_id,
                events,
                terminal,
                sequence: AtomicU64::new(1),
                activity,
                output: Mutex::new(String::new()),
                usage: Mutex::new(None),
                steering_scheduled: AtomicBool::new(false),
                follow_up_scheduled: AtomicBool::new(false),
                abort_state: Mutex::new(AbortSettlementState::default()),
                timed_out: AtomicBool::new(false),
                finished: AtomicBool::new(false),
                completed_prompts,
                finish_signal: Arc::new(Mutex::new(FinishedState::default())),
                #[cfg(test)]
                after_abort_in_flight_check: Mutex::new(None),
                #[cfg(test)]
                before_reconcile_lock: Mutex::new(None),
            }),
            stream,
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
            .expect("sidecar abort-settlement lock poisoned")
            .pending_settled
    }

    pub(crate) fn handle_event(&self, event: PiSdkSidecarEvent) -> Result<(), RuntimeFailure> {
        if self.is_finished() || self.has_pending_settled() {
            return Err(failure(
                "swallowtail.pi.sdk-sidecar.event_after_terminal",
                "Pi SDK sidecar emitted an event after the active turn terminated",
            ));
        }
        if matches!(event, PiSdkSidecarEvent::ReplayItem { .. }) {
            return Err(failure(
                "swallowtail.pi.sdk-sidecar.replay_unexpected",
                "Pi SDK sidecar emitted replay evidence inside a fresh session",
            ));
        }
        if let PiSdkSidecarEvent::MessageEnded { stop_reason, .. } = &event
            && stop_reason == "error"
        {
            self.complete_activity(ActivityStatus::Failed)?;
            self.finish(TerminalStatus::ProviderFailed(
                SafeDiagnostic::new(
                    "swallowtail.pi.sdk-sidecar.provider_failed",
                    "Pi SDK sidecar reported a downstream provider failure",
                )
                .with_failure_classification(FailureClassification::new(
                    FailureOrigin::Provider,
                    FailureKind::Unknown,
                    FailureRecovery::Unknown,
                )),
            ));
            return Ok(());
        }
        self.project_activity(&event)?;
        match event {
            PiSdkSidecarEvent::Started | PiSdkSidecarEvent::Progress => self.progress(),
            PiSdkSidecarEvent::OutputDelta(delta) => self.output_delta(delta),
            PiSdkSidecarEvent::ReasoningDelta(delta) => {
                self.content_event(RuntimeEventKind::ReasoningProgress, delta)
            }
            PiSdkSidecarEvent::MessageEnded {
                usage: Some(usage), ..
            } => self.add_usage(usage),
            PiSdkSidecarEvent::TurnStarted
            | PiSdkSidecarEvent::TurnEnded
            | PiSdkSidecarEvent::Ended
            | PiSdkSidecarEvent::MessageStarted
            | PiSdkSidecarEvent::MessageEnded { usage: None, .. }
            | PiSdkSidecarEvent::ReasoningStarted
            | PiSdkSidecarEvent::ReasoningEnded
            | PiSdkSidecarEvent::ToolStarted { .. }
            | PiSdkSidecarEvent::ToolUpdated { .. }
            | PiSdkSidecarEvent::ToolEnded { .. } => Ok(()),
            PiSdkSidecarEvent::Settled => {
                {
                    let mut state = self
                        .abort_state
                        .lock()
                        .expect("sidecar abort-settlement lock poisoned");
                    if !self.timed_out.load(Ordering::SeqCst)
                        && !state.cancelled
                        && state.abort_in_flight
                    {
                        #[cfg(test)]
                        if let Some(hook) = self
                            .after_abort_in_flight_check
                            .lock()
                            .expect("sidecar test hook lock poisoned")
                            .clone()
                        {
                            hook();
                        }
                        state.pending_settled = true;
                        return Ok(());
                    }
                }
                self.finish_settled()
            }
            PiSdkSidecarEvent::ReplayItem { .. } => unreachable!("replay events fail closed above"),
        }
    }

    fn finish_settled(&self) -> Result<(), RuntimeFailure> {
        let usage = *self.usage.lock().expect("sidecar usage lock poisoned");
        let cancelled = self
            .abort_state
            .lock()
            .expect("sidecar abort-settlement lock poisoned")
            .cancelled;
        let (status, activity_status) = if self.timed_out.load(Ordering::SeqCst) {
            (TerminalStatus::TimedOut, ActivityStatus::Failed)
        } else if cancelled {
            (TerminalStatus::Cancelled, ActivityStatus::Cancelled)
        } else if usage.is_none() {
            (
                TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
                    "swallowtail.pi.sdk-sidecar.usage_missing",
                    "Pi SDK sidecar completed without required token usage",
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
            .expect("sidecar abort-settlement lock poisoned")
            .abort_in_flight = true;
    }

    pub(crate) fn mark_abort_acknowledged(&self) {
        let mut state = self
            .abort_state
            .lock()
            .expect("sidecar abort-settlement lock poisoned");
        state.cancelled = true;
        state.abort_in_flight = false;
        state.abort_rejected = false;
    }

    #[cfg(test)]
    pub(crate) fn mark_abort_rejected(&self) {
        let mut state = self
            .abort_state
            .lock()
            .expect("sidecar abort-settlement lock poisoned");
        state.abort_rejected = true;
        state.abort_in_flight = false;
    }

    pub(crate) fn reconcile_abort_response(&self, success: bool) -> Result<(), RuntimeFailure> {
        #[cfg(test)]
        if let Some(hook) = self
            .before_reconcile_lock
            .lock()
            .expect("sidecar test hook lock poisoned")
            .clone()
        {
            let contended = matches!(
                self.abort_state.try_lock(),
                Err(std::sync::TryLockError::WouldBlock)
            );
            hook(contended);
        }
        let settled = {
            let mut state = self
                .abort_state
                .lock()
                .expect("sidecar abort-settlement lock poisoned");
            if success {
                state.cancelled = true;
                state.abort_in_flight = false;
                state.abort_rejected = false;
            } else {
                state.abort_rejected = true;
                state.abort_in_flight = false;
            }
            std::mem::replace(&mut state.pending_settled, false)
        };
        if settled && let Err(error) = self.finish_settled() {
            self.finish(TerminalStatus::RuntimeFailed(error.diagnostic().clone()));
            return Err(error);
        }
        Ok(())
    }

    pub(crate) fn mark_cancelled(&self) {
        self.abort_state
            .lock()
            .expect("sidecar abort-settlement lock poisoned")
            .cancelled = true;
    }

    pub(crate) fn mark_timed_out(&self) {
        self.timed_out.store(true, Ordering::SeqCst);
    }

    pub(crate) fn fail_connection(&self, diagnostic: SafeDiagnostic) {
        let (cancelled, abort_in_flight, abort_rejected) = {
            let mut state = self
                .abort_state
                .lock()
                .expect("sidecar abort-settlement lock poisoned");
            state.pending_settled = false;
            (state.cancelled, state.abort_in_flight, state.abort_rejected)
        };
        let (status, activity_status) = if self.timed_out.load(Ordering::SeqCst) {
            (TerminalStatus::TimedOut, ActivityStatus::Failed)
        } else if cancelled {
            (TerminalStatus::Cancelled, ActivityStatus::Cancelled)
        } else if abort_in_flight || abort_rejected {
            (
                TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
                    "swallowtail.pi.sdk-sidecar.cancel_rejected",
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

    pub(crate) fn fail_rejected_abort(&self, diagnostic: SafeDiagnostic) {
        self.abort_state
            .lock()
            .expect("sidecar abort-settlement lock poisoned")
            .pending_settled = false;
        let status = if self.timed_out.load(Ordering::SeqCst) {
            TerminalStatus::TimedOut
        } else {
            TerminalStatus::RuntimeFailed(diagnostic)
        };
        self.finish_with_activity_cleanup(status, None, Some(ActivityStatus::Failed));
    }

    pub(crate) fn reserve_scheduling(&self, class: swallowtail_core::HarnessMessageClass) -> bool {
        match class {
            swallowtail_core::HarnessMessageClass::Steering => {
                !self.steering_scheduled.swap(true, Ordering::SeqCst)
            }
            swallowtail_core::HarnessMessageClass::FollowUp => {
                !self.follow_up_scheduled.swap(true, Ordering::SeqCst)
            }
            swallowtail_core::HarnessMessageClass::Prompt => false,
        }
    }

    pub(crate) fn release_scheduling(&self, class: swallowtail_core::HarnessMessageClass) {
        match class {
            swallowtail_core::HarnessMessageClass::Steering => {
                self.steering_scheduled.store(false, Ordering::SeqCst);
            }
            swallowtail_core::HarnessMessageClass::FollowUp => {
                self.follow_up_scheduled.store(false, Ordering::SeqCst);
            }
            swallowtail_core::HarnessMessageClass::Prompt => {}
        }
    }

    fn output_delta(&self, delta: String) -> Result<(), RuntimeFailure> {
        {
            let mut output = self.output.lock().expect("sidecar output lock poisoned");
            if output.len().saturating_add(delta.len()) > MAXIMUM_OUTPUT_BYTES {
                return Err(failure(
                    "swallowtail.pi.sdk-sidecar.output_limit_exceeded",
                    "Pi SDK sidecar output exceeded the adapter limit",
                ));
            }
            output.push_str(&delta);
        }
        self.content_event(RuntimeEventKind::OutputDelta, delta)
    }

    fn project_activity(&self, event: &PiSdkSidecarEvent) -> Result<(), RuntimeFailure> {
        let observations = self
            .activity
            .lock()
            .expect("sidecar activity lock poisoned")
            .project(event)?;
        for observation in observations {
            self.events.send(RuntimeEvent::new(
                self.next_sequence(),
                RuntimeEventKind::Activity(observation),
            ))?;
        }
        Ok(())
    }

    fn complete_activity(&self, status: ActivityStatus) -> Result<(), RuntimeFailure> {
        let observations = self
            .activity
            .lock()
            .expect("sidecar activity lock poisoned")
            .complete(status)?;
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
        let mut aggregate = self.usage.lock().expect("sidecar usage lock poisoned");
        *aggregate = Some(match *aggregate {
            Some(current) => current.checked_add_disjoint(usage).ok_or_else(|| {
                failure(
                    "swallowtail.pi.sdk-sidecar.usage_overflow",
                    "Pi SDK sidecar token usage exceeded the supported range",
                )
            })?,
            None => usage,
        });
        Ok(())
    }

    fn content_event(&self, kind: RuntimeEventKind, value: String) -> Result<(), RuntimeFailure> {
        let content = OperationContent::new(value).map_err(|_| {
            failure(
                "swallowtail.pi.sdk-sidecar.event_content_invalid",
                "Pi SDK sidecar event content was invalid",
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
            .expect("sidecar activity lock poisoned")
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
        if let Some(activity_status) = complete_activity {
            self.complete_activity_best_effort(activity_status);
        }
        let output = self
            .output
            .lock()
            .expect("sidecar output lock poisoned")
            .clone();
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
            .expect("sidecar turn-finished lock poisoned");
        signal.finished = true;
        if let Some(waiter) = signal.waiter.take() {
            waiter.wake();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::StreamExt;
    use swallowtail_runtime::ActivityLifecyclePhase;

    #[test]
    fn provider_error_fails_every_open_activity_before_terminal() {
        let (turn, mut events, terminal) = SidecarActiveTurn::new(
            RuntimeTurnId::new("sidecar-provider-error").expect("turn id is valid"),
            Arc::new(AtomicU32::new(0)),
        )
        .expect("sidecar turn constructs");
        for event in [
            PiSdkSidecarEvent::MessageStarted,
            PiSdkSidecarEvent::ReasoningStarted,
            PiSdkSidecarEvent::ToolStarted {
                call_id: "tool-1".to_owned(),
                name: "read".to_owned(),
            },
            PiSdkSidecarEvent::MessageEnded {
                stop_reason: "error".to_owned(),
                usage: None,
            },
        ] {
            turn.handle_event(event).expect("event is handled");
        }

        let failed_activities = futures_executor::block_on(async {
            let mut failed_activities = 0;
            while let Some(event) = events.next().await {
                let event = event.expect("event is valid");
                if let RuntimeEventKind::Activity(activity) = event.kind()
                    && activity.phase() == ActivityLifecyclePhase::Completed
                {
                    assert_eq!(activity.status(), ActivityStatus::Failed);
                    failed_activities += 1;
                }
            }
            failed_activities
        });
        assert_eq!(failed_activities, 3);
        assert!(matches!(
            futures_executor::block_on(terminal).status(),
            TerminalStatus::ProviderFailed(diagnostic)
                if diagnostic.code() == "swallowtail.pi.sdk-sidecar.provider_failed"
        ));
    }

    #[test]
    fn failed_activity_completion_does_not_consume_a_prompt_slot() {
        let completed_prompts = Arc::new(AtomicU32::new(0));
        let (turn, _events, _terminal) = SidecarActiveTurn::new(
            RuntimeTurnId::new("activity-overflow").expect("turn id is valid"),
            Arc::clone(&completed_prompts),
        )
        .expect("turn constructs");
        turn.handle_event(PiSdkSidecarEvent::ReasoningStarted)
            .expect("reasoning starts");
        for _ in 2..EVENT_CAPACITY {
            turn.progress().expect("progress fills the event buffer");
        }
        turn.add_usage(TokenUsage::new(Some(2), Some(1)))
            .expect("usage is valid");
        let error = turn
            .handle_event(PiSdkSidecarEvent::Settled)
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
            let (turn, events, terminal) = SidecarActiveTurn::new(
                RuntimeTurnId::new("terminal-overflow").expect("turn id is valid"),
                Arc::new(AtomicU32::new(0)),
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
            turn.handle_event(PiSdkSidecarEvent::Settled)
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
    fn eof_after_session_cancellation_or_acknowledged_retry_reports_cancelled() {
        for acknowledged_retry in [false, true] {
            let (turn, _events, terminal) = SidecarActiveTurn::new(
                RuntimeTurnId::new("sidecar-cancel-eof").expect("turn id is valid"),
                Arc::new(AtomicU32::new(0)),
            )
            .expect("sidecar turn constructs");
            turn.mark_abort_rejected();
            turn.mark_abort_in_flight();
            if acknowledged_retry {
                turn.mark_abort_acknowledged();
                assert!(
                    !turn
                        .abort_state
                        .lock()
                        .expect("sidecar abort-settlement lock poisoned")
                        .abort_rejected,
                    "retry clears the previous rejection"
                );
            } else {
                turn.mark_cancelled();
            }
            turn.fail_connection(SafeDiagnostic::new("test.eof", "sidecar disconnected"));
            assert_eq!(
                futures_executor::block_on(terminal).status(),
                &TerminalStatus::Cancelled
            );
        }
    }

    #[test]
    fn sidecar_fail_connection_closes_open_tool_and_reasoning_activities_before_terminal() {
        let (turn, mut events, terminal) = SidecarActiveTurn::new(
            RuntimeTurnId::new("sidecar-disconnect-activities").expect("turn id is valid"),
            Arc::new(AtomicU32::new(0)),
        )
        .expect("sidecar turn constructs");
        turn.handle_event(PiSdkSidecarEvent::ReasoningStarted)
            .expect("reasoning starts");
        turn.handle_event(PiSdkSidecarEvent::ToolStarted {
            call_id: "tool-1".to_owned(),
            name: "bash".to_owned(),
        })
        .expect("tool starts");
        turn.fail_connection(SafeDiagnostic::new(
            "swallowtail.pi.sdk-sidecar.disconnected",
            "Pi SDK sidecar disconnected",
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
            "both open sidecar reasoning and tool activities must close as Failed before terminal"
        );
        let outcome = futures_executor::block_on(terminal);
        assert!(matches!(
            outcome.status(),
            TerminalStatus::RuntimeFailed(diagnostic)
                if diagnostic.code() == "swallowtail.pi.sdk-sidecar.disconnected"
        ));
    }

    #[test]
    fn sidecar_settled_without_usage_fails_open_activities_and_does_not_increment_completed_prompts()
     {
        let completed_prompts = Arc::new(AtomicU32::new(0));
        let (turn, mut events, terminal) = SidecarActiveTurn::new(
            RuntimeTurnId::new("sidecar-usage-missing").expect("turn id is valid"),
            Arc::clone(&completed_prompts),
        )
        .expect("sidecar turn constructs");
        turn.handle_event(PiSdkSidecarEvent::ReasoningStarted)
            .expect("reasoning starts");
        turn.handle_event(PiSdkSidecarEvent::ToolStarted {
            call_id: "tool-1".to_owned(),
            name: "read".to_owned(),
        })
        .expect("tool starts");
        turn.handle_event(PiSdkSidecarEvent::Settled)
            .expect("settled handles usage_missing");

        assert_eq!(
            completed_prompts.load(Ordering::SeqCst),
            0,
            "sidecar usage_missing must not consume a completed prompt slot"
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
            "open sidecar activities must not close as Completed on usage_missing"
        );
        assert_eq!(
            completed_failed, 2,
            "open sidecar reasoning and tool activities must close as Failed on usage_missing"
        );
        let outcome = futures_executor::block_on(terminal);
        assert!(matches!(
            outcome.status(),
            TerminalStatus::RuntimeFailed(diagnostic)
                if diagnostic.code() == "swallowtail.pi.sdk-sidecar.usage_missing"
        ));
    }

    #[test]
    fn sidecar_settled_and_reconcile_abort_response_share_mutex_and_never_strand_turn() {
        use futures_util::FutureExt;
        use std::sync::mpsc;
        use std::time::Duration;

        // Pause `handle_event(Settled)` between checking `abort_in_flight` and
        // storing `pending_settled`, then run `reconcile_abort_response`. The
        // reconciliation probe runs `try_lock()` on `abort_state` while
        // settlement is still paused and only releases the settlement gate
        // after recording contention (or after reconciliation completes if
        // uncontended). Any timeout is strictly a failure watchdog.
        const WATCHDOG: Duration = Duration::from_secs(5);
        for success in [true, false] {
            let (turn, _events, terminal) = SidecarActiveTurn::new(
                RuntimeTurnId::new(format!("sidecar-toctou-gated-{success}"))
                    .expect("turn id is valid"),
                Arc::new(AtomicU32::new(0)),
            )
            .expect("sidecar turn constructs");
            turn.add_usage(TokenUsage::new(Some(3), Some(2)))
                .expect("usage recorded");
            turn.mark_abort_in_flight();

            let (checked_tx, checked_rx) = mpsc::channel();
            let (release_settled_tx, release_settled_rx) = mpsc::channel();
            let probe_contended = Arc::new(AtomicBool::new(false));
            let probe_contended_hook = Arc::clone(&probe_contended);
            let release_settled_rx = Mutex::new(release_settled_rx);
            let release_settled_hook = Mutex::new(Some(release_settled_tx.clone()));

            *turn
                .after_abort_in_flight_check
                .lock()
                .expect("sidecar test hook lock poisoned") = Some(Arc::new(move || {
                checked_tx.send(()).expect("signal checked");
                release_settled_rx
                    .lock()
                    .expect("release rx lock")
                    .recv_timeout(WATCHDOG)
                    .expect("watchdog: reconcile probe did not release settlement gate");
            }));

            *turn
                .before_reconcile_lock
                .lock()
                .expect("sidecar test hook lock poisoned") = Some(Arc::new(move |contended| {
                probe_contended_hook.store(contended, Ordering::SeqCst);
                if contended && let Some(tx) = release_settled_hook.lock().expect("tx lock").take()
                {
                    let _ = tx.send(());
                }
            }));

            let settled_turn = Arc::clone(&turn);
            let settled_thread = std::thread::spawn(move || {
                settled_turn
                    .handle_event(PiSdkSidecarEvent::Settled)
                    .expect("settled succeeds");
            });

            checked_rx
                .recv_timeout(WATCHDOG)
                .expect("watchdog: settled did not reach abort_in_flight check");
            turn.reconcile_abort_response(success)
                .expect("reconcile succeeds");
            // If a broken split-lock implementation allowed `try_lock()` to
            // succeed uncontended, `reconcile_abort_response` has already
            // finished without draining `pending_settled`; releasing settlement
            // now lets it write `pending_settled` late and strand the turn.
            let _ = release_settled_tx.send(());

            settled_thread.join().expect("settled thread joins");

            assert!(
                probe_contended.load(Ordering::SeqCst),
                "reconcile_abort_response must observe abort_state lock contention while handle_event(Settled) is between check and store (success={success})"
            );
            assert!(
                turn.is_finished(),
                "sidecar turn must not be stranded after gated interleaving (success={success})"
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
