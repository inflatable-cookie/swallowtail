use crate::DeepSeekPreparedIntegration;
use swallowtail_core::{
    CapabilityProfile, CapabilityRequirement, ConfiguredInstance, CredentialState, DriverRole,
    ExecutionLayer, ModelRoute, OperationRequirements, OperationShape, PreflightPlan, ProviderId,
    ReasoningMode,
};
use swallowtail_runtime::{PreparationFailure, PreparedOperationEvidence};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Inspectable prepared evidence for one DeepSeek operation.
pub struct DeepSeekPreparedEvidence {
    operation: PreparedOperationEvidence,
    reasoning_mode: Option<ReasoningMode>,
    thinking_mode: Option<crate::DeepSeekThinkingMode>,
}

impl DeepSeekPreparedEvidence {
    pub(super) fn from_prepared(
        prepared: &DeepSeekPreparedIntegration,
        plan: PreflightPlan,
    ) -> Result<Self, PreparationFailure> {
        Ok(Self {
            operation: PreparedOperationEvidence::from_plan(
                plan,
                prepared.access_evidence().clone(),
            )?,
            reasoning_mode: None,
            thinking_mode: None,
        })
    }

    pub(super) fn from_prepared_with_activity(
        prepared: &DeepSeekPreparedIntegration,
        plan: PreflightPlan,
        activity_profile: swallowtail_core::ObservableActivityProfile,
        reasoning_mode: Option<ReasoningMode>,
        thinking_mode: Option<crate::DeepSeekThinkingMode>,
    ) -> Result<Self, PreparationFailure> {
        Ok(Self {
            operation: PreparedOperationEvidence::from_plan_with_activity_profile(
                plan,
                prepared.access_evidence().clone(),
                activity_profile,
            )?,
            reasoning_mode,
            thinking_mode,
        })
    }

    #[must_use]
    /// Returns the access evidence and provenance bound to the operation.
    pub const fn access(&self) -> &swallowtail_runtime::PreparedAccessEvidence {
        self.operation.access()
    }

    #[must_use]
    /// Returns the shared prepared-operation evidence.
    pub const fn operation(&self) -> &PreparedOperationEvidence {
        &self.operation
    }

    #[must_use]
    /// Returns the route's observable-activity contract.
    pub const fn observable_activity(&self) -> &swallowtail_core::ObservableActivityProfile {
        self.operation.observable_activity()
    }

    #[must_use]
    /// Returns the immutable preflight plan.
    pub const fn plan(&self) -> &PreflightPlan {
        self.operation.plan()
    }

    #[must_use]
    /// Returns the exact reasoning selection bound to this operation, if any.
    pub const fn reasoning_mode(&self) -> Option<&ReasoningMode> {
        self.reasoning_mode.as_ref()
    }

    #[must_use]
    /// Returns the exact adapter-local thinking-mode selection, if any.
    pub const fn thinking_mode(&self) -> Option<crate::DeepSeekThinkingMode> {
        self.thinking_mode
    }
}

pub(super) fn instance_with_capabilities(
    prepared: &DeepSeekPreparedIntegration,
    capabilities: CapabilityProfile,
) -> ConfiguredInstance {
    swallowtail_runtime::instance_with_capabilities(prepared.instance(), capabilities)
}

pub(super) fn model_route(
    prepared: &DeepSeekPreparedIntegration,
    model: super::DeepSeekModelSelection,
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
    .with_provider_id(ProviderId::new("deepseek").expect("static provider id is valid"))
}

pub(super) fn catalogue_requirements(
    prepared: &DeepSeekPreparedIntegration,
    capabilities: impl IntoIterator<Item = CapabilityRequirement>,
) -> OperationRequirements {
    let descriptor = crate::deepseek_direct_descriptor();
    swallowtail_runtime::base_requirements(
        ExecutionLayer::DirectModelInference,
        OperationShape::InteractiveSession,
        DriverRole::ModelCatalog,
        prepared.instance(),
        prepared.access_profile(),
        [CredentialState::Ready],
        capabilities,
    )
    .with_host_services(descriptor.required_host_services(DriverRole::ModelCatalog))
}

pub(super) fn build_plan(
    prepared: &DeepSeekPreparedIntegration,
    instance: &ConfiguredInstance,
    route: Option<&ModelRoute>,
    requirements: &OperationRequirements,
) -> Result<PreflightPlan, PreparationFailure> {
    swallowtail_runtime::build_plan(
        &crate::deepseek_direct_descriptor(),
        instance,
        route,
        requirements,
        prepared.access_profile(),
        prepared.access_evidence().status(),
        prepared.available_host_services(),
    )
}
