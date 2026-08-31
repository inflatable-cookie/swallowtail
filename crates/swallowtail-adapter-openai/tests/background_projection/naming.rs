use std::collections::BTreeSet;
use swallowtail_adapter_openai::OPENAI_BACKGROUND_FACADE_REVISION;
use swallowtail_runtime::{
    ConsumerRouteControlId, ConsumerRouteFeatureId, ConsumerRouteNamespacedExtension,
    ConsumerRouteProjectionContribution, ConsumerRouteProjectionSourceId, ConsumerRouteRowIdentity,
};

use super::ledger::BACKGROUND_ROUTE;

pub(super) fn source(id: &str) -> ConsumerRouteProjectionSourceId {
    ConsumerRouteProjectionSourceId::new(id).expect("fixture source id is valid")
}

/// Returns the census semantic id of one published background row.
///
/// An identity outside the background tranche panics rather than being folded
/// into a neighbouring row, so a borrowed Realtime identity cannot pass.
pub(super) fn semantic_id(identity: &ConsumerRouteRowIdentity) -> String {
    if let Some(extension) = identity.namespaced_extension() {
        return route_local(extension);
    }
    match identity {
        ConsumerRouteRowIdentity::Feature(feature) => match feature {
            ConsumerRouteFeatureId::StructuredRun => "feature.structured-run",
            ConsumerRouteFeatureId::StreamingEvents => "feature.streaming-events",
            ConsumerRouteFeatureId::UsageEvidence => "feature.usage-evidence",
            ConsumerRouteFeatureId::OutputTokenLimit => "feature.output-token-limit",
            ConsumerRouteFeatureId::ReasoningSelection => "feature.reasoning-selection",
            ConsumerRouteFeatureId::StructuredOutput => "feature.structured-output",
            ConsumerRouteFeatureId::CancellationOrInterruption => {
                "feature.cancellation-or-interruption"
            }
            ConsumerRouteFeatureId::ActivityObservation => "feature.activity-observation",
            ConsumerRouteFeatureId::PreparedFacade => "feature.prepared-facade",
            other => panic!("unexpected openai.background projection feature {other:?}"),
        },
        ConsumerRouteRowIdentity::Control(control) => match control {
            ConsumerRouteControlId::ModelSelection => "control.model-selection",
            ConsumerRouteControlId::ReasoningSelection => "control.reasoning-selection",
            ConsumerRouteControlId::MaximumOutputTokens => "control.maximum-output-tokens",
            other => panic!("unexpected openai.background projection control {other:?}"),
        },
    }
    .to_owned()
}

/// Requires every bounded background descriptor to name route and revision.
fn route_local(extension: &ConsumerRouteNamespacedExtension) -> String {
    assert_eq!(
        extension.route(),
        BACKGROUND_ROUTE,
        "a bounded background descriptor names its exact route"
    );
    assert_eq!(
        extension.version_segment(),
        OPENAI_BACKGROUND_FACADE_REVISION,
        "a bounded background descriptor names its qualified facade revision"
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

/// One exact census row identity: route, operation shape, and semantic id.
pub(super) type RowIdentity = (String, &'static str, String);

/// Returns the census operation shape one published background row belongs to.
///
/// This map is written independently of the ledger, so a drifted ledger
/// operation shape fails the emitted comparison instead of agreeing with
/// itself.
pub(super) fn operation_shape(identity: &ConsumerRouteRowIdentity) -> &'static str {
    if let Some(extension) = identity.namespaced_extension() {
        return match extension.semantic_id() {
            "feature.retained-background-execution" | "feature.stream-reattachment" => {
                "session-lifecycle"
            }
            "feature.owned-remote-resource-cleanup" => "route-capability",
            "control.structured-output"
            | "control.provider-execution-policy"
            | "control.provider-retention-policy"
            | "control.stream-reattachment"
            | "control.service-tier"
            | "control.active-run-detachment" => "structured-run",
            other => panic!("unexpected bounded openai.background descriptor {other}"),
        };
    }
    match identity {
        ConsumerRouteRowIdentity::Feature(feature) => match feature {
            ConsumerRouteFeatureId::StructuredRun => "structured-run",
            ConsumerRouteFeatureId::StreamingEvents
            | ConsumerRouteFeatureId::UsageEvidence
            | ConsumerRouteFeatureId::ActivityObservation => "route-observation",
            ConsumerRouteFeatureId::OutputTokenLimit
            | ConsumerRouteFeatureId::ReasoningSelection
            | ConsumerRouteFeatureId::StructuredOutput
            | ConsumerRouteFeatureId::CancellationOrInterruption
            | ConsumerRouteFeatureId::PreparedFacade => "route-capability",
            other => panic!("unexpected openai.background projection feature {other:?}"),
        },
        ConsumerRouteRowIdentity::Control(control) => match control {
            ConsumerRouteControlId::ModelSelection
            | ConsumerRouteControlId::ReasoningSelection
            | ConsumerRouteControlId::MaximumOutputTokens => "structured-run",
            other => panic!("unexpected openai.background projection control {other:?}"),
        },
    }
}

/// Returns the exact route one published background row was admitted under.
///
/// A bounded descriptor names its own route, so a drifted extension route is
/// visible here as well as in the qualification assertions.
pub(super) fn route_of(identity: &ConsumerRouteRowIdentity) -> String {
    identity.namespaced_extension().map_or_else(
        || BACKGROUND_ROUTE.to_owned(),
        |extension| extension.route().to_owned(),
    )
}

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
