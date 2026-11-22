use super::input::SharedAttachmentMaterialization;
use super::session::{ActiveSlot, cleanup_failure};
use crate::sidecar::connection::SidecarConnection;
use crate::sidecar::failure::failure;
use crate::sidecar::turn::SidecarActiveTurn;
use crate::sidecar::wire::PiSdkSidecarCommand;
use serde_json::json;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use swallowtail_core::{CancellationScope, HarnessMessageClass, TurnRef};
use swallowtail_runtime::{
    BoxEventStream, BoxFuture, CallbackExchange, CancellationAcknowledgement, CancellationControl,
    CleanupOutcome, HarnessCommandAcknowledgement, HarnessCommandResponse, HarnessScheduledMessage,
    RuntimeFailure, RuntimeTurnId, TerminalOutcome, TurnHandle,
};

pub(super) struct TurnCancellation {
    connection: Arc<SidecarConnection>,
    turn: Arc<SidecarActiveTurn>,
    requested: AtomicBool,
    rejected: AtomicBool,
}

impl TurnCancellation {
    pub(super) fn new(connection: Arc<SidecarConnection>, turn: Arc<SidecarActiveTurn>) -> Self {
        Self {
            connection,
            turn,
            requested: AtomicBool::new(false),
            rejected: AtomicBool::new(false),
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
                if self.rejected.load(Ordering::SeqCst) {
                    return Err(failure(
                        "swallowtail.pi.sdk-sidecar.abort_rejected",
                        "Pi SDK sidecar rejected native abort",
                    ));
                }
                return Ok(CancellationAcknowledgement::AlreadyRequested);
            }
            self.turn.mark_cancelled();
            let id = format!("abort:{}", self.turn.runtime_id().as_str());
            let error = match self
                .connection
                .command(id, PiSdkSidecarCommand::Abort, json!({}))
                .await
            {
                Ok(response) if response.success => {
                    return Ok(CancellationAcknowledgement::Requested);
                }
                Ok(_) => failure(
                    "swallowtail.pi.sdk-sidecar.abort_rejected",
                    "Pi SDK sidecar rejected native abort",
                ),
                Err(error) => error,
            };
            // A turn that could not be aborted may still be running on the
            // provider. Record the refusal and close the connection immediately
            // so late events can never be assigned to a replacement turn and
            // the turn's deadline watcher wakes cleanly.
            self.rejected.store(true, Ordering::SeqCst);
            self.connection
                .close_on_rejected_abort(swallowtail_core::SafeDiagnostic::new(
                    "swallowtail.pi.sdk-sidecar.cancel_rejected",
                    "Pi could not abort the active turn",
                ));
            Err(error)
        })
    }
}

pub(super) struct SessionCancellation {
    connection: Arc<SidecarConnection>,
    active: ActiveSlot,
    requested: AtomicBool,
}

impl SessionCancellation {
    pub(super) fn new(connection: Arc<SidecarConnection>, active: ActiveSlot) -> Self {
        Self {
            connection,
            active,
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
                return Ok(CancellationAcknowledgement::AlreadyRequested);
            }
            if let Some(active) = self
                .active
                .lock()
                .expect("sidecar active-task lock poisoned")
                .as_ref()
            {
                active.turn.mark_cancelled();
            }
            self.connection.force_stop().await?;
            Ok(CancellationAcknowledgement::Requested)
        })
    }
}

pub(super) struct PiSdkSidecarTurnHandle {
    runtime_id: RuntimeTurnId,
    events: Option<BoxEventStream>,
    terminal: Option<BoxFuture<'static, TerminalOutcome>>,
    cancellation: TurnCancellation,
    connection: Arc<SidecarConnection>,
    active: ActiveSlot,
    attachment: SharedAttachmentMaterialization,
}

pub(super) struct PiSdkSidecarTurnBinding {
    pub(super) connection: Arc<SidecarConnection>,
    pub(super) turn: Arc<SidecarActiveTurn>,
    pub(super) active: ActiveSlot,
    pub(super) attachment: SharedAttachmentMaterialization,
}

impl PiSdkSidecarTurnHandle {
    pub(super) fn new(
        runtime_id: RuntimeTurnId,
        events: BoxEventStream,
        terminal: BoxFuture<'static, TerminalOutcome>,
        binding: PiSdkSidecarTurnBinding,
    ) -> Self {
        Self {
            runtime_id,
            events: Some(events),
            terminal: Some(terminal),
            cancellation: TurnCancellation::new(Arc::clone(&binding.connection), binding.turn),
            connection: binding.connection,
            active: binding.active,
            attachment: binding.attachment,
        }
    }
}

impl TurnHandle for PiSdkSidecarTurnHandle {
    fn turn_id(&self) -> &RuntimeTurnId {
        &self.runtime_id
    }

    fn provider_turn_ref(&self) -> Option<&TurnRef> {
        None
    }

    fn take_events(&mut self) -> Option<BoxEventStream> {
        self.events.take()
    }

    fn take_callbacks(&mut self) -> Option<CallbackExchange> {
        None
    }

    fn schedule_harness_message(
        &mut self,
        message: HarnessScheduledMessage,
    ) -> BoxFuture<'_, Result<HarnessCommandResponse, RuntimeFailure>> {
        Box::pin(async move {
            let class = message.class();
            if class == HarnessMessageClass::Prompt || self.cancellation.turn.is_finished() {
                return Ok(HarnessCommandResponse::new(
                    message.command_id().clone(),
                    HarnessCommandAcknowledgement::Rejected,
                ));
            }
            if !self.cancellation.turn.reserve_scheduling(class) {
                return Ok(HarnessCommandResponse::new(
                    message.command_id().clone(),
                    HarnessCommandAcknowledgement::Rejected,
                ));
            }
            let command = match class {
                HarnessMessageClass::Steering => PiSdkSidecarCommand::Steer,
                HarnessMessageClass::FollowUp => PiSdkSidecarCommand::FollowUp,
                HarnessMessageClass::Prompt => unreachable!(),
            };
            let id = message.command_id().as_str().to_owned();
            let result = self
                .connection
                .command(id, command, json!({"text": message.content().as_str()}))
                .await;
            match result {
                Ok(response) if response.success => Ok(HarnessCommandResponse::new(
                    message.command_id().clone(),
                    HarnessCommandAcknowledgement::Accepted,
                )),
                Ok(_) => {
                    self.cancellation.turn.release_scheduling(class);
                    Ok(HarnessCommandResponse::new(
                        message.command_id().clone(),
                        HarnessCommandAcknowledgement::Rejected,
                    ))
                }
                Err(error) => {
                    self.cancellation.turn.release_scheduling(class);
                    Err(error)
                }
            }
        })
    }

    fn cancellation(&self) -> &dyn CancellationControl {
        &self.cancellation
    }

    fn take_terminal_outcome(&mut self) -> Option<BoxFuture<'static, TerminalOutcome>> {
        self.terminal.take()
    }

    fn close(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome> {
        Box::pin(async move {
            let cancel_cleanup = if self.cancellation.rejected.load(Ordering::SeqCst) {
                cleanup_failure(
                    "cancel_rejected",
                    "Pi could not abort the active turn during close",
                )
            } else if self.cancellation.turn.is_finished() {
                CleanupOutcome::NotApplicable
            } else {
                match self.cancellation.request().await {
                    Ok(_) => CleanupOutcome::NotApplicable,
                    Err(_) => cleanup_failure(
                        "cancel_rejected",
                        "Pi could not abort the active turn during close",
                    ),
                }
            };
            let active = {
                let mut slot = self
                    .active
                    .lock()
                    .expect("sidecar active-task lock poisoned");
                if slot
                    .as_ref()
                    .is_some_and(|active| Arc::ptr_eq(&active.turn, &self.cancellation.turn))
                {
                    slot.take()
                } else {
                    None
                }
            };
            let task_cleanup =
                if let Some(task) = active.and_then(|mut active| active.deadline_task.take()) {
                    if task.join().await.is_err() {
                        cleanup_failure(
                            "turn_join_failed",
                            "Pi SDK sidecar turn deadline task did not join cleanly",
                        )
                    } else {
                        CleanupOutcome::NotApplicable
                    }
                } else {
                    CleanupOutcome::NotApplicable
                };
            CleanupOutcome::merge(
                CleanupOutcome::merge(cancel_cleanup, task_cleanup),
                self.attachment.release().await,
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sidecar::driver::session::ActiveTask;
    use std::sync::Mutex;
    use std::sync::atomic::AtomicU32;
    use swallowtail_core::{ExecutionHostId, SafeDiagnostic};
    use swallowtail_runtime::{
        HostServices, JoinedTask, ProcessExit, ProcessHandle, ProcessInputChunk, ProcessOutputChunk,
    };

    struct RefusingStdin;

    impl ProcessHandle for RefusingStdin {
        fn write_stdin(&self, _: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async {
                Err(RuntimeFailure::new(SafeDiagnostic::new(
                    "test.stdin_refused",
                    "stdin refused",
                )))
            })
        }

        fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }

        fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
            Box::pin(async { Ok(None) })
        }

        fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }

        fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }

        fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
            Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
        }
    }

    struct WatchSidecarTurnFinish {
        turn: Arc<SidecarActiveTurn>,
    }

    impl JoinedTask for WatchSidecarTurnFinish {
        fn join(self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
            Box::pin(async move {
                let mut finished = Box::pin(self.turn.finished_future());
                let mut polls = 0usize;
                std::future::poll_fn(move |context| {
                    if std::future::Future::poll(finished.as_mut(), context).is_ready() {
                        return std::task::Poll::Ready(Ok(()));
                    }
                    polls += 1;
                    if polls > 16 {
                        return std::task::Poll::Ready(Err(RuntimeFailure::new(
                            SafeDiagnostic::new(
                                "test.deadline_watch_hung",
                                "deadline watcher waited past rejected abort",
                            ),
                        )));
                    }
                    context.waker().wake_by_ref();
                    std::task::Poll::Pending
                })
                .await
            })
        }
    }

    #[test]
    fn rejected_abort_during_close_wakes_deadline_task_and_reports_cleanup_failure() {
        let services = HostServices::new(
            ExecutionHostId::new("pi.sidecar.close-test").expect("host id is valid"),
        );
        let connection = SidecarConnection::new(Arc::new(RefusingStdin), services);
        let (turn, events, terminal) = SidecarActiveTurn::new(
            RuntimeTurnId::new("sidecar-turn-close").expect("turn id is valid"),
            Arc::new(AtomicU32::new(0)),
        )
        .expect("turn constructs");
        let active: ActiveSlot = Arc::new(Mutex::new(Some(ActiveTask {
            turn: Arc::clone(&turn),
            deadline_task: Some(Box::new(WatchSidecarTurnFinish {
                turn: Arc::clone(&turn),
            })),
            attachment: SharedAttachmentMaterialization::default(),
        })));
        let connection_ref = Arc::clone(&connection);
        connection
            .set_active_turn(Arc::clone(&turn))
            .expect("initial turn attaches");
        let handle = PiSdkSidecarTurnHandle::new(
            RuntimeTurnId::new("sidecar-turn-close").expect("turn id is valid"),
            events,
            Box::pin(terminal),
            super::PiSdkSidecarTurnBinding {
                connection,
                turn: Arc::clone(&turn),
                active,
                attachment: SharedAttachmentMaterialization::default(),
            },
        );
        let handle: Box<dyn TurnHandle> = Box::new(handle);
        let cleanup = futures_executor::block_on(handle.close());
        assert!(
            matches!(cleanup, CleanupOutcome::Failed(ref diagnostic) if diagnostic.code() == "swallowtail.pi.sdk-sidecar.cleanup_failed" && diagnostic.message().contains("cancel_rejected")),
            "expected cancel_rejected cleanup failure, got {cleanup:?}"
        );
        assert!(
            turn.is_finished(),
            "turn must finish so deadline task wakes"
        );
        let (next_turn, _next_events, _next_terminal) = SidecarActiveTurn::new(
            RuntimeTurnId::new("sidecar-turn-next").expect("turn id is valid"),
            Arc::new(AtomicU32::new(0)),
        )
        .expect("next turn constructs");
        let next_err = connection_ref
            .set_active_turn(next_turn)
            .expect_err("connection must refuse a replacement turn after rejected abort");
        assert_eq!(
            next_err.diagnostic().code(),
            "swallowtail.pi.sdk-sidecar.cancel_rejected"
        );
    }
}
