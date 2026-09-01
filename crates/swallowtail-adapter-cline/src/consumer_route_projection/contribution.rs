use super::{ProjectionBuilder, ProjectionRoute};
use crate::{ClineHeadlessPreparedRun, ClinePreparedSession};
use swallowtail_runtime::{
    ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure,
    ConsumerRouteProjectionSourceId,
};

impl ClinePreparedSession {
    /// Emits only the ACP truth this exact prepared session proves.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        ProjectionBuilder::prepared(self.plan(), ProjectionRoute::Acp, source_id)
            .with_prepared_capabilities()
            .with_harness_mode(true)
            .build()
    }
}

impl ClineHeadlessPreparedRun {
    /// Emits only the headless truth this exact prepared run proves.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        ProjectionBuilder::prepared(self.plan(), ProjectionRoute::Headless, source_id)
            .with_prepared_capabilities()
            .with_harness_mode(false)
            .build()
    }
}

pub(super) fn observed_session_contribution(
    session: &ClinePreparedSession,
    prepared_source_id: ConsumerRouteProjectionSourceId,
    active_source_id: ConsumerRouteProjectionSourceId,
    acknowledgement: Option<(&str, bool)>,
    has_model_observation: bool,
) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
    let mut builder =
        ProjectionBuilder::observed(session.plan(), prepared_source_id, active_source_id)
            .with_prepared_capabilities()
            .with_harness_mode(true);
    if let Some((value, rejected)) = acknowledgement {
        builder = builder.with_plan_acknowledgement(value, rejected)?;
    }
    if has_model_observation {
        builder = builder.with_model_observation()?;
    }
    builder.build()
}
