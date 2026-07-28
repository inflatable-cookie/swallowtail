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
        let policy = SessionAccessPolicy::ambient_harness(ResourceAccess::Read);
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
                    session_create(provider_id.as_str(), model_id.as_str(), &directory),
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
        })
    }
}

struct OpenCodeRunResources {
    turn: Box<dyn TurnHandle>,
    session: OpenCodeSessionHandle,
    terminal: BoxFuture<'static, TerminalOutcome>,
}

struct OpenCodeRunCancellation {
    inner: Arc<TurnCancellation>,
    terminal: Arc<AtomicBool>,
    requested: AtomicBool,
}

impl CancellationControl for OpenCodeRunCancellation {
    fn scope(&self) -> CancellationScope {
        CancellationScope::StructuredRun
    }

    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        let already = self.requested.swap(true, Ordering::SeqCst)
            || self.terminal.load(Ordering::SeqCst);
        Box::pin(async move {
            if already {
                Ok(CancellationAcknowledgement::AlreadyRequested)
            } else {
                self.inner.request().await
            }
        })
    }
}

struct OpenCodeRunHandle {
    request_id: RequestId,
    run_id: RuntimeRunId,
    events: Option<BoxEventStream>,
    terminal: Option<BoxFuture<'static, TerminalOutcome>>,
    cancellation: Arc<OpenCodeRunCancellation>,
    task: Box<dyn JoinedTask>,
}

impl RunHandle for OpenCodeRunHandle {
    fn request_id(&self) -> &RequestId {
        &self.request_id
    }

    fn run_id(&self) -> &RuntimeRunId {
        &self.run_id
    }

    fn provider_run_ref(&self) -> Option<&RunRef> {
        None
    }

    fn take_events(&mut self) -> Option<BoxEventStream> {
        self.events.take()
    }

    fn cancellation(&self) -> &dyn CancellationControl {
        self.cancellation.as_ref()
    }

    fn take_terminal_outcome(&mut self) -> Option<BoxFuture<'static, TerminalOutcome>> {
        self.terminal.take()
    }

    fn close(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome> {
        Box::pin(async move {
            let _ = self.cancellation.request().await;
            self.task.join().await.map_or_else(
                |_| {
                    CleanupOutcome::Failed(SafeDiagnostic::new(
                        "swallowtail.opencode.run_join_failed",
                        "OpenCode structured-run cleanup task did not join",
                    ))
                },
                |_| CleanupOutcome::Clean,
            )
        })
    }
}

struct CreatedSessionCleanup {
    cleanup: CleanupOutcome,
    deletion: RemoteResourceDeletionOutcome,
}

impl OpenCodeSessionHandle {
    async fn close_and_delete(mut self) -> CreatedSessionCleanup {
        let active_cleanup = close_active(&self.active).await;
        let delete_scope = scope("run-delete", self.request_id.as_str());
        let deletion = match delete_scope {
            Ok(delete_scope) => match session_delete(&self.provider_session_id, &self.directory) {
                Ok(request) => {
                    let cancelled = Arc::new(AtomicBool::new(false));
                    match self
                        .transport
                        .request(
                            delete_scope,
                            self.endpoint.clone(),
                            request,
                            &self.services,
                            cancelled,
                        )
                        .await
                    {
                        Ok(response) => match classify_session_delete(&response) {
                            SessionDeleteResponse::Applied => (
                                CleanupOutcome::Clean,
                                RemoteResourceDeletionOutcome::Confirmed,
                            ),
                            SessionDeleteResponse::Rejected => (
                                CleanupOutcome::Failed(SafeDiagnostic::new(
                                    "swallowtail.opencode.run_delete_rejected",
                                    "OpenCode rejected deletion of its structured-run session",
                                )),
                                RemoteResourceDeletionOutcome::Unconfirmed,
                            ),
                            SessionDeleteResponse::Unconfirmed => (
                                CleanupOutcome::Failed(SafeDiagnostic::new(
                                    "swallowtail.opencode.run_delete_unconfirmed",
                                    "OpenCode did not confirm deletion of its structured-run session",
                                )),
                                RemoteResourceDeletionOutcome::Unconfirmed,
                            ),
                        },
                        Err(error) => (
                            CleanupOutcome::Failed(error.diagnostic().clone()),
                            RemoteResourceDeletionOutcome::Unconfirmed,
                        ),
                    }
                }
                Err(error) => (
                    CleanupOutcome::Failed(error.diagnostic().clone()),
                    RemoteResourceDeletionOutcome::Unconfirmed,
                ),
            },
            Err(error) => (
                CleanupOutcome::Failed(error.diagnostic().clone()),
                RemoteResourceDeletionOutcome::Unconfirmed,
            ),
        };
        let lease_cleanup = match self.access.as_mut() {
            Some(access) => access.release(&self.services).await,
            None => CleanupOutcome::NotApplicable,
        };
        CreatedSessionCleanup {
            cleanup: merge_cleanup(
                merge_cleanup(active_cleanup, deletion.0),
                lease_cleanup,
            ),
            deletion: deletion.1,
        }
    }
}

fn validate_run(
    plan: &PreflightPlan,
    request: &StructuredRunRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    require_services(services, true)?;
    if plan.requirements().execution_layer() != ExecutionLayer::HarnessInteraction
        || plan.requirements().operation_shape() != OperationShape::StructuredRun
        || plan.requirements().driver_role() != DriverRole::StructuredRun
        || plan.ownership() != InstanceOwnership::ExternalAttached
        || plan.provider_id().is_none()
        || plan.model_id().is_none()
        || plan.model_route_id().is_none()
    {
        return Err(failure(
            "swallowtail.opencode.run_plan_mismatch",
            "OpenCode structured run does not match its preflight plan",
        ));
    }
    for service in [
        HostServiceKind::Task,
        HostServiceKind::BlockingWork,
        HostServiceKind::Time,
        HostServiceKind::Network,
        HostServiceKind::Credential,
        HostServiceKind::WorkingResource,
    ] {
        if !plan
            .requirements()
            .host_services()
            .any(|required| required == service)
            || !services.available_kinds().contains(&service)
        {
            return Err(failure(
                "swallowtail.opencode.run_host_service_missing",
                "OpenCode structured run requires its preflight-bound host services",
            ));
        }
    }
    for capability in [
        Capability::StructuredRun,
        Capability::StreamingEvents,
        Capability::ProviderTemporaryRetention,
        Capability::OwnedRemoteResourceDeletion,
        Capability::WorkingResource,
    ] {
        if !plan
            .requirements()
            .capabilities()
            .any(|required| required.capability() == capability)
        {
            return Err(failure(
                "swallowtail.opencode.run_capability_mismatch",
                "OpenCode structured-run capabilities do not match the preflight plan",
            ));
        }
    }
    require_run_constraint(
        plan,
        Capability::Interruption,
        CapabilityConstraint::CancellationScope(CancellationScope::StructuredRun),
    )?;
    let reasoning_constraints = request
        .policy()
        .reasoning_mode()
        .map(|mode| vec![CapabilityConstraint::ReasoningMode(mode.clone())])
        .unwrap_or_default();
    require_optional_run_control(
        plan,
        Capability::ReasoningSelection,
        reasoning_constraints,
        request.policy().reasoning_mode().is_some(),
    )?;
    let structured_constraints = request
        .structured_output()
        .map(|output| {
            vec![
                CapabilityConstraint::SchemaDialect(output.dialect().to_owned()),
                CapabilityConstraint::StructuredOutputEnforcement(
                    StructuredOutputEnforcement::HarnessValidated,
                ),
            ]
        })
        .unwrap_or_default();
    require_optional_run_control(
        plan,
        Capability::StructuredOutput,
        structured_constraints,
        request.structured_output().is_some(),
    )?;
    require_run_constraint(
        plan,
        Capability::WorkingResource,
        CapabilityConstraint::ResourceAccess(ResourceAccess::Read),
    )?;
    require_run_constraint(
        plan,
        Capability::WorkingResource,
        CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
    )?;
    require_run_constraint(
        plan,
        Capability::OwnedRemoteResourceDeletion,
        CapabilityConstraint::OwnedRemoteResource(OwnedRemoteResourceKind::Session),
    )?;
    if request.working_resource().is_none() {
        return Err(unsupported("a resource-free structured run"));
    }
    if request.attachments().len() != 0
        || request.tools().len() != 0
        || request.maximum_output_tokens().is_some()
    {
        return Err(unsupported(
            "structured-run attachments, consumer tools, or output-token limit",
        ));
    }
    let policy = request.policy();
    if policy.external_network() != swallowtail_runtime::ExternalNetworkPolicy::Denied
        || policy.external_search() != swallowtail_runtime::ExternalSearchPolicy::Disabled
        || policy.provider_execution() != ProviderExecutionPolicy::Attached
        || policy.provider_retention() != ProviderRetentionPolicy::TemporaryAllowed
        || policy.provider_recovery() != ProviderRecoveryPolicy::Prohibited
        || policy.stream_reattachment() != StreamReattachmentPolicy::Disabled
        || policy.harness_isolation() != Some(HarnessIsolation::AmbientHost)
        || policy.harness_configuration_posture() != Some(HarnessConfigurationPosture::Ambient)
    {
        return Err(unsupported("structured-run lifecycle or inference policy"));
    }
    validate_deadline(request.deadline(), services)
}

fn require_optional_run_control(
    plan: &PreflightPlan,
    capability: Capability,
    constraints: impl IntoIterator<Item = CapabilityConstraint>,
    expected: bool,
) -> Result<(), RuntimeFailure> {
    let expected_constraints = constraints.into_iter().collect::<BTreeSet<_>>();
    let actual = plan
        .requirements()
        .capabilities()
        .find(|required| required.capability() == capability);
    match actual {
        Some(required)
            if expected
                && required.constraints().cloned().collect::<BTreeSet<_>>()
                    == expected_constraints =>
        {
            Ok(())
        }
        None if !expected => Ok(()),
        _ => Err(failure(
            "swallowtail.opencode.run_capability_mismatch",
            "OpenCode generation controls do not match the preflight plan",
        )),
    }
}

fn require_run_constraint(
    plan: &PreflightPlan,
    capability: Capability,
    constraint: CapabilityConstraint,
) -> Result<(), RuntimeFailure> {
    if plan.requirements().capabilities().any(|required| {
        required.capability() == capability
            && required
                .constraints()
                .any(|required| required == &constraint)
    }) {
        Ok(())
    } else {
        Err(failure(
            "swallowtail.opencode.run_capability_mismatch",
            "OpenCode structured-run capability constraints do not match the preflight plan",
        ))
    }
}

fn copy_terminal_outcome(
    outcome: TerminalOutcome,
    cleanup: CleanupOutcome,
) -> TerminalOutcome {
    let mut finished = TerminalOutcome::new(outcome.status().clone(), cleanup);
    if let Some(output) = outcome.output().cloned() {
        finished = finished.with_output(output);
    }
    if let Some(cancellation) = outcome.provider_cancellation() {
        finished = finished.with_provider_cancellation(cancellation);
    }
    for (resource, deletion) in outcome.remote_resource_deletions() {
        finished = finished.with_remote_resource_deletion(resource, deletion);
    }
    finished
}

fn invalid_run_identity() -> RuntimeFailure {
    failure(
        "swallowtail.opencode.run_identity_invalid",
        "OpenCode structured-run identity was invalid",
    )
}
