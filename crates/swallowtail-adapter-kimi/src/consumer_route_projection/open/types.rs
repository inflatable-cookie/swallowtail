use swallowtail_runtime::{
    BoxFuture, ConsumerRouteProjectionContribution, InteractiveSessionHandle,
    NegotiatedSessionModelOptions, RuntimeFailure,
};

/// Future returned by the additive Kimi projected-open seam.
pub type KimiProjectionOpenFuture =
    BoxFuture<'static, Result<KimiProjectionOpenOutcome, KimiProjectionOpenFailure>>;

/// Open Kimi session together with its exact prepared and observed contribution.
pub struct KimiProjectionOpenOutcome {
    pub(super) session: Box<dyn InteractiveSessionHandle>,
    pub(super) contribution: ConsumerRouteProjectionContribution,
}

impl KimiProjectionOpenOutcome {
    /// Returns the open interactive session.
    #[must_use]
    pub fn session(&self) -> &dyn InteractiveSessionHandle {
        self.session.as_ref()
    }

    /// Returns the exact contribution proved during open.
    #[must_use]
    pub const fn contribution(&self) -> &ConsumerRouteProjectionContribution {
        &self.contribution
    }

    /// Returns exact bounded model options observed during open, when present.
    #[must_use]
    pub fn negotiated_model_options(&self) -> Option<&NegotiatedSessionModelOptions> {
        self.session.negotiated_model_options()
    }

    /// Splits the outcome into its session and contribution.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        Box<dyn InteractiveSessionHandle>,
        ConsumerRouteProjectionContribution,
    ) {
        (self.session, self.contribution)
    }
}

#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
/// Typed failure returned by the additive Kimi projected-open seam.
pub enum KimiProjectionOpenFailure {
    /// Transport, setup, malformed, ambiguous, or projection failure.
    Runtime(RuntimeFailure),
    /// Exact admitted confirmation rejected reasoning or Plan.
    Rejected {
        /// Exact route failure produced by the rejection.
        failure: RuntimeFailure,
        /// Contribution carrying the exact compound acknowledgement.
        contribution: ConsumerRouteProjectionContribution,
    },
}

impl KimiProjectionOpenFailure {
    /// Returns the exact route failure.
    #[must_use]
    pub const fn failure(&self) -> &RuntimeFailure {
        match self {
            Self::Runtime(failure) | Self::Rejected { failure, .. } => failure,
        }
    }

    /// Returns the rejected contribution when exact evidence proved one.
    #[must_use]
    pub const fn rejected_contribution(&self) -> Option<&ConsumerRouteProjectionContribution> {
        match self {
            Self::Runtime(_) => None,
            Self::Rejected { contribution, .. } => Some(contribution),
        }
    }

    /// Splits the failure into route failure and optional contribution.
    #[must_use]
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
