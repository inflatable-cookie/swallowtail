struct FixtureSelection {
    plan: swallowtail_core::PreflightPlan,
    resource: WorkingResourceRef,
}

fn selection(host: ExecutionHostId) -> FixtureSelection {
    let descriptor = cursor_acp_descriptor();
    let access = cursor_subscription_access_profile(
        AccessProfileId::new("cursor.fixture.subscription").expect("access id"),
    );
    let capabilities = CapabilityProfile::new([
        CapabilityRequirement::new(Capability::InteractiveSession, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(Capability::ObservableActivity, []),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                swallowtail_core::CancellationScope::ActiveTurn,
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
                    ExtensionNamespace::new("cursor-agent").expect("namespace"),
                    "local-session",
                )
                .expect("constraint"),
            )],
        ),
        CapabilityRequirement::new(
            Capability::ProviderSessionAttachmentRecovery,
            [
                CapabilityConstraint::ReplayMaximumItems(4096),
                CapabilityConstraint::ReplayMaximumBytes(8 * 1024 * 1024),
            ],
        ),
    ]);
    let version =
        cursor_agent_release_binding("2026.07.01-41b2de7").expect("Cursor version");
    let services = [
        swallowtail_core::HostServiceKind::Task,
        swallowtail_core::HostServiceKind::Process,
        swallowtail_core::HostServiceKind::WorkingResource,
        swallowtail_core::HostServiceKind::WorkingResourceIo,
    ];
    let instance = ConfiguredInstance::new(
        ConfiguredInstanceId::new("cursor.fixture.instance").expect("instance"),
        InstanceRevision::new("cursor.fixture.instance-r1").expect("revision"),
        descriptor.identity().id().clone(),
        host.clone(),
        InstanceTargetRef::new("cursor.fixture.executable").expect("target"),
        InstanceOwnership::HostOwnedEphemeral,
        access.id().clone(),
        SupportAuthority::ProviderSupported,
        ProtocolFacadeId::new("acp-v1").expect("facade"),
        InstancePolicyId::new("cursor.fixture.ambient").expect("policy"),
        capabilities.clone(),
    )
    .with_interface_versions([version.clone()])
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
    let status = AccessStatus::new(
        access.id().clone(),
        CredentialState::NotRequired,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    );
    let requirements = OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        OperationShape::InteractiveSession,
        DriverRole::InteractiveSession,
        host,
        AccessRequirement::new(access.id().clone())
            .with_credential_states([CredentialState::NotRequired])
            .with_entitlement_states([EntitlementState::Available])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([SupportAuthority::ProviderSupported]),
    )
    .with_ownership_modes([InstanceOwnership::HostOwnedEphemeral])
    .with_host_services(services)
    .with_capabilities(capabilities.iter().map(|(capability, constraints)| {
        CapabilityRequirement::new(capability, constraints.iter().cloned())
    }))
    .with_interface_versions([version])
    .with_harness_isolation(HarnessIsolation::AmbientHost)
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
    .with_session_access_policy(SessionAccessPolicy::ambient_harness(
        ResourceAccess::ReadWrite,
    ))
    .with_session_provider_state_policy(
        SessionProviderStatePolicy::DurableProviderSessionPreserved,
    );
    let context = PreflightContext::new(&descriptor, &instance, &access, &status, services);
    FixtureSelection {
        plan: preflight(&context, &requirements).expect("fixture preflight"),
        resource: WorkingResourceRef::new("cursor.fixture.workspace").expect("resource"),
    }
}

fn open_on(
    host_id: ExecutionHostId,
    scenario: Scenario,
) -> (
    FixtureHost,
    HostServices,
    Box<dyn swallowtail_runtime::InteractiveSessionHandle>,
) {
    let selected = selection(host_id.clone());
    let host = FixtureHost::new(scenario);
    let services = host.services(host_id);
    let driver = CursorAcpDriver::new(
        EnvironmentRef::new("cursor.fixture.ambient").expect("environment"),
    );
    let request = OpenSessionRequest::new(
        RequestId::new("cursor-open").expect("request"),
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

fn open(
    scenario: Scenario,
) -> (
    FixtureHost,
    HostServices,
    Box<dyn swallowtail_runtime::InteractiveSessionHandle>,
) {
    open_on(
        ExecutionHostId::new("fixture.host.cursor").expect("host"),
        scenario,
    )
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
        "fixture.cursor_acp.failed",
        "Cursor ACP fixture failed",
    ))
}
