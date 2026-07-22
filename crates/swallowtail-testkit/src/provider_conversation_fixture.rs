use std::cell::Cell;
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, AdapterId, AdapterIdentity,
    AdapterVersion, Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement,
    ConfiguredInstance, ConfiguredInstanceId, CredentialMechanism, CredentialRef, CredentialState,
    DriverDescriptor, DriverRole, EndpointAudience, EndpointAuthorization, EntitlementMetering,
    EntitlementState, ExecutionHostId, ExecutionLayer, HostServiceKind, InstanceOwnership,
    InstancePolicyId, InstanceRevision, InstanceTargetRef, IntegrationFamilyId, ModelId,
    ModelRoute, ModelRouteId, ModelRouteRevision, OperationRequirements, OperationShape,
    OwnedRemoteResourceKind, PreflightContext, PreflightFailure, PreflightPlan, ProtocolFacadeId,
    RuntimeReadiness, SessionAccessPolicy, SessionProviderStatePolicy, SupportAuthority,
    TransportFamilyId, preflight,
};
use swallowtail_runtime::{OpenSessionRequest, RequestId};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderConversationPreflightCase {
    Canonical,
    PolicyProhibited,
    MissingDurableRequirement,
    MissingConversationDeletionRequirement,
    MissingItemDeletionRequirement,
    AdvertisedMissingDurable,
    AdvertisedMissingConversationDeletion,
    AdvertisedMissingItemDeletion,
}

pub struct ProviderConversationPreflightFixture {
    driver: DriverDescriptor,
    instance: ConfiguredInstance,
    route: ModelRoute,
    access_profile: AccessProfile,
    access_status: AccessStatus,
    requirements: OperationRequirements,
    provider_side_effects: Cell<usize>,
}

impl ProviderConversationPreflightFixture {
    #[must_use]
    pub fn for_case(case: ProviderConversationPreflightCase) -> Self {
        let adapter_id = valid(AdapterId::new, "fixture.provider-conversation");
        let instance_id = valid(ConfiguredInstanceId::new, "fixture.conversation-instance");
        let profile_id = valid(AccessProfileId::new, "fixture.conversation-access");
        let host_id = valid(ExecutionHostId::new, "fixture.conversation-host");
        let advertised = capabilities(case, true);
        let required = capabilities(case, false);
        let driver = DriverDescriptor::new(
            AdapterIdentity::new(adapter_id.clone(), valid(AdapterVersion::new, "fixture-v1")),
            valid(IntegrationFamilyId::new, "fixture-direct-provider"),
            valid(TransportFamilyId::new, "https-sse-conversation"),
        )
        .with_roles([DriverRole::InteractiveSession])
        .with_execution_layers([ExecutionLayer::DirectModelInference])
        .with_operation_shapes([OperationShape::InteractiveSession])
        .with_required_host_services(DriverRole::InteractiveSession, host_services());
        let instance = ConfiguredInstance::new(
            instance_id.clone(),
            valid(InstanceRevision::new, "fixture-revision-1"),
            adapter_id,
            host_id.clone(),
            valid(InstanceTargetRef::new, "fixture-conversation-endpoint"),
            InstanceOwnership::ExternalAttached,
            profile_id.clone(),
            SupportAuthority::ProviderSupported,
            valid(ProtocolFacadeId::new, "fixture-responses-conversations"),
            valid(InstancePolicyId::new, "fixture-conversation-policy"),
            CapabilityProfile::new(advertised),
        );
        let route = ModelRoute::new(
            valid(ModelRouteId::new, "fixture-conversation-route"),
            valid(ModelRouteRevision::new, "fixture-route-revision-1"),
            instance_id,
            valid(ModelId::new, "fixture-conversation-model"),
            CapabilityProfile::new(capabilities(case, true)),
        );
        let access_profile = AccessProfile::new(
            profile_id.clone(),
            CredentialMechanism::ApiKey,
            EntitlementMetering::PayAsYouGo,
            valid(EndpointAudience::new, "fixture.workspace.region"),
            SupportAuthority::ProviderSupported,
        )
        .with_credential_reference(valid(CredentialRef::new, "fixture-conversation-key"));
        let access_status = AccessStatus::new(
            profile_id.clone(),
            CredentialState::Ready,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::ProviderSupported,
        );
        let access = AccessRequirement::new(profile_id)
            .with_credential_states([CredentialState::Ready])
            .with_entitlement_states([EntitlementState::Available])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([SupportAuthority::ProviderSupported]);
        let policy = if case == ProviderConversationPreflightCase::PolicyProhibited {
            SessionProviderStatePolicy::Prohibited
        } else {
            SessionProviderStatePolicy::DurableConversationDeleteOnClose
        };
        let requirements = OperationRequirements::new(
            ExecutionLayer::DirectModelInference,
            OperationShape::InteractiveSession,
            DriverRole::InteractiveSession,
            host_id,
            access,
        )
        .with_ownership_modes([InstanceOwnership::ExternalAttached])
        .with_host_services(host_services())
        .with_capabilities(required)
        .with_session_access_policy(SessionAccessPolicy::resource_free())
        .with_session_provider_state_policy(policy)
        .require_model_route();
        Self {
            driver,
            instance,
            route,
            access_profile,
            access_status,
            requirements,
            provider_side_effects: Cell::new(0),
        }
    }

    pub fn preflight(&self) -> Result<PreflightPlan, PreflightFailure> {
        preflight(&self.context(), &self.requirements)
    }

    #[must_use]
    pub fn open_request(&self) -> OpenSessionRequest {
        OpenSessionRequest::resource_free(
            RequestId::new("fixture-conversation-open").expect("static request id is valid"),
            None,
        )
        .with_provider_state_policy(SessionProviderStatePolicy::DurableConversationDeleteOnClose)
    }

    pub fn record_provider_side_effect(&self) {
        self.provider_side_effects
            .set(self.provider_side_effects.get() + 1);
    }

    #[must_use]
    pub fn provider_side_effect_count(&self) -> usize {
        self.provider_side_effects.get()
    }

    fn context(&self) -> PreflightContext<'_> {
        PreflightContext::new(
            &self.driver,
            &self.instance,
            &self.access_profile,
            &self.access_status,
            host_services(),
        )
        .with_model_route(&self.route)
    }
}

fn capabilities(
    case: ProviderConversationPreflightCase,
    advertised: bool,
) -> Vec<CapabilityRequirement> {
    let missing_durable = matches!(
        (case, advertised),
        (
            ProviderConversationPreflightCase::MissingDurableRequirement,
            false
        ) | (
            ProviderConversationPreflightCase::AdvertisedMissingDurable,
            true
        )
    );
    let missing_conversation = matches!(
        (case, advertised),
        (
            ProviderConversationPreflightCase::MissingConversationDeletionRequirement,
            false
        ) | (
            ProviderConversationPreflightCase::AdvertisedMissingConversationDeletion,
            true
        )
    );
    let missing_items = matches!(
        (case, advertised),
        (
            ProviderConversationPreflightCase::MissingItemDeletionRequirement,
            false
        ) | (
            ProviderConversationPreflightCase::AdvertisedMissingItemDeletion,
            true
        )
    );
    let mut values = vec![
        CapabilityRequirement::new(
            Capability::InteractiveSession,
            [
                CapabilityConstraint::MaximumConcurrency(1),
                CapabilityConstraint::MaximumTurns(2),
            ],
        ),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(Capability::UsageReporting, []),
    ];
    if !missing_durable {
        values.push(CapabilityRequirement::new(
            Capability::ProviderDurableRetention,
            [],
        ));
    }
    let mut deletion = Vec::new();
    if !missing_conversation {
        deletion.push(CapabilityConstraint::OwnedRemoteResource(
            OwnedRemoteResourceKind::Conversation,
        ));
    }
    if !missing_items {
        deletion.push(CapabilityConstraint::OwnedRemoteResource(
            OwnedRemoteResourceKind::ConversationItems,
        ));
    }
    values.push(CapabilityRequirement::new(
        Capability::OwnedRemoteResourceDeletion,
        deletion,
    ));
    values
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

fn valid<T, E>(constructor: impl FnOnce(String) -> Result<T, E>, value: &str) -> T
where
    E: std::fmt::Debug,
{
    constructor(value.to_owned()).expect("static conversation fixture text must be valid")
}
