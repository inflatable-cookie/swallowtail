/// Layer that reported or caused a portable failure.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FailureOrigin {
    Provider,
    Harness,
    Host,
    Transport,
    Protocol,
    Runtime,
    Unknown,
}

/// Coarse machine-supported meaning shared across provider routes.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FailureKind {
    AuthenticationRequired,
    AuthenticationRejected,
    AuthorizationDenied,
    EntitlementUnavailable,
    ModelUnavailable,
    RateLimited,
    QuotaExhausted,
    ProviderUnavailable,
    InvalidRequest,
    InputLimitExceeded,
    ContextLimitExceeded,
    ResourceNotFound,
    ResourceStale,
    HarnessUnavailable,
    HarnessIncompatible,
    TransportInterrupted,
    ProtocolIncompatible,
    MalformedData,
    HostServiceUnavailable,
    RuntimeInvariant,
    Unknown,
}

/// Bounded recovery evidence. Consumers retain retry and fallback policy.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FailureRecovery {
    RetryMaySucceed,
    ReauthenticationRequired,
    ConfigurationChangeRequired,
    InputChangeRequired,
    HarnessUpdateRequired,
    SameRequestNotRetryable,
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
    #[must_use]
    pub const fn new(origin: FailureOrigin, kind: FailureKind, recovery: FailureRecovery) -> Self {
        Self {
            origin,
            kind,
            recovery,
        }
    }

    #[must_use]
    pub const fn unknown() -> Self {
        Self::new(
            FailureOrigin::Unknown,
            FailureKind::Unknown,
            FailureRecovery::Unknown,
        )
    }

    #[must_use]
    pub const fn origin(self) -> FailureOrigin {
        self.origin
    }

    #[must_use]
    pub const fn kind(self) -> FailureKind {
        self.kind
    }

    #[must_use]
    pub const fn recovery(self) -> FailureRecovery {
        self.recovery
    }

    #[must_use]
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
