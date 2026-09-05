use std::collections::BTreeSet;
use swallowtail_core::OperationShape;
use swallowtail_runtime::{
    ConsumerRouteControlId, ConsumerRouteFeatureId, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionRow, ConsumerRouteRowIdentity,
};

use super::ledger::{RPC_ROUTE, SIDECAR_ROUTE};

pub(super) type RowIdentity = (String, &'static str, String);

pub(super) fn semantic_id(identity: &ConsumerRouteRowIdentity) -> String {
    if let Some(extension) = identity.namespaced_extension() {
        assert!(extension.route() == RPC_ROUTE || extension.route() == SIDECAR_ROUTE);
        return extension.semantic_id().to_owned();
    }
    match identity {
        ConsumerRouteRowIdentity::Feature(feature) => match feature {
            ConsumerRouteFeatureId::ModelCatalogue => "feature.model-catalogue",
            ConsumerRouteFeatureId::StructuredRun => "feature.structured-run",
            ConsumerRouteFeatureId::InteractiveSession => "feature.interactive-session",
            ConsumerRouteFeatureId::StreamingEvents => "feature.streaming-events",
            ConsumerRouteFeatureId::UsageEvidence => "feature.usage-evidence",
            ConsumerRouteFeatureId::Attachments => "feature.attachments",
            ConsumerRouteFeatureId::QuestionExchange => "feature.question-exchange",
            ConsumerRouteFeatureId::CancellationOrInterruption => {
                "feature.cancellation-or-interruption"
            }
            ConsumerRouteFeatureId::WorkingResource => "feature.working-resource",
            ConsumerRouteFeatureId::PreparedFacade => "feature.prepared-facade",
            ConsumerRouteFeatureId::ActivityObservation => "feature.activity-observation",
            ConsumerRouteFeatureId::ReasoningSelection => "feature.reasoning-selection",
            ConsumerRouteFeatureId::LoadSession => "feature.load-session",
            ConsumerRouteFeatureId::ResumeSession => "feature.resume-session",
            ConsumerRouteFeatureId::PersistentSessionPosture => {
                "feature.persistent-session-posture"
            }
            other => panic!("unexpected Pi projection feature {other:?}"),
        },
        ConsumerRouteRowIdentity::Control(control) => match control {
            ConsumerRouteControlId::ModelSelection => "control.model-selection",
            ConsumerRouteControlId::ReasoningSelection => "control.reasoning-selection",
            ConsumerRouteControlId::LoadSession => "control.load-session",
            ConsumerRouteControlId::ResumeSession => "control.resume-session",
            other => panic!("unexpected Pi projection control {other:?}"),
        },
    }
    .to_owned()
}

fn operation_shape(
    identity: &ConsumerRouteRowIdentity,
    contribution_shape: OperationShape,
) -> &'static str {
    let semantic = semantic_id(identity);
    match semantic.as_str() {
        "feature.model-catalogue" => "model-catalogue",
        "feature.structured-run" => "structured-run",
        "feature.interactive-session" => "interactive-session",
        "feature.streaming-events" | "feature.usage-evidence" | "feature.activity-observation" => {
            "route-observation"
        }
        "feature.load-session" | "feature.resume-session" => "session-lifecycle",
        "feature.attachments"
        | "feature.question-exchange"
        | "feature.cancellation-or-interruption"
        | "feature.working-resource"
        | "feature.prepared-facade"
        | "feature.reasoning-selection" => "route-capability",
        "feature.persistent-session-posture" => "session-lifecycle",
        "control.load-session" | "control.resume-session" => "session-management",
        "control.model-selection" | "control.reasoning-selection" | "control.attachments" => {
            match contribution_shape {
                OperationShape::StructuredRun => "structured-run",
                OperationShape::InteractiveSession => "interactive-session",
                other => panic!("unexpected Pi control operation shape {other:?}"),
            }
        }
        "control.session-options" => "interactive-session",
        other => panic!("unexpected Pi projection semantic {other}"),
    }
}

pub(super) fn row_identity(
    row: &ConsumerRouteProjectionRow,
    contribution: &ConsumerRouteProjectionContribution,
    route: &str,
) -> RowIdentity {
    let route = row.identity().namespaced_extension().map_or_else(
        || route.to_owned(),
        |extension| extension.route().to_owned(),
    );
    (
        route,
        operation_shape(
            row.identity(),
            contribution.applicability().operation_shape(),
        ),
        semantic_id(row.identity()),
    )
}

pub(super) fn identities(
    contribution: &ConsumerRouteProjectionContribution,
    route: &str,
) -> BTreeSet<RowIdentity> {
    all_rows(contribution)
        .map(|row| row_identity(row, contribution, route))
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
