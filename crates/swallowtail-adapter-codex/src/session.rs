use crate::process_close::AppServerTask;
use crate::registered_tools::CodexRegisteredTurn;
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
    CleanupOutcome, HostServices, InteractiveSessionHandle, JoinedTask, RegisteredToolCleanupCause,
    RequestId, RuntimeFailure, RuntimeSessionId, RuntimeTurnId, SessionResumeBinding,
    TerminalOutcome, TerminalStatus, TurnHandle, TurnRequest,
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
        Box::pin(async move {
            if self.requested.swap(true, Ordering::SeqCst) {
                return Ok(CancellationAcknowledgement::AlreadyRequested);
            }
            if let Err(error) = self.connection.cancel_session().await {
                self.requested.store(false, Ordering::SeqCst);
                return Err(error);
            }
            Ok(CancellationAcknowledgement::Requested)
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

impl TurnCancellation {
    /// Marks the turn cancelled, freezes registered admission, rejects abandoned
    /// callbacks and interrupts the provider turn.
    async fn interrupt(&self) -> Result<(), RuntimeFailure> {
        self.turn.mark_cancelled();
        // Freeze registered admission before the interrupt: no new
        // registered call is admitted once cancellation is requested.
        self.turn.freeze_registered_tools().await;
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
        Ok(())
    }
}

impl CancellationControl for TurnCancellation {
    fn scope(&self) -> CancellationScope {
        CancellationScope::ActiveTurn
    }

    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        Box::pin(async move {
            if self.requested.swap(true, Ordering::SeqCst) {
                return Ok(CancellationAcknowledgement::AlreadyRequested);
            }
            let interrupted = self.interrupt().await;
            if interrupted.is_err() {
                // A failed interrupt leaves cancellation unrequested, so a retry may try again.
                self.requested.store(false, Ordering::SeqCst);
            }
            interrupted.map(|()| CancellationAcknowledgement::Requested)
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
    registered: Option<Arc<CodexRegisteredTurn>>,
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
            let cancelled = !self.cancellation.turn.is_finished();
            let interrupt = if cancelled {
                match self.cancellation.request().await {
                    Ok(_) => CleanupOutcome::NotApplicable,
                    Err(error) => {
                        // Without an acknowledged interrupt no provider terminal
                        // event finishes the turn, so the deadline watcher would
                        // otherwise wait out the whole turn deadline. Finish the
                        // turn so the watcher exits, and report the failure.
                        if !self.cancellation.turn.is_finished() {
                            self.cancellation.turn.finish(
                                TerminalStatus::Cancelled,
                                CleanupOutcome::Failed(error.diagnostic().clone()),
                            );
                        }
                        CleanupOutcome::Failed(error.diagnostic().clone())
                    }
                }
            } else {
                CleanupOutcome::NotApplicable
            };
            let registered_cleanup = match self.registered {
                Some(registered) => {
                    let cause = if cancelled {
                        RegisteredToolCleanupCause::Cancellation
                    } else {
                        RegisteredToolCleanupCause::Completion
                    };
                    registered.close(cause).await
                }
                None => CleanupOutcome::NotApplicable,
            };
            let task_cleanup = if let Some(task) = self.deadline_task {
                match task.join().await {
                    Ok(()) => CleanupOutcome::NotApplicable,
                    Err(error) => CleanupOutcome::Failed(error.diagnostic().clone()),
                }
            } else {
                CleanupOutcome::NotApplicable
            };
            CleanupOutcome::merge(
                interrupt,
                CleanupOutcome::merge(registered_cleanup, task_cleanup),
            )
        })
    }
}

#[cfg(test)]
#[path = "session/turn_close_tests.rs"]
mod turn_close_tests;

pub(crate) struct CodexSessionHandle {
    request_id: RequestId,
    runtime_id: RuntimeSessionId,
    resume_binding: SessionResumeBinding,
    connection: Arc<RpcConnection>,
    cancellation: SessionCancellation,
    task: AppServerTask,
    runtime: CodexSessionRuntime,
    access: CodexSessionAccess,
}

impl CodexSessionHandle {
    pub(crate) fn new(
        request_id: RequestId,
        runtime_id: RuntimeSessionId,
        resume_binding: SessionResumeBinding,
        connection: Arc<RpcConnection>,
        task: AppServerTask,
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
