use crate::host_id;
use crate::sdk_support::{
    CleanupEvent, SdkFixtureHost, SdkScenario, prepared_session, turn_request,
};
use futures_executor::block_on;
use swallowtail_runtime::{
    CancellationAcknowledgement, CleanupOutcome, RuntimeEventKind, TerminalStatus,
};

fn drain(events: &mut swallowtail_runtime::BoxEventStream) -> Vec<RuntimeEventKind> {
    use futures_util::StreamExt;
    let mut kinds = Vec::new();
    while let Some(event) = block_on(events.next()) {
        kinds.push(
            event
                .expect("fixture event stream stays healthy")
                .kind()
                .clone(),
        );
    }
    kinds
}

#[test]
fn open_streams_one_turn_and_closes_without_claiming_a_graceful_tree() {
    let host = host_id("claude-agent-sdk.fixture.complete");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let mut session =
        block_on(prepared.open_session(services.clone())).expect("SDK sidecar session opens");
    let mut turn = block_on(session.start_turn(turn_request("turn-1", "read it"), services))
        .expect("SDK sidecar turn starts");

    let mut events = turn.take_events().expect("turn exposes events");
    let kinds = drain(&mut events);
    assert!(
        kinds
            .iter()
            .any(|kind| matches!(kind, RuntimeEventKind::OutputDelta))
    );
    assert!(
        kinds
            .iter()
            .any(|kind| matches!(kind, RuntimeEventKind::Activity(_)))
    );
    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome exists"),
    );
    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);

    assert_eq!(fixture.credential_acquisitions(), 1);
    // Both known processes exited and the sidecar proved its own native join,
    // yet the host process API does not attest the owned tree was empty
    // beforehand, so close reports degraded rather than clean.
    let outcome = block_on(session.close());
    let CleanupOutcome::Degraded(diagnostic) = &outcome else {
        panic!("an unattested tree cannot close clean, got {outcome:?}");
    };
    assert_eq!(
        diagnostic.code(),
        "swallowtail.claude-agent.sdk.close_escalated_host_owned_tree_cleanup"
    );
    let cleanup = fixture.cleanup_events();
    assert!(
        !cleanup.contains(&CleanupEvent::ProcessForceStop),
        "a sidecar-joined native exit needs no driver-forced termination"
    );
    assert_ordered(
        &cleanup,
        &[
            CleanupEvent::ProcessWait,
            CleanupEvent::ResourceRelease,
            CleanupEvent::CredentialRelease,
        ],
    );
    let inputs = fixture.inputs();
    let close = inputs
        .iter()
        .find(|value| value["command"] == "close")
        .expect("close command is sent");
    assert_eq!(close["params"]["joinBoundMs"], 2000);
}

#[test]
fn an_unconfirmed_sidecar_join_escalates_through_host_authority() {
    let (outcome, cleanup) = close_with(SdkScenario::CloseUnconfirmed, true);
    let CleanupOutcome::Degraded(diagnostic) = &outcome else {
        panic!("escalated close is honest degradation, not a clean stop");
    };
    assert_eq!(
        diagnostic.code(),
        "swallowtail.claude-agent.sdk.close_escalated_host_termination"
    );
    assert_ordered(
        &cleanup,
        &[
            CleanupEvent::ProcessForceStop,
            CleanupEvent::ResourceRelease,
            CleanupEvent::CredentialRelease,
        ],
    );
    assert!(cleanup.contains(&CleanupEvent::ProcessWait));
}

#[test]
fn a_claimed_graceful_join_without_an_observed_exit_is_not_evidence() {
    // The nearest-child story alone is insufficient: a graceful claim that
    // carries no observation is rejected and the host still terminates the
    // whole descendant tree.
    let (outcome, cleanup) = close_with(SdkScenario::CloseGracefulWithoutObservation, true);
    let CleanupOutcome::Degraded(diagnostic) = &outcome else {
        panic!("an unproved join cannot close clean, got {outcome:?}");
    };
    assert_eq!(
        diagnostic.code(),
        "swallowtail.claude-agent.sdk.close_escalated_host_termination"
    );
    assert!(cleanup.contains(&CleanupEvent::ProcessForceStop));
}

#[test]
fn an_exit_that_is_never_observed_is_cleanup_failure() {
    let (outcome, cleanup) = close_with(SdkScenario::CloseUnconfirmed, false);
    assert!(
        matches!(outcome, CleanupOutcome::Failed(_)),
        "an unconfirmed exit is cleanup failure, never a slow success"
    );
    assert!(cleanup.contains(&CleanupEvent::ProcessForceStop));
    assert!(cleanup.contains(&CleanupEvent::ProcessWait));
    assert!(cleanup.contains(&CleanupEvent::CredentialRelease));
}

#[test]
fn session_cancellation_terminates_the_descendant_tree() {
    let host = host_id("claude-agent-sdk.fixture.cancel");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let session = block_on(prepared.open_session(services)).expect("SDK sidecar session opens");
    assert_eq!(
        block_on(session.cancellation().request()).expect("session cancellation succeeds"),
        CancellationAcknowledgement::Requested
    );
    assert_eq!(
        block_on(session.cancellation().request()).expect("repeat cancellation is classified"),
        CancellationAcknowledgement::AlreadyRequested
    );
    assert!(
        fixture
            .cleanup_events()
            .contains(&CleanupEvent::ProcessForceStop)
    );
    let _ = block_on(session.close());
}

fn close_with(scenario: SdkScenario, observable_exit: bool) -> (CleanupOutcome, Vec<CleanupEvent>) {
    let host = host_id("claude-agent-sdk.fixture.close");
    let fixture = SdkFixtureHost::new(scenario);
    let fixture = if observable_exit {
        fixture
    } else {
        fixture.without_observable_exit()
    };
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let session = block_on(prepared.open_session(services)).expect("SDK sidecar session opens");
    let outcome = block_on(session.close());
    (outcome, fixture.cleanup_events())
}

fn assert_ordered(events: &[CleanupEvent], expected: &[CleanupEvent]) {
    let positions: Vec<usize> = expected
        .iter()
        .map(|event| {
            events
                .iter()
                .position(|candidate| candidate == event)
                .unwrap_or_else(|| panic!("cleanup event {event:?} is missing from {events:?}"))
        })
        .collect();
    assert!(
        positions.windows(2).all(|window| window[0] < window[1]),
        "cleanup order {events:?} does not follow {expected:?}"
    );
}
