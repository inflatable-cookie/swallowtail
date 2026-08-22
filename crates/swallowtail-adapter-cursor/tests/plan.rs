use swallowtail_adapter_cursor::{
    cursor_agent_release_binding, cursor_catalogue_descriptor, cursor_headless_descriptor,
    cursor_subscription_access_profile,
};
use swallowtail_core::{
    AccessProfileId, AccessRequirement, AccessStatus, Capability, CapabilityConstraint,
    CapabilityProfile, CapabilityRequirement, ConfiguredInstance, ConfiguredInstanceId,
    CredentialState, DriverRole, EndpointAuthorization, EntitlementState, ExecutionHostId,
    ExecutionLayer, HarnessConfigurationPosture, HarnessIsolation, HostServiceKind,
    InstanceOwnership, InstancePolicyId, InstanceRevision, InstanceTargetRef, ModelId, ModelRoute,
    ModelRouteId, ModelRouteRevision, OperationRequirements, OperationShape, PreflightContext,
    PreflightPlan, ProtocolFacadeId, ProviderId, ResourceAccess, ResourceRepresentation,
    RuntimeReadiness, SupportAuthority, preflight,
};

#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn headless_plan(
    host: ExecutionHostId,
    target: &str,
    resource_access: ResourceAccess,
) -> PreflightPlan {
    let descriptor = cursor_headless_descriptor();
    let access_id = AccessProfileId::new("access.cursor.subscription").expect("valid access id");
    let access = cursor_subscription_access_profile(access_id.clone());
    let version = cursor_agent_release_binding("2026.07.01-41b2de7")
        .expect("fixture Cursor release is valid");
    let capabilities = headless_capabilities(resource_access);
    let profile = CapabilityProfile::new(capabilities.clone());
    let instance = ConfiguredInstance::new(
        ConfiguredInstanceId::new("cursor-agent.headless.fixture").expect("valid instance id"),
        InstanceRevision::new("1").expect("valid revision"),
        descriptor.identity().id().clone(),
        host.clone(),
        InstanceTargetRef::new(target).expect("valid target"),
        InstanceOwnership::HostOwnedEphemeral,
        access_id.clone(),
        SupportAuthority::ProviderSupported,
        ProtocolFacadeId::new("cursor-stream-json-v1").expect("valid facade"),
        InstancePolicyId::new("cursor-agent-ambient-headless").expect("valid policy"),
        profile.clone(),
    )
    .with_interface_versions([version.clone()])
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
    let route = ModelRoute::new(
        ModelRouteId::new("cursor-headless-model-route").expect("valid route id"),
        ModelRouteRevision::new("1").expect("valid route revision"),
        instance.id().clone(),
        ModelId::new("fixture-model").expect("valid model id"),
        profile,
    )
    .with_provider_id(ProviderId::new("cursor").expect("valid provider id"));
    let status = AccessStatus::new(
        access_id.clone(),
        CredentialState::NotRequired,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    );
    let services = [
        HostServiceKind::Task,
        HostServiceKind::Process,
        HostServiceKind::Time,
    ];
    let requirements = OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        OperationShape::StructuredRun,
        DriverRole::StructuredRun,
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
    .with_interface_versions([version])
    .require_model_route();
    preflight(
        &PreflightContext::new(&descriptor, &instance, &access, &status, services)
            .with_model_route(&route),
        &requirements,
    )
    .expect("Cursor headless fixture preflight succeeds")
}

#[allow(dead_code)]
pub fn headless_plan_with_model(
    host: ExecutionHostId,
    target: &str,
    resource_access: ResourceAccess,
    model_id: &str,
) -> PreflightPlan {
    let descriptor = cursor_headless_descriptor();
    let access_id = AccessProfileId::new("access.cursor.subscription").expect("valid access id");
    let access = cursor_subscription_access_profile(access_id.clone());
    let version = cursor_agent_release_binding("2026.07.01-41b2de7")
        .expect("fixture Cursor release is valid");
    let capabilities = headless_capabilities(resource_access);
    let profile = CapabilityProfile::new(capabilities.clone());
    let instance = ConfiguredInstance::new(
        ConfiguredInstanceId::new("cursor-agent.headless.fixture").expect("valid instance id"),
        InstanceRevision::new("1").expect("valid revision"),
        descriptor.identity().id().clone(),
        host.clone(),
        InstanceTargetRef::new(target).expect("valid target"),
        InstanceOwnership::HostOwnedEphemeral,
        access_id.clone(),
        SupportAuthority::ProviderSupported,
        ProtocolFacadeId::new("cursor-stream-json-v1").expect("valid facade"),
        InstancePolicyId::new("cursor-agent-ambient-headless").expect("valid policy"),
        profile.clone(),
    )
    .with_interface_versions([version.clone()])
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
    let route = ModelRoute::new(
        ModelRouteId::new("cursor-headless-model-route").expect("valid route id"),
        ModelRouteRevision::new("1").expect("valid route revision"),
        instance.id().clone(),
        ModelId::new(model_id).expect("valid model id"),
        profile,
    )
    .with_provider_id(ProviderId::new("cursor").expect("valid provider id"));
    let status = AccessStatus::new(
        access_id.clone(),
        CredentialState::NotRequired,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    );
    let services = [
        HostServiceKind::Task,
        HostServiceKind::Process,
        HostServiceKind::Time,
    ];
    let requirements = OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        OperationShape::StructuredRun,
        DriverRole::StructuredRun,
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
    .with_interface_versions([version])
    .require_model_route();
    preflight(
        &PreflightContext::new(&descriptor, &instance, &access, &status, services)
            .with_model_route(&route),
        &requirements,
    )
    .expect("Cursor headless fixture preflight succeeds")
}

#[allow(dead_code)]
fn headless_capabilities(resource_access: ResourceAccess) -> Vec<CapabilityRequirement> {
    vec![
        CapabilityRequirement::new(Capability::StructuredRun, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(Capability::ObservableActivity, []),
        CapabilityRequirement::new(Capability::UsageReporting, []),
        CapabilityRequirement::new(Capability::ProviderDurableRetention, []),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                swallowtail_core::CancellationScope::StructuredRun,
            )],
        ),
        CapabilityRequirement::new(
            Capability::WorkingResource,
            [
                CapabilityConstraint::ResourceAccess(resource_access),
                CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
            ],
        ),
    ]
}
