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
