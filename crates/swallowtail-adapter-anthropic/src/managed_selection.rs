use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, CancellationScope, Capability,
    CapabilityConstraint, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    ConfiguredInstanceId, CredentialMechanism, CredentialRef, CredentialState, DriverRole,
    EndpointAudience, EndpointAuthorization, EntitlementMetering, EntitlementState,
    ExecutionHostId, ExecutionLayer, HostServiceKind, InstanceOwnership, InstancePolicyId,
    InstanceRevision, InstanceTargetRef, InterfaceBehaviorRevision, InterfaceCompatibilityClaim,
    InterfaceCompatibilityClaimId, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment, ModelId, ModelRoute,
    ModelRouteId, ModelRouteRevision, OperationRequirements, OperationShape,
    OwnedRemoteResourceKind, ProtocolFacadeId, ProviderAgentBinding, ProviderId, RuntimeReadiness,
    SupportAuthority,
};

/// Official first-party endpoint admitted by the Managed Agents route.
pub const ANTHROPIC_MANAGED_API_ENDPOINT: &str = "https://api.anthropic.com";
/// Endpoint audience required by Managed Agents credential leases.
pub const ANTHROPIC_MANAGED_ENDPOINT_AUDIENCE: &str = "api.anthropic.com";
/// Public API-key access profile admitted by Managed Agents.
pub const ANTHROPIC_MANAGED_ACCESS_PROFILE_ID: &str =
    "anthropic.managed-agents.public-api-key.payg";
/// Exact opaque Managed Agents beta facade revision.
pub const ANTHROPIC_MANAGED_FACADE_REVISION: &str = "managed-agents-2026-04-01";

const ANTHROPIC_MANAGED_FACADE_AXIS: &str = "anthropic.managed-agents-facade";

#[must_use]
/// Creates the provider-supported Managed Agents API-key access profile.
pub fn anthropic_managed_access_profile(credential: CredentialRef) -> AccessProfile {
    AccessProfile::new(
        id(AccessProfileId::new, ANTHROPIC_MANAGED_ACCESS_PROFILE_ID),
        CredentialMechanism::ApiKey,
        EntitlementMetering::PayAsYouGo,
        id(EndpointAudience::new, ANTHROPIC_MANAGED_ENDPOINT_AUDIENCE),
        SupportAuthority::ProviderSupported,
    )
    .with_credential_reference(credential)
}

#[must_use]
/// Returns the exact Managed Agents facade version binding.
pub fn anthropic_managed_facade_binding() -> InterfaceVersionBinding {
    InterfaceVersionBinding::new(
        id(InterfaceVersionAxis::new, ANTHROPIC_MANAGED_FACADE_AXIS),
        id(InterfaceVersion::new, ANTHROPIC_MANAGED_FACADE_REVISION),
    )
}

#[must_use]
/// Returns the qualified-only compatibility claim for Managed Agents.
pub fn anthropic_managed_facade_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        id(
            InterfaceCompatibilityClaimId::new,
            "anthropic.managed-agents-window-1",
        ),
        id(InterfaceVersionAxis::new, ANTHROPIC_MANAGED_FACADE_AXIS),
        InterfaceVersionScheme::Opaque,
        swallowtail_core::InterfaceNewerVersionPosture::QualifiedOnly,
        [InterfaceVersionSegment::exact(
            id(InterfaceVersion::new, ANTHROPIC_MANAGED_FACADE_REVISION),
            id(
                InterfaceBehaviorRevision::new,
                "anthropic.managed-agents-resource-free-v1",
            ),
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("static Anthropic Managed Agents facade claim is valid")
}

#[must_use]
#[allow(clippy::too_many_arguments)]
/// Builds a configured Managed Agents instance with one operator-owned agent.
pub fn anthropic_managed_instance(
    instance_id: ConfiguredInstanceId,
    revision: InstanceRevision,
    host: ExecutionHostId,
    endpoint_target: InstanceTargetRef,
    access_profile_id: AccessProfileId,
    provider_agent: ProviderAgentBinding,
) -> ConfiguredInstance {
    ConfiguredInstance::new(
        instance_id,
        revision,
        crate::anthropic_managed_agent_descriptor()
            .identity()
            .id()
            .clone(),
        host,
        endpoint_target,
        InstanceOwnership::ExternalAttached,
        access_profile_id,
        SupportAuthority::ProviderSupported,
        id(ProtocolFacadeId::new, ANTHROPIC_MANAGED_FACADE_REVISION),
        id(
            InstancePolicyId::new,
            "managed-resource-free-delete-on-close",
        ),
        CapabilityProfile::new(anthropic_managed_route_capabilities()),
    )
    .with_provider_agent(provider_agent)
    .with_interface_versions([anthropic_managed_facade_binding()])
}

#[must_use]
/// Builds one exact model route for a Managed Agents instance.
pub fn anthropic_managed_model_route(
    instance_id: ConfiguredInstanceId,
    route_id: ModelRouteId,
    revision: ModelRouteRevision,
    model_id: ModelId,
) -> ModelRoute {
    ModelRoute::new(
        route_id,
        revision,
        instance_id,
        model_id,
        CapabilityProfile::new(anthropic_managed_route_capabilities()),
    )
    .with_provider_id(id(ProviderId::new, "anthropic"))
}

#[must_use]
/// Returns requirements for a durable delete-on-close managed-agent run.
pub fn anthropic_managed_requirements(host: ExecutionHostId) -> OperationRequirements {
    OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        OperationShape::StructuredRun,
        DriverRole::StructuredRun,
        host,
        AccessRequirement::new(id(
            AccessProfileId::new,
            ANTHROPIC_MANAGED_ACCESS_PROFILE_ID,
        ))
        .with_credential_states([CredentialState::Ready])
        .with_entitlement_states([EntitlementState::Available])
        .with_endpoint_authorizations([EndpointAuthorization::Allowed])
        .with_runtime_readiness([RuntimeReadiness::Ready])
        .with_support_authorities([SupportAuthority::ProviderSupported]),
    )
    .with_ownership_modes([InstanceOwnership::ExternalAttached])
    .with_host_services([
        HostServiceKind::Task,
        HostServiceKind::BlockingWork,
        HostServiceKind::Time,
        HostServiceKind::Network,
        HostServiceKind::Credential,
    ])
    .with_capabilities(anthropic_managed_capabilities())
    .with_interface_versions([anthropic_managed_facade_binding()])
    .require_model_route()
}

fn anthropic_managed_capabilities() -> Vec<CapabilityRequirement> {
    let mut capabilities = vec![
        CapabilityRequirement::new(Capability::StructuredRun, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(
            Capability::ToolCalls,
            [
                CapabilityConstraint::ToolSchemaDialect("json-schema-2020-12".to_owned()),
                CapabilityConstraint::ToolMaximumSchemaBytes(16_384),
                CapabilityConstraint::ToolMaximumCount(8),
            ],
        ),
        CapabilityRequirement::new(Capability::UsageReporting, []),
        CapabilityRequirement::new(Capability::ProviderDurableRetention, []),
        CapabilityRequirement::new(Capability::ProviderManagedRecovery, []),
        CapabilityRequirement::new(
            Capability::OwnedRemoteResourceDeletion,
            [
                CapabilityConstraint::OwnedRemoteResource(OwnedRemoteResourceKind::Environment),
                CapabilityConstraint::OwnedRemoteResource(OwnedRemoteResourceKind::Session),
            ],
        ),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                CancellationScope::StructuredRun,
            )],
        ),
        CapabilityRequirement::new(
            Capability::StreamReattachment,
            [CapabilityConstraint::ReattachmentMaximumCount(1)],
        ),
    ];
    capabilities.push(
        crate::managed_activity::profile::activity_profile()
            .capability_requirement()
            .expect("managed-agent activity is available"),
    );
    capabilities
}

pub(crate) fn anthropic_managed_recoverable_requirements(
    host: ExecutionHostId,
) -> OperationRequirements {
    let mut capabilities = anthropic_managed_capabilities();
    capabilities.extend(recovery_capabilities());
    OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        OperationShape::StructuredRun,
        DriverRole::StructuredRun,
        host,
        AccessRequirement::new(id(
            AccessProfileId::new,
            ANTHROPIC_MANAGED_ACCESS_PROFILE_ID,
        ))
        .with_credential_states([CredentialState::Ready])
        .with_entitlement_states([EntitlementState::Available])
        .with_endpoint_authorizations([EndpointAuthorization::Allowed])
        .with_runtime_readiness([RuntimeReadiness::Ready])
        .with_support_authorities([SupportAuthority::ProviderSupported]),
    )
    .with_ownership_modes([InstanceOwnership::ExternalAttached])
    .with_host_services([
        HostServiceKind::Task,
        HostServiceKind::BlockingWork,
        HostServiceKind::Time,
        HostServiceKind::Network,
        HostServiceKind::Credential,
    ])
    .with_capabilities(capabilities)
    .with_interface_versions([anthropic_managed_facade_binding()])
    .require_model_route()
}

fn anthropic_managed_route_capabilities() -> Vec<CapabilityRequirement> {
    let mut capabilities = anthropic_managed_capabilities();
    capabilities.extend(recovery_capabilities());
    capabilities
}

fn recovery_capabilities() -> [CapabilityRequirement; 2] {
    [
        CapabilityRequirement::new(Capability::ProviderRunReconciliation, []),
        CapabilityRequirement::new(
            Capability::ProviderRecoveredResourceCleanup,
            [
                CapabilityConstraint::OwnedRemoteResource(OwnedRemoteResourceKind::Environment),
                CapabilityConstraint::OwnedRemoteResource(OwnedRemoteResourceKind::Session),
            ],
        ),
    ]
}

fn id<T, E>(constructor: impl FnOnce(String) -> Result<T, E>, value: &str) -> T
where
    E: std::fmt::Debug,
{
    constructor(value.to_owned()).expect("static Anthropic Managed Agents identity is valid")
}
