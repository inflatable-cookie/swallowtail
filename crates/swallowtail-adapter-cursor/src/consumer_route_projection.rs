//! Contract 061 projection contributions for prepared Cursor agent operations.

#[path = "consumer_route_projection/builder.rs"]
mod builder;

use crate::headless_command::CursorHeadlessReadMode;
use crate::headless_model_parameters::{
    CursorHeadlessContext, CursorHeadlessFast, parse_plan_model_id,
};
use crate::{CursorPreparedAcpSession, CursorPreparedCatalogue, CursorPreparedHeadlessRun};
use builder::{ProjectionBuilder, exact, route_local};
use swallowtail_runtime::{
    ConsumerRouteControlId, ConsumerRouteOmissionSemantics, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionFailure, ConsumerRouteProjectionSourceId, ConsumerRouteValueKind,
};

const CURSOR_HEADLESS_ROUTE: &str = "cursor-agent.headless";

impl CursorPreparedAcpSession {
    /// Emits only the interactive-session truth this prepared ACP session proves.
    ///
    /// The supplied source identity is preserved as one adapter contribution.
    /// Activity observation stays a post-open descriptor-only row.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        ProjectionBuilder::new(self.plan(), source_id)
            .with_prepared_facade()
            .with_prepared_capabilities()
            .build()
    }
}

impl CursorPreparedCatalogue {
    /// Emits only the authenticated model-catalogue truth this prepared catalogue proves.
    ///
    /// The supplied source identity is preserved as one adapter contribution.
    /// Catalogue operations attach no activity profile and prove no execution
    /// controls, so those rows are withheld at construction.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        ProjectionBuilder::new(self.plan(), source_id)
            .with_prepared_facade()
            .with_prepared_capabilities()
            .build()
    }
}

impl CursorPreparedHeadlessRun {
    /// Emits only the stream-JSON structured-run truth this prepared run proves.
    ///
    /// The supplied source identity is preserved as one adapter contribution.
    /// Activity observation stays a post-open descriptor-only row.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        let segment = self.plan().protocol_facade_id().as_str();
        let mut builder = ProjectionBuilder::new(self.plan(), source_id)
            .with_prepared_facade()
            .with_prepared_capabilities()
            .with_model_selection()?;

        if let Some(parsed) = self
            .plan()
            .model_id()
            .and_then(|model_id| parse_plan_model_id(model_id.as_str()).ok())
        {
            if let Some(fast) = parsed.parameters.fast() {
                let fast_str = match fast {
                    CursorHeadlessFast::Standard => "Standard",
                };
                let control = route_local(segment, "control.fast")?;
                builder.push_session_start_control(
                    ConsumerRouteControlId::Namespaced(control),
                    ConsumerRouteValueKind::BoundedEnumeration,
                    exact(fast_str)?,
                    ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
                );
            }

            if let Some(context) = parsed.parameters.context() {
                let context_str = match context {
                    CursorHeadlessContext::ThreeHundredK => "ThreeHundredK",
                    CursorHeadlessContext::OneMillion => "OneMillion",
                };
                let control = route_local(segment, "control.context-window")?;
                builder.push_session_start_control(
                    ConsumerRouteControlId::Namespaced(control),
                    ConsumerRouteValueKind::BoundedEnumeration,
                    exact(context_str)?,
                    ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
                );
            }

            if let Some(effort) = parsed.parameters.effort() {
                let control = route_local(segment, "control.reasoning-effort")?;
                builder.push_session_start_control(
                    ConsumerRouteControlId::Namespaced(control),
                    ConsumerRouteValueKind::BoundedEnumeration,
                    exact(effort.as_str())?,
                    ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
                );
            }
        }

        if let Some(read_mode) = self.read_mode() {
            let mode_str = match read_mode {
                CursorHeadlessReadMode::Plan => "Plan",
                CursorHeadlessReadMode::Ask => "Ask",
            };
            let control = route_local(segment, "control.read-mode")?;
            builder.push_session_start_control(
                ConsumerRouteControlId::Namespaced(control),
                ConsumerRouteValueKind::BoundedEnumeration,
                exact(mode_str)?,
                ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
            );
        }

        builder.build()
    }
}
