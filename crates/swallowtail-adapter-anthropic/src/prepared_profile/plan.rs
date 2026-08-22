use crate::AnthropicPreparedIntegration;
use swallowtail_core::{
    Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    CredentialState, DriverRole, ExecutionLayer, HostServiceKind, ModelRoute,
    OperationRequirements, OperationShape, PreflightPlan, ProviderId, ReasoningMode,
    SessionAccessPolicy, SessionProviderStatePolicy,
};
use swallowtail_runtime::{PreparationFailure, PreparedOperationEvidence};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Inspectable prepared evidence for one Anthropic Messages operation.
pub struct AnthropicPreparedEvidence {
    operation: PreparedOperationEvidence,
    reasoning_mode: Option<ReasoningMode>,
}

impl AnthropicPreparedEvidence {
    pub(super) fn from_prepared(
        prepared: &AnthropicPreparedIntegration,
        plan: PreflightPlan,
    ) -> Result<Self, PreparationFailure> {
        let reasoning_mode = reasoning_mode_from_plan(&plan)?;
        Ok(Self {
            operation: PreparedOperationEvidence::from_plan(
                plan,
                prepared.access_evidence().clone(),
            )?,
            reasoning_mode,
        })
    }

    pub(super) fn from_prepared_with_activity(
        prepared: &AnthropicPreparedIntegration,
        plan: PreflightPlan,
        activity_profile: swallowtail_core::ObservableActivityProfile,
    ) -> Result<Self, PreparationFailure> {
        let reasoning_mode = reasoning_mode_from_plan(&plan)?;
        Ok(Self {
            operation: PreparedOperationEvidence::from_plan_with_activity_profile(
                plan,
                prepared.access_evidence().clone(),
                activity_profile,
            )?,
            reasoning_mode,
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
    /// Returns the exact prepared effort selection, when supplied.
    pub const fn reasoning_mode(&self) -> Option<&ReasoningMode> {
        self.reasoning_mode.as_ref()
    }
}

fn reasoning_mode_from_plan(
    plan: &PreflightPlan,
) -> Result<Option<ReasoningMode>, PreparationFailure> {
    let requirements = plan
        .requirements()
        .capabilities()
        .filter(|requirement| requirement.capability() == Capability::ReasoningSelection)
        .collect::<Vec<_>>();
    match requirements.as_slice() {
        [] => Ok(None),
        [requirement] => {
            let constraints = requirement.constraints().cloned().collect::<Vec<_>>();
            match constraints.as_slice() {
                [CapabilityConstraint::ReasoningMode(mode)] => Ok(Some(mode.clone())),
                _ => Err(crate::prepared::failure(
                    swallowtail_runtime::PreparationStage::Preflight,
                    "swallowtail.anthropic.preparation.reasoning_plan_invalid",
                    "Anthropic effort plan must contain one exact reasoning selection",
                )),
            }
        }
        _ => Err(crate::prepared::failure(
            swallowtail_runtime::PreparationStage::Preflight,
            "swallowtail.anthropic.preparation.reasoning_plan_invalid",
            "Anthropic effort plan must contain one exact reasoning selection",
        )),
    }
}

pub(super) fn instance_with_capabilities(
    prepared: &AnthropicPreparedIntegration,
    capabilities: CapabilityProfile,
) -> ConfiguredInstance {
    swallowtail_runtime::instance_with_capabilities(prepared.instance(), capabilities)
}

pub(super) fn model_route(
    prepared: &AnthropicPreparedIntegration,
    model: super::AnthropicModelSelection,
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
    .with_provider_id(ProviderId::new("anthropic").expect("static provider id is valid"))
}

pub(super) fn requirements(
    prepared: &AnthropicPreparedIntegration,
    role: DriverRole,
    capabilities: impl IntoIterator<Item = CapabilityRequirement>,
    extra_host_services: impl IntoIterator<Item = HostServiceKind>,
) -> OperationRequirements {
    let descriptor = crate::anthropic_direct_descriptor();
    let mut host_services: Vec<_> = descriptor.required_host_services(role).collect();
    host_services.extend(extra_host_services);
    let mut requirements = swallowtail_runtime::base_requirements(
        ExecutionLayer::DirectModelInference,
        if role == DriverRole::InteractiveSession {
            OperationShape::InteractiveSession
        } else {
            OperationShape::StructuredRun
        },
        role,
        prepared.instance(),
        prepared.access_profile(),
        [CredentialState::Ready],
        capabilities,
    )
    .with_host_services(host_services);
    requirements =
        requirements.with_interface_versions([crate::anthropic_messages_facade_binding()]);
    if role == DriverRole::InteractiveSession {
        requirements = requirements
            .with_session_access_policy(SessionAccessPolicy::resource_free())
            .with_session_provider_state_policy(SessionProviderStatePolicy::Prohibited);
    }
    requirements
}

pub(super) fn build_plan(
    prepared: &AnthropicPreparedIntegration,
    instance: &ConfiguredInstance,
    route: Option<&ModelRoute>,
    requirements: &OperationRequirements,
) -> Result<PreflightPlan, PreparationFailure> {
    swallowtail_runtime::build_plan(
        &crate::anthropic_direct_descriptor(),
        instance,
        route,
        requirements,
        prepared.access_profile(),
        prepared.access_evidence().status(),
        prepared.available_host_services(),
    )
}
