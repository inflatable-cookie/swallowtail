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
    let completed_subagent = observations
        .iter()
        .find(|item| {
            item.kind() == &ActivityKind::SubagentOrCollaboration
                && item.phase() == ActivityLifecyclePhase::Completed
        })
        .expect("subagent completion is visible");
    let child = completed_subagent
        .subagents()
        .next()
        .expect("child snapshot");
    assert_eq!(
        child.status(),
        swallowtail_runtime::SubagentStatus::Completed
    );
    assert_eq!(
        child.parent(),
        &swallowtail_runtime::SubagentParent::Operation
    );
    assert_eq!(child.label().unwrap().as_str(), "fixture");
    assert_eq!(child.background(), Some(false));
    assert_eq!(
        child.originating_activity().unwrap().as_provider_value(),
        "tool-fixture"
    );
    assert!(observations.iter().any(|item| {
        item.kind() == &ActivityKind::Task && item.phase() == ActivityLifecyclePhase::Completed
    }));
    let rendered = format!("{observations:?}");
    assert!(!rendered.contains("fixture-private-output"));
    assert!(!rendered.contains("fixture-private-summary"));
}

#[test]
fn richer_0_31_subagent_status_remains_non_rendered_progress() {
    let mut projection = KimiLocalActivityProjection::new(
        RuntimeTurnId::new("kimi-status-fixture").expect("turn id"),
    );
    let fixture = include_str!(concat!(
        "../../../tests/fixtures/kimi-local-server-0.31.0/",
        "subagent-status.jsonl"
    ));
    let lines = fixture.lines().collect::<Vec<_>>();
    assert_eq!(lines.len(), 1);

    let WsFrame::Event(event) = decode_ws_frame(lines[0].as_bytes()).expect("status event decodes")
    else {
        panic!("status fixture must contain one event");
    };
    assert_eq!(
        event.event,
        crate::local_server::protocol::WsEvent::Progress
    );
    assert!(
        projection
            .project(&event.event)
            .expect("status projects")
            .is_empty()
    );
}

#[test]
fn suspended_subagent_is_waiting_without_completing_its_activity() {
    let mut projection =
        KimiLocalActivityProjection::new(RuntimeTurnId::new("kimi-waiting-fixture").unwrap());
    projection
        .project(&crate::local_server::protocol::WsEvent::SubagentSpawned {
            subagent_id: "child".to_owned(),
            name: "research".to_owned(),
            parent_tool_call_id: "tool".to_owned(),
            background: true,
        })
        .unwrap();
    let waiting = projection
        .project(&crate::local_server::protocol::WsEvent::SubagentUpdated {
            subagent_id: "child".to_owned(),
            suspended: true,
        })
        .unwrap();

    assert_eq!(waiting[0].phase(), ActivityLifecyclePhase::Updated);
    assert_eq!(
        waiting[0].subagents().next().unwrap().status(),
        swallowtail_runtime::SubagentStatus::Waiting
    );
}
