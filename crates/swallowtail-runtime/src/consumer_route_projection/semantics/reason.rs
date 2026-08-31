use swallowtail_core::SafeDiagnostic;

use super::super::MAX_CONSUMER_ROUTE_SAFE_REASON_BYTES;
use super::super::failure::{ConsumerRouteProjectionFailure, ConsumerRouteProjectionFailureKind};
use super::super::identity::ConsumerRouteProjectionSourceId;
use super::super::text::admit_text;
use super::posture::ConsumerRouteAvailabilityDimension;

/// Bounded safe reason copied from the source dimension that supplied it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConsumerRouteSafeReason {
    dimension: ConsumerRouteAvailabilityDimension,
    source: ConsumerRouteProjectionSourceId,
    diagnostic: SafeDiagnostic,
}

impl ConsumerRouteSafeReason {
    /// Admits one bounded safe reason supplied by its named source.
    pub fn new(
        dimension: ConsumerRouteAvailabilityDimension,
        source: ConsumerRouteProjectionSourceId,
        diagnostic: SafeDiagnostic,
    ) -> Result<Self, ConsumerRouteProjectionFailure> {
        admit_text(
            diagnostic.message(),
            MAX_CONSUMER_ROUTE_SAFE_REASON_BYTES,
            ConsumerRouteProjectionFailureKind::SafeReasonLimitExceeded,
            "swallowtail.consumer_route_projection.safe_reason_limit_exceeded",
            "Projected safe reason exceeds the fixed safe-reason byte maximum",
        )?;
        Ok(Self {
            dimension,
            source,
            diagnostic,
        })
    }

    #[must_use]
    /// Returns the authoritative dimension the reason belongs to.
    pub const fn dimension(&self) -> ConsumerRouteAvailabilityDimension {
        self.dimension
    }

    #[must_use]
    /// Returns the source that supplied the reason.
    pub const fn source(&self) -> &ConsumerRouteProjectionSourceId {
        &self.source
    }

    #[must_use]
    /// Returns the bounded safe diagnostic.
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}
