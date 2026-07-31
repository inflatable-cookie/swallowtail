use swallowtail_adapter_cursor::{
    cursor_agent_release_binding, cursor_catalogue_descriptor, cursor_subscription_access_profile,
};
use swallowtail_core::{
    AccessProfileId, AccessRequirement, AccessStatus, Capability, CapabilityProfile,
    CapabilityRequirement, ConfiguredInstance, ConfiguredInstanceId, CredentialState, DriverRole,
    EndpointAuthorization, EntitlementState, ExecutionHostId, ExecutionLayer,
    HarnessConfigurationPosture, HarnessIsolation, HostServiceKind, InstanceOwnership,
    InstancePolicyId, InstanceRevision, InstanceTargetRef, OperationRequirements, OperationShape,
    PreflightContext, PreflightPlan, ProtocolFacadeId, RuntimeReadiness, SupportAuthority,
    preflight,
};

pub fn catalogue_plan(host: ExecutionHostId, target: &str, release: &str) -> PreflightPlan {
    let descriptor = cursor_catalogue_descriptor();
    let access_id = AccessProfileId::new("access.cursor.subscription").expect("valid access id");
    let access = cursor_subscription_access_profile(access_id.clone());
    let capabilities = [CapabilityRequirement::new(Capability::ModelCatalog, [])];
    let instance = ConfiguredInstance::new(
        ConfiguredInstanceId::new("cursor-agent.fixture").expect("valid instance id"),
        InstanceRevision::new("1").expect("valid revision"),
        descriptor.identity().id().clone(),
        host.clone(),
        InstanceTargetRef::new(target).expect("valid target"),
        InstanceOwnership::HostOwnedEphemeral,
        access_id.clone(),
        SupportAuthority::ProviderSupported,
        ProtocolFacadeId::new("cursor-agent-models-v1").expect("valid facade"),
        InstancePolicyId::new("cursor-agent-ambient-catalogue").expect("valid policy"),
        CapabilityProfile::new(capabilities.clone()),
    )
    .with_interface_versions([
        cursor_agent_release_binding(release).expect("fixture Cursor release is valid")
    ])
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
    let status = AccessStatus::new(
        access_id.clone(),
        CredentialState::NotRequired,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    );
    let services = [HostServiceKind::Process, HostServiceKind::Time];
    let requirements = OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        OperationShape::StructuredRun,
        DriverRole::ModelCatalog,
        host,
        AccessRequirement::new(access_id)
            .with_credential_states([CredentialState::NotRequired])
            .with_entitlement_states([EntitlementState::Available])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([SupportAuthority::ProviderSupported]),
    )
    .with_ownership_modes([InstanceOwnership::HostOwnedEphemeral])
    .with_host_services(services)
    .with_capabilities(capabilities)
    .with_harness_isolation(HarnessIsolation::AmbientHost)
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
    .with_interface_versions([
        cursor_agent_release_binding(release).expect("fixture Cursor release is valid")
    ]);
    preflight(
        &PreflightContext::new(&descriptor, &instance, &access, &status, services),
        &requirements,
    )
    .expect("Cursor catalogue fixture preflight succeeds")
}
