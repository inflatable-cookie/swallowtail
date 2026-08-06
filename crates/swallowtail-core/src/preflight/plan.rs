use super::{
    PlanBinding, PreflightContext, PreflightPlan, StalePreflightPlan, validation::validate,
};
use crate::{
    AccessProfileId, AdapterIdentity, AttachedModelObservation, CapabilityRequirement,
    ConfiguredInstanceId, CredentialMechanism, CredentialRef, EndpointAudience, ExecutionHostId,
    HarnessConfigurationPosture, HarnessRpcPolicy, InstanceOwnership, InstancePolicyId,
    InstanceRevision, InstanceTargetRef, InterfaceCompatibilityAssessment,
    InterfaceCompatibilityMatch, InterfaceVersionBinding, ModelId, ModelRoute, ModelRouteId,
    ModelRouteRevision, OperationRequirements, ProtocolFacadeId, ProviderAgentBinding, ProviderId,
};

impl PreflightPlan {
    #[must_use]
    /// Returns the exact driver identity validated by preflight.
    pub const fn driver_identity(&self) -> &AdapterIdentity {
        self.binding.driver.identity()
    }

    #[must_use]
    /// Returns the validated integration family.
    pub const fn integration_family(&self) -> &crate::IntegrationFamilyId {
        self.binding.driver.integration_family()
    }

    #[must_use]
    /// Returns the validated transport family.
    pub const fn transport_family(&self) -> &crate::TransportFamilyId {
        self.binding.driver.transport_family()
    }

    #[must_use]
    /// Returns the configured instance selected by preflight.
    pub const fn instance_id(&self) -> &ConfiguredInstanceId {
        self.binding.instance.id()
    }

    #[must_use]
    /// Returns the exact configured-instance revision.
    pub const fn instance_revision(&self) -> &InstanceRevision {
        self.binding.instance.revision()
    }

    #[must_use]
    /// Returns the opaque target reference bound to the instance.
    pub const fn instance_target_ref(&self) -> &InstanceTargetRef {
        self.binding.instance.target_reference()
    }

    #[must_use]
    /// Returns the protocol facade bound to the instance.
    pub const fn protocol_facade_id(&self) -> &ProtocolFacadeId {
        self.binding.instance.protocol_facade_id()
    }

    #[must_use]
    /// Returns the instance policy revision selected by the host.
    pub const fn instance_policy_id(&self) -> &InstancePolicyId {
        self.binding.instance.policy_id()
    }

    #[must_use]
    /// Returns the selected model-route identity, when one was required.
    pub fn model_route_id(&self) -> Option<&ModelRouteId> {
        self.binding.model_route.as_ref().map(ModelRoute::id)
    }

    #[must_use]
    /// Returns the selected model-route revision, when present.
    pub fn model_route_revision(&self) -> Option<&ModelRouteRevision> {
        self.binding.model_route.as_ref().map(ModelRoute::revision)
    }

    #[must_use]
    /// Returns the selected model identity, when a route was bound.
    pub fn model_id(&self) -> Option<&ModelId> {
        self.binding.model_route.as_ref().map(ModelRoute::model_id)
    }

    #[must_use]
    /// Returns the provider identity reported by the selected route.
    pub fn provider_id(&self) -> Option<&ProviderId> {
        self.binding
            .model_route
            .as_ref()
            .and_then(ModelRoute::provider_id)
    }

    #[must_use]
    /// Returns the provider-agent binding, when the instance declares one.
    pub const fn provider_agent(&self) -> Option<&ProviderAgentBinding> {
        self.binding.instance.provider_agent()
    }

    /// Iterates exact interface versions bound to the configured instance.
    pub fn interface_versions(&self) -> impl ExactSizeIterator<Item = &InterfaceVersionBinding> {
        self.binding.instance.interface_versions()
    }

    #[must_use]
    /// Returns the attached-model observation frozen by preflight.
    pub const fn attached_model_observation(&self) -> Option<&AttachedModelObservation> {
        self.binding.attached_model_observation.as_ref()
    }

    #[must_use]
    /// Returns qualified behavior evidence for one bound interface version.
    pub fn classify_interface_version(
        &self,
        binding: &InterfaceVersionBinding,
    ) -> Option<InterfaceCompatibilityMatch> {
        self.binding.driver.classify_interface_version(binding)
    }

    #[must_use]
    /// Assesses one interface version against the validated driver claim.
    pub fn assess_interface_version(
        &self,
        binding: &InterfaceVersionBinding,
    ) -> InterfaceCompatibilityAssessment {
        self.binding.driver.assess_interface_version(binding)
    }

    #[must_use]
    /// Returns the validated harness-RPC policy, when present.
    pub const fn harness_rpc_policy(&self) -> Option<&HarnessRpcPolicy> {
        self.binding.instance.harness_rpc_policy()
    }

    #[must_use]
    /// Returns the validated harness-configuration posture, when present.
    pub const fn harness_configuration_posture(&self) -> Option<HarnessConfigurationPosture> {
        self.binding.instance.harness_configuration_posture()
    }

    #[must_use]
    /// Returns the access profile selected by preflight.
    pub const fn access_profile_id(&self) -> &AccessProfileId {
        self.binding.access_profile.id()
    }

    #[must_use]
    /// Returns the complete access snapshot frozen by preflight.
    pub const fn access_status(&self) -> &crate::AccessStatus {
        &self.binding.access_status
    }

    #[must_use]
    /// Returns the selected credential mechanism.
    pub const fn credential_mechanism(&self) -> &CredentialMechanism {
        self.binding.access_profile.credential_mechanism()
    }

    #[must_use]
    /// Returns the opaque host credential reference, when configured.
    pub const fn credential_reference(&self) -> Option<&CredentialRef> {
        self.binding.access_profile.credential_reference()
    }

    #[must_use]
    /// Returns the endpoint audience validated for access.
    pub const fn endpoint_audience(&self) -> &EndpointAudience {
        self.binding.access_profile.endpoint_audience()
    }

    #[must_use]
    /// Returns the configured-instance ownership mode.
    pub const fn ownership(&self) -> InstanceOwnership {
        self.binding.instance.ownership()
    }

    #[must_use]
    /// Returns the execution host bound to preparation.
    pub const fn execution_host_id(&self) -> &ExecutionHostId {
        self.binding.instance.execution_host_id()
    }

    #[must_use]
    /// Returns the complete immutable operation requirements.
    pub const fn requirements(&self) -> &OperationRequirements {
        &self.requirements
    }

    /// Checks one exact capability claim against every bound capability scope.
    #[must_use]
    pub fn supports_capability_requirement(&self, requirement: &CapabilityRequirement) -> bool {
        profile_supports(self.binding.instance.capabilities(), requirement)
            && self
                .binding
                .model_route
                .as_ref()
                .is_none_or(|route| profile_supports(route.capabilities(), requirement))
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

fn profile_supports(
    profile: &crate::CapabilityProfile,
    requirement: &CapabilityRequirement,
) -> bool {
    profile.supports(requirement.capability())
        && requirement
            .constraints()
            .all(|constraint| profile.supports_constraint(requirement.capability(), constraint))
}
