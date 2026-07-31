use super::AppServerActivityProjection;
use crate::turn_state::canonical_provider_request_id;
use serde_json::Value;
use swallowtail_runtime::{
    ActivityActor, ActivityCorrelation, ActivityKind, ActivityLifecyclePhase, CallbackId,
    RuntimeTurnId, SubagentControlActionKind, SubagentParent, SubagentStatus,
};

const CORPUS: &str = include_str!("../../tests/fixtures/activity/app-server.jsonl");

#[test]
fn qualified_corpus_projects_rich_items_without_raw_reasoning() {
    let cases = cases();
    for name in [
        "item-lifecycle",
        "assistant-commentary",
        "assistant-final",
        "assistant-phase-unknown",
        "plan-replacement",
        "plan-item",
        "reasoning-summary",
        "command-output",
        "file-change-and-turn-diff",
        "deprecated-file-output",
        "mcp-tool",
        "collaboration",
        "child-owned-envelope",
        "subagent-activity",
        "search-image-review",
        "compaction-replacement",
        "compaction-deprecated",
        "hook-lifecycle",
        "hook-prompt",
        "task-list",
        "item-timestamps",
        "namespaced-unknown-item",
        "qualified-current-additive",
    ] {
        let observations = project(case(&cases, name)).expect("qualified case projects");
        assert!(!observations.is_empty(), "{name} produced no activity");
    }

    let raw = project(case(&cases, "raw-reasoning-excluded")).expect("raw case remains decodable");
    assert!(
        raw.iter()
            .flat_map(|observation| observation.content())
            .all(|content| !content.content().as_str().contains("private scratchpad"))
    );
    let unknown =
        project(case(&cases, "namespaced-unknown-item")).expect("unknown semantic item projects");
    assert!(matches!(unknown[0].kind(), ActivityKind::Unknown(_)));
}

#[test]
fn malformed_items_fail_and_completed_items_are_authoritative() {
    let cases = cases();
    assert!(project(case(&cases, "malformed-item")).is_err());

    let observations = project(case(&cases, "command-output")).expect("command case projects");
    let completed = observations
        .iter()
        .find(|observation| observation.phase() == ActivityLifecyclePhase::Completed)
        .expect("command completion is present");
    let content = completed
        .content()
        .expect("command completion carries bounded content")
        .content()
        .as_str();
    for expected in [
        "cargo check",
        "building\nok",
        "exit_status: 0",
        "duration_ms: 20",
    ] {
        assert!(
            content.contains(expected),
            "missing command detail {expected}"
        );
    }

    let file =
        project(case(&cases, "file-change-and-turn-diff")).expect("file change case projects");
    assert!(file.iter().any(|observation| {
        observation.content().is_some_and(|content| {
            content
                .content()
                .as_str()
                .contains("diff --git a/src/lib.rs")
        })
    }));
}

#[test]
fn plan_updates_carry_typed_authoritative_task_lists() {
    let observations = project(case(&cases(), "plan-replacement")).unwrap();
    assert_eq!(observations.len(), 2);
    let first = observations[0].task_list().unwrap();
    let first_items = first.items().collect::<Vec<_>>();
    assert_eq!(first_items[0].content().as_str(), "Inspect");
    assert_eq!(
        first_items[0].status(),
        swallowtail_runtime::TaskListItemStatus::InProgress
    );
    assert_eq!(
        first_items[1].status(),
        swallowtail_runtime::TaskListItemStatus::Pending
    );

    let revised = observations[1].task_list().unwrap();
    let revised_items = revised.items().collect::<Vec<_>>();
    assert_eq!(
        revised_items[0].status(),
        swallowtail_runtime::TaskListItemStatus::Completed
    );
    assert_eq!(revised_items[1].content().as_str(), "Implement safely");

    let cleared = projector()
        .project_notification(
            "turn/plan/updated",
            &serde_json::json!({
                "threadId": "thread-fixture",
                "turnId": "turn-fixture",
                "plan": []
            }),
        )
        .unwrap();
    assert!(cleared[0].content().is_none());
    assert!(cleared[0].task_list().unwrap().is_empty());
}

#[test]
fn tool_labels_stay_separate_from_progress_and_result_payloads() {
    let observations = project(case(&cases(), "mcp-tool")).expect("MCP case projects");
    assert!(observations.iter().all(|observation| {
        observation
            .label()
            .is_some_and(|label| label.as_str() == "fixture.read")
    }));
    assert!(observations[0].content().is_none());
    assert_eq!(
        observations[1].content().unwrap().content().as_str(),
        "Reading"
    );
    assert_eq!(
        observations[2].content().unwrap().content().as_str(),
        "done"
    );
}

#[test]
fn collaboration_keeps_child_topology_metadata_and_attribution() {
    let collaboration = project(case(&cases(), "collaboration")).unwrap();
    let started = &collaboration[0];
    let child = started
        .subagents()
        .next()
        .expect("spawned child is visible");
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

    let activity = project(case(&cases(), "subagent-activity")).unwrap();
    assert!(matches!(activity[0].actor(), ActivityActor::Subagent(_)));
    assert_eq!(
        activity[0].subagents().next().unwrap().status(),
        SubagentStatus::Running
    );
}

#[test]
fn every_provider_collaboration_action_is_typed_without_granting_control() {
    for (tool, expected) in [
        ("sendInput", SubagentControlActionKind::SendInput),
        ("resumeAgent", SubagentControlActionKind::Resume),
        ("wait", SubagentControlActionKind::Wait),
        ("closeAgent", SubagentControlActionKind::Close),
    ] {
        let mut projection = projector();
        let observations = projection
            .project_notification(
                "item/completed",
                &serde_json::json!({
                    "threadId": "thread-fixture",
                    "turnId": "turn-fixture",
                    "item": {
                        "id": format!("control-{tool}"),
                        "type": "collabAgentToolCall",
                        "tool": tool,
                        "status": "completed",
                        "senderThreadId": "thread-parent",
                        "receiverThreadIds": ["thread-child"],
                        "prompt": "Continue",
                        "agentsStates": {
                            "thread-child": {"status": "running", "message": null}
                        }
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
fn callbacks_and_provider_requests_remain_separate_correlations() {
    let mut projection = projector();
    let callback = CallbackId::new("callback-1").unwrap();
    projection.register_callback("call-1", callback.clone());
    let request_id = canonical_provider_request_id(&serde_json::json!("request-1")).unwrap();
    let request = request_id.clone();
    let request_activity = projection
        .provider_request_started(request_id, Some("call-1"), "dynamicTool")
        .expect("request activity starts");
    assert_eq!(
        request_activity.correlation(),
        Some(&ActivityCorrelation::ProviderRequest(request.clone()))
    );
    let request_activity_id = request_activity.activity_id().clone();

    let started = serde_json::json!({
        "item": {
            "id": "call-1",
            "type": "dynamicToolCall",
            "tool": "fixture_tool",
            "status": "inProgress",
            "contentItems": null,
            "success": null,
            "durationMs": null
        }
    });
    let tool = projection
        .project_notification("item/started", &started)
        .expect("tool activity starts");
    assert_eq!(
        tool[0].correlation(),
        Some(&ActivityCorrelation::Callback(callback))
    );
    assert_eq!(tool[0].label().unwrap().as_str(), "fixture_tool");
    assert!(tool[0].content().is_none());

    let resolved = serde_json::json!({"requestId": "request-1"});
    let resolution = projection
        .project_notification("serverRequest/resolved", &resolved)
        .expect("request resolution projects");
    assert_eq!(resolution[0].phase(), ActivityLifecyclePhase::Completed);
    assert!(matches!(resolution[0].kind(), ActivityKind::Unknown(_)));
    assert_eq!(resolution[0].activity_id(), &request_activity_id);
    assert_eq!(
        resolution[0].provider_activity_ref(),
        request_activity.provider_activity_ref()
    );
    assert_eq!(
        resolution[0].correlation(),
        Some(&ActivityCorrelation::ProviderRequest(request))
    );
    assert!(
        projection
            .project_notification("serverRequest/resolved", &resolved)
            .expect("duplicate resolution is unmatched")
            .is_empty()
    );
}

#[test]
fn request_id_union_fixture_completes_the_same_correlated_activity() {
    let cases = cases();
    let messages = case(&cases, "request-id-union")["messages"]
        .as_array()
        .expect("request-id messages are an array");
    let mut projection = projector();
    let mut started = Vec::new();
    for message in &messages[..2] {
        let request_id =
            canonical_provider_request_id(&message["id"]).expect("request id is legal");
        let request_ref = request_id.clone();
        let activity = projection
            .provider_request_started(
                request_id,
                message["params"]["itemId"].as_str(),
                "userInput",
            )
            .expect("request activity starts");
        assert_eq!(activity.phase(), ActivityLifecyclePhase::Started);
        started.push((activity, request_ref));
    }
    assert_eq!(
        started[0].1.as_provider_value(),
        started[1].1.as_provider_value()
    );
    assert_ne!(started[0].1, started[1].1);
    assert_eq!(
        started[0].1.representation(),
        swallowtail_core::ProviderRequestRepresentation::Text
    );
    assert_eq!(
        started[1].1.representation(),
        swallowtail_core::ProviderRequestRepresentation::SignedInteger
    );
    assert_ne!(started[0].0.activity_id(), started[1].0.activity_id());

    for (index, message) in messages[2..].iter().enumerate() {
        let completed = projection
            .project_notification(message["method"].as_str().unwrap(), &message["params"])
            .expect("request activity completes");

        assert_eq!(completed.len(), 1);
        assert_eq!(completed[0].phase(), ActivityLifecyclePhase::Completed);
        assert_eq!(completed[0].activity_id(), started[index].0.activity_id());
        assert_eq!(
            completed[0].provider_activity_ref(),
            started[index].0.provider_activity_ref()
        );
        assert_eq!(
            completed[0].correlation(),
            Some(&ActivityCorrelation::ProviderRequest(
                started[index].1.clone()
            ))
        );
    }
}

#[test]
fn request_id_normalization_rejects_non_protocol_shapes_and_ignores_unmatched_ids() {
    for request_id in [serde_json::json!("missing"), serde_json::json!(901)] {
        assert!(
            projector()
                .project_notification(
                    "serverRequest/resolved",
                    &serde_json::json!({"requestId": request_id}),
                )
                .expect("unmatched legal request id is harmless")
                .is_empty()
        );
    }

    for request_id in [
        Value::Null,
        serde_json::json!(true),
        serde_json::json!({"id": 1}),
        serde_json::json!([1]),
        serde_json::json!(1.5),
        serde_json::json!(u64::MAX),
    ] {
        assert!(canonical_provider_request_id(&request_id).is_err());
        assert!(
            projector()
                .project_notification(
                    "serverRequest/resolved",
                    &serde_json::json!({"requestId": request_id}),
                )
                .is_err()
        );
    }

    assert!(
        projector()
            .project_notification("serverRequest/resolved", &serde_json::json!({}))
            .is_err()
    );
}

fn project(case: &Value) -> Result<Vec<swallowtail_runtime::ActivityObservation>, ()> {
    let mut projection = projector();
    let mut observations = Vec::new();
    for message in case["messages"].as_array().expect("messages are an array") {
        let method = message["method"].as_str().expect("method is text");
        if message.get("id").is_some() {
            continue;
        }
        observations.extend(
            projection
                .project_notification(method, &message["params"])
                .map_err(|_| ())?,
        );
    }
    Ok(observations)
}

fn projector() -> AppServerActivityProjection {
    AppServerActivityProjection::new(
        RuntimeTurnId::new("turn-fixture").unwrap(),
        "thread-fixture".to_owned(),
    )
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
