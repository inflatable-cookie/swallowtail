use std::collections::BTreeSet;
use swallowtail_runtime::{
    ConsumerRouteControlId, ConsumerRouteFeatureId, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionRow, ConsumerRouteProjectionSourceId, ConsumerRouteRowIdentity,
};

use super::ledger::LedgerEntry;

pub(super) fn source(id: &str) -> ConsumerRouteProjectionSourceId {
    ConsumerRouteProjectionSourceId::new(id).expect("valid source id")
}

pub(super) fn row_semantic_id(row: &ConsumerRouteProjectionRow) -> String {
    match row.identity() {
        ConsumerRouteRowIdentity::Feature(f) => match f {
            ConsumerRouteFeatureId::ModelCatalogue => "feature.model-catalogue",
            ConsumerRouteFeatureId::StructuredRun => "feature.structured-run",
            ConsumerRouteFeatureId::InteractiveSession => "feature.interactive-session",
            ConsumerRouteFeatureId::StreamingEvents => "feature.streaming-events",
            ConsumerRouteFeatureId::UsageEvidence => "feature.usage-evidence",
            ConsumerRouteFeatureId::ReasoningSelection => "feature.reasoning-selection",
            ConsumerRouteFeatureId::CancellationOrInterruption => {
                "feature.cancellation-or-interruption"
            }
            ConsumerRouteFeatureId::WorkingResource => "feature.working-resource",
            ConsumerRouteFeatureId::PersistentSessionPosture => {
                "feature.persistent-session-posture"
            }
            ConsumerRouteFeatureId::PreparedFacade => "feature.prepared-facade",
            ConsumerRouteFeatureId::ActivityObservation => "feature.activity-observation",
            other => panic!("unexpected feature {other:?}"),
        }
        .to_string(),
        ConsumerRouteRowIdentity::Control(c) => match c {
            ConsumerRouteControlId::ModelSelection => "control.model-selection".to_string(),
            ConsumerRouteControlId::Namespaced(ext) => ext.semantic_id().to_string(),
            other => panic!("unexpected control {other:?}"),
        },
    }
}

pub(super) fn row_operation_shape(row: &ConsumerRouteProjectionRow) -> &'static str {
    match row.identity() {
        ConsumerRouteRowIdentity::Feature(f) => match f {
            ConsumerRouteFeatureId::ModelCatalogue => "model-catalogue",
            ConsumerRouteFeatureId::StructuredRun => "structured-run",
            ConsumerRouteFeatureId::InteractiveSession => "interactive-session",
            ConsumerRouteFeatureId::StreamingEvents
            | ConsumerRouteFeatureId::UsageEvidence
            | ConsumerRouteFeatureId::ActivityObservation => "route-observation",
            ConsumerRouteFeatureId::PersistentSessionPosture => "session-lifecycle",
            ConsumerRouteFeatureId::PreparedFacade
            | ConsumerRouteFeatureId::CancellationOrInterruption
            | ConsumerRouteFeatureId::WorkingResource
            | ConsumerRouteFeatureId::ReasoningSelection => "route-capability",
            other => panic!("unexpected feature for shape: {other:?}"),
        },
        ConsumerRouteRowIdentity::Control(_) => "structured-run",
    }
}

pub(super) fn row_tuple(
    row: &ConsumerRouteProjectionRow,
    route_id: &'static str,
) -> (&'static str, &'static str, String) {
    (route_id, row_operation_shape(row), row_semantic_id(row))
}

pub(super) fn contribution_tuples(
    contribution: &ConsumerRouteProjectionContribution,
    route_id: &'static str,
) -> BTreeSet<(&'static str, &'static str, String)> {
    contribution
        .selection_rows()
        .chain(contribution.session_start_rows())
        .chain(contribution.active_session_rows())
        .map(|row| row_tuple(row, route_id))
        .collect()
}

pub(super) fn claimed_tuples(
    tranche: &[LedgerEntry],
    profile: &str,
) -> BTreeSet<(&'static str, &'static str, String)> {
    tranche
        .iter()
        .filter(|entry| entry.emitted_by.contains(&profile))
        .map(|entry| {
            (
                entry.route_id,
                entry.operation_shape,
                entry.semantic_id.to_string(),
            )
        })
        .collect()
}

pub(super) fn contribution_semantic_ids(
    contribution: &ConsumerRouteProjectionContribution,
) -> BTreeSet<String> {
    contribution
        .selection_rows()
        .chain(contribution.session_start_rows())
        .chain(contribution.active_session_rows())
        .map(row_semantic_id)
        .collect()
}
