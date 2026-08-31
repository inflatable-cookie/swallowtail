use super::semantics::ConsumerRouteLifecycle;
use super::{
    MAX_CONSUMER_ROUTE_ACTIVE_SESSION_ROWS, MAX_CONSUMER_ROUTE_SELECTION_SUMMARY_ROWS,
    MAX_CONSUMER_ROUTE_SESSION_START_ROWS,
};

/// One of the three immutable projection views a row may be admitted to.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ConsumerRouteView {
    SelectionSummary,
    SessionStart,
    ActiveSession,
}

impl ConsumerRouteView {
    pub(super) const fn maximum_rows(self) -> usize {
        match self {
            Self::SelectionSummary => MAX_CONSUMER_ROUTE_SELECTION_SUMMARY_ROWS,
            Self::SessionStart => MAX_CONSUMER_ROUTE_SESSION_START_ROWS,
            Self::ActiveSession => MAX_CONSUMER_ROUTE_ACTIVE_SESSION_ROWS,
        }
    }

    pub(super) const fn admits(self, lifecycle: ConsumerRouteLifecycle) -> bool {
        match self {
            Self::SelectionSummary => matches!(lifecycle, ConsumerRouteLifecycle::SelectionSummary),
            Self::SessionStart => matches!(
                lifecycle,
                ConsumerRouteLifecycle::SessionStartOnly | ConsumerRouteLifecycle::PerTurn
            ),
            Self::ActiveSession => matches!(
                lifecycle,
                ConsumerRouteLifecycle::BetweenTurnNegotiable
                    | ConsumerRouteLifecycle::QualifiedMidTurnNegotiable
                    | ConsumerRouteLifecycle::PostOpenObservationOnly
            ),
        }
    }

    pub(super) const fn limit_code(self) -> (&'static str, &'static str) {
        match self {
            Self::SelectionSummary => (
                "swallowtail.consumer_route_projection.selection_summary_limit_exceeded",
                "Projected selection summary exceeds the fixed row maximum",
            ),
            Self::SessionStart => (
                "swallowtail.consumer_route_projection.session_start_limit_exceeded",
                "Projected session-start controls exceed the fixed row maximum",
            ),
            Self::ActiveSession => (
                "swallowtail.consumer_route_projection.active_session_limit_exceeded",
                "Projected active-session state exceeds the fixed row maximum",
            ),
        }
    }
}
