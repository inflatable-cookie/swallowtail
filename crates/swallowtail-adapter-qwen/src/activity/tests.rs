use crate::events::QwenEventParser;
use swallowtail_runtime::{
    ActivityKind, ActivityLifecyclePhase, ActivityOperationId, RuntimeEventKind, RuntimeRunId,
};

const ACTIVITY: &str = include_str!("../../tests/fixtures/qwen-code-v0.19.11/activity-tools.jsonl");
const SUCCESS: &str = include_str!("../../tests/fixtures/qwen-code-v0.19.11/success.jsonl");

#[test]
fn partial_message_and_tool_records_project_exact_lifecycle_without_input_disclosure() {
    let events = parse(ACTIVITY);
    let activity = events
        .iter()
        .filter_map(|event| match event.kind() {
            RuntimeEventKind::Activity(activity) => Some(activity),
            _ => None,
        })
        .collect::<Vec<_>>();

    let tool = activity
        .iter()
        .filter(|activity| activity.kind() == &ActivityKind::ProviderOwnedTool)
        .collect::<Vec<_>>();
    assert_eq!(
        tool.iter()
            .map(|activity| activity.phase())
            .collect::<Vec<_>>(),
        [
            ActivityLifecyclePhase::Started,
            ActivityLifecyclePhase::Updated,
            ActivityLifecyclePhase::Completed,
        ]
    );
    assert!(
        tool.iter()
            .all(|activity| { activity.label().is_some() && activity.content().is_none() })
    );
    assert!(activity.iter().any(|activity| {
        activity.kind() == &ActivityKind::AssistantMessage
            && activity.phase() == ActivityLifecyclePhase::Started
    }));
    assert!(
        activity
            .iter()
            .all(|activity| !format!("{activity:?}").contains("fixture-private-path"))
    );
}

#[test]
fn text_delta_stays_visible_on_the_existing_stream() {
    let events = parse(SUCCESS);
    assert!(events.iter().any(|event| {
        matches!(event.kind(), RuntimeEventKind::Activity(activity)
            if activity.kind() == &ActivityKind::AssistantMessage
                && activity.phase() == ActivityLifecyclePhase::Updated)
    }));
}

fn parse(input: &str) -> Vec<swallowtail_runtime::RuntimeEvent> {
    let operation_id =
        ActivityOperationId::Run(RuntimeRunId::new("qwen-activity-fixture").expect("valid run id"));
    let mut parser = QwenEventParser::with_expected_session(
        swallowtail_core::ModelId::new("qwen3-coder-plus").expect("valid model"),
        swallowtail_core::InterfaceVersion::new("0.19.11").expect("valid version"),
        None,
        operation_id,
    );
    let mut events = parser.push(input.as_bytes()).expect("fixture parses");
    let (trailing, _, _) = parser.finish().expect("fixture finishes");
    events.extend(trailing);
    events
}
