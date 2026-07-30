use crate::headless_events::KimiHeadlessEventParser;
use swallowtail_runtime::{
    ActivityKind, ActivityLifecyclePhase, ActivityOperationId, RuntimeEventKind, RuntimeRunId,
};

const TOOLS: &str =
    include_str!("../../tests/fixtures/kimi-code-0.29.1-0.29.2/headless-tools.jsonl");
const RETRY: &str =
    include_str!("../../tests/fixtures/kimi-code-0.29.1-0.29.2/headless-retry.jsonl");

#[test]
fn exact_kimi_corpus_projects_completion_only_messages_and_correlated_tools() {
    let activity = parse(TOOLS);
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
        !debug.contains("fixture.txt") && !debug.contains("fixture content")
    }));
}

#[test]
fn retry_metadata_is_namespaced_without_private_error_disclosure() {
    let activity = parse(RETRY);
    assert!(activity.iter().any(|activity| {
        matches!(activity.kind(), ActivityKind::Unknown(namespace)
            if namespace.as_str() == "kimi-code.headless.retry")
    }));
    assert!(
        activity
            .iter()
            .all(|activity| !format!("{activity:?}").contains("fixture retry"))
    );
}

fn parse(input: &str) -> Vec<swallowtail_runtime::ActivityObservation> {
    let operation_id = ActivityOperationId::Run(
        RuntimeRunId::new("kimi-code-headless-activity-fixture").expect("valid run id"),
    );
    let mut parser = KimiHeadlessEventParser::new(operation_id);
    let mut events = parser.push(input.as_bytes()).expect("fixture parses");
    let (trailing, _) = parser.finish().expect("fixture finishes");
    events.extend(trailing);
    events
        .into_iter()
        .filter_map(|event| match event.kind() {
            RuntimeEventKind::Activity(activity) => Some(activity.clone()),
            _ => None,
        })
        .collect()
}
