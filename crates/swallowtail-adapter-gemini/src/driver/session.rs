type ActiveSlot = Arc<Mutex<Option<ActiveTask>>>;

struct ActiveTask {
    turn: Arc<ActiveTurn>,
    task: Option<Box<dyn JoinedTask>>,
}

struct GeminiSessionHandle {
    request_id: RequestId,
    runtime_id: RuntimeSessionId,
    provider_ref: SessionRef,
    provider_id: String,
    execution_host_id: swallowtail_core::ExecutionHostId,
    connection: Arc<AcpConnection>,
    cancellation: SessionCancellation,
    pump_task: Option<Box<dyn JoinedTask>>,
    services: HostServices,
    resource: Option<ResourceLease>,
    active: ActiveSlot,
}

impl InteractiveSessionHandle for GeminiSessionHandle {
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
            validate_turn(&request)?;
            reap_finished(&self.active).await?;
            if self
                .active
                .lock()
                .expect("ACP active-task lock poisoned")
                .is_some()
            {
                return Err(failure(
                    "swallowtail.gemini.acp.turn_active",
                    "Gemini CLI session already has an active turn",
                ));
            }
            let (turn, events, terminal) =
                ActiveTurn::new(request.turn_id().clone(), self.provider_id.clone())?;
            self.connection.set_active_turn(Arc::clone(&turn))?;
            let connection = Arc::clone(&self.connection);
            let prompt_turn = Arc::clone(&turn);
            let prompt = request.content().as_str().to_owned();
            let session_id = self.provider_id.clone();
            let task_service = services.task().cloned().ok_or_else(|| {
                failure(
                    "swallowtail.gemini.acp.task_service_missing",
                    "Gemini ACP requires a scoped task service",
                )
            })?;
            let scope = ScopeId::new(format!("gemini-acp:turn:{}", request.turn_id().as_str()))
                .map_err(|_| malformed())?;
            let response = match connection
                .begin_request(
                    "session/prompt",
                    json!({
                        "sessionId": session_id,
                        "prompt": [{"type": "text", "text": prompt}]
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
            let task = match task_service.spawn(
                scope,
                Box::pin(async move {
                    let response = response.await;
                    match response {
                        Ok(response) => match response.get("stopReason").and_then(Value::as_str) {
                            Some(reason) => prompt_turn.finish_prompt(reason),
                            None => prompt_turn.fail(&malformed()),
                        },
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
            *self.active.lock().expect("ACP active-task lock poisoned") = Some(ActiveTask {
                turn: Arc::clone(&turn),
                task: Some(task),
            });
            Ok(Box::new(GeminiTurnHandle {
                runtime_id: request.turn_id().clone(),
                events: Some(events),
                terminal: Some(Box::pin(terminal)),
                cancellation: TurnCancellation {
                    connection: Arc::clone(&self.connection),
                    session_id: self.provider_id.clone(),
                    turn: Arc::clone(&turn),
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
            let active = self
                .active
                .lock()
                .expect("ACP active-task lock poisoned")
                .take();
            if let Some(mut active) = active {
                if !active.turn.is_finished() {
                    active.turn.mark_cancelled();
                    let _ = self
                        .connection
                        .notify("session/cancel", json!({"sessionId": self.provider_id}))
                        .await;
                }
                self.connection.begin_close().await;
                if let Some(task) = active.task.take() {
                    let _ = task.join().await;
                }
            } else {
                self.connection.begin_close().await;
            }
            let task_cleanup = match self.pump_task.take() {
                Some(task) => match task.join().await {
                    Ok(()) => self.connection.cleanup_outcome(),
                    Err(_) => CleanupOutcome::Failed(swallowtail_core::SafeDiagnostic::new(
                        "swallowtail.gemini.acp.task_join_failed",
                        "Gemini ACP protocol task did not join cleanly",
                    )),
                },
                None => CleanupOutcome::NotApplicable,
            };
            let resource_cleanup = match self.resource.take() {
                Some(resource) => match self.services.working_resource() {
                    Some(service) => service.release(resource).await,
                    None => CleanupOutcome::Failed(swallowtail_core::SafeDiagnostic::new(
                        "swallowtail.gemini.acp.resource_release_failed",
                        "Gemini ACP working-resource service disappeared during cleanup",
                    )),
                },
                None => CleanupOutcome::NotApplicable,
            };
            merge_cleanup(task_cleanup, resource_cleanup)
        })
    }
}

fn validate_turn(request: &TurnRequest) -> Result<(), RuntimeFailure> {
    if request.deadline().is_some() {
        return Err(unsupported("turn deadline"));
    }
    if request.attachments().len() != 0 {
        return Err(unsupported("turn attachments"));
    }
    if request.structured_output().is_some() {
        return Err(unsupported("structured output"));
    }
    Ok(())
}

async fn reap_finished(active: &ActiveSlot) -> Result<(), RuntimeFailure> {
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
    if let Some(mut finished) = finished
        && let Some(task) = finished.task.take()
    {
        task.join().await?;
    }
    Ok(())
}

fn merge_cleanup(left: CleanupOutcome, right: CleanupOutcome) -> CleanupOutcome {
    match (left, right) {
        (CleanupOutcome::Failed(error), _) | (_, CleanupOutcome::Failed(error)) => {
            CleanupOutcome::Failed(error)
        }
        (CleanupOutcome::Degraded(error), _) | (_, CleanupOutcome::Degraded(error)) => {
            CleanupOutcome::Degraded(error)
        }
        (CleanupOutcome::Clean, CleanupOutcome::Clean | CleanupOutcome::NotApplicable)
        | (CleanupOutcome::NotApplicable, CleanupOutcome::Clean) => CleanupOutcome::Clean,
        (CleanupOutcome::NotApplicable, CleanupOutcome::NotApplicable) => {
            CleanupOutcome::NotApplicable
        }
    }
}
