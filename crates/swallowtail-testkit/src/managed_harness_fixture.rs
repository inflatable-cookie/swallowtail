use std::cell::Cell;
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, AdapterId, AdapterIdentity,
    AdapterVersion, CancellationScope, Capability, CapabilityConstraint, CapabilityProfile,
    CapabilityRequirement, ConfiguredInstance, ConfiguredInstanceId, CredentialMechanism,
    CredentialRef, CredentialState, DriverDescriptor, DriverRole, EndpointAudience,
    EndpointAuthorization, EntitlementMetering, EntitlementState, ExecutionHostId, ExecutionLayer,
    HostServiceKind, InstanceOwnership, InstancePolicyId, InstanceRevision, InstanceTargetRef,
    IntegrationFamilyId, ModelId, ModelRoute, ModelRouteId, ModelRouteRevision,
    OperationRequirements, OperationShape, OwnedRemoteResourceKind, PreflightContext,
    PreflightFailure, PreflightPlan, ProtocolFacadeId, ProviderAgentBinding, ProviderAgentId,
    ProviderAgentVersion, ProviderId, RuntimeReadiness, SupportAuthority, TransportFamilyId,
    preflight,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ManagedHarnessPreflightCase {
    Canonical,
    MissingDurableRetention,
    MissingManagedRecovery,
    MissingOwnedEnvironment,
    MissingOwnedSession,
}

pub struct ManagedHarnessPreflightFixture {
    driver: DriverDescriptor,
    instance: ConfiguredInstance,
    route: ModelRoute,
    access_profile: AccessProfile,
    access_status: AccessStatus,
    requirements: OperationRequirements,
    provider_side_effects: Cell<usize>,
}

impl ManagedHarnessPreflightFixture {
    #[must_use]
    pub fn for_case(case: ManagedHarnessPreflightCase) -> Self {
        let adapter_id = valid(AdapterId::new, "fixture.managed-harness");
        let instance_id = valid(ConfiguredInstanceId::new, "fixture.managed-instance");
        let access_profile_id = valid(AccessProfileId::new, "fixture.managed-access");
        let host_id = valid(ExecutionHostId::new, "fixture.managed-host");
        let advertised = CapabilityProfile::new(advertised_capabilities(case));
        let required = required_capabilities();
        let driver = DriverDescriptor::new(
            AdapterIdentity::new(
                adapter_id.clone(),
                valid(AdapterVersion::new, "fixture-version-1"),
            ),
            valid(IntegrationFamilyId::new, "fixture-managed-provider"),
            valid(TransportFamilyId::new, "https-sse-managed-harness"),
        )
        .with_roles([DriverRole::StructuredRun])
        .with_execution_layers([ExecutionLayer::HarnessInteraction])
        .with_operation_shapes([OperationShape::StructuredRun])
        .with_required_host_services(
            DriverRole::StructuredRun,
            [
                HostServiceKind::Task,
                HostServiceKind::BlockingWork,
                HostServiceKind::Time,
                HostServiceKind::Network,
                HostServiceKind::Credential,
            ],
        );
        let instance = ConfiguredInstance::new(
            instance_id.clone(),
            valid(InstanceRevision::new, "fixture-revision-1"),
            adapter_id,
            host_id.clone(),
            valid(InstanceTargetRef::new, "fixture-managed-endpoint"),
            InstanceOwnership::ExternalAttached,
            access_profile_id.clone(),
            SupportAuthority::ProviderSupported,
            valid(ProtocolFacadeId::new, "fixture-managed-api"),
            valid(InstancePolicyId::new, "fixture-managed-policy"),
            advertised.clone(),
        )
        .with_provider_agent(ProviderAgentBinding::new(
            valid(ProviderAgentId::new, "fixture-managed-agent"),
            valid(ProviderAgentVersion::new, "7"),
        ));
        let route = ModelRoute::new(
            valid(ModelRouteId::new, "fixture-managed-route"),
            valid(ModelRouteRevision::new, "fixture-route-revision-1"),
            instance_id,
            valid(ModelId::new, "fixture-managed-model"),
            advertised,
        )
        .with_provider_id(valid(ProviderId::new, "fixture-managed-provider"));
        let access_profile = AccessProfile::new(
            access_profile_id.clone(),
            CredentialMechanism::ApiKey,
            EntitlementMetering::PayAsYouGo,
            valid(EndpointAudience::new, "fixture-managed-api"),
            SupportAuthority::ProviderSupported,
        )
        .with_credential_reference(valid(CredentialRef::new, "fixture-managed-credential"));
        let access_status = AccessStatus::new(
            access_profile_id.clone(),
            CredentialState::Ready,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::ProviderSupported,
        );
        let access = AccessRequirement::new(access_profile_id)
            .with_credential_states([CredentialState::Ready])
            .with_entitlement_states([EntitlementState::Available])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([SupportAuthority::ProviderSupported]);
        let requirements = OperationRequirements::new(
            ExecutionLayer::HarnessInteraction,
            OperationShape::StructuredRun,
            DriverRole::StructuredRun,
            host_id,
            access,
        )
        .with_ownership_modes([InstanceOwnership::ExternalAttached])
        .with_host_services([
            HostServiceKind::Task,
            HostServiceKind::BlockingWork,
            HostServiceKind::Time,
            HostServiceKind::Network,
            HostServiceKind::Credential,
        ])
        .with_capabilities(required)
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
    pub fn context(&self) -> PreflightContext<'_> {
        PreflightContext::new(
            &self.driver,
            &self.instance,
            &self.access_profile,
            &self.access_status,
            [
                HostServiceKind::Task,
                HostServiceKind::BlockingWork,
                HostServiceKind::Time,
                HostServiceKind::Network,
                HostServiceKind::Credential,
            ],
        )
        .with_model_route(&self.route)
    }

    pub fn record_provider_side_effect(&self) {
        self.provider_side_effects
            .set(self.provider_side_effects.get() + 1);
    }

    #[must_use]
    pub fn provider_side_effect_count(&self) -> usize {
        self.provider_side_effects.get()
    }
}

fn advertised_capabilities(case: ManagedHarnessPreflightCase) -> Vec<CapabilityRequirement> {
    required_capabilities()
        .into_iter()
        .filter_map(|requirement| match (case, requirement.capability()) {
            (
                ManagedHarnessPreflightCase::MissingDurableRetention,
                Capability::ProviderDurableRetention,
            )
            | (
                ManagedHarnessPreflightCase::MissingManagedRecovery,
                Capability::ProviderManagedRecovery,
            ) => None,
            (_, Capability::OwnedRemoteResourceDeletion) => {
                let constraints = requirement.constraints().filter(|constraint| {
                    !matches!(
                        (case, *constraint),
                        (
                            ManagedHarnessPreflightCase::MissingOwnedEnvironment,
                            CapabilityConstraint::OwnedRemoteResource(
                                OwnedRemoteResourceKind::Environment
                            )
                        ) | (
                            ManagedHarnessPreflightCase::MissingOwnedSession,
                            CapabilityConstraint::OwnedRemoteResource(
                                OwnedRemoteResourceKind::Session
                            )
                        )
                    )
                });
                Some(CapabilityRequirement::new(
                    Capability::OwnedRemoteResourceDeletion,
                    constraints.cloned(),
                ))
            }
            _ => Some(requirement),
        })
        .collect()
}

fn required_capabilities() -> Vec<CapabilityRequirement> {
    vec![
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
    ]
}

fn valid<T, E>(constructor: impl FnOnce(String) -> Result<T, E>, value: &str) -> T
where
    E: std::fmt::Debug,
{
    constructor(value.to_owned()).expect("static managed-harness fixture text must be valid")
}
