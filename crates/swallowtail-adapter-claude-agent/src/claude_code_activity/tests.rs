use crate::claude_code_events::ClaudeCodeEventParser;
use swallowtail_runtime::{
    ActivityKind, ActivityLifecyclePhase, ActivityOperationId, RuntimeEventKind, RuntimeRunId,
};

const TOOLS: &str = include_str!("../../tests/fixtures/claude-code-2.1.220/headless-tools.jsonl");

#[test]
fn exact_claude_corpus_projects_completion_only_messages_and_correlated_tools() {
    let mut parser = ClaudeCodeEventParser::new(
        swallowtail_core::ModelId::new("claude-opus-5").expect("valid model"),
        ActivityOperationId::Run(
            RuntimeRunId::new("claude-code-activity-fixture").expect("valid run id"),
        ),
    );
    let mut events = parser.push(TOOLS.as_bytes()).expect("fixture parses");
    let (trailing, _) = parser.finish().expect("fixture finishes");
    events.extend(trailing);
    let activity = events
        .iter()
        .filter_map(|event| match event.kind() {
            RuntimeEventKind::Activity(activity) => Some(activity),
            _ => None,
        })
        .collect::<Vec<_>>();

    assert!(
        activity
            .iter()
            .all(|activity| activity.phase() == ActivityLifecyclePhase::Completed)
    );
    assert_eq!(
        activity
            .iter()
            .filter(|activity| activity.kind() == &ActivityKind::ProviderOwnedTool)
            .count(),
        2
    );
    assert!(
        activity
            .iter()
            .filter(|activity| { activity.kind() == &ActivityKind::ProviderOwnedTool })
            .all(|activity| activity.label().is_some() && activity.content().is_none())
    );
    assert!(activity.iter().all(|activity| {
        let debug = format!("{activity:?}");
        !debug.contains("/fixture/src/lib.rs") && !debug.contains("private fixture file content")
    }));
}

#[test]
fn forked_assistant_frame_accepts_null_stop_reason_without_final_activity() {
    let mut parser = ClaudeCodeEventParser::new(
        swallowtail_core::ModelId::new("claude-opus-5").expect("valid model"),
        ActivityOperationId::Run(
            RuntimeRunId::new("claude-code-null-stop-reason").expect("valid run id"),
        ),
    );
    let stream = concat!(
        "{\"type\":\"system\",\"subtype\":\"init\",\"session_id\":\"fixture-session\",\"model\":\"claude-opus-5\",\"permissionMode\":\"plan\"}\n",
        "{\"type\":\"assistant\",\"message\":{\"id\":\"msg_forked\",\"role\":\"assistant\",\"model\":\"claude-opus-5\",\"content\":[{\"type\":\"text\",\"text\":\"intermediate update\"}],\"stop_reason\":null},\"parent_tool_use_id\":\"tool-fork\",\"session_id\":\"fixture-session\"}\n",
    );
    let events = parser
        .push(stream.as_bytes())
        .expect("nullable intermediate assistant frame parses");
    let assistant = events
        .iter()
        .find_map(|event| match event.kind() {
            RuntimeEventKind::Activity(activity)
                if activity.kind() == &ActivityKind::AssistantMessage =>
            {
                Some(activity)
            }
            _ => None,
        })
        .expect("assistant activity is present");
    assert_eq!(
        assistant.assistant_phase(),
        Some(swallowtail_runtime::ActivityAssistantPhase::ProviderUnspecified)
    );
    assert!(assistant.content().is_none());
    assert!(events.iter().any(|event| {
        matches!(event.kind(), RuntimeEventKind::OutputDelta)
            && event
                .content()
                .is_some_and(|content| content.as_str() == "intermediate update")
    }));
    assert!(
        !events
            .iter()
            .any(|event| { matches!(event.kind(), RuntimeEventKind::OutputAvailable) })
    );

    let (terminal_events, _) = parser.finish().expect("incomplete stream may be inspected");
    assert!(terminal_events.is_empty());
}
