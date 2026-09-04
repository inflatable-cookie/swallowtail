use std::collections::BTreeSet;
use swallowtail_runtime::{
    ConsumerRouteControlId, ConsumerRouteFeatureId, ConsumerRouteNamespacedExtension,
    ConsumerRouteProjectionContribution, ConsumerRouteProjectionRow,
    ConsumerRouteProjectionSourceId, ConsumerRouteRowIdentity, ConsumerRouteValueDomain,
};

use super::ledger::OLLAMA_ROUTE;

pub(super) fn source(id: &str) -> ConsumerRouteProjectionSourceId {
    ConsumerRouteProjectionSourceId::new(id).expect("fixture source id is valid")
}

pub(super) fn semantic_id(identity: &ConsumerRouteRowIdentity) -> String {
    if let Some(extension) = identity.namespaced_extension() {
        return route_local(extension);
    }
    match identity {
        ConsumerRouteRowIdentity::Feature(feature) => match feature {
            ConsumerRouteFeatureId::ModelCatalogue => "feature.model-catalogue",
            ConsumerRouteFeatureId::StructuredRun => "feature.structured-run",
            ConsumerRouteFeatureId::InteractiveSession => "feature.interactive-session",
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
            other => panic!("unexpected ollama.attached projection feature {other:?}"),
        },
        ConsumerRouteRowIdentity::Control(control) => match control {
            ConsumerRouteControlId::ModelSelection => "control.model-selection",
            ConsumerRouteControlId::ReasoningSelection => "control.reasoning-selection",
            ConsumerRouteControlId::MaximumOutputTokens => "control.maximum-output-tokens",
            other => panic!("unexpected ollama.attached projection control {other:?}"),
        },
    }
    .to_owned()
}

fn route_local(extension: &ConsumerRouteNamespacedExtension) -> String {
    assert_eq!(
        extension.route(),
        OLLAMA_ROUTE,
        "a bounded Ollama descriptor names its exact route"
    );
    match extension.semantic_id() {
        "control.structured-output" | "control.context-window" => {
            extension.semantic_id().to_owned()
        }
        other => panic!("unexpected bounded ollama.attached descriptor {other}"),
    }
}

pub(super) fn operation_shape(identity: &ConsumerRouteRowIdentity) -> &'static str {
    if let Some(extension) = identity.namespaced_extension() {
        return match extension.semantic_id() {
            "control.structured-output" => "structured-run",
            "control.context-window" => {
                panic!("context-window shape is contribution-local")
            }
            other => panic!("unexpected bounded ollama.attached descriptor {other}"),
        };
    }
    match identity {
        ConsumerRouteRowIdentity::Feature(feature) => match feature {
            ConsumerRouteFeatureId::ModelCatalogue => "model-catalogue",
            ConsumerRouteFeatureId::StructuredRun => "structured-run",
            ConsumerRouteFeatureId::InteractiveSession => "interactive-session",
            ConsumerRouteFeatureId::StreamingEvents
            | ConsumerRouteFeatureId::UsageEvidence
            | ConsumerRouteFeatureId::ActivityObservation => "route-observation",
            ConsumerRouteFeatureId::OutputTokenLimit
            | ConsumerRouteFeatureId::ReasoningSelection
            | ConsumerRouteFeatureId::StructuredOutput
            | ConsumerRouteFeatureId::CancellationOrInterruption
            | ConsumerRouteFeatureId::PreparedFacade => "route-capability",
            other => panic!("unexpected ollama.attached projection feature {other:?}"),
        },
        ConsumerRouteRowIdentity::Control(control) => match control {
            ConsumerRouteControlId::ModelSelection
            | ConsumerRouteControlId::ReasoningSelection
            | ConsumerRouteControlId::MaximumOutputTokens => {
                panic!("portable control shape is contribution-local")
            }
            other => panic!("unexpected ollama.attached projection control {other:?}"),
        },
    }
}

/// Census operation shape for one published row on this contribution.
pub(super) fn census_shape(
    identity: &ConsumerRouteRowIdentity,
    prepared_shape: swallowtail_core::OperationShape,
) -> &'static str {
    if let Some(extension) = identity.namespaced_extension() {
        return match (extension.semantic_id(), prepared_shape) {
            ("control.structured-output", swallowtail_core::OperationShape::StructuredRun) => {
                "structured-run"
            }
            ("control.context-window", swallowtail_core::OperationShape::StructuredRun) => {
                "structured-run"
            }
            ("control.context-window", swallowtail_core::OperationShape::InteractiveSession) => {
                "interactive-session"
            }
            (other, shape) => panic!("unexpected bounded Ollama descriptor {other} on {shape:?}"),
        };
    }
    match identity {
        ConsumerRouteRowIdentity::Feature(_) => operation_shape(identity),
        ConsumerRouteRowIdentity::Control(control) => match (control, prepared_shape) {
            (
                ConsumerRouteControlId::ModelSelection,
                swallowtail_core::OperationShape::StructuredRun,
            )
            | (
                ConsumerRouteControlId::ReasoningSelection,
                swallowtail_core::OperationShape::StructuredRun,
            )
            | (
                ConsumerRouteControlId::MaximumOutputTokens,
                swallowtail_core::OperationShape::StructuredRun,
            ) => "structured-run",
            (
                ConsumerRouteControlId::ModelSelection,
                swallowtail_core::OperationShape::InteractiveSession,
            )
            | (
                ConsumerRouteControlId::ReasoningSelection,
                swallowtail_core::OperationShape::InteractiveSession,
            ) => "interactive-session",
            (other, shape) => panic!("unexpected Ollama control {other:?} on {shape:?}"),
        },
    }
}

pub(super) type RowIdentity = (String, &'static str, String);

pub(super) fn row_identity(
    identity: &ConsumerRouteRowIdentity,
    prepared_shape: swallowtail_core::OperationShape,
) -> RowIdentity {
    (
        OLLAMA_ROUTE.to_owned(),
        census_shape(identity, prepared_shape),
        semantic_id(identity),
    )
}

pub(super) fn identities(
    contribution: &ConsumerRouteProjectionContribution,
) -> BTreeSet<RowIdentity> {
    let shape = contribution.applicability().operation_shape();
    all_rows(contribution)
        .map(|row| row_identity(row.identity(), shape))
        .collect()
}

pub(super) fn rows(contribution: &ConsumerRouteProjectionContribution) -> BTreeSet<String> {
    all_rows(contribution)
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
