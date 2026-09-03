//! Adversarial proofs that a stalled host service cannot hold a public
//! operation, and that host termination is never skipped.
//!
//! Each case hangs exactly one host future forever and fires the caller's
//! deadline. A repair that reintroduces an unbounded await fails here rather
//! than in production.

use crate::claude_agent_sdk_driver::lifecycle::assert_ordered;
use crate::host_id;
use crate::sdk_support::{
    CleanupEvent, SdkFixtureHost, SdkScenario, Stall, cleanup_request, prepared_session,
    turn_request,
};
use futures_executor::block_on;
use swallowtail_runtime::{CancellationAcknowledgement, CleanupOutcome, TerminalStatus};

/// Opens against a host whose named service never answers, with every deadline
/// already firing, and returns the failure plus the cleanup the guard reached.
fn stalled_open(stall: Stall) -> (String, Vec<CleanupEvent>) {
    let host = host_id("claude-agent-sdk.fixture.stalled-open");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete)
        .stalling(stall)
        .with_immediate_time();
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let Err(error) = block_on(prepared.open_session(services)) else {
        panic!("a stalled {stall:?} must not produce a session");
    };
    if matches!(stall, Stall::ForceStop) {
        // The guard's termination request lands in its own host task.
        fixture.wait_for_cleanup(CleanupEvent::ProcessForceStop);
    }
    (
        error.diagnostic().code().to_owned(),
        fixture.cleanup_events(),
    )
}

#[test]
fn a_stalled_credential_acquisition_still_returns_on_the_open_deadline() {
    let (code, _) = stalled_open(Stall::CredentialAcquire);
    assert!(
        code.starts_with("swallowtail.claude-agent.sdk.open_"),
        "unexpected diagnostic {code}"
    );
}

#[test]
fn a_stalled_resource_resolution_still_returns_on_the_open_deadline() {
    let (code, _) = stalled_open(Stall::ResourceResolve);
    assert!(code.starts_with("swallowtail.claude-agent.sdk.open_"));
}

#[test]
fn a_stalled_process_start_still_returns_on_the_open_deadline() {
    let (code, _) = stalled_open(Stall::ProcessStart);
    assert!(code.starts_with("swallowtail.claude-agent.sdk.open_"));
}

#[test]
fn a_stalled_open_cleanup_still_makes_the_termination_request() {
    // The process exists, then everything after it hangs. The guard owns the
    // termination request, so it happens even though the public future returns
    // at the caller's deadline.
    let (code, cleanup) = stalled_open(Stall::ForceStop);
    assert_eq!(
        code,
        "swallowtail.claude-agent.sdk.open_cleanup_unconfirmed"
    );
    assert!(
        cleanup.contains(&CleanupEvent::ProcessForceStop),
        "the descendant termination request must still be made: {cleanup:?}"
    );
    assert_eq!(
        code,
        "swallowtail.claude-agent.sdk.open_cleanup_unconfirmed"
    );
}

#[test]
fn a_close_response_that_never_arrives_still_terminates_the_tree() {
    // The sidecar accepts input and goes silent. Before this repair the close
    // future was dropped at the deadline while still awaiting that response,
    // and escalation never ran.
    let host = host_id("claude-agent-sdk.fixture.silent-close");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let services_for_cleanup = services.clone();
    let session = block_on(prepared.open_session(services)).expect("SDK sidecar session opens");

    fixture.hold_responses();
    fixture.fire_deadlines();
    let outcome = block_on(session.close(cleanup_request(), services_for_cleanup));
    // A missing native-join report is not a survivor observation, so the
    // outcome rests on host root evidence: root-only completion after the
    // declared termination attempt is the accepted degraded posture. What must
    // never happen is `Clean`, or a close that skipped termination entirely.
    assert!(
        !matches!(outcome, CleanupOutcome::Clean),
        "a silent close cannot be clean, got {outcome:?}"
    );
    // The guard owns the request, so it may land just after the public future
    // returns; what matters is that it lands.
    fixture.wait_for_cleanup(CleanupEvent::ProcessForceStop);
}

#[test]
fn an_accepted_turn_reaches_its_terminal_outcome_without_an_interrupt_response() {
    // The query was accepted, so a handle exists. The sidecar then stops
    // answering entirely. The terminal outcome must not wait on the interrupt
    // receipt that will never come.
    let host = host_id("claude-agent-sdk.fixture.accepted-no-interrupt");
    let fixture = SdkFixtureHost::new(SdkScenario::ToolAdmission);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let services_for_cleanup = services.clone();
    let mut session =
        block_on(prepared.open_session(services.clone())).expect("SDK sidecar session opens");
    let mut turn = block_on(session.start_turn(turn_request("turn-1", "read it"), services))
        .expect("SDK sidecar turn starts");

    fixture.hold_responses();
    fixture.fire_deadlines();
    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome exists"),
    );
    assert_eq!(terminal.status(), &TerminalStatus::TimedOut);
    let _ = block_on(turn.close());
    let _ = block_on(session.close(cleanup_request(), services_for_cleanup));
}

#[test]
fn a_stalled_wire_write_cannot_hold_turn_cancellation() {
    let host = host_id("claude-agent-sdk.fixture.stalled-write");
    let fixture = SdkFixtureHost::new(SdkScenario::ToolAdmission);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let services_for_cleanup = services.clone();
    let mut session =
        block_on(prepared.open_session(services.clone())).expect("SDK sidecar session opens");
    let turn = block_on(session.start_turn(turn_request("turn-1", "read it"), services))
        .expect("SDK sidecar turn starts");

    // Every later write hangs, including the interrupt this cancellation makes.
    fixture.stall_writes();
    fixture.fire_deadlines();
    assert_eq!(
        block_on(turn.cancellation().request()).expect("cancellation returns inside its bound"),
        CancellationAcknowledgement::Requested
    );
    let _ = block_on(session.close(cleanup_request(), services_for_cleanup));
}

#[test]
fn session_cancellation_makes_no_host_call_at_all() {
    // Session-scope cancellation carries no caller deadline, so it performs no
    // host await; close owns the bounded termination instead.
    let host = host_id("claude-agent-sdk.fixture.session-cancel");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete).stalling(Stall::ForceStop);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let services_for_cleanup = services.clone();
    let session = block_on(prepared.open_session(services)).expect("SDK sidecar session opens");

    fixture.fire_deadlines();
    assert_eq!(
        block_on(session.cancellation().request()).expect("session cancellation is local"),
        CancellationAcknowledgement::Requested
    );
    assert_eq!(
        block_on(session.cancellation().request()).expect("repeat is classified"),
        CancellationAcknowledgement::AlreadyRequested
    );
    assert!(
        !fixture
            .cleanup_events()
            .contains(&CleanupEvent::ProcessForceStop),
        "session cancellation must not make an unbounded host call"
    );
    // Termination is close's job, and close is bounded.
    let outcome = block_on(session.close(cleanup_request(), services_for_cleanup));
    assert!(matches!(outcome, CleanupOutcome::Failed(_)));
    fixture.wait_for_cleanup(CleanupEvent::ProcessForceStop);
}

#[test]
fn an_admission_request_the_turn_end_raced_is_denied_rather_than_fatal() {
    // The sidecar writes an admission request and the turn ends before it is
    // read. That is a race, not a protocol violation: the request is denied on
    // the wire, fail closed, and the transport stays usable for the close that
    // follows.
    let host = host_id("claude-agent-sdk.fixture.late-admission");
    let fixture = SdkFixtureHost::new(SdkScenario::AdmissionAfterResult);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let services_for_cleanup = services.clone();
    let mut session =
        block_on(prepared.open_session(services.clone())).expect("SDK sidecar session opens");
    let mut turn = block_on(session.start_turn(turn_request("turn-1", "read it"), services))
        .expect("SDK sidecar turn starts");
    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome exists"),
    );
    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);

    fixture.wait_for_input("the raced admission denial", |value| {
        value["type"] == "callback_response"
            && value["id"] == "cb-late"
            && value["decision"] == "deny"
    });
    // The transport was never poisoned: close still completes its own exchange.
    let outcome = block_on(session.close(cleanup_request(), services_for_cleanup));
    assert!(
        matches!(outcome, CleanupOutcome::Degraded(_)),
        "a raced admission must not turn close into a failure, got {outcome:?}"
    );
}

#[test]
fn a_stalled_open_returns_on_the_deadline_against_the_real_local_task_host() {
    // The same adversarial open, but the guard runs on `LocalScopedTaskService`
    // rather than the fixture's cooperative task seam. That host's handle owns
    // its worker thread: joining it blocks, and so does dropping it. If the
    // route ever waits on a join instead of the finished observation, this
    // public operation stops returning inside its deadline.
    let host = host_id("claude-agent-sdk.fixture.local-tasks");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete).stalling(Stall::ProcessStart);
    let prepared = prepared_session(host.clone());
    let services = fixture.services_with_local_tasks(host);
    fixture.fire_deadlines();

    let started = std::time::Instant::now();
    let Err(error) = block_on(prepared.open_session(services)) else {
        panic!("a stalled process start must fail on the host deadline");
    };
    let elapsed = started.elapsed();

    assert!(
        [
            "swallowtail.claude-agent.sdk.open_deadline_elapsed",
            "swallowtail.claude-agent.sdk.open_cleanup_unconfirmed",
        ]
        .contains(&error.diagnostic().code()),
        "unexpected open expiry diagnostic {}",
        error.diagnostic().code()
    );
    assert!(
        elapsed < std::time::Duration::from_secs(5),
        "open returned only after {elapsed:?}, so a real local join blocked it"
    );
}

#[test]
fn a_pump_that_outlives_process_exit_holds_both_lease_releases() {
    // Process exit wakes the pump but is not evidence that its host task has
    // run to completion. Contract 019 orders the scoped-work join before either
    // release, so neither lease may be released, and open may not report the
    // guard's cleanup as done, while the pump task is still alive.
    let host = host_id("claude-agent-sdk.fixture.pump-outlives");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete).stalling(Stall::PumpRead);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    fixture.fire_deadlines();

    let Err(error) = block_on(prepared.open_session(services)) else {
        panic!("a pump that never drains must fail open on the host deadline");
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.claude-agent.sdk.open_cleanup_unconfirmed",
        "an unjoined pump cannot be reported as a completed cleanup"
    );

    // The guard has terminated and waited; the join is where it now sits.
    fixture.wait_for_cleanup(CleanupEvent::ProcessWait);
    let cleanup = fixture.cleanup_events();
    assert!(
        !cleanup.contains(&CleanupEvent::ResourceRelease),
        "the resource lease was released before the pump was joined: {cleanup:?}"
    );
    assert!(
        !cleanup.contains(&CleanupEvent::CredentialRelease),
        "the credential lease was released before the pump was joined: {cleanup:?}"
    );

    // The guard still owns the whole ordered cleanup. Letting the pump end is
    // enough: no second call from the route, no later retention pass.
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
}
