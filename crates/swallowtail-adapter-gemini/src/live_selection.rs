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

/// Exact WebSocket endpoint for the selected Gemini Live preview facade.
pub const GEMINI_LIVE_ENDPOINT: &str = "wss://generativelanguage.googleapis.com/ws/google.ai.generativelanguage.v1beta.GenerativeService.BidiGenerateContent";
/// Credential and network-grant audience for Gemini Live.
pub const GEMINI_LIVE_ENDPOINT_AUDIENCE: &str = "generativelanguage.googleapis.com";
/// Required project API-key access-profile identifier.
pub const GEMINI_LIVE_ACCESS_PROFILE_ID: &str = "gemini.authorization-api-key.project";
/// Stable configured-instance identifier for the hosted Live route.
pub const GEMINI_LIVE_CONFIGURED_INSTANCE_ID: &str = "gemini.public.live-preview";
/// Exact opaque protocol-facade revision qualified by this adapter.
pub const GEMINI_LIVE_FACADE_REVISION: &str = "google.generativelanguage.v1beta.GenerativeService.BidiGenerateContent.thinking-output-max-2026-08-23";
/// Historical facade point qualified before caller-selected thinking levels.
///
/// It names the frozen `gemini.live-preview-manual-pcm-rollover-v1` proof. It
/// is not a supported route claim, and the driver rejects plans that carry it.
pub const GEMINI_LIVE_SUPERSEDED_FACADE_REVISION: &str =
    "google.generativelanguage.v1beta.GenerativeService.BidiGenerateContent";
/// Historical facade point qualified for thinking levels before output maxima.
///
/// It names the frozen `gemini.live-preview-manual-pcm-rollover-thinking-v2`
/// proof. It is not a supported route claim, and the driver rejects plans that
/// carry it.
pub const GEMINI_LIVE_THINKING_SUPERSEDED_FACADE_REVISION: &str =
    "google.generativelanguage.v1beta.GenerativeService.BidiGenerateContent.thinking-2026-08-23";
/// Provider model identifier selected for the Live preview route.
pub const GEMINI_LIVE_MODEL_ID: &str = "gemini-3.1-flash-live-preview";
/// Stable Swallowtail model-route identifier for the Live preview model.
pub const GEMINI_LIVE_MODEL_ROUTE_ID: &str = "gemini-3-1-flash-live-preview";
/// Exact positive output-token maximum admitted for this Live model.
pub const GEMINI_LIVE_MAX_OUTPUT_TOKENS: u64 = 65_536;

const FACADE_AXIS: &str = "gemini.live-facade";

/// Builds the provider-supported project API-key access profile.
#[must_use]
pub fn gemini_live_access_profile(credential: CredentialRef) -> AccessProfile {
    AccessProfile::new(
        id(AccessProfileId::new, GEMINI_LIVE_ACCESS_PROFILE_ID),
        CredentialMechanism::ApiKey,
        EntitlementMetering::PayAsYouGo,
        id(EndpointAudience::new, GEMINI_LIVE_ENDPOINT_AUDIENCE),
        SupportAuthority::ProviderSupported,
    )
    .with_credential_reference(credential)
}

/// Returns the exact interface binding for the qualified Live facade.
#[must_use]
pub fn gemini_live_facade_binding() -> InterfaceVersionBinding {
    InterfaceVersionBinding::new(
        id(InterfaceVersionAxis::new, FACADE_AXIS),
        id(InterfaceVersion::new, GEMINI_LIVE_FACADE_REVISION),
    )
}

/// Returns the compatibility claim for the exact Live preview facade.
#[must_use]
pub fn gemini_live_facade_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        id(
            InterfaceCompatibilityClaimId::new,
            "gemini.live-preview-window-3",
        ),
        id(InterfaceVersionAxis::new, FACADE_AXIS),
        InterfaceVersionScheme::Opaque,
        swallowtail_core::InterfaceNewerVersionPosture::QualifiedOnly,
        [InterfaceVersionSegment::exact(
            id(InterfaceVersion::new, GEMINI_LIVE_FACADE_REVISION),
            id(
                InterfaceBehaviorRevision::new,
                "gemini.live-preview-manual-pcm-rollover-thinking-output-max-v3",
            ),
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("static Gemini Live facade claim is valid")
}

/// Returns the fixed PCM media formats and bounded buffering contract.
#[must_use]
pub fn gemini_live_media_config() -> RealtimeMediaConfig {
    let input = MediaFormat::audio(
        AudioEncoding::Pcm16LittleEndian,
        NonZeroU32::new(16_000).expect("sample rate is non-zero"),
        NonZeroU16::new(1).expect("channel count is non-zero"),
    );
    let output = MediaFormat::audio(
        AudioEncoding::Pcm16LittleEndian,
        NonZeroU32::new(24_000).expect("sample rate is non-zero"),
        NonZeroU16::new(1).expect("channel count is non-zero"),
    );
    RealtimeMediaConfig::new(
        input,
        output,
        NonZeroU64::new(32_768).expect("chunk bound is non-zero"),
        NonZeroU32::new(2).expect("turn bound is non-zero"),
    )
}

/// Permits exactly one planned connection rollover per prepared session.
#[must_use]
pub fn gemini_live_rollover_policy() -> PlannedConnectionRolloverPolicy {
    PlannedConnectionRolloverPolicy::Bounded(
        NonZeroU32::new(1).expect("rollover bound is non-zero"),
    )
}

/// Builds the externally attached configured instance for Gemini Live.
#[must_use]
pub fn gemini_live_instance(
    revision: InstanceRevision,
    host: ExecutionHostId,
    target: InstanceTargetRef,
    access_profile_id: AccessProfileId,
) -> ConfiguredInstance {
    ConfiguredInstance::new(
        id(
            ConfiguredInstanceId::new,
            GEMINI_LIVE_CONFIGURED_INSTANCE_ID,
        ),
        revision,
        crate::gemini_live_descriptor().identity().id().clone(),
        host,
        target,
        InstanceOwnership::ExternalAttached,
        access_profile_id,
        SupportAuthority::ProviderSupported,
        id(ProtocolFacadeId::new, GEMINI_LIVE_FACADE_REVISION),
        id(
            InstancePolicyId::new,
            "gemini-live-preview-authorization-key-manual-audio",
        ),
        CapabilityProfile::new(capabilities()),
    )
    .with_interface_versions([gemini_live_facade_binding()])
}

/// Builds the exact Live preview model route for a configured instance.
#[must_use]
pub fn gemini_live_model_route(
    instance_id: ConfiguredInstanceId,
    revision: ModelRouteRevision,
) -> ModelRoute {
    ModelRoute::new(
        id(ModelRouteId::new, GEMINI_LIVE_MODEL_ROUTE_ID),
        revision,
        instance_id,
        id(ModelId::new, GEMINI_LIVE_MODEL_ID),
        CapabilityProfile::new(capabilities()),
    )
    .with_provider_id(id(ProviderId::new, "gemini"))
}

/// Declares host, access, capability, media, and rollover requirements.
#[must_use]
pub fn gemini_live_requirements(host: ExecutionHostId) -> OperationRequirements {
    gemini_live_requirements_with_capabilities(host, capabilities())
}

/// Declares the same fixed route with an explicit operation capability set.
#[must_use]
pub fn gemini_live_requirements_with_capabilities(
    host: ExecutionHostId,
    capabilities: impl IntoIterator<Item = CapabilityRequirement>,
) -> OperationRequirements {
    OperationRequirements::new(
        ExecutionLayer::DirectModelInference,
        OperationShape::InteractiveSession,
        DriverRole::RealtimeMediaSession,
        host,
        AccessRequirement::new(id(AccessProfileId::new, GEMINI_LIVE_ACCESS_PROFILE_ID))
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
    .with_interface_versions([gemini_live_facade_binding()])
    .with_session_access_policy(SessionAccessPolicy::resource_free())
    .with_session_provider_state_policy(SessionProviderStatePolicy::Prohibited)
    .with_realtime_media(RealtimeMediaRequirements::new(
        id(ModelId::new, GEMINI_LIVE_MODEL_ID),
        gemini_live_media_config(),
    ))
    .with_planned_connection_rollover(gemini_live_rollover_policy())
    .require_model_route()
}

/// Returns the fixed capability set the route always requires.
#[must_use]
pub fn gemini_live_base_capabilities() -> Vec<CapabilityRequirement> {
    capabilities()
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
        gemini_live_media_config().capability_requirement(),
        CapabilityRequirement::new(
            Capability::PlannedConnectionRollover,
            [CapabilityConstraint::PlannedConnectionRolloverMaximumCount(
                1,
            )],
        ),
    ]
}

fn id<T, E>(constructor: impl FnOnce(String) -> Result<T, E>, value: &str) -> T
where
    E: std::fmt::Debug,
{
    constructor(value.to_owned()).expect("static Gemini Live identity is valid")
}
