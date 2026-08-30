use super::LifecycleBuffer;
use swallowtail_core::{WatcherOperationData, WatcherRequester};
use swallowtail_runtime::{RuntimeTurnId, WatcherRegistry};

fn snapshots() -> (
    swallowtail_runtime::WatcherSnapshot,
    swallowtail_runtime::WatcherSnapshot,
) {
    let mut registry =
        WatcherRegistry::new(RuntimeTurnId::new("turn-feed-buffer").expect("turn"), 2)
            .expect("registry");
    let accepted = registry
        .accept_start(
            WatcherRequester::Operator,
            WatcherOperationData::new("exit-zero-operation").expect("operation"),
        )
        .expect("accepted");
    let running = registry
        .mark_running(accepted.watcher_id())
        .expect("running");
    (accepted, running)
}

#[test]
fn overflow_fails_closed() {
    let mut buffer = LifecycleBuffer::new(1);
    let (accepted, running) = snapshots();
    buffer.push(accepted).expect("first retained");
    let error = buffer.push(running).expect_err("overflow");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.local_watcher.lifecycle_feed_overflow"
    );
    let later = snapshots().0;
    let closed = buffer.push(later).expect_err("already failed");
    assert_eq!(
        closed.diagnostic().code(),
        "swallowtail.local_watcher.lifecycle_feed_overflow"
    );
}

#[test]
fn closed_feed_rejects_later_snapshots() {
    let mut buffer = LifecycleBuffer::new(4);
    buffer.close();
    let error = buffer.push(snapshots().0).expect_err("closed");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.local_watcher.lifecycle_feed_closed"
    );
}

#[test]
fn regressing_revision_fails_closed() {
    let mut buffer = LifecycleBuffer::new(4);
    let (accepted, running) = snapshots();
    buffer.push(running).expect("higher revision");
    let error = buffer.push(accepted).expect_err("regression");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.local_watcher.lifecycle_feed_regression"
    );
}
