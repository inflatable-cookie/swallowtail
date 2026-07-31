type ActiveSlot = Arc<Mutex<Option<ActiveTask>>>;

struct ActiveTask {
    turn: Arc<ActiveTurn>,
    task: Option<Box<dyn JoinedTask>>,
}

struct CursorSessionHandle {
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

impl InteractiveSessionHandle for CursorSessionHandle {
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

    fn negotiated_model_options(
        &self,
    ) -> Option<&swallowtail_runtime::NegotiatedSessionModelOptions> {
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
                    "swallowtail.cursor.acp.turn_active",
                    "Cursor Agent session already has an active turn",
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
                    "swallowtail.cursor.acp.task_service_missing",
                    "Cursor Agent ACP requires a scoped task service",
                )
            })?;
            let scope = ScopeId::new(format!("cursor-acp:turn:{}", request.turn_id().as_str()))
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
            let deadline = request
                .deadline()
                .map(|deadline| services.time().expect("validated time").wait_until(deadline));
            let task = match task_service.spawn(
                scope,
                Box::pin(async move {
                    if let Some(mut deadline) = deadline {
                        let mut response = Box::pin(response);
                        let result = std::future::poll_fn(|context| {
                            use std::future::Future;
                            use std::task::Poll;
                            if let Poll::Ready(response) = response.as_mut().poll(context) {
                                Poll::Ready(Some(response))
                            } else if deadline.as_mut().poll(context).is_ready() {
                                Poll::Ready(None)
                            } else {
                                Poll::Pending
                            }
                        })
                        .await;
                        match result {
                            Some(Ok(response)) => finish_prompt_response(&prompt_turn, &response),
                            Some(Err(error)) => prompt_turn.fail(&error),
                            None => {
                                prompt_turn.timeout();
                                let _ = connection
                                    .notify(
                                        "session/cancel",
                                        json!({"sessionId": prompt_turn.session_id()}),
                                    )
                                    .await;
                            }
                        }
                    } else {
                        match response.await {
                            Ok(response) => finish_prompt_response(&prompt_turn, &response),
                            Err(error) => prompt_turn.fail(&error),
                        }
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
            Ok(Box::new(CursorTurnHandle {
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
            let task = match self.pump_task.take() {
                Some(task) => match task.join().await {
                    Ok(()) => self.connection.cleanup_outcome(),
                    Err(_) => cleanup_failure(
                        "swallowtail.cursor.acp.task_join_failed",
                        "Cursor Agent ACP protocol task did not join",
                    ),
                },
                None => CleanupOutcome::NotApplicable,
            };
            let resource = release_resource(self.resource.take(), &self.services).await;
            merge_cleanup(task, resource)
        })
    }
}

include!("cancellation.rs");

fn finish_prompt_response(turn: &ActiveTurn, response: &Value) {
    match response.get("stopReason").and_then(Value::as_str) {
        Some(reason) => turn.finish_prompt(reason),
        None => turn.fail(&malformed()),
    }
}
