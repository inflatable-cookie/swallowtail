use std::fmt;

use super::MAX_CONSUMER_ROUTE_SOURCE_ID_BYTES;
use super::failure::{ConsumerRouteProjectionFailure, ConsumerRouteProjectionFailureKind};
use super::text::admit_text;

/// Class of independently replaceable evidence one projection source names.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ConsumerRouteProjectionSourceKind {
    /// The immutable configured provider-instance record.
    ConfiguredInstance,
    /// The immutable prepared-operation record.
    PreparedOperation,
    /// One adapter-owned prepared contribution.
    AdapterContribution,
    /// One exact post-open active-session observation.
    ActiveSessionObservation,
}

/// Bounded identity of one independently replaceable projection source.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ConsumerRouteProjectionSourceId(String);

impl ConsumerRouteProjectionSourceId {
    /// Admits bounded, non-blank, control-free source-identity text.
    pub fn new(value: impl Into<String>) -> Result<Self, ConsumerRouteProjectionFailure> {
        let value = value.into();
        admit_text(
            &value,
            MAX_CONSUMER_ROUTE_SOURCE_ID_BYTES,
            ConsumerRouteProjectionFailureKind::LimitExceeded,
            "swallowtail.consumer_route_projection.source_id_limit_exceeded",
            "Projection source id exceeds the fixed source-identity byte maximum",
        )?;
        Ok(Self(value))
    }

    #[must_use]
    /// Returns the exact admitted source-identity text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ConsumerRouteProjectionSourceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Exact identity of one evidence record used to assemble a projection.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ConsumerRouteProjectionSourceIdentity {
    id: ConsumerRouteProjectionSourceId,
    kind: ConsumerRouteProjectionSourceKind,
}

impl ConsumerRouteProjectionSourceIdentity {
    #[must_use]
    /// Binds one admitted source id to the evidence class that supplied it.
    pub const fn new(
        id: ConsumerRouteProjectionSourceId,
        kind: ConsumerRouteProjectionSourceKind,
    ) -> Self {
        Self { id, kind }
    }

    #[must_use]
    /// Returns the bounded source id.
    pub const fn id(&self) -> &ConsumerRouteProjectionSourceId {
        &self.id
    }

    #[must_use]
    /// Returns the evidence class this source belongs to.
    pub const fn kind(&self) -> ConsumerRouteProjectionSourceKind {
        self.kind
    }
}
