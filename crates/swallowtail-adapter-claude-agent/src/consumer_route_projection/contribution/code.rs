use super::Contribution;
use crate::consumer_route_projection::builder::{ProjectionBuilder, ProjectionRoute, exact};
use crate::{ClaudeCodePreparedRun, ClaudeCodeResponsePreparedRun};
use swallowtail_runtime::{
    ConsumerRouteControlId, ConsumerRouteOmissionSemantics, ConsumerRouteProjectionSourceId,
    ConsumerRouteValueKind,
};

impl ClaudeCodePreparedRun {
    /// Emits only exact native headless structured-run truth.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Contribution {
        let mut builder =
            ProjectionBuilder::prepared(self.plan(), ProjectionRoute::CodeHeadless, source_id)
                .with_prepared_capabilities()
                .with_model_selection();
        if let Some(reasoning) = self.request().policy().reasoning_mode() {
            builder.push_control(
                ConsumerRouteControlId::ReasoningSelection,
                ConsumerRouteValueKind::BoundedEnumeration,
                exact(reasoning.as_str())?,
                ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
                false,
            );
        }
        if let Some(maximum) = self.maximum_turns() {
            let control = builder.local_control("control.maximum-agentic-turns")?;
            builder.push_control(
                control,
                ConsumerRouteValueKind::BoundedInteger,
                exact(&maximum.as_u32().to_string())?,
                ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
                false,
            );
        }
        builder.build()
    }
}

impl ClaudeCodeResponsePreparedRun {
    /// Emits only exact response-only structured-run truth.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Contribution {
        let mut builder =
            ProjectionBuilder::prepared(self.plan(), ProjectionRoute::CodeResponseOnly, source_id)
                .with_prepared_capabilities()
                .with_model_selection();
        if let Some(reasoning) = self.request().policy().reasoning_mode() {
            builder.push_control(
                ConsumerRouteControlId::ReasoningSelection,
                ConsumerRouteValueKind::BoundedEnumeration,
                exact(reasoning.as_str())?,
                ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
                false,
            );
        }
        builder.build()
    }
}
