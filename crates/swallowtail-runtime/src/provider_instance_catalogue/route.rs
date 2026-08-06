use crate::PreparedOperationEvidence;
use swallowtail_core::{
    CapabilityRequirement, DriverRole, ExecutionLayer, ModelId, ModelRouteId, ModelRouteRevision,
    OperationShape, ProviderId,
};

/// Exact selected model route projected without executable authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfiguredProviderModelRoute {
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    model_id: ModelId,
    provider_id: Option<ProviderId>,
}

impl ConfiguredProviderModelRoute {
    #[must_use]
    /// Returns the model-route identity.
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

/// Non-executable projection of one exact prepared operation route.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfiguredProviderInstanceRoute {
    driver_role: DriverRole,
    execution_layer: ExecutionLayer,
    operation_shape: OperationShape,
    capabilities: Vec<CapabilityRequirement>,
    model_route: Option<ConfiguredProviderModelRoute>,
}

impl ConfiguredProviderInstanceRoute {
    pub(super) fn from_evidence(evidence: &PreparedOperationEvidence) -> Self {
        let plan = evidence.plan();
        let model_route = plan
            .model_route_id()
            .map(|route_id| ConfiguredProviderModelRoute {
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
            });
        Self {
            driver_role: evidence.binding().driver_role(),
            execution_layer: evidence.binding().execution_layer(),
            operation_shape: evidence.binding().operation_shape(),
            capabilities: plan.requirements().capabilities().cloned().collect(),
            model_route,
        }
    }

    #[must_use]
    /// Returns the prepared driver role.
    pub const fn driver_role(&self) -> DriverRole {
        self.driver_role
    }

    #[must_use]
    /// Returns the prepared execution layer.
    pub const fn execution_layer(&self) -> ExecutionLayer {
        self.execution_layer
    }

    #[must_use]
    /// Returns the prepared operation shape.
    pub const fn operation_shape(&self) -> OperationShape {
        self.operation_shape
    }

    /// Iterates the exact prepared capability requirements.
    pub fn capabilities(&self) -> impl ExactSizeIterator<Item = &CapabilityRequirement> {
        self.capabilities.iter()
    }

    #[must_use]
    /// Returns the selected model route, when this operation fixed one.
    pub const fn model_route(&self) -> Option<&ConfiguredProviderModelRoute> {
        self.model_route.as_ref()
    }
}
