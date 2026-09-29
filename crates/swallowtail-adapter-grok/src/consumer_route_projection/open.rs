use swallowtail_runtime::{
    BoxFuture, ConsumerRouteProjectionContribution, InteractiveSessionHandle,
    NegotiatedSessionModelOptions, RuntimeFailure,
};

/// Result of opening Grok ACP through the additive projection seam.
pub struct GrokProjectionOpenOutcome {
    session: Box<dyn InteractiveSessionHandle>,
    contribution: ConsumerRouteProjectionContribution,
}
impl GrokProjectionOpenOutcome {
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
    /// Returns the exact prepared and active-session contribution.
    #[must_use]
    pub const fn contribution(&self) -> &ConsumerRouteProjectionContribution {
        &self.contribution
    }
    /// Returns options retained and validated during the successful open.
    #[must_use]
    pub fn negotiated_model_options(&self) -> Option<&NegotiatedSessionModelOptions> {
        self.session.negotiated_model_options()
    }
    /// Splits session and contribution.
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
/// Failure returned by the additive Grok projected-open seam.
pub enum GrokProjectionOpenFailure {
    /// The underlying route or projection failure.
    Runtime(RuntimeFailure),
}
impl GrokProjectionOpenFailure {
    /// Returns the underlying route failure.
    #[must_use]
    pub const fn failure(&self) -> &RuntimeFailure {
        match self {
            Self::Runtime(failure) => failure,
        }
    }
}
/// Future returned by the additive Grok projected-open seam.
pub type GrokProjectionOpenFuture =
    BoxFuture<'static, Result<GrokProjectionOpenOutcome, GrokProjectionOpenFailure>>;
