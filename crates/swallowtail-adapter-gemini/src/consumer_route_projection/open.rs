use swallowtail_runtime::{
    BoxFuture, ConsumerRouteProjectionContribution, InteractiveSessionHandle,
    NegotiatedSessionModelOptions, RuntimeFailure,
};

/// Result of opening Gemini ACP through the additive projection seam.
pub struct GeminiProjectionOpenOutcome {
    session: Box<dyn InteractiveSessionHandle>,
    contribution: ConsumerRouteProjectionContribution,
}

impl GeminiProjectionOpenOutcome {
    pub(crate) fn new(
        session: Box<dyn InteractiveSessionHandle>,
        contribution: ConsumerRouteProjectionContribution,
    ) -> Self {
        Self {
            session,
            contribution,
        }
    }
    /// Returns the open session.
    #[must_use]
    pub fn session(&self) -> &dyn InteractiveSessionHandle {
        self.session.as_ref()
    }
    /// Returns the prepared and active-session contribution.
    #[must_use]
    pub const fn contribution(&self) -> &ConsumerRouteProjectionContribution {
        &self.contribution
    }
    /// Returns model options retained and validated during open.
    #[must_use]
    pub fn negotiated_model_options(&self) -> Option<&NegotiatedSessionModelOptions> {
        self.session.negotiated_model_options()
    }
    /// Splits the opened session from its contribution.
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

/// Failure returned by the additive Gemini projected-open seam.
pub enum GeminiProjectionOpenFailure {
    /// The underlying route or projection failure.
    Runtime(RuntimeFailure),
}
impl GeminiProjectionOpenFailure {
    /// Returns the underlying route failure.
    #[must_use]
    pub const fn failure(&self) -> &RuntimeFailure {
        match self {
            Self::Runtime(failure) => failure,
        }
    }
}
/// Future returned by the additive Gemini projected-open seam.
pub type GeminiProjectionOpenFuture =
    BoxFuture<'static, Result<GeminiProjectionOpenOutcome, GeminiProjectionOpenFailure>>;
