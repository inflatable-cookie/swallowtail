pub struct FixtureSelection {
    pub plan: PreflightPlan,
    pub credential: CredentialRef,
    pub resource: WorkingResourceRef,
}

pub fn selection(host: ExecutionHostId) -> FixtureSelection {
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
                CapabilityConstraint::ResourceAccess(ResourceAccess::Read),
                CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
            ],
        ),
    ]);
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
    );
    let route = ModelRoute::new(
        ModelRouteId::new("gemini.fixture.route").expect("valid route"),
        ModelRouteRevision::new("fixture-route-revision").expect("valid route revision"),
        instance_id,
        ModelId::new("fixture-observed-model").expect("valid model"),
        capabilities.clone(),
    );
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
    .with_session_access_policy(SessionAccessPolicy::ambient_harness(ResourceAccess::Read))
    .require_model_route();
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
    )
    .with_model_route(&route);
    let plan = preflight(&context, &requirements).expect("Gemini fixture preflight succeeds");
    FixtureSelection {
        plan,
        credential,
        resource: WorkingResourceRef::new("gemini.fixture.workspace").expect("valid resource"),
    }
}
