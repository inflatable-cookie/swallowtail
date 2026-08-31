use std::collections::BTreeSet;
use swallowtail_runtime::{
    ConsumerRouteControlId, ConsumerRouteFeatureId, ConsumerRouteNamespacedExtension,
    ConsumerRouteProjectionContribution, ConsumerRouteProjectionSourceId, ConsumerRouteRowIdentity,
};

use super::ledger::{EXEC_BEHAVIOR, EXEC_ROUTE};

pub(super) fn source(id: &str) -> ConsumerRouteProjectionSourceId {
    ConsumerRouteProjectionSourceId::new(id).expect("fixture source id is valid")
}

/// Returns the census semantic id of one published exec row.
///
/// An identity outside the exec tranche panics rather than being folded into a
/// neighbouring row, so a borrowed app-server identity cannot pass silently.
pub(super) fn semantic_id(identity: &ConsumerRouteRowIdentity) -> String {
    if let Some(extension) = identity.namespaced_extension() {
        return route_local(extension);
    }
    match identity {
        ConsumerRouteRowIdentity::Feature(feature) => match feature {
            ConsumerRouteFeatureId::StructuredRun => "feature.structured-run",
            ConsumerRouteFeatureId::StreamingEvents => "feature.streaming-events",
            ConsumerRouteFeatureId::ActivityObservation => "feature.activity-observation",
            ConsumerRouteFeatureId::ReasoningSelection => "feature.reasoning-selection",
            ConsumerRouteFeatureId::StructuredOutput => "feature.structured-output",
            ConsumerRouteFeatureId::Attachments => "feature.attachments",
            ConsumerRouteFeatureId::ExternalSearch => "feature.external-search",
            ConsumerRouteFeatureId::PreparedFacade => "feature.prepared-facade",
            other => panic!("unexpected codex.exec projection feature {other:?}"),
        },
        ConsumerRouteRowIdentity::Control(control) => match control {
            ConsumerRouteControlId::ModelSelection => "control.model-selection",
            ConsumerRouteControlId::ReasoningSelection => "control.reasoning-selection",
            other => panic!("unexpected codex.exec projection control {other:?}"),
        },
    }
    .to_owned()
}

/// Requires every bounded exec descriptor to name its route and revision.
fn route_local(extension: &ConsumerRouteNamespacedExtension) -> String {
    assert_eq!(
        extension.route(),
        EXEC_ROUTE,
        "a bounded exec descriptor names its exact route"
    );
    assert_eq!(
        extension.version_segment(),
        EXEC_BEHAVIOR,
        "a bounded exec descriptor names its qualified behavior revision"
    );
    extension.semantic_id().to_owned()
}

pub(super) fn rows(contribution: &ConsumerRouteProjectionContribution) -> BTreeSet<String> {
    contribution
        .selection_rows()
        .chain(contribution.session_start_rows())
        .chain(contribution.active_session_rows())
        .map(|row| semantic_id(row.identity()))
        .collect()
}
