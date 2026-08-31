use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, AdapterId, AdapterIdentity,
    AdapterVersion, Capability, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    ConfiguredInstanceId, CredentialMechanism, CredentialRef, CredentialState, DriverDescriptor,
    DriverRole, EndpointAudience, EndpointAuthorization, EntitlementMetering, EntitlementState,
    ExecutionHostId, ExecutionLayer, InstanceOwnership, InstancePolicyId, InstanceRevision,
    InstanceTargetRef, IntegrationFamilyId, ModelId, ModelRoute, ModelRouteId, ModelRouteRevision,
    OperationRequirements, OperationShape, PreflightContext, PreflightPlan, ProtocolFacadeId,
    RuntimeReadiness, SessionAccessPolicy, SessionProviderStatePolicy, SupportAuthority,
    TransportFamilyId, preflight,
};
use swallowtail_runtime::{
    ConfiguredProviderInstanceAdmission, ConfiguredProviderInstanceRecord,
    ConsumerRouteApplicability, ConsumerRouteProjectionSourceId,
    ConsumerRouteProjectionSourceIdentity, ConsumerRouteProjectionSourceKind,
    PreparedAccessEvidence, PreparedOperationEvidence,
};

/// Private fixture text that must never reach a projected row.
pub const CONSUMER_ROUTE_PRIVATE_TARGET: &str = "private-instance-target";
/// Private fixture credential that must never reach a projected row.
pub const CONSUMER_ROUTE_PRIVATE_CREDENTIAL: &str = "private-instance-credential";

/// Portable Contract 061 projection fixture with no adapter dependency.
///
/// The fixture builds one exact configured record and prepared-operation
/// record that agree, so conformance can prove admission, composition, and
/// replacement without provider contact.
#[derive(Clone, Debug)]
pub struct ConsumerRouteProjectionFixture {
    driver: DriverDescriptor,
    instance: ConfiguredInstance,
    model_route: ModelRoute,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
}

impl ConsumerRouteProjectionFixture {
    #[must_use]
    /// Builds the canonical ready interactive-session projection fixture.
    pub fn canonical() -> Self {
        Self::with_revision("revision-1")
    }

    #[must_use]
    /// Builds the same fixture under a different configured-instance revision.
    pub fn superseded() -> Self {
        Self::with_revision("revision-2")
    }

    fn with_revision(revision: &str) -> Self {
        let adapter_id = AdapterId::new("fixture.consumer-route").expect("adapter id is valid");
        let host_id = ExecutionHostId::new("fixture.host.local").expect("host id is valid");
        let access_id =
            AccessProfileId::new("fixture.access.consumer-route").expect("access id is valid");
        let driver = DriverDescriptor::new(
            AdapterIdentity::new(
                adapter_id.clone(),
                AdapterVersion::new("1").expect("adapter version is valid"),
            ),
            IntegrationFamilyId::new("fixture-family").expect("family id is valid"),
            TransportFamilyId::new("fixture-transport").expect("transport id is valid"),
        )
        .with_roles([DriverRole::InteractiveSession])
        .with_execution_layers([ExecutionLayer::HarnessInteraction])
        .with_operation_shapes([OperationShape::InteractiveSession]);
        let capabilities = CapabilityProfile::new([
            CapabilityRequirement::new(Capability::InteractiveSession, []),
            CapabilityRequirement::new(Capability::StreamingEvents, []),
        ]);
        let instance = ConfiguredInstance::new(
            ConfiguredInstanceId::new("fixture.instance.consumer-route")
                .expect("instance id is valid"),
            InstanceRevision::new(revision).expect("instance revision is valid"),
            adapter_id,
            host_id,
            InstanceTargetRef::new(CONSUMER_ROUTE_PRIVATE_TARGET).expect("target is valid"),
            InstanceOwnership::HostOwnedPersistent,
            access_id.clone(),
            SupportAuthority::ProviderSupported,
            ProtocolFacadeId::new("fixture-facade").expect("facade id is valid"),
            InstancePolicyId::new("fixture-policy").expect("policy id is valid"),
            capabilities.clone(),
        );
        let model_route = ModelRoute::new(
            ModelRouteId::new("fixture.route.model").expect("route id is valid"),
            ModelRouteRevision::new("route-revision-1").expect("route revision is valid"),
            instance.id().clone(),
            ModelId::new("fixture-model").expect("model id is valid"),
            capabilities,
        );
        let access_profile = AccessProfile::new(
            access_id.clone(),
            CredentialMechanism::ApiKey,
            EntitlementMetering::PayAsYouGo,
            EndpointAudience::new("fixture-audience").expect("audience is valid"),
            SupportAuthority::ProviderSupported,
        )
        .with_credential_reference(
            CredentialRef::new(CONSUMER_ROUTE_PRIVATE_CREDENTIAL).expect("credential is valid"),
        );
        let access_evidence = PreparedAccessEvidence::caller_asserted(AccessStatus::new(
            access_id,
            CredentialState::Ready,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::ProviderSupported,
        ));
        Self {
            driver,
            instance,
            model_route,
            access_profile,
            access_evidence,
        }
    }

    #[must_use]
    /// Returns the immutable preflight plan the fixture prepares.
    pub fn plan(&self) -> PreflightPlan {
        let status = self.access_evidence.status();
        let requirements = OperationRequirements::new(
            ExecutionLayer::HarnessInteraction,
            OperationShape::InteractiveSession,
            DriverRole::InteractiveSession,
            self.instance.execution_host_id().clone(),
            AccessRequirement::new(self.access_profile.id().clone())
                .with_credential_states([status.credential()])
                .with_entitlement_states([status.entitlement()])
                .with_endpoint_authorizations([status.endpoint_authorization()])
                .with_runtime_readiness([status.runtime_readiness()])
                .with_support_authorities([status.support_authority()]),
        )
        .with_ownership_modes([self.instance.ownership()])
        .with_capabilities([
            CapabilityRequirement::new(Capability::InteractiveSession, []),
            CapabilityRequirement::new(Capability::StreamingEvents, []),
        ])
        .with_session_access_policy(SessionAccessPolicy::read_only())
        .with_session_provider_state_policy(SessionProviderStatePolicy::Prohibited)
        .require_model_route();
        preflight(
            &PreflightContext::new(
                &self.driver,
                &self.instance,
                &self.access_profile,
                status,
                [],
            )
            .with_model_route(&self.model_route),
            &requirements,
        )
        .expect("fixture preflight succeeds")
    }

    #[must_use]
    /// Returns the exact prepared-operation record.
    pub fn prepared(&self) -> PreparedOperationEvidence {
        PreparedOperationEvidence::from_plan(self.plan(), self.access_evidence.clone())
            .expect("fixture prepared evidence is admitted")
    }

    #[must_use]
    /// Returns the exact configured provider-instance record.
    pub fn record(&self) -> ConfiguredProviderInstanceRecord {
        ConfiguredProviderInstanceRecord::admit(
            ConfiguredProviderInstanceAdmission::new(
                self.driver.clone(),
                self.instance.clone(),
                self.access_profile.clone(),
                self.access_evidence.clone(),
            )
            .with_prepared_routes([self.prepared()]),
        )
        .expect("fixture configured record is admitted")
    }

    #[must_use]
    /// Returns the exact applicability every fixture row is bound to.
    pub fn applicability(&self) -> ConsumerRouteApplicability {
        ConsumerRouteApplicability::from_prepared_operation(&self.prepared())
    }
}

#[must_use]
/// Builds one bounded projection source identity for conformance fixtures.
pub fn consumer_route_projection_source(
    id: &str,
    kind: ConsumerRouteProjectionSourceKind,
) -> ConsumerRouteProjectionSourceIdentity {
    ConsumerRouteProjectionSourceIdentity::new(
        ConsumerRouteProjectionSourceId::new(id).expect("fixture source id is valid"),
        kind,
    )
}
