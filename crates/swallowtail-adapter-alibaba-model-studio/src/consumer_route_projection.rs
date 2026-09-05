//! Contract 061 contributions from prepared Alibaba Model Studio operations.

#[path = "consumer_route_projection/builder.rs"]
mod builder;

use crate::{
    AlibabaModelStudioPreparedConversation, AlibabaModelStudioPreparedDelete,
    AlibabaModelStudioPreparedRetainedConversation, AlibabaModelStudioPreparedRun,
    AlibabaModelStudioPreparedSessionHistory, AlibabaPreparedDeployableModels,
};
use builder::Projection;
use swallowtail_runtime::{
    ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure,
    ConsumerRouteProjectionSourceId,
};

/// Exact census route for the Alibaba Conversations package.
pub(crate) const ROUTE: &str = "alibaba.conversations";

impl AlibabaPreparedDeployableModels {
    /// Emits the prepared Alibaba model-catalogue truth.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        Projection::new(self.plan(), source_id)
            .with_prepared_capabilities()
            .build()
    }
}

impl AlibabaModelStudioPreparedConversation {
    /// Emits the prepared delete-on-close conversation truth.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        Projection::new(self.plan(), source_id)
            .with_prepared_capabilities()
            .with_model_selection()
            .build()
    }
}

impl AlibabaModelStudioPreparedRun {
    /// Emits the prepared structured-run truth.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        Projection::new(self.plan(), source_id)
            .with_prepared_capabilities()
            .with_model_selection()
            .with_fixed_wire_turn_options()
            .build()
    }
}

impl AlibabaModelStudioPreparedRetainedConversation {
    /// Emits the prepared retained-conversation truth.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        Projection::new(self.plan(), source_id)
            .with_prepared_capabilities()
            .with_model_selection()
            .with_provider_state_policy()
            .with_load_session()
            .build()
    }
}

impl AlibabaModelStudioPreparedSessionHistory {
    /// Emits only the prepared-facade truth for retained history.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        Projection::new(self.plan().preflight(), source_id)
            .with_prepared_facade()
            .build()
    }
}

impl AlibabaModelStudioPreparedDelete {
    /// Emits the prepared retained-session deletion truth.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        Projection::new(self.plan().preflight(), source_id)
            .with_prepared_capabilities()
            .build()
    }
}
