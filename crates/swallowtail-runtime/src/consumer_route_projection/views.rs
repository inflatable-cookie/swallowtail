use super::applicability::ConsumerRouteApplicability;
use super::identity::ConsumerRouteProjectionSourceIdentity;
use super::row::ConsumerRouteProjectionRow;

macro_rules! projection_view {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct $name {
            rows: Vec<ConsumerRouteProjectionRow>,
        }

        impl $name {
            pub(super) const fn from_rows(rows: Vec<ConsumerRouteProjectionRow>) -> Self {
                Self { rows }
            }

            /// Iterates the exact admitted rows of this view.
            pub fn rows(&self) -> impl ExactSizeIterator<Item = &ConsumerRouteProjectionRow> {
                self.rows.iter()
            }
        }
    };
}

projection_view!(
    ConsumerRouteSelectionSummary,
    "Immutable selection-time feature and control summary for one exact route."
);
projection_view!(
    ConsumerRouteSessionStartControls,
    "Immutable session-start and per-turn controls the exact route admits."
);
projection_view!(
    ConsumerRouteActiveSessionState,
    "Immutable post-open observation and exact negotiated state."
);

/// Exact identity one immutable projection snapshot binds.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConsumerRouteProjectionIdentity {
    applicability: ConsumerRouteApplicability,
    sources: Vec<ConsumerRouteProjectionSourceIdentity>,
}

impl ConsumerRouteProjectionIdentity {
    pub(super) const fn from_parts(
        applicability: ConsumerRouteApplicability,
        sources: Vec<ConsumerRouteProjectionSourceIdentity>,
    ) -> Self {
        Self {
            applicability,
            sources,
        }
    }

    #[must_use]
    /// Returns the exact instance, route, model, and operation binding.
    pub const fn applicability(&self) -> &ConsumerRouteApplicability {
        &self.applicability
    }

    /// Iterates every source evidence identity used to assemble the snapshot.
    pub fn sources(&self) -> impl ExactSizeIterator<Item = &ConsumerRouteProjectionSourceIdentity> {
        self.sources.iter()
    }
}

/// One immutable Contract 061 route projection snapshot.
///
/// The snapshot is descriptive. Any change to a source record produces a
/// separately composed replacement rather than a mutation of this value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConsumerRouteProjection {
    identity: ConsumerRouteProjectionIdentity,
    selection_summary: ConsumerRouteSelectionSummary,
    session_start_controls: ConsumerRouteSessionStartControls,
    active_session_state: ConsumerRouteActiveSessionState,
}

impl ConsumerRouteProjection {
    pub(super) const fn from_parts(
        identity: ConsumerRouteProjectionIdentity,
        selection_summary: ConsumerRouteSelectionSummary,
        session_start_controls: ConsumerRouteSessionStartControls,
        active_session_state: ConsumerRouteActiveSessionState,
    ) -> Self {
        Self {
            identity,
            selection_summary,
            session_start_controls,
            active_session_state,
        }
    }

    #[must_use]
    /// Returns the exact snapshot identity.
    pub const fn identity(&self) -> &ConsumerRouteProjectionIdentity {
        &self.identity
    }

    #[must_use]
    /// Returns the immutable selection-summary view.
    pub const fn selection_summary(&self) -> &ConsumerRouteSelectionSummary {
        &self.selection_summary
    }

    #[must_use]
    /// Returns the immutable session-start control view.
    pub const fn session_start_controls(&self) -> &ConsumerRouteSessionStartControls {
        &self.session_start_controls
    }

    #[must_use]
    /// Returns the immutable active-session state view.
    pub const fn active_session_state(&self) -> &ConsumerRouteActiveSessionState {
        &self.active_session_state
    }

    /// Iterates every source evidence identity used to assemble the snapshot.
    pub fn sources(&self) -> impl ExactSizeIterator<Item = &ConsumerRouteProjectionSourceIdentity> {
        self.identity.sources()
    }
}
