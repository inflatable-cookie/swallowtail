use super::support::block_on;
use super::{operation_data, runtime_turn, watcher_host, watcher_owning_turn};
use std::time::Duration;
use swallowtail_core::{
    WatcherCleanupCause, WatcherLifecyclePhase, WatcherRequester, WatcherTerminalCause,
};
use swallowtail_runtime::{CleanupOutcome, WatcherWaitRepresentation};

#[test]
fn watcher_stop_is_idempotent_and_wait_reports_the_joined_stop() {
    let local = watcher_host("sleep", 2);
    let watcher = local
        .services()
        .watcher()
        .expect("local composition includes watcher");
    let turn = runtime_turn("turn-stop");
    let owning_turn = watcher_owning_turn("turn-stop");
    let watcher_id = block_on(watcher.accept_start(
        turn,
        WatcherRequester::Model,
        operation_data("sleep-operation"),
    ))
    .expect("stoppable watcher starts")
    .watcher_id()
    .clone();

    let (acknowledgement, stopped) =
        block_on(watcher.request_stop(owning_turn.clone(), watcher_id.clone()))
            .expect("first stop is accepted");
    assert_eq!(
        acknowledgement,
        swallowtail_runtime::WatcherStopAcknowledgement::Stopped
    );
    assert_eq!(stopped.phase(), WatcherLifecyclePhase::Terminal);
    assert_eq!(
        stopped.terminal_cause(),
        Some(WatcherTerminalCause::Stopped)
    );
    let (repeat, repeated_snapshot) =
        block_on(watcher.request_stop(owning_turn.clone(), watcher_id.clone()))
            .expect("repeated stop is idempotent");
    assert_eq!(
        repeat,
        swallowtail_runtime::WatcherStopAcknowledgement::AlreadyTerminal(
            WatcherTerminalCause::Stopped
        )
    );
    assert_eq!(repeated_snapshot.phase(), WatcherLifecyclePhase::Terminal);
    assert_eq!(
        block_on(watcher.wait(owning_turn, watcher_id)).expect("stop joins before wait resolves"),
        WatcherWaitRepresentation::Satisfied(WatcherTerminalCause::Stopped)
    );
}

#[test]
fn watcher_capacity_and_foreign_stop_fail_closed() {
    let local = watcher_host("sleep", 1);
    let watcher = local
        .services()
        .watcher()
        .expect("local composition includes watcher");
    let operation = operation_data("sleep-operation");
    let turn = runtime_turn("turn-capacity");
    let first =
        block_on(watcher.accept_start(turn.clone(), WatcherRequester::Operator, operation.clone()))
            .expect("first watcher starts");
    let capacity = block_on(watcher.accept_start(turn, WatcherRequester::Model, operation))
        .expect_err("second watcher must exceed the host turn bound");
    assert_eq!(
        capacity.diagnostic().code(),
        "swallowtail.local_watcher.capacity_rejected"
    );

    let foreign_started = block_on(watcher.accept_start(
        runtime_turn("turn-foreign"),
        WatcherRequester::Model,
        operation_data("sleep-operation"),
    ))
    .expect("foreign turn watcher starts");
    let foreign = block_on(watcher.request_stop(
        watcher_owning_turn("turn-foreign"),
        first.watcher_id().clone(),
    ))
    .expect_err("foreign owning turn cannot stop a watcher");
    assert_eq!(
        foreign.diagnostic().code(),
        "swallowtail.local_watcher.identity_rejected"
    );

    let (snapshots, cleanup) = block_on(watcher.stop_and_join_all(
        runtime_turn("turn-capacity"),
        WatcherCleanupCause::Cancelled,
    ))
    .expect("turn cleanup joins the owned watcher");
    assert_eq!(cleanup, CleanupOutcome::Clean);
    assert_eq!(snapshots.len(), 1);
    assert_eq!(snapshots[0].phase(), WatcherLifecyclePhase::Joined);
    assert_eq!(
        snapshots[0].terminal_cause(),
        Some(WatcherTerminalCause::Cancelled)
    );
    let (_, foreign_cleanup) = block_on(
        watcher.stop_and_join_all(runtime_turn("turn-foreign"), WatcherCleanupCause::Cancelled),
    )
    .expect("foreign turn cleanup joins its own watcher");
    assert_eq!(foreign_cleanup, CleanupOutcome::Clean);
    assert_eq!(foreign_started.accepted_by(), WatcherRequester::Model);
}

#[test]
fn watcher_stop_and_join_cleans_process_tree() {
    let local = watcher_host("spawn-long-descendant", 2);
    let watcher = local
        .services()
        .watcher()
        .expect("local composition includes watcher");
    let turn = runtime_turn("turn-tree");
    let owning_turn = watcher_owning_turn("turn-tree");
    let started = std::time::Instant::now();
    let watcher_id = block_on(watcher.accept_start(
        turn.clone(),
        WatcherRequester::Operator,
        operation_data("spawn-long-descendant-operation"),
    ))
    .expect("descendant watcher starts")
    .watcher_id()
    .clone();

    let (snapshots, cleanup) =
        block_on(watcher.stop_and_join_all(turn, WatcherCleanupCause::TimedOut))
            .expect("deadline cleanup stops and joins the process tree");
    assert_eq!(cleanup, CleanupOutcome::Clean);
    assert_eq!(snapshots[0].watcher_id(), &watcher_id);
    assert_eq!(snapshots[0].phase(), WatcherLifecyclePhase::Joined);
    assert_eq!(
        snapshots[0].terminal_cause(),
        Some(WatcherTerminalCause::TimedOut)
    );
    assert!(
        started.elapsed() < Duration::from_secs(4),
        "process-tree cleanup must not wait for the five-second descendant"
    );
    assert_eq!(
        block_on(watcher.wait(owning_turn, watcher_id)).expect("joined wait is repeatable"),
        WatcherWaitRepresentation::Satisfied(WatcherTerminalCause::TimedOut)
    );
}
