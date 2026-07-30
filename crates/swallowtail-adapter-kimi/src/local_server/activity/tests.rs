use super::*;
use crate::local_server::protocol::{WsFrame, decode_ws_frame};

#[test]
fn frozen_cursor_order_projects_steps_tools_and_subagents_without_raw_results() {
    let mut projection = KimiLocalActivityProjection::new(
        RuntimeTurnId::new("kimi-activity-fixture").expect("turn id"),
    );
    let fixture = include_str!(concat!(
        "../../../tests/fixtures/kimi-local-server-0.28.1-0.29.0/",
        "activity.jsonl"
    ));
    let mut observations = Vec::new();
    for line in fixture.lines() {
        let WsFrame::Event(event) = decode_ws_frame(line.as_bytes()).expect("event decodes") else {
            panic!("activity fixture must contain events");
        };
        observations.extend(projection.project(&event.event).expect("activity projects"));
    }
    assert!(observations.iter().any(|item| {
        item.kind() == &ActivityKind::ReasoningSummary
            && item.phase() == ActivityLifecyclePhase::Updated
    }));
    assert!(observations.iter().any(|item| {
        item.kind() == &ActivityKind::ProviderOwnedTool
            && item.phase() == ActivityLifecyclePhase::Completed
            && item.label().is_some()
            && item.content().is_none()
    }));
    assert!(observations.iter().any(|item| {
        item.kind() == &ActivityKind::SubagentOrCollaboration
            && item.phase() == ActivityLifecyclePhase::Completed
    }));
    assert!(observations.iter().any(|item| {
        item.kind() == &ActivityKind::Task && item.phase() == ActivityLifecyclePhase::Completed
    }));
    let rendered = format!("{observations:?}");
    assert!(!rendered.contains("fixture-private-output"));
    assert!(!rendered.contains("fixture-private-summary"));
}
