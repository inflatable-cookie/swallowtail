//! Cancellation and rejection proofs for the `claude-agent.sdk` cleanup
//! continuation.
//!
//! A caller deadline is the easy case. These are the hard ones: the caller
//! drops a public future where it happens to be suspended, or the runtime
//! refuses the cleanup future before ever polling it. Ownership must already
//! sit with a guardian that hands itself to the owning host, because both
//! guards hold real `LocalJoinedTask` handles whose ordinary drop joins live
//! work on the dropping thread.

use crate::claude_agent_sdk_driver::lifecycle::assert_ordered;
use crate::host_id;
use crate::sdk_support::{
    CleanupEvent, SdkFixtureHost, SdkScenario, Stall, cleanup_request, drop_within,
    expired_cleanup_request, poll_once, prepared_session,
};
use futures_executor::block_on;
use swallowtail_runtime::CleanupOutcome;

#[test]
fn cancelling_a_pending_open_starts_its_ordered_cleanup_without_blocking() {
    // Caller cancellation is not a deadline: it drops the public future where
    // it is suspended, and the caller is owed an answer now, not at the open
    // deadline it just abandoned. By then the open guard already holds the
    // credential and the resource, and its task handle is a real
    // `LocalJoinedTask` whose drop joins. So the guard's drop must do two
    // things: release its own cleanup signal so the ordered continuation starts
    // at once, and hand the task to the owning host rather than join it.
    let host = host_id("claude-agent-sdk.fixture.open-cancel");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete).stalling(Stall::ProcessStart);
    let prepared = prepared_session(host.clone());
    let local = SdkFixtureHost::local_task_composition(host.clone());
    let services = fixture.services_with_local_tasks(host, &local);

    let mut opening = Box::pin(prepared.open_session(services));
    assert!(
        poll_once(opening.as_mut()).is_pending(),
        "the stalled process start must leave open suspended"
    );
    drop_within(
        "the cancelled open future",
        std::time::Duration::from_secs(5),
        opening,
    );

    // Cancellation itself starts the ordered cleanup. The original open
    // deadline is deliberately never fired: waiting for it would leave the
    // credential and the working resource held for the rest of the open budget.
    fixture.wait_for_cleanup(CleanupEvent::CredentialRelease);
    assert!(
        !fixture.deadlines_fired(),
        "the cleanup was caused by the open deadline arriving, not by cancellation"
    );
    assert_ordered(
        &fixture.cleanup_events(),
        &[
            CleanupEvent::ResourceRelease,
            CleanupEvent::CredentialRelease,
        ],
    );
    // The guard the host retained is the one that ran that cleanup, and the
    // outer owner is what joins it.
    local
        .shutdown_task_reapers()
        .expect("the outer host owner joins what it accepted");
}

#[test]
fn a_runtime_rejected_close_still_completes_the_ordered_cleanup() {
    // The runtime refuses an already-elapsed cleanup deadline before it ever
    // polls the cleanup future. Ownership must already have moved to the
    // enclosing guardian by then, or the process, the pump, and both leases
    // would be dropped outside any ordered continuation.
    let host = host_id("claude-agent-sdk.fixture.close-rejected");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let prepared = prepared_session(host.clone());
    let local = SdkFixtureHost::local_task_composition(host.clone());
    let services = fixture.services_with_local_tasks(host, &local);
    let services_for_cleanup = services.clone();
    let session = block_on(prepared.open_session(services)).expect("SDK sidecar session opens");
    fixture.hold_pump();

    let outcome = block_on(session.close(expired_cleanup_request(), services_for_cleanup));
    let CleanupOutcome::Failed(diagnostic) = &outcome else {
        panic!("an expired cleanup deadline fails, got {outcome:?}");
    };
    assert_eq!(
        diagnostic.code(),
        "swallowtail.session_cleanup.deadline_expired"
    );
    // Ownership moved before the runtime ever saw the future, so the guardian
    // is already running the *cooperative* continuation: the sidecar received
    // the explicit close command. A guardian activated only inside the
    // rejected future could not have sent it.
    fixture.wait_for_command("close");
    // Its cooperative stages are bounded by the same caller deadline, so
    // firing the host clock lets it reach termination.
    fixture.fire_deadlines();
    fixture.wait_for_cleanup(CleanupEvent::ProcessWait);
    let at_rejection = fixture.cleanup_events();
    assert!(
        !at_rejection.contains(&CleanupEvent::ResourceRelease)
            && !at_rejection.contains(&CleanupEvent::CredentialRelease),
        "a lease was released around the still-live pump: {at_rejection:?}"
    );

    // The guardian was never polled by the caller, yet it still owns and
    // finishes the whole continuation once the pump ends.
    fixture.release_pump();
    fixture.wait_for_cleanup(CleanupEvent::CredentialRelease);
    assert_ordered(
        &fixture.cleanup_events(),
        &[
            CleanupEvent::ProcessForceStop,
            CleanupEvent::ProcessWait,
            CleanupEvent::ResourceRelease,
            CleanupEvent::CredentialRelease,
        ],
    );
    local
        .shutdown_task_reapers()
        .expect("the outer host owner joins what it accepted");
}

#[test]
fn dropping_close_before_any_poll_still_completes_the_ordered_cleanup() {
    // The same ownership question without the runtime: a caller can build the
    // cleanup future and drop it without ever polling it.
    let host = host_id("claude-agent-sdk.fixture.close-unpolled");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let prepared = prepared_session(host.clone());
    let local = SdkFixtureHost::local_task_composition(host.clone());
    let services = fixture.services_with_local_tasks(host, &local);
    let services_for_cleanup = services.clone();
    let session = block_on(prepared.open_session(services)).expect("SDK sidecar session opens");
    fixture.hold_pump();

    let closing = session.close(cleanup_request(), services_for_cleanup);
    drop_within(
        "the unpolled close future",
        std::time::Duration::from_secs(5),
        closing,
    );

    // The guardian owned the continuation before the future existed, so the
    // cooperative close reached the sidecar even though nothing polled it.
    fixture.wait_for_command("close");
    fixture.fire_deadlines();
    fixture.wait_for_cleanup(CleanupEvent::ProcessWait);
    let after_drop = fixture.cleanup_events();
    assert!(
        !after_drop.contains(&CleanupEvent::ResourceRelease)
            && !after_drop.contains(&CleanupEvent::CredentialRelease),
        "a lease was released around the still-live pump: {after_drop:?}"
    );
    fixture.release_pump();
    fixture.wait_for_cleanup(CleanupEvent::CredentialRelease);
    assert_ordered(
        &fixture.cleanup_events(),
        &[
            CleanupEvent::ProcessForceStop,
            CleanupEvent::ProcessWait,
            CleanupEvent::ResourceRelease,
            CleanupEvent::CredentialRelease,
        ],
    );
    local
        .shutdown_task_reapers()
        .expect("the outer host owner joins what it accepted");
}

#[test]
fn dropping_close_after_one_pending_poll_hands_the_guardian_to_the_host() {
    // Cancellation mid-cleanup, with the pump still live. Dropping the future
    // drops the guardian, whose real `LocalJoinedTask` would synchronously join
    // the held pump. The reserved handoff is what keeps that non-blocking.
    let host = host_id("claude-agent-sdk.fixture.close-cancel");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let prepared = prepared_session(host.clone());
    let local = SdkFixtureHost::local_task_composition(host.clone());
    let services = fixture.services_with_local_tasks(host, &local);
    let services_for_cleanup = services.clone();
    let session = block_on(prepared.open_session(services)).expect("SDK sidecar session opens");
    fixture.hold_pump();

    let mut closing = Box::pin(session.close(cleanup_request(), services_for_cleanup));
    assert!(
        poll_once(closing.as_mut()).is_pending(),
        "a held pump must leave the ordered cleanup unfinished"
    );
    drop_within(
        "the cancelled close future",
        std::time::Duration::from_secs(5),
        closing,
    );
    fixture.wait_for_command("close");
    fixture.fire_deadlines();
    fixture.wait_for_cleanup(CleanupEvent::ProcessWait);
    let at_cancel = fixture.cleanup_events();
    assert!(
        !at_cancel.contains(&CleanupEvent::ResourceRelease)
            && !at_cancel.contains(&CleanupEvent::CredentialRelease),
        "a lease was released around the still-live pump: {at_cancel:?}"
    );

    fixture.release_pump();
    fixture.wait_for_cleanup(CleanupEvent::CredentialRelease);
    assert_ordered(
        &fixture.cleanup_events(),
        &[
            CleanupEvent::ProcessForceStop,
            CleanupEvent::ProcessWait,
            CleanupEvent::ResourceRelease,
            CleanupEvent::CredentialRelease,
        ],
    );
    local
        .shutdown_task_reapers()
        .expect("the outer host owner joins what it accepted");
}
