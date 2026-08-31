use std::collections::BTreeSet;
use swallowtail_runtime::{
    ConsumerRouteFeatureId, ConsumerRouteProjectionContribution, ConsumerRouteProjectionSourceId,
    ConsumerRouteRowIdentity,
};

use super::ledger::KIRO_ROUTE;

pub(super) fn source(id: &str) -> ConsumerRouteProjectionSourceId {
    ConsumerRouteProjectionSourceId::new(id).expect("fixture source id is valid")
}

/// Returns the census semantic id of one published `kiro.acp` row.
///
/// An identity outside the tranche panics rather than being folded into a
/// neighbouring row, so a borrowed Deep Agents, Qoder, or ZCode identity
/// cannot pass.
pub(super) fn semantic_id(identity: &ConsumerRouteRowIdentity) -> String {
    assert!(
        identity.namespaced_extension().is_none(),
        "kiro.acp publishes no bounded route-local descriptor: {identity:?}"
    );
    match identity {
        ConsumerRouteRowIdentity::Feature(feature) => match feature {
            ConsumerRouteFeatureId::InteractiveSession => "feature.interactive-session",
            ConsumerRouteFeatureId::StreamingEvents => "feature.streaming-events",
            ConsumerRouteFeatureId::CancellationOrInterruption => {
                "feature.cancellation-or-interruption"
            }
            ConsumerRouteFeatureId::WorkingResource => "feature.working-resource",
            ConsumerRouteFeatureId::ActivityObservation => "feature.activity-observation",
            ConsumerRouteFeatureId::PreparedFacade => "feature.prepared-facade",
            other => panic!("unexpected kiro.acp projection feature {other:?}"),
        },
        ConsumerRouteRowIdentity::Control(control) => {
            panic!("kiro.acp publishes no selectable control: {control:?}")
        }
    }
    .to_owned()
}

/// Returns the census operation shape one published row belongs to.
///
/// This map is written independently of the ledger, so a drifted ledger
/// operation shape fails the emitted comparison instead of agreeing with
/// itself.
pub(super) fn operation_shape(identity: &ConsumerRouteRowIdentity) -> &'static str {
    match identity {
        ConsumerRouteRowIdentity::Feature(feature) => match feature {
            ConsumerRouteFeatureId::InteractiveSession => "interactive-session",
            ConsumerRouteFeatureId::StreamingEvents
            | ConsumerRouteFeatureId::ActivityObservation => "route-observation",
            ConsumerRouteFeatureId::CancellationOrInterruption
            | ConsumerRouteFeatureId::WorkingResource
            | ConsumerRouteFeatureId::PreparedFacade => "route-capability",
            other => panic!("unexpected kiro.acp projection feature {other:?}"),
        },
        ConsumerRouteRowIdentity::Control(control) => {
            panic!("kiro.acp publishes no selectable control: {control:?}")
        }
    }
}

/// One exact census row identity: route, operation shape, and semantic id.
pub(super) type RowIdentity = (String, &'static str, String);

pub(super) fn row_identity(identity: &ConsumerRouteRowIdentity) -> RowIdentity {
    (
        KIRO_ROUTE.to_owned(),
        operation_shape(identity),
        semantic_id(identity),
    )
}

pub(super) fn identities(
    contribution: &ConsumerRouteProjectionContribution,
) -> BTreeSet<RowIdentity> {
    contribution
        .selection_rows()
        .chain(contribution.session_start_rows())
        .chain(contribution.active_session_rows())
        .map(|row| row_identity(row.identity()))
        .collect()
}

pub(super) fn rows(contribution: &ConsumerRouteProjectionContribution) -> BTreeSet<String> {
    contribution
        .selection_rows()
        .chain(contribution.session_start_rows())
        .chain(contribution.active_session_rows())
        .map(|row| semantic_id(row.identity()))
        .collect()
}
