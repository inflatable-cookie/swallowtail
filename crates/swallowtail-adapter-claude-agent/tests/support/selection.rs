use super::*;

pub struct FixtureSelection {
    pub plan: PreflightPlan,
    pub credential: CredentialRef,
    pub resource: WorkingResourceRef,
}

pub fn open_request(
    id: impl Into<String>,
    resource: WorkingResourceRef,
) -> swallowtail_runtime::OpenSessionRequest {
    swallowtail_runtime::OpenSessionRequest::new(
        swallowtail_runtime::RequestId::new(id).expect("valid request"),
        resource,
        None,
        swallowtail_runtime::SessionPlanAgreement::explicit(
            SessionAccessPolicy::ambient_harness(ResourceAccess::Read),
            Some(SessionProviderStatePolicy::Prohibited),
            Some(HarnessConfigurationPosture::Ambient),
        ),
    )
}

pub fn selection(host: ExecutionHostId, version: &str) -> FixtureSelection {
    selection_for_role(host, version, DriverRole::InteractiveSession)
}

#[allow(dead_code)]
pub fn run_selection(host: ExecutionHostId, version: &str) -> FixtureSelection {
    selection_for_role(host, version, DriverRole::StructuredRun)
}

fn selection_for_role(host: ExecutionHostId, version: &str, role: DriverRole) -> FixtureSelection {
    let descriptor = swallowtail_adapter_claude_agent::claude_agent_acp_descriptor();
    let credential = CredentialRef::new("claude-agent.fixture.api-key").expect("valid credential");
    let access_id =
        AccessProfileId::new("claude-agent.fixture.public-api").expect("valid access id");
    let instance_id =
        ConfiguredInstanceId::new("claude-agent.fixture.instance").expect("valid instance");
    let capabilities = capabilities(role);
    let version_binding = swallowtail_adapter_claude_agent::claude_agent_acp_binding(version)
        .expect("fixture version is valid");
    let instance = ConfiguredInstance::new(
        instance_id.clone(),
        InstanceRevision::new("fixture-revision").expect("valid revision"),
        AdapterId::new("swallowtail.claude-agent.acp").expect("valid adapter"),
        host.clone(),
        InstanceTargetRef::new("claude-agent.fixture.executable").expect("valid target"),
        InstanceOwnership::HostOwnedEphemeral,
        access_id.clone(),
        SupportAuthority::IntegrationMaintainerSupported,
        ProtocolFacadeId::new("acp-v1").expect("valid facade"),
        InstancePolicyId::new("claude-agent.fixture.ambient").expect("valid policy"),
        capabilities.clone(),
    )
    .with_interface_versions([version_binding.clone()])
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
    let route = ModelRoute::new(
        ModelRouteId::new("claude-agent.fixture.route").expect("valid route"),
        ModelRouteRevision::new("fixture-route-revision").expect("valid route revision"),
        instance_id,
        ModelId::new("claude-sonnet-4-6").expect("valid model"),
        capabilities.clone(),
    );
    let access = AccessProfile::new(
        access_id.clone(),
        CredentialMechanism::ApiKey,
        EntitlementMetering::PayAsYouGo,
        EndpointAudience::new("api.anthropic.com").expect("valid audience"),
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
        HostServiceKind::Time,
        HostServiceKind::Process,
        HostServiceKind::Credential,
        HostServiceKind::WorkingResource,
        HostServiceKind::WorkingResourceIo,
    ];
    let requirements = OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        match role {
            DriverRole::StructuredRun => OperationShape::StructuredRun,
            _ => OperationShape::InteractiveSession,
        },
        role,
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
    .with_interface_versions([version_binding])
    .with_harness_isolation(HarnessIsolation::AmbientHost)
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
    .require_model_route();
    let requirements = if role == DriverRole::InteractiveSession {
        requirements
            .with_session_access_policy(SessionAccessPolicy::ambient_harness(ResourceAccess::Read))
            .with_session_provider_state_policy(SessionProviderStatePolicy::Prohibited)
    } else {
        requirements
    };
    let context = PreflightContext::new(&descriptor, &instance, &access, &status, service_kinds)
        .with_model_route(&route);
    let plan = preflight(&context, &requirements).expect("fixture preflight succeeds");
    FixtureSelection {
        plan,
        credential,
        resource: WorkingResourceRef::new("claude-agent.fixture.workspace")
            .expect("valid resource"),
    }
}

fn capabilities(role: DriverRole) -> CapabilityProfile {
    let cancellation = match role {
        DriverRole::StructuredRun => swallowtail_core::CancellationScope::StructuredRun,
        _ => swallowtail_core::CancellationScope::ActiveTurn,
    };
    let mut requirements = vec![
        CapabilityRequirement::new(
            match role {
                DriverRole::StructuredRun => Capability::StructuredRun,
                _ => Capability::InteractiveSession,
            },
            [],
        ),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(cancellation)],
        ),
        CapabilityRequirement::new(
            Capability::WorkingResource,
            [
                CapabilityConstraint::ResourceAccess(match role {
                    DriverRole::StructuredRun => ResourceAccess::ReadWrite,
                    _ => ResourceAccess::Read,
                }),
                CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
            ],
        ),
    ];
    if role == DriverRole::StructuredRun {
        requirements.push(CapabilityRequirement::new(
            Capability::ProviderDurableRetention,
            [],
        ));
    }
    CapabilityProfile::new(requirements)
}
