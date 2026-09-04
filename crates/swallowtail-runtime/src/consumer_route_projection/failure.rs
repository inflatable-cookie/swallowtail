use std::error::Error;
use std::fmt;
use swallowtail_core::SafeDiagnostic;

/// Stable reason a Contract 061 route projection rejected its input.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ConsumerRouteProjectionFailureKind {
    /// Identity text is blank, control-bearing, over-long, or not supplied.
    IdentityInvalid,
    /// A bounded collection or text value exceeds its fixed projection maximum.
    LimitExceeded,
    /// Two admitted source identities carry the same source id.
    DuplicateSource,
    /// One projection view repeats the same semantic row identity.
    DuplicateRow,
    /// A row is not applicable to its contribution binding or its view.
    ApplicabilityDisagreement,
    /// Configured record, prepared evidence, and contributions disagree.
    SnapshotIdentityDisagreement,
    /// A row claims selectable, effective, or acknowledged posture without authority.
    MutationAuthorityAbsent,
    /// A value kind, admitted domain, or omission claim is not admissible.
    ValueDomainInvalid,
    /// A bounded safe reason exceeds its fixed byte maximum.
    SafeReasonLimitExceeded,
    /// Provider-operation source, shape, outcome, or row semantics are invalid.
    ProviderOperationObservationInvalid,
}

/// Safe failure returned while admitting or composing a route projection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConsumerRouteProjectionFailure {
    kind: ConsumerRouteProjectionFailureKind,
    diagnostic: SafeDiagnostic,
}

impl ConsumerRouteProjectionFailure {
    #[must_use]
    /// Returns the stable failure classification.
    pub const fn kind(&self) -> ConsumerRouteProjectionFailureKind {
        self.kind
    }

    #[must_use]
    /// Returns the bounded, redacted diagnostic.
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}

impl fmt::Display for ConsumerRouteProjectionFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for ConsumerRouteProjectionFailure {}

pub(super) fn failure(
    kind: ConsumerRouteProjectionFailureKind,
    code: &'static str,
    message: &'static str,
) -> ConsumerRouteProjectionFailure {
    ConsumerRouteProjectionFailure {
        kind,
        diagnostic: SafeDiagnostic::new(code, message),
    }
}
