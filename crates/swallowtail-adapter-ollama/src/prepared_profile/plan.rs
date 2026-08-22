use crate::OllamaPreparedIntegration;
use swallowtail_core::{
    AccessRequirement, AttachedRuntimeRequirements, AttachedRuntimeResidency, CapabilityProfile,
    CapabilityRequirement, ConfiguredInstance, CredentialState, Diagnostic, DriverRole,
    EndpointAuthorization, EntitlementState, ExecutionLayer, ModelRoute, OperationRequirements,
    OperationShape, PreflightContext, PreflightPlan, RuntimeReadiness, preflight,
};
use swallowtail_runtime::{PreparationFailure, PreparationStage, PreparedOperationEvidence};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Immutable runtime, access, activity, and preflight evidence for an operation.
pub struct OllamaPreparedEvidence {
    runtime: crate::OllamaPreparedRuntimeObservation,
    operation: PreparedOperationEvidence,
    context_window: Option<crate::OllamaContextWindow>,
}

impl OllamaPreparedEvidence {
    pub(super) fn from_prepared(
        prepared: &OllamaPreparedIntegration,
        plan: PreflightPlan,
        context_window: Option<crate::OllamaContextWindow>,
    ) -> Result<Self, PreparationFailure> {
        Ok(Self {
            runtime: prepared.runtime().clone(),
            operation: PreparedOperationEvidence::from_plan(
                plan,
                prepared.access_evidence().clone(),
            )?,
            context_window,
        })
    }

    pub(super) fn from_prepared_with_activity(
        prepared: &OllamaPreparedIntegration,
        plan: PreflightPlan,
        activity_profile: swallowtail_core::ObservableActivityProfile,
        context_window: Option<crate::OllamaContextWindow>,
    ) -> Result<Self, PreparationFailure> {
        Ok(Self {
            runtime: prepared.runtime().clone(),
            operation: PreparedOperationEvidence::from_plan_with_activity_profile(
                plan,
                prepared.access_evidence().clone(),
                activity_profile,
            )?,
            context_window,
        })
    }

    /// Returns the runtime and selected-model observations used at preflight.
    #[must_use]
    pub const fn runtime(&self) -> &crate::OllamaPreparedRuntimeObservation {
        &self.runtime
    }

    /// Returns the admitted access evidence.
    #[must_use]
    pub const fn access(&self) -> &swallowtail_runtime::PreparedAccessEvidence {
        self.operation.access()
    }

    /// Returns the complete prepared operation evidence.
    #[must_use]
    pub const fn operation(&self) -> &PreparedOperationEvidence {
        &self.operation
    }

    /// Returns the observable activity contract selected for the operation.
    #[must_use]
    pub const fn observable_activity(&self) -> &swallowtail_core::ObservableActivityProfile {
        self.operation.observable_activity()
    }

    /// Returns the immutable preflight plan.
    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.operation.plan()
    }

    /// Returns the selected native context window when one was prepared.
    #[must_use]
    pub const fn context_window(&self) -> Option<crate::OllamaContextWindow> {
        self.context_window
    }
}

pub(super) fn bind_low_level_driver(
    evidence: &OllamaPreparedEvidence,
) -> crate::OllamaNativeAttachedDriver {
    crate::OllamaNativeAttachedDriver::bound_to_prepared_evidence(evidence)
}

pub(super) fn instance_with_capabilities(
    prepared: &OllamaPreparedIntegration,
    capabilities: CapabilityProfile,
) -> ConfiguredInstance {
    let base = prepared.instance();
    ConfiguredInstance::new(
        base.id().clone(),
        base.revision().clone(),
        base.driver_id().clone(),
        base.execution_host_id().clone(),
        base.target_reference().clone(),
        base.ownership(),
        base.access_profile_id().clone(),
        base.support_authority(),
        base.protocol_facade_id().clone(),
        base.policy_id().clone(),
        capabilities,
    )
    .with_interface_versions(base.interface_versions().cloned())
}

pub(super) fn model_route(
    prepared: &OllamaPreparedIntegration,
    model: super::OllamaModelSelection,
    capabilities: CapabilityProfile,
) -> ModelRoute {
    let (route_id, route_revision, model_id) = model.into_parts();
    ModelRoute::new(
        route_id,
        route_revision,
        prepared.instance().id().clone(),
        model_id,
        capabilities,
    )
}

pub(super) fn requirements(
    prepared: &OllamaPreparedIntegration,
    route: &ModelRoute,
    operation_shape: OperationShape,
    role: DriverRole,
    capabilities: impl IntoIterator<Item = CapabilityRequirement>,
) -> OperationRequirements {
    let descriptor = crate::ollama_native_descriptor();
    let detail = prepared.runtime().selected_detail();
    OperationRequirements::new(
        ExecutionLayer::DirectModelInference,
        operation_shape,
        role,
        prepared.instance().execution_host_id().clone(),
        AccessRequirement::new(prepared.access_profile().id().clone())
            .with_credential_states([CredentialState::NotRequired])
            .with_entitlement_states([EntitlementState::Available])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([prepared.access_profile().support_authority()]),
    )
    .with_ownership_modes([prepared.instance().ownership()])
    .with_host_services(descriptor.required_host_services(role))
    .with_capabilities(capabilities)
    .with_interface_versions([prepared.runtime().runtime_version().clone()])
    .with_attached_runtime(AttachedRuntimeRequirements::new(
        prepared.runtime().runtime_version().clone(),
        route.model_id().clone(),
        detail.model_tag().clone(),
        detail
            .manifest_digest()
            .expect("prepared Ollama detail binds a manifest digest")
            .clone(),
        AttachedRuntimeResidency::RuntimeManaged,
    ))
    .require_model_route()
}

pub(super) fn build_plan(
    prepared: &OllamaPreparedIntegration,
    instance: &ConfiguredInstance,
    route: &ModelRoute,
    requirements: &OperationRequirements,
) -> Result<PreflightPlan, PreparationFailure> {
    let descriptor = crate::ollama_native_descriptor();
    let context = PreflightContext::new(
        &descriptor,
        instance,
        prepared.access_profile(),
        prepared.access_evidence().status(),
        prepared.available_host_services(),
    )
    .with_model_route(route)
    .with_attached_model_observation(prepared.runtime().selected_detail());
    preflight(&context, requirements).map_err(|error| {
        PreparationFailure::new(
            PreparationStage::Preflight,
            Diagnostic::new(error.diagnostic().clone()),
        )
    })
}
