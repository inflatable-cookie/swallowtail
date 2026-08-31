use super::ZCODE_APP_SERVER_ROUTE;
use crate::selection::ZCODE_PROTOCOL_FACADE_REVISION;
use std::collections::BTreeSet;
use swallowtail_runtime::{
    ConsumerRouteControlId, ConsumerRouteFeatureId, ConsumerRouteNamespacedExtension,
    ConsumerRouteProjectionContribution, ConsumerRouteProjectionRow,
    ConsumerRouteProjectionSourceId, ConsumerRouteRowIdentity, ConsumerRouteValueDomain,
};

pub(super) fn source(id: &str) -> ConsumerRouteProjectionSourceId {
    ConsumerRouteProjectionSourceId::new(id).expect("fixture source id is valid")
}

/// Returns the census semantic id of one published `zcode.app-server` row.
///
/// An identity outside the tranche panics rather than being folded into a
/// neighbouring row, so a borrowed Deep Agents, Kiro, or Qoder identity cannot
/// pass.
pub(super) fn semantic_id(identity: &ConsumerRouteRowIdentity) -> String {
    if let Some(extension) = identity.namespaced_extension() {
        return route_local(extension);
    }
    match identity {
        ConsumerRouteRowIdentity::Feature(feature) => match feature {
            ConsumerRouteFeatureId::StructuredRun => "feature.structured-run",
            ConsumerRouteFeatureId::StreamingEvents => "feature.streaming-events",
            ConsumerRouteFeatureId::UsageEvidence => "feature.usage-evidence",
            ConsumerRouteFeatureId::CancellationOrInterruption => {
                "feature.cancellation-or-interruption"
            }
            ConsumerRouteFeatureId::WorkingResource => "feature.working-resource",
            ConsumerRouteFeatureId::ActivityObservation => "feature.activity-observation",
            ConsumerRouteFeatureId::PreparedFacade => "feature.prepared-facade",
            other => panic!("unexpected zcode.app-server projection feature {other:?}"),
        },
        ConsumerRouteRowIdentity::Control(control) => match control {
            ConsumerRouteControlId::ModelSelection => "control.model-selection",
            other => panic!("unexpected zcode.app-server projection control {other:?}"),
        },
    }
    .to_owned()
}

/// Requires every bounded app-server descriptor to name route and revision.
fn route_local(extension: &ConsumerRouteNamespacedExtension) -> String {
    assert_eq!(
        extension.route(),
        ZCODE_APP_SERVER_ROUTE,
        "a bounded app-server descriptor names its exact route"
    );
    assert_eq!(
        extension.version_segment(),
        ZCODE_PROTOCOL_FACADE_REVISION,
        "a bounded app-server descriptor names its qualified facade revision"
    );
    extension.semantic_id().to_owned()
}

/// Returns the census operation shape one published row belongs to.
///
/// This map is written independently of the ledger, so a drifted ledger
/// operation shape fails the emitted comparison instead of agreeing with
/// itself.
pub(super) fn operation_shape(identity: &ConsumerRouteRowIdentity) -> &'static str {
    if let Some(extension) = identity.namespaced_extension() {
        return match extension.semantic_id() {
            "feature.owned-runtime-lifecycle" => "route-capability",
            "control.app-server-mode" => "structured-run",
            other => panic!("unexpected bounded zcode.app-server descriptor {other}"),
        };
    }
    match identity {
        ConsumerRouteRowIdentity::Feature(feature) => match feature {
            ConsumerRouteFeatureId::StructuredRun => "structured-run",
            ConsumerRouteFeatureId::StreamingEvents
            | ConsumerRouteFeatureId::UsageEvidence
            | ConsumerRouteFeatureId::ActivityObservation => "route-observation",
            ConsumerRouteFeatureId::CancellationOrInterruption
            | ConsumerRouteFeatureId::WorkingResource
            | ConsumerRouteFeatureId::PreparedFacade => "route-capability",
            other => panic!("unexpected zcode.app-server projection feature {other:?}"),
        },
        ConsumerRouteRowIdentity::Control(control) => match control {
            ConsumerRouteControlId::ModelSelection => "structured-run",
            other => panic!("unexpected zcode.app-server projection control {other:?}"),
        },
    }
}

/// Returns the exact route one published row was admitted under.
pub(super) fn route_of(identity: &ConsumerRouteRowIdentity) -> String {
    identity.namespaced_extension().map_or_else(
        || ZCODE_APP_SERVER_ROUTE.to_owned(),
        |extension| extension.route().to_owned(),
    )
}

/// One exact census row identity: route, operation shape, and semantic id.
pub(super) type RowIdentity = (String, &'static str, String);

pub(super) fn row_identity(identity: &ConsumerRouteRowIdentity) -> RowIdentity {
    (
        route_of(identity),
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

pub(super) fn all_rows(
    contribution: &ConsumerRouteProjectionContribution,
) -> impl Iterator<Item = &ConsumerRouteProjectionRow> {
    contribution
        .selection_rows()
        .chain(contribution.session_start_rows())
        .chain(contribution.active_session_rows())
}

pub(super) fn row<'a>(
    contribution: &'a ConsumerRouteProjectionContribution,
    semantic: &str,
) -> &'a ConsumerRouteProjectionRow {
    all_rows(contribution)
        .find(|row| semantic_id(row.identity()) == semantic)
        .unwrap_or_else(|| panic!("{semantic} is published"))
}

/// Returns the exactly admitted values one published control row carries.
pub(super) fn admitted(
    contribution: &ConsumerRouteProjectionContribution,
    semantic: &str,
) -> Vec<String> {
    let ConsumerRouteValueDomain::Enumerated(values) = row(contribution, semantic)
        .control_value()
        .unwrap_or_else(|| panic!("{semantic} carries its exact value"))
        .domain()
    else {
        panic!("{semantic} publishes an exactly admitted domain");
    };
    values
        .values()
        .map(|value| value.as_str().to_owned())
        .collect()
}
