use std::collections::BTreeSet;
use swallowtail_runtime::{
    ConsumerRouteControlId, ConsumerRouteFeatureId, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionSourceId, ConsumerRouteRowIdentity,
};

pub(super) fn source(id: &str) -> ConsumerRouteProjectionSourceId {
    ConsumerRouteProjectionSourceId::new(id).expect("fixture source id is valid")
}

pub(super) fn semantic_id(identity: &ConsumerRouteRowIdentity) -> &'static str {
    match identity {
        ConsumerRouteRowIdentity::Feature(feature) => match feature {
            ConsumerRouteFeatureId::ModelCatalogue => "feature.model-catalogue",
            ConsumerRouteFeatureId::StructuredRun => "feature.structured-run",
            ConsumerRouteFeatureId::InteractiveSession => "feature.interactive-session",
            ConsumerRouteFeatureId::RealtimeMediaSession => "feature.realtime-media-session",
            ConsumerRouteFeatureId::StreamingEvents => "feature.streaming-events",
            ConsumerRouteFeatureId::UsageEvidence => "feature.usage-evidence",
            ConsumerRouteFeatureId::ActivityObservation => "feature.activity-observation",
            ConsumerRouteFeatureId::ReasoningSelection => "feature.reasoning-selection",
            ConsumerRouteFeatureId::StructuredOutput => "feature.structured-output",
            ConsumerRouteFeatureId::Attachments => "feature.attachments",
            ConsumerRouteFeatureId::ConsumerToolExchange => "feature.consumer-tool-exchange",
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
            ConsumerRouteFeatureId::ProviderSessionArchive => "feature.provider-session-archive",
            ConsumerRouteFeatureId::ProviderSessionRestore => "feature.provider-session-restore",
            ConsumerRouteFeatureId::ProviderSessionDelete => "feature.provider-session-delete",
            ConsumerRouteFeatureId::ProviderSessionReconciliation => {
                "feature.provider-session-reconciliation"
            }
            ConsumerRouteFeatureId::ProviderSessionHistory => "feature.provider-session-history",
            ConsumerRouteFeatureId::PersistentSessionPosture => {
                "feature.persistent-session-posture"
            }
            ConsumerRouteFeatureId::WorkingResource => "feature.working-resource",
            ConsumerRouteFeatureId::BoundedWorkspaceTextWrite => {
                "feature.bounded-workspace-text-write"
            }
            ConsumerRouteFeatureId::ExternalSearch => "feature.external-search",
            ConsumerRouteFeatureId::OutputTokenLimit => "feature.output-token-limit",
            ConsumerRouteFeatureId::PreparedFacade => "feature.prepared-facade",
            ConsumerRouteFeatureId::ActiveSessionReasoningAcknowledgement => {
                "feature.active-session-reasoning-ack"
            }
            ConsumerRouteFeatureId::Namespaced(_) => "feature.namespaced-extension",
        },
        ConsumerRouteRowIdentity::Control(control) => match control {
            ConsumerRouteControlId::ModelSelection => "control.model-selection",
            ConsumerRouteControlId::ReasoningSelection => "control.reasoning-selection",
            ConsumerRouteControlId::SessionOptions => "control.session-options",
            ConsumerRouteControlId::ToolDeclarations => "control.tool-declarations",
            ConsumerRouteControlId::DeveloperInstructions => "control.developer-instructions",
            ConsumerRouteControlId::Idioms => "control.idioms",
            ConsumerRouteControlId::UserInputExchange => "control.user-input-exchange",
            ConsumerRouteControlId::LoadSession => "control.load-session",
            ConsumerRouteControlId::ResumeSession => "control.resume-session",
            ConsumerRouteControlId::SessionCatalogueBounds => "control.session-catalogue-bounds",
            ConsumerRouteControlId::SessionHistoryBounds => "control.session-history-bounds",
            ConsumerRouteControlId::SessionReconciliation => "control.session-reconciliation",
            ConsumerRouteControlId::MaximumOutputTokens => "control.maximum-output-tokens",
            ConsumerRouteControlId::RealtimeMediaConfig => "control.realtime-media-config",
            ConsumerRouteControlId::PlannedConnectionRollover => {
                "control.planned-connection-rollover"
            }
            ConsumerRouteControlId::Namespaced(_) => "control.namespaced-extension",
        },
    }
}

pub(super) fn rows(contribution: &ConsumerRouteProjectionContribution) -> BTreeSet<&'static str> {
    contribution
        .selection_rows()
        .chain(contribution.session_start_rows())
        .chain(contribution.active_session_rows())
        .map(|row| semantic_id(row.identity()))
        .collect()
}
