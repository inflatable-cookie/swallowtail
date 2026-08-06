#![deny(missing_docs)]

/// Layer that reported or caused a portable failure.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FailureOrigin {
    /// Provider API or model service reported the failure.
    Provider,
    /// Installed agent harness reported the failure.
    Harness,
    /// Consumer-supplied host service reported the failure.
    Host,
    /// Transport connection or framing reported the failure.
    Transport,
    /// Protocol decoding or compatibility reported the failure.
    Protocol,
    /// Swallowtail runtime invariant or lifecycle reported the failure.
    Runtime,
    /// Exact origin is unavailable.
    Unknown,
}

/// Coarse machine-supported meaning shared across provider routes.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FailureKind {
    /// Credentials must be configured or supplied.
    AuthenticationRequired,
    /// Supplied credentials were rejected.
    AuthenticationRejected,
    /// Identity is valid but lacks required authority.
    AuthorizationDenied,
    /// Required subscription, billing, or account entitlement is unavailable.
    EntitlementUnavailable,
    /// Selected model is unavailable through the route.
    ModelUnavailable,
    /// Provider or harness imposed a request-rate limit.
    RateLimited,
    /// Available quota or credits are exhausted.
    QuotaExhausted,
    /// Provider service is currently unavailable.
    ProviderUnavailable,
    /// Request is invalid for the selected route.
    InvalidRequest,
    /// Input exceeds a route-specific size or item bound.
    InputLimitExceeded,
    /// Combined input exceeds the model context limit.
    ContextLimitExceeded,
    /// Requested provider or host resource does not exist.
    ResourceNotFound,
    /// Retained reference no longer matches current resource state.
    ResourceStale,
    /// Required installed harness is unavailable.
    HarnessUnavailable,
    /// Installed harness version or behavior is incompatible.
    HarnessIncompatible,
    /// Transport ended before a trustworthy terminal outcome.
    TransportInterrupted,
    /// Protocol version or behavior is incompatible.
    ProtocolIncompatible,
    /// Provider or harness data could not be safely decoded.
    MalformedData,
    /// Required consumer host service is unavailable.
    HostServiceUnavailable,
    /// Runtime encountered an internal contract violation.
    RuntimeInvariant,
    /// No common failure kind can be established safely.
    Unknown,
}

/// Bounded recovery evidence. Consumers retain retry and fallback policy.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FailureRecovery {
    /// Repeating the request later may succeed without another change.
    RetryMaySucceed,
    /// Credentials or login state must be refreshed first.
    ReauthenticationRequired,
    /// Route, account, host, or provider configuration must change first.
    ConfigurationChangeRequired,
    /// Request input must change before another attempt.
    InputChangeRequired,
    /// Installed harness must be updated before another attempt.
    HarnessUpdateRequired,
    /// Repeating the same request is not a supported recovery.
    SameRequestNotRetryable,
    /// No portable recovery guidance can be established safely.
    Unknown,
}

/// Provider-neutral classification carried alongside an exact safe diagnostic.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FailureClassification {
    origin: FailureOrigin,
    kind: FailureKind,
    recovery: FailureRecovery,
}

impl FailureClassification {
    /// Creates a classification from independent origin, kind, and recovery axes.
    #[must_use]
    pub const fn new(origin: FailureOrigin, kind: FailureKind, recovery: FailureRecovery) -> Self {
        Self {
            origin,
            kind,
            recovery,
        }
    }

    #[must_use]
    /// Returns the explicit all-unknown fallback classification.
    pub const fn unknown() -> Self {
        Self::new(
            FailureOrigin::Unknown,
            FailureKind::Unknown,
            FailureRecovery::Unknown,
        )
    }

    #[must_use]
    /// Returns the layer that reported or caused the failure.
    pub const fn origin(self) -> FailureOrigin {
        self.origin
    }

    #[must_use]
    /// Returns the coarse portable failure meaning.
    pub const fn kind(self) -> FailureKind {
        self.kind
    }

    #[must_use]
    /// Returns bounded recovery evidence without prescribing consumer policy.
    pub const fn recovery(self) -> FailureRecovery {
        self.recovery
    }

    #[must_use]
    /// Reports whether all three classification axes remain unknown.
    pub const fn is_unknown(self) -> bool {
        matches!(self.origin, FailureOrigin::Unknown)
            && matches!(self.kind, FailureKind::Unknown)
            && matches!(self.recovery, FailureRecovery::Unknown)
    }
}

impl Default for FailureClassification {
    fn default() -> Self {
        Self::unknown()
    }
}

#[cfg(test)]
mod tests {
    use super::{FailureClassification, FailureKind, FailureOrigin, FailureRecovery};

    #[test]
    fn unknown_classification_is_explicit() {
        let classification = FailureClassification::unknown();

        assert!(classification.is_unknown());
        assert_eq!(classification.origin(), FailureOrigin::Unknown);
        assert_eq!(classification.kind(), FailureKind::Unknown);
        assert_eq!(classification.recovery(), FailureRecovery::Unknown);
    }

    #[test]
    fn classification_axes_remain_independent() {
        let classification = FailureClassification::new(
            FailureOrigin::Provider,
            FailureKind::RateLimited,
            FailureRecovery::RetryMaySucceed,
        );

        assert!(!classification.is_unknown());
        assert_eq!(classification.origin(), FailureOrigin::Provider);
        assert_eq!(classification.kind(), FailureKind::RateLimited);
        assert_eq!(classification.recovery(), FailureRecovery::RetryMaySucceed);
    }
}
