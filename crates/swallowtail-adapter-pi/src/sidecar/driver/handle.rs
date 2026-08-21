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
}

impl TurnCancellation {
    pub(super) fn new(connection: Arc<SidecarConnection>, turn: Arc<SidecarActiveTurn>) -> Self {
        Self {
            connection,
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
            let id = format!("abort:{}", self.turn.runtime_id().as_str());
            let response = self
                .connection
                .command(id, PiSdkSidecarCommand::Abort, json!({}))
                .await?;
            if response.success {
                Ok(CancellationAcknowledgement::Requested)
            } else {
                Err(failure(
                    "swallowtail.pi.sdk-sidecar.abort_rejected",
                    "Pi SDK sidecar rejected native abort",
                ))
            }
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
            if !self.cancellation.turn.is_finished() {
                let _ = self.cancellation.request().await;
            }
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
            super::session::cleanup::merge_cleanup(task_cleanup, self.attachment.release().await)
        })
    }
}
