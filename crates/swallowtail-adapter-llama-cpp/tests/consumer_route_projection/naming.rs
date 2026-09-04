use std::collections::BTreeSet;
use swallowtail_runtime::{
    ConsumerRouteControlId, ConsumerRouteFeatureId, ConsumerRouteNamespacedExtension,
    ConsumerRouteProjectionContribution, ConsumerRouteProjectionRow,
    ConsumerRouteProjectionSourceId, ConsumerRouteRowIdentity, ConsumerRouteValueDomain,
};

use super::ledger::{ATTACHED_ROUTE, OWNED_ROUTE};

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
            ConsumerRouteFeatureId::StreamingEvents => "feature.streaming-events",
            ConsumerRouteFeatureId::UsageEvidence => "feature.usage-evidence",
            ConsumerRouteFeatureId::OutputTokenLimit => "feature.output-token-limit",
            ConsumerRouteFeatureId::ActivityObservation => "feature.activity-observation",
            ConsumerRouteFeatureId::PreparedFacade => "feature.prepared-facade",
            other => panic!("unexpected llama.cpp projection feature {other:?}"),
        },
        ConsumerRouteRowIdentity::Control(control) => match control {
            ConsumerRouteControlId::ModelSelection => "control.model-selection",
            ConsumerRouteControlId::MaximumOutputTokens => "control.maximum-output-tokens",
            other => panic!("unexpected llama.cpp projection control {other:?}"),
        },
    }
    .to_owned()
}

fn route_local(extension: &ConsumerRouteNamespacedExtension) -> String {
    assert!(
        extension.route() == ATTACHED_ROUTE || extension.route() == OWNED_ROUTE,
        "a bounded llama.cpp descriptor names its exact route"
    );
    match extension.semantic_id() {
        "feature.owned-runtime-lifecycle"
        | "control.serving-model-artifact"
        | "control.serving-context-size"
        | "control.serving-reasoning" => extension.semantic_id().to_owned(),
        other => panic!("unexpected bounded llama.cpp descriptor {other}"),
    }
}

pub(super) fn operation_shape(identity: &ConsumerRouteRowIdentity) -> &'static str {
    if let Some(extension) = identity.namespaced_extension() {
        return match extension.semantic_id() {
            "feature.owned-runtime-lifecycle" => "route-capability",
            "control.serving-model-artifact"
            | "control.serving-context-size"
            | "control.serving-reasoning" => "session-lifecycle",
            other => panic!("unexpected bounded llama.cpp descriptor {other}"),
        };
    }
    match identity {
        ConsumerRouteRowIdentity::Feature(feature) => match feature {
            ConsumerRouteFeatureId::ModelCatalogue => "model-catalogue",
            ConsumerRouteFeatureId::StructuredRun => "structured-run",
            ConsumerRouteFeatureId::StreamingEvents
            | ConsumerRouteFeatureId::UsageEvidence
            | ConsumerRouteFeatureId::ActivityObservation => "route-observation",
            ConsumerRouteFeatureId::OutputTokenLimit | ConsumerRouteFeatureId::PreparedFacade => {
                "route-capability"
            }
            other => panic!("unexpected llama.cpp projection feature {other:?}"),
        },
        ConsumerRouteRowIdentity::Control(control) => match control {
            ConsumerRouteControlId::ModelSelection
            | ConsumerRouteControlId::MaximumOutputTokens => "structured-run",
            other => panic!("unexpected llama.cpp projection control {other:?}"),
        },
    }
}

pub(super) fn route_of(identity: &ConsumerRouteRowIdentity) -> String {
    identity.namespaced_extension().map_or_else(
        || {
            match identity {
                ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::ModelCatalogue)
                | ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::StructuredRun)
                | ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::StreamingEvents)
                | ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::UsageEvidence)
                | ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::OutputTokenLimit)
                | ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::ActivityObservation)
                | ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::ModelSelection)
                | ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::MaximumOutputTokens) => {
                    ATTACHED_ROUTE
                }
                ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::PreparedFacade) => {
                    panic!("prepared-facade route is contribution-local")
                }
                other => panic!("unexpected portable llama.cpp identity {other:?}"),
            }
            .to_owned()
        },
        |extension| extension.route().to_owned(),
    )
}

pub(super) type RowIdentity = (String, &'static str, String);

pub(super) fn row_identity(
    identity: &ConsumerRouteRowIdentity,
    fallback_route: &str,
) -> RowIdentity {
    let route = identity.namespaced_extension().map_or_else(
        || fallback_route.to_owned(),
        |extension| extension.route().to_owned(),
    );
    (route, operation_shape(identity), semantic_id(identity))
}

pub(super) fn identities(
    contribution: &ConsumerRouteProjectionContribution,
    route: &str,
) -> BTreeSet<RowIdentity> {
    all_rows(contribution)
        .map(|row| row_identity(row.identity(), route))
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
