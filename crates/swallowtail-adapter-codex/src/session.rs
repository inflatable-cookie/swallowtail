use crate::rpc::{RpcConnection, failure};
use crate::session_access::CodexSessionAccess;
use crate::session_input::CodexSessionRuntime;
use crate::turn_state::{ActiveTurn, malformed_notification};
use futures_util::future::{Either, select};
use serde_json::Value;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use swallowtail_core::{CancellationScope, SessionRef, TurnRef};
use swallowtail_runtime::{
    BoxEventStream, BoxFuture, CallbackExchange, CancellationAcknowledgement, CancellationControl,
    CleanupOutcome, HostServices, InteractiveSessionHandle, JoinedTask, RequestId, RuntimeFailure,
    RuntimeSessionId, RuntimeTurnId, SessionResumeBinding, TerminalOutcome, TerminalStatus,
    TurnHandle, TurnRequest,
};

pub(crate) struct SessionCancellation {
    connection: Arc<RpcConnection>,
    requested: AtomicBool,
}

impl SessionCancellation {
    fn new(connection: Arc<RpcConnection>) -> Self {
        Self {
            connection,
            requested: AtomicBool::new(false),
        }
    }
}

impl CancellationControl for SessionCancellation {
    fn scope(&self) -> CancellationScope {
        CancellationScope::InteractiveSession
    }

    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        let already = self.requested.swap(true, Ordering::SeqCst);
        Box::pin(async move {
            if already {
                Ok(CancellationAcknowledgement::AlreadyRequested)
            } else {
                self.connection.cancel_session().await?;
                Ok(CancellationAcknowledgement::Requested)
            }
        })
    }
}

struct TurnCancellation {
    connection: Arc<RpcConnection>,
    thread_id: String,
    turn_id: String,
    turn: Arc<ActiveTurn>,
    requested: AtomicBool,
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
            let callbacks = self
                .connection
                .reject_abandoned_callbacks(self.turn.take_abandoned_provider_requests())
                .await;
            let interruption = self
                .connection
                .request(
                    "turn/interrupt",
                    serde_json::json!({
                        "threadId": self.thread_id,
                        "turnId": self.turn_id
                    }),
                )
                .await;
            callbacks?;
            interruption?;
            Ok(CancellationAcknowledgement::Requested)
        })
    }
}

pub(crate) struct CodexTurnHandle {
    runtime_id: RuntimeTurnId,
    provider_ref: TurnRef,
    events: Option<BoxEventStream>,
    callbacks: Option<CallbackExchange>,
    terminal: Option<BoxFuture<'static, TerminalOutcome>>,
    cancellation: TurnCancellation,
    deadline_task: Option<Box<dyn JoinedTask>>,
}

impl TurnHandle for CodexTurnHandle {
    fn turn_id(&self) -> &RuntimeTurnId {
        &self.runtime_id
    }

    fn provider_turn_ref(&self) -> Option<&TurnRef> {
        Some(&self.provider_ref)
    }

    fn take_events(&mut self) -> Option<BoxEventStream> {
        self.events.take()
    }

    fn take_callbacks(&mut self) -> Option<CallbackExchange> {
        self.callbacks.take()
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
            }
            if let Some(task) = self.deadline_task {
                match task.join().await {
                    Ok(()) => CleanupOutcome::NotApplicable,
                    Err(error) => CleanupOutcome::Failed(error.diagnostic().clone()),
                }
            } else {
                CleanupOutcome::NotApplicable
            }
        })
    }
}

pub(crate) struct CodexSessionHandle {
    request_id: RequestId,
    runtime_id: RuntimeSessionId,
    resume_binding: SessionResumeBinding,
    connection: Arc<RpcConnection>,
    cancellation: SessionCancellation,
    task: Box<dyn JoinedTask>,
    runtime: CodexSessionRuntime,
    access: CodexSessionAccess,
}

impl CodexSessionHandle {
    pub(crate) fn new(
        request_id: RequestId,
        runtime_id: RuntimeSessionId,
        resume_binding: SessionResumeBinding,
        connection: Arc<RpcConnection>,
        task: Box<dyn JoinedTask>,
        runtime: CodexSessionRuntime,
        access: CodexSessionAccess,
    ) -> Self {
        Self {
            request_id,
            runtime_id,
            resume_binding,
            cancellation: SessionCancellation::new(Arc::clone(&connection)),
            connection,
            task,
            runtime,
            access,
        }
    }
}

include!("session/interactive.rs");
