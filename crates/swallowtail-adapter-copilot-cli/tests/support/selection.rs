pub struct FixtureSelection {
    pub plan: PreflightPlan,
    pub resource: WorkingResourceRef,
}

pub fn selection(host: ExecutionHostId) -> FixtureSelection {
    selection_with_access(host, ResourceAccess::Read)
}

pub fn selection_with_access(
    host: ExecutionHostId,
    resource_access: ResourceAccess,
) -> FixtureSelection {
    let descriptor = copilot_cli_acp_descriptor();
    let access_id =
        AccessProfileId::new("copilot-cli.fixture.host-account").expect("valid access id");
    let instance_id =
        ConfiguredInstanceId::new("copilot-cli.fixture.instance").expect("valid instance");
    let capabilities = CapabilityProfile::new([
        CapabilityRequirement::new(Capability::InteractiveSession, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                swallowtail_core::CancellationScope::ActiveTurn,
            )],
        ),
        CapabilityRequirement::new(
            Capability::WorkingResource,
            [
                CapabilityConstraint::ResourceAccess(resource_access),
                CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
            ],
        ),
    ]);
    let version_binding = copilot_cli_package_binding("1.0.80").expect("fixture version is valid");
    let instance = ConfiguredInstance::new(
        instance_id.clone(),
        InstanceRevision::new("fixture-revision").expect("valid revision"),
        AdapterId::new("swallowtail.copilot-cli.acp").expect("valid adapter"),
        host.clone(),
        InstanceTargetRef::new("copilot-cli.fixture.executable").expect("valid target"),
        InstanceOwnership::HostOwnedEphemeral,
        access_id.clone(),
        SupportAuthority::ExperimentalObserved,
        ProtocolFacadeId::new("acp-v1").expect("valid facade"),
        InstancePolicyId::new("copilot-cli.fixture.isolated").expect("valid policy"),
        capabilities.clone(),
    )
    .with_interface_versions([version_binding.clone()])
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
    let access = copilot_cli_host_account_access_profile(access_id.clone());
    let status = AccessStatus::new(
        access_id.clone(),
        CredentialState::NotRequired,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ExperimentalObserved,
    );
    let requirements = OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        OperationShape::InteractiveSession,
        DriverRole::InteractiveSession,
        host,
        AccessRequirement::new(access_id)
            .with_credential_states([CredentialState::NotRequired])
            .with_entitlement_states([EntitlementState::Available])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([SupportAuthority::ExperimentalObserved]),
    )
    .with_ownership_modes([InstanceOwnership::HostOwnedEphemeral])
    .with_host_services([
        swallowtail_core::HostServiceKind::Task,
        swallowtail_core::HostServiceKind::Process,
        swallowtail_core::HostServiceKind::WorkingResource,
    ])
    .with_capabilities(capabilities.iter().map(|(capability, constraints)| {
        CapabilityRequirement::new(capability, constraints.iter().cloned())
    }))
    .with_interface_versions([version_binding])
    .with_harness_isolation(HarnessIsolation::AmbientHost)
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
    .with_session_access_policy(SessionAccessPolicy::ambient_harness(resource_access))
    .with_session_provider_state_policy(SessionProviderStatePolicy::Prohibited);
    let context = PreflightContext::new(
        &descriptor,
        &instance,
        &access,
        &status,
        [
            swallowtail_core::HostServiceKind::Task,
            swallowtail_core::HostServiceKind::Process,
            swallowtail_core::HostServiceKind::WorkingResource,
        ],
    );
    let plan = preflight(&context, &requirements).expect("Copilot CLI fixture preflight succeeds");
    FixtureSelection {
        plan,
        resource: WorkingResourceRef::new("copilot-cli.fixture.workspace").expect("valid resource"),
    }
}
