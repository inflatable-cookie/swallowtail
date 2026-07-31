use super::{ActiveTurn, MAX_ADMITTED_CHILD_THREADS};
use crate::rpc::RpcConnection;
use futures_executor::block_on;
use futures_util::StreamExt;
use serde_json::Value;
use std::collections::BTreeSet;
use std::sync::{Arc, Weak};
use swallowtail_core::ProviderRequestPolicy;
use swallowtail_runtime::{
    ActivityActor, BoxEventStream, CleanupOutcome, RuntimeEventKind, RuntimeTurnId, SubagentParent,
    TerminalStatus,
};

const CORPUS: &str = include_str!("../../tests/fixtures/activity/app-server.jsonl");

#[test]
fn root_spawn_admits_and_attributes_the_exact_child_until_termination() {
    let messages = child_owned_messages();
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
        .expect("established child-owned activity is accepted");

    let child_terminal = serde_json::json!({
        "threadId": "thread-child",
        "turn": {"id": "turn-fixture", "status": "completed"}
    });
    assert_session_mismatch(
        turn.handle_notification("turn/completed", &child_terminal)
            .expect_err("child thread cannot complete the root turn"),
    );

    turn.finish(TerminalStatus::Completed, CleanupOutcome::NotApplicable);
    assert!(turn.admitted_child_threads.lock().unwrap().is_empty());
    assert_session_mismatch(
        turn.handle_notification(method(&messages[1]), &messages[1]["params"])
            .expect_err("terminated operation does not retain child admission"),
    );

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
    assert_eq!(activities.len(), 2);
    assert_eq!(activities[0].actor(), &ActivityActor::Primary);
    let ActivityActor::Subagent(child) = activities[1].actor() else {
        panic!("child-owned envelope must carry child attribution");
    };
    assert_eq!(child.as_str(), "thread-child");
    assert_eq!(
        activities[1]
            .subagents()
            .next()
            .expect("child lifecycle snapshot is retained")
            .parent(),
        &SubagentParent::Unknown
    );
}

#[test]
fn unknown_and_cross_operation_child_threads_fail_closed() {
    let messages = child_owned_messages();
    let (first, _first_events) = turn("operation-first", "thread-fixture");
    let (second, _second_events) = turn("operation-second", "thread-fixture");

    assert_session_mismatch(
        first
            .handle_notification(method(&messages[1]), &messages[1]["params"])
            .expect_err("unknown child is rejected before spawn evidence"),
    );
    first
        .handle_notification(method(&messages[0]), &messages[0]["params"])
        .expect("first operation admits its child");
    assert_session_mismatch(
        second
            .handle_notification(method(&messages[1]), &messages[1]["params"])
            .expect_err("another operation cannot reuse child admission"),
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

fn child_owned_messages() -> Vec<Value> {
    CORPUS
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).expect("corpus line is valid"))
        .find(|case| case["case"] == "child-owned-envelope")
        .expect("child-owned envelope case exists")["messages"]
        .as_array()
        .expect("messages are an array")
        .clone()
}

fn method(message: &Value) -> &str {
    message["method"].as_str().expect("method is text")
}

fn assert_session_mismatch(error: swallowtail_runtime::RuntimeFailure) {
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.codex.app_server.session_id_mismatch"
    );
}
