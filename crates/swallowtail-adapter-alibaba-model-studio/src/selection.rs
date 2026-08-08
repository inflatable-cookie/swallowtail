use crate::failure::AlibabaProtocolFailure;
use crate::protocol::{MAXIMUM_REPLAY_PAGE_BYTES, MAXIMUM_REPLAY_PAGE_ITEMS};
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

/// Date of the protocol evidence qualifying this route.
pub const EVIDENCE_DATE: &str = "2026-07-22";
/// Exact Alibaba workspace region qualified by the adapter.
pub const REGION: &str = "ap-southeast-1";
/// Provider workspace endpoint template; the host supplies the approved expansion.
pub const WORKSPACE_ENDPOINT_TEMPLATE: &str =
    "https://{WorkspaceId}.ap-southeast-1.maas.aliyuncs.com";
/// Credential audience for the qualified Singapore workspace endpoint.
pub const ENDPOINT_AUDIENCE: &str = "model-studio.workspace.ap-southeast-1";
/// Exact model identity qualified for the workspace route.
pub const EXACT_MODEL_ID: &str = "qwen3.7-plus-2026-05-26";
/// Canonical workspace API-key access-profile identity.
pub const ACCESS_PROFILE_ID: &str = "alibaba-model-studio.sg.workspace.api-key.payg";
pub const FACADE_REVISION: &str = "model-studio-2026-07-22";

const DRIVER_ID: &str = "swallowtail.alibaba-model-studio.conversations-responses";
/// Canonical configured-instance identity for the dedicated workspace.
pub const CONFIGURED_INSTANCE_ID: &str = "alibaba-model-studio.sg.workspace-dedicated";
/// Canonical model-route identity for the qualified Qwen model.
pub const MODEL_ROUTE_ID: &str = "alibaba-model-studio.sg.qwen3.7-plus-2026-05-26";

#[must_use]
/// Returns the descriptor for conversation, structured-run, and management roles.
pub fn alibaba_model_studio_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            id(AdapterId::new, DRIVER_ID),
            id(AdapterVersion::new, env!("CARGO_PKG_VERSION")),
        ),
        id(IntegrationFamilyId::new, "alibaba-model-studio"),
        id(TransportFamilyId::new, "https-sse"),
    )
    .with_interface_compatibility(alibaba_model_studio_facade_claim())
    .with_roles([
        DriverRole::InteractiveSession,
        DriverRole::StructuredRun,
        DriverRole::ProviderSessionManagement,
        DriverRole::ProviderSessionHistory,
    ])
    .with_execution_layers([ExecutionLayer::DirectModelInference])
    .with_operation_shapes([
        OperationShape::InteractiveSession,
        OperationShape::StructuredRun,
        OperationShape::ProviderSessionManagement,
        OperationShape::ProviderSessionHistory,
    ])
    .with_required_host_services(DriverRole::InteractiveSession, host_services())
    .with_required_host_services(DriverRole::StructuredRun, host_services())
    .with_required_host_services(DriverRole::ProviderSessionManagement, host_services())
    .with_required_host_services(DriverRole::ProviderSessionHistory, host_services())
}

#[must_use]
/// Builds the provider-supported Singapore workspace API-key profile.
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
/// Builds the configured workspace instance for one execution host.
pub fn alibaba_model_studio_instance(host_id: ExecutionHostId) -> ConfiguredInstance {
    ConfiguredInstance::new(
        id(ConfiguredInstanceId::new, CONFIGURED_INSTANCE_ID),
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
        id(ProtocolFacadeId::new, FACADE_REVISION),
        id(InstancePolicyId::new, "alibaba-model-studio.sg.exact-route"),
        CapabilityProfile::new(instance_capabilities()),
    )
    .with_interface_versions([alibaba_model_studio_facade_binding()])
}

#[must_use]
/// Returns the exact interface binding for the workspace Responses facade.
pub fn alibaba_model_studio_facade_binding() -> swallowtail_core::InterfaceVersionBinding {
    swallowtail_core::InterfaceVersionBinding::new(
        swallowtail_core::InterfaceVersionAxis::new("alibaba-model-studio.responses-facade")
            .expect("static Alibaba interface axis is valid"),
        swallowtail_core::InterfaceVersion::new(FACADE_REVISION)
            .expect("static Alibaba facade revision is valid"),
    )
}

#[must_use]
/// Returns the qualified-only workspace Responses compatibility claim.
pub fn alibaba_model_studio_facade_claim() -> swallowtail_core::InterfaceCompatibilityClaim {
    swallowtail_core::InterfaceCompatibilityClaim::new(
        swallowtail_core::InterfaceCompatibilityClaimId::new(
            "alibaba-model-studio-responses-window-1",
        )
        .expect("static Alibaba compatibility claim is valid"),
        swallowtail_core::InterfaceVersionAxis::new("alibaba-model-studio.responses-facade")
            .expect("static Alibaba interface axis is valid"),
        swallowtail_core::InterfaceVersionScheme::Opaque,
        swallowtail_core::InterfaceNewerVersionPosture::QualifiedOnly,
        [swallowtail_core::InterfaceVersionSegment::exact(
            swallowtail_core::InterfaceVersion::new(FACADE_REVISION)
                .expect("static Alibaba facade revision is valid"),
            swallowtail_core::InterfaceBehaviorRevision::new("alibaba-responses-text-v1")
                .expect("static Alibaba behavior revision is valid"),
            swallowtail_core::InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("static Alibaba compatibility claim is valid")
}

#[must_use]
/// Builds the exact configured model route.
pub fn alibaba_model_studio_route() -> ModelRoute {
    ModelRoute::new(
        id(ModelRouteId::new, MODEL_ROUTE_ID),
        id(ModelRouteRevision::new, "fixture-1"),
        id(ConfiguredInstanceId::new, CONFIGURED_INSTANCE_ID),
        id(ModelId::new, EXACT_MODEL_ID),
        CapabilityProfile::new(all_capabilities()),
    )
}

#[must_use]
/// Builds delete-on-close conversation requirements.
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

#[must_use]
/// Builds requirements for one unstored, tool-free structured response.
pub fn alibaba_model_studio_run_requirements(host_id: ExecutionHostId) -> OperationRequirements {
    let access = AccessRequirement::new(id(AccessProfileId::new, ACCESS_PROFILE_ID))
        .with_credential_states([CredentialState::Ready])
        .with_entitlement_states([EntitlementState::Available])
        .with_endpoint_authorizations([EndpointAuthorization::Allowed])
        .with_runtime_readiness([RuntimeReadiness::Ready])
        .with_support_authorities([SupportAuthority::ProviderSupported]);
    OperationRequirements::new(
        ExecutionLayer::DirectModelInference,
        OperationShape::StructuredRun,
        DriverRole::StructuredRun,
        host_id,
        access,
    )
    .with_ownership_modes([InstanceOwnership::ExternalAttached])
    .with_host_services(host_services())
    .with_capabilities(run_capabilities())
    .require_model_route()
}

#[must_use]
/// Builds requirements for a preserved, loadable conversation.
pub fn alibaba_model_studio_retained_requirements(
    host_id: ExecutionHostId,
) -> OperationRequirements {
    interactive_requirements(
        host_id,
        SessionProviderStatePolicy::DurableProviderSessionPreserved,
    )
    .with_capabilities(retained_capabilities())
}

#[must_use]
/// Builds requirements for read-only retained conversation history pages.
pub fn alibaba_model_studio_history_requirements(
    host_id: ExecutionHostId,
) -> OperationRequirements {
    let access = AccessRequirement::new(id(AccessProfileId::new, ACCESS_PROFILE_ID))
        .with_credential_states([CredentialState::Ready])
        .with_entitlement_states([EntitlementState::Available])
        .with_endpoint_authorizations([EndpointAuthorization::Allowed])
        .with_runtime_readiness([RuntimeReadiness::Ready])
        .with_support_authorities([SupportAuthority::ProviderSupported]);
    OperationRequirements::new(
        ExecutionLayer::DirectModelInference,
        OperationShape::ProviderSessionHistory,
        DriverRole::ProviderSessionHistory,
        host_id,
        access,
    )
    .with_ownership_modes([InstanceOwnership::ExternalAttached])
    .with_host_services(host_services())
    .with_capabilities([
        history_capability(),
        CapabilityRequirement::new(Capability::ProviderDurableRetention, []),
    ])
    .with_session_access_policy(SessionAccessPolicy::resource_free())
    .with_session_provider_state_policy(
        SessionProviderStatePolicy::DurableProviderSessionPreserved,
    )
    .require_model_route()
}

#[must_use]
/// Builds inactive-session requirements for retained conversation deletion.
pub fn alibaba_model_studio_management_requirements(
    host_id: ExecutionHostId,
) -> OperationRequirements {
    let access = AccessRequirement::new(id(AccessProfileId::new, ACCESS_PROFILE_ID))
        .with_credential_states([CredentialState::Ready])
        .with_entitlement_states([EntitlementState::Available])
        .with_endpoint_authorizations([EndpointAuthorization::Allowed])
        .with_runtime_readiness([RuntimeReadiness::Ready])
        .with_support_authorities([SupportAuthority::ProviderSupported]);
    OperationRequirements::new(
        ExecutionLayer::DirectModelInference,
        OperationShape::ProviderSessionManagement,
        DriverRole::ProviderSessionManagement,
        host_id,
        access,
    )
    .with_ownership_modes([InstanceOwnership::ExternalAttached])
    .with_host_services(host_services())
    .with_capabilities([CapabilityRequirement::new(
        Capability::ProviderSessionDelete,
        [],
    )])
}

/// Validates that a plan matches one exact qualified workspace operation.
pub fn validate_alibaba_model_studio_plan(
    plan: &PreflightPlan,
) -> Result<(), AlibabaProtocolFailure> {
    let requirements = plan.requirements();
    let interactive_state = requirements.session_provider_state_policy();
    let interactive = requirements.driver_role() == DriverRole::InteractiveSession
        && requirements.operation_shape() == OperationShape::InteractiveSession
        && requirements.session_access_policy() == Some(&SessionAccessPolicy::resource_free())
        && matches!(
            interactive_state,
            Some(SessionProviderStatePolicy::DurableConversationDeleteOnClose)
                | Some(SessionProviderStatePolicy::DurableProviderSessionPreserved)
        )
        && (interactive_state != Some(SessionProviderStatePolicy::DurableProviderSessionPreserved)
            || requirements
                .capabilities()
                .any(|required| required.capability() == Capability::LoadSession));
    let structured = requirements.driver_role() == DriverRole::StructuredRun
        && requirements.operation_shape() == OperationShape::StructuredRun
        && requirements.session_access_policy().is_none()
        && requirements.session_provider_state_policy().is_none();
    let management = requirements.driver_role() == DriverRole::ProviderSessionManagement
        && requirements.operation_shape() == OperationShape::ProviderSessionManagement
        && requirements.session_access_policy().is_none()
        && requirements.session_provider_state_policy().is_none()
        && requirements
            .capabilities()
            .any(|required| required.capability() == Capability::ProviderSessionDelete);
    let history = requirements.driver_role() == DriverRole::ProviderSessionHistory
        && requirements.operation_shape() == OperationShape::ProviderSessionHistory
        && requirements.session_access_policy() == Some(&SessionAccessPolicy::resource_free())
        && requirements.session_provider_state_policy()
            == Some(SessionProviderStatePolicy::DurableProviderSessionPreserved)
        && requirements
            .capabilities()
            .any(|required| required.capability() == Capability::ProviderSessionHistory)
        && !requirements
            .capabilities()
            .any(|required| required.capability() == Capability::LoadSession);
    if plan.driver_identity().id().as_str() != DRIVER_ID
        || plan.instance_id().as_str() != CONFIGURED_INSTANCE_ID
        || plan.instance_target_ref()
            != &id(
                InstanceTargetRef::new,
                "alibaba-model-studio.sg.workspace-endpoint",
            )
        || plan.access_profile_id().as_str() != ACCESS_PROFILE_ID
        || plan.endpoint_audience().as_str() != ENDPOINT_AUDIENCE
        || plan.credential_mechanism() != &CredentialMechanism::ApiKey
        || (!management
            && (plan.model_route_id().map(ModelRouteId::as_str) != Some(MODEL_ROUTE_ID)
                || plan.model_id().map(ModelId::as_str) != Some(EXACT_MODEL_ID)))
        || (management && (plan.model_route_id().is_some() || plan.model_id().is_some()))
        || requirements.execution_layer() != ExecutionLayer::DirectModelInference
        || !(interactive || structured || management || history)
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

fn run_capabilities() -> Vec<CapabilityRequirement> {
    [
        Capability::StructuredRun,
        Capability::StreamingEvents,
        Capability::UsageReporting,
        Capability::Interruption,
    ]
    .into_iter()
    .map(|capability| CapabilityRequirement::new(capability, []))
    .collect()
}

fn all_capabilities() -> Vec<CapabilityRequirement> {
    let mut requirements = capabilities();
    requirements.push(CapabilityRequirement::new(Capability::LoadSession, []));
    requirements.push(history_capability());
    requirements.extend(run_capabilities());
    requirements
}

fn history_capability() -> CapabilityRequirement {
    CapabilityRequirement::new(
        Capability::ProviderSessionHistory,
        [
            CapabilityConstraint::ReplayMaximumItems(MAXIMUM_REPLAY_PAGE_ITEMS as u32),
            CapabilityConstraint::ReplayMaximumBytes(MAXIMUM_REPLAY_PAGE_BYTES as u64),
        ],
    )
}

fn instance_capabilities() -> Vec<CapabilityRequirement> {
    let mut requirements = all_capabilities();
    requirements.push(CapabilityRequirement::new(
        Capability::ProviderSessionDelete,
        [],
    ));
    requirements
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

fn retained_capabilities() -> Vec<CapabilityRequirement> {
    let mut requirements = capabilities()
        .into_iter()
        .filter(|required| required.capability() != Capability::OwnedRemoteResourceDeletion)
        .collect::<Vec<_>>();
    requirements.push(CapabilityRequirement::new(Capability::LoadSession, []));
    requirements
}

fn interactive_requirements(
    host_id: ExecutionHostId,
    provider_state: SessionProviderStatePolicy,
) -> OperationRequirements {
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
    .with_session_access_policy(SessionAccessPolicy::resource_free())
    .with_session_provider_state_policy(provider_state)
    .require_model_route()
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
