use super::ExecActivityProjection;
use semver::Version;
use serde_json::Value;
use swallowtail_runtime::{
    ActivityActor, ActivityAssistantPhase, ActivityKind, ActivityLifecyclePhase, RuntimeRunId,
    SubagentControlActionKind, SubagentParent, SubagentStatus,
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
        "search-other-navigation-lifecycle",
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
fn queryless_other_search_lifecycle_retains_identity_without_content() {
    let observations = project(
        case(&cases(), "search-other-navigation-lifecycle"),
        Version::new(0, 145, 0),
    )
    .expect("0.146 queryless navigation lifecycle projects");

    let search = &observations[..2];
    assert_eq!(
        search.iter().map(|item| item.phase()).collect::<Vec<_>>(),
        [
            ActivityLifecyclePhase::Started,
            ActivityLifecyclePhase::Completed
        ]
    );
    assert_eq!(search[0].activity_id(), search[1].activity_id());
    assert_eq!(
        search[0].provider_activity_ref(),
        search[1].provider_activity_ref()
    );
    assert!(search.iter().all(|item| item.content().is_none()));
}

#[test]
fn queryless_completed_actual_search_still_fails_closed() {
    let mut projection = projector(Version::new(0, 145, 0));
    let cases = cases();
    let event = &case(&cases, "malformed-completed-search-missing-query")["events"][0];

    assert!(projection.project("item.completed", event).is_err());
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
fn collaboration_keeps_child_topology_and_spawn_configuration() {
    let collaboration = project(
        case(&cases(), "collaboration-lifecycle"),
        Version::new(0, 145, 0),
    )
    .unwrap();
    let started = &collaboration[0];
    let child = started.subagents().next().unwrap();
    assert_eq!(started.actor(), &ActivityActor::Primary);
    assert_eq!(
        started.subagent_control(),
        Some(SubagentControlActionKind::Spawn)
    );
    assert_eq!(child.parent(), &SubagentParent::Operation);
    assert_eq!(child.status(), SubagentStatus::Pending);
    assert_eq!(child.description().unwrap().as_str(), "Inspect");
    assert_eq!(child.model().unwrap().as_str(), "gpt-fixture");
    assert_eq!(child.reasoning().unwrap().as_str(), "high");
    assert_eq!(
        collaboration[1].subagents().next().unwrap().status(),
        SubagentStatus::Completed
    );
}

#[test]
fn every_exec_collaboration_action_is_typed() {
    let mut projection = projector(Version::new(0, 145, 0));
    projection
        .project(
            "thread.started",
            &serde_json::json!({"type": "thread.started", "thread_id": "thread-fixture"}),
        )
        .unwrap();
    for (tool, expected) in [
        ("send_input", SubagentControlActionKind::SendInput),
        ("resume_agent", SubagentControlActionKind::Resume),
        ("wait", SubagentControlActionKind::Wait),
        ("close_agent", SubagentControlActionKind::Close),
    ] {
        let observations = projection
            .project(
                "item.completed",
                &serde_json::json!({
                    "type": "item.completed",
                    "item": {
                        "id": format!("control-{tool}"),
                        "type": "collab_tool_call",
                        "tool": tool,
                        "sender_thread_id": "thread-parent",
                        "receiver_thread_ids": ["thread-child"],
                        "prompt": "Continue",
                        "agents_states": {
                            "thread-child": {"status": "running", "message": null}
                        },
                        "status": "completed"
                    }
                }),
            )
            .unwrap();
        assert_eq!(observations[0].subagent_control(), Some(expected));
        assert!(matches!(
            observations[0].actor(),
            ActivityActor::Subagent(_)
        ));
        assert!(matches!(
            observations[0].subagents().next().unwrap().parent(),
            SubagentParent::Subagent(_)
        ));
    }
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
    let tasks = todo[1].task_list().unwrap().items().collect::<Vec<_>>();
    assert_eq!(tasks[0].content().as_str(), "Inspect");
    assert_eq!(
        tasks[0].status(),
        swallowtail_runtime::TaskListItemStatus::Completed
    );
    assert_eq!(
        tasks[1].status(),
        swallowtail_runtime::TaskListItemStatus::Pending
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
