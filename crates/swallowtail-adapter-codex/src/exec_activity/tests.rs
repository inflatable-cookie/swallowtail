use super::ExecActivityProjection;
use semver::Version;
use serde_json::Value;
use swallowtail_runtime::{
    ActivityAssistantPhase, ActivityKind, ActivityLifecyclePhase, RuntimeRunId,
};

const CORPUS: &str = include_str!("../../tests/fixtures/activity/exec.jsonl");

#[test]
fn qualified_corpus_projects_exact_exec_lifecycle_and_content() {
    let cases = cases();
    for name in [
        "agent-message-completion",
        "reasoning-summary-completion",
        "command-lifecycle",
        "file-change-completion",
        "mcp-tool-lifecycle",
        "search-lifecycle",
        "search-deferred-query-lifecycle",
        "collaboration-lifecycle",
        "todo-list-lifecycle",
        "warning-completion",
        "additive-event",
    ] {
        let observations =
            project(case(&cases, name), Version::new(0, 145, 0)).expect("qualified case projects");
        assert!(!observations.is_empty(), "{name} produced no activity");
    }

    let assistant = project(
        case(&cases, "agent-message-completion"),
        Version::new(0, 80, 0),
    )
    .unwrap();
    assert_eq!(
        assistant[0].assistant_phase(),
        Some(ActivityAssistantPhase::Final)
    );
    assert_eq!(assistant[0].phase(), ActivityLifecyclePhase::Completed);
    assert_eq!(
        assistant[0]
            .content()
            .expect("final answer content is visible")
            .content()
            .as_str(),
        "final answer"
    );
}

#[test]
fn deferred_search_query_attaches_content_at_completion() {
    let cases = cases();
    let search = project(
        case(&cases, "search-deferred-query-lifecycle"),
        Version::new(0, 145, 0),
    )
    .expect("0.146 search lifecycle remains compatible with the qualified profile");

    assert_eq!(
        search.iter().map(|item| item.phase()).collect::<Vec<_>>(),
        [
            ActivityLifecyclePhase::Started,
            ActivityLifecyclePhase::Completed
        ]
    );
    assert!(search[0].content().is_none());
    assert_eq!(
        search[1]
            .content()
            .expect("completed search carries its query")
            .content()
            .as_str(),
        "official Codex exec documentation"
    );
}

#[test]
fn mcp_label_stays_separate_from_result_payload() {
    let tool = project(
        case(&cases(), "mcp-tool-lifecycle"),
        Version::new(0, 145, 0),
    )
    .expect("MCP lifecycle projects");
    assert!(tool.iter().all(|observation| {
        observation
            .label()
            .is_some_and(|label| label.as_str() == "fixture.read")
    }));
    assert!(tool[0].content().is_none());
    assert_eq!(tool[1].content().unwrap().content().as_str(), "done");
}

#[test]
fn command_todo_and_unknown_events_keep_their_distinct_truth() {
    let cases = cases();
    let command = project(case(&cases, "command-lifecycle"), Version::new(0, 80, 0)).unwrap();
    assert_eq!(
        command.iter().map(|item| item.phase()).collect::<Vec<_>>(),
        [
            ActivityLifecyclePhase::Started,
            ActivityLifecyclePhase::Completed
        ]
    );
    assert!(
        command[1]
            .content()
            .unwrap()
            .content()
            .as_str()
            .contains("exit_status: 0")
    );

    let todo = project(case(&cases, "todo-list-lifecycle"), Version::new(0, 80, 0)).unwrap();
    assert_eq!(todo[1].phase(), ActivityLifecyclePhase::Updated);
    assert!(
        todo[1]
            .content()
            .unwrap()
            .content()
            .as_str()
            .contains("[ ] Test")
    );

    let unknown = project(case(&cases, "additive-event"), Version::new(0, 145, 0)).unwrap();
    assert!(matches!(unknown[0].kind(), ActivityKind::Unknown(_)));
    assert!(unknown[0].content().is_none());
}

#[test]
fn malformed_identity_and_unqualified_lifecycle_fail_closed() {
    let cases = cases();
    assert!(project(case(&cases, "malformed-item"), Version::new(0, 145, 0)).is_err());
    assert!(
        project(
            case(&cases, "collaboration-lifecycle"),
            Version::new(0, 80, 0)
        )
        .is_ok_and(|items| items
            .iter()
            .all(|item| matches!(item.kind(), ActivityKind::Unknown(_))))
    );

    let started_message = serde_json::json!({
        "type": "item.started",
        "item": {"id": "message", "type": "agent_message", "text": "not qualified"}
    });
    let mut projection = projector(Version::new(0, 145, 0));
    assert!(
        projection
            .project("item.started", &started_message)
            .is_err()
    );
}

fn project(
    case: &Value,
    version: Version,
) -> Result<Vec<swallowtail_runtime::ActivityObservation>, ()> {
    let mut projection = projector(version);
    let mut observations = Vec::new();
    for event in case["events"].as_array().expect("events are an array") {
        let event_type = event["type"].as_str().expect("event type is text");
        observations.extend(projection.project(event_type, event).map_err(|_| ())?);
    }
    Ok(observations)
}

fn projector(version: Version) -> ExecActivityProjection {
    ExecActivityProjection::new(RuntimeRunId::new("run-fixture").unwrap(), version)
}

fn cases() -> Vec<Value> {
    CORPUS
        .lines()
        .map(|line| serde_json::from_str(line).expect("corpus line is valid"))
        .collect()
}

fn case<'a>(cases: &'a [Value], name: &str) -> &'a Value {
    cases
        .iter()
        .find(|case| case["case"] == name)
        .expect("named case exists")
}
