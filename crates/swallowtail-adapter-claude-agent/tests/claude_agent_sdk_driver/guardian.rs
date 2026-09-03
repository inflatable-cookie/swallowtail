//! Enclosing-guardian proofs for the `claude-agent.sdk` cleanup continuation.
//!
//! Reap authority is taken before any effect, one guardian owns the whole
//! ordered continuation, and the caller's deadline transfers that guardian
//! rather than the pump. These are the three findings that rejected PR 188,
//! written as counterexamples rather than as claims.

use crate::claude_agent_sdk_driver::lifecycle::assert_ordered;
use crate::host_id;
use crate::sdk_support::{
    CleanupEvent, SdkFixtureHost, SdkScenario, cleanup_request, prepared_session,
};
use futures_executor::block_on;
use swallowtail_runtime::CleanupOutcome;

#[test]
fn an_unfinished_guardian_is_transferred_without_releasing_either_lease() {
    // At the caller's deadline the *enclosing guardian* is handed to the exact
    // host and scope that own it — not the pump on its own. Acceptance is
    // ownership transfer, never a join, so close reports unconfirmed cleanup,
    // and neither lease may be released while the transferred continuation is
    // still live.
    let host = host_id("claude-agent-sdk.fixture.relinquish");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let services_for_cleanup = services.clone();
    let session = block_on(prepared.open_session(services)).expect("SDK sidecar session opens");
    // The pump only stops draining after the session is open, so the close
    // deadline is what it outlives.
    fixture.hold_pump();
    fixture.fire_deadlines();

    let outcome = block_on(session.close(cleanup_request(), services_for_cleanup));
    let CleanupOutcome::Failed(diagnostic) = &outcome else {
        panic!("a transferred continuation cannot close clean: {outcome:?}");
    };
    assert_eq!(
        diagnostic.code(),
        "swallowtail.claude-agent.sdk.close_cleanup_unconfirmed"
    );

    let scopes = fixture.relinquished_scopes();
    assert!(
        scopes
            .iter()
            .any(|scope| scope.starts_with("claude-agent-sdk:close-guard:")),
        "the enclosing guardian was not transferred to its owning host: {scopes:?}"
    );
    assert!(
        !scopes
            .iter()
            .any(|scope| scope.starts_with("claude-agent-sdk:session:")),
        "the pump was transferred on its own instead of inside its guardian: {scopes:?}"
    );

    // Termination was requested, but nothing that the still-live pump uses has
    // been released.
    fixture.wait_for_cleanup(CleanupEvent::ProcessWait);
    let during = fixture.cleanup_events();
    assert!(
        !during.contains(&CleanupEvent::ResourceRelease)
            && !during.contains(&CleanupEvent::CredentialRelease),
        "a lease was released around still-live transferred work: {during:?}"
    );

    // The transferred guardian still owns the remainder. Letting the pump end
    // is enough: no second call into the route.
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
    // The outer host owner, never the route, joins what it accepted.
    fixture.reaper().shutdown();
}

#[test]
fn a_host_that_cannot_reserve_reap_is_refused_before_any_effect() {
    // Reap authority is taken before anything else exists. A host that cannot
    // commit the lane refuses here, with no credential acquired, no process
    // started, and no provider contact.
    let host = host_id("claude-agent-sdk.fixture.no-reservation");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    fixture.reaper().close_admission();

    let Err(error) = block_on(prepared.open_session(services)) else {
        panic!("a host without reap authority must not open a session");
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.fixture_task.unavailable",
        "the refusal must be the host's own, raised before any effect"
    );
    assert_eq!(
        fixture.credential_acquisitions(),
        0,
        "a credential was acquired before reap authority was granted"
    );
    assert!(
        fixture.cleanup_events().is_empty() && fixture.inputs().is_empty(),
        "the sidecar was contacted before reap authority was granted"
    );
}

#[test]
fn the_real_local_host_retains_transfers_and_reaps_the_close_guardian() {
    // The integrated deadline proof runs on the real `LocalHostServices` task
    // lifecycle: handles own their worker threads, join blocks, drop joins, and
    // the outer owner is the only thing that reaps. Nothing here is a fixture
    // that claims to reap a worker it does not own.
    let host = host_id("claude-agent-sdk.fixture.local-reap");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let prepared = prepared_session(host.clone());
    let local = SdkFixtureHost::local_task_composition(host.clone());
    let services = fixture.services_with_local_tasks(host, &local);
    let services_for_cleanup = services.clone();
    let session = block_on(prepared.open_session(services)).expect("SDK sidecar session opens");
    fixture.hold_pump();
    fixture.fire_deadlines();

    let started = std::time::Instant::now();
    let outcome = block_on(session.close(cleanup_request(), services_for_cleanup));
    let elapsed = started.elapsed();
    assert!(
        elapsed < std::time::Duration::from_secs(5),
        "close returned only after {elapsed:?}, so a real local handle blocked it"
    );
    let CleanupOutcome::Failed(diagnostic) = &outcome else {
        panic!("a transferred continuation cannot close clean: {outcome:?}");
    };
    assert_eq!(
        diagnostic.code(),
        "swallowtail.claude-agent.sdk.close_cleanup_unconfirmed"
    );
    let during = fixture.cleanup_events();
    assert!(
        !during.contains(&CleanupEvent::ResourceRelease)
            && !during.contains(&CleanupEvent::CredentialRelease),
        "a lease was released around still-live transferred work: {during:?}"
    );

    // The host still owns the worker. Letting the pump end lets the transferred
    // guardian finish its ordered cleanup, and the outer owner then joins it.
    fixture.release_pump();
    fixture.wait_for_cleanup(CleanupEvent::CredentialRelease);
    local
        .shutdown_task_reapers()
        .expect("the outer host owner joins what it accepted");
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
