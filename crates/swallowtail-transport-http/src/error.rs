use std::fmt;

/// Provider-neutral failure from one bounded HTTP exchange.
///
/// Adapters translate these variants into their own provider-specific
/// diagnostics, so no provider identity leaks into the transport vocabulary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransportError {
    /// The host exposed no blocking-work service for the exchange.
    BlockingServiceMissing,
    /// The blocking worker finished without returning a result.
    BlockingResultMissing,
    /// The endpoint was not an eligible HTTP endpoint for this request.
    EndpointInvalid,
    /// The credential could not form a safe secret header.
    CredentialInvalid,
    /// The bounded response or event buffer limit was exceeded.
    ResponseLimit,
    /// The operation was cancelled, or its consumer was dropped.
    Cancelled,
    /// The underlying HTTP transport failed for a redacted reason.
    Transport,
}

impl TransportError {
    /// Returns a stable provider-neutral explanation for the failure.
    #[must_use]
    pub const fn description(self) -> &'static str {
        match self {
            Self::BlockingServiceMissing => "HTTP exchange requires a blocking-work service",
            Self::BlockingResultMissing => "blocking HTTP work returned no result",
            Self::EndpointInvalid => "endpoint is not an eligible HTTP endpoint",
            Self::CredentialInvalid => "credential cannot form a safe secret header",
            Self::ResponseLimit => "response exceeded its bounded input limit",
            Self::Cancelled => "HTTP exchange was cancelled",
            Self::Transport => "HTTP transport failed",
        }
    }

    /// Reports whether the failure means the consumer abandoned the exchange.
    #[must_use]
    pub const fn is_cancelled(self) -> bool {
        matches!(self, Self::Cancelled)
    }
}

impl fmt::Display for TransportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.description())
    }
}

impl std::error::Error for TransportError {}
