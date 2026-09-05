//! Contract 061 contributions from prepared xAI Responses operations.

#[path = "consumer_route_projection/builder.rs"]
mod builder;

use crate::{XaiPreparedModels, XaiPreparedResponsesRun, XaiPreparedResponsesSession};
use builder::Projection;
use swallowtail_runtime::{
    ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure,
    ConsumerRouteProjectionSourceId,
};

/// Exact census route for xAI Responses WebSocket operations.
pub(crate) const ROUTE: &str = "xai.responses-websocket";

impl XaiPreparedModels {
    /// Emits the prepared xAI model-catalogue truth.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        Projection::new(self.plan(), source_id)
            .with_prepared_capabilities()
            .build()
    }
}

impl XaiPreparedResponsesRun {
    /// Emits the prepared xAI structured-run truth.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        Projection::new(self.plan(), source_id)
            .with_prepared_capabilities()
            .with_model_selection()
            .with_reasoning(self.evidence().reasoning_mode())
            .with_maximum_output_tokens(self.evidence().maximum_output_tokens())
            .build()
    }
}

impl XaiPreparedResponsesSession {
    /// Emits the prepared xAI interactive-session truth.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        Projection::new(self.plan(), source_id)
            .with_prepared_capabilities()
            .with_persistent_session_posture()
            .with_model_selection()
            .with_reasoning(self.evidence().reasoning_mode())
            .build()
    }
}
