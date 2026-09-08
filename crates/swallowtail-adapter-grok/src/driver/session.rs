type ActiveSlot = Arc<Mutex<Option<ActiveTask>>>;

struct ActiveTask {
    turn: Arc<ActiveTurn>,
    task: Option<Box<dyn JoinedTask>>,
}

struct GrokSessionHandle {
    request_id: RequestId,
    runtime_id: RuntimeSessionId,
    provider_ref: SessionRef,
    provider_id: String,
    binding: SessionResumeBinding,
    model_options: NegotiatedSessionModelOptions,
    permission_handling: crate::GrokPermissionHandling,
    execution_host_id: swallowtail_core::ExecutionHostId,
    connection: Arc<AcpConnection>,
    cancellation: SessionCancellation,
    pump_task: Option<Box<dyn JoinedTask>>,
    services: HostServices,
    resource: Option<ResourceLease>,
    credential: Option<CredentialLease>,
    registered: Option<Arc<crate::registered_tool::GrokRegisteredToolSession>>,
    active: ActiveSlot,
}

impl InteractiveSessionHandle for GrokSessionHandle {
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
        Some(&self.binding)
    }

    fn negotiated_model_options(&self) -> Option<&NegotiatedSessionModelOptions> {
        Some(&self.model_options)
    }

    fn start_turn<'a>(
        &'a mut self,
        request: TurnRequest,
        services: HostServices,
    ) -> BoxFuture<'a, Result<Box<dyn TurnHandle>, RuntimeFailure>> {
        Box::pin(async move {
            services.require_execution_host(&self.execution_host_id)?;
            validate_turn(&request, &services)?;
            // One active provider turn per server lease. Grok spawns the
            // courier at session setup, so while that lease is live only the
            // turn it was bound to may start; any other turn would run beside
            // a connected courier under a binding its own turn never made.
            if let Some(registered) = self.registered.as_ref()
                && !registered.settled_clean()
            {
                // A lease that failed to settle is still held by the host with
                // work it could not join, so the operation is not over and no
                // turn may follow it.
                if registered.cleanup_failed() {
                    return Err(failure(
                        "swallowtail.grok.acp.registered_tool.turn_retained",
                        "Grok Build ACP cannot start a turn while a registered lease it could not join is retained",
                    ));
                }
                if request.turn_id() != registered.turn() {
                    return Err(failure(
                        "swallowtail.grok.acp.registered_tool.turn_mismatch",
                        "Grok Build ACP admits only the bound registered turn while its registered lease is live",
                    ));
                }
            }
            reap_finished(&self.active).await?;
            if self
                .active
                .lock()
                .expect("ACP active-task lock poisoned")
                .is_some()
            {
                return Err(failure(
                    "swallowtail.grok.acp.turn_active",
                    "Grok Build session already has an active turn",
                ));
            }
            let (turn, events, callbacks, terminal) = ActiveTurn::new(
                request.turn_id().clone(),
                self.provider_id.clone(),
                request.deadline(),
                matches!(
                    self.permission_handling,
                    crate::GrokPermissionHandling::ConsumerMediated
                ),
                Arc::downgrade(&self.connection),
            )?;
            self.connection.set_active_turn(Arc::clone(&turn))?;
            let connection = Arc::clone(&self.connection);
            let prompt_turn = Arc::clone(&turn);
            let prompt = request.content().as_str().to_owned();
            let session_id = self.provider_id.clone();
            let task_service = services.task().cloned().ok_or_else(|| {
                failure(
                    "swallowtail.grok.acp.task_service_missing",
                    "Grok Build ACP requires a scoped task service",
                )
            })?;
            let scope = ScopeId::new(format!("grok-acp:turn:{}", request.turn_id().as_str()))
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
                    // The connection survives an encode or send failure, so the
                    // courier stays connected. Settle before the attempt is
                    // reported failed, or a later call would dispatch under a
                    // turn that never started.
                    if let Some(registered) = self.registered.as_ref() {
                        let _ = registered
                            .settle(&services, RegisteredToolCleanupCause::TransportFailure)
                            .await;
                    }
                    self.connection.clear_active_turn(&turn);
                    turn.fail(&error);
                    return Err(error);
                }
            };
            let deadline = request
                .deadline()
                .map(|deadline| services.time().expect("validated time").wait_until(deadline));
            let registered_turn = self.registered.clone();
            let registered_services = services.clone();
            let task = match task_service.spawn(
                scope,
                Box::pin(async move {
                    let result = match deadline {
                        Some(mut deadline) => {
                            let mut response = Box::pin(response);
                            std::future::poll_fn(|context| {
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
                            .await
                        }
                        None => Some(response.await),
                    };
                    // Terminal, cancellation, and deadline freeze registered
                    // admission and settle the lease before the consumer sees
                    // this turn's terminal outcome, so no registered call can
                    // dispatch under a finished turn. The cleanup truth is
                    // retained on the lease and reported by session close.
                    let registered_cleanup = match registered_turn.as_ref() {
                        Some(registered) => {
                            let cause = registered_cleanup_cause(&prompt_turn, result.as_ref());
                            registered.settle(&registered_services, cause).await
                        }
                        None => CleanupOutcome::NotApplicable,
                    };
                    // A registered lease the host could not join is not a
                    // completed turn. The cleanup failure replaces the terminal
                    // status rather than being reported beside a success.
                    if let CleanupOutcome::Failed(diagnostic) = registered_cleanup {
                        prompt_turn.fail(&RuntimeFailure::new(diagnostic));
                        connection.clear_active_turn(&prompt_turn);
                        return;
                    }
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
                    connection.clear_active_turn(&prompt_turn);
                }),
            ) {
                Ok(task) => task,
                Err(error) => {
                    // No task will settle this lease, so the failed start must.
                    if let Some(registered) = self.registered.as_ref() {
                        let _ = registered
                            .settle(&services, RegisteredToolCleanupCause::ProviderFailure)
                            .await;
                    }
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
            Ok(Box::new(GrokTurnHandle {
                runtime_id: request.turn_id().clone(),
                events: Some(events),
                callbacks,
                terminal: Some(Box::pin(terminal)),
                cancellation: TurnCancellation {
                    connection: Arc::clone(&self.connection),
                    session_id: self.provider_id.clone(),
                    turn: Arc::clone(&turn),
                    requested: AtomicBool::new(false),
                    registered: self.registered.clone(),
                    services: services.clone(),
                },
                active: Arc::clone(&self.active),
            }) as Box<dyn TurnHandle>)
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
                        "swallowtail.grok.acp.task_join_failed",
                        "Grok Build ACP protocol task did not join",
                    ),
                },
                None => CleanupOutcome::NotApplicable,
            };
            // The registered lease settles before the route releases its own
            // leases. Its completion gate and joined listener teardown are the
            // Contract 063 cleanup evidence, and a failed close retains
            // ownership rather than being masked by a later clean release.
            let registered = match self.registered.as_ref() {
                Some(registered) => {
                    registered
                        .settle(&self.services, RegisteredToolCleanupCause::ExplicitClose)
                        .await
                }
                None => CleanupOutcome::NotApplicable,
            };
            if matches!(registered, CleanupOutcome::Failed(_)) {
                // A retained bridge lease means the operation is not over. The
                // working resource and the credential stay held rather than
                // being returned for reuse beside work that may still execute.
                return merge_cleanup(task, registered);
            }
            let resource = release_resource(self.resource.take(), &self.services).await;
            let credential = release_credential(self.credential.take(), &self.services).await;
                merge_cleanup(
                    merge_cleanup(merge_cleanup(task, registered), resource),
                    credential,
                )
            }),
        )
    }
}

include!("cancellation.rs");

/// Maps one turn outcome onto the exact Contract 063 cleanup cause.
fn registered_cleanup_cause(
    turn: &ActiveTurn,
    result: Option<&Result<Value, RuntimeFailure>>,
) -> RegisteredToolCleanupCause {
    if turn.was_cancelled() {
        return RegisteredToolCleanupCause::Cancellation;
    }
    match result {
        None => RegisteredToolCleanupCause::Deadline,
        Some(Err(_)) => RegisteredToolCleanupCause::TransportFailure,
        Some(Ok(response)) => match response.get("stopReason").and_then(Value::as_str) {
            Some("end_turn") => RegisteredToolCleanupCause::Completion,
            Some("cancelled") => RegisteredToolCleanupCause::Cancellation,
            _ => RegisteredToolCleanupCause::ProviderFailure,
        },
    }
}

fn finish_prompt_response(turn: &ActiveTurn, response: &Value) {
    match response.get("stopReason").and_then(Value::as_str) {
        Some(reason) => turn.finish_prompt(reason),
        None => turn.fail(&malformed()),
    }
}
