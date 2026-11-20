use crate::activity::{AcpActivityProjection, text_content};
use crate::connection::AcpConnection;
use crate::diagnostics::{AcpDiagnostics, StopReasonPolicy, UsageProgressPolicy};
use serde_json::{Value, json};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, AtomicU64, Ordering},
};
use swallowtail_core::{CancellationScope, ProviderRequestRef, TurnRef};
use swallowtail_protocol_acp::{AcpMessageRole, AcpSessionUpdate};
use swallowtail_runtime::{
    BoxEventStream, BoxFuture, CallbackId, CancellationAcknowledgement, CancellationControl,
    CleanupOutcome, JoinedTask, OperationContent, ProviderObservation, ProviderRequestObservation,
    RuntimeEvent, RuntimeEventKind, RuntimeEventSender, RuntimeFailure, RuntimeTurnId,
    TerminalOutcome, TerminalOutcomeFuture, TerminalOutcomeSender, TerminalStatus, TurnHandle,
    runtime_event_channel, terminal_outcome_channel,
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
}

impl ActiveTurnConfig {
    /// Creates a standard turn configuration for an adapter.
    #[must_use]
    pub const fn new(diagnostics: AcpDiagnostics) -> Self {
        Self {
            diagnostics,
            stop_reason_policy: StopReasonPolicy::Standard,
            usage_progress_policy: UsageProgressPolicy::Silent,
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
}

/// Shared active turn state for standard ACP v1 adapters.
pub struct ActiveTurn {
    config: ActiveTurnConfig,
    runtime_id: RuntimeTurnId,
    session_id: String,
    events: RuntimeEventSender,
    terminal: TerminalOutcomeSender,
    sequence: AtomicU64,
    output: Mutex<String>,
    activity: Mutex<AcpActivityProjection>,
    provider_observation: Mutex<Option<ProviderRequestObservation>>,
    cancelled: AtomicBool,
    finished: AtomicBool,
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
        let (terminal, future) = terminal_outcome_channel();
        Ok((
            Arc::new(Self {
                config,
                runtime_id: runtime_id.clone(),
                session_id,
                events,
                terminal,
                sequence: AtomicU64::new(1),
                output: Mutex::new(String::new()),
                activity: Mutex::new(AcpActivityProjection::new(
                    runtime_id,
                    config.diagnostics.activity,
                )),
                provider_observation: Mutex::new(None),
                cancelled: AtomicBool::new(false),
                finished: AtomicBool::new(false),
            }),
            Box::pin(stream),
            future,
        ))
    }

    /// Returns whether the turn has been marked cancelled.
    #[must_use]
    pub fn was_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }

    fn finish(&self, status: TerminalStatus) {
        if self.finished.swap(true, Ordering::SeqCst) {
            return;
        }
        if let Ok(observations) = self
            .activity
            .lock()
            .expect("turn activity lock poisoned")
            .complete(&status)
        {
            for observation in observations {
                let _ = self.emit(RuntimeEventKind::Activity(observation), None);
            }
        }
        let output = self
            .output
            .lock()
            .expect("turn output lock poisoned")
            .clone();
        let mut outcome = TerminalOutcome::new(status, CleanupOutcome::NotApplicable);
        if let Ok(content) = OperationContent::new(output) {
            let _ = self.emit(RuntimeEventKind::OutputAvailable, Some(content.clone()));
            outcome = outcome.with_output(content);
        }
        self.events.mark_terminal();
        let _ = self.terminal.complete(outcome);
    }

    fn append_output(&self, text: &str) -> Result<(), RuntimeFailure> {
        let mut output = self.output.lock().expect("turn output lock poisoned");
        if output.len().saturating_add(text.len()) > MAXIMUM_OUTPUT_BYTES {
            return Err((self.config.diagnostics.output_limit_exceeded)());
        }
        output.push_str(text);
        Ok(())
    }

    fn emit_content(&self, kind: RuntimeEventKind, text: &str) -> Result<(), RuntimeFailure> {
        match OperationContent::new(text) {
            Ok(content) => self.emit(kind, Some(content)),
            Err(_) if text.trim().is_empty() => Ok(()),
            Err(_) => Err((self.config.diagnostics.malformed)()),
        }
    }

    fn emit(
        &self,
        kind: RuntimeEventKind,
        content: Option<OperationContent>,
    ) -> Result<(), RuntimeFailure> {
        let sequence = self.sequence.fetch_add(1, Ordering::SeqCst);
        let event = match content {
            Some(content) => RuntimeEvent::with_content(sequence, kind, content),
            None => RuntimeEvent::new(sequence, kind),
        };
        self.events.send(event)
    }

    fn verify_session(&self, session_id: &str) -> Result<(), RuntimeFailure> {
        if session_id == self.session_id {
            Ok(())
        } else {
            Err((self.config.diagnostics.session_mismatch_update)())
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
        let sequence = self.sequence.fetch_add(1, Ordering::SeqCst);
        let callback_id = CallbackId::new(format!(
            "{}:permission:{sequence}",
            self.runtime_id.as_str()
        ))
        .map_err(|_| malformed())?;
        let provider_request_ref = ProviderRequestRef::new(match provider_request_id {
            Value::String(value) => format!("acp:{value}"),
            Value::Number(value) => format!("acp:{value}"),
            _ => return Err(malformed()),
        })
        .map_err(|_| malformed())?;
        let observation = ProviderRequestObservation::new(
            callback_id,
            (self.config.diagnostics.permission_namespace)(),
            provider_request_ref.clone(),
        );
        *self
            .provider_observation
            .lock()
            .expect("permission observation lock poisoned") = Some(observation.clone());
        self.mark_cancelled();
        self.emit(
            RuntimeEventKind::ProviderObservation(ProviderObservation::RequestCorrelation(
                provider_request_ref,
            )),
            None,
        )?;
        Ok(observation)
    }

    fn handle_update(&self, params: &Value) -> Result<(), RuntimeFailure> {
        let malformed = self.config.diagnostics.malformed;
        let decoded =
            swallowtail_protocol_acp::decode_session_update(params).map_err(|_| malformed())?;
        self.verify_session(decoded.session_id.as_str())?;
        match &decoded.update {
            AcpSessionUpdate::Message(message) if message.role == AcpMessageRole::Agent => {
                let text = text_content(&message.content, malformed)?;
                self.append_output(text)?;
                self.emit_content(RuntimeEventKind::OutputDelta, text)?;
            }
            AcpSessionUpdate::Message(message) if message.role == AcpMessageRole::Thought => {
                self.emit_content(
                    RuntimeEventKind::ReasoningProgress,
                    text_content(&message.content, malformed)?,
                )?;
            }
            AcpSessionUpdate::CurrentMode(_)
            | AcpSessionUpdate::Message(_)
            | AcpSessionUpdate::Plan(_)
            | AcpSessionUpdate::AvailableCommands(_)
            | AcpSessionUpdate::ConfigOptions(_)
            | AcpSessionUpdate::SessionInfo { .. } => {
                self.emit(RuntimeEventKind::Progress, None)?;
            }
            AcpSessionUpdate::Usage(_) => {
                if self.config.usage_progress_policy == UsageProgressPolicy::EmitProgress {
                    self.emit(RuntimeEventKind::Progress, None)?;
                }
            }
            AcpSessionUpdate::ToolCall(_)
            | AcpSessionUpdate::ToolCallUpdate(_)
            | AcpSessionUpdate::Unknown { .. } => {}
        }
        let observations = self
            .activity
            .lock()
            .expect("turn activity lock poisoned")
            .project(&decoded.update)?;
        for observation in observations {
            self.emit(RuntimeEventKind::Activity(observation), None)?;
        }
        Ok(())
    }

    fn finish_prompt(&self, stop_reason: &str) {
        let status = if let Some(observation) = self
            .provider_observation
            .lock()
            .expect("permission observation lock poisoned")
            .clone()
        {
            TerminalStatus::ProviderRequestObserved(observation)
        } else {
            match stop_reason {
                "end_turn" => TerminalStatus::Completed,
                "cancelled" => TerminalStatus::Cancelled,
                "max_tokens" => {
                    TerminalStatus::ProviderFailed((self.config.diagnostics.prompt_stopped)())
                }
                "max_turn_requests" | "refusal"
                    if self.config.stop_reason_policy
                        == StopReasonPolicy::ExtendedProviderStops =>
                {
                    TerminalStatus::ProviderFailed((self.config.diagnostics.prompt_stopped)())
                }
                _ => TerminalStatus::RuntimeFailed(
                    (self.config.diagnostics.stop_reason_unsupported)()
                        .diagnostic()
                        .clone(),
                ),
            }
        };
        self.finish(status);
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
