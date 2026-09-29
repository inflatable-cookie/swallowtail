use super::open::{
    GeminiProjectionOpenFailure, GeminiProjectionOpenFuture, GeminiProjectionOpenOutcome,
};
use super::{Projection, Route};
use crate::{GeminiHeadlessPreparedRun, GeminiPreparedLiveSession, GeminiPreparedSession};
use swallowtail_runtime::{
    ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure,
    ConsumerRouteProjectionSourceId, HostServices, RuntimeFailure, SessionCleanupRequest,
};

impl GeminiPreparedSession {
    /// Emits only the ACP rows this prepared session proves.
    pub fn consumer_route_projection_contribution(
        &self,
        source: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        Projection::new(self.plan(), Route::Acp, source)
            .prepared()
            .harness_mode()
            .http_mcp_placement(self.http_mcp())
            .build()
    }
    /// Opens ACP and publishes retained negotiated options only after success.
    pub fn open_session_with_projection(
        &self,
        prepared_source: ConsumerRouteProjectionSourceId,
        active_source: ConsumerRouteProjectionSourceId,
        cleanup: SessionCleanupRequest,
        services: HostServices,
    ) -> GeminiProjectionOpenFuture {
        if prepared_source == active_source {
            return Box::pin(async {
                Err(GeminiProjectionOpenFailure::Runtime(
                    crate::failure::failure(
                        "swallowtail.gemini.projection_source_identity_invalid",
                        "Gemini prepared and active-session projection sources must differ",
                    ),
                ))
            });
        }
        let prepared = self.clone();
        Box::pin(async move {
            let session = prepared
                .open_session(services.clone())
                .await
                .map_err(GeminiProjectionOpenFailure::Runtime)?;
            let contribution = Projection::observed(
                prepared.plan(),
                Route::Acp,
                prepared_source,
                active_source,
                session.negotiated_model_options().is_some(),
            )
            .prepared()
            .harness_mode()
            .http_mcp_placement(prepared.http_mcp())
            .build();
            match contribution {
                Ok(contribution) => Ok(GeminiProjectionOpenOutcome::new(session, contribution)),
                Err(rejection) => {
                    let _ = session.close(cleanup, services).await;
                    Err(GeminiProjectionOpenFailure::Runtime(RuntimeFailure::new(
                        rejection.diagnostic().clone(),
                    )))
                }
            }
        })
    }
}
impl GeminiHeadlessPreparedRun {
    /// Emits only the headless rows this prepared run proves.
    pub fn consumer_route_projection_contribution(
        &self,
        source: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        Projection::new(self.plan(), Route::Headless, source)
            .prepared()
            .model_selection()
            .build()
    }
}
impl GeminiPreparedLiveSession {
    /// Emits only the Live rows this prepared session and request prove.
    pub fn consumer_route_projection_contribution(
        &self,
        source: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        Projection::new(self.plan(), Route::Live, source)
            .prepared()
            .live_controls(self)
            .build()
    }
}
