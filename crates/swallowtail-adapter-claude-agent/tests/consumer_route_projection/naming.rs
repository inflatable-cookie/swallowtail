use std::collections::BTreeSet;
use swallowtail_core::OperationShape;
use swallowtail_runtime::{
    ConsumerRouteControlId, ConsumerRouteFeatureId, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionSourceId, ConsumerRouteRowIdentity,
};

pub(super) fn source(value: &str) -> ConsumerRouteProjectionSourceId {
    ConsumerRouteProjectionSourceId::new(value).expect("source id is valid")
}

pub(super) fn semantic_id(identity: &ConsumerRouteRowIdentity) -> String {
    if let Some(extension) = identity.namespaced_extension() {
        return extension.semantic_id().to_owned();
    }
    match identity {
        ConsumerRouteRowIdentity::Feature(feature) => match feature {
            ConsumerRouteFeatureId::StructuredRun => "feature.structured-run",
            ConsumerRouteFeatureId::InteractiveSession => "feature.interactive-session",
            ConsumerRouteFeatureId::StreamingEvents => "feature.streaming-events",
            ConsumerRouteFeatureId::UsageEvidence => "feature.usage-evidence",
            ConsumerRouteFeatureId::ReasoningSelection => "feature.reasoning-selection",
            ConsumerRouteFeatureId::QuestionExchange => "feature.question-exchange",
            ConsumerRouteFeatureId::CancellationOrInterruption => {
                "feature.cancellation-or-interruption"
            }
            ConsumerRouteFeatureId::LoadSession => "feature.load-session",
            ConsumerRouteFeatureId::ResumeSession => "feature.resume-session",
            ConsumerRouteFeatureId::ProviderSessionDelete => "feature.provider-session-delete",
            ConsumerRouteFeatureId::PersistentSessionPosture => {
                "feature.persistent-session-posture"
            }
            ConsumerRouteFeatureId::WorkingResource => "feature.working-resource",
            ConsumerRouteFeatureId::PreparedFacade => "feature.prepared-facade",
            ConsumerRouteFeatureId::ActivityObservation => "feature.activity-observation",
            ConsumerRouteFeatureId::ActiveSessionReasoningAcknowledgement => {
                "feature.active-session-reasoning-ack"
            }
            other => panic!("unexpected projected feature {other:?}"),
        },
        ConsumerRouteRowIdentity::Control(control) => match control {
            ConsumerRouteControlId::ModelSelection => "control.model-selection",
            ConsumerRouteControlId::ReasoningSelection => "control.reasoning-selection",
            ConsumerRouteControlId::SessionOptions => "control.session-options",
            ConsumerRouteControlId::LoadSession => "control.load-session",
            ConsumerRouteControlId::ResumeSession => "control.resume-session",
            other => panic!("unexpected projected control {other:?}"),
        },
    }
    .to_owned()
}

pub(super) fn rows(contribution: &ConsumerRouteProjectionContribution) -> BTreeSet<String> {
    contribution
        .selection_rows()
        .chain(contribution.session_start_rows())
        .chain(contribution.active_session_rows())
        .map(|row| semantic_id(row.identity()))
        .collect()
}

pub(super) type RowIdentity = (String, &'static str, String);

pub(super) fn identities(
    route_id: &str,
    contribution: &ConsumerRouteProjectionContribution,
) -> BTreeSet<RowIdentity> {
    let prepared_shape = contribution.applicability().operation_shape();
    contribution
        .selection_rows()
        .chain(contribution.session_start_rows())
        .chain(contribution.active_session_rows())
        .map(|row| {
            assert_eq!(row.applicability(), contribution.applicability());
            let semantic = semantic_id(row.identity());
            (
                route_id.to_owned(),
                operation_shape(&semantic, prepared_shape),
                semantic,
            )
        })
        .collect()
}

fn operation_shape(semantic_id: &str, prepared_shape: OperationShape) -> &'static str {
    match semantic_id {
        "feature.model-catalogue" => "model-catalogue",
        "feature.structured-run" => "structured-run",
        "feature.interactive-session"
        | "feature.active-session-reasoning-ack"
        | "feature.negotiated-model-options-observation" => "interactive-session",
        "feature.streaming-events" | "feature.usage-evidence" | "feature.activity-observation" => {
            "route-observation"
        }
        "feature.load-session"
        | "feature.resume-session"
        | "feature.provider-session-delete"
        | "feature.native-session-close"
        | "feature.persistent-session-posture" => "session-lifecycle",
        "feature.reasoning-selection"
        | "feature.permission-exchange"
        | "feature.question-exchange"
        | "feature.cancellation-or-interruption"
        | "feature.working-resource"
        | "feature.owned-remote-resource-cleanup"
        | "feature.prepared-facade" => "route-capability",
        "control.load-session" | "control.resume-session" | "control.provider-session-delete" => {
            "session-management"
        }
        "control.model-selection"
        | "control.reasoning-selection"
        | "control.session-options"
        | "control.permission-handling"
        | "control.run-retention"
        | "control.maximum-agentic-turns" => match prepared_shape {
            OperationShape::StructuredRun => "structured-run",
            OperationShape::InteractiveSession => "interactive-session",
            OperationShape::ProviderSessionManagement => "session-management",
            other => panic!("control {semantic_id} published from unexpected {other:?}"),
        },
        other => panic!("unexpected census identity {other}"),
    }
}
