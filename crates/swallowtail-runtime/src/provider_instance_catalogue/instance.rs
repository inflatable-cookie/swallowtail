use swallowtail_core::{
    AdapterIdentity, CapabilityProfile, ConfiguredInstanceId, DriverDescriptor, ExecutionHostId,
    HarnessConfigurationPosture, HarnessRpcPolicy, InstanceOwnership, InstancePolicyId,
    InstanceRevision, IntegrationFamilyId, InterfaceVersionBinding, ProtocolFacadeId,
    ProviderAgentBinding, TransportFamilyId,
};

use super::failure::failure;
use super::validation::{project_model_catalogue, project_routes, validate_base, validate_route};
use super::{
    ConfiguredProviderCredentialPosture, ConfiguredProviderInstanceAdmission,
    ConfiguredProviderInstanceCatalogueFailure, ConfiguredProviderInstanceCatalogueFailureKind,
    ConfiguredProviderInstanceRoute, ConfiguredProviderModelCatalogue,
    MAX_CONFIGURED_PROVIDER_ROUTES_PER_INSTANCE,
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ConfiguredProviderInstanceSelectionReadiness {
    Ready,
    NotReady,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfiguredProviderInstanceRecord {
    instance_id: ConfiguredInstanceId,
    instance_revision: InstanceRevision,
    driver: DriverDescriptor,
    protocol_facade_id: ProtocolFacadeId,
    instance_policy_id: InstancePolicyId,
    execution_host_id: ExecutionHostId,
    ownership: InstanceOwnership,
    provider_agent: Option<ProviderAgentBinding>,
    interface_versions: Vec<InterfaceVersionBinding>,
    harness_rpc_policy: Option<HarnessRpcPolicy>,
    harness_configuration_posture: Option<HarnessConfigurationPosture>,
    capabilities: CapabilityProfile,
    credential_posture: ConfiguredProviderCredentialPosture,
    selection_readiness: ConfiguredProviderInstanceSelectionReadiness,
    routes: Vec<ConfiguredProviderInstanceRoute>,
    model_catalogue: Option<ConfiguredProviderModelCatalogue>,
}

impl ConfiguredProviderInstanceRecord {
    pub fn admit(
        admission: ConfiguredProviderInstanceAdmission,
    ) -> Result<Self, ConfiguredProviderInstanceCatalogueFailure> {
        validate_base(&admission)?;
        if admission.prepared_routes.len() > MAX_CONFIGURED_PROVIDER_ROUTES_PER_INSTANCE {
            return Err(failure(
                ConfiguredProviderInstanceCatalogueFailureKind::LimitExceeded,
                "swallowtail.provider_instance_catalogue.route_limit_exceeded",
                "Configured provider instance exceeds the portable route limit",
            ));
        }
        for evidence in &admission.prepared_routes {
            validate_route(&admission, evidence)?;
        }
        let routes = project_routes(&admission.prepared_routes)?;
        let model_catalogue = admission
            .model_catalogue
            .as_ref()
            .map(|catalogue| project_model_catalogue(&admission, &routes, catalogue))
            .transpose()?;
        let credential_posture = ConfiguredProviderCredentialPosture::from_evidence(
            &admission.access_profile,
            &admission.access_evidence,
        );
        let selection_readiness = if credential_posture.permits_selection()
            && model_catalogue
                .as_ref()
                .is_some_and(ConfiguredProviderModelCatalogue::permits_selection)
        {
            ConfiguredProviderInstanceSelectionReadiness::Ready
        } else {
            ConfiguredProviderInstanceSelectionReadiness::NotReady
        };
        Ok(Self {
            instance_id: admission.instance.id().clone(),
            instance_revision: admission.instance.revision().clone(),
            driver: admission.driver,
            protocol_facade_id: admission.instance.protocol_facade_id().clone(),
            instance_policy_id: admission.instance.policy_id().clone(),
            execution_host_id: admission.instance.execution_host_id().clone(),
            ownership: admission.instance.ownership(),
            provider_agent: admission.instance.provider_agent().cloned(),
            interface_versions: admission.instance.interface_versions().cloned().collect(),
            harness_rpc_policy: admission.instance.harness_rpc_policy().cloned(),
            harness_configuration_posture: admission.instance.harness_configuration_posture(),
            capabilities: admission.instance.capabilities().clone(),
            credential_posture,
            selection_readiness,
            routes,
            model_catalogue,
        })
    }

    #[must_use]
    pub const fn instance_id(&self) -> &ConfiguredInstanceId {
        &self.instance_id
    }

    #[must_use]
    pub const fn instance_revision(&self) -> &InstanceRevision {
        &self.instance_revision
    }

    #[must_use]
    pub const fn driver_identity(&self) -> &AdapterIdentity {
        self.driver.identity()
    }

    #[must_use]
    pub const fn driver_descriptor(&self) -> &DriverDescriptor {
        &self.driver
    }

    #[must_use]
    pub const fn integration_family(&self) -> &IntegrationFamilyId {
        self.driver.integration_family()
    }

    #[must_use]
    pub const fn transport_family(&self) -> &TransportFamilyId {
        self.driver.transport_family()
    }

    #[must_use]
    pub const fn protocol_facade_id(&self) -> &ProtocolFacadeId {
        &self.protocol_facade_id
    }

    #[must_use]
    pub const fn instance_policy_id(&self) -> &InstancePolicyId {
        &self.instance_policy_id
    }

    #[must_use]
    pub const fn execution_host_id(&self) -> &ExecutionHostId {
        &self.execution_host_id
    }

    #[must_use]
    pub const fn ownership(&self) -> InstanceOwnership {
        self.ownership
    }

    #[must_use]
    pub const fn provider_agent(&self) -> Option<&ProviderAgentBinding> {
        self.provider_agent.as_ref()
    }

    pub fn interface_versions(&self) -> impl ExactSizeIterator<Item = &InterfaceVersionBinding> {
        self.interface_versions.iter()
    }

    #[must_use]
    pub const fn harness_rpc_policy(&self) -> Option<&HarnessRpcPolicy> {
        self.harness_rpc_policy.as_ref()
    }

    #[must_use]
    pub const fn harness_configuration_posture(&self) -> Option<HarnessConfigurationPosture> {
        self.harness_configuration_posture
    }

    #[must_use]
    pub const fn capabilities(&self) -> &CapabilityProfile {
        &self.capabilities
    }

    #[must_use]
    pub const fn credential_posture(&self) -> &ConfiguredProviderCredentialPosture {
        &self.credential_posture
    }

    #[must_use]
    pub const fn selection_readiness(&self) -> ConfiguredProviderInstanceSelectionReadiness {
        self.selection_readiness
    }

    pub fn routes(&self) -> impl ExactSizeIterator<Item = &ConfiguredProviderInstanceRoute> {
        self.routes.iter()
    }

    #[must_use]
    pub const fn model_catalogue(&self) -> Option<&ConfiguredProviderModelCatalogue> {
        self.model_catalogue.as_ref()
    }
}
