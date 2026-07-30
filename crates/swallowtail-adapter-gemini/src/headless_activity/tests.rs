use crate::headless_events::GeminiHeadlessEventParser;
use swallowtail_runtime::{
    ActivityKind, ActivityLifecyclePhase, ActivityOperationId, RuntimeEventKind, RuntimeRunId,
};

const SUCCESS: &str =
    include_str!("../../tests/fixtures/gemini-headless-0.51.0-0.52.0/success.jsonl");

#[test]
fn exact_gemini_corpus_projects_assistant_tools_and_excludes_private_bodies() {
    let mut parser = GeminiHeadlessEventParser::new(
        swallowtail_core::ModelId::new("gemini-2.5-flash").expect("valid model"),
        "<SESSION_ID>".to_owned(),
        ActivityOperationId::Run(
            RuntimeRunId::new("gemini-activity-fixture").expect("valid run id"),
        ),
    );
    let mut events = parser.push(SUCCESS.as_bytes()).expect("fixture parses");
    let (trailing, _) = parser.finish().expect("fixture finishes");
    events.extend(trailing);
    let activity = events
        .iter()
        .filter_map(|event| match event.kind() {
            RuntimeEventKind::Activity(activity) => Some(activity),
            _ => None,
        })
        .collect::<Vec<_>>();

    assert!(activity.iter().any(|activity| {
        activity.kind() == &ActivityKind::AssistantMessage
            && activity.phase() == ActivityLifecyclePhase::Updated
    }));
    let tools = activity
        .iter()
        .filter(|activity| activity.kind() == &ActivityKind::ProviderOwnedTool)
        .collect::<Vec<_>>();
    assert_eq!(tools.len(), 2);
    assert!(
        tools
            .iter()
            .all(|activity| { activity.label().is_some() && activity.content().is_none() })
    );
    assert!(activity.iter().all(|activity| {
        let debug = format!("{activity:?}");
        !debug.contains("fixture-private-path")
            && !debug.contains("fixture-provider-secret-never-diagnose")
    }));
}
