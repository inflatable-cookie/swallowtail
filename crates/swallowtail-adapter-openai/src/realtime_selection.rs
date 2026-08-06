use std::num::{NonZeroU16, NonZeroU32, NonZeroU64};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AudioEncoding, CancellationScope,
    Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    ConfiguredInstanceId, CredentialMechanism, CredentialRef, CredentialState, DriverRole,
    EndpointAudience, EndpointAuthorization, EntitlementMetering, EntitlementState,
    ExecutionHostId, ExecutionLayer, HostServiceKind, InstanceOwnership, InstancePolicyId,
    InstanceRevision, InstanceTargetRef, InterfaceBehaviorRevision, InterfaceCompatibilityClaim,
    InterfaceCompatibilityClaimId, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment, MediaFormat, ModelId,
    ModelRoute, ModelRouteId, ModelRouteRevision, OperationRequirements, OperationShape,
    PlannedConnectionRolloverPolicy, ProtocolFacadeId, ProviderId, RealtimeMediaConfig,
    RealtimeMediaRequirements, RuntimeReadiness, SessionAccessPolicy, SessionProviderStatePolicy,
    SupportAuthority,
};

/// Exact OpenAI Realtime WebSocket endpoint.
pub const OPENAI_REALTIME_ENDPOINT: &str = "wss://api.openai.com/v1/realtime";
/// Credential audience required by the Realtime route.
pub const OPENAI_REALTIME_ENDPOINT_AUDIENCE: &str = "api.openai.com";
/// Canonical public API-key access-profile identity for Realtime.
pub const OPENAI_REALTIME_ACCESS_PROFILE_ID: &str = "openai.realtime.public-api.api-key.payg";
/// Canonical configured-instance identity for OpenAI Realtime.
pub const OPENAI_REALTIME_CONFIGURED_INSTANCE_ID: &str = "openai.public.realtime";
/// Exact qualified revision of the Realtime facade.
pub const OPENAI_REALTIME_FACADE_REVISION: &str = "openai-realtime-2026-07-22";
/// Exact model identity qualified for the Realtime route.
pub const OPENAI_REALTIME_MODEL_ID: &str = "gpt-realtime-2.1";
/// Canonical model-route identity for OpenAI Realtime.
pub const OPENAI_REALTIME_MODEL_ROUTE_ID: &str = "openai.public.gpt-realtime-2.1";

const FACADE_AXIS: &str = "openai.realtime-facade";

#[must_use]
/// Builds the provider-supported Realtime API-key access profile.
pub fn openai_realtime_access_profile(credential: CredentialRef) -> AccessProfile {
    AccessProfile::new(
        id(AccessProfileId::new, OPENAI_REALTIME_ACCESS_PROFILE_ID),
        CredentialMechanism::ApiKey,
        EntitlementMetering::PayAsYouGo,
        id(EndpointAudience::new, OPENAI_REALTIME_ENDPOINT_AUDIENCE),
        SupportAuthority::ProviderSupported,
    )
    .with_credential_reference(credential)
}

#[must_use]
/// Returns the exact interface-version binding for OpenAI Realtime.
pub fn openai_realtime_facade_binding() -> InterfaceVersionBinding {
    InterfaceVersionBinding::new(
        id(InterfaceVersionAxis::new, FACADE_AXIS),
        id(InterfaceVersion::new, OPENAI_REALTIME_FACADE_REVISION),
    )
}

#[must_use]
/// Returns the qualified-only OpenAI Realtime compatibility claim.
pub fn openai_realtime_facade_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        id(
            InterfaceCompatibilityClaimId::new,
            "openai.realtime-window-1",
        ),
        id(InterfaceVersionAxis::new, FACADE_AXIS),
        InterfaceVersionScheme::Opaque,
        swallowtail_core::InterfaceNewerVersionPosture::QualifiedOnly,
        [InterfaceVersionSegment::exact(
            id(InterfaceVersion::new, OPENAI_REALTIME_FACADE_REVISION),
            id(
                InterfaceBehaviorRevision::new,
                "openai.realtime-manual-pcm-v1",
            ),
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("static OpenAI Realtime facade claim is valid")
}

#[must_use]
/// Returns the exact manual mono 24 kHz PCM session configuration.
pub fn openai_realtime_media_config() -> RealtimeMediaConfig {
    let format = MediaFormat::audio(
        AudioEncoding::Pcm16LittleEndian,
        NonZeroU32::new(24_000).expect("sample rate is non-zero"),
        NonZeroU16::new(1).expect("channel count is non-zero"),
    );
    RealtimeMediaConfig::new(
        format,
        format,
        NonZeroU64::new(32_768).expect("chunk bound is non-zero"),
        NonZeroU32::new(2).expect("turn bound is non-zero"),
    )
}

#[must_use]
/// Builds the configured Realtime instance for one host and endpoint.
pub fn openai_realtime_instance(
    revision: InstanceRevision,
    host: ExecutionHostId,
    target: InstanceTargetRef,
    access_profile_id: AccessProfileId,
) -> ConfiguredInstance {
    ConfiguredInstance::new(
        id(
            ConfiguredInstanceId::new,
            OPENAI_REALTIME_CONFIGURED_INSTANCE_ID,
        ),
        revision,
        crate::openai_realtime_descriptor().identity().id().clone(),
        host,
        target,
        InstanceOwnership::ExternalAttached,
        access_profile_id,
        SupportAuthority::ProviderSupported,
        id(ProtocolFacadeId::new, OPENAI_REALTIME_FACADE_REVISION),
        id(InstancePolicyId::new, "openai-realtime-manual-pcm"),
        CapabilityProfile::new(capabilities()),
    )
    .with_interface_versions([openai_realtime_facade_binding()])
}

#[must_use]
/// Builds the exact GPT Realtime model route.
pub fn openai_realtime_model_route(
    instance_id: ConfiguredInstanceId,
    revision: ModelRouteRevision,
) -> ModelRoute {
    ModelRoute::new(
        id(ModelRouteId::new, OPENAI_REALTIME_MODEL_ROUTE_ID),
        revision,
        instance_id,
        id(ModelId::new, OPENAI_REALTIME_MODEL_ID),
        CapabilityProfile::new(capabilities()),
    )
    .with_provider_id(id(ProviderId::new, "openai"))
}

#[must_use]
/// Builds requirements for a resource-free manual PCM Realtime session.
pub fn openai_realtime_requirements(
    host: ExecutionHostId,
    capabilities: impl IntoIterator<Item = CapabilityRequirement>,
) -> OperationRequirements {
    OperationRequirements::new(
        ExecutionLayer::DirectModelInference,
        OperationShape::InteractiveSession,
        DriverRole::RealtimeMediaSession,
        host,
        AccessRequirement::new(id(AccessProfileId::new, OPENAI_REALTIME_ACCESS_PROFILE_ID))
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
    .with_interface_versions([openai_realtime_facade_binding()])
    .with_session_access_policy(SessionAccessPolicy::resource_free())
    .with_session_provider_state_policy(SessionProviderStatePolicy::Prohibited)
    .with_realtime_media(RealtimeMediaRequirements::new(
        id(ModelId::new, OPENAI_REALTIME_MODEL_ID),
        openai_realtime_media_config(),
    ))
    .with_planned_connection_rollover(PlannedConnectionRolloverPolicy::Disabled)
    .require_model_route()
}

fn capabilities() -> Vec<CapabilityRequirement> {
    vec![
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(Capability::UsageReporting, []),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                CancellationScope::ActiveResponse,
            )],
        ),
        openai_realtime_media_config().capability_requirement(),
    ]
}

fn id<T, E>(constructor: impl FnOnce(String) -> Result<T, E>, value: &str) -> T
where
    E: std::fmt::Debug,
{
    constructor(value.to_owned()).expect("static OpenAI Realtime identity is valid")
}
