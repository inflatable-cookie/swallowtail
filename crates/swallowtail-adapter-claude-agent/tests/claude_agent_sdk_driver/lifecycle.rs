use crate::host_id;
use crate::sdk_support::{
    CleanupEvent, SdkFixtureHost, SdkScenario, cleanup_request, expired_cleanup_request,
    prepared_session, turn_request,
};
use futures_executor::block_on;
use swallowtail_runtime::{CleanupOutcome, RuntimeEventKind, TerminalStatus};

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
fn open_streams_one_turn_and_closes_without_claiming_tree_completion() {
    let host = host_id("claude-agent-sdk.fixture.complete");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let services_for_cleanup = services.clone();
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
    // Both known processes exited and the sidecar observed its own native
    // child's exit, yet this host attests root completion only, so close
    // reports the accepted degraded posture rather than clean.
    let outcome = block_on(session.close(cleanup_request(), services_for_cleanup.clone()));
    let CleanupOutcome::Degraded(diagnostic) = &outcome else {
        panic!("an unattested tree cannot close clean, got {outcome:?}");
    };
    assert_eq!(
        diagnostic.code(),
        "swallowtail.claude-agent.sdk.close_root_only_degraded"
    );
    let cleanup = fixture.cleanup_events();
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
fn root_only_completion_is_degraded_never_clean() {
    // The accepted macOS posture: the host attests root completion only, so a
    // confirmed root exit after the declared termination attempt is degraded.
    let (outcome, cleanup) = close_with(SdkScenario::Complete, ExitEvidence::RootOnly);
    let CleanupOutcome::Degraded(diagnostic) = &outcome else {
        panic!("root-only completion is degraded, got {outcome:?}");
    };
    assert_eq!(
        diagnostic.code(),
        "swallowtail.claude-agent.sdk.close_root_only_degraded"
    );
    // The declared descendant termination attempt always precedes the claim,
    // and both leases are released after it in contract order.
    assert!(cleanup.contains(&CleanupEvent::ProcessWait));
    assert_ordered(
        &cleanup,
        &[
            CleanupEvent::ProcessForceStop,
            CleanupEvent::ResourceRelease,
            CleanupEvent::CredentialRelease,
        ],
    );
}

#[test]
fn only_attested_owned_tree_emptiness_reports_clean() {
    // No host in this repository can make this observation today. The path
    // exists so `Clean` stays reachable exactly once, from positive evidence.
    let (outcome, cleanup) = close_with(SdkScenario::Complete, ExitEvidence::OwnedTreeEmpty);
    assert_eq!(outcome, CleanupOutcome::Clean);
    assert!(cleanup.contains(&CleanupEvent::ProcessForceStop));
}

#[test]
fn an_observed_surviving_descendant_is_cleanup_failure() {
    // The sidecar's retained handle still shows a live native child. That is a
    // positive survivor observation and outranks the root's own exit.
    let (outcome, cleanup) = close_with(SdkScenario::NativeChildSurvives, ExitEvidence::RootOnly);
    let CleanupOutcome::Failed(diagnostic) = &outcome else {
        panic!("an observed survivor is cleanup failure, got {outcome:?}");
    };
    assert_eq!(
        diagnostic.code(),
        "swallowtail.claude-agent.sdk.close_descendant_survived"
    );
    assert!(cleanup.contains(&CleanupEvent::ProcessForceStop));
}

#[test]
fn a_survivor_outranks_even_attested_tree_emptiness() {
    let (outcome, _) = close_with(
        SdkScenario::NativeChildSurvives,
        ExitEvidence::OwnedTreeEmpty,
    );
    let CleanupOutcome::Failed(diagnostic) = &outcome else {
        panic!("a survivor cannot be overruled by an emptiness claim, got {outcome:?}");
    };
    assert_eq!(
        diagnostic.code(),
        "swallowtail.claude-agent.sdk.close_descendant_survived"
    );
}

#[test]
fn a_claimed_native_join_without_an_observation_is_not_evidence() {
    // The claim carries no observation, so it is discarded entirely and the
    // outcome rests on host root evidence alone.
    let (outcome, cleanup) = close_with(
        SdkScenario::NativeJoinWithoutObservation,
        ExitEvidence::RootOnly,
    );
    assert!(matches!(outcome, CleanupOutcome::Degraded(_)));
    assert!(cleanup.contains(&CleanupEvent::ProcessForceStop));
}

#[test]
fn an_unconfirmed_root_exit_is_cleanup_failure() {
    let (outcome, cleanup) = close_with(SdkScenario::Complete, ExitEvidence::Unobservable);
    let CleanupOutcome::Failed(diagnostic) = &outcome else {
        panic!("an unconfirmed root exit is cleanup failure, got {outcome:?}");
    };
    assert_eq!(
        diagnostic.code(),
        "swallowtail.claude-agent.sdk.close_root_unconfirmed"
    );
    assert!(cleanup.contains(&CleanupEvent::ProcessForceStop));
    assert!(cleanup.contains(&CleanupEvent::CredentialRelease));
}

#[test]
fn an_expired_cleanup_deadline_returns_bounded_failure() {
    // The caller's one deadline bounds the whole public close. An expired
    // deadline fails immediately instead of extending the future.
    let host = host_id("claude-agent-sdk.fixture.cleanup-deadline");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let services_for_cleanup = services.clone();
    let session = block_on(prepared.open_session(services)).expect("SDK sidecar session opens");
    let outcome = block_on(session.close(expired_cleanup_request(), services_for_cleanup));
    let CleanupOutcome::Failed(diagnostic) = &outcome else {
        panic!("an expired cleanup deadline fails, got {outcome:?}");
    };
    assert_eq!(
        diagnostic.code(),
        "swallowtail.session_cleanup.deadline_expired"
    );
}

/// How far the fixture execution host can prove completion behind the root.
#[derive(Clone, Copy, Debug)]
enum ExitEvidence {
    /// Root exit observed; owned descendants unattested. Every host in this
    /// repository reports this today.
    RootOnly,
    /// A hypothetical host that concretely observed its owned tree empty.
    OwnedTreeEmpty,
    /// No root exit observation at all.
    Unobservable,
}

fn close_with(
    scenario: SdkScenario,
    evidence: ExitEvidence,
) -> (CleanupOutcome, Vec<CleanupEvent>) {
    let host = host_id("claude-agent-sdk.fixture.close");
    let fixture = SdkFixtureHost::new(scenario);
    let fixture = match evidence {
        ExitEvidence::RootOnly => fixture,
        ExitEvidence::OwnedTreeEmpty => fixture.attesting_empty_owned_tree(),
        ExitEvidence::Unobservable => fixture.without_observable_exit(),
    };
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let services_for_cleanup = services.clone();
    let session = block_on(prepared.open_session(services)).expect("SDK sidecar session opens");
    let outcome = block_on(session.close(cleanup_request(), services_for_cleanup.clone()));
    (outcome, fixture.cleanup_events())
}

pub(crate) fn assert_ordered(events: &[CleanupEvent], expected: &[CleanupEvent]) {
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
