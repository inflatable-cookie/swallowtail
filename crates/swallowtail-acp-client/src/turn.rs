use crate::activity::{AcpActivityProjection, text_content};
use crate::connection::AcpConnection;
use crate::diagnostics::{AcpDiagnostics, StopReasonPolicy, UsageProgressPolicy};
use futures_core::Stream;
use serde_json::{Value, json};
use std::collections::VecDeque;
use std::pin::Pin;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, AtomicUsize, Ordering},
};
use std::task::{Context, Poll};
use swallowtail_core::{CancellationScope, ProviderRequestRef, SafeDiagnostic, TurnRef};
use swallowtail_protocol_acp::{AcpMessageRole, AcpSessionUpdate};
use swallowtail_runtime::{
    BoxEventStream, BoxFuture, CallbackId, CancellationAcknowledgement, CancellationControl,
    CleanupOutcome, JoinedTask, OperationContent, ProviderObservation, ProviderRequestObservation,
    RuntimeEvent, RuntimeEventKind, RuntimeEventSender, RuntimeEventStream, RuntimeFailure,
    RuntimeTurnId, TerminalOutcome, TerminalOutcomeFuture, TerminalOutcomeSender, TerminalStatus,
    TurnHandle, runtime_event_channel, terminal_outcome_channel,
};

const EVENT_CAPACITY: usize = 128;
const MAXIMUM_OUTPUT_BYTES: usize = 4 * 1024 * 1024;

/// Trait implemented by an active ACP turn so [`AcpConnection`] can route updates,
/// permission observations, and transport failures to either the shared [`ActiveTurn`]
/// or an adapter-specialized turn (e.g. interactive permission callbacks).
pub trait AcpTurnSink: Send + Sync {
    /// Returns the provider session identifier bound to this turn.
    fn session_id(&self) -> &str;

    /// Returns whether this turn has already reached a terminal state.
    fn is_finished(&self) -> bool;

    /// Marks the turn as cancelled.
    fn mark_cancelled(&self);

    /// Records a rejected/observed provider permission request on this turn.
    fn observe_permission(
        &self,
        provider_request_id: &Value,
    ) -> Result<ProviderRequestObservation, RuntimeFailure>;

    /// Records a rejected/observed provider permission request with its ACP `toolCallId`.
    ///
    /// Defaults to delegating to [`Self::observe_permission`].
    fn observe_permission_with_tool_call(
        &self,
        provider_request_id: &Value,
        _tool_call_id: &str,
    ) -> Result<ProviderRequestObservation, RuntimeFailure> {
        self.observe_permission(provider_request_id)
    }

    /// Handles one `session/update` notification parameter payload.
    fn handle_update(&self, params: &Value) -> Result<(), RuntimeFailure>;

    /// Completes the turn with an ACP `stopReason`.
    fn finish_prompt(&self, stop_reason: &str);

    /// Fails the turn due to a runtime or transport failure.
    fn fail(&self, error: &RuntimeFailure);
}

/// Configuration for [`ActiveTurn`].
#[derive(Clone, Copy, Debug)]
pub struct ActiveTurnConfig {
    /// Provider-scoped diagnostics and activity configuration.
    pub diagnostics: AcpDiagnostics,
    /// Policy for mapping ACP `stopReason` strings.
    pub stop_reason_policy: StopReasonPolicy,
    /// Policy for emitting progress events on `Usage` updates.
    pub usage_progress_policy: UsageProgressPolicy,
    /// Optional provider-scoped failure for non-text ACP message content.
    /// Defaults to `diagnostics.malformed` when unset.
    pub content_unsupported: Option<fn() -> RuntimeFailure>,
    /// Optional provider-scoped failure diagnostic for `"auth_required"` stop reasons.
    pub auth_required_stop: Option<fn() -> SafeDiagnostic>,
}

impl ActiveTurnConfig {
    /// Creates a standard turn configuration for an adapter.
    #[must_use]
    pub const fn new(diagnostics: AcpDiagnostics) -> Self {
        Self {
            diagnostics,
            stop_reason_policy: StopReasonPolicy::Standard,
            usage_progress_policy: UsageProgressPolicy::Silent,
            content_unsupported: None,
            auth_required_stop: None,
        }
    }

    /// Sets the stop-reason policy.
    #[must_use]
    pub const fn with_stop_reason_policy(mut self, policy: StopReasonPolicy) -> Self {
        self.stop_reason_policy = policy;
        self
    }

    /// Sets the usage-progress policy.
    #[must_use]
    pub const fn with_usage_progress_policy(mut self, policy: UsageProgressPolicy) -> Self {
        self.usage_progress_policy = policy;
        self
    }

    /// Sets the provider-scoped failure returned for non-text ACP message content.
    #[must_use]
    pub const fn with_content_unsupported(
        mut self,
        content_unsupported: fn() -> RuntimeFailure,
    ) -> Self {
        self.content_unsupported = Some(content_unsupported);
        self
    }

    /// Maps the `"auth_required"` ACP stop reason to `TerminalStatus::ProviderFailed`.
    #[must_use]
    pub const fn with_auth_required_stop(
        mut self,
        auth_required_stop: fn() -> SafeDiagnostic,
    ) -> Self {
        self.auth_required_stop = Some(auth_required_stop);
        self
    }
}

struct TrackedEventStream {
    inner: RuntimeEventStream,
    buffered: Arc<AtomicUsize>,
}

impl Stream for TrackedEventStream {
    type Item = Result<RuntimeEvent, RuntimeFailure>;

    fn poll_next(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.inner).poll_next(context) {
            Poll::Ready(Some(Ok(event))) => {
                self.buffered.fetch_sub(1, Ordering::SeqCst);
                Poll::Ready(Some(Ok(event)))
            }
            other => other,
        }
    }
}

enum PendingEvent {
    Reserved(u64),
    Event {
        event: Box<RuntimeEvent>,
        staged: bool,
    },
    Terminal(TerminalOutcome),
}

enum UpdateAdmissionError {
    OutputLimitExceeded,
    ActivityMalformed,
    SendFailed(RuntimeFailure),
}

fn internal_activity_malformed() -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(
        "swallowtail.acp.internal.activity_malformed",
        "Internal ACP activity projection sentinel",
    ))
}

fn event_buffer_overflow() -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(
        "swallowtail.event_buffer_rejected",
        "Event buffer cannot discard a semantic event",
    ))
}

struct TurnState {
    next_sequence: u64,
    terminal: bool,
    output: String,
    activity: AcpActivityProjection,
    provider_observation: Option<ProviderRequestObservation>,
    outbox: VecDeque<PendingEvent>,
    staged_events: usize,
    flushing: bool,
    send_error: Option<RuntimeFailure>,
}

impl TurnState {
    fn allocate_sequence(&mut self) -> u64 {
        let sequence = self.next_sequence;
        self.next_sequence = sequence + 1;
        sequence
    }

    fn push_event(&mut self, kind: RuntimeEventKind, content: Option<OperationContent>) {
        let sequence = self.allocate_sequence();
        let event = match content {
            Some(content) => RuntimeEvent::with_content(sequence, kind, content),
            None => RuntimeEvent::new(sequence, kind),
        };
        self.outbox.push_back(PendingEvent::Event {
            event: Box::new(event),
            staged: true,
        });
        self.staged_events += 1;
    }

    fn push_terminal_event(&mut self, kind: RuntimeEventKind, content: Option<OperationContent>) {
        let sequence = self.allocate_sequence();
        let event = match content {
            Some(content) => RuntimeEvent::with_content(sequence, kind, content),
            None => RuntimeEvent::new(sequence, kind),
        };
        self.outbox.push_back(PendingEvent::Event {
            event: Box::new(event),
            staged: false,
        });
    }

    fn remove_reservation(&mut self, sequence: u64) -> bool {
        if let Some(position) = self
            .outbox
            .iter()
            .position(|item| matches!(item, PendingEvent::Reserved(seq) if *seq == sequence))
        {
            self.outbox.remove(position);
            self.staged_events = self.staged_events.saturating_sub(1);
            true
        } else {
            false
        }
    }

    fn terminal_reserve(&self) -> usize {
        self.activity
            .open_count()
            .saturating_add(usize::from(!self.output.is_empty()))
    }

    fn check_non_terminal_capacity(
        &self,
        downstream_buffered: usize,
        incoming: usize,
        terminal_reserve: usize,
    ) -> Result<(), RuntimeFailure> {
        if let Some(error) = &self.send_error {
            return Err(error.clone());
        }
        let required = downstream_buffered
            .saturating_add(self.staged_events)
            .saturating_add(incoming)
            .saturating_add(terminal_reserve);
        if required > EVENT_CAPACITY {
            return Err(event_buffer_overflow());
        }
        Ok(())
    }

    fn finish_locked(&mut self, status: TerminalStatus) -> bool {
        if self.terminal {
            return false;
        }
        self.terminal = true;
        // Abandon any incomplete event-builder reservations so terminal
        // publication and committed events can drain immediately.
        let mut abandoned = 0_usize;
        self.outbox.retain(|item| {
            if matches!(item, PendingEvent::Reserved(_)) {
                abandoned += 1;
                false
            } else {
                true
            }
        });
        self.staged_events = self.staged_events.saturating_sub(abandoned);
        if let Ok(observations) = self.activity.complete(&status) {
            for observation in observations {
                self.push_terminal_event(RuntimeEventKind::Activity(observation), None);
            }
        }
        let output = self.output.clone();
        let mut outcome = TerminalOutcome::new(status, CleanupOutcome::NotApplicable);
        if let Ok(content) = OperationContent::new(output) {
            self.push_terminal_event(RuntimeEventKind::OutputAvailable, Some(content.clone()));
            outcome = outcome.with_output(content);
        }
        self.outbox.push_back(PendingEvent::Terminal(outcome));
        true
    }
}

struct FlushGuard<'a> {
    state: &'a Mutex<TurnState>,
    active: bool,
}

impl Drop for FlushGuard<'_> {
    fn drop(&mut self) {
        if self.active {
            let mut state = self
                .state
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            state.flushing = false;
        }
    }
}

struct ReservationGuard<'a> {
    turn: &'a ActiveTurn,
    sequence: u64,
    active: bool,
}

impl Drop for ReservationGuard<'_> {
    fn drop(&mut self) {
        if self.active {
            {
                let mut state = self
                    .turn
                    .state
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner);
                state.remove_reservation(self.sequence);
            }
            let _ = self.turn.flush_outbox();
        }
    }
}

/// Shared active turn state for standard ACP v1 adapters.
pub struct ActiveTurn {
    config: ActiveTurnConfig,
    runtime_id: RuntimeTurnId,
    session_id: String,
    events: RuntimeEventSender,
    buffered: Arc<AtomicUsize>,
    terminal: TerminalOutcomeSender,
    state: Mutex<TurnState>,
    cancelled: AtomicBool,
    finished: AtomicBool,
    #[cfg(test)]
    before_update_flush: Mutex<Option<Arc<dyn Fn() + Send + Sync>>>,
}

impl ActiveTurn {
    /// Creates a new active turn along with its runtime event stream and terminal outcome future.
    pub fn new(
        runtime_id: RuntimeTurnId,
        session_id: String,
        config: ActiveTurnConfig,
    ) -> Result<(Arc<Self>, BoxEventStream, TerminalOutcomeFuture), RuntimeFailure> {
        let (events, stream) = runtime_event_channel(EVENT_CAPACITY)?;
        events.send(RuntimeEvent::new(0, RuntimeEventKind::Started))?;
        let buffered = Arc::new(AtomicUsize::new(1));
        let tracked = TrackedEventStream {
            inner: stream,
            buffered: Arc::clone(&buffered),
        };
        let (terminal, future) = terminal_outcome_channel();
        // Use an internal sentinel factory inside `AcpActivityProjection` so
        // adapter-supplied diagnostic factories are never invoked while the
        // turn state mutex is held; `handle_update` maps failures outside the lock.
        let mut activity_config = config.diagnostics.activity;
        activity_config.malformed = internal_activity_malformed;
        Ok((
            Arc::new(Self {
                config,
                runtime_id: runtime_id.clone(),
                session_id,
                events,
                buffered,
                terminal,
                state: Mutex::new(TurnState {
                    next_sequence: 1,
                    terminal: false,
                    output: String::new(),
                    activity: AcpActivityProjection::new(runtime_id, activity_config),
                    provider_observation: None,
                    outbox: VecDeque::new(),
                    staged_events: 0,
                    flushing: false,
                    send_error: None,
                }),
                cancelled: AtomicBool::new(false),
                finished: AtomicBool::new(false),
                #[cfg(test)]
                before_update_flush: Mutex::new(None),
            }),
            Box::pin(tracked),
            future,
        ))
    }

    #[cfg(test)]
    pub(crate) fn set_before_update_flush(&self, hook: Option<Arc<dyn Fn() + Send + Sync>>) {
        *self
            .before_update_flush
            .lock()
            .expect("test update flush hook lock poisoned") = hook;
    }

    /// Returns whether the turn has been marked cancelled.
    #[must_use]
    pub fn was_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }

    /// Completes the turn with an explicit [`TerminalStatus`] if it has not already finished.
    pub fn finish_with_status(&self, status: TerminalStatus) {
        self.finish(status);
    }

    /// Allocates the next event sequence number, constructs a [`RuntimeEventKind`], and sends it.
    pub fn emit_event<F>(&self, build: F) -> Result<(), RuntimeFailure>
    where
        F: FnOnce(u64) -> Result<RuntimeEventKind, RuntimeFailure>,
    {
        let sequence = {
            let mut state = self.state.lock().expect("turn state lock poisoned");
            if state.terminal {
                return Ok(());
            }
            let downstream = self.buffered.load(Ordering::SeqCst);
            state.check_non_terminal_capacity(downstream, 1, state.terminal_reserve())?;
            let sequence = state.allocate_sequence();
            state.outbox.push_back(PendingEvent::Reserved(sequence));
            state.staged_events += 1;
            sequence
        };
        let mut guard = ReservationGuard {
            turn: self,
            sequence,
            active: true,
        };
        let built = build(sequence);
        guard.active = false;
        let result = match built {
            Ok(kind) => {
                let mut state = self.state.lock().expect("turn state lock poisoned");
                if let Some(position) = state.outbox.iter().position(
                    |item| matches!(item, PendingEvent::Reserved(seq) if *seq == sequence),
                ) {
                    state.outbox[position] = PendingEvent::Event {
                        event: Box::new(RuntimeEvent::new(sequence, kind)),
                        staged: true,
                    };
                }
                Ok(())
            }
            Err(error) => {
                let mut state = self.state.lock().expect("turn state lock poisoned");
                state.remove_reservation(sequence);
                Err(error)
            }
        };
        let flush_result = self.flush_outbox();
        result.and(flush_result)
    }

    fn flush_outbox(&self) -> Result<(), RuntimeFailure> {
        {
            let mut state = self.state.lock().expect("turn state lock poisoned");
            if state.flushing
                || matches!(state.outbox.front(), None | Some(PendingEvent::Reserved(_)))
            {
                return Ok(());
            }
            state.flushing = true;
        }
        let mut guard = FlushGuard {
            state: &self.state,
            active: true,
        };
        let mut first_error = None;
        loop {
            let next = {
                let mut state = self.state.lock().expect("turn state lock poisoned");
                match state.outbox.front() {
                    Some(PendingEvent::Event { staged, .. }) => {
                        if *staged {
                            state.staged_events = state.staged_events.saturating_sub(1);
                        }
                        self.buffered.fetch_add(1, Ordering::SeqCst);
                        state.outbox.pop_front()
                    }
                    Some(PendingEvent::Terminal(_)) => state.outbox.pop_front(),
                    None | Some(PendingEvent::Reserved(_)) => {
                        state.flushing = false;
                        guard.active = false;
                        None
                    }
                }
            };
            let Some(item) = next else {
                break;
            };
            match item {
                PendingEvent::Event { event, .. } => {
                    if let Err(error) = self.events.send(*event) {
                        self.buffered.fetch_sub(1, Ordering::SeqCst);
                        {
                            let mut state = self.state.lock().expect("turn state lock poisoned");
                            if state.send_error.is_none() {
                                state.send_error = Some(error.clone());
                            }
                        }
                        if first_error.is_none() {
                            first_error = Some(error);
                        }
                    }
                }
                PendingEvent::Terminal(outcome) => {
                    self.events.mark_terminal();
                    let _ = self.terminal.complete(outcome);
                }
                PendingEvent::Reserved(_) => unreachable!("reserved entries halt flushing above"),
            }
        }
        match first_error {
            Some(error) => Err(error),
            None => Ok(()),
        }
    }

    fn finish(&self, status: TerminalStatus) {
        let transitioned = {
            let mut state = self.state.lock().expect("turn state lock poisoned");
            let transitioned = state.finish_locked(status);
            if transitioned {
                self.finished.store(true, Ordering::SeqCst);
            }
            transitioned
        };
        if transitioned {
            let _ = self.flush_outbox();
        }
    }

    fn verify_session(&self, session_id: &str) -> Result<(), RuntimeFailure> {
        if session_id == self.session_id {
            Ok(())
        } else {
            Err((self.config.diagnostics.session_mismatch_update)())
        }
    }

    fn stop_reason_status(&self, stop_reason: &str) -> TerminalStatus {
        match stop_reason {
            "end_turn" => TerminalStatus::Completed,
            "cancelled" => TerminalStatus::Cancelled,
            "auth_required" if self.config.auth_required_stop.is_some() => {
                let diagnostic = self
                    .config
                    .auth_required_stop
                    .expect("auth_required_stop checked above");
                TerminalStatus::ProviderFailed(diagnostic())
            }
            "max_tokens" => {
                TerminalStatus::ProviderFailed((self.config.diagnostics.prompt_stopped)())
            }
            "max_turn_requests" | "refusal"
                if self.config.stop_reason_policy == StopReasonPolicy::ExtendedProviderStops =>
            {
                TerminalStatus::ProviderFailed((self.config.diagnostics.prompt_stopped)())
            }
            _ => TerminalStatus::RuntimeFailed(
                (self.config.diagnostics.stop_reason_unsupported)()
                    .diagnostic()
                    .clone(),
            ),
        }
    }
}

impl AcpTurnSink for ActiveTurn {
    fn session_id(&self) -> &str {
        &self.session_id
    }

    fn is_finished(&self) -> bool {
        self.finished.load(Ordering::SeqCst)
    }

    fn mark_cancelled(&self) {
        self.cancelled.store(true, Ordering::SeqCst);
    }

    fn observe_permission(
        &self,
        provider_request_id: &Value,
    ) -> Result<ProviderRequestObservation, RuntimeFailure> {
        let malformed = self.config.diagnostics.malformed;
        let provider_request_ref = ProviderRequestRef::new(match provider_request_id {
            Value::String(value) => format!("acp:{value}"),
            Value::Number(value) => format!("acp:{value}"),
            _ => return Err(malformed()),
        })
        .map_err(|_| malformed())?;
        let permission_namespace = (self.config.diagnostics.permission_namespace)();
        let observation = {
            let mut state = self.state.lock().expect("turn state lock poisoned");
            if !state.terminal {
                let downstream = self.buffered.load(Ordering::SeqCst);
                state.check_non_terminal_capacity(downstream, 1, state.terminal_reserve())?;
            }
            let sequence = state.allocate_sequence();
            let Ok(callback_id) = CallbackId::new(format!(
                "{}:permission:{sequence}",
                self.runtime_id.as_str()
            )) else {
                drop(state);
                return Err(malformed());
            };
            let observation = ProviderRequestObservation::new(
                callback_id,
                permission_namespace,
                provider_request_ref.clone(),
            );
            state.provider_observation = Some(observation.clone());
            self.mark_cancelled();
            if !state.terminal {
                state.push_event(
                    RuntimeEventKind::ProviderObservation(ProviderObservation::RequestCorrelation(
                        provider_request_ref,
                    )),
                    None,
                );
            }
            observation
        };
        self.flush_outbox()?;
        Ok(observation)
    }

    fn handle_update(&self, params: &Value) -> Result<(), RuntimeFailure> {
        let malformed = self.config.diagnostics.malformed;
        let content_unsupported = self.config.content_unsupported.unwrap_or(malformed);
        let decoded =
            swallowtail_protocol_acp::decode_session_update(params).map_err(|_| malformed())?;
        self.verify_session(decoded.session_id.as_str())?;

        // Extract and validate text/content before acquiring the turn state
        // mutex so adapter-supplied diagnostic factories never run under lock.
        let mut agent_text: Option<&str> = None;
        let primary_event: Option<(RuntimeEventKind, Option<OperationContent>)> =
            match &decoded.update {
                AcpSessionUpdate::Message(message) if message.role == AcpMessageRole::Agent => {
                    let text = text_content(&message.content, content_unsupported)?;
                    agent_text = Some(text);
                    match OperationContent::new(text) {
                        Ok(content) => Some((RuntimeEventKind::OutputDelta, Some(content))),
                        Err(_) if text.trim().is_empty() => None,
                        Err(_) => return Err(malformed()),
                    }
                }
                AcpSessionUpdate::Message(message) if message.role == AcpMessageRole::Thought => {
                    let text = text_content(&message.content, content_unsupported)?;
                    match OperationContent::new(text) {
                        Ok(content) => Some((RuntimeEventKind::ReasoningProgress, Some(content))),
                        Err(_) if text.trim().is_empty() => None,
                        Err(_) => return Err(malformed()),
                    }
                }
                AcpSessionUpdate::CurrentMode(_)
                | AcpSessionUpdate::Message(_)
                | AcpSessionUpdate::Plan(_)
                | AcpSessionUpdate::AvailableCommands(_)
                | AcpSessionUpdate::ConfigOptions(_)
                | AcpSessionUpdate::SessionInfo { .. } => Some((RuntimeEventKind::Progress, None)),
                AcpSessionUpdate::Usage(_) => {
                    if self.config.usage_progress_policy == UsageProgressPolicy::EmitProgress {
                        Some((RuntimeEventKind::Progress, None))
                    } else {
                        None
                    }
                }
                AcpSessionUpdate::ToolCall(_)
                | AcpSessionUpdate::ToolCallUpdate(_)
                | AcpSessionUpdate::Unknown { .. } => None,
            };

        let commit = {
            let mut state = self.state.lock().expect("turn state lock poisoned");
            if state.terminal {
                return Ok(());
            }
            if let Some(text) = agent_text
                && state.output.len().saturating_add(text.len()) > MAXIMUM_OUTPUT_BYTES
            {
                Err(UpdateAdmissionError::OutputLimitExceeded)
            } else {
                // Project onto a snapshot first so a rejected admission never
                // mutates `state.activity`.
                let mut next_activity = state.activity.snapshot();
                match next_activity.project(&decoded.update) {
                    Err(_) => Err(UpdateAdmissionError::ActivityMalformed),
                    Ok(observations) => {
                        let primary_count = usize::from(primary_event.is_some());
                        let total_incoming = primary_count.saturating_add(observations.len());
                        let next_has_output = !state.output.is_empty()
                            || agent_text.is_some_and(|text| !text.is_empty());
                        let next_terminal_reserve = next_activity
                            .open_count()
                            .saturating_add(usize::from(next_has_output));
                        let downstream = self.buffered.load(Ordering::SeqCst);
                        if let Err(error) = state.check_non_terminal_capacity(
                            downstream,
                            total_incoming,
                            next_terminal_reserve,
                        ) {
                            Err(UpdateAdmissionError::SendFailed(error))
                        } else {
                            state.activity = next_activity;
                            if let Some(text) = agent_text {
                                state.output.push_str(text);
                            }
                            if let Some((kind, content)) = primary_event {
                                state.push_event(kind, content);
                            }
                            for observation in observations {
                                state.push_event(RuntimeEventKind::Activity(observation), None);
                            }
                            Ok(())
                        }
                    }
                }
            }
        };
        match commit {
            Ok(()) => {}
            Err(UpdateAdmissionError::OutputLimitExceeded) => {
                return Err((self.config.diagnostics.output_limit_exceeded)());
            }
            Err(UpdateAdmissionError::ActivityMalformed) => {
                return Err((self.config.diagnostics.activity.malformed)());
            }
            Err(UpdateAdmissionError::SendFailed(error)) => return Err(error),
        }
        #[cfg(test)]
        {
            let hook = self
                .before_update_flush
                .lock()
                .expect("test update flush hook lock poisoned")
                .clone();
            if let Some(hook) = hook {
                hook();
            }
        }
        self.flush_outbox()
    }

    fn finish_prompt(&self, stop_reason: &str) {
        // Fast-path: if already terminal or a permission observation is already
        // recorded, commit immediately under the lock without invoking any
        // fallback stop-reason diagnostic factory.
        let needs_fallback = {
            let mut state = self.state.lock().expect("turn state lock poisoned");
            if state.terminal {
                return;
            }
            if let Some(observation) = state.provider_observation.clone() {
                let transitioned =
                    state.finish_locked(TerminalStatus::ProviderRequestObserved(observation));
                if transitioned {
                    self.finished.store(true, Ordering::SeqCst);
                }
                false
            } else {
                true
            }
        };
        if !needs_fallback {
            let _ = self.flush_outbox();
            return;
        }
        // Compute fallback stop status outside the turn state mutex so
        // adapter-supplied diagnostic factories cannot deadlock or poison it.
        let fallback_status = self.stop_reason_status(stop_reason);
        let transitioned = {
            let mut state = self.state.lock().expect("turn state lock poisoned");
            if state.terminal {
                false
            } else {
                let status = state
                    .provider_observation
                    .clone()
                    .map_or(fallback_status, TerminalStatus::ProviderRequestObserved);
                let transitioned = state.finish_locked(status);
                if transitioned {
                    self.finished.store(true, Ordering::SeqCst);
                }
                transitioned
            }
        };
        if transitioned {
            let _ = self.flush_outbox();
        }
    }

    fn fail(&self, error: &RuntimeFailure) {
        self.finish(TerminalStatus::RuntimeFailed(error.diagnostic().clone()));
    }
}

pub(crate) type ActiveSlot = Arc<Mutex<Option<ActiveTask>>>;

pub(crate) struct ActiveTask {
    pub(crate) turn: Arc<dyn AcpTurnSink>,
    pub(crate) task: Option<Box<dyn JoinedTask>>,
}

/// Cancellation controller for an active ACP turn.
pub struct TurnCancellation {
    connection: Arc<AcpConnection>,
    session_id: String,
    turn: Arc<dyn AcpTurnSink>,
    requested: AtomicBool,
}

impl TurnCancellation {
    /// Creates a new turn cancellation controller.
    #[must_use]
    pub fn new(
        connection: Arc<AcpConnection>,
        session_id: String,
        turn: Arc<dyn AcpTurnSink>,
    ) -> Self {
        Self {
            connection,
            session_id,
            turn,
            requested: AtomicBool::new(false),
        }
    }
}

impl CancellationControl for TurnCancellation {
    fn scope(&self) -> CancellationScope {
        CancellationScope::ActiveTurn
    }

    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        let already = self.requested.swap(true, Ordering::SeqCst);
        Box::pin(async move {
            if already {
                return Ok(CancellationAcknowledgement::AlreadyRequested);
            }
            self.turn.mark_cancelled();
            self.connection
                .notify("session/cancel", json!({"sessionId": self.session_id}))
                .await?;
            Ok(CancellationAcknowledgement::Requested)
        })
    }
}

/// Shared [`TurnHandle`] implementation for standard ACP sessions.
pub struct AcpTurnHandle {
    runtime_id: RuntimeTurnId,
    events: Option<BoxEventStream>,
    terminal: Option<BoxFuture<'static, TerminalOutcome>>,
    cancellation: TurnCancellation,
    active: ActiveSlot,
    diagnostics: AcpDiagnostics,
}

impl AcpTurnHandle {
    pub(crate) fn new(
        runtime_id: RuntimeTurnId,
        events: BoxEventStream,
        terminal: TerminalOutcomeFuture,
        cancellation: TurnCancellation,
        active: ActiveSlot,
        diagnostics: AcpDiagnostics,
    ) -> Self {
        Self {
            runtime_id,
            events: Some(events),
            terminal: Some(Box::pin(terminal)),
            cancellation,
            active,
            diagnostics,
        }
    }
}

impl TurnHandle for AcpTurnHandle {
    fn turn_id(&self) -> &RuntimeTurnId {
        &self.runtime_id
    }

    fn provider_turn_ref(&self) -> Option<&TurnRef> {
        None
    }

    fn take_events(&mut self) -> Option<BoxEventStream> {
        self.events.take()
    }

    fn cancellation(&self) -> &dyn CancellationControl {
        &self.cancellation
    }

    fn take_terminal_outcome(&mut self) -> Option<BoxFuture<'static, TerminalOutcome>> {
        self.terminal.take()
    }

    fn close(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome> {
        Box::pin(async move {
            if !self.cancellation.turn.is_finished() {
                let _ = self.cancellation.request().await;
                let _ = self.cancellation.connection.cancel_session().await;
            }
            let active = {
                let mut slot = self.active.lock().expect("ACP active-task lock poisoned");
                if slot
                    .as_ref()
                    .is_some_and(|active| Arc::ptr_eq(&active.turn, &self.cancellation.turn))
                {
                    slot.take()
                } else {
                    None
                }
            };
            match active.and_then(|mut active| active.task.take()) {
                Some(task) => match task.join().await {
                    Ok(()) => CleanupOutcome::NotApplicable,
                    Err(_) => CleanupOutcome::Failed((self.diagnostics.turn_join_failed)()),
                },
                None => CleanupOutcome::NotApplicable,
            }
        })
    }
}
