impl InteractiveSessionHandle for CodexSessionHandle {
    fn request_id(&self) -> &RequestId {
        &self.request_id
    }

    fn session_id(&self) -> &RuntimeSessionId {
        &self.runtime_id
    }

    fn provider_session_ref(&self) -> Option<&SessionRef> {
        Some(self.resume_binding.provider_session_ref())
    }

    fn resume_binding(&self) -> Option<&SessionResumeBinding> {
        Some(&self.resume_binding)
    }

    fn start_turn<'a>(
        &'a mut self,
        request: TurnRequest,
        services: HostServices,
    ) -> BoxFuture<'a, Result<Box<dyn TurnHandle>, RuntimeFailure>> {
        Box::pin(async move {
            services.require_execution_host(self.resume_binding.execution_host_id())?;
            validate_turn_request(&request, self.runtime.deadline_planned, &services)?;
            let runtime_id = request.turn_id().clone();
            let provider_thread_id = self
                .resume_binding
                .provider_session_ref()
                .as_provider_value()
                .to_owned();
            let (turn, events, callbacks, terminal) = ActiveTurn::new(
                runtime_id.clone(),
                request.deadline(),
                self.runtime.declared_tools.clone(),
                self.runtime.provider_requests.clone(),
                provider_thread_id.clone(),
                Arc::downgrade(&self.connection),
            )?;
            let exposes_callbacks = !self.runtime.declared_tools.is_empty()
                || self.runtime.provider_requests.observed_extensions().len() != 0
                || self.runtime.provider_requests.exchanged_extensions().len() != 0;
            let callbacks = exposes_callbacks.then_some(callbacks);
            self.connection.set_active_turn(Arc::clone(&turn))?;
            let mut params = serde_json::json!({
                "threadId": provider_thread_id,
                "input": [{"type": "text", "text": request.content().as_str()}]
            });
            if let Some(effort) = &self.runtime.reasoning_effort {
                params
                    .as_object_mut()
                    .expect("static turn/start parameters are an object")
                    .insert("effort".to_owned(), Value::String(effort.clone()));
            }
            if let Some(mode) = &self.runtime.collaboration_mode {
                params
                    .as_object_mut()
                    .expect("static turn/start parameters are an object")
                    .insert("collaborationMode".to_owned(), mode.clone());
            }
            if let Some(policy) = &self.runtime.turn_sandbox_policy {
                params
                    .as_object_mut()
                    .expect("static turn/start parameters are an object")
                    .insert("sandboxPolicy".to_owned(), policy.clone());
            }
            let response = self.connection.request("turn/start", params).await;
            let response = match response {
                Ok(response) => response,
                Err(error) => {
                    self.connection.clear_active_turn(&turn);
                    turn.finish(
                        TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                        CleanupOutcome::NotApplicable,
                    );
                    return Err(error);
                }
            };
            let provider_id = response
                .get("turn")
                .and_then(|turn| turn.get("id"))
                .and_then(Value::as_str)
                .map(str::to_owned);
            let provider_id = match provider_id {
                Some(provider_id) => provider_id,
                None => {
                    self.connection.clear_active_turn(&turn);
                    let error = malformed_notification();
                    turn.finish(
                        TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                        CleanupOutcome::NotApplicable,
                    );
                    return Err(error);
                }
            };
            if let Err(error) = turn.set_provider_id(&provider_id) {
                self.connection.clear_active_turn(&turn);
                turn.finish(
                    TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                    CleanupOutcome::NotApplicable,
                );
                return Err(error);
            }
            let provider_ref = match TurnRef::new(&provider_id) {
                Ok(provider_ref) => provider_ref,
                Err(_) => {
                    self.connection.clear_active_turn(&turn);
                    let error = malformed_notification();
                    turn.finish(
                        TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                        CleanupOutcome::NotApplicable,
                    );
                    return Err(error);
                }
            };
            let deadline_task = match start_deadline_task(
                &request,
                &services,
                Arc::clone(&turn),
                Arc::clone(&self.connection),
                provider_thread_id.clone(),
                provider_id.clone(),
            ) {
                Ok(task) => task,
                Err(error) => {
                    turn.mark_cancelled();
                    let _ = self
                        .connection
                        .request_without_waiting(
                            "turn/interrupt",
                            serde_json::json!({
                                "threadId": provider_thread_id,
                                "turnId": provider_id
                            }),
                        )
                        .await;
                    turn.finish(
                        TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                        CleanupOutcome::NotApplicable,
                    );
                    self.connection.clear_active_turn(&turn);
                    return Err(error);
                }
            };
            Ok(Box::new(CodexTurnHandle {
                runtime_id,
                provider_ref,
                events: Some(events),
                callbacks,
                terminal: Some(Box::pin(terminal)),
                cancellation: TurnCancellation {
                    connection: Arc::clone(&self.connection),
                    thread_id: provider_thread_id,
                    turn_id: provider_id,
                    turn,
                    requested: AtomicBool::new(false),
                },
                deadline_task,
            }) as Box<dyn TurnHandle>)
        })
    }

    fn cancellation(&self) -> &dyn CancellationControl {
        &self.cancellation
    }

    fn close(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome> {
        Box::pin(async move {
            let close = self.connection.close_input().await;
            let join = self.task.join().await;
            let process_cleanup = if close.is_err() || join.is_err() {
                CleanupOutcome::Failed(swallowtail_core::SafeDiagnostic::new(
                    "swallowtail.codex.app_server.session_close_failed",
                    "Codex app-server session cleanup failed",
                ))
            } else {
                self.connection.cleanup_outcome()
            };
            let resource_cleanup = self.access.release().await;
            merge_cleanup(process_cleanup, resource_cleanup)
        })
    }
}

fn merge_cleanup(current: CleanupOutcome, next: CleanupOutcome) -> CleanupOutcome {
    match (&current, &next) {
        (CleanupOutcome::Failed(_), _) => current,
        (_, CleanupOutcome::Failed(_)) => next,
        (CleanupOutcome::Degraded(_), _) => current,
        (_, CleanupOutcome::Degraded(_)) => next,
        (CleanupOutcome::Clean, CleanupOutcome::NotApplicable) => current,
        (CleanupOutcome::NotApplicable, CleanupOutcome::Clean) => next,
        _ => current,
    }
}

fn validate_turn_request(
    request: &TurnRequest,
    deadline_planned: bool,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    if request.deadline().is_some()
        && (!deadline_planned || services.time().is_none() || services.task().is_none())
    {
        return Err(failure(
            "swallowtail.codex.app_server.preflight_mismatch",
            "Codex turn deadline requires preflight-bound task and time services",
        ));
    }
    if let Some(deadline) = request.deadline()
        && services
            .time()
            .is_some_and(|time| time.now() >= deadline.instant())
    {
        return Err(failure(
            "swallowtail.codex.app_server.deadline_elapsed",
            "Codex turn deadline elapsed before provider work",
        ));
    }
    if request.attachments().len() != 0 {
        return Err(unsupported("turn attachments"));
    }
    if request.structured_output().is_some() {
        return Err(unsupported("structured turn output"));
    }
    Ok(())
}

fn start_deadline_task(
    request: &TurnRequest,
    services: &HostServices,
    turn: Arc<ActiveTurn>,
    connection: Arc<RpcConnection>,
    thread_id: String,
    provider_turn_id: String,
) -> Result<Option<Box<dyn JoinedTask>>, RuntimeFailure> {
    let Some(deadline) = request.deadline() else {
        return Ok(None);
    };
    let time = services.time().expect("deadline services were validated");
    let wait = time.wait_until(deadline);
    let finished = Box::pin(turn.finished_future());
    let scope = swallowtail_runtime::ScopeId::new(format!(
        "codex-app-server:turn:{}",
        request.turn_id().as_str()
    ))
    .expect("runtime turn id produces a valid scope id");
    let task = services
        .task()
        .expect("deadline services were validated")
        .spawn(
            scope,
            Box::pin(async move {
                if let Either::Left((_observation, _finished)) = select(wait, finished).await {
                    if turn.is_finished() {
                        return;
                    }
                    turn.mark_timed_out();
                    let _ = connection
                        .reject_abandoned_callbacks(turn.take_abandoned_provider_requests())
                        .await;
                    let _ = connection
                        .request_without_waiting(
                            "turn/interrupt",
                            serde_json::json!({
                                "threadId": thread_id,
                                "turnId": provider_turn_id
                            }),
                        )
                        .await;
                    turn.finish(TerminalStatus::TimedOut, CleanupOutcome::NotApplicable);
                    connection.clear_active_turn(&turn);
                }
            }),
        )?;
    Ok(Some(task))
}

fn unsupported(feature: &str) -> RuntimeFailure {
    failure(
        "swallowtail.codex.app_server.unsupported_input",
        format!("Codex app-server proof driver does not support {feature}"),
    )
}
