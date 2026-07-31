use super::{ActiveTurn, MAX_ADMITTED_CHILD_THREADS};
use crate::rpc::RpcConnection;
use futures_executor::block_on;
use futures_util::StreamExt;
use serde_json::Value;
use std::collections::BTreeSet;
use std::sync::{Arc, Weak};
use swallowtail_core::ProviderRequestPolicy;
use swallowtail_runtime::{
    ActivityActor, ActivityLifecyclePhase, ActivityStatus, BoxEventStream, CleanupOutcome,
    RuntimeEventKind, RuntimeTurnId, SubagentParent, SubagentStatus, TerminalStatus,
};

const CORPUS: &str = include_str!("../../tests/fixtures/activity/app-server.jsonl");

#[test]
fn admitted_child_lifecycle_is_observed_without_mutating_the_root_turn() {
    let messages = child_lifecycle_messages();
    let (turn, mut events) = turn("operation-a", "thread-fixture");

    turn.handle_notification(method(&messages[0]), &messages[0]["params"])
        .expect("root-owned spawn activity is accepted");
    assert!(
        turn.admitted_child_threads
            .lock()
            .unwrap()
            .contains("thread-child")
    );
    turn.handle_notification(method(&messages[1]), &messages[1]["params"])
        .expect("established child lifecycle starts");
    assert_eq!(
        turn.active_child_turns
            .lock()
            .unwrap()
            .get("thread-child")
            .map(String::as_str),
        Some("turn-child")
    );
    assert_eq!(
        turn.provider_id.lock().unwrap().as_deref(),
        Some("turn-fixture")
    );

    turn.handle_notification(method(&messages[2]), &messages[2]["params"])
        .expect("child activity matches its child-local turn");
    turn.handle_notification(method(&messages[3]), &messages[3]["params"])
        .expect("child completion remains observational");
    assert!(!turn.is_finished());
    assert!(turn.active_child_turns.lock().unwrap().is_empty());
    assert_eq!(
        turn.provider_id.lock().unwrap().as_deref(),
        Some("turn-fixture")
    );
    assert_child_turn_mismatch(
        turn.handle_notification(method(&messages[2]), &messages[2]["params"])
            .expect_err("completed child turn cannot admit stale activity"),
    );
    assert_child_turn_mismatch(
        turn.handle_notification(method(&messages[3]), &messages[3]["params"])
            .expect_err("completed child lifecycle cannot be replayed"),
    );

    turn.handle_notification(
        "turn/completed",
        &serde_json::json!({
            "threadId": "thread-fixture",
            "turn": {"id": "turn-fixture", "status": "completed"}
        }),
    )
    .expect("root completion retains root authority");
    assert!(turn.is_finished());
    assert!(turn.admitted_child_threads.lock().unwrap().is_empty());
    assert!(turn.active_child_turns.lock().unwrap().is_empty());

    let emitted = block_on(async move {
        let mut emitted = Vec::new();
        while let Some(event) = events.next().await {
            emitted.push(event.expect("runtime event is valid"));
        }
        emitted
    });
    let activities = emitted
        .iter()
        .filter_map(|event| match event.kind() {
            RuntimeEventKind::Activity(activity) => Some(activity),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(activities.len(), 4);
    assert_eq!(activities[0].actor(), &ActivityActor::Primary);
    let ActivityActor::Subagent(child) = activities[1].actor() else {
        panic!("child-owned envelope must carry child attribution");
    };
    assert_eq!(child.as_str(), "thread-child");
    assert_eq!(activities[1].phase(), ActivityLifecyclePhase::Started);
    assert_eq!(activities[1].status(), ActivityStatus::InProgress);
    assert_eq!(activities[1].activity_id(), activities[3].activity_id());
    assert_eq!(activities[3].phase(), ActivityLifecyclePhase::Completed);
    assert_eq!(activities[3].status(), ActivityStatus::Completed);
    assert_eq!(
        activities[1].provider_activity_ref(),
        activities[3].provider_activity_ref()
    );
    assert_eq!(
        activities[1]
            .subagents()
            .next()
            .expect("child lifecycle snapshot is retained")
            .parent(),
        &SubagentParent::Unknown
    );
    assert_eq!(
        activities[1].subagents().next().unwrap().status(),
        SubagentStatus::Running
    );
    assert_eq!(
        activities[3].subagents().next().unwrap().status(),
        SubagentStatus::Completed
    );
    assert!(matches!(activities[2].actor(), ActivityActor::Subagent(_)));
}

#[test]
fn root_lifecycle_remains_root_owned_and_terminal() {
    let (turn, _events) = turn("operation-root", "thread-fixture");
    turn.handle_notification(
        "turn/started",
        &serde_json::json!({
            "threadId": "thread-fixture",
            "turn": {"id": "turn-root", "status": "inProgress"}
        }),
    )
    .unwrap();
    assert_eq!(
        turn.provider_id.lock().unwrap().as_deref(),
        Some("turn-root")
    );
    turn.handle_notification(
        "turn/completed",
        &serde_json::json!({
            "threadId": "thread-fixture",
            "turn": {"id": "turn-root", "status": "completed"}
        }),
    )
    .unwrap();
    assert!(turn.is_finished());
}

#[test]
fn unknown_cross_operation_mismatched_and_post_terminal_children_fail_closed() {
    let messages = child_lifecycle_messages();
    let (first, _first_events) = turn("operation-first", "thread-fixture");
    let (second, _second_events) = turn("operation-second", "thread-fixture");

    assert_lifecycle_owner_mismatch(
        first
            .handle_notification(method(&messages[1]), &messages[1]["params"])
            .expect_err("unknown child lifecycle is rejected before spawn evidence"),
    );
    first
        .handle_notification(method(&messages[0]), &messages[0]["params"])
        .expect("first operation admits its child");
    assert_lifecycle_owner_mismatch(
        second
            .handle_notification(method(&messages[1]), &messages[1]["params"])
            .expect_err("another operation cannot reuse child lifecycle admission"),
    );
    first
        .handle_notification(method(&messages[1]), &messages[1]["params"])
        .expect("first operation starts its child turn");
    let mismatched = serde_json::json!({
        "threadId": "thread-child",
        "turnId": "turn-other",
        "item": {"id": "child-other", "type": "contextCompaction"}
    });
    assert_child_turn_mismatch(
        first
            .handle_notification("item/completed", &mismatched)
            .expect_err("child activity cannot switch child turn ids"),
    );

    first.finish(TerminalStatus::Completed, CleanupOutcome::NotApplicable);
    assert_child_lifecycle_after_terminal(
        first
            .handle_notification(method(&messages[1]), &messages[1]["params"])
            .expect_err("terminated operation does not retain child admission"),
    );

    let (bounded, _bounded_events) = turn("operation-bounded", "thread-fixture");
    bounded
        .admitted_child_threads
        .lock()
        .unwrap()
        .extend((0..MAX_ADMITTED_CHILD_THREADS).map(|index| format!("existing-child-{index}")));
    let error = bounded
        .handle_notification(method(&messages[0]), &messages[0]["params"])
        .expect_err("operation child admission stays bounded");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.codex.app_server.child_thread_limit_exceeded"
    );
    let admitted = bounded.admitted_child_threads.lock().unwrap();
    assert_eq!(admitted.len(), MAX_ADMITTED_CHILD_THREADS);
    assert!(!admitted.contains("thread-child"));
}

#[test]
fn child_error_does_not_fail_the_root_operation() {
    let messages = child_lifecycle_messages();
    let (turn, _events) = turn("operation-child-error", "thread-fixture");
    turn.handle_notification(method(&messages[0]), &messages[0]["params"])
        .unwrap();
    turn.handle_notification(method(&messages[1]), &messages[1]["params"])
        .unwrap();

    turn.handle_notification(
        "error",
        &serde_json::json!({
            "threadId": "thread-child",
            "turnId": "turn-child",
            "willRetry": true,
            "error": {"message": "redacted fixture"}
        }),
    )
    .expect("known child errors remain observational");
    assert!(!turn.is_finished());
    assert_eq!(
        turn.provider_id.lock().unwrap().as_deref(),
        Some("turn-fixture")
    );

    let child_owned = serde_json::json!({
        "threadId": "thread-child",
        "turnId": "turn-child"
    });
    assert_session_mismatch(
        turn.verify_turn(&child_owned)
            .expect_err("child lifecycle does not grant root turn authority"),
    );
    assert_session_mismatch(
        turn.verify_provider_request(&child_owned)
            .expect_err("child lifecycle does not grant provider-request authority"),
    );
}

#[test]
fn child_failed_completion_does_not_fail_the_root_operation() {
    let messages = child_lifecycle_messages();
    let (turn, _events) = turn("operation-child-failure", "thread-fixture");
    turn.handle_notification(method(&messages[0]), &messages[0]["params"])
        .unwrap();
    turn.handle_notification(method(&messages[1]), &messages[1]["params"])
        .unwrap();

    let failed = serde_json::json!({
        "threadId": "thread-child",
        "turn": {"id": "turn-child", "status": "failed"}
    });
    turn.handle_notification("turn/completed", &failed)
        .expect("child failure remains observational");
    assert!(!turn.is_finished());
    assert!(turn.active_child_turns.lock().unwrap().is_empty());
    assert_eq!(
        turn.provider_id.lock().unwrap().as_deref(),
        Some("turn-fixture")
    );
}

fn turn(name: &str, root_thread: &str) -> (Arc<ActiveTurn>, BoxEventStream) {
    let (turn, events, _callbacks, _terminal) = ActiveTurn::new(
        RuntimeTurnId::new(name).expect("runtime turn id is valid"),
        None,
        BTreeSet::new(),
        ProviderRequestPolicy::reject_all(),
        root_thread.to_owned(),
        Weak::<RpcConnection>::new(),
    )
    .expect("turn state is valid");
    (turn, events)
}

fn child_lifecycle_messages() -> Vec<Value> {
    CORPUS
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).expect("corpus line is valid"))
        .find(|case| case["case"] == "child-turn-lifecycle")
        .expect("child lifecycle case exists")["messages"]
        .as_array()
        .expect("messages are an array")
        .clone()
}

fn method(message: &Value) -> &str {
    message["method"].as_str().expect("method is text")
}

fn assert_lifecycle_owner_mismatch(error: swallowtail_runtime::RuntimeFailure) {
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.codex.app_server.lifecycle_owner_mismatch"
    );
}

fn assert_child_turn_mismatch(error: swallowtail_runtime::RuntimeFailure) {
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.codex.app_server.child_turn_id_mismatch"
    );
}

fn assert_child_lifecycle_after_terminal(error: swallowtail_runtime::RuntimeFailure) {
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.codex.app_server.child_lifecycle_after_terminal"
    );
}

fn assert_session_mismatch(error: swallowtail_runtime::RuntimeFailure) {
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.codex.app_server.session_id_mismatch"
    );
}
