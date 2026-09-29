use super::Projection;
use super::open::{GrokProjectionOpenFailure, GrokProjectionOpenFuture, GrokProjectionOpenOutcome};
use crate::{GrokPreparedRun, GrokPreparedSession};
use swallowtail_runtime::{
    ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure,
    ConsumerRouteProjectionSourceId, HostServices, RuntimeFailure, SessionCleanupRequest,
};

impl GrokPreparedSession {
    /// Emits only the interactive-session rows this prepared session proves.
    pub fn consumer_route_projection_contribution(
        &self,
        source: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        Projection::new(self.plan(), source)
            .prepared(true)
            .model_selection()
            .session_options()
            .build()
    }
    /// Opens ACP and publishes negotiated options only when the open retained them.
    pub fn open_session_with_projection(
        &self,
        prepared_source: ConsumerRouteProjectionSourceId,
        active_source: ConsumerRouteProjectionSourceId,
        cleanup: SessionCleanupRequest,
        services: HostServices,
    ) -> GrokProjectionOpenFuture {
        if prepared_source == active_source {
            return Box::pin(async {
                Err(GrokProjectionOpenFailure::Runtime(crate::failure::failure(
                    "swallowtail.grok.projection_source_identity_invalid",
                    "Grok prepared and active-session projection sources must differ",
                )))
            });
        }
        let prepared = self.clone();
        Box::pin(async move {
            let session = prepared
                .open_session(services.clone())
                .await
                .map_err(GrokProjectionOpenFailure::Runtime)?;
            let contribution = Projection::observed(
                prepared.plan(),
                prepared_source,
                active_source,
                session.negotiated_model_options().is_some(),
            )
            .prepared(true)
            .model_selection()
            .session_options()
            .build();
            match contribution {
                Ok(contribution) => Ok(GrokProjectionOpenOutcome::new(session, contribution)),
                Err(rejection) => {
                    let _ = session.close(cleanup, services).await;
                    Err(GrokProjectionOpenFailure::Runtime(RuntimeFailure::new(
                        rejection.diagnostic().clone(),
                    )))
                }
            }
        })
    }
}
impl GrokPreparedRun {
    /// Emits only the structured-run rows this prepared run proves.
    pub fn consumer_route_projection_contribution(
        &self,
        source: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        Projection::new(self.plan(), source)
            .prepared(false)
            .model_selection()
            .build()
    }
}
impl crate::GrokPreparedCatalogue {
    /// Emits only the model-catalogue rows this prepared catalogue proves.
    pub fn consumer_route_projection_contribution(
        &self,
        source: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        Projection::new(self.plan(), source).prepared(false).build()
    }
}
