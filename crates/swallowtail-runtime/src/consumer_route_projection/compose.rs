use std::collections::BTreeSet;

use super::MAX_CONSUMER_ROUTE_NAMESPACED_EXTENSIONS;
use super::applicability::ConsumerRouteApplicability;
use super::contribution::{
    ConsumerRouteProjectionContribution, ConsumerRouteView, admit_sources, admit_view,
};
use super::failure::{ConsumerRouteProjectionFailure, ConsumerRouteProjectionFailureKind, failure};
use super::identity::{ConsumerRouteProjectionSourceIdentity, ConsumerRouteProjectionSourceKind};
use super::row::ConsumerRouteProjectionRow;
use super::views::{
    ConsumerRouteActiveSessionState, ConsumerRouteProjection, ConsumerRouteProjectionIdentity,
    ConsumerRouteSelectionSummary, ConsumerRouteSessionStartControls,
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
            return Err(failure(
                ConsumerRouteProjectionFailureKind::SnapshotIdentityDisagreement,
                "swallowtail.consumer_route_projection.contribution_binding_rejected",
                "A contribution does not belong to the exact composed snapshot binding",
            ));
        }
        sources.extend(contribution.sources().cloned());
    }
    let source_ids = admit_sources(&sources)?;
    let selection_rows = merge(
        ConsumerRouteView::SelectionSummary,
        &input,
        ConsumerRouteProjectionContribution::selection_rows,
        &applicability,
        &source_ids,
    )?;
    let session_start_rows = merge(
        ConsumerRouteView::SessionStart,
        &input,
        ConsumerRouteProjectionContribution::session_start_rows,
        &applicability,
        &source_ids,
    )?;
    let active_session_rows = merge(
        ConsumerRouteView::ActiveSession,
        &input,
        ConsumerRouteProjectionContribution::active_session_rows,
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
            "Projection exceeds the fixed namespaced-extension maximum",
        ));
    }
    Ok(ConsumerRouteProjection::from_parts(
        ConsumerRouteProjectionIdentity::from_parts(applicability, sources),
        ConsumerRouteSelectionSummary::from_rows(selection_rows),
        ConsumerRouteSessionStartControls::from_rows(session_start_rows),
        ConsumerRouteActiveSessionState::from_rows(active_session_rows),
    ))
}

fn merge<'a, I>(
    view: ConsumerRouteView,
    input: &'a ConsumerRouteProjectionInput<'a>,
    rows_of: fn(&'a ConsumerRouteProjectionContribution) -> I,
    applicability: &ConsumerRouteApplicability,
    sources: &BTreeSet<super::identity::ConsumerRouteProjectionSourceId>,
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

fn require_record_agreement(
    record: &ConfiguredProviderInstanceRecord,
    evidence: &PreparedOperationEvidence,
) -> Result<(), ConsumerRouteProjectionFailure> {
    let binding = evidence.binding();
    let plan = evidence.plan();
    if record.instance_id() != binding.instance_id()
        || record.instance_revision() != binding.instance_revision()
        || record.driver_identity() != binding.driver_identity()
        || record.protocol_facade_id() != binding.protocol_facade_id()
        || record.execution_host_id() != binding.execution_host_id()
        || record.transport_family() != binding.transport_family()
    {
        return Err(snapshot_disagreement());
    }
    let matched = record.routes().any(|route| {
        route.driver_role() == binding.driver_role()
            && route.execution_layer() == binding.execution_layer()
            && route.operation_shape() == binding.operation_shape()
            && route
                .model_route()
                .map(super::super::ConfiguredProviderModelRoute::route_id)
                == plan.model_route_id()
            && route
                .model_route()
                .map(super::super::ConfiguredProviderModelRoute::route_revision)
                == plan.model_route_revision()
            && route
                .model_route()
                .map(super::super::ConfiguredProviderModelRoute::model_id)
                == plan.model_id()
    });
    if matched {
        Ok(())
    } else {
        Err(snapshot_disagreement())
    }
}

fn snapshot_disagreement() -> ConsumerRouteProjectionFailure {
    failure(
        ConsumerRouteProjectionFailureKind::SnapshotIdentityDisagreement,
        "swallowtail.consumer_route_projection.snapshot_identity_rejected",
        "Configured record, prepared evidence, and contributions do not describe one snapshot",
    )
}
