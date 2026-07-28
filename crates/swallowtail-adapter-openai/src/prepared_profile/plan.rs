use crate::OpenAiBackgroundPreparedIntegration;
use swallowtail_core::{
    CapabilityProfile, CapabilityRequirement, ConfiguredInstance, Diagnostic, ModelRoute,
    PreflightContext, PreflightPlan, ProviderId, preflight,
};
use swallowtail_runtime::{PreparationFailure, PreparationStage, PreparedOperationEvidence};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenAiBackgroundPreparedEvidence {
    operation: PreparedOperationEvidence,
}

impl OpenAiBackgroundPreparedEvidence {
    pub(super) fn from_prepared(
        prepared: &OpenAiBackgroundPreparedIntegration,
        plan: PreflightPlan,
    ) -> Result<Self, PreparationFailure> {
        Ok(Self {
            operation: PreparedOperationEvidence::from_plan(
                plan,
                prepared.access_evidence().clone(),
            )?,
        })
    }

    #[must_use]
    pub const fn access(&self) -> &swallowtail_runtime::PreparedAccessEvidence {
        self.operation.access()
    }

    #[must_use]
    pub const fn operation(&self) -> &PreparedOperationEvidence {
        &self.operation
    }

    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.operation.plan()
    }
}

pub(super) fn model_route(
    prepared: &OpenAiBackgroundPreparedIntegration,
    selection: super::OpenAiBackgroundModelSelection,
    capabilities: CapabilityProfile,
) -> ModelRoute {
    let (route_id, route_revision, model_id) = selection.into_parts();
    ModelRoute::new(
        route_id,
        route_revision,
        prepared.instance().id().clone(),
        model_id,
        capabilities,
    )
    .with_provider_id(ProviderId::new("openai").expect("static OpenAI provider identity is valid"))
}

pub(super) fn instance_with_capabilities(
    prepared: &OpenAiBackgroundPreparedIntegration,
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

pub(super) fn build_plan(
    prepared: &OpenAiBackgroundPreparedIntegration,
    instance: &ConfiguredInstance,
    route: &ModelRoute,
    capabilities: impl IntoIterator<Item = CapabilityRequirement>,
) -> Result<PreflightPlan, PreparationFailure> {
    let descriptor = crate::openai_background_descriptor();
    let requirements = crate::openai_background_requirements(
        prepared.instance().execution_host_id().clone(),
        capabilities,
    );
    preflight(
        &PreflightContext::new(
            &descriptor,
            instance,
            prepared.access_profile(),
            prepared.access_evidence().status(),
            prepared.available_host_services(),
        )
        .with_model_route(route),
        &requirements,
    )
    .map_err(|error| {
        PreparationFailure::new(
            PreparationStage::Preflight,
            Diagnostic::new(error.diagnostic().clone()),
        )
    })
}
