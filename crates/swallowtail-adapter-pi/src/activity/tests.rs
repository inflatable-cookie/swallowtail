use super::PiActivityProjection;
use crate::protocol::{PiAgentEvent, PiRpcDecoder, PiRpcRecord};
use swallowtail_runtime::{ActivityKind, ActivityLifecyclePhase, ActivityStatus, RuntimeTurnId};

const ACTIVITY: &str = include_str!("../../tests/fixtures/pi-rpc-0.80.10/activity.jsonl");

#[test]
fn exact_pi_corpus_projects_message_reasoning_tool_compaction_and_unknown() {
    let mut decoder = PiRpcDecoder::new();
    let mut projection = PiActivityProjection::new(
        RuntimeTurnId::new("pi-activity-fixture").expect("valid turn id"),
    );
    let mut observations = Vec::new();
    for record in decoder
        .push(ACTIVITY.as_bytes())
        .expect("activity corpus decodes")
    {
        let PiRpcRecord::AgentEvent(event) = record else {
            panic!("activity corpus contains only agent events");
        };
        observations.extend(projection.project(&event).expect("event projects"));
        if matches!(event, PiAgentEvent::Settled) {
            observations.extend(
                projection
                    .complete(ActivityStatus::Completed)
                    .expect("open activity completes"),
            );
        }
    }
    decoder.finish().expect("corpus is LF terminated");

    assert!(observations.iter().any(|observation| {
        observation.kind() == &ActivityKind::AssistantMessage
            && observation.phase() == ActivityLifecyclePhase::Started
    }));
    assert!(observations.iter().any(|observation| {
        observation.kind() == &ActivityKind::ReasoningSummary
            && observation.phase() == ActivityLifecyclePhase::Updated
    }));
    let tool: Vec<_> = observations
        .iter()
        .filter(|observation| observation.kind() == &ActivityKind::ProviderOwnedTool)
        .collect();
    assert_eq!(
        tool.iter()
            .map(|observation| observation.phase())
            .collect::<Vec<_>>(),
        [
            ActivityLifecyclePhase::Started,
            ActivityLifecyclePhase::Updated,
            ActivityLifecyclePhase::Completed,
        ]
    );
    assert!(
        tool.iter().all(|observation| {
            observation.label().is_some() && observation.content().is_none()
        })
    );
    assert!(observations.iter().any(|observation| {
        observation.kind() == &ActivityKind::ContextCompaction
            && observation.phase() == ActivityLifecyclePhase::Completed
    }));
    assert!(
        observations
            .iter()
            .all(|observation| { !format!("{observation:?}").contains("fixture-private") })
    );
}
