mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use support::{DriverCall, DriverFixture, ServerScenario};
use swallowtail_adapter_xai::XaiWebSocketDriver;
use swallowtail_runtime::{
    CleanupOutcome, InteractiveSessionDriver, OpenSessionRequest, OperationContent,
    ProviderObservation, RequestId, RuntimeEvent, RuntimeEventKind, RuntimeTurnId, TerminalOutcome,
    TerminalStatus, TurnHandle, TurnRequest,
};
use swallowtail_testkit::{
    ConformanceAssertion, ExecutionTopologyFixture, SyntheticProfile,
    run_connection_scoped_direct_session_profile,
};

#[test]
fn provider_neutral_profile_covers_connection_scoped_direct_sessions() {
    let report = run_connection_scoped_direct_session_profile();
    assert_eq!(
        report.profile(),
        SyntheticProfile::ConnectionScopedDirectSession
    );
    for assertion in [
        ConformanceAssertion::PreflightBeforeSideEffects,
        ConformanceAssertion::BoundSelection,
        ConformanceAssertion::OrderedEvents,
        ConformanceAssertion::SingleTerminalOutcome,
        ConformanceAssertion::CancellationAndTimeoutDistinct,
        ConformanceAssertion::Redaction,
        ConformanceAssertion::NoImplicitFallback,
        ConformanceAssertion::SessionLifecycle,
        ConformanceAssertion::HostTopologyPreserved,
        ConformanceAssertion::DirectSessionNoResource,
        ConformanceAssertion::ConnectionScopedLeaseLifecycle,
        ConformanceAssertion::BilledCostTurnScoped,
        ConformanceAssertion::NoImplicitSessionRecovery,
    ] {
        assert!(report.covers(assertion), "missing {assertion:?}");
    }
}

#[test]
fn xai_driver_preserves_local_and_remote_authority_through_chained_cleanup() {
    for topology in [
        ExecutionTopologyFixture::local(),
        ExecutionTopologyFixture::remote_authoritative(),
    ] {
        let fixture = DriverFixture::for_host(
            ServerScenario::Success,
            topology.execution_host_id().clone(),
        );
        let driver = XaiWebSocketDriver::new();
        let mut session = block_on(driver.open_session(
            fixture.plan(),
            OpenSessionRequest::resource_free(
                RequestId::new("conformance-session").expect("request id is valid"),
                None,
            ),
            fixture.services(),
        ))
        .expect("session opens");
        assert!(session.provider_session_ref().is_none());
        assert!(session.resume_binding().is_none());

        for (turn_id, expected_output, expected_cost) in [
            ("conformance-first", "First response.", 125_000),
            ("conformance-second", "Second response.", 175_000),
        ] {
            let mut turn = block_on(session.start_turn(
                TurnRequest::new(
                    RuntimeTurnId::new(turn_id).expect("turn id is valid"),
                    OperationContent::new("private conformance prompt").expect("content is valid"),
                ),
                fixture.services(),
            ))
            .expect("turn starts");
            let (events, outcome) = complete(&mut turn);
            assert_eq!(outcome.status(), &TerminalStatus::Completed);
            assert_eq!(
                outcome.output().expect("output exists").as_str(),
                expected_output
            );
            let cost = events.iter().find_map(|event| match event.kind() {
                RuntimeEventKind::ProviderObservation(ProviderObservation::BilledCost(cost)) => {
                    Some(cost)
                }
                _ => None,
            });
            let cost = cost.expect("turn-scoped cost exists");
            assert_eq!(cost.amount(), expected_cost);
            assert_eq!(cost.turn_id().as_str(), turn_id);
            assert_eq!(cost.provider_attempt().get(), 1);
            assert!(!format!("{events:?}").contains("private conformance prompt"));
            assert!(!format!("{outcome:?}").contains(expected_output));
            assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
        }

        assert_eq!(fixture.server.frames().len(), 2);
        assert!(fixture.server.frames()[1].contains("previous_response_id"));
        assert_eq!(fixture.calls.count(DriverCall::NetworkAuthorize), 1);
        assert_eq!(fixture.calls.count(DriverCall::CredentialAcquire), 1);
        assert_eq!(fixture.calls.count(DriverCall::CredentialRelease), 0);
        assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
        assert_eq!(fixture.calls.count(DriverCall::CredentialRelease), 1);
        let calls = fixture.calls.calls();
        assert!(last(&calls, DriverCall::TaskJoin) < last(&calls, DriverCall::CredentialRelease));
        assert!(
            last(&calls, DriverCall::BlockingWork) < last(&calls, DriverCall::CredentialRelease)
        );
    }
}

fn complete(turn: &mut Box<dyn TurnHandle>) -> (Vec<RuntimeEvent>, TerminalOutcome) {
    let mut stream = turn.take_events().expect("events are available");
    let terminal = turn
        .take_terminal_outcome()
        .expect("terminal outcome is available");
    block_on(async {
        let mut events = Vec::new();
        while let Some(event) = stream.next().await {
            events.push(event.expect("event is valid"));
        }
        (events, terminal.await)
    })
}

fn last(calls: &[DriverCall], expected: DriverCall) -> usize {
    calls
        .iter()
        .rposition(|call| *call == expected)
        .expect("expected driver call exists")
}
