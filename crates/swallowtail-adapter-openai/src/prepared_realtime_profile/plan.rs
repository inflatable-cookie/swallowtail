use crate::OpenAiRealtimePreparedIntegration;
use swallowtail_core::{
    CapabilityProfile, CapabilityRequirement, ConfiguredInstance, Diagnostic, ModelRoute,
    PreflightContext, PreflightPlan, preflight,
};
use swallowtail_runtime::{PreparationFailure, PreparationStage, PreparedOperationEvidence};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenAiRealtimePreparedEvidence {
    operation: PreparedOperationEvidence,
}

impl OpenAiRealtimePreparedEvidence {
    pub(super) fn from_prepared(
        prepared: &OpenAiRealtimePreparedIntegration,
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
    prepared: &OpenAiRealtimePreparedIntegration,
    capabilities: CapabilityProfile,
) -> ModelRoute {
    ModelRoute::new(
        swallowtail_core::ModelRouteId::new(crate::OPENAI_REALTIME_MODEL_ROUTE_ID)
            .expect("static OpenAI Realtime route id is valid"),
        swallowtail_core::ModelRouteRevision::new("prepared-1")
            .expect("static OpenAI Realtime route revision is valid"),
        prepared.instance().id().clone(),
        swallowtail_core::ModelId::new(crate::OPENAI_REALTIME_MODEL_ID)
            .expect("static OpenAI Realtime model id is valid"),
        capabilities,
    )
    .with_provider_id(
        swallowtail_core::ProviderId::new("openai").expect("static OpenAI provider id is valid"),
    )
}

pub(super) fn instance_with_capabilities(
    prepared: &OpenAiRealtimePreparedIntegration,
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
    prepared: &OpenAiRealtimePreparedIntegration,
    instance: &ConfiguredInstance,
    route: &ModelRoute,
    capabilities: impl IntoIterator<Item = CapabilityRequirement>,
) -> Result<PreflightPlan, PreparationFailure> {
    preflight(
        &PreflightContext::new(
            &crate::openai_realtime_descriptor(),
            instance,
            prepared.access_profile(),
            prepared.access_evidence().status(),
            prepared.available_host_services(),
        )
        .with_model_route(route),
        &crate::openai_realtime_requirements(
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
