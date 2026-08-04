impl StructuredRunDriver for OpenCodeHttpDriver {
    fn start_run(
        &self,
        plan: PreflightPlan,
        request: StructuredRunRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        Box::pin(async move {
            let version = Self::validate_plan(&plan)?;
            services.require_execution_host(plan.execution_host_id())?;
            validate_run(&plan, &request, &services)?;
            let run_id =
                RuntimeRunId::new(format!("opencode:run:{}", request.request_id().as_str()))
                    .map_err(|_| invalid_run_identity())?;
            let turn_id =
                RuntimeTurnId::new(format!("opencode:run:{}", request.request_id().as_str()))
                    .map_err(|_| invalid_run_identity())?;
            let task_scope = scope("run-cleanup", request.request_id().as_str())?;
            let mut session = self
                .open_run_session(&plan, &request, &services, &version)
                .await?;
            let mut turn_request = TurnRequest::new(turn_id, request.content().clone());
            turn_request = turn_request.with_attachments(request.attachments().cloned());
            if let Some(deadline) = request.deadline() {
                turn_request = turn_request.with_deadline(deadline);
            }
            let mut turn = match session.start_turn(turn_request, services.clone()).await {
                Ok(turn) => turn,
                Err(error) => {
                    let _ = session.close_and_delete().await;
                    return Err(error);
                }
            };
            let events = match turn.take_events() {
                Some(events) => events,
                None => {
                    let _ = turn.close().await;
                    let _ = session.close_and_delete().await;
                    return Err(failure(
                        "swallowtail.opencode.run_events_missing",
                        "OpenCode structured run did not expose its event stream",
                    ));
                }
            };
            let callbacks = turn.take_callbacks();
            let terminal = match turn.take_terminal_outcome() {
                Some(terminal) => terminal,
                None => {
                    let _ = turn.close().await;
                    let _ = session.close_and_delete().await;
                    return Err(failure(
                        "swallowtail.opencode.run_terminal_missing",
                        "OpenCode structured run did not expose its terminal outcome",
                    ));
                }
            };
            let active_state = {
                let active = session.active.lock().expect("active turn lock poisoned");
                active.as_ref().map(|active| {
                    (
                        Arc::clone(&active.cancellation),
                        Arc::clone(&active.terminal),
                    )
                })
            };
            let (active_cancellation, terminal_flag) = match active_state {
                Some(state) => state,
                None => {
                    let _ = turn.close().await;
                    let _ = session.close_and_delete().await;
                    return Err(failure(
                        "swallowtail.opencode.run_active_missing",
                        "OpenCode structured run lost its active prompt",
                    ));
                }
            };
            let cancellation = Arc::new(OpenCodeRunCancellation {
                inner: active_cancellation,
                terminal: terminal_flag,
                requested: AtomicBool::new(false),
            });
            let pending = Arc::new(Mutex::new(Some(OpenCodeRunResources {
                turn,
                session,
                terminal,
            })));
            let (terminal_sender, terminal) = terminal_outcome_channel();
            let task_pending = Arc::clone(&pending);
            let task = services.task().expect("validated task service").spawn(
                task_scope,
                Box::pin(async move {
                    let resources = task_pending
                        .lock()
                        .expect("OpenCode pending run lock poisoned")
                        .take()
                        .expect("OpenCode pending run exists");
                    let outcome = resources.terminal.await;
                    let turn_cleanup = resources.turn.close().await;
                    let session_cleanup = resources.session.close_and_delete().await;
                    let cleanup = merge_cleanup(
                        outcome.cleanup().clone(),
                        merge_cleanup(turn_cleanup, session_cleanup.cleanup),
                    );
                    let mut finished = copy_terminal_outcome(outcome, cleanup);
                    finished = finished.with_remote_resource_deletion(
                        OwnedRemoteResourceKind::Session,
                        session_cleanup.deletion,
                    );
                    let _ = terminal_sender.complete(finished);
                }),
            );
            let task = match task {
                Ok(task) => task,
                Err(error) => {
                    let _ = cancellation.request().await;
                    let resources = pending
                        .lock()
                        .expect("OpenCode pending run lock poisoned")
                        .take();
                    if let Some(resources) = resources {
                        let _ = resources.turn.close().await;
                        let _ = resources.session.close_and_delete().await;
                    }
                    return Err(error);
                }
            };
            Ok(Box::new(OpenCodeRunHandle {
                request_id: request.request_id().clone(),
                run_id,
                events: Some(events),
                callbacks,
                terminal: Some(Box::pin(terminal)),
                cancellation,
                task,
            }) as Box<dyn RunHandle>)
        })
    }
}

impl OpenCodeHttpDriver {
    async fn open_run_session(
        &self,
        plan: &PreflightPlan,
        request: &StructuredRunRequest,
        services: &HostServices,
        version: &OpenCodePlanVersion,
    ) -> Result<OpenCodeSessionHandle, RuntimeFailure> {
        let runtime_id =
            RuntimeSessionId::new(format!("opencode:run:{}", request.request_id().as_str()))
                .map_err(|_| invalid_run_identity())?;
        let provider_id = plan.provider_id().cloned().ok_or_else(|| {
            failure(
                "swallowtail.opencode.provider_missing",
                "OpenCode run requires a preflight-bound provider",
            )
        })?;
        let model_id = plan.model_id().cloned().ok_or_else(|| {
            failure(
                "swallowtail.opencode.model_missing",
                "OpenCode run requires a preflight-bound model",
            )
        })?;
        let model_route_id = plan.model_route_id().cloned().ok_or_else(|| {
            failure(
                "swallowtail.opencode.model_route_missing",
                "OpenCode run requires a preflight-bound model route",
            )
        })?;
        let working_resource = request
            .working_resource()
            .expect("validated OpenCode run resource");
        let callback_enabled = provider_callbacks(plan)?;
        let policy = if callback_enabled {
            SessionAccessPolicy::ambient_harness_with_consumer_mediated_requests(
                ResourceAccess::ReadWrite,
                [
                    callback::permission_namespace(),
                    callback::question_namespace(),
                ],
            )
        } else {
            SessionAccessPolicy::ambient_harness(ResourceAccess::Read)
        };
        let operation_scope = scope("run", request.request_id().as_str())?;
        let mut access = AccessLeases::acquire(
            plan,
            operation_scope.clone(),
            services,
            Some((working_resource, &policy)),
        )
        .await?;
        let directory = access
            .directory
            .clone()
            .expect("OpenCode run resource was acquired");
        let cancelled = Arc::new(AtomicBool::new(false));
        let image_attachments = validate_attachment_plan(plan, services)?;
        let open = async {
            let health = complete_before_deadline(
                self.transport.request(
                    operation_scope.clone(),
                    access.endpoint.clone(),
                    Request::get("/global/health"),
                    services,
                    Arc::clone(&cancelled),
                ),
                request.deadline(),
                services,
                Arc::clone(&cancelled),
                "swallowtail.opencode.run_timed_out",
                "OpenCode structured run timed out during health validation",
            )
            .await?;
            require_health_matches(&health, version)?;
            let response = complete_before_deadline(
                self.transport.request(
                    operation_scope,
                    access.endpoint.clone(),
                    session_create(
                        provider_id.as_str(),
                        model_id.as_str(),
                        &directory,
                        callback_enabled,
                    ),
                    services,
                    Arc::clone(&cancelled),
                ),
                request.deadline(),
                services,
                cancelled,
                "swallowtail.opencode.run_timed_out",
                "OpenCode structured run timed out during session creation",
            )
            .await?;
            parse_session_for_version(&response, version.binding())
        }
        .await;
        let provider_session_id = match open {
            Ok(id) => id,
            Err(error) => {
                let _ = access.release(services).await;
                return Err(error);
            }
        };
        let provider_ref = SessionRef::new(&provider_session_id).map_err(|_| {
            failure(
                "swallowtail.opencode.session_invalid",
                "OpenCode returned an invalid session identity",
            )
        })?;
        let resume_binding = SessionResumeBinding::new(
            provider_ref,
            plan.instance_id().clone(),
            plan.execution_host_id().clone(),
            model_route_id,
            model_id.clone(),
            working_resource.clone(),
            policy,
        );
        let active = Arc::new(Mutex::new(None));
        Ok(OpenCodeSessionHandle {
            request_id: request.request_id().clone(),
            runtime_id,
            resume_binding,
            provider_id,
            model_id,
            provider_session_id,
            directory,
            endpoint: access.endpoint.clone(),
            services: services.clone(),
            transport: self.transport.clone(),
            access: Some(access),
            active: Arc::clone(&active),
            cancellation: SessionCancellation::new(active),
            reasoning_mode: request.policy().reasoning_mode().cloned(),
            structured_output: request.structured_output().cloned(),
            image_attachments,
            provider_callbacks: callback_enabled,
            active_turn_detachment: false,
            callback_run_id: Some(
                RuntimeRunId::new(format!(
                    "opencode:run:{}",
                    request.request_id().as_str()
                ))
                .map_err(|_| invalid_run_identity())?,
            ),
        })
    }
}
