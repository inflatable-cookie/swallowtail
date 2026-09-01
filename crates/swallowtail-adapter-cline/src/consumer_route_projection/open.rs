use super::contribution::observed_session_contribution;
use crate::ClinePreparedSession;
use crate::driver::{ClineModelObservation, ClineOpenRejection, ClinePlanAcknowledgement};
use crate::failure::failure;
use swallowtail_runtime::{
    BoxFuture, ConsumerRouteProjectionContribution, ConsumerRouteProjectionSourceId, HostServices,
    InteractiveSessionHandle, NegotiatedSessionModelOptions, RuntimeFailure,
};

/// Future returned by the additive prepared Cline open path.
pub type ClineProjectionOpenFuture =
    BoxFuture<'static, Result<ClineProjectionOpenOutcome, ClineProjectionOpenFailure>>;

/// Open Cline session together with its exact prepared and observed contribution.
pub struct ClineProjectionOpenOutcome {
    session: Box<dyn InteractiveSessionHandle>,
    contribution: ConsumerRouteProjectionContribution,
}

impl ClineProjectionOpenOutcome {
    #[must_use]
    /// Returns the open interactive session.
    pub fn session(&self) -> &dyn InteractiveSessionHandle {
        self.session.as_ref()
    }

    #[must_use]
    /// Returns the exact contribution proved during open.
    pub const fn contribution(&self) -> &ConsumerRouteProjectionContribution {
        &self.contribution
    }

    #[must_use]
    /// Returns the exact bounded model options observed during open, when present.
    pub fn negotiated_model_options(&self) -> Option<&NegotiatedSessionModelOptions> {
        self.session.negotiated_model_options()
    }

    #[must_use]
    /// Splits the outcome into its session and contribution.
    pub fn into_parts(
        self,
    ) -> (
        Box<dyn InteractiveSessionHandle>,
        ConsumerRouteProjectionContribution,
    ) {
        (self.session, self.contribution)
    }
}

/// Typed failure returned by the additive prepared Cline open path.
#[allow(clippy::large_enum_variant)]
pub enum ClineProjectionOpenFailure {
    /// Transport, setup, malformed, ambiguous, or projection failure.
    Runtime(RuntimeFailure),
    /// Exact admitted Act confirmation rejected the requested Plan mode.
    Rejected {
        /// Exact route failure produced by the mismatch.
        failure: RuntimeFailure,
        /// Contribution carrying the exact rejected Plan value.
        contribution: ConsumerRouteProjectionContribution,
    },
}

impl ClineProjectionOpenFailure {
    #[must_use]
    /// Returns the exact route failure.
    pub const fn failure(&self) -> &RuntimeFailure {
        match self {
            Self::Runtime(failure) | Self::Rejected { failure, .. } => failure,
        }
    }

    #[must_use]
    /// Returns the rejected contribution when exact evidence proved one.
    pub const fn rejected_contribution(&self) -> Option<&ConsumerRouteProjectionContribution> {
        match self {
            Self::Runtime(_) => None,
            Self::Rejected { contribution, .. } => Some(contribution),
        }
    }

    #[must_use]
    /// Splits the failure into its route failure and optional contribution.
    pub fn into_parts(self) -> (RuntimeFailure, Option<ConsumerRouteProjectionContribution>) {
        match self {
            Self::Runtime(failure) => (failure, None),
            Self::Rejected {
                failure,
                contribution,
            } => (failure, Some(contribution)),
        }
    }
}

impl ClinePreparedSession {
    /// Opens the prepared session and projects exact Plan and model observations.
    pub fn open_session_with_projection(
        &self,
        prepared_source_id: ConsumerRouteProjectionSourceId,
        active_session_source_id: ConsumerRouteProjectionSourceId,
        services: HostServices,
    ) -> ClineProjectionOpenFuture {
        if prepared_source_id == active_session_source_id {
            return Box::pin(async {
                Err(ClineProjectionOpenFailure::Runtime(failure(
                    "swallowtail.cline.projection_source_identity_invalid",
                    "Cline prepared and active-session projection sources must differ",
                )))
            });
        }
        let prepared = self.clone();
        let lifecycle = self.open_lifecycle(services);
        Box::pin(async move {
            match lifecycle.await {
                Ok((session, observation)) => {
                    let acknowledgement = match &observation.plan_acknowledgement {
                        ClinePlanAcknowledgement::NotRequested => None,
                        ClinePlanAcknowledgement::Effective(value) => Some((value.as_str(), false)),
                    };
                    let has_model = match observation.model {
                        ClineModelObservation::Absent => false,
                        ClineModelObservation::Exact(_) => true,
                        ClineModelObservation::Invalid(error) => {
                            let _ = session.close().await;
                            return Err(ClineProjectionOpenFailure::Runtime(error));
                        }
                    };
                    let contribution = match observed_session_contribution(
                        &prepared,
                        prepared_source_id,
                        active_session_source_id,
                        acknowledgement,
                        has_model,
                    ) {
                        Ok(contribution) => contribution,
                        Err(rejection) => {
                            let _ = session.close().await;
                            return Err(ClineProjectionOpenFailure::Runtime(RuntimeFailure::new(
                                rejection.diagnostic().clone(),
                            )));
                        }
                    };
                    Ok(ClineProjectionOpenOutcome {
                        session,
                        contribution,
                    })
                }
                Err(rejection) => rejected_failure(
                    &prepared,
                    prepared_source_id,
                    active_session_source_id,
                    rejection,
                ),
            }
        })
    }
}

#[allow(clippy::result_large_err)]
fn rejected_failure(
    prepared: &ClinePreparedSession,
    prepared_source_id: ConsumerRouteProjectionSourceId,
    active_session_source_id: ConsumerRouteProjectionSourceId,
    rejection: ClineOpenRejection,
) -> Result<ClineProjectionOpenOutcome, ClineProjectionOpenFailure> {
    let Some(value) = rejection.rejected_plan_value().map(str::to_owned) else {
        return Err(ClineProjectionOpenFailure::Runtime(
            rejection.into_failure(),
        ));
    };
    let failure = rejection.into_failure();
    let contribution = match observed_session_contribution(
        prepared,
        prepared_source_id,
        active_session_source_id,
        Some((&value, true)),
        false,
    ) {
        Ok(contribution) => contribution,
        Err(_) => return Err(ClineProjectionOpenFailure::Runtime(failure)),
    };
    Err(ClineProjectionOpenFailure::Rejected {
        failure,
        contribution,
    })
}
