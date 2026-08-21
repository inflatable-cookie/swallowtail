use self::cleanup::{merge_cleanup, release_credential, release_resource};
use super::handle::{PiSdkSidecarTurnBinding, PiSdkSidecarTurnHandle, SessionCancellation};
use super::input::{SharedAttachmentMaterialization, prepare_attachment};
use super::validation::validate_turn;
use crate::failure::failure;
use crate::sidecar::connection::SidecarConnection;
use crate::sidecar::turn::SidecarActiveTurn;
use crate::sidecar::wire::PiSdkSidecarCommand;
use serde_json::json;
use std::future::{Future, poll_fn};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::task::Poll;
use swallowtail_runtime::{
    BoxFuture, CancellationControl, CleanupOutcome, CredentialLease, HostServices,
    InteractiveSessionHandle, JoinedTask, RequestId, ResourceLease, RuntimeFailure,
    RuntimeSessionId, ScopeId, SessionResumeBinding, TurnHandle, TurnRequest,
};

pub(super) mod cleanup;

pub(super) fn cleanup_failure(code: &'static str, message: &'static str) -> CleanupOutcome {
    CleanupOutcome::Failed(swallowtail_core::SafeDiagnostic::new(
        "swallowtail.pi.sdk-sidecar.cleanup_failed",
        format!("{message} ({code})"),
    ))
}

pub(super) type ActiveSlot = Arc<Mutex<Option<ActiveTask>>>;

pub(super) struct ActiveTask {
    pub(super) turn: Arc<SidecarActiveTurn>,
    pub(super) deadline_task: Option<Box<dyn JoinedTask>>,
    pub(super) attachment: SharedAttachmentMaterialization,
}

pub(super) struct PiSdkSidecarSessionHandle {
    pub(super) request_id: RequestId,
    pub(super) runtime_id: RuntimeSessionId,
    pub(super) execution_host_id: swallowtail_core::ExecutionHostId,
    pub(super) binding: SessionResumeBinding,
    pub(super) connection: Arc<SidecarConnection>,
    pub(super) cancellation: SessionCancellation,
    pub(super) pump_task: Option<Box<dyn JoinedTask>>,
    pub(super) services: HostServices,
    pub(super) resource: Option<ResourceLease>,
    pub(super) credential: Option<CredentialLease>,
    pub(super) active: ActiveSlot,
    pub(super) completed_prompts: Arc<AtomicU32>,
    pub(super) image_attachments: bool,
}

impl InteractiveSessionHandle for PiSdkSidecarSessionHandle {
    fn request_id(&self) -> &RequestId {
        &self.request_id
    }

    fn session_id(&self) -> &RuntimeSessionId {
        &self.runtime_id
    }

    fn provider_session_ref(&self) -> Option<&swallowtail_core::SessionRef> {
        Some(self.binding.provider_session_ref())
    }

    fn resume_binding(&self) -> Option<&swallowtail_runtime::SessionResumeBinding> {
        Some(&self.binding)
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
                    "swallowtail.pi.sdk-sidecar.prompt_limit_reached",
                    "Pi SDK sidecar session reached its completed prompt limit",
                ));
            }
            if self
                .active
                .lock()
                .expect("sidecar active-task lock poisoned")
                .is_some()
            {
                return Err(failure(
                    "swallowtail.pi.sdk-sidecar.turn_active",
                    "Pi SDK sidecar session already has an active turn",
                ));
            }
            let (turn, events, terminal) = SidecarActiveTurn::new(
                request.turn_id().clone(),
                Arc::clone(&self.completed_prompts),
            )?;
            let turn_scope = ScopeId::new(format!(
                "pi-sdk-sidecar:turn:{}",
                request.turn_id().as_str()
            ))
            .map_err(|_| {
                failure(
                    "swallowtail.pi.sdk-sidecar.scope_invalid",
                    "Pi SDK sidecar turn scope was invalid",
                )
            })?;
            let attachment =
                prepare_attachment(request.attachments().next().cloned(), &services, turn_scope)
                    .await?;
            let materialization = attachment
                .as_ref()
                .map_or_else(SharedAttachmentMaterialization::default, |input| {
                    input.materialization()
                });
            self.connection.set_active_turn(Arc::clone(&turn))?;
            let deadline_task = match spawn_deadline(
                &services,
                Arc::clone(&self.connection),
                Arc::clone(&turn),
                request.deadline().expect("validated turn deadline"),
            ) {
                Ok(task) => task,
                Err(error) => {
                    turn.fail_connection(error.diagnostic().clone());
                    self.connection.clear_active_turn(&turn);
                    let _ = materialization.release().await;
                    return Err(error);
                }
            };
            *self
                .active
                .lock()
                .expect("sidecar active-task lock poisoned") = Some(ActiveTask {
                turn: Arc::clone(&turn),
                deadline_task: Some(deadline_task),
                attachment: materialization.clone(),
            });
            let id = format!("prompt:{}", request.turn_id().as_str());
            let mut params = json!({"text": request.content().as_str()});
            if let Some(attachment) = attachment {
                params["images"] = json!([{
                    "data": attachment.encoded(),
                    "mimeType": "image/png"
                }]);
            }
            let response = self
                .connection
                .command(id, PiSdkSidecarCommand::Prompt, params)
                .await;
            match response {
                Ok(response) if response.success => Ok(Box::new(PiSdkSidecarTurnHandle::new(
                    request.turn_id().clone(),
                    events,
                    Box::pin(terminal),
                    PiSdkSidecarTurnBinding {
                        connection: Arc::clone(&self.connection),
                        turn,
                        active: Arc::clone(&self.active),
                        attachment: materialization,
                    },
                )) as Box<dyn TurnHandle>),
                Ok(_) => {
                    turn.fail_connection(swallowtail_core::SafeDiagnostic::new(
                        "swallowtail.pi.sdk-sidecar.prompt_rejected",
                        "Pi SDK sidecar rejected the prompt before acceptance",
                    ));
                    self.connection.clear_active_turn(&turn);
                    let _ = materialization.release().await;
                    Err(failure(
                        "swallowtail.pi.sdk-sidecar.prompt_rejected",
                        "Pi SDK sidecar rejected the prompt before acceptance",
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

    fn close(mut self: Box<Self>) -> BoxFuture<'static, CleanupOutcome> {
        Box::pin(async move {
            let active = self
                .active
                .lock()
                .expect("sidecar active-task lock poisoned")
                .take();
            if let Some(active) = &active
                && !active.turn.is_finished()
            {
                active.turn.mark_cancelled();
            }
            let id = format!("close:{}", self.request_id.as_str());
            let _ = self
                .connection
                .command(id, PiSdkSidecarCommand::Close, json!({}))
                .await;
            self.connection.begin_close().await;
            if let Some(mut active) = active {
                if let Some(task) = active.deadline_task.take() {
                    let _ = task.join().await;
                }
                let _ = active.attachment.release().await;
            }
            let process = match self.pump_task.take() {
                Some(task) => match task.join().await {
                    Ok(()) => self.connection.cleanup_outcome(),
                    Err(_) => cleanup_failure(
                        "pump_join_failed",
                        "Pi SDK sidecar protocol task did not join cleanly",
                    ),
                },
                None => CleanupOutcome::NotApplicable,
            };
            let resource = release_resource(self.resource.take(), &self.services).await;
            let credential = release_credential(self.credential.take(), &self.services).await;
            merge_cleanup(merge_cleanup(process, resource), credential)
        })
    }
}

fn spawn_deadline(
    services: &HostServices,
    connection: Arc<SidecarConnection>,
    turn: Arc<SidecarActiveTurn>,
    deadline: swallowtail_runtime::Deadline,
) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
    let mut wait = services
        .time()
        .cloned()
        .expect("validated sidecar time service")
        .wait_until(deadline);
    let mut finished = Box::pin(turn.finished_future());
    let deadline_turn = Arc::clone(&turn);
    let scope = ScopeId::new(format!(
        "pi-sdk-sidecar:deadline:{}",
        turn.runtime_id().as_str()
    ))
    .map_err(|_| {
        failure(
            "swallowtail.pi.sdk-sidecar.scope_invalid",
            "Pi SDK sidecar scope was invalid",
        )
    })?;
    services
        .task()
        .expect("validated sidecar task service")
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
                        .command(id, PiSdkSidecarCommand::Abort, json!({}))
                        .await;
                }
            }),
        )
}

async fn reap_finished(active: &ActiveSlot) -> Result<(), RuntimeFailure> {
    let finished = {
        let mut active = active.lock().expect("sidecar active-task lock poisoned");
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
                "swallowtail.pi.sdk-sidecar.attachment_cleanup_failed",
                "Pi SDK sidecar attachment cleanup failed",
            ));
        }
    }
    Ok(())
}
