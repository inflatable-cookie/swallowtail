mod fixture;
mod http;

use fixture::{fixture, fixture_with_wait_bound};
use futures_executor::block_on;
use http::{handshake, open, post, spin_until, start_body, tool_watcher_id, wait_body};
use std::thread;
use std::time::Duration;
use swallowtail_core::{SafeDiagnostic, WatcherCleanupCause};
use swallowtail_runtime::{CleanupOutcome, RuntimeFailure, WatcherBridgeHostService};

#[test]
fn lazy_start_holds_the_creating_guard_until_the_future_resolves() {
    let (bridge, watcher) = fixture("start-freeze");
    let lease = open(&bridge, "turn-race");
    let endpoint = lease.endpoint().expose().to_owned();
    let bearer = lease.bearer().expose().to_owned();
    handshake(&endpoint, &bearer);
    let start = thread::spawn({
        let endpoint = endpoint.clone();
        let bearer = bearer.clone();
        move || post(&endpoint, &bearer, start_body(2))
    });
    spin_until(&watcher.start_entered);
    let gate = thread::spawn(move || {
        let state = block_on(bridge.completion_gate(&lease)).expect("gate");
        (lease, state)
    });
    thread::sleep(Duration::from_millis(50));
    assert!(!gate.is_finished());
    watcher.start_hold.release();
    let (status, body) = start.join().expect("start thread");
    assert_eq!(status, 200, "{body}");
    let (lease, state) = gate.join().expect("gate thread");
    assert!(!state.allows_successful_completion());
    drop(lease);
}

#[test]
fn close_cancels_an_in_flight_wait_and_joins_it() {
    let (bridge, watcher) = fixture("close-wait");
    watcher.start_hold.release();
    let lease = open(&bridge, "turn-wait");
    let endpoint = lease.endpoint().expose().to_owned();
    let bearer = lease.bearer().expose().to_owned();
    handshake(&endpoint, &bearer);
    let (status, body) = post(&endpoint, &bearer, start_body(2));
    assert_eq!(status, 200, "{body}");
    let watcher_id = tool_watcher_id(&body);
    let wait = thread::spawn({
        let endpoint = endpoint.clone();
        let bearer = bearer.clone();
        move || post(&endpoint, &bearer, wait_body(3, &watcher_id))
    });
    spin_until(&watcher.wait_entered);
    let outcome = block_on(bridge.close(lease, WatcherCleanupCause::Cancelled)).expect("close");
    assert_eq!(outcome, CleanupOutcome::Clean);
    let (status, body) = wait.join().expect("wait thread");
    assert_eq!(status, 200, "{body}");
    assert!(body.contains("cancelled"), "{body}");
}

#[test]
fn close_completes_while_lazy_start_is_still_pending() {
    let (bridge, watcher) = fixture("close-pending-start");
    let lease = open(&bridge, "turn-pending-start");
    let endpoint = lease.endpoint().expose().to_owned();
    let bearer = lease.bearer().expose().to_owned();
    handshake(&endpoint, &bearer);
    let start = thread::spawn({
        let endpoint = endpoint.clone();
        let bearer = bearer.clone();
        move || post(&endpoint, &bearer, start_body(2))
    });
    spin_until(&watcher.start_entered);
    let _ = block_on(bridge.close(lease, WatcherCleanupCause::Cancelled)).expect("close");
    let (status, body) = start.join().expect("start thread");
    assert!(
        status == 200 || status == 0,
        "pending start must not block close: {status} {body}"
    );
}

#[test]
fn close_propagates_stop_and_join_failure() {
    let (bridge, watcher) = fixture("close-fail");
    watcher.start_hold.release();
    watcher.wait_hold.release();
    let lease = open(&bridge, "turn-fail");
    *watcher.stop_all_error.lock().expect("error") = Some(RuntimeFailure::new(
        SafeDiagnostic::new("fixture.bridge.stop_failed", "Fixture stop failed"),
    ));
    let failure = block_on(bridge.close(lease, WatcherCleanupCause::Failed))
        .expect_err("close surfaces host failure");
    assert_eq!(failure.diagnostic().code(), "fixture.bridge.stop_failed");
}

#[test]
fn lazy_wait_resolves_deadline_after_the_future_completes() {
    let (bridge, watcher) = fixture_with_wait_bound("deadline", Duration::from_millis(80));
    watcher.start_hold.release();
    let lease = open(&bridge, "turn-deadline");
    let endpoint = lease.endpoint().expose().to_owned();
    let bearer = lease.bearer().expose().to_owned();
    handshake(&endpoint, &bearer);
    let (status, body) = post(&endpoint, &bearer, start_body(2));
    assert_eq!(status, 200, "{body}");
    let watcher_id = tool_watcher_id(&body);
    let wait = thread::spawn({
        let endpoint = endpoint.clone();
        let bearer = bearer.clone();
        move || post(&endpoint, &bearer, wait_body(3, &watcher_id))
    });
    spin_until(&watcher.wait_entered);
    assert!(!wait.is_finished());
    let (status, body) = wait.join().expect("wait thread");
    assert_eq!(status, 200, "{body}");
    assert!(body.contains("deadline_exceeded"), "{body}");
    drop(lease);
}
