use super::contribution::{ObservedReasoning, observed_contribution};
use crate::OpenAiPreparedRealtimeSession;
use crate::failure::failure;
use crate::realtime::open_realtime_lifecycle;
use swallowtail_runtime::{
    BoxFuture, ConsumerRouteProjectionContribution, ConsumerRouteProjectionSourceId, HostServices,
    RealtimeMediaSessionHandle, RuntimeFailure,
};

/// Future returned by the additive prepared Realtime open path.
pub type OpenAiRealtimeProjectionOpenFuture = BoxFuture<
    'static,
    Result<OpenAiRealtimeProjectionOpenOutcome, OpenAiRealtimeProjectionOpenFailure>,
>;

/// Open Realtime session together with its exact acknowledged contribution.
///
/// The outcome is returned only after the exact `session.updated` event.
pub struct OpenAiRealtimeProjectionOpenOutcome {
    session: Box<dyn RealtimeMediaSessionHandle>,
    contribution: ConsumerRouteProjectionContribution,
}

impl OpenAiRealtimeProjectionOpenOutcome {
    #[must_use]
    /// Returns the open Realtime media session.
    pub fn session(&self) -> &dyn RealtimeMediaSessionHandle {
        self.session.as_ref()
    }

    #[must_use]
    /// Returns the exact contribution the acknowledgement proved.
    pub const fn contribution(&self) -> &ConsumerRouteProjectionContribution {
        &self.contribution
    }

    #[must_use]
    /// Splits the outcome into its session handle and contribution.
    pub fn into_parts(
        self,
    ) -> (
        Box<dyn RealtimeMediaSessionHandle>,
        ConsumerRouteProjectionContribution,
    ) {
        (self.session, self.contribution)
    }
}

/// Typed failure returned by the additive prepared Realtime open path.
///
/// The variant shapes are fixed by the accepted public baseline, so the
/// rejected contribution stays inline rather than behind a box.
#[allow(clippy::large_enum_variant)]
pub enum OpenAiRealtimeProjectionOpenFailure {
    /// Transport, setup, ordering, timeout, disconnect, or unknown evidence.
    ///
    /// This variant never carries a rejected contribution.
    Runtime(RuntimeFailure),
    /// The provider acknowledged an exact, well-formed different effort.
    Rejected {
        /// The exact route failure the acknowledgement produced.
        failure: RuntimeFailure,
        /// The contribution carrying the exact rejected reasoning state.
        contribution: ConsumerRouteProjectionContribution,
    },
}

impl OpenAiRealtimeProjectionOpenFailure {
    #[must_use]
    /// Returns the exact route failure.
    pub const fn failure(&self) -> &RuntimeFailure {
        match self {
            Self::Runtime(failure) | Self::Rejected { failure, .. } => failure,
        }
    }

    #[must_use]
    /// Returns the rejected contribution when an exact acknowledgement proved one.
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

impl OpenAiPreparedRealtimeSession {
    /// Opens the prepared Realtime session and projects its exact acknowledgement.
    ///
    /// This path shares one private low-level lifecycle with `open_session`,
    /// so transport, setup, `session.updated` validation, handle construction,
    /// failure, and cleanup are identical. A success contribution is returned
    /// only after a matching acknowledgement; a rejected contribution only
    /// after an exact, well-formed different effort. Every other failure
    /// carries no contribution, and omitted reasoning produces no reasoning
    /// state.
    ///
    /// The two source ids are independently admitted and must differ. Prepared
    /// selection and session-start rows keep `prepared_source_id`; post-open
    /// observation, provider-effective, rejected, and acknowledgement truth
    /// keeps `active_session_source_id`. Equal ids are rejected before any
    /// provider work rather than collapsing the two evidence sources.
    pub fn open_session_with_projection(
        &self,
        prepared_source_id: ConsumerRouteProjectionSourceId,
        active_session_source_id: ConsumerRouteProjectionSourceId,
        services: HostServices,
    ) -> OpenAiRealtimeProjectionOpenFuture {
        if prepared_source_id == active_session_source_id {
            return Box::pin(async {
                Err(OpenAiRealtimeProjectionOpenFailure::Runtime(failure(
                    "swallowtail.openai.realtime_projection_source_identity_invalid",
                    "OpenAI Realtime prepared and active-session projection sources must differ",
                )))
            });
        }
        let prepared = self.clone();
        let plan = self.plan().clone();
        let request = self.request().clone();
        Box::pin(async move {
            match open_realtime_lifecycle(plan, request, services).await {
                Ok((session, acknowledgement)) => {
                    let observed = match &acknowledgement {
                        crate::realtime::RealtimeAcknowledgement::NotRequested => {
                            ObservedReasoning::None
                        }
                        crate::realtime::RealtimeAcknowledgement::Effective(effort) => {
                            ObservedReasoning::Effective(effort)
                        }
                    };
                    let contribution = observed_contribution(
                        &prepared,
                        prepared_source_id,
                        active_session_source_id,
                        observed,
                    )
                    .map_err(|rejection| {
                        OpenAiRealtimeProjectionOpenFailure::Runtime(RuntimeFailure::new(
                            rejection.diagnostic().clone(),
                        ))
                    })?;
                    Ok(OpenAiRealtimeProjectionOpenOutcome {
                        session,
                        contribution,
                    })
                }
                Err(rejection) => {
                    let Some(effort) = rejection.rejected_effort().map(str::to_owned) else {
                        return Err(OpenAiRealtimeProjectionOpenFailure::Runtime(
                            rejection.into_failure(),
                        ));
                    };
                    match observed_contribution(
                        &prepared,
                        prepared_source_id,
                        active_session_source_id,
                        ObservedReasoning::Rejected(&effort),
                    ) {
                        Ok(contribution) => Err(OpenAiRealtimeProjectionOpenFailure::Rejected {
                            failure: rejection.into_failure(),
                            contribution,
                        }),
                        Err(_) => Err(OpenAiRealtimeProjectionOpenFailure::Runtime(
                            rejection.into_failure(),
                        )),
                    }
                }
            }
        })
    }
}
