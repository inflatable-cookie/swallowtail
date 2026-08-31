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
