use std::collections::BTreeSet;

use super::MAX_CONSUMER_ROUTE_SOURCE_IDENTITIES;
use super::applicability::ConsumerRouteApplicability;
use super::failure::{ConsumerRouteProjectionFailure, ConsumerRouteProjectionFailureKind, failure};
use super::identity::{ConsumerRouteProjectionSourceId, ConsumerRouteProjectionSourceIdentity};
use super::row::ConsumerRouteProjectionRow;
use super::semantics::{
    ConsumerRouteActorPosture, ConsumerRouteLifecycle, ConsumerRouteMutationAuthority,
};
use super::value::{ConsumerRouteOmissionSemantics, ConsumerRouteValueDomain};
use super::view::ConsumerRouteView;

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
    match row.control_value() {
        Some(value) => {
            if matches!(value.domain(), ConsumerRouteValueDomain::Descriptor)
                && !matches!(
                    value.omission(),
                    ConsumerRouteOmissionSemantics::NotSelectable
                )
            {
                return Err(failure(
                    ConsumerRouteProjectionFailureKind::ValueDomainInvalid,
                    "swallowtail.consumer_route_projection.value_domain_rejected",
                    "A descriptor-only domain cannot carry selectable omission truth",
                ));
            }
            Ok(())
        }
        None if matches!(
            row.actor_posture(),
            ConsumerRouteActorPosture::ConsumerSelectable
        ) =>
        {
            Err(failure(
                ConsumerRouteProjectionFailureKind::ValueDomainInvalid,
                "swallowtail.consumer_route_projection.value_domain_rejected",
                "A consumer-selectable row must publish its value kind, domain, and omission truth",
            ))
        }
        None => Ok(()),
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

/// Rejects any selectable, prepared, or acknowledged claim without authority.
///
/// A `PerTurn` row is consumer-mediated: it never carries prepared
/// session-start authority, prepared state, or a provider acknowledgement.
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
    admit_lifecycle_authority(row)?;
    let state = row.state_support();
    let authority = row.mutation_authority();
    let selectable = matches!(
        row.actor_posture(),
        ConsumerRouteActorPosture::ConsumerSelectable
    );
    let post_session_start = matches!(
        row.lifecycle(),
        ConsumerRouteLifecycle::PerTurn | ConsumerRouteLifecycle::PostOpenObservationOnly
    );
    let acknowledged_claim = state.provider_effective() || state.rejected();
    let selectable_after_open = matches!(
        row.lifecycle(),
        ConsumerRouteLifecycle::PostOpenObservationOnly
    ) && selectable;
    if (post_session_start && state.prepared())
        || (acknowledged_claim && !authority.is_acknowledged())
        || (selectable_after_open && !authority.is_acknowledged())
        || (matches!(authority, ConsumerRouteMutationAuthority::Absent) && selectable)
    {
        return Err(failure(
            ConsumerRouteProjectionFailureKind::MutationAuthorityAbsent,
            "swallowtail.consumer_route_projection.mutation_authority_absent",
            "A projected row claims selectable or acknowledged posture without exact authority",
        ));
    }
    Ok(())
}

/// Requires the exact authority posture the row's lifecycle admits.
fn admit_lifecycle_authority(
    row: &ConsumerRouteProjectionRow,
) -> Result<(), ConsumerRouteProjectionFailure> {
    let per_turn = matches!(row.lifecycle(), ConsumerRouteLifecycle::PerTurn);
    let authority = row.mutation_authority();
    let selectable = matches!(
        row.actor_posture(),
        ConsumerRouteActorPosture::ConsumerSelectable
    );
    let mismatched = if per_turn {
        authority.is_prepared_session_start()
            || authority.is_acknowledged()
            || row.state_support().provider_effective()
            || row.state_support().rejected()
            || (selectable && !authority.is_consumer_mediated_per_turn())
    } else {
        authority.is_consumer_mediated_per_turn()
    };
    if mismatched {
        return Err(failure(
            ConsumerRouteProjectionFailureKind::MutationAuthorityAbsent,
            "swallowtail.consumer_route_projection.lifecycle_authority_rejected",
            "A projected row's mutation authority does not match its exact lifecycle",
        ));
    }
    Ok(())
}
