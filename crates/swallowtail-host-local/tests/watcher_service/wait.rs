use super::support::block_on;
use super::{operation_data, runtime_turn, watcher_host, watcher_owning_turn};
use std::future::Future;
use std::task::{Context, Poll, Waker};
use std::time::Duration;
use swallowtail_core::{
    CancellationScope, WatcherCleanupCause, WatcherLifecyclePhase, WatcherRequester,
    WatcherTerminalCause,
};
use swallowtail_runtime::{CancellationControl, ImmediateCancellation};
use swallowtail_runtime::{CleanupOutcome, WatcherWaitOptions, WatcherWaitRepresentation};

#[test]
fn watcher_wait_returns_live_cancellation_before_join_and_cleanup_preserves_truth() {
    let local = watcher_host("sleep", 2);
    let watcher = local
        .services()
        .watcher()
        .expect("local composition includes watcher");
    let turn = runtime_turn("turn-wait-cancelled");
    let owning_turn = watcher_owning_turn("turn-wait-cancelled");
    let watcher_id = block_on(watcher.accept_start(
        turn.clone(),
        WatcherRequester::Model,
        operation_data("sleep-operation"),
    ))
    .expect("watcher starts")
    .watcher_id()
    .clone();
    let cancellation =
        std::sync::Arc::new(ImmediateCancellation::new(CancellationScope::ActiveTurn));
    let mut wait = Box::pin(watcher.wait(
        owning_turn,
        watcher_id,
        WatcherWaitOptions::new().with_cancellation(cancellation.wait_requested()),
    ));
    let waker = Waker::noop();
    let mut context = Context::from_waker(waker);
    assert!(matches!(wait.as_mut().poll(&mut context), Poll::Pending));

    block_on(cancellation.request()).expect("cancellation request is recorded");
    assert_eq!(
        block_on(wait).expect("live cancellation resolves the wait"),
        WatcherWaitRepresentation::Cancelled
    );

    let (snapshots, cleanup) =
        block_on(watcher.stop_and_join_all(turn, WatcherCleanupCause::Cancelled))
            .expect("cancellation cleanup joins the watcher");
    assert_eq!(cleanup, CleanupOutcome::Clean);
    assert_eq!(snapshots[0].phase(), WatcherLifecyclePhase::Joined);
    assert_eq!(
        snapshots[0].terminal_cause(),
        Some(WatcherTerminalCause::Cancelled)
    );
}

#[test]
fn watcher_wait_returns_live_deadline_before_join_and_cleanup_preserves_truth() {
    let local = watcher_host("hold", 2);
    let watcher = local
        .services()
        .watcher()
        .expect("local composition includes watcher");
    let turn = runtime_turn("turn-wait-deadline");
    let owning_turn = watcher_owning_turn("turn-wait-deadline");
    let watcher_id = block_on(watcher.accept_start(
        turn.clone(),
        WatcherRequester::Model,
        operation_data("hold-operation"),
    ))
    .expect("watcher starts")
    .watcher_id()
    .clone();
    let deadline = local.deadline_after(Duration::from_millis(50));
    let time = local
        .services()
        .time()
        .expect("local time service is present");
    let mut wait = Box::pin(watcher.wait(
        owning_turn,
        watcher_id,
        WatcherWaitOptions::new().with_deadline(time.wait_until(deadline)),
    ));
    let waker = Waker::noop();
    let mut context = Context::from_waker(waker);
    assert!(matches!(wait.as_mut().poll(&mut context), Poll::Pending));

    assert_eq!(
        block_on(wait).expect("live deadline resolves the wait"),
        WatcherWaitRepresentation::DeadlineExceeded
    );

    let (snapshots, cleanup) =
        block_on(watcher.stop_and_join_all(turn, WatcherCleanupCause::TimedOut))
            .expect("deadline cleanup joins the watcher");
    assert_eq!(cleanup, CleanupOutcome::Clean);
    assert_eq!(snapshots[0].phase(), WatcherLifecyclePhase::Joined);
    assert_eq!(
        snapshots[0].terminal_cause(),
        Some(WatcherTerminalCause::TimedOut)
    );
}
