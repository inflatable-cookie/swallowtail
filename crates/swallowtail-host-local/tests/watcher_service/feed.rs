use super::{operation_data, runtime_turn, watcher_host};
use futures_executor::block_on;
use std::task::{Context, Poll, Waker};
use std::time::{Duration, Instant};
use swallowtail_core::{WatcherLifecyclePhase, WatcherRequester};
use swallowtail_runtime::{
    ActivityKind, ActivityLifecyclePhase, WatcherActivityProjection, project_watcher_activity,
};

#[test]
fn silent_fast_watcher_emits_accepted_running_and_terminal() {
    let local = watcher_host("exit-zero", 2);
    let watcher = local.services().watcher().expect("watcher").clone();
    let turn = runtime_turn("turn-fast");
    let mut feed = block_on(watcher.open_lifecycle_feed(turn.clone())).expect("feed");
    let _accepted = block_on(watcher.accept_start(
        turn.clone(),
        WatcherRequester::Operator,
        operation_data("exit-zero-operation"),
    ))
    .expect("fast watcher starts");
    let snapshots = drain_until_terminal(&mut feed, Duration::from_secs(2));
    let phases: Vec<_> = snapshots.iter().map(|snapshot| snapshot.phase()).collect();
    assert!(
        phases.contains(&WatcherLifecyclePhase::Accepted),
        "accepted was dropped: {phases:?}"
    );
    assert!(
        phases.contains(&WatcherLifecyclePhase::Running),
        "running was dropped: {phases:?}"
    );
    assert!(
        phases.contains(&WatcherLifecyclePhase::Terminal),
        "terminal was dropped: {phases:?}"
    );
    assert!(!phases.contains(&WatcherLifecyclePhase::Joined));
    let mut projections = Vec::new();
    for snapshot in &snapshots {
        match project_watcher_activity(&turn, snapshot).expect("projection") {
            WatcherActivityProjection::Activity(observation) => projections.push(*observation),
            WatcherActivityProjection::Joined { .. } => {
                panic!("joined must not emit completed activity")
            }
        }
    }
    assert_eq!(projections[0].kind(), &ActivityKind::HostWatcher);
    assert_eq!(projections[0].phase(), ActivityLifecyclePhase::Started);
    assert!(
        projections
            .iter()
            .any(|observation| observation.phase() == ActivityLifecyclePhase::Updated)
    );
    assert_eq!(
        projections.last().map(|observation| observation.phase()),
        Some(ActivityLifecyclePhase::Completed)
    );
}

#[test]
fn interleaved_watchers_preserve_host_order_without_regression() {
    let local = watcher_host("sleep", 4);
    let watcher = local.services().watcher().expect("watcher").clone();
    let turn = runtime_turn("turn-interleaved");
    let mut feed = block_on(watcher.open_lifecycle_feed(turn.clone())).expect("feed");
    let first = block_on(watcher.accept_start(
        turn.clone(),
        WatcherRequester::Model,
        operation_data("sleep-operation"),
    ))
    .expect("first starts");
    let second = block_on(watcher.accept_start(
        turn.clone(),
        WatcherRequester::Operator,
        operation_data("sleep-operation"),
    ))
    .expect("second starts");
    block_on(watcher.request_stop(
        swallowtail_core::WatcherOwningTurn::new(turn.as_str()).expect("owning"),
        first.watcher_id().clone(),
    ))
    .expect("stop first");
    block_on(watcher.request_stop(
        swallowtail_core::WatcherOwningTurn::new(turn.as_str()).expect("owning"),
        second.watcher_id().clone(),
    ))
    .expect("stop second");
    let snapshots = drain_until_count(&mut feed, 6, Duration::from_secs(2));
    let mut last = std::collections::BTreeMap::new();
    for snapshot in &snapshots {
        let identity = snapshot.watcher_id().as_str().to_owned();
        let revision = snapshot.revision().get();
        if let Some(previous) = last.insert(identity, revision) {
            assert!(revision > previous, "revision regressed");
        }
        match project_watcher_activity(&turn, snapshot).expect("projection") {
            WatcherActivityProjection::Joined { .. } => panic!("joined activity"),
            WatcherActivityProjection::Activity(_) => {}
        }
    }
    assert!(snapshots.len() >= 6);
}

fn drain_until_terminal(
    feed: &mut swallowtail_runtime::WatcherLifecycleSubscription,
    bound: Duration,
) -> Vec<swallowtail_runtime::WatcherSnapshot> {
    let started = Instant::now();
    let mut snapshots = Vec::new();
    while started.elapsed() < bound {
        snapshots.extend(try_drain(feed));
        if snapshots
            .iter()
            .any(|snapshot| snapshot.phase() == WatcherLifecyclePhase::Terminal)
        {
            return snapshots;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    snapshots
}

fn drain_until_count(
    feed: &mut swallowtail_runtime::WatcherLifecycleSubscription,
    count: usize,
    bound: Duration,
) -> Vec<swallowtail_runtime::WatcherSnapshot> {
    let started = Instant::now();
    let mut snapshots = Vec::new();
    while snapshots.len() < count && started.elapsed() < bound {
        snapshots.extend(try_drain(feed));
        if snapshots.len() >= count {
            break;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    snapshots
}

fn try_drain(
    feed: &mut swallowtail_runtime::WatcherLifecycleSubscription,
) -> Vec<swallowtail_runtime::WatcherSnapshot> {
    let waker = Waker::noop();
    let mut context = Context::from_waker(waker);
    let mut snapshots = Vec::new();
    loop {
        match feed.poll_snapshot(&mut context) {
            Poll::Ready(Some(Ok(snapshot))) => snapshots.push(snapshot),
            Poll::Ready(Some(Err(error))) => panic!("feed failed: {error}"),
            Poll::Ready(None) | Poll::Pending => break,
        }
    }
    snapshots
}
