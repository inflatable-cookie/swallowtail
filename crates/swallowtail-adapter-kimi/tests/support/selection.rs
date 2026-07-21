use super::*;

pub struct FixtureSelection {
    pub plan: PreflightPlan,
    pub credential: CredentialRef,
    pub resource: WorkingResourceRef,
}

pub fn selection(host: ExecutionHostId) -> FixtureSelection {
    let descriptor = swallowtail_adapter_kimi::kimi_acp_descriptor();
    let credential = CredentialRef::new("kimi.fixture.delegated-auth").expect("valid credential");
    let access_id = AccessProfileId::new("kimi.fixture.membership-oauth").expect("valid access id");
    let instance_id = ConfiguredInstanceId::new("kimi.fixture.instance").expect("valid instance");
    let capabilities = CapabilityProfile::new([
        CapabilityRequirement::new(Capability::InteractiveSession, []),
        CapabilityRequirement::new(
            Capability::LoadSession,
            [
                CapabilityConstraint::ReplayMaximumItems(512),
                CapabilityConstraint::ReplayMaximumBytes(4 * 1024 * 1024),
            ],
        ),
        CapabilityRequirement::new(Capability::Resume, []),
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
                CapabilityConstraint::ResourceAccess(ResourceAccess::ReadWrite),
                CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
            ],
        ),
        CapabilityRequirement::new(
            Capability::WorkingResourceTextWrite,
            [CapabilityConstraint::WorkingResourceMaximumBytes(
                1024 * 1024,
            )],
        ),
    ]);
    let instance = ConfiguredInstance::new(
        instance_id.clone(),
        InstanceRevision::new("fixture-revision").expect("valid revision"),
        AdapterId::new("swallowtail.kimi.acp").expect("valid adapter"),
        host.clone(),
        InstanceTargetRef::new("kimi.fixture.pinned-executable").expect("valid target"),
        InstanceOwnership::HostOwnedEphemeral,
        access_id.clone(),
        SupportAuthority::IntegrationMaintainerSupported,
        ProtocolFacadeId::new("acp-v1").expect("valid facade"),
        InstancePolicyId::new("kimi.fixture.ambient").expect("valid policy"),
        capabilities.clone(),
    );
    let route = ModelRoute::new(
        ModelRouteId::new("kimi.fixture.route").expect("valid route"),
        ModelRouteRevision::new("fixture-route-revision").expect("valid route revision"),
        instance_id,
        ModelId::new("kimi-coder").expect("valid model"),
        capabilities.clone(),
    );
    let access = AccessProfile::new(
        access_id.clone(),
        CredentialMechanism::InteractiveOauth,
        EntitlementMetering::SubscriptionAllowance,
        EndpointAudience::new("kimi-code-membership").expect("valid audience"),
        SupportAuthority::IntegrationMaintainerSupported,
    )
    .with_credential_reference(credential.clone());
    let status = AccessStatus::new(
        access_id.clone(),
        CredentialState::Ready,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::IntegrationMaintainerSupported,
    );
    let service_kinds = [
        HostServiceKind::Task,
        HostServiceKind::Process,
        HostServiceKind::Credential,
        HostServiceKind::WorkingResource,
        HostServiceKind::WorkingResourceIo,
    ];
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
            .with_support_authorities([SupportAuthority::IntegrationMaintainerSupported]),
    )
    .with_ownership_modes([InstanceOwnership::HostOwnedEphemeral])
    .with_host_services(service_kinds)
    .with_capabilities(capabilities.iter().map(|(capability, constraints)| {
        CapabilityRequirement::new(capability, constraints.iter().cloned())
    }))
    .with_session_access_policy(SessionAccessPolicy::ambient_harness(
        ResourceAccess::ReadWrite,
    ))
    .require_model_route();
    let context = PreflightContext::new(&descriptor, &instance, &access, &status, service_kinds)
        .with_model_route(&route);
    let plan = preflight(&context, &requirements).expect("Kimi fixture preflight succeeds");
    FixtureSelection {
        plan,
        credential,
        resource: WorkingResourceRef::new("kimi.fixture.workspace").expect("valid resource"),
    }
}
