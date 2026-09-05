use std::collections::BTreeSet;

use swallowtail_core::OperationShape;
use swallowtail_runtime::{
    ConsumerRouteControlId, ConsumerRouteFeatureId, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionRow, ConsumerRouteRowIdentity,
};

use super::ledger::ROUTE;

pub(super) type RowIdentity = (String, &'static str, String);

pub(super) fn semantic_id(identity: &ConsumerRouteRowIdentity) -> String {
    if let Some(extension) = identity.namespaced_extension() {
        assert_eq!(extension.route(), ROUTE);
        return extension.semantic_id().to_owned();
    }
    match identity {
        ConsumerRouteRowIdentity::Feature(feature) => match feature {
            ConsumerRouteFeatureId::ModelCatalogue => "feature.model-catalogue",
            ConsumerRouteFeatureId::StructuredRun => "feature.structured-run",
            ConsumerRouteFeatureId::InteractiveSession => "feature.interactive-session",
            ConsumerRouteFeatureId::StreamingEvents => "feature.streaming-events",
            ConsumerRouteFeatureId::UsageEvidence => "feature.usage-evidence",
            ConsumerRouteFeatureId::ActivityObservation => "feature.activity-observation",
            ConsumerRouteFeatureId::ReasoningSelection => "feature.reasoning-selection",
            ConsumerRouteFeatureId::StructuredOutput => "feature.structured-output",
            ConsumerRouteFeatureId::Attachments => "feature.attachments",
            ConsumerRouteFeatureId::QuestionExchange => "feature.question-exchange",
            ConsumerRouteFeatureId::CancellationOrInterruption => {
                "feature.cancellation-or-interruption"
            }
            ConsumerRouteFeatureId::LoadSession => "feature.load-session",
            ConsumerRouteFeatureId::ResumeSession => "feature.resume-session",
            ConsumerRouteFeatureId::ProviderSessionCatalogue => {
                "feature.provider-session-catalogue"
            }
            ConsumerRouteFeatureId::ProviderSessionImport => "feature.provider-session-import",
            ConsumerRouteFeatureId::ProviderSessionDelete => "feature.provider-session-delete",
            ConsumerRouteFeatureId::PersistentSessionPosture => {
                "feature.persistent-session-posture"
            }
            ConsumerRouteFeatureId::WorkingResource => "feature.working-resource",
            ConsumerRouteFeatureId::PreparedFacade => "feature.prepared-facade",
            other => panic!("unexpected OpenCode projection feature {other:?}"),
        },
        ConsumerRouteRowIdentity::Control(control) => match control {
            ConsumerRouteControlId::ModelSelection => "control.model-selection",
            ConsumerRouteControlId::ReasoningSelection => "control.reasoning-selection",
            ConsumerRouteControlId::LoadSession => "control.load-session",
            ConsumerRouteControlId::ResumeSession => "control.resume-session",
            ConsumerRouteControlId::SessionCatalogueBounds => "control.provider-session-catalogue",
            other => panic!("unexpected OpenCode projection control {other:?}"),
        },
    }
    .to_owned()
}

fn operation_shape(
    identity: &ConsumerRouteRowIdentity,
    contribution_shape: OperationShape,
) -> &'static str {
    match semantic_id(identity).as_str() {
        "feature.model-catalogue" => "model-catalogue",
        "feature.structured-run" => "structured-run",
        "feature.interactive-session" => "interactive-session",
        "feature.streaming-events" | "feature.usage-evidence" | "feature.activity-observation" => {
            "route-observation"
        }
        "feature.load-session"
        | "feature.resume-session"
        | "feature.provider-session-catalogue"
        | "feature.provider-session-import"
        | "feature.provider-session-delete"
        | "feature.persistent-session-posture" => "session-lifecycle",
        "feature.reasoning-selection"
        | "feature.structured-output"
        | "feature.attachments"
        | "feature.permission-exchange"
        | "feature.question-exchange"
        | "feature.cancellation-or-interruption"
        | "feature.working-resource"
        | "feature.prepared-facade"
        | "feature.owned-remote-resource-cleanup" => "route-capability",
        "control.model-selection"
        | "control.reasoning-selection"
        | "control.structured-output"
        | "control.attachments"
        | "control.provider-callbacks"
        | "control.active-turn-detachment" => match contribution_shape {
            OperationShape::StructuredRun => "structured-run",
            OperationShape::InteractiveSession => "interactive-session",
            other => panic!("unexpected OpenCode control operation shape {other:?}"),
        },
        "control.load-session"
        | "control.resume-session"
        | "control.provider-session-catalogue" => "session-management",
        other => panic!("unexpected OpenCode projection semantic {other}"),
    }
}

pub(super) fn row_identity(
    row: &ConsumerRouteProjectionRow,
    contribution: &ConsumerRouteProjectionContribution,
) -> RowIdentity {
    (
        ROUTE.to_owned(),
        operation_shape(
            row.identity(),
            contribution.applicability().operation_shape(),
        ),
        semantic_id(row.identity()),
    )
}

pub(super) fn identities(
    contribution: &ConsumerRouteProjectionContribution,
) -> BTreeSet<RowIdentity> {
    contribution
        .selection_rows()
        .chain(contribution.session_start_rows())
        .chain(contribution.active_session_rows())
        .map(|row| row_identity(row, contribution))
        .collect()
}
