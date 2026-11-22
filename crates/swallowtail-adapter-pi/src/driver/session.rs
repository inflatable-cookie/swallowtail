use self::cleanup::{release_credential, release_resource};
use super::handle::{PiTurnBinding, PiTurnHandle, SessionCancellation};
use super::input::{SharedAttachmentMaterialization, prepare_attachment};
use super::validation::validate_turn;
use crate::connection::PiConnection;
use crate::driver::disposition::{CommandDisposition, parse_command_disposition};
use crate::driver::{ABORT_ACKNOWLEDGEMENT_GRACE_TICKS, grace_deadline, run_before_deadline};
use crate::failure::failure;
use crate::selection::PiRpcVersionFeatures;
use crate::turn::ActiveTurn;
use serde_json::json;
use std::future::{Future, poll_fn};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::task::Poll;
use swallowtail_runtime::{
    BoxFuture, CancellationControl, CleanupOutcome, CredentialLease, HostServices,
    InteractiveSessionHandle, JoinedTask, RequestId, ResourceLease, RuntimeFailure,
    RuntimeSessionId, ScopeId, TurnHandle, TurnRequest,
};

pub(super) mod cleanup;

pub(super) fn cleanup_failure(code: &'static str, message: &'static str) -> CleanupOutcome {
    CleanupOutcome::Failed(swallowtail_core::SafeDiagnostic::new(
        "swallowtail.pi.rpc.cleanup_failed",
        format!("{message} ({code})"),
    ))
}

pub(super) type ActiveSlot = Arc<Mutex<Option<ActiveTask>>>;

pub(super) struct ActiveTask {
    pub(super) turn: Arc<ActiveTurn>,
    pub(super) deadline_task: Option<Box<dyn JoinedTask>>,
    pub(super) attachment: SharedAttachmentMaterialization,
}

pub(super) struct PiSessionHandle {
    pub(super) request_id: RequestId,
    pub(super) runtime_id: RuntimeSessionId,
    pub(super) execution_host_id: swallowtail_core::ExecutionHostId,
    pub(super) connection: Arc<PiConnection>,
    pub(super) cancellation: SessionCancellation,
    pub(super) pump_task: Option<Box<dyn JoinedTask>>,
    pub(super) services: HostServices,
    pub(super) resource: Option<ResourceLease>,
    pub(super) credential: Option<CredentialLease>,
    pub(super) active: ActiveSlot,
    pub(super) completed_prompts: Arc<AtomicU32>,
    pub(super) version_features: PiRpcVersionFeatures,
    pub(super) image_attachments: bool,
}

impl InteractiveSessionHandle for PiSessionHandle {
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
                    "swallowtail.pi.rpc.prompt_limit_reached",
                    "Pi RPC session reached its completed prompt limit",
                ));
            }
            if self
                .active
                .lock()
                .expect("Pi active-task lock poisoned")
                .is_some()
            {
                return Err(failure(
                    "swallowtail.pi.rpc.turn_active",
                    "Pi RPC session already has an active turn",
                ));
            }
            let deadline = request.deadline().expect("validated turn deadline");
            if services.time().expect("validated Pi time service").now() >= deadline.instant() {
                return Err(failure(
                    "swallowtail.pi.rpc.prompt_timed_out",
                    "Pi RPC prompt timed out before acceptance",
                ));
            }
            let (turn, events, callbacks, terminal) = ActiveTurn::new(
                request.turn_id().clone(),
                Arc::clone(&self.completed_prompts),
                Arc::downgrade(&self.connection),
                self.version_features.aborted_settled,
            )?;
            let turn_scope = ScopeId::new(format!("pi-rpc:turn:{}", request.turn_id().as_str()))
                .map_err(|_| {
                    failure(
                        "swallowtail.pi.rpc.scope_invalid",
                        "Pi RPC turn scope was invalid",
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
            if let Err(error) = self.connection.set_active_turn(Arc::clone(&turn)) {
                let _ = materialization.release().await;
                return Err(error);
            }
            // Spawn the deadline watcher before sending `prompt` so a spawn
            // failure rolls back cleanly without dispatching provider work, but
            // keep it parked on `arm_rx` until `prompt` is accepted so
            // `start_turn` remains the sole deadline owner during dispatch.
            // Store the task in `self.active` immediately and bind `arm_tx` to
            // `PendingStartGuard` so dropping a pending `start_turn` future
            // disarms the task before `session.close()` joins it.
            let (arm_tx, arm_rx) = futures_channel::oneshot::channel();
            let deadline_task = match spawn_deadline(
                &services,
                Arc::clone(&self.connection),
                Arc::clone(&turn),
                deadline,
                arm_rx,
            ) {
                Ok(task) => task,
                Err(error) => {
                    self.connection.clear_active_turn(&turn);
                    let _ = materialization.release().await;
                    return Err(error);
                }
            };
            *self.active.lock().expect("Pi active-task lock poisoned") = Some(ActiveTask {
                turn: Arc::clone(&turn),
                deadline_task: Some(deadline_task),
                attachment: materialization.clone(),
            });
            let prompt_dispatched = Arc::new(std::sync::atomic::AtomicBool::new(false));
            struct PendingStartGuard<'a> {
                arm_tx: Option<futures_channel::oneshot::Sender<()>>,
                connection: &'a Arc<PiConnection>,
                turn: &'a Arc<ActiveTurn>,
                dispatched: Arc<std::sync::atomic::AtomicBool>,
                active: bool,
            }
            impl PendingStartGuard<'_> {
                fn accept(mut self) {
                    if let Some(arm_tx) = self.arm_tx.take() {
                        let _ = arm_tx.send(());
                    }
                    self.active = false;
                }

                fn disarm(mut self) {
                    drop(self.arm_tx.take());
                    self.active = false;
                }
            }
            impl Drop for PendingStartGuard<'_> {
                fn drop(&mut self) {
                    // Always disarm `spawn_deadline` first so any subsequent
                    // join (including join-on-drop hosts) never deadlocks.
                    drop(self.arm_tx.take());
                    if !self.active {
                        return;
                    }
                    if self.dispatched.load(Ordering::SeqCst) {
                        let _ = self.connection.close_on_rejected_abort(
                            self.turn,
                            swallowtail_core::SafeDiagnostic::new(
                                "swallowtail.pi.rpc.prompt_abandoned",
                                "Pi RPC prompt dispatch was dropped before acceptance",
                            ),
                        );
                    } else {
                        self.turn
                            .fail_connection(swallowtail_core::SafeDiagnostic::new(
                                "swallowtail.pi.rpc.prompt_abandoned",
                                "Pi RPC prompt startup was dropped before dispatch",
                            ));
                        self.connection.clear_active_turn(self.turn);
                    }
                }
            }
            let guard = PendingStartGuard {
                arm_tx: Some(arm_tx),
                connection: &self.connection,
                turn: &turn,
                dispatched: Arc::clone(&prompt_dispatched),
                active: true,
            };
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
            let response = crate::driver::run_before_deadline(
                self.connection
                    .command_tracked(id.clone(), "prompt", payload, &prompt_dispatched),
                Some(deadline),
                &services,
                "swallowtail.pi.rpc.prompt_timed_out",
                "Pi RPC prompt timed out before acceptance",
            )
            .await;
            let accepted = match response {
                Ok(response) if response.success => {
                    match parse_command_disposition(
                        response.data.as_ref(),
                        "prompt",
                        self.version_features.command_dispositions,
                    ) {
                        Ok(Some(CommandDisposition::Handled)) => Err(failure(
                            "swallowtail.pi.rpc.prompt_handled",
                            "Pi handled the prompt without starting an agent run",
                        )),
                        Ok(Some(CommandDisposition::Queued | CommandDisposition::Started))
                        | Ok(None) => Ok(()),
                        Err(error) => Err(error),
                    }
                }
                Ok(_) => Err(failure(
                    "swallowtail.pi.rpc.prompt_rejected",
                    "Pi RPC rejected the prompt before acceptance",
                )),
                Err(error) => Err(error),
            };
            match accepted {
                Ok(()) => {
                    guard.accept();
                    Ok(Box::new(PiTurnHandle::new(
                        request.turn_id().clone(),
                        events,
                        callbacks,
                        Box::pin(terminal),
                        PiTurnBinding {
                            connection: Arc::clone(&self.connection),
                            turn,
                            active: Arc::clone(&self.active),
                            attachment: materialization,
                            command_dispositions: self.version_features.command_dispositions,
                        },
                    )) as Box<dyn TurnHandle>)
                }
                Err(error) => {
                    guard.disarm();
                    if error.diagnostic().code() == "swallowtail.pi.rpc.prompt_timed_out"
                        && prompt_dispatched.load(Ordering::SeqCst)
                    {
                        // The prompt write may have already reached Pi with
                        // unconfirmed settlement. Close this exact connection
                        // and force-stop so live work cannot outlive the timed-
                        // out call or collide with a subsequent turn.
                        turn.mark_timed_out();
                        if self
                            .connection
                            .close_on_rejected_abort(&turn, error.diagnostic().clone())
                        {
                            let _ = self.connection.force_stop().await;
                        }
                    } else {
                        turn.fail_connection(error.diagnostic().clone());
                        self.connection.clear_active_turn(&turn);
                    }
                    let _ = reap_finished(&self.active).await;
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
        self: Box<Self>,
        request: swallowtail_runtime::SessionCleanupRequest,
        services: HostServices,
    ) -> BoxFuture<'static, CleanupOutcome> {
        let execution_host_id = self.execution_host_id.clone();
        swallowtail_runtime::bound_session_cleanup(
            execution_host_id,
            request,
            services,
            Box::pin(self.close_session()),
        )
    }
}

/// Grace for joining the private process and releasing the run's leases.
const RUN_SESSION_CLEANUP_GRACE_TICKS: u64 = 500;

impl PiSessionHandle {
    pub(super) async fn close_run(self) -> CleanupOutcome {
        let Some(deadline) = grace_deadline(&self.services, RUN_SESSION_CLEANUP_GRACE_TICKS) else {
            return self.close_session().await;
        };
        let services = self.services.clone();
        Box::new(self)
            .close(
                swallowtail_runtime::SessionCleanupRequest::new(deadline),
                services,
            )
            .await
    }

    async fn close_session(mut self) -> CleanupOutcome {
        let active = self
            .active
            .lock()
            .expect("Pi active-task lock poisoned")
            .take();
        let active_cleanup = settle_active_task(active, &self.connection).await;
        let process = match self.pump_task.take() {
            Some(task) => match task.join().await {
                Ok(()) => self.connection.cleanup_outcome(),
                Err(_) => cleanup_failure(
                    "pump_join_failed",
                    "Pi RPC protocol task did not join cleanly",
                ),
            },
            None => CleanupOutcome::NotApplicable,
        };
        let callbacks = self.connection.join_callback_tasks().await;
        let resource = release_resource(self.resource.take(), &self.services).await;
        let credential = release_credential(self.credential.take(), &self.services).await;
        CleanupOutcome::merge(
            CleanupOutcome::merge(
                CleanupOutcome::merge(CleanupOutcome::merge(active_cleanup, process), callbacks),
                resource,
            ),
            credential,
        )
    }
}

/// Settles the active turn when a session closes: cancels it, joins its
/// deadline task, and releases its attachment. Each failure merges into the
/// returned outcome instead of being discarded.
async fn settle_active_task(
    active: Option<ActiveTask>,
    connection: &PiConnection,
) -> CleanupOutcome {
    let Some(mut active) = active else {
        connection.begin_close().await;
        return CleanupOutcome::NotApplicable;
    };
    if !active.turn.is_finished() {
        active.turn.mark_cancelled();
    }
    connection.begin_close().await;
    let deadline = match active.deadline_task.take() {
        Some(task) => match task.join().await {
            Ok(()) => CleanupOutcome::Clean,
            Err(_) => cleanup_failure(
                "deadline_join_failed",
                "Pi session deadline task did not join cleanly",
            ),
        },
        None => CleanupOutcome::NotApplicable,
    };
    CleanupOutcome::merge(deadline, active.attachment.release().await)
}

#[cfg(test)]
#[path = "session/tests.rs"]
mod tests;

fn spawn_deadline(
    services: &HostServices,
    connection: Arc<PiConnection>,
    turn: Arc<ActiveTurn>,
    deadline: swallowtail_runtime::Deadline,
    arm: futures_channel::oneshot::Receiver<()>,
) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
    let time = services.time().cloned().expect("validated Pi time service");
    let mut wait = time.wait_until(deadline);
    let mut finished = Box::pin(turn.finished_future());
    let deadline_turn = Arc::clone(&turn);
    let deadline_services = services.clone();
    let scope =
        ScopeId::new(format!("pi-rpc:deadline:{}", turn.runtime_id().as_str())).map_err(|_| {
            failure(
                "swallowtail.pi.rpc.scope_invalid",
                "Pi RPC scope was invalid",
            )
        })?;
    services.task().expect("validated Pi task service").spawn(
        scope,
        Box::pin(async move {
            if arm.await.is_err() {
                return;
            }
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
                let abort = run_before_deadline(
                    connection.abort_command(
                        id.clone(),
                        json!({"id": id, "type": "abort"}),
                        Arc::clone(&deadline_turn),
                    ),
                    grace_deadline(&deadline_services, ABORT_ACKNOWLEDGEMENT_GRACE_TICKS),
                    &deadline_services,
                    "swallowtail.pi.rpc.deadline_abort_timed_out",
                    "Pi RPC deadline abort timed out awaiting acknowledgement",
                )
                .await;
                if !matches!(abort, Ok(response) if response.success) {
                    let diagnostic = swallowtail_core::SafeDiagnostic::new(
                        "swallowtail.pi.rpc.deadline_abort_failed",
                        "Pi RPC could not abort the turn after its deadline elapsed",
                    );
                    if connection.close_on_rejected_abort(&deadline_turn, diagnostic) {
                        let _ = connection.force_stop().await;
                    }
                }
            }
        }),
    )
}

async fn reap_finished(active: &ActiveSlot) -> Result<(), RuntimeFailure> {
    let finished = {
        let mut active = active.lock().expect("Pi active-task lock poisoned");
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
                "swallowtail.pi.rpc.attachment_cleanup_failed",
                "Pi RPC attachment cleanup failed",
            ));
        }
    }
    Ok(())
}
