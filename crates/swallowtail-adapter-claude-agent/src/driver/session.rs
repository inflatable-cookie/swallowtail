use super::*;
use crate::driver::access::{release_credential, release_resource};
use crate::driver::handle::{ClaudeAgentTurnHandle, SessionCancellation, TurnCancellation};
use crate::driver::validation::validate_turn;

mod cleanup;
mod deadline;

pub(in crate::driver) use cleanup::{cleanup_failure, merge_cleanup};
use cleanup::{close_provider_session, finish_cleanup, join_connection};
use deadline::spawn_deadline;

pub(super) type ActiveSlot = Arc<Mutex<Option<ActiveTask>>>;

pub(super) struct ActiveTask {
    pub(super) turn: Arc<ActiveTurn>,
    prompt_task: Option<Box<dyn JoinedTask>>,
    deadline_task: Option<Box<dyn JoinedTask>>,
}

pub(super) struct ClaudeAgentSessionHandle {
    pub(super) request_id: RequestId,
    pub(super) runtime_id: RuntimeSessionId,
    pub(super) provider_ref: SessionRef,
    pub(super) provider_id: String,
    pub(super) execution_host_id: swallowtail_core::ExecutionHostId,
    pub(super) native_close: bool,
    pub(super) provider_requests: swallowtail_core::ProviderRequestPolicy,
    pub(super) connection: Arc<AcpConnection>,
    pub(super) cancellation: SessionCancellation,
    pub(super) pump_task: Option<Box<dyn JoinedTask>>,
    pub(super) services: HostServices,
    pub(super) resource: Option<ResourceLease>,
    pub(super) credential: Option<CredentialLease>,
    pub(super) active: ActiveSlot,
}

impl InteractiveSessionHandle for ClaudeAgentSessionHandle {
    fn request_id(&self) -> &RequestId {
        &self.request_id
    }

    fn session_id(&self) -> &RuntimeSessionId {
        &self.runtime_id
    }

    fn provider_session_ref(&self) -> Option<&SessionRef> {
        Some(&self.provider_ref)
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
            validate_turn(&request, &services)?;
            reap_finished(&self.active).await?;
            if self
                .active
                .lock()
                .expect("ACP active-task lock poisoned")
                .is_some()
            {
                return Err(failure(
                    "swallowtail.claude_agent.acp.turn_active",
                    "Claude Agent session already has an active turn",
                ));
            }
            let (turn, events, callbacks, terminal) = ActiveTurn::new(
                request.turn_id().clone(),
                self.provider_id.clone(),
                request.deadline(),
                &self.provider_requests,
                Arc::downgrade(&self.connection),
            )?;
            self.connection.set_active_turn(Arc::clone(&turn))?;
            let response = match self
                .connection
                .begin_request(
                    "session/prompt",
                    json!({
                        "sessionId": self.provider_id,
                        "prompt": [{"type": "text", "text": request.content().as_str()}]
                    }),
                )
                .await
            {
                Ok(response) => response,
                Err(error) => {
                    self.connection.clear_active_turn(&turn);
                    turn.fail(&error);
                    return Err(error);
                }
            };
            let connection = Arc::clone(&self.connection);
            let prompt_turn = Arc::clone(&turn);
            let prompt_scope = ScopeId::new(format!(
                "claude-agent-acp:turn:{}",
                request.turn_id().as_str()
            ))
            .map_err(|_| malformed())?;
            let prompt_task = match services.task().expect("validated task service").spawn(
                prompt_scope,
                Box::pin(async move {
                    match response.await {
                        Ok(response) => prompt_turn.finish_prompt(&response),
                        Err(error) => prompt_turn.fail(&error),
                    }
                    connection.clear_active_turn(&prompt_turn);
                }),
            ) {
                Ok(task) => task,
                Err(error) => {
                    self.connection.clear_active_turn(&turn);
                    turn.fail(&error);
                    let _ = self.connection.cancel_session().await;
                    return Err(error);
                }
            };
            let deadline_task = match spawn_deadline(
                &services,
                Arc::clone(&self.connection),
                Arc::clone(&turn),
                request.deadline(),
            ) {
                Ok(task) => task,
                Err(error) => {
                    let _ = self.connection.cancel_session().await;
                    let _ = prompt_task.join().await;
                    self.connection.clear_active_turn(&turn);
                    turn.fail(&error);
                    return Err(error);
                }
            };
            *self.active.lock().expect("ACP active-task lock poisoned") = Some(ActiveTask {
                turn: Arc::clone(&turn),
                prompt_task: Some(prompt_task),
                deadline_task,
            });
            Ok(Box::new(ClaudeAgentTurnHandle {
                runtime_id: request.turn_id().clone(),
                events: Some(events),
                callbacks,
                terminal: Some(Box::pin(terminal)),
                cancellation: TurnCancellation {
                    connection: Arc::clone(&self.connection),
                    session_id: self.provider_id.clone(),
                    turn,
                    requested: AtomicBool::new(false),
                },
                active: Arc::clone(&self.active),
            }) as Box<dyn TurnHandle>)
        })
    }

    fn cancellation(&self) -> &dyn CancellationControl {
        &self.cancellation
    }

    fn close(mut self: Box<Self>) -> BoxFuture<'static, CleanupOutcome> {
        Box::pin(async move {
            let mut active = self
                .active
                .lock()
                .expect("ACP active-task lock poisoned")
                .take();
            if let Some(active) = active.as_mut()
                && !active.turn.is_finished()
            {
                active.turn.mark_cancelled();
            }
            let native_close =
                close_provider_session(&self.connection, &self.provider_id, self.native_close)
                    .await;
            self.connection.begin_close().await;
            if let Some(active) = active.as_mut() {
                let _ = join_active(active).await;
            }
            let task = join_connection(&mut self).await;
            finish_cleanup(self, native_close, task).await
        })
    }
}

pub(super) async fn reap_finished(active: &ActiveSlot) -> Result<(), RuntimeFailure> {
    let finished = {
        let mut active = active.lock().expect("ACP active-task lock poisoned");
        if active
            .as_ref()
            .is_some_and(|active| active.turn.is_finished())
        {
            active.take()
        } else {
            None
        }
    };
    if let Some(mut finished) = finished {
        join_active(&mut finished).await?;
    }
    Ok(())
}

pub(super) async fn join_active(active: &mut ActiveTask) -> Result<(), RuntimeFailure> {
    if let Some(task) = active.prompt_task.take() {
        task.join().await?;
    }
    if let Some(task) = active.deadline_task.take() {
        task.join().await?;
    }
    Ok(())
}
