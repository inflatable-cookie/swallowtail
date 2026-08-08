use crate::XaiPreparedIntegration;
use swallowtail_core::{ConfiguredInstance, ModelRoute, OperationRequirements, PreflightPlan};
use swallowtail_runtime::{PreparationFailure, PreparedOperationEvidence};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Inspectable prepared evidence for one xAI Responses operation.
pub struct XaiPreparedEvidence {
    operation: PreparedOperationEvidence,
}

impl XaiPreparedEvidence {
    pub(super) fn from_prepared(
        prepared: &XaiPreparedIntegration,
        plan: PreflightPlan,
        activity_profile: swallowtail_core::ObservableActivityProfile,
    ) -> Result<Self, PreparationFailure> {
        Ok(Self {
            operation: PreparedOperationEvidence::from_plan_with_activity_profile(
                plan,
                prepared.access_evidence().clone(),
                activity_profile,
            )?,
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
}

pub(super) fn model_route(
    prepared: &XaiPreparedIntegration,
    selection: super::XaiModelSelection,
    capabilities: swallowtail_core::CapabilityProfile,
) -> ModelRoute {
    let (route_id, revision, model_id) = selection.into_parts();
    let base = crate::xai_responses_model_route(
        prepared.instance().id().clone(),
        route_id,
        revision,
        model_id,
    );
    ModelRoute::new(
        base.id().clone(),
        base.revision().clone(),
        base.instance_id().clone(),
        base.model_id().clone(),
        capabilities,
    )
    .with_provider_id(base.provider_id().expect("xAI route has provider").clone())
}

pub(super) fn instance_with_capabilities(
    prepared: &XaiPreparedIntegration,
    capabilities: swallowtail_core::CapabilityProfile,
) -> ConfiguredInstance {
    swallowtail_runtime::instance_with_capabilities(prepared.instance(), capabilities)
}

pub(super) fn build_plan(
    prepared: &XaiPreparedIntegration,
    instance: &ConfiguredInstance,
    route: &ModelRoute,
    requirements: &OperationRequirements,
) -> Result<PreflightPlan, PreparationFailure> {
    swallowtail_runtime::build_plan(
        &crate::xai_websocket_descriptor(),
        instance,
        Some(route),
        requirements,
        prepared.access_profile(),
        prepared.access_evidence().status(),
        prepared.available_host_services(),
    )
}
