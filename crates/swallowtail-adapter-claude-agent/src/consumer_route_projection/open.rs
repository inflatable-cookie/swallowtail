use super::contribution::observed_session_contribution;
use crate::ClaudeAgentPreparedSession;
use crate::driver::{ClaudeAgentOpenRejection, ClaudeAgentReasoningAcknowledgement};
use crate::failure::failure;
use swallowtail_runtime::{
    BoxFuture, ConsumerRouteProjectionContribution, ConsumerRouteProjectionSourceId, HostServices,
    InteractiveSessionHandle, RuntimeFailure,
};

/// Future returned by the additive prepared Claude Agent open path.
pub type ClaudeAgentProjectionOpenFuture =
    BoxFuture<'static, Result<ClaudeAgentProjectionOpenOutcome, ClaudeAgentProjectionOpenFailure>>;

/// Open Claude Agent session together with its exact acknowledged contribution.
pub struct ClaudeAgentProjectionOpenOutcome {
    session: Box<dyn InteractiveSessionHandle>,
    contribution: ConsumerRouteProjectionContribution,
}

impl ClaudeAgentProjectionOpenOutcome {
    #[must_use]
    /// Returns the open interactive session.
    pub fn session(&self) -> &dyn InteractiveSessionHandle {
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
        Box<dyn InteractiveSessionHandle>,
        ConsumerRouteProjectionContribution,
    ) {
        (self.session, self.contribution)
    }
}

/// Typed failure returned by the additive prepared Claude Agent open path.
#[allow(clippy::large_enum_variant)]
pub enum ClaudeAgentProjectionOpenFailure {
    /// Transport, setup, malformed, ambiguous, or otherwise unknown failure.
    Runtime(RuntimeFailure),
    /// Exact well-formed differing reasoning acknowledged by the provider.
    Rejected {
        /// Exact route failure produced by the mismatch.
        failure: RuntimeFailure,
        /// Contribution carrying the exact rejected reasoning value.
        contribution: ConsumerRouteProjectionContribution,
    },
}

impl ClaudeAgentProjectionOpenFailure {
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
    /// Splits the failure into route failure and optional contribution.
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

impl ClaudeAgentPreparedSession {
    /// Opens the prepared session and projects its exact reasoning acknowledgement.
    pub fn open_session_with_projection(
        &self,
        prepared_source_id: ConsumerRouteProjectionSourceId,
        active_session_source_id: ConsumerRouteProjectionSourceId,
        services: HostServices,
    ) -> ClaudeAgentProjectionOpenFuture {
        if prepared_source_id == active_session_source_id {
            return Box::pin(async {
                Err(ClaudeAgentProjectionOpenFailure::Runtime(failure(
                    "swallowtail.claude_agent.projection_source_identity_invalid",
                    "Claude Agent prepared and active-session projection sources must differ",
                )))
            });
        }
        let prepared = self.clone();
        let lifecycle = self.open_lifecycle(services);
        Box::pin(async move {
            match lifecycle.await {
                Ok((session, acknowledgement)) => {
                    let reasoning = match &acknowledgement {
                        ClaudeAgentReasoningAcknowledgement::NotRequested => None,
                        ClaudeAgentReasoningAcknowledgement::Effective(value) => {
                            Some((value.as_str(), false))
                        }
                    };
                    let contribution = match observed_session_contribution(
                        &prepared,
                        prepared_source_id,
                        active_session_source_id,
                        reasoning,
                    ) {
                        Ok(contribution) => contribution,
                        Err(rejection) => {
                            let _ = session.close().await;
                            return Err(ClaudeAgentProjectionOpenFailure::Runtime(
                                RuntimeFailure::new(rejection.diagnostic().clone()),
                            ));
                        }
                    };
                    Ok(ClaudeAgentProjectionOpenOutcome {
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

#[allow(clippy::result_large_err)] // The accepted public gate fixes the unboxed failure family.
fn rejected_failure(
    prepared: &ClaudeAgentPreparedSession,
    prepared_source_id: ConsumerRouteProjectionSourceId,
    active_session_source_id: ConsumerRouteProjectionSourceId,
    rejection: ClaudeAgentOpenRejection,
) -> Result<ClaudeAgentProjectionOpenOutcome, ClaudeAgentProjectionOpenFailure> {
    let Some(reasoning) = rejection.rejected_reasoning().map(str::to_owned) else {
        return Err(ClaudeAgentProjectionOpenFailure::Runtime(
            rejection.into_failure(),
        ));
    };
    let failure = rejection.into_failure();
    let contribution = match observed_session_contribution(
        prepared,
        prepared_source_id,
        active_session_source_id,
        Some((&reasoning, true)),
    ) {
        Ok(contribution) => contribution,
        Err(_) => return Err(ClaudeAgentProjectionOpenFailure::Runtime(failure)),
    };
    Err(ClaudeAgentProjectionOpenFailure::Rejected {
        failure,
        contribution,
    })
}
