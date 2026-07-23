use super::{
    PlanBinding, PreflightContext, PreflightPlan, StalePreflightPlan, validation::validate,
};
use crate::{
    AccessProfileId, AdapterIdentity, AttachedModelObservation, ConfiguredInstanceId,
    CredentialMechanism, CredentialRef, EndpointAudience, ExecutionHostId,
    HarnessConfigurationPosture, HarnessRpcPolicy, InstanceOwnership, InstancePolicyId,
    InstanceRevision, InstanceTargetRef, InterfaceCompatibilityMatch, InterfaceVersionBinding,
    ModelId, ModelRoute, ModelRouteId, OperationRequirements, ProtocolFacadeId,
    ProviderAgentBinding, ProviderId,
};

impl PreflightPlan {
    #[must_use]
    pub const fn driver_identity(&self) -> &AdapterIdentity {
        self.binding.driver.identity()
    }

    #[must_use]
    pub const fn instance_id(&self) -> &ConfiguredInstanceId {
        self.binding.instance.id()
    }

    #[must_use]
    pub const fn instance_revision(&self) -> &InstanceRevision {
        self.binding.instance.revision()
    }

    #[must_use]
    pub const fn instance_target_ref(&self) -> &InstanceTargetRef {
        self.binding.instance.target_reference()
    }

    #[must_use]
    pub const fn protocol_facade_id(&self) -> &ProtocolFacadeId {
        self.binding.instance.protocol_facade_id()
    }

    #[must_use]
    pub const fn instance_policy_id(&self) -> &InstancePolicyId {
        self.binding.instance.policy_id()
    }

    #[must_use]
    pub fn model_route_id(&self) -> Option<&ModelRouteId> {
        self.binding.model_route.as_ref().map(ModelRoute::id)
    }

    #[must_use]
    pub fn model_id(&self) -> Option<&ModelId> {
        self.binding.model_route.as_ref().map(ModelRoute::model_id)
    }

    #[must_use]
    pub fn provider_id(&self) -> Option<&ProviderId> {
        self.binding
            .model_route
            .as_ref()
            .and_then(ModelRoute::provider_id)
    }

    #[must_use]
    pub const fn provider_agent(&self) -> Option<&ProviderAgentBinding> {
        self.binding.instance.provider_agent()
    }

    pub fn interface_versions(&self) -> impl ExactSizeIterator<Item = &InterfaceVersionBinding> {
        self.binding.instance.interface_versions()
    }

    #[must_use]
    pub const fn attached_model_observation(&self) -> Option<&AttachedModelObservation> {
        self.binding.attached_model_observation.as_ref()
    }

    #[must_use]
    pub fn classify_interface_version(
        &self,
        binding: &InterfaceVersionBinding,
    ) -> Option<InterfaceCompatibilityMatch> {
        self.binding.driver.classify_interface_version(binding)
    }

    #[must_use]
    pub const fn harness_rpc_policy(&self) -> Option<&HarnessRpcPolicy> {
        self.binding.instance.harness_rpc_policy()
    }

    #[must_use]
    pub const fn harness_configuration_posture(&self) -> Option<HarnessConfigurationPosture> {
        self.binding.instance.harness_configuration_posture()
    }

    #[must_use]
    pub const fn access_profile_id(&self) -> &AccessProfileId {
        self.binding.access_profile.id()
    }

    #[must_use]
    pub const fn credential_mechanism(&self) -> &CredentialMechanism {
        self.binding.access_profile.credential_mechanism()
    }

    #[must_use]
    pub const fn credential_reference(&self) -> Option<&CredentialRef> {
        self.binding.access_profile.credential_reference()
    }

    #[must_use]
    pub const fn endpoint_audience(&self) -> &EndpointAudience {
        self.binding.access_profile.endpoint_audience()
    }

    #[must_use]
    pub const fn ownership(&self) -> InstanceOwnership {
        self.binding.instance.ownership()
    }

    #[must_use]
    pub const fn execution_host_id(&self) -> &ExecutionHostId {
        self.binding.instance.execution_host_id()
    }

    #[must_use]
    pub const fn requirements(&self) -> &OperationRequirements {
        &self.requirements
    }

    /// Rejects execution if a material preflight binding changed.
    pub fn validate_current(
        &self,
        context: &PreflightContext<'_>,
    ) -> Result<(), StalePreflightPlan> {
        validate(context, &self.requirements).map_err(StalePreflightPlan::preflight_failed)?;
        let current = PlanBinding::from_context(context);
        if current == self.binding {
            Ok(())
        } else {
            Err(StalePreflightPlan::binding_changed())
        }
    }
}
