use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, CancellationScope, Capability,
    CapabilityConstraint, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    ConfiguredInstanceId, CredentialMechanism, CredentialRef, CredentialState, DriverRole,
    EndpointAudience, EndpointAuthorization, EntitlementMetering, EntitlementState,
    ExecutionHostId, ExecutionLayer, HostServiceKind, InstanceOwnership, InstancePolicyId,
    InstanceRevision, InstanceTargetRef, InterfaceBehaviorRevision, InterfaceCompatibilityClaim,
    InterfaceCompatibilityClaimId, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment, ModelId, ModelRoute,
    ModelRouteId, ModelRouteRevision, OperationRequirements, OperationShape, ProtocolFacadeId,
    ProviderId, RuntimeReadiness, SessionAccessPolicy, SessionProviderStatePolicy,
    SupportAuthority,
};

/// Official xAI Responses WebSocket endpoint admitted by this route.
pub const XAI_RESPONSES_ENDPOINT: &str = "wss://api.x.ai/v1/responses";
/// Endpoint audience required by xAI Responses credential leases.
pub const XAI_RESPONSES_ENDPOINT_AUDIENCE: &str = "api.x.ai";
/// Public API-key profile admitted by the Responses route.
pub const XAI_RESPONSES_ACCESS_PROFILE_ID: &str = "xai.public-api.api-key.payg";
/// Stable configured-instance identity for the Responses route.
pub const XAI_RESPONSES_CONFIGURED_INSTANCE_ID: &str = "xai.public.responses-websocket";
/// Exact opaque xAI Responses WebSocket facade revision.
pub const XAI_RESPONSES_FACADE_REVISION: &str = "xai-responses-websocket-2026-04-23";

const FACADE_AXIS: &str = "xai.responses-websocket-facade";

#[must_use]
/// Creates the provider-supported xAI public API-key access profile.
pub fn xai_responses_access_profile(credential: CredentialRef) -> AccessProfile {
    AccessProfile::new(
        id(AccessProfileId::new, XAI_RESPONSES_ACCESS_PROFILE_ID),
        CredentialMechanism::ApiKey,
        EntitlementMetering::PayAsYouGo,
        id(EndpointAudience::new, XAI_RESPONSES_ENDPOINT_AUDIENCE),
        SupportAuthority::ProviderSupported,
    )
    .with_credential_reference(credential)
}

#[must_use]
/// Returns the exact xAI Responses facade version binding.
pub fn xai_responses_facade_binding() -> InterfaceVersionBinding {
    InterfaceVersionBinding::new(
        id(InterfaceVersionAxis::new, FACADE_AXIS),
        id(InterfaceVersion::new, XAI_RESPONSES_FACADE_REVISION),
    )
}

#[must_use]
/// Returns the qualified-only compatibility claim for the Responses facade.
pub fn xai_responses_facade_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        id(
            InterfaceCompatibilityClaimId::new,
            "xai.responses-websocket-window-1",
        ),
        id(InterfaceVersionAxis::new, FACADE_AXIS),
        InterfaceVersionScheme::Opaque,
        swallowtail_core::InterfaceNewerVersionPosture::QualifiedOnly,
        [InterfaceVersionSegment::exact(
            id(InterfaceVersion::new, XAI_RESPONSES_FACADE_REVISION),
            id(
                InterfaceBehaviorRevision::new,
                "xai.responses-websocket-store-false-v1",
            ),
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("static xAI Responses facade claim is valid")
}

#[must_use]
/// Builds the configured external Responses WebSocket instance.
pub fn xai_responses_instance(
    revision: InstanceRevision,
    host: ExecutionHostId,
    target: InstanceTargetRef,
    access_profile_id: AccessProfileId,
) -> ConfiguredInstance {
    ConfiguredInstance::new(
        id(
            ConfiguredInstanceId::new,
            XAI_RESPONSES_CONFIGURED_INSTANCE_ID,
        ),
        revision,
        crate::xai_websocket_descriptor().identity().id().clone(),
        host,
        target,
        InstanceOwnership::ExternalAttached,
        access_profile_id,
        SupportAuthority::ProviderSupported,
        id(ProtocolFacadeId::new, XAI_RESPONSES_FACADE_REVISION),
        id(
            InstancePolicyId::new,
            "xai-public-api-store-false-resource-free",
        ),
        CapabilityProfile::new(all_capabilities()),
    )
    .with_interface_versions([xai_responses_facade_binding()])
}

#[must_use]
/// Builds one exact model route for the configured Responses instance.
pub fn xai_responses_model_route(
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
        CapabilityProfile::new(all_capabilities()),
    )
    .with_provider_id(id(ProviderId::new, "xai"))
}

#[must_use]
/// Returns requirements for a serial resource-free Responses session.
pub fn xai_responses_requirements(host: ExecutionHostId) -> OperationRequirements {
    OperationRequirements::new(
        ExecutionLayer::DirectModelInference,
        OperationShape::InteractiveSession,
        DriverRole::InteractiveSession,
        host,
        AccessRequirement::new(id(AccessProfileId::new, XAI_RESPONSES_ACCESS_PROFILE_ID))
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
    .with_capabilities(capabilities())
    .with_interface_versions([xai_responses_facade_binding()])
    .with_session_access_policy(SessionAccessPolicy::resource_free())
    .with_session_provider_state_policy(SessionProviderStatePolicy::Prohibited)
    .require_model_route()
}

#[must_use]
/// Returns requirements for a one-attempt Responses structured run.
pub fn xai_responses_run_requirements(host: ExecutionHostId) -> OperationRequirements {
    OperationRequirements::new(
        ExecutionLayer::DirectModelInference,
        OperationShape::StructuredRun,
        DriverRole::StructuredRun,
        host,
        AccessRequirement::new(id(AccessProfileId::new, XAI_RESPONSES_ACCESS_PROFILE_ID))
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
    .with_capabilities(run_capabilities())
    .with_interface_versions([xai_responses_facade_binding()])
    .require_model_route()
}

fn capabilities() -> Vec<CapabilityRequirement> {
    vec![
        CapabilityRequirement::new(Capability::InteractiveSession, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                CancellationScope::ActiveTurn,
            )],
        ),
        CapabilityRequirement::new(Capability::UsageReporting, []),
        CapabilityRequirement::new(Capability::BilledCostReporting, []),
    ]
}

fn run_capabilities() -> Vec<CapabilityRequirement> {
    vec![
        CapabilityRequirement::new(Capability::StructuredRun, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                CancellationScope::StructuredRun,
            )],
        ),
        CapabilityRequirement::new(Capability::UsageReporting, []),
        CapabilityRequirement::new(Capability::BilledCostReporting, []),
    ]
}

fn all_capabilities() -> Vec<CapabilityRequirement> {
    let mut requirements = capabilities();
    requirements.extend(run_capabilities());
    requirements
}

fn id<T, E>(constructor: impl FnOnce(String) -> Result<T, E>, value: &str) -> T
where
    E: std::fmt::Debug,
{
    constructor(value.to_owned()).expect("static xAI Responses identity is valid")
}
