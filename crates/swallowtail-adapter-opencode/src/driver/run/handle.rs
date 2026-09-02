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
    callbacks: Option<swallowtail_runtime::CallbackExchange>,
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

    fn take_callbacks(&mut self) -> Option<swallowtail_runtime::CallbackExchange> {
        self.callbacks.take()
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

#[derive(Clone)]
struct RunSessionCleanupBoundary {
    request: SessionCleanupRequest,
    services: HostServices,
}

async fn close_run_session(
    session: OpenCodeSessionHandle,
    boundary: Option<RunSessionCleanupBoundary>,
) -> CreatedSessionCleanup {
    let Some(boundary) = boundary else {
        return session.close_and_delete().await;
    };
    let expected_execution_host_id = session.resume_binding.execution_host_id().clone();
    let deletion = Arc::new(Mutex::new(None));
    let captured_deletion = Arc::clone(&deletion);
    let cleanup = swallowtail_runtime::bound_session_cleanup(
        expected_execution_host_id,
        boundary.request,
        boundary.services,
        Box::pin(async move {
            let result = session.close_and_delete().await;
            *captured_deletion
                .lock()
                .expect("OpenCode run deletion lock poisoned") = Some(result.deletion);
            result.cleanup
        }),
    )
    .await;
    let deletion = deletion
        .lock()
        .expect("OpenCode run deletion lock poisoned")
        .take()
        .unwrap_or(RemoteResourceDeletionOutcome::Unconfirmed);
    CreatedSessionCleanup { cleanup, deletion }
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
    let image_attachments = validate_attachment_plan(plan, services)?;
    validate_attachments(request.attachments(), services, image_attachments)?;
    let callbacks = provider_callbacks(plan)?;
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
        CapabilityConstraint::ResourceAccess(if callbacks {
            ResourceAccess::ReadWrite
        } else {
            ResourceAccess::Read
        }),
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
    if request.tools().len() != 0 || request.maximum_output_tokens().is_some() {
        return Err(unsupported(
            "structured-run consumer tools or output-token limit",
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
