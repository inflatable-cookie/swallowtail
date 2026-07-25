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
    ProviderId, RuntimeReadiness, SupportAuthority,
};

pub const OPENAI_BACKGROUND_ENDPOINT: &str = "https://api.openai.com";
pub const OPENAI_BACKGROUND_ENDPOINT_AUDIENCE: &str = "api.openai.com";
pub const OPENAI_BACKGROUND_ACCESS_PROFILE_ID: &str = "openai.public-api.api-key.payg";
pub const OPENAI_BACKGROUND_CONFIGURED_INSTANCE_ID: &str = "openai.public.responses-background";
pub const OPENAI_BACKGROUND_FACADE_REVISION: &str = "openai-responses-background-2026-07-21";
pub const OPENAI_BACKGROUND_MODEL_ID: &str = "gpt-5.6";
pub const OPENAI_BACKGROUND_MODEL_ROUTE_ID: &str = "openai.public.gpt-5.6.background";

const OPENAI_PROVIDER_ID: &str = "openai";
const OPENAI_BACKGROUND_FACADE_AXIS: &str = "openai.responses-background-facade";

#[must_use]
pub fn openai_background_access_profile(credential: CredentialRef) -> AccessProfile {
    AccessProfile::new(
        id(AccessProfileId::new, OPENAI_BACKGROUND_ACCESS_PROFILE_ID),
        CredentialMechanism::ApiKey,
        EntitlementMetering::PayAsYouGo,
        id(EndpointAudience::new, OPENAI_BACKGROUND_ENDPOINT_AUDIENCE),
        SupportAuthority::ProviderSupported,
    )
    .with_credential_reference(credential)
}

#[must_use]
pub fn openai_background_facade_binding() -> InterfaceVersionBinding {
    InterfaceVersionBinding::new(
        id(InterfaceVersionAxis::new, OPENAI_BACKGROUND_FACADE_AXIS),
        id(InterfaceVersion::new, OPENAI_BACKGROUND_FACADE_REVISION),
    )
}

#[must_use]
pub fn openai_background_facade_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        id(
            InterfaceCompatibilityClaimId::new,
            "openai.responses-background-window-1",
        ),
        id(InterfaceVersionAxis::new, OPENAI_BACKGROUND_FACADE_AXIS),
        InterfaceVersionScheme::Opaque,
        swallowtail_core::InterfaceNewerVersionPosture::QualifiedOnly,
        [InterfaceVersionSegment::exact(
            id(InterfaceVersion::new, OPENAI_BACKGROUND_FACADE_REVISION),
            id(
                InterfaceBehaviorRevision::new,
                "openai.responses-background-v1",
            ),
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("static OpenAI background facade claim is valid")
}

#[must_use]
pub fn openai_background_instance(
    revision: InstanceRevision,
    host: ExecutionHostId,
    endpoint_target: InstanceTargetRef,
    access_profile_id: AccessProfileId,
) -> ConfiguredInstance {
    ConfiguredInstance::new(
        id(
            ConfiguredInstanceId::new,
            OPENAI_BACKGROUND_CONFIGURED_INSTANCE_ID,
        ),
        revision,
        crate::openai_background_descriptor()
            .identity()
            .id()
            .clone(),
        host,
        endpoint_target,
        InstanceOwnership::ExternalAttached,
        access_profile_id,
        SupportAuthority::ProviderSupported,
        id(ProtocolFacadeId::new, OPENAI_BACKGROUND_FACADE_REVISION),
        id(
            InstancePolicyId::new,
            "openai-public-api-background-explicit-retention",
        ),
        CapabilityProfile::new(openai_background_capabilities()),
    )
    .with_interface_versions([openai_background_facade_binding()])
}

#[must_use]
pub fn openai_background_model_route(
    instance_id: ConfiguredInstanceId,
    revision: ModelRouteRevision,
) -> ModelRoute {
    ModelRoute::new(
        id(ModelRouteId::new, OPENAI_BACKGROUND_MODEL_ROUTE_ID),
        revision,
        instance_id,
        id(ModelId::new, OPENAI_BACKGROUND_MODEL_ID),
        CapabilityProfile::new(openai_background_capabilities()),
    )
    .with_provider_id(id(ProviderId::new, OPENAI_PROVIDER_ID))
}

#[must_use]
pub fn openai_background_requirements(host: ExecutionHostId) -> OperationRequirements {
    OperationRequirements::new(
        ExecutionLayer::DirectModelInference,
        OperationShape::StructuredRun,
        DriverRole::StructuredRun,
        host,
        AccessRequirement::new(id(
            AccessProfileId::new,
            OPENAI_BACKGROUND_ACCESS_PROFILE_ID,
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
    .with_capabilities(openai_background_capabilities())
    .with_interface_versions([openai_background_facade_binding()])
    .require_model_route()
}

fn openai_background_capabilities() -> Vec<CapabilityRequirement> {
    vec![
        CapabilityRequirement::new(Capability::StructuredRun, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(Capability::UsageReporting, []),
        CapabilityRequirement::new(Capability::OutputTokenLimit, []),
        CapabilityRequirement::new(Capability::ProviderBackgroundExecution, []),
        CapabilityRequirement::new(Capability::ProviderTemporaryRetention, []),
        CapabilityRequirement::new(
            Capability::StreamReattachment,
            [CapabilityConstraint::ReattachmentMaximumCount(1)],
        ),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                CancellationScope::StructuredRun,
            )],
        ),
    ]
}

fn id<T, E>(constructor: impl FnOnce(String) -> Result<T, E>, value: &str) -> T
where
    E: std::fmt::Debug,
{
    constructor(value.to_owned()).expect("static OpenAI background identity is valid")
}
