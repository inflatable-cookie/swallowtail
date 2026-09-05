//! Contract 061 contributions from prepared Anthropic Messages operations.

#[path = "consumer_route_projection/builder.rs"]
mod builder;

use crate::{
    AnthropicPreparedCatalogue, AnthropicPreparedInferenceAttempt,
    AnthropicPreparedManagedAgentRun, AnthropicPreparedSession,
};
use builder::Projection;
use swallowtail_runtime::{
    ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure,
    ConsumerRouteProjectionSourceId,
};

/// Exact census route for Anthropic Messages operations.
pub(crate) const MESSAGES_ROUTE: &str = "anthropic.messages";
/// Exact census route for Anthropic Managed Agents operations.
pub(crate) const MANAGED_ROUTE: &str = "anthropic.managed-agent";

impl AnthropicPreparedCatalogue {
    /// Emits the prepared Anthropic model-catalogue truth.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        Projection::new(self.plan(), MESSAGES_ROUTE, source_id)
            .with_prepared_capabilities(true)
            .build()
    }
}

impl AnthropicPreparedInferenceAttempt {
    /// Emits the prepared Anthropic structured-run truth.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        Projection::new(self.plan(), MESSAGES_ROUTE, source_id)
            .with_prepared_capabilities(true)
            .with_model_selection()
            .with_reasoning(self.evidence().reasoning_mode())
            .with_maximum_output_tokens(self.request().maximum_output_tokens())
            .with_attachments(self.request().attachments().len() > 0)
            .with_web_search()
            .with_thinking(self.evidence().thinking_mode())
            .build()
    }
}

impl AnthropicPreparedSession {
    /// Emits the prepared Anthropic interactive-session truth.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        Projection::new(self.plan(), MESSAGES_ROUTE, source_id)
            .with_prepared_capabilities(false)
            .with_model_selection()
            .with_reasoning(self.evidence().reasoning_mode())
            .with_session_tools_and_reasoning()
            .with_thinking(self.evidence().thinking_mode())
            .build()
    }
}

impl AnthropicPreparedManagedAgentRun {
    /// Emits the prepared Anthropic Managed Agents structured-run truth.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        let mut projection = Projection::new(self.plan(), MANAGED_ROUTE, source_id)
            .with_prepared_capabilities(false)
            .with_model_selection()
            .with_managed_policies();
        if self.request().tools().len() > 0
            && self
                .plan()
                .requirements()
                .capabilities()
                .any(|required| required.capability() == swallowtail_core::Capability::ToolCalls)
        {
            projection = projection.with_per_turn_tool_exchange();
        }
        projection.build()
    }
}
