use super::*;
use crate::protocol::{SseDecoder, parse_event};

#[test]
fn frozen_rich_sse_projects_reasoning_tool_and_step_completion() {
    let bytes = include_bytes!(concat!(
        "../../tests/fixtures/opencode-v1.14.48-v1.18.4/",
        "activity-rich.sse"
    ));
    let mut decoder = SseDecoder::default();
    let frames = decoder.push(bytes).expect("SSE fixture decodes");
    decoder.finish().expect("SSE fixture terminates");
    let mut projection = OpenCodeActivityProjection::new(
        RuntimeTurnId::new("opencode-activity-fixture").expect("turn id"),
    );
    let mut observations = Vec::new();
    for frame in frames {
        let event = parse_event(&frame, "session-fixture").expect("event parses");
        observations.extend(projection.project(&event).expect("activity projects"));
    }
    assert!(observations.iter().any(|item| {
        item.kind() == &ActivityKind::ReasoningSummary
            && item.phase() == ActivityLifecyclePhase::Completed
    }));
    assert!(observations.iter().any(|item| {
        item.kind() == &ActivityKind::ProviderOwnedTool
            && item.phase() == ActivityLifecyclePhase::Completed
            && item.label().is_some()
            && item.content().is_none()
    }));
    assert!(observations.iter().any(|item| {
        item.kind() == &ActivityKind::Task && item.phase() == ActivityLifecyclePhase::Completed
    }));
    let rendered = format!("{observations:?}");
    assert!(!rendered.contains("fixture-private-output"));
    assert!(!rendered.contains("fixture-private-raw"));
}

#[test]
fn frozen_gap_sse_exposes_only_operation_local_assistant_updates() {
    let bytes = include_bytes!(concat!(
        "../../tests/fixtures/opencode-v1.14.48-v1.18.4/",
        "activity-gap-1.14.51.sse"
    ));
    let mut decoder = SseDecoder::default();
    let frames = decoder.push(bytes).expect("gap fixture decodes");
    decoder.finish().expect("gap fixture terminates");
    let mut projection = OpenCodeActivityProjection::new(
        RuntimeTurnId::new("opencode-gap-fixture").expect("turn id"),
    );
    let observations = frames
        .into_iter()
        .flat_map(|frame| {
            let event = parse_event(&frame, "session-fixture").expect("event parses");
            projection.project(&event).expect("activity projects")
        })
        .collect::<Vec<_>>();
    assert!(
        observations
            .iter()
            .all(|item| item.kind() == &ActivityKind::AssistantMessage)
    );
}
