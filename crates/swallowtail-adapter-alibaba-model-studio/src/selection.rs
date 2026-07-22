use crate::failure::AlibabaProtocolFailure;
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AdapterId, AdapterIdentity, AdapterVersion,
    Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    ConfiguredInstanceId, CredentialMechanism, CredentialRef, CredentialState, DriverDescriptor,
    DriverRole, EndpointAudience, EndpointAuthorization, EntitlementMetering, EntitlementState,
    ExecutionHostId, ExecutionLayer, HostServiceKind, InstanceOwnership, InstancePolicyId,
    InstanceRevision, InstanceTargetRef, IntegrationFamilyId, ModelId, ModelRoute, ModelRouteId,
    ModelRouteRevision, OperationRequirements, OperationShape, OwnedRemoteResourceKind,
    PreflightPlan, ProtocolFacadeId, RuntimeReadiness, SessionAccessPolicy,
    SessionProviderStatePolicy, SupportAuthority, TransportFamilyId,
};

pub const EVIDENCE_DATE: &str = "2026-07-22";
pub const REGION: &str = "ap-southeast-1";
pub const WORKSPACE_ENDPOINT_TEMPLATE: &str =
    "https://{WorkspaceId}.ap-southeast-1.maas.aliyuncs.com";
pub const ENDPOINT_AUDIENCE: &str = "model-studio.workspace.ap-southeast-1";
pub const EXACT_MODEL_ID: &str = "qwen3.7-plus-2026-05-26";
pub const ACCESS_PROFILE_ID: &str = "alibaba-model-studio.sg.workspace.api-key.payg";

const DRIVER_ID: &str = "swallowtail.alibaba-model-studio.conversations-responses";
const INSTANCE_ID: &str = "alibaba-model-studio.sg.workspace-dedicated";
const ROUTE_ID: &str = "alibaba-model-studio.sg.qwen3.7-plus-2026-05-26";

#[must_use]
pub fn alibaba_model_studio_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            id(AdapterId::new, DRIVER_ID),
            id(AdapterVersion::new, env!("CARGO_PKG_VERSION")),
        ),
        id(IntegrationFamilyId::new, "alibaba-model-studio"),
        id(TransportFamilyId::new, "https-sse"),
    )
    .with_roles([DriverRole::InteractiveSession])
    .with_execution_layers([ExecutionLayer::DirectModelInference])
    .with_operation_shapes([OperationShape::InteractiveSession])
    .with_required_host_services(DriverRole::InteractiveSession, host_services())
}

#[must_use]
pub fn alibaba_model_studio_access_profile() -> AccessProfile {
    AccessProfile::new(
        id(AccessProfileId::new, ACCESS_PROFILE_ID),
        CredentialMechanism::ApiKey,
        EntitlementMetering::PayAsYouGo,
        id(EndpointAudience::new, ENDPOINT_AUDIENCE),
        SupportAuthority::ProviderSupported,
    )
    .with_credential_reference(id(
        CredentialRef::new,
        "alibaba-model-studio.sg.workspace.general-api-key",
    ))
}

#[must_use]
pub fn alibaba_model_studio_instance(host_id: ExecutionHostId) -> ConfiguredInstance {
    ConfiguredInstance::new(
        id(ConfiguredInstanceId::new, INSTANCE_ID),
        id(InstanceRevision::new, "fixture-1"),
        id(AdapterId::new, DRIVER_ID),
        host_id,
        id(
            InstanceTargetRef::new,
            "alibaba-model-studio.sg.workspace-endpoint",
        ),
        InstanceOwnership::ExternalAttached,
        id(AccessProfileId::new, ACCESS_PROFILE_ID),
        SupportAuthority::ProviderSupported,
        id(ProtocolFacadeId::new, "openai-conversations-responses"),
        id(InstancePolicyId::new, "alibaba-model-studio.sg.exact-route"),
        CapabilityProfile::new(capabilities()),
    )
}

#[must_use]
pub fn alibaba_model_studio_route() -> ModelRoute {
    ModelRoute::new(
        id(ModelRouteId::new, ROUTE_ID),
        id(ModelRouteRevision::new, "fixture-1"),
        id(ConfiguredInstanceId::new, INSTANCE_ID),
        id(ModelId::new, EXACT_MODEL_ID),
        CapabilityProfile::new(capabilities()),
    )
}

#[must_use]
pub fn alibaba_model_studio_requirements(host_id: ExecutionHostId) -> OperationRequirements {
    let access = AccessRequirement::new(id(AccessProfileId::new, ACCESS_PROFILE_ID))
        .with_credential_states([CredentialState::Ready])
        .with_entitlement_states([EntitlementState::Available])
        .with_endpoint_authorizations([EndpointAuthorization::Allowed])
        .with_runtime_readiness([RuntimeReadiness::Ready])
        .with_support_authorities([SupportAuthority::ProviderSupported]);
    OperationRequirements::new(
        ExecutionLayer::DirectModelInference,
        OperationShape::InteractiveSession,
        DriverRole::InteractiveSession,
        host_id,
        access,
    )
    .with_ownership_modes([InstanceOwnership::ExternalAttached])
    .with_host_services(host_services())
    .with_capabilities(capabilities())
    .with_session_access_policy(SessionAccessPolicy::resource_free())
    .with_session_provider_state_policy(
        SessionProviderStatePolicy::DurableConversationDeleteOnClose,
    )
    .require_model_route()
}

pub fn validate_alibaba_model_studio_plan(
    plan: &PreflightPlan,
) -> Result<(), AlibabaProtocolFailure> {
    let requirements = plan.requirements();
    if plan.driver_identity().id().as_str() != DRIVER_ID
        || plan.instance_id().as_str() != INSTANCE_ID
        || plan.instance_target_ref()
            != &id(
                InstanceTargetRef::new,
                "alibaba-model-studio.sg.workspace-endpoint",
            )
        || plan.access_profile_id().as_str() != ACCESS_PROFILE_ID
        || plan.endpoint_audience().as_str() != ENDPOINT_AUDIENCE
        || plan.credential_mechanism() != &CredentialMechanism::ApiKey
        || plan.model_route_id().map(ModelRouteId::as_str) != Some(ROUTE_ID)
        || plan.model_id().map(ModelId::as_str) != Some(EXACT_MODEL_ID)
        || requirements.execution_layer() != ExecutionLayer::DirectModelInference
        || requirements.operation_shape() != OperationShape::InteractiveSession
        || requirements.session_access_policy() != Some(&SessionAccessPolicy::resource_free())
        || requirements.session_provider_state_policy()
            != Some(SessionProviderStatePolicy::DurableConversationDeleteOnClose)
        || requirements
            .capabilities()
            .any(|requirement| requirement.capability() == Capability::Resume)
    {
        return Err(AlibabaProtocolFailure::invalid(
            "preflight-bound Singapore workspace selection",
        ));
    }
    Ok(())
}

fn capabilities() -> Vec<CapabilityRequirement> {
    vec![
        CapabilityRequirement::new(
            Capability::InteractiveSession,
            [
                CapabilityConstraint::MaximumConcurrency(1),
                CapabilityConstraint::MaximumTurns(2),
            ],
        ),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(Capability::UsageReporting, []),
        CapabilityRequirement::new(Capability::ProviderDurableRetention, []),
        CapabilityRequirement::new(
            Capability::OwnedRemoteResourceDeletion,
            [
                CapabilityConstraint::OwnedRemoteResource(
                    OwnedRemoteResourceKind::ConversationItems,
                ),
                CapabilityConstraint::OwnedRemoteResource(OwnedRemoteResourceKind::Conversation),
            ],
        ),
    ]
}

fn host_services() -> [HostServiceKind; 5] {
    [
        HostServiceKind::Task,
        HostServiceKind::BlockingWork,
        HostServiceKind::Time,
        HostServiceKind::Network,
        HostServiceKind::Credential,
    ]
}

fn id<T, E>(constructor: impl FnOnce(String) -> Result<T, E>, value: &str) -> T
where
    E: std::fmt::Debug,
{
    constructor(value.to_owned()).expect("static Alibaba fixture identity is valid")
}
