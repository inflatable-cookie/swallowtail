//! Bounded correlated connection to one Claude Agent SDK Node sidecar.
//!
//! Commands are correlated by single-use id, responses match their command,
//! events reach the active turn, and admission callbacks reach the turn's
//! bounded exchange. Pending work is capped, so a stalled consumer cannot
//! make the sidecar an unbounded buffer.

use super::failure::{failure, protocol_failure};
use super::turn::SdkActiveTurn;
use super::wire::{
    ClaudeAgentSdkCommand, ClaudeAgentSdkToolDecision, encode_callback_response, encode_command,
};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    DebugObservationKind, HostServices, ProcessExit, ProcessHandle, ProcessInputChunk,
    RuntimeFailure,
};

mod pump;

const MAXIMUM_PENDING_COMMANDS: usize = 16;

pub(crate) struct CommandResult {
    pub(crate) success: bool,
    pub(crate) data: Option<Value>,
}

pub(crate) struct SdkConnection {
    process: Arc<dyn ProcessHandle>,
    services: HostServices,
    pending: Mutex<BTreeMap<String, PendingCommand>>,
    used_ids: Mutex<BTreeSet<String>>,
    active_turn: Mutex<Option<Arc<SdkActiveTurn>>>,
    closed: AtomicBool,
    terminal_error: Mutex<Option<SafeDiagnostic>>,
    exit: Mutex<Option<ProcessExit>>,
}

impl SdkConnection {
    pub(crate) fn new(process: Arc<dyn ProcessHandle>, services: HostServices) -> Arc<Self> {
        Arc::new(Self {
            process,
            services,
            pending: Mutex::new(BTreeMap::new()),
            used_ids: Mutex::new(BTreeSet::new()),
            active_turn: Mutex::new(None),
            closed: AtomicBool::new(false),
            terminal_error: Mutex::new(None),
            exit: Mutex::new(None),
        })
    }

    pub(crate) fn emit_protocol_debug(&self, error: &RuntimeFailure, stage: &'static str) {
        let diagnostic = error.diagnostic();
        self.services.emit_failure_debug(
            DebugObservationKind::ProtocolParse,
            "claude-agent.sdk",
            stage,
            diagnostic.code(),
            diagnostic.message(),
        );
    }

    /// Writes one correlated command and returns its pending response.
    ///
    /// The caller decides how the response is bounded. Nothing here awaits the
    /// provider, so no call site can accidentally inherit an unbounded wait.
    pub(crate) async fn send(
        &self,
        id: String,
        command: ClaudeAgentSdkCommand,
        params: Value,
    ) -> Result<ResponseFuture, RuntimeFailure> {
        if self.closed.load(Ordering::SeqCst) {
            return Err(self.closed_failure());
        }
        if !self
            .used_ids
            .lock()
            .expect("SDK sidecar command id lock poisoned")
            .insert(id.clone())
        {
            return Err(failure(
                "swallowtail.claude-agent.sdk.command_id_reused",
                "Claude Agent SDK sidecar command id was reused",
            ));
        }
        let (sender, response) = response_channel();
        {
            let mut pending = self
                .pending
                .lock()
                .expect("SDK sidecar pending lock poisoned");
            if pending.len() >= MAXIMUM_PENDING_COMMANDS {
                return Err(failure(
                    "swallowtail.claude-agent.sdk.command_capacity_exceeded",
                    "Claude Agent SDK sidecar exceeded the pending command limit",
                ));
            }
            pending.insert(id.clone(), PendingCommand { command, sender });
        }
        let bytes = encode_command(&id, command, params).map_err(|_| protocol_failure())?;
        if let Err(error) = self
            .process
            .write_stdin(ProcessInputChunk::new(bytes))
            .await
        {
            self.pending
                .lock()
                .expect("SDK sidecar pending lock poisoned")
                .remove(&id);
            return Err(error);
        }
        Ok(response)
    }

    /// Sends one command and awaits its response inside a caller-bounded
    /// context. Only close, which the shared cleanup bound already covers, and
    /// bounded races use this.
    pub(crate) async fn command(
        &self,
        id: String,
        command: ClaudeAgentSdkCommand,
        params: Value,
    ) -> Result<CommandResult, RuntimeFailure> {
        self.send(id, command, params).await?.await
    }

    pub(crate) async fn respond_admission(
        &self,
        sidecar_id: &str,
        decision: ClaudeAgentSdkToolDecision,
    ) -> Result<(), RuntimeFailure> {
        if self.closed.load(Ordering::SeqCst) {
            return Err(self.closed_failure());
        }
        let bytes =
            encode_callback_response(sidecar_id, decision).map_err(|_| protocol_failure())?;
        self.process
            .write_stdin(ProcessInputChunk::new(bytes))
            .await
    }

    pub(crate) fn set_active_turn(&self, turn: Arc<SdkActiveTurn>) -> Result<(), RuntimeFailure> {
        let mut active = self
            .active_turn
            .lock()
            .expect("SDK sidecar active lock poisoned");
        if active.as_ref().is_some_and(|active| !active.is_finished()) {
            return Err(failure(
                "swallowtail.claude-agent.sdk.turn_active",
                "Claude Agent SDK sidecar session already has an active turn",
            ));
        }
        *active = Some(turn);
        Ok(())
    }

    pub(crate) fn clear_active_turn(&self, turn: &Arc<SdkActiveTurn>) {
        let mut active = self
            .active_turn
            .lock()
            .expect("SDK sidecar active lock poisoned");
        if active
            .as_ref()
            .is_some_and(|current| Arc::ptr_eq(current, turn))
        {
            *active = None;
        }
    }

    /// Ends sidecar input and requests cooperative stop. This is not a join
    /// and is never evidence that any process exited.
    pub(crate) async fn begin_close(&self) {
        self.closed.store(true, Ordering::SeqCst);
        let _ = self.process.close_stdin().await;
        let _ = self.process.request_stop().await;
    }

    /// Escalates through the host's descendant-tree termination authority.
    pub(crate) async fn escalate(&self) -> Result<(), RuntimeFailure> {
        self.closed.store(true, Ordering::SeqCst);
        self.process.force_stop().await
    }

    /// Returns the sidecar root exit the pump joined, carrying the host's own
    /// owned-tree completion evidence. `None` means no exit was observed, which
    /// is never evidence that the process stopped.
    pub(crate) fn observed_exit(&self) -> Option<ProcessExit> {
        *self.exit.lock().expect("SDK sidecar exit lock poisoned")
    }

    /// Records the terminal transport failure before the closed flag so a
    /// later command observes the exact cause instead of a generic close.
    pub(crate) fn record_terminal_error(&self, error: &RuntimeFailure) {
        *self
            .terminal_error
            .lock()
            .expect("SDK sidecar terminal-error lock poisoned") = Some(error.diagnostic().clone());
    }

    fn closed_failure(&self) -> RuntimeFailure {
        match self
            .terminal_error
            .lock()
            .expect("SDK sidecar terminal-error lock poisoned")
            .clone()
        {
            Some(diagnostic) => RuntimeFailure::new(diagnostic),
            None => connection_closed(),
        }
    }
}

struct PendingCommand {
    command: ClaudeAgentSdkCommand,
    sender: ResponseSender,
}

#[derive(Default)]
struct ResponseState {
    result: Option<Result<CommandResult, RuntimeFailure>>,
    waiter: Option<Waker>,
}

struct ResponseSender(Arc<Mutex<ResponseState>>);
pub(crate) struct ResponseFuture(Arc<Mutex<ResponseState>>);

fn response_channel() -> (ResponseSender, ResponseFuture) {
    let state = Arc::new(Mutex::new(ResponseState::default()));
    (ResponseSender(Arc::clone(&state)), ResponseFuture(state))
}

impl ResponseSender {
    fn complete(self, result: Result<CommandResult, RuntimeFailure>) {
        let mut state = self.0.lock().expect("SDK sidecar response lock poisoned");
        state.result = Some(result);
        if let Some(waiter) = state.waiter.take() {
            waiter.wake();
        }
    }
}

impl Future for ResponseFuture {
    type Output = Result<CommandResult, RuntimeFailure>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.0.lock().expect("SDK sidecar response lock poisoned");
        if let Some(result) = state.result.take() {
            Poll::Ready(result)
        } else {
            state.waiter = Some(context.waker().clone());
            Poll::Pending
        }
    }
}

pub(crate) fn connection_closed() -> RuntimeFailure {
    failure(
        "swallowtail.claude-agent.sdk.connection_closed",
        "Claude Agent SDK sidecar connection is closed",
    )
}
