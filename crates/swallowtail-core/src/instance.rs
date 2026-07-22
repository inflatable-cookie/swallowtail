use crate::HarnessRpcPolicy;
use crate::InterfaceVersionBinding;
use crate::access::SupportAuthority;
use crate::identity::AdapterId;
use crate::model::{ModelId, ProviderId};
use crate::provider_agent::ProviderAgentBinding;
use crate::requirement::CapabilityProfile;
use crate::runtime_identity::{
    AccessProfileId, ConfiguredInstanceId, ExecutionHostId, InstanceOwnership, InstancePolicyId,
    InstanceRevision, InstanceTargetRef, ModelRouteId, ModelRouteRevision, ProtocolFacadeId,
};
use std::collections::BTreeSet;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfiguredInstance {
    id: ConfiguredInstanceId,
    revision: InstanceRevision,
    driver_id: AdapterId,
    execution_host_id: ExecutionHostId,
    target_reference: InstanceTargetRef,
    ownership: InstanceOwnership,
    access_profile_id: AccessProfileId,
    support_authority: SupportAuthority,
    protocol_facade_id: ProtocolFacadeId,
    policy_id: InstancePolicyId,
    capabilities: CapabilityProfile,
    provider_agent: Option<ProviderAgentBinding>,
    interface_versions: BTreeSet<InterfaceVersionBinding>,
    harness_rpc_policy: Option<HarnessRpcPolicy>,
}

impl ConfiguredInstance {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        id: ConfiguredInstanceId,
        revision: InstanceRevision,
        driver_id: AdapterId,
        execution_host_id: ExecutionHostId,
        target_reference: InstanceTargetRef,
        ownership: InstanceOwnership,
        access_profile_id: AccessProfileId,
        support_authority: SupportAuthority,
        protocol_facade_id: ProtocolFacadeId,
        policy_id: InstancePolicyId,
        capabilities: CapabilityProfile,
    ) -> Self {
        Self {
            id,
            revision,
            driver_id,
            execution_host_id,
            target_reference,
            ownership,
            access_profile_id,
            support_authority,
            protocol_facade_id,
            policy_id,
            capabilities,
            provider_agent: None,
            interface_versions: BTreeSet::new(),
            harness_rpc_policy: None,
        }
    }

    #[must_use]
    pub const fn id(&self) -> &ConfiguredInstanceId {
        &self.id
    }

    #[must_use]
    pub const fn revision(&self) -> &InstanceRevision {
        &self.revision
    }

    #[must_use]
    pub const fn driver_id(&self) -> &AdapterId {
        &self.driver_id
    }

    #[must_use]
    pub const fn execution_host_id(&self) -> &ExecutionHostId {
        &self.execution_host_id
    }

    #[must_use]
    pub const fn target_reference(&self) -> &InstanceTargetRef {
        &self.target_reference
    }

    #[must_use]
    pub const fn ownership(&self) -> InstanceOwnership {
        self.ownership
    }

    #[must_use]
    pub const fn access_profile_id(&self) -> &AccessProfileId {
        &self.access_profile_id
    }

    #[must_use]
    pub const fn support_authority(&self) -> SupportAuthority {
        self.support_authority
    }

    #[must_use]
    pub const fn protocol_facade_id(&self) -> &ProtocolFacadeId {
        &self.protocol_facade_id
    }

    #[must_use]
    pub const fn policy_id(&self) -> &InstancePolicyId {
        &self.policy_id
    }

    #[must_use]
    pub const fn capabilities(&self) -> &CapabilityProfile {
        &self.capabilities
    }

    #[must_use]
    pub fn with_provider_agent(mut self, binding: ProviderAgentBinding) -> Self {
        self.provider_agent = Some(binding);
        self
    }

    #[must_use]
    pub const fn provider_agent(&self) -> Option<&ProviderAgentBinding> {
        self.provider_agent.as_ref()
    }

    #[must_use]
    pub fn with_interface_versions(
        mut self,
        versions: impl IntoIterator<Item = InterfaceVersionBinding>,
    ) -> Self {
        self.interface_versions = versions.into_iter().collect();
        self
    }

    pub fn interface_versions(&self) -> impl ExactSizeIterator<Item = &InterfaceVersionBinding> {
        self.interface_versions.iter()
    }

    #[must_use]
    pub fn has_interface_version(&self, binding: &InterfaceVersionBinding) -> bool {
        self.interface_versions.contains(binding)
    }

    #[must_use]
    pub fn with_harness_rpc_policy(mut self, policy: HarnessRpcPolicy) -> Self {
        self.harness_rpc_policy = Some(policy);
        self
    }

    #[must_use]
    pub const fn harness_rpc_policy(&self) -> Option<&HarnessRpcPolicy> {
        self.harness_rpc_policy.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelRoute {
    id: ModelRouteId,
    revision: ModelRouteRevision,
    instance_id: ConfiguredInstanceId,
    model_id: ModelId,
    provider_id: Option<ProviderId>,
    capabilities: CapabilityProfile,
}

impl ModelRoute {
    #[must_use]
    pub const fn new(
        id: ModelRouteId,
        revision: ModelRouteRevision,
        instance_id: ConfiguredInstanceId,
        model_id: ModelId,
        capabilities: CapabilityProfile,
    ) -> Self {
        Self {
            id,
            revision,
            instance_id,
            model_id,
            provider_id: None,
            capabilities,
        }
    }

    #[must_use]
    pub const fn id(&self) -> &ModelRouteId {
        &self.id
    }

    #[must_use]
    pub const fn revision(&self) -> &ModelRouteRevision {
        &self.revision
    }

    #[must_use]
    pub const fn instance_id(&self) -> &ConfiguredInstanceId {
        &self.instance_id
    }

    #[must_use]
    pub const fn model_id(&self) -> &ModelId {
        &self.model_id
    }

    #[must_use]
    pub fn with_provider_id(mut self, provider_id: ProviderId) -> Self {
        self.provider_id = Some(provider_id);
        self
    }

    #[must_use]
    pub const fn provider_id(&self) -> Option<&ProviderId> {
        self.provider_id.as_ref()
    }

    #[must_use]
    pub const fn capabilities(&self) -> &CapabilityProfile {
        &self.capabilities
    }
}
