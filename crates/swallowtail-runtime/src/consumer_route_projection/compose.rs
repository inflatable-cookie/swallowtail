use std::collections::BTreeSet;

use super::admission::{admit_sources, admit_view};
use super::agreement::{require_record_agreement, snapshot_disagreement};
use super::applicability::ConsumerRouteApplicability;
use super::contribution::{ConsumerRouteProjectionContribution, admit_extension_budget};
use super::failure::{ConsumerRouteProjectionFailure, ConsumerRouteProjectionFailureKind, failure};
use super::identity::{ConsumerRouteProjectionSourceIdentity, ConsumerRouteProjectionSourceKind};
use super::provider_operation_observation::ConsumerRouteProviderOperationObservation;
use super::row::ConsumerRouteProjectionRow;
use super::view::ConsumerRouteView;
use super::views::{
    ConsumerRouteActiveSessionState, ConsumerRouteProjection, ConsumerRouteProjectionIdentity,
    ConsumerRouteProviderOperationState, ConsumerRouteSelectionSummary,
    ConsumerRouteSessionStartControls,
};
use crate::{ConfiguredProviderInstanceRecord, PreparedOperationEvidence};

/// Exact borrowed evidence one route projection is composed from.
///
/// The input borrows the authoritative records, owns their supplied source
/// identities, and borrows only the contributions the consumer linked.
#[derive(Clone, Debug)]
pub struct ConsumerRouteProjectionInput<'a> {
    record: &'a ConfiguredProviderInstanceRecord,
    record_source: ConsumerRouteProjectionSourceIdentity,
    evidence: &'a PreparedOperationEvidence,
    evidence_source: ConsumerRouteProjectionSourceIdentity,
    contributions: Vec<&'a ConsumerRouteProjectionContribution>,
    provider_operation_observations: Vec<&'a ConsumerRouteProviderOperationObservation>,
}

impl<'a> ConsumerRouteProjectionInput<'a> {
    #[must_use]
    /// Binds the exact configured record and prepared evidence to compose.
    pub const fn new(
        record: &'a ConfiguredProviderInstanceRecord,
        record_source: ConsumerRouteProjectionSourceIdentity,
        evidence: &'a PreparedOperationEvidence,
        evidence_source: ConsumerRouteProjectionSourceIdentity,
    ) -> Self {
        Self {
            record,
            record_source,
            evidence,
            evidence_source,
            contributions: Vec::new(),
            provider_operation_observations: Vec::new(),
        }
    }

    #[must_use]
    /// Adds the exact adapter contributions the consumer supplied.
    pub fn with_contributions(
        mut self,
        contributions: impl IntoIterator<Item = &'a ConsumerRouteProjectionContribution>,
    ) -> Self {
        self.contributions = contributions.into_iter().collect();
        self
    }

    #[must_use]
    /// Adds completed provider-operation observations to the fourth view.
    pub fn with_provider_operation_observations(
        mut self,
        observations: impl IntoIterator<Item = &'a ConsumerRouteProviderOperationObservation>,
    ) -> Self {
        self.provider_operation_observations = observations.into_iter().collect();
        self
    }

    #[must_use]
    /// Returns the borrowed configured provider-instance record.
    pub const fn record(&self) -> &'a ConfiguredProviderInstanceRecord {
        self.record
    }

    #[must_use]
    /// Returns the borrowed prepared-operation record.
    pub const fn evidence(&self) -> &'a PreparedOperationEvidence {
        self.evidence
    }

    /// Iterates the borrowed adapter contributions.
    pub fn contributions(
        &self,
    ) -> impl ExactSizeIterator<Item = &&'a ConsumerRouteProjectionContribution> {
        self.contributions.iter()
    }
}

/// Composes one immutable Contract 061 route projection or fails closed.
///
/// The composer is pure. It receives no prior projection, mutates nothing, and
/// creates no request, watcher, default, or provider effect.
pub fn compose_consumer_route_projection(
    input: ConsumerRouteProjectionInput<'_>,
) -> Result<ConsumerRouteProjection, ConsumerRouteProjectionFailure> {
    require_source_kind(
        &input.record_source,
        ConsumerRouteProjectionSourceKind::ConfiguredInstance,
    )?;
    require_source_kind(
        &input.evidence_source,
        ConsumerRouteProjectionSourceKind::PreparedOperation,
    )?;
    require_record_agreement(input.record, input.evidence)?;
    let applicability = ConsumerRouteApplicability::from_prepared_operation(input.evidence);
    let mut sources = vec![input.record_source.clone(), input.evidence_source.clone()];
    for contribution in &input.contributions {
        if contribution.applicability() != &applicability {
            return Err(snapshot_disagreement());
        }
        sources.extend(contribution.sources().cloned());
    }
    for observation in &input.provider_operation_observations {
        if observation.applicability() != &applicability {
            return Err(snapshot_disagreement());
        }
        sources.push(observation.source().clone());
    }
    let source_identities = admit_sources(&sources)?;
    let selection_rows = merge(
        ConsumerRouteView::SelectionSummary,
        &input,
        ConsumerRouteProjectionContribution::selection_rows,
        &applicability,
        &source_identities,
    )?;
    let session_start_rows = merge(
        ConsumerRouteView::SessionStart,
        &input,
        ConsumerRouteProjectionContribution::session_start_rows,
        &applicability,
        &source_identities,
    )?;
    let active_session_rows = merge(
        ConsumerRouteView::ActiveSession,
        &input,
        ConsumerRouteProjectionContribution::active_session_rows,
        &applicability,
        &source_identities,
    )?;
    let provider_operation_rows = input
        .provider_operation_observations
        .iter()
        .flat_map(|observation| observation.rows())
        .cloned()
        .collect::<Vec<_>>();
    admit_view(
        ConsumerRouteView::ProviderOperation,
        &provider_operation_rows,
        &applicability,
        &source_identities,
    )?;
    admit_extension_budget(
        selection_rows
            .iter()
            .chain(&session_start_rows)
            .chain(&active_session_rows)
            .chain(&provider_operation_rows),
        "Projection exceeds the fixed namespaced-extension maximum",
    )?;
    Ok(ConsumerRouteProjection::from_parts(
        ConsumerRouteProjectionIdentity::from_parts(applicability, sources),
        ConsumerRouteSelectionSummary::from_rows(selection_rows),
        ConsumerRouteSessionStartControls::from_rows(session_start_rows),
        ConsumerRouteActiveSessionState::from_rows(active_session_rows),
        ConsumerRouteProviderOperationState::from_rows(provider_operation_rows),
    ))
}

fn merge<'a, I>(
    view: ConsumerRouteView,
    input: &'a ConsumerRouteProjectionInput<'a>,
    rows_of: fn(&'a ConsumerRouteProjectionContribution) -> I,
    applicability: &ConsumerRouteApplicability,
    sources: &BTreeSet<ConsumerRouteProjectionSourceIdentity>,
) -> Result<Vec<ConsumerRouteProjectionRow>, ConsumerRouteProjectionFailure>
where
    I: Iterator<Item = &'a ConsumerRouteProjectionRow>,
{
    let rows = input
        .contributions
        .iter()
        .flat_map(|contribution| rows_of(contribution))
        .cloned()
        .collect::<Vec<_>>();
    admit_view(view, &rows, applicability, sources)?;
    Ok(rows)
}

fn require_source_kind(
    source: &ConsumerRouteProjectionSourceIdentity,
    expected: ConsumerRouteProjectionSourceKind,
) -> Result<(), ConsumerRouteProjectionFailure> {
    if source.kind() == expected {
        Ok(())
    } else {
        Err(failure(
            ConsumerRouteProjectionFailureKind::IdentityInvalid,
            "swallowtail.consumer_route_projection.source_kind_rejected",
            "A composed source identity does not belong to its evidence class",
        ))
    }
}
