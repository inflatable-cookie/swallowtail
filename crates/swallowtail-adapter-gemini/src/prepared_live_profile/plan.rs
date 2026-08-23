use crate::GeminiLivePreparedIntegration;
use swallowtail_core::{
    CapabilityProfile, CapabilityRequirement, ConfiguredInstance, Diagnostic, ModelRoute,
    PreflightContext, PreflightPlan, preflight,
};
use swallowtail_runtime::{PreparationFailure, PreparationStage, PreparedOperationEvidence};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Immutable access and preflight evidence for a Gemini Live session.
pub struct GeminiLivePreparedEvidence {
    operation: PreparedOperationEvidence,
}

impl GeminiLivePreparedEvidence {
    pub(super) fn from_prepared(
        prepared: &GeminiLivePreparedIntegration,
        plan: PreflightPlan,
    ) -> Result<Self, PreparationFailure> {
        Ok(Self {
            operation: PreparedOperationEvidence::from_plan(
                plan,
                prepared.access_evidence().clone(),
            )?,
        })
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

    /// Returns the immutable preflight plan.
    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.operation.plan()
    }
}

pub(super) fn model_route(
    prepared: &GeminiLivePreparedIntegration,
    capabilities: CapabilityProfile,
) -> ModelRoute {
    ModelRoute::new(
        swallowtail_core::ModelRouteId::new(crate::GEMINI_LIVE_MODEL_ROUTE_ID)
            .expect("static Gemini Live route id is valid"),
        swallowtail_core::ModelRouteRevision::new("prepared-2")
            .expect("static Gemini Live route revision is valid"),
        prepared.instance().id().clone(),
        swallowtail_core::ModelId::new(crate::GEMINI_LIVE_MODEL_ID)
            .expect("static Gemini Live model id is valid"),
        capabilities,
    )
    .with_provider_id(
        swallowtail_core::ProviderId::new("gemini").expect("static Gemini provider id is valid"),
    )
}

pub(super) fn instance_with_capabilities(
    prepared: &GeminiLivePreparedIntegration,
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
    prepared: &GeminiLivePreparedIntegration,
    instance: &ConfiguredInstance,
    route: &ModelRoute,
    capabilities: impl IntoIterator<Item = CapabilityRequirement>,
) -> Result<PreflightPlan, PreparationFailure> {
    preflight(
        &PreflightContext::new(
            &crate::gemini_live_descriptor(),
            instance,
            prepared.access_profile(),
            prepared.access_evidence().status(),
            prepared.available_host_services(),
        )
        .with_model_route(route),
        &crate::gemini_live_requirements_with_capabilities(
            prepared.instance().execution_host_id().clone(),
            capabilities,
        ),
    )
    .map_err(|error| {
        PreparationFailure::new(
            PreparationStage::Preflight,
            Diagnostic::new(error.diagnostic().clone()),
        )
    })
}
