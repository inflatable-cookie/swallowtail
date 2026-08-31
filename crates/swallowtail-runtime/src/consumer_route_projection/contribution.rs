use std::collections::BTreeSet;

use super::applicability::ConsumerRouteApplicability;
use super::failure::{ConsumerRouteProjectionFailure, ConsumerRouteProjectionFailureKind, failure};
use super::identity::{ConsumerRouteProjectionSourceId, ConsumerRouteProjectionSourceIdentity};
use super::row::{ConsumerRouteProjectionRow, ConsumerRouteValueDomain};
use super::semantics::{
    ConsumerRouteActorPosture, ConsumerRouteLifecycle, ConsumerRouteMutationAuthority,
    ConsumerRouteRowIdentity,
};
use super::{
    MAX_CONSUMER_ROUTE_ACTIVE_SESSION_ROWS, MAX_CONSUMER_ROUTE_NAMESPACED_EXTENSIONS,
    MAX_CONSUMER_ROUTE_SELECTION_SUMMARY_ROWS, MAX_CONSUMER_ROUTE_SESSION_START_ROWS,
    MAX_CONSUMER_ROUTE_SOURCE_IDENTITIES,
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
        let source_ids = admit_sources(&sources)?;
        admit_view(
            ConsumerRouteView::SelectionSummary,
            &selection_rows,
            &applicability,
            &source_ids,
        )?;
        admit_view(
            ConsumerRouteView::SessionStart,
            &session_start_rows,
            &applicability,
            &source_ids,
        )?;
        admit_view(
            ConsumerRouteView::ActiveSession,
            &active_session_rows,
            &applicability,
            &source_ids,
        )?;
        let extensions = selection_rows
            .iter()
            .chain(&session_start_rows)
            .chain(&active_session_rows)
            .filter(|row| row.identity().namespaced_extension().is_some())
            .count();
        if extensions > MAX_CONSUMER_ROUTE_NAMESPACED_EXTENSIONS {
            return Err(failure(
                ConsumerRouteProjectionFailureKind::LimitExceeded,
                "swallowtail.consumer_route_projection.namespaced_extension_limit_exceeded",
                "Contribution exceeds the fixed namespaced-extension maximum",
            ));
        }
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

pub(super) fn admit_sources(
    sources: &[ConsumerRouteProjectionSourceIdentity],
) -> Result<BTreeSet<ConsumerRouteProjectionSourceId>, ConsumerRouteProjectionFailure> {
    if sources.is_empty() {
        return Err(failure(
            ConsumerRouteProjectionFailureKind::IdentityInvalid,
            "swallowtail.consumer_route_projection.source_identity_missing",
            "A projection contribution must name at least one source identity",
        ));
    }
    if sources.len() > MAX_CONSUMER_ROUTE_SOURCE_IDENTITIES {
        return Err(failure(
            ConsumerRouteProjectionFailureKind::LimitExceeded,
            "swallowtail.consumer_route_projection.source_identity_limit_exceeded",
            "Projection exceeds the fixed source-identity maximum",
        ));
    }
    let mut ids = BTreeSet::new();
    for source in sources {
        if !ids.insert(source.id().clone()) {
            return Err(failure(
                ConsumerRouteProjectionFailureKind::DuplicateSource,
                "swallowtail.consumer_route_projection.source_identity_duplicate",
                "Projection repeats one source identity",
            ));
        }
    }
    Ok(ids)
}

pub(super) fn admit_view(
    view: ConsumerRouteView,
    rows: &[ConsumerRouteProjectionRow],
    applicability: &ConsumerRouteApplicability,
    sources: &BTreeSet<ConsumerRouteProjectionSourceId>,
) -> Result<(), ConsumerRouteProjectionFailure> {
    if rows.len() > view.maximum_rows() {
        let (code, message) = view.limit_code();
        return Err(failure(
            ConsumerRouteProjectionFailureKind::LimitExceeded,
            code,
            message,
        ));
    }
    let mut identities = BTreeSet::new();
    for row in rows {
        if !identities.insert(row.identity().clone()) {
            return Err(failure(
                ConsumerRouteProjectionFailureKind::DuplicateRow,
                "swallowtail.consumer_route_projection.row_duplicate",
                "One projection view repeats a semantic row identity",
            ));
        }
        admit_row(view, row, applicability, sources)?;
    }
    Ok(())
}

fn admit_row(
    view: ConsumerRouteView,
    row: &ConsumerRouteProjectionRow,
    applicability: &ConsumerRouteApplicability,
    sources: &BTreeSet<ConsumerRouteProjectionSourceId>,
) -> Result<(), ConsumerRouteProjectionFailure> {
    if !view.admits(row.lifecycle()) || row.applicability() != applicability {
        return Err(failure(
            ConsumerRouteProjectionFailureKind::ApplicabilityDisagreement,
            "swallowtail.consumer_route_projection.row_applicability_rejected",
            "A projected row is not applicable to its exact binding or view",
        ));
    }
    if !sources.contains(row.source().id()) {
        return Err(failure(
            ConsumerRouteProjectionFailureKind::IdentityInvalid,
            "swallowtail.consumer_route_projection.row_source_unknown",
            "A projected row names a source the contribution did not supply",
        ));
    }
    admit_value(row)?;
    admit_reason(row, sources)?;
    admit_authority(row, sources)
}

fn admit_value(row: &ConsumerRouteProjectionRow) -> Result<(), ConsumerRouteProjectionFailure> {
    let is_control = matches!(row.identity(), ConsumerRouteRowIdentity::Control(_));
    match (is_control, row.control_value()) {
        (true, Some(value)) => {
            if matches!(value.domain(), ConsumerRouteValueDomain::Descriptor)
                && !matches!(
                    value.omission(),
                    super::row::ConsumerRouteOmissionSemantics::NotSelectable
                )
            {
                return Err(failure(
                    ConsumerRouteProjectionFailureKind::ValueDomainInvalid,
                    "swallowtail.consumer_route_projection.value_domain_rejected",
                    "A descriptor-only control domain cannot carry selectable omission truth",
                ));
            }
            Ok(())
        }
        (false, None) => Ok(()),
        _ => Err(failure(
            ConsumerRouteProjectionFailureKind::ValueDomainInvalid,
            "swallowtail.consumer_route_projection.value_domain_rejected",
            "Only a control row carries a value kind, domain, and omission truth",
        )),
    }
}

fn admit_reason(
    row: &ConsumerRouteProjectionRow,
    sources: &BTreeSet<ConsumerRouteProjectionSourceId>,
) -> Result<(), ConsumerRouteProjectionFailure> {
    let Some(reason) = row.safe_reason() else {
        return Ok(());
    };
    if sources.contains(reason.source()) {
        Ok(())
    } else {
        Err(failure(
            ConsumerRouteProjectionFailureKind::IdentityInvalid,
            "swallowtail.consumer_route_projection.safe_reason_source_unknown",
            "A bounded safe reason names a source the contribution did not supply",
        ))
    }
}

fn admit_authority(
    row: &ConsumerRouteProjectionRow,
    sources: &BTreeSet<ConsumerRouteProjectionSourceId>,
) -> Result<(), ConsumerRouteProjectionFailure> {
    if let Some(source) = row.mutation_authority().source()
        && !sources.contains(source)
    {
        return Err(failure(
            ConsumerRouteProjectionFailureKind::IdentityInvalid,
            "swallowtail.consumer_route_projection.mutation_authority_source_unknown",
            "A mutation authority names a source the contribution did not supply",
        ));
    }
    let state = row.state_support();
    let post_session_start = matches!(
        row.lifecycle(),
        ConsumerRouteLifecycle::PerTurn | ConsumerRouteLifecycle::PostOpenObservationOnly
    );
    let acknowledged_claim = state.provider_effective() || state.rejected();
    let selectable_after_open = matches!(
        row.lifecycle(),
        ConsumerRouteLifecycle::PostOpenObservationOnly
    ) && matches!(
        row.actor_posture(),
        ConsumerRouteActorPosture::ConsumerSelectable
    );
    if (post_session_start && state.prepared())
        || (acknowledged_claim && !row.mutation_authority().is_acknowledged())
        || (selectable_after_open && !row.mutation_authority().is_acknowledged())
    {
        return Err(failure(
            ConsumerRouteProjectionFailureKind::MutationAuthorityAbsent,
            "swallowtail.consumer_route_projection.mutation_authority_absent",
            "A projected row claims selectable or acknowledged posture without exact authority",
        ));
    }
    if matches!(
        row.mutation_authority(),
        ConsumerRouteMutationAuthority::Absent
    ) && matches!(
        row.actor_posture(),
        ConsumerRouteActorPosture::ConsumerSelectable
    ) {
        return Err(failure(
            ConsumerRouteProjectionFailureKind::MutationAuthorityAbsent,
            "swallowtail.consumer_route_projection.mutation_authority_absent",
            "A consumer-selectable row requires an exact prepared or acknowledged authority",
        ));
    }
    Ok(())
}
