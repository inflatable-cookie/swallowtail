use swallowtail_core::{ModelId, ModelRouteId, ModelRouteRevision, PreflightPlan, ProviderId};

/// Exact model binding one projection row applies to.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConsumerRouteModelBinding {
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    model_id: ModelId,
    provider_id: Option<ProviderId>,
}

impl ConsumerRouteModelBinding {
    /// Derives the exact model binding when the plan fixed a model route.
    pub(super) fn from_plan(plan: &PreflightPlan) -> Option<Self> {
        plan.model_route_id().map(|route_id| Self {
            route_id: route_id.clone(),
            route_revision: plan
                .model_route_revision()
                .expect("a model route id always has a route revision")
                .clone(),
            model_id: plan
                .model_id()
                .expect("a model route id always has a model id")
                .clone(),
            provider_id: plan.provider_id().cloned(),
        })
    }

    #[must_use]
    /// Returns the exact model-route identity.
    pub const fn route_id(&self) -> &ModelRouteId {
        &self.route_id
    }

    #[must_use]
    /// Returns the exact model-route revision.
    pub const fn route_revision(&self) -> &ModelRouteRevision {
        &self.route_revision
    }

    #[must_use]
    /// Returns the selected model identity.
    pub const fn model_id(&self) -> &ModelId {
        &self.model_id
    }

    #[must_use]
    /// Returns the provider identity when the model source supplied one.
    pub const fn provider_id(&self) -> Option<&ProviderId> {
        self.provider_id.as_ref()
    }
}
