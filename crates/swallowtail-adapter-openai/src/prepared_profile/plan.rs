use crate::OpenAiBackgroundPreparedIntegration;
use swallowtail_core::{
    CapabilityProfile, CapabilityRequirement, ConfiguredInstance, ModelRoute, PreflightPlan,
    ProviderId,
};
use swallowtail_runtime::{PreparationFailure, PreparedOperationEvidence};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Prepared background-run evidence, including observable activity support.
pub struct OpenAiBackgroundPreparedEvidence {
    operation: PreparedOperationEvidence,
    service_tier: Option<crate::OpenAiBackgroundServiceTier>,
}

impl OpenAiBackgroundPreparedEvidence {
    pub(super) fn from_prepared(
        prepared: &OpenAiBackgroundPreparedIntegration,
        plan: PreflightPlan,
        activity_profile: swallowtail_core::ObservableActivityProfile,
        service_tier: Option<crate::OpenAiBackgroundServiceTier>,
    ) -> Result<Self, PreparationFailure> {
        Ok(Self {
            operation: PreparedOperationEvidence::from_plan_with_activity_profile(
                plan,
                prepared.access_evidence().clone(),
                activity_profile,
            )?,
            service_tier,
        })
    }

    #[must_use]
    /// Returns access evidence and its provenance.
    pub const fn access(&self) -> &swallowtail_runtime::PreparedAccessEvidence {
        self.operation.access()
    }

    #[must_use]
    /// Returns the provider-neutral prepared operation evidence.
    pub const fn operation(&self) -> &PreparedOperationEvidence {
        &self.operation
    }

    #[must_use]
    /// Returns the activity profile promised by the prepared operation.
    pub const fn observable_activity(&self) -> &swallowtail_core::ObservableActivityProfile {
        self.operation.observable_activity()
    }

    #[must_use]
    /// Returns the immutable background-run preflight plan.
    pub const fn plan(&self) -> &PreflightPlan {
        self.operation.plan()
    }

    #[must_use]
    /// Returns the exact adapter-local service-tier selection, if present.
    pub const fn service_tier(&self) -> Option<crate::OpenAiBackgroundServiceTier> {
        self.service_tier
    }
}

pub(crate) fn model_route(
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

pub(crate) fn instance_with_capabilities(
    prepared: &OpenAiBackgroundPreparedIntegration,
    capabilities: CapabilityProfile,
) -> ConfiguredInstance {
    swallowtail_runtime::instance_with_capabilities(prepared.instance(), capabilities)
}

pub(super) fn build_plan(
    prepared: &OpenAiBackgroundPreparedIntegration,
    instance: &ConfiguredInstance,
    route: &ModelRoute,
    capabilities: impl IntoIterator<Item = CapabilityRequirement>,
) -> Result<PreflightPlan, PreparationFailure> {
    let requirements = crate::openai_background_requirements(
        prepared.instance().execution_host_id().clone(),
        capabilities,
    );
    swallowtail_runtime::build_plan(
        &crate::openai_background_descriptor(),
        instance,
        Some(route),
        &requirements,
        prepared.access_profile(),
        prepared.access_evidence().status(),
        prepared.available_host_services(),
    )
}
