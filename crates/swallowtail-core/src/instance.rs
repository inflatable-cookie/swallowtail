#![deny(missing_docs)]

use crate::HarnessConfigurationPosture;
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
/// One host-admitted provider or harness instance with exact route policy.
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
    harness_configuration_posture: Option<HarnessConfigurationPosture>,
}

impl ConfiguredInstance {
    /// Creates an instance without optional provider-agent or interface evidence.
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
            harness_configuration_posture: None,
        }
    }

    #[must_use]
    /// Returns the stable configured-instance identity.
    pub const fn id(&self) -> &ConfiguredInstanceId {
        &self.id
    }

    #[must_use]
    /// Returns the revision invalidated when material configuration changes.
    pub const fn revision(&self) -> &InstanceRevision {
        &self.revision
    }

    #[must_use]
    /// Returns the driver selected for this instance.
    pub const fn driver_id(&self) -> &AdapterId {
        &self.driver_id
    }

    #[must_use]
    /// Returns the host on which this instance executes.
    pub const fn execution_host_id(&self) -> &ExecutionHostId {
        &self.execution_host_id
    }

    #[must_use]
    /// Returns the opaque host-private target reference.
    pub const fn target_reference(&self) -> &InstanceTargetRef {
        &self.target_reference
    }

    #[must_use]
    /// Returns whether the instance is attached, external, or host-owned.
    pub const fn ownership(&self) -> InstanceOwnership {
        self.ownership
    }

    #[must_use]
    /// Returns the access profile bound to the instance.
    pub const fn access_profile_id(&self) -> &AccessProfileId {
        &self.access_profile_id
    }

    #[must_use]
    /// Returns the authority supporting this configured route.
    pub const fn support_authority(&self) -> SupportAuthority {
        self.support_authority
    }

    #[must_use]
    /// Returns the protocol facade exposed by the instance.
    pub const fn protocol_facade_id(&self) -> &ProtocolFacadeId {
        &self.protocol_facade_id
    }

    #[must_use]
    /// Returns the host-owned policy revision applied to the instance.
    pub const fn policy_id(&self) -> &InstancePolicyId {
        &self.policy_id
    }

    #[must_use]
    /// Returns capabilities admitted at the instance scope.
    pub const fn capabilities(&self) -> &CapabilityProfile {
        &self.capabilities
    }

    #[must_use]
    /// Binds exact provider-agent identity and version evidence.
    pub fn with_provider_agent(mut self, binding: ProviderAgentBinding) -> Self {
        self.provider_agent = Some(binding);
        self
    }

    #[must_use]
    /// Returns provider-agent evidence, when the route has one.
    pub const fn provider_agent(&self) -> Option<&ProviderAgentBinding> {
        self.provider_agent.as_ref()
    }

    #[must_use]
    /// Replaces exact interface-version observations for the instance.
    pub fn with_interface_versions(
        mut self,
        versions: impl IntoIterator<Item = InterfaceVersionBinding>,
    ) -> Self {
        self.interface_versions = versions.into_iter().collect();
        self
    }

    /// Iterates bound interface versions in stable order.
    pub fn interface_versions(&self) -> impl ExactSizeIterator<Item = &InterfaceVersionBinding> {
        self.interface_versions.iter()
    }

    #[must_use]
    /// Reports whether the exact interface binding was observed.
    pub fn has_interface_version(&self, binding: &InterfaceVersionBinding) -> bool {
        self.interface_versions.contains(binding)
    }

    #[must_use]
    /// Adds harness-RPC scheduling and message policy.
    pub fn with_harness_rpc_policy(mut self, policy: HarnessRpcPolicy) -> Self {
        self.harness_rpc_policy = Some(policy);
        self
    }

    #[must_use]
    /// Returns harness-RPC policy, when this route uses it.
    pub const fn harness_rpc_policy(&self) -> Option<&HarnessRpcPolicy> {
        self.harness_rpc_policy.as_ref()
    }

    #[must_use]
    /// Adds the observed harness-configuration posture.
    pub const fn with_harness_configuration_posture(
        mut self,
        posture: HarnessConfigurationPosture,
    ) -> Self {
        self.harness_configuration_posture = Some(posture);
        self
    }

    #[must_use]
    /// Returns harness-configuration posture, when applicable.
    pub const fn harness_configuration_posture(&self) -> Option<HarnessConfigurationPosture> {
        self.harness_configuration_posture
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// One exact model exposed through a configured provider instance.
pub struct ModelRoute {
    id: ModelRouteId,
    revision: ModelRouteRevision,
    instance_id: ConfiguredInstanceId,
    model_id: ModelId,
    provider_id: Option<ProviderId>,
    capabilities: CapabilityProfile,
}

impl ModelRoute {
    /// Creates a model route without a separate provider identity.
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
    /// Returns the stable model-route identity.
    pub const fn id(&self) -> &ModelRouteId {
        &self.id
    }

    #[must_use]
    /// Returns the exact model-route revision.
    pub const fn revision(&self) -> &ModelRouteRevision {
        &self.revision
    }

    #[must_use]
    /// Returns the configured instance that owns the route.
    pub const fn instance_id(&self) -> &ConfiguredInstanceId {
        &self.instance_id
    }

    #[must_use]
    /// Returns the adapter-owned model identity.
    pub const fn model_id(&self) -> &ModelId {
        &self.model_id
    }

    #[must_use]
    /// Adds a separate provider-reported model identity.
    pub fn with_provider_id(mut self, provider_id: ProviderId) -> Self {
        self.provider_id = Some(provider_id);
        self
    }

    #[must_use]
    /// Returns the provider-reported identity, when supplied.
    pub const fn provider_id(&self) -> Option<&ProviderId> {
        self.provider_id.as_ref()
    }

    #[must_use]
    /// Returns capabilities admitted at the model-route scope.
    pub const fn capabilities(&self) -> &CapabilityProfile {
        &self.capabilities
    }
}
