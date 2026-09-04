use super::MAX_CONSUMER_ROUTE_NAMESPACED_EXTENSIONS;
use super::admission::{admit_sources, admit_view};
use super::applicability::ConsumerRouteApplicability;
use super::failure::{ConsumerRouteProjectionFailure, ConsumerRouteProjectionFailureKind, failure};
use super::identity::{ConsumerRouteProjectionSourceIdentity, ConsumerRouteProjectionSourceKind};
use super::row::ConsumerRouteProjectionRow;
use super::view::ConsumerRouteView;

/// Immutable adapter-owned contribution admitted before composition.
///
/// A contribution publishes only what its exact prepared operation proves. It
/// carries no provider payload, executable handle, or mutation authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConsumerRouteProjectionContribution {
    applicability: ConsumerRouteApplicability,
    sources: Vec<ConsumerRouteProjectionSourceIdentity>,
    selection_rows: Vec<ConsumerRouteProjectionRow>,
    session_start_rows: Vec<ConsumerRouteProjectionRow>,
    active_session_rows: Vec<ConsumerRouteProjectionRow>,
}

impl ConsumerRouteProjectionContribution {
    /// Admits one exact contribution or rejects it before composition.
    pub fn new(
        applicability: ConsumerRouteApplicability,
        sources: impl IntoIterator<Item = ConsumerRouteProjectionSourceIdentity>,
        selection_rows: impl IntoIterator<Item = ConsumerRouteProjectionRow>,
        session_start_rows: impl IntoIterator<Item = ConsumerRouteProjectionRow>,
        active_session_rows: impl IntoIterator<Item = ConsumerRouteProjectionRow>,
    ) -> Result<Self, ConsumerRouteProjectionFailure> {
        let sources = sources.into_iter().collect::<Vec<_>>();
        let selection_rows = selection_rows.into_iter().collect::<Vec<_>>();
        let session_start_rows = session_start_rows.into_iter().collect::<Vec<_>>();
        let active_session_rows = active_session_rows.into_iter().collect::<Vec<_>>();
        if sources.iter().any(|source| {
            source.kind() == ConsumerRouteProjectionSourceKind::ProviderOperationObservation
        }) {
            return Err(failure(
                ConsumerRouteProjectionFailureKind::ProviderOperationObservationInvalid,
                "swallowtail.consumer_route_projection.provider_operation_source_rejected",
                "Prepared contribution cannot publish provider-operation observation",
            ));
        }
        let source_identities = admit_sources(&sources)?;
        admit_view(
            ConsumerRouteView::SelectionSummary,
            &selection_rows,
            &applicability,
            &source_identities,
        )?;
        admit_view(
            ConsumerRouteView::SessionStart,
            &session_start_rows,
            &applicability,
            &source_identities,
        )?;
        admit_view(
            ConsumerRouteView::ActiveSession,
            &active_session_rows,
            &applicability,
            &source_identities,
        )?;
        admit_extension_budget(
            selection_rows
                .iter()
                .chain(&session_start_rows)
                .chain(&active_session_rows),
            "Contribution exceeds the fixed namespaced-extension maximum",
        )?;
        Ok(Self {
            applicability,
            sources,
            selection_rows,
            session_start_rows,
            active_session_rows,
        })
    }

    #[must_use]
    /// Returns the exact applicability every contributed row is bound to.
    pub const fn applicability(&self) -> &ConsumerRouteApplicability {
        &self.applicability
    }

    /// Iterates the source identities this contribution names.
    pub fn sources(&self) -> impl ExactSizeIterator<Item = &ConsumerRouteProjectionSourceIdentity> {
        self.sources.iter()
    }

    /// Iterates contributed selection-summary rows.
    pub fn selection_rows(&self) -> impl ExactSizeIterator<Item = &ConsumerRouteProjectionRow> {
        self.selection_rows.iter()
    }

    /// Iterates contributed session-start control rows.
    pub fn session_start_rows(&self) -> impl ExactSizeIterator<Item = &ConsumerRouteProjectionRow> {
        self.session_start_rows.iter()
    }

    /// Iterates contributed active-session rows.
    pub fn active_session_rows(
        &self,
    ) -> impl ExactSizeIterator<Item = &ConsumerRouteProjectionRow> {
        self.active_session_rows.iter()
    }
}

/// Rejects more bounded namespaced extensions than the fixed maximum admits.
pub(super) fn admit_extension_budget<'a>(
    rows: impl Iterator<Item = &'a ConsumerRouteProjectionRow>,
    message: &'static str,
) -> Result<(), ConsumerRouteProjectionFailure> {
    let extensions = rows
        .filter(|row| row.identity().namespaced_extension().is_some())
        .count();
    if extensions > MAX_CONSUMER_ROUTE_NAMESPACED_EXTENSIONS {
        return Err(failure(
            ConsumerRouteProjectionFailureKind::LimitExceeded,
            "swallowtail.consumer_route_projection.namespaced_extension_limit_exceeded",
            message,
        ));
    }
    Ok(())
}
