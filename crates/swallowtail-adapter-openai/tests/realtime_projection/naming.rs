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
        ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::ModelCatalogue) => {
            "feature.model-catalogue"
        }
        ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::RealtimeMediaSession) => {
            "feature.realtime-media-session"
        }
        ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::StreamingEvents) => {
            "feature.streaming-events"
        }
        ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::UsageEvidence) => {
            "feature.usage-evidence"
        }
        ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::OutputTokenLimit) => {
            "feature.output-token-limit"
        }
        ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::ReasoningSelection) => {
            "feature.reasoning-selection"
        }
        ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::CancellationOrInterruption) => {
            "feature.cancellation-or-interruption"
        }
        ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::PersistentSessionPosture) => {
            "feature.persistent-session-posture"
        }
        ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::PreparedFacade) => {
            "feature.prepared-facade"
        }
        ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::ActivityObservation) => {
            "feature.activity-observation"
        }
        ConsumerRouteRowIdentity::Feature(
            ConsumerRouteFeatureId::ActiveSessionReasoningAcknowledgement,
        ) => "feature.active-session-reasoning-ack",
        ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::ReasoningSelection) => {
            "control.reasoning-selection-session-start"
        }
        ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::MaximumOutputTokens) => {
            "control.maximum-output-tokens"
        }
        ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::RealtimeMediaConfig) => {
            "control.realtime-media-config"
        }
        ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::PlannedConnectionRollover) => {
            "control.planned-connection-rollover"
        }
        other => panic!("unexpected realtime projection row {other:?}"),
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
