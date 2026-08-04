use super::super::{ConfiguredProviderInstanceAdmission, ConfiguredProviderModelCatalogueInput};
use crate::{PreparedAccessEvidence, PreparedOperationEvidence};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, AdapterId, AdapterIdentity,
    AdapterVersion, Capability, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    ConfiguredInstanceId, CredentialMechanism, CredentialRef, CredentialState, DriverDescriptor,
    DriverRole, EndpointAudience, EndpointAuthorization, EntitlementMetering, EntitlementState,
    ExecutionHostId, ExecutionLayer, InstanceOwnership, InstancePolicyId, InstanceRevision,
    InstanceTargetRef, IntegrationFamilyId, ModelCatalogEntry, ModelId, ModelMetadata,
    OperationRequirements, OperationShape, PreflightContext, ProtocolFacadeId, ProviderId,
    RuntimeReadiness, SupportAuthority, TransportFamilyId, preflight,
};

pub(super) struct Fixture {
    pub(super) driver: DriverDescriptor,
    pub(super) instance: ConfiguredInstance,
    pub(super) access_profile: AccessProfile,
    pub(super) access_evidence: PreparedAccessEvidence,
}

impl Fixture {
    pub(super) fn ready(instance_id: &str) -> Self {
        Self::with_status(
            instance_id,
            CredentialState::Ready,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
        )
    }

    pub(super) fn with_status(
        instance_id: &str,
        credential: CredentialState,
        entitlement: EntitlementState,
        endpoint: EndpointAuthorization,
        runtime: RuntimeReadiness,
    ) -> Self {
        let adapter_id = AdapterId::new("fixture.provider").expect("adapter id");
        let host_id = ExecutionHostId::new("fixture.host").expect("host id");
        let access_id = AccessProfileId::new("fixture.access").expect("access id");
        let driver = DriverDescriptor::new(
            AdapterIdentity::new(
                adapter_id.clone(),
                AdapterVersion::new("1").expect("adapter version"),
            ),
            IntegrationFamilyId::new("fixture-family").expect("family id"),
            TransportFamilyId::new("fixture-transport").expect("transport id"),
        )
        .with_roles([DriverRole::ModelCatalog, DriverRole::StructuredRun])
        .with_execution_layers([ExecutionLayer::HarnessInteraction])
        .with_operation_shapes([
            OperationShape::InteractiveSession,
            OperationShape::StructuredRun,
        ]);
        let capabilities = CapabilityProfile::new([
            CapabilityRequirement::new(Capability::ModelCatalog, []),
            CapabilityRequirement::new(Capability::StructuredRun, []),
        ]);
        let instance = ConfiguredInstance::new(
            ConfiguredInstanceId::new(instance_id).expect("instance id"),
            InstanceRevision::new("revision-1").expect("instance revision"),
            adapter_id,
            host_id,
            InstanceTargetRef::new("private-target").expect("target"),
            InstanceOwnership::HostOwnedPersistent,
            access_id.clone(),
            SupportAuthority::ProviderSupported,
            ProtocolFacadeId::new("fixture-facade").expect("facade"),
            InstancePolicyId::new("fixture-policy").expect("policy"),
            capabilities,
        );
        let access_profile = AccessProfile::new(
            access_id.clone(),
            CredentialMechanism::ApiKey,
            EntitlementMetering::PayAsYouGo,
            EndpointAudience::new("fixture-audience").expect("audience"),
            SupportAuthority::ProviderSupported,
        )
        .with_credential_reference(
            CredentialRef::new("private-credential").expect("credential reference"),
        );
        let access_evidence = PreparedAccessEvidence::caller_asserted(AccessStatus::new(
            access_id,
            credential,
            entitlement,
            endpoint,
            runtime,
            SupportAuthority::ProviderSupported,
        ));
        Self {
            driver,
            instance,
            access_profile,
            access_evidence,
        }
    }

    pub(super) fn admission(&self) -> ConfiguredProviderInstanceAdmission {
        ConfiguredProviderInstanceAdmission::new(
            self.driver.clone(),
            self.instance.clone(),
            self.access_profile.clone(),
            self.access_evidence.clone(),
        )
    }

    pub(super) fn prepared(&self, role: DriverRole) -> PreparedOperationEvidence {
        let (capability, operation_shape) = match role {
            DriverRole::ModelCatalog => {
                (Capability::ModelCatalog, OperationShape::InteractiveSession)
            }
            DriverRole::StructuredRun => (Capability::StructuredRun, OperationShape::StructuredRun),
            _ => panic!("fixture role is unsupported"),
        };
        let status = self.access_evidence.status();
        let requirements = OperationRequirements::new(
            ExecutionLayer::HarnessInteraction,
            operation_shape,
            role,
            self.instance.execution_host_id().clone(),
            AccessRequirement::new(self.access_profile.id().clone())
                .with_credential_states([status.credential()])
                .with_entitlement_states([status.entitlement()])
                .with_endpoint_authorizations([status.endpoint_authorization()])
                .with_runtime_readiness([status.runtime_readiness()])
                .with_support_authorities([status.support_authority()]),
        )
        .with_ownership_modes([self.instance.ownership()])
        .with_capabilities([CapabilityRequirement::new(capability, [])]);
        let plan = preflight(
            &PreflightContext::new(
                &self.driver,
                &self.instance,
                &self.access_profile,
                status,
                [],
            ),
            &requirements,
        )
        .expect("fixture preflight succeeds");
        PreparedOperationEvidence::from_plan(plan, self.access_evidence.clone())
            .expect("fixture evidence prepares")
    }

    pub(super) fn model(&self, model_id: &str, provider_id: Option<&str>) -> ModelCatalogEntry {
        let entry = ModelCatalogEntry::new(
            ModelId::new(model_id).expect("model id"),
            ModelMetadata::default(),
        );
        match provider_id {
            Some(provider_id) => {
                entry.with_provider_id(ProviderId::new(provider_id).expect("provider id"))
            }
            None => entry,
        }
    }

    pub(super) fn available_model_catalogue(
        &self,
        source: PreparedOperationEvidence,
    ) -> ConfiguredProviderModelCatalogueInput {
        ConfiguredProviderModelCatalogueInput::available(source, [self.model("model-a", None)])
    }
}
