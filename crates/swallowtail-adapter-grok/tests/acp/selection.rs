struct FixtureSelection {
    plan: swallowtail_core::PreflightPlan,
    credential: CredentialRef,
    resource: WorkingResourceRef,
}

fn selection(host: ExecutionHostId) -> FixtureSelection {
    selection_for(host, "0.2.114", false)
}

fn run_selection(host: ExecutionHostId, version: &str) -> FixtureSelection {
    selection_for(host, version, true)
}

fn selection_for(host: ExecutionHostId, version: &str, structured: bool) -> FixtureSelection {
    let descriptor = grok_build_acp_descriptor();
    let credential = CredentialRef::new("grok.fixture.credential").expect("credential");
    let access = grok_build_subscription_access_profile(credential.clone());
    let instance_id = ConfiguredInstanceId::new("grok.fixture.instance").expect("instance");
    let operation = if structured {
        Capability::StructuredRun
    } else {
        Capability::InteractiveSession
    };
    let cancellation = if structured {
        swallowtail_core::CancellationScope::StructuredRun
    } else {
        swallowtail_core::CancellationScope::ActiveTurn
    };
    let capabilities = CapabilityProfile::new([
        CapabilityRequirement::new(operation, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(Capability::ObservableActivity, []),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                cancellation,
            )],
        ),
        CapabilityRequirement::new(
            Capability::WorkingResource,
            [
                CapabilityConstraint::ResourceAccess(ResourceAccess::ReadWrite),
                CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
            ],
        ),
        CapabilityRequirement::new(
            Capability::ProviderDurableRetention,
            [CapabilityConstraint::Named(
                NamedCapabilityConstraint::new(
                    ExtensionNamespace::new("grok-build").expect("namespace"),
                    "local-session",
                )
                .expect("constraint"),
            )],
        ),
    ]);
    let version = grok_build_acp_binding(version).expect("version");
    let instance = ConfiguredInstance::new(
        instance_id.clone(),
        InstanceRevision::new("grok.fixture.instance-r1").expect("revision"),
        descriptor.identity().id().clone(),
        host.clone(),
        InstanceTargetRef::new("grok.fixture.executable").expect("target"),
        InstanceOwnership::HostOwnedEphemeral,
        access.id().clone(),
        SupportAuthority::ProviderSupported,
        ProtocolFacadeId::new("acp-v1").expect("facade"),
        InstancePolicyId::new("grok.fixture.ambient").expect("policy"),
        capabilities.clone(),
    )
    .with_interface_versions([version.clone()])
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
    let status = AccessStatus::new(
        access.id().clone(),
        CredentialState::Ready,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    );
    let route = ModelRoute::new(
        ModelRouteId::new("grok.fixture.route").expect("route"),
        ModelRouteRevision::new("grok.fixture.route-r1").expect("route revision"),
        instance_id,
        ModelId::new("grok-4.5").expect("model"),
        capabilities.clone(),
    );
    let shape = if structured {
        OperationShape::StructuredRun
    } else {
        OperationShape::InteractiveSession
    };
    let role = if structured {
        DriverRole::StructuredRun
    } else {
        DriverRole::InteractiveSession
    };
    let mut host_services = vec![
        swallowtail_core::HostServiceKind::Task,
        swallowtail_core::HostServiceKind::Process,
        swallowtail_core::HostServiceKind::Credential,
        swallowtail_core::HostServiceKind::WorkingResource,
        swallowtail_core::HostServiceKind::WorkingResourceIo,
    ];
    if structured {
        host_services.push(swallowtail_core::HostServiceKind::Time);
    }
    let requirements = OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        shape,
        role,
        host,
        AccessRequirement::new(access.id().clone())
            .with_credential_states([CredentialState::Ready])
            .with_entitlement_states([EntitlementState::Available])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([SupportAuthority::ProviderSupported]),
    )
    .with_ownership_modes([InstanceOwnership::HostOwnedEphemeral])
    .with_host_services(host_services.iter().copied())
    .with_capabilities(capabilities.iter().map(|(capability, constraints)| {
        CapabilityRequirement::new(capability, constraints.iter().cloned())
    }))
    .with_interface_versions([version])
    .with_harness_isolation(HarnessIsolation::AmbientHost)
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
    .require_model_route();
    let requirements = if structured {
        requirements
    } else {
        requirements
            .with_session_access_policy(SessionAccessPolicy::ambient_harness(
                ResourceAccess::ReadWrite,
            ))
            .with_session_provider_state_policy(
                SessionProviderStatePolicy::DurableProviderSessionPreserved,
            )
    };
    let context = PreflightContext::new(
        &descriptor,
        &instance,
        &access,
        &status,
        host_services,
    )
    .with_model_route(&route);
    FixtureSelection {
        plan: preflight(&context, &requirements).expect("fixture preflight"),
        credential,
        resource: WorkingResourceRef::new("grok.fixture.workspace").expect("resource"),
    }
}

fn start_run(
    host_id: ExecutionHostId,
    host: &FixtureHost,
    version: &str,
    deadline: Option<swallowtail_runtime::Deadline>,
) -> Box<dyn swallowtail_runtime::RunHandle> {
    try_start_run(host_id, host, version, deadline).expect("run starts")
}

fn try_start_run(
    host_id: ExecutionHostId,
    host: &FixtureHost,
    version: &str,
    deadline: Option<swallowtail_runtime::Deadline>,
) -> Result<Box<dyn swallowtail_runtime::RunHandle>, RuntimeFailure> {
    let selected = run_selection(host_id.clone(), version);
    let services = host.services(host_id);
    let driver = GrokAcpDriver::new(
        EnvironmentRef::new("grok.fixture.ambient").expect("environment"),
        selected.credential,
    );
    let policy = swallowtail_runtime::OperationPolicy::offline()
        .with_provider_retention(swallowtail_runtime::ProviderRetentionPolicy::DurableAllowed)
        .with_harness_isolation(HarnessIsolation::AmbientHost)
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
    let mut request = swallowtail_runtime::StructuredRunRequest::new(
        RequestId::new("grok-run").expect("request"),
        OperationContent::new("private fixture prompt").expect("prompt"),
        policy,
    )
    .with_working_resource(selected.resource);
    if let Some(deadline) = deadline {
        request = request.with_deadline(deadline);
    }
    block_on(swallowtail_runtime::StructuredRunDriver::start_run(
        &driver,
        selected.plan,
        request,
        services,
    ))
}

fn open(
    scenario: Scenario,
) -> (
    FixtureHost,
    HostServices,
    Box<dyn swallowtail_runtime::InteractiveSessionHandle>,
) {
    let host_id = ExecutionHostId::new("fixture.host.grok").expect("host");
    let selected = selection(host_id.clone());
    let host = FixtureHost::new(scenario);
    let services = host.services(host_id);
    let driver = GrokAcpDriver::new(
        EnvironmentRef::new("grok.fixture.ambient").expect("environment"),
        selected.credential,
    );
    let request = OpenSessionRequest::new(
        RequestId::new("grok-open").expect("request"),
        selected.resource,
        None,
        SessionPlanAgreement::explicit(
            SessionAccessPolicy::ambient_harness(ResourceAccess::ReadWrite),
            Some(SessionProviderStatePolicy::DurableProviderSessionPreserved),
            Some(HarnessConfigurationPosture::Ambient),
        ),
    );
    let session = block_on(driver.open_session(selected.plan, request, services.clone()))
        .expect("session opens");
    (host, services, session)
}

fn start(
    session: &mut dyn swallowtail_runtime::InteractiveSessionHandle,
    services: HostServices,
    id: &str,
) -> Box<dyn swallowtail_runtime::TurnHandle> {
    block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new(id).expect("turn"),
            OperationContent::new("private fixture prompt").expect("prompt"),
        ),
        services,
    ))
    .expect("turn starts")
}

fn fixture_failure() -> RuntimeFailure {
    RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
        "fixture.grok_acp.failed",
        "Grok ACP fixture failed",
    ))
}
