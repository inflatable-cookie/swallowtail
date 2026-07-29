pub struct FixtureSelection {
    pub plan: PreflightPlan,
    pub credential: CredentialRef,
    pub resource: WorkingResourceRef,
}

pub fn selection(host: ExecutionHostId) -> FixtureSelection {
    selection_with_access(host, ResourceAccess::Read)
}

pub fn selection_with_access(
    host: ExecutionHostId,
    resource_access: ResourceAccess,
) -> FixtureSelection {
    let descriptor = swallowtail_adapter_gemini::gemini_acp_descriptor();
    let credential = CredentialRef::new("gemini.fixture.credential").expect("valid credential");
    let access_id = AccessProfileId::new("gemini.fixture.api-key").expect("valid access id");
    let instance_id = ConfiguredInstanceId::new("gemini.fixture.instance").expect("valid instance");
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
    let version_binding = swallowtail_adapter_gemini::gemini_cli_acp_binding("0.51.0")
        .expect("fixture version is valid");
    let instance = ConfiguredInstance::new(
        instance_id.clone(),
        InstanceRevision::new("fixture-revision").expect("valid revision"),
        AdapterId::new("swallowtail.gemini.acp").expect("valid adapter"),
        host.clone(),
        InstanceTargetRef::new("gemini.fixture.executable").expect("valid target"),
        InstanceOwnership::HostOwnedEphemeral,
        access_id.clone(),
        SupportAuthority::ProviderSupported,
        ProtocolFacadeId::new("acp-v1").expect("valid facade"),
        InstancePolicyId::new("gemini.fixture.isolated-plan").expect("valid policy"),
        capabilities.clone(),
    )
    .with_interface_versions([version_binding.clone()])
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
    let access = AccessProfile::new(
        access_id.clone(),
        CredentialMechanism::ApiKey,
        EntitlementMetering::PayAsYouGo,
        EndpointAudience::new("gemini-developer-api").expect("valid audience"),
        SupportAuthority::ProviderSupported,
    )
    .with_credential_reference(credential.clone());
    let status = AccessStatus::new(
        access_id.clone(),
        CredentialState::Ready,
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
        AccessRequirement::new(access_id)
            .with_credential_states([CredentialState::Ready])
            .with_entitlement_states([EntitlementState::Available])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([SupportAuthority::ProviderSupported]),
    )
    .with_ownership_modes([InstanceOwnership::HostOwnedEphemeral])
    .with_host_services([
        swallowtail_core::HostServiceKind::Task,
        swallowtail_core::HostServiceKind::Process,
        swallowtail_core::HostServiceKind::WorkingResource,
        swallowtail_core::HostServiceKind::WorkingResourceIo,
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
            swallowtail_core::HostServiceKind::WorkingResourceIo,
        ],
    );
    let plan = preflight(&context, &requirements).expect("Gemini fixture preflight succeeds");
    FixtureSelection {
        plan,
        credential,
        resource: WorkingResourceRef::new("gemini.fixture.workspace").expect("valid resource"),
    }
}
