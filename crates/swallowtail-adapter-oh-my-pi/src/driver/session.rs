use self::cleanup::{merge_cleanup, release_resource};
use super::handle::{OhMyPiTurnBinding, OhMyPiTurnHandle, SessionCancellation};
use super::input::{SharedAttachmentMaterialization, prepare_attachment};
use super::validation::validate_turn;
use crate::connection::OhMyPiConnection;
use crate::failure::failure;
use crate::turn::ActiveTurn;
use serde_json::json;
use std::future::{Future, poll_fn};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::task::Poll;
use swallowtail_runtime::{
    BoxFuture, CancellationControl, CleanupOutcome, HostServices, InteractiveSessionHandle,
    JoinedTask, RequestId, ResourceLease, RuntimeFailure, RuntimeSessionId, ScopeId, TurnHandle,
    TurnRequest,
};

pub(super) mod cleanup;

pub(super) fn cleanup_failure(code: &'static str, message: &'static str) -> CleanupOutcome {
    CleanupOutcome::Failed(swallowtail_core::SafeDiagnostic::new(
        "swallowtail.oh_my_pi.rpc.cleanup_failed",
        format!("{message} ({code})"),
    ))
}

pub(super) type ActiveSlot = Arc<Mutex<Option<ActiveTask>>>;

pub(super) struct ActiveTask {
    pub(super) turn: Arc<ActiveTurn>,
    pub(super) deadline_task: Option<Box<dyn JoinedTask>>,
    pub(super) attachment: SharedAttachmentMaterialization,
}

pub(super) struct OhMyPiSessionHandle {
    pub(super) request_id: RequestId,
    pub(super) runtime_id: RuntimeSessionId,
    pub(super) execution_host_id: swallowtail_core::ExecutionHostId,
    pub(super) connection: Arc<OhMyPiConnection>,
    pub(super) cancellation: SessionCancellation,
    pub(super) pump_task: Option<Box<dyn JoinedTask>>,
    pub(super) services: HostServices,
    pub(super) resource: Option<ResourceLease>,
    pub(super) active: ActiveSlot,
    pub(super) completed_prompts: Arc<AtomicU32>,
    pub(super) image_attachments: bool,
}

impl InteractiveSessionHandle for OhMyPiSessionHandle {
    fn request_id(&self) -> &RequestId {
        &self.request_id
    }

    fn session_id(&self) -> &RuntimeSessionId {
        &self.runtime_id
    }

    fn provider_session_ref(&self) -> Option<&swallowtail_core::SessionRef> {
        None
    }

    fn resume_binding(&self) -> Option<&swallowtail_runtime::SessionResumeBinding> {
        None
    }

    fn start_turn<'a>(
        &'a mut self,
        request: TurnRequest,
        services: HostServices,
    ) -> BoxFuture<'a, Result<Box<dyn TurnHandle>, RuntimeFailure>> {
        Box::pin(async move {
            services.require_execution_host(&self.execution_host_id)?;
            validate_turn(&request, &services, self.image_attachments)?;
            reap_finished(&self.active).await?;
            if self.completed_prompts.load(Ordering::SeqCst) >= 2 {
                return Err(failure(
                    "swallowtail.oh_my_pi.rpc.prompt_limit_reached",
                    "OhMyPi RPC session reached its completed prompt limit",
                ));
            }
            if self
                .active
                .lock()
                .expect("OhMyPi active-task lock poisoned")
                .is_some()
            {
                return Err(failure(
                    "swallowtail.oh_my_pi.rpc.turn_active",
                    "OhMyPi RPC session already has an active turn",
                ));
            }
            let (turn, events, callbacks, terminal) = ActiveTurn::new(
                request.turn_id().clone(),
                Arc::clone(&self.completed_prompts),
                Arc::downgrade(&self.connection),
            )?;
            let turn_scope =
                ScopeId::new(format!("oh-my-pi-rpc:turn:{}", request.turn_id().as_str())).map_err(
                    |_| {
                        failure(
                            "swallowtail.oh_my_pi.rpc.scope_invalid",
                            "OhMyPi RPC turn scope was invalid",
                        )
                    },
                )?;
            let attachment =
                prepare_attachment(request.attachments().next().cloned(), &services, turn_scope)
                    .await?;
            let materialization = attachment
                .as_ref()
                .map_or_else(SharedAttachmentMaterialization::default, |input| {
                    input.materialization()
                });
            self.connection.set_active_turn(Arc::clone(&turn))?;
            let deadline_task = spawn_deadline(
                &services,
                Arc::clone(&self.connection),
                Arc::clone(&turn),
                request.deadline().expect("validated turn deadline"),
            )?;
            *self
                .active
                .lock()
                .expect("OhMyPi active-task lock poisoned") = Some(ActiveTask {
                turn: Arc::clone(&turn),
                deadline_task: Some(deadline_task),
                attachment: materialization.clone(),
            });
            let id = format!("prompt:{}", request.turn_id().as_str());
            let mut payload =
                json!({"id": id, "type": "prompt", "message": request.content().as_str()});
            if let Some(attachment) = attachment {
                payload["images"] = json!([{
                    "type": "image",
                    "data": attachment.encoded(),
                    "mimeType": "image/png"
                }]);
            }
            let response = self.connection.command(id.clone(), "prompt", payload).await;
            match response {
                Ok(response) if response.success => Ok(Box::new(OhMyPiTurnHandle::new(
                    request.turn_id().clone(),
                    events,
                    callbacks,
                    Box::pin(terminal),
                    OhMyPiTurnBinding {
                        connection: Arc::clone(&self.connection),
                        turn,
                        active: Arc::clone(&self.active),
                        attachment: materialization,
                    },
                )) as Box<dyn TurnHandle>),
                Ok(_) => {
                    turn.fail_connection(swallowtail_core::SafeDiagnostic::new(
                        "swallowtail.oh_my_pi.rpc.prompt_rejected",
                        "OhMyPi RPC rejected the prompt before acceptance",
                    ));
                    self.connection.clear_active_turn(&turn);
                    let _ = materialization.release().await;
                    Err(failure(
                        "swallowtail.oh_my_pi.rpc.prompt_rejected",
                        "OhMyPi RPC rejected the prompt before acceptance",
                    ))
                }
                Err(error) => {
                    turn.fail_connection(error.diagnostic().clone());
                    self.connection.clear_active_turn(&turn);
                    let _ = materialization.release().await;
                    Err(error)
                }
            }
        })
    }

    fn cancellation(&self) -> &dyn CancellationControl {
        &self.cancellation
    }

    fn close(
        mut self: Box<Self>,
        request: swallowtail_runtime::SessionCleanupRequest,
        services: HostServices,
    ) -> BoxFuture<'static, CleanupOutcome> {
        let execution_host_id = self.execution_host_id.clone();
        swallowtail_runtime::bound_session_cleanup(
            execution_host_id,
            request,
            services,
            Box::pin(async move {
                let active = self
                    .active
                    .lock()
                    .expect("OhMyPi active-task lock poisoned")
                    .take();
                if let Some(mut active) = active {
                    if !active.turn.is_finished() {
                        active.turn.mark_cancelled();
                    }
                    self.connection.begin_close().await;
                    if let Some(task) = active.deadline_task.take() {
                        let _ = task.join().await;
                    }
                    let _ = active.attachment.release().await;
                } else {
                    self.connection.begin_close().await;
                }
                let process = match self.pump_task.take() {
                    Some(task) => match task.join().await {
                        Ok(()) => self.connection.cleanup_outcome(),
                        Err(_) => cleanup_failure(
                            "pump_join_failed",
                            "OhMyPi RPC protocol task did not join cleanly",
                        ),
                    },
                    None => CleanupOutcome::NotApplicable,
                };
                let callbacks = self.connection.join_callback_tasks().await;
                let resource = release_resource(self.resource.take(), &self.services).await;
                merge_cleanup(merge_cleanup(process, callbacks), resource)
            }),
        )
    }
}

fn spawn_deadline(
    services: &HostServices,
    connection: Arc<OhMyPiConnection>,
    turn: Arc<ActiveTurn>,
    deadline: swallowtail_runtime::Deadline,
) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
    let mut wait = services
        .time()
        .cloned()
        .expect("validated OhMyPi time service")
        .wait_until(deadline);
    let mut finished = Box::pin(turn.finished_future());
    let deadline_turn = Arc::clone(&turn);
    let scope = ScopeId::new(format!(
        "oh-my-pi-rpc:deadline:{}",
        turn.runtime_id().as_str()
    ))
    .map_err(|_| {
        failure(
            "swallowtail.oh_my_pi.rpc.scope_invalid",
            "OhMyPi RPC scope was invalid",
        )
    })?;
    services
        .task()
        .expect("validated OhMyPi task service")
        .spawn(
            scope,
            Box::pin(async move {
                let timed_out = poll_fn(|context| {
                    if finished.as_mut().poll(context).is_ready() {
                        Poll::Ready(false)
                    } else if wait.as_mut().poll(context).is_ready() {
                        Poll::Ready(true)
                    } else {
                        Poll::Pending
                    }
                })
                .await;
                if timed_out {
                    deadline_turn.mark_timed_out();
                    let id = format!("deadline-abort:{}", deadline_turn.runtime_id().as_str());
                    let _ = connection
                        .command(id.clone(), "abort", json!({"id": id, "type": "abort"}))
                        .await;
                }
            }),
        )
}

async fn reap_finished(active: &ActiveSlot) -> Result<(), RuntimeFailure> {
    let finished = {
        let mut active = active.lock().expect("OhMyPi active-task lock poisoned");
        if active
            .as_ref()
            .is_some_and(|active| active.turn.is_finished())
        {
            active.take()
        } else {
            None
        }
    };
    if let Some(mut active) = finished {
        if let Some(task) = active.deadline_task.take() {
            task.join().await?;
        }
        if matches!(active.attachment.release().await, CleanupOutcome::Failed(_)) {
            return Err(failure(
                "swallowtail.oh_my_pi.rpc.attachment_cleanup_failed",
                "OhMyPi RPC attachment cleanup failed",
            ));
        }
    }
    Ok(())
}
