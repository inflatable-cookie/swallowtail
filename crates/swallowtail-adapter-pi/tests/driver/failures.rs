use crate::support::{FixtureHost, Scenario, open_request, selection_for_topology, turn_request};
use futures_executor::block_on;
use swallowtail_adapter_pi::PiRpcDriver;
use swallowtail_runtime::{
    CleanupOutcome, Deadline, EnvironmentRef, InteractiveSessionDriver, MonotonicInstant,
    TerminalOutcome, TerminalStatus,
};
use swallowtail_testkit::ExecutionTopologyFixture;

#[test]
fn provider_retry_disconnect_and_protocol_failures_remain_distinct_in_both_topologies() {
    for topology in [
        ExecutionTopologyFixture::local(),
        ExecutionTopologyFixture::remote_authoritative(),
    ] {
        for (scenario, expected_code, provider_failure) in [
            (
                Scenario::ProviderFailure,
                "swallowtail.pi.rpc.provider_failed",
                true,
            ),
            (
                Scenario::RetryDrift,
                "swallowtail.pi.rpc.retry_policy_drift",
                false,
            ),
            (
                Scenario::Disconnect,
                "swallowtail.pi.rpc.connection_ended",
                false,
            ),
            (
                Scenario::Malformed,
                "swallowtail.pi.rpc.protocol_failed",
                false,
            ),
        ] {
            let (outcome, cleanup) = terminal_case(&topology, scenario, expected_code);
            assert_eq!(cleanup, CleanupOutcome::Clean);
            let diagnostic = match outcome.status() {
                TerminalStatus::ProviderFailed(diagnostic) if provider_failure => diagnostic,
                TerminalStatus::RuntimeFailed(diagnostic) if !provider_failure => diagnostic,
                status => panic!("unexpected Pi terminal status: {status:?}"),
            };
            assert_eq!(diagnostic.code(), expected_code);
            let public = format!("{outcome:?}");
            assert!(!public.contains("fixture provider secret"));
            assert!(!public.contains("fixture private prompt"));
        }
    }
}

#[test]
fn correlation_drift_fails_the_command_and_still_joins() {
    let topology = ExecutionTopologyFixture::local();
    let fixture = FixtureHost::new(Scenario::ResponseMismatch);
    let selected = selection_for_topology(&topology);
    let services = fixture.services(topology.execution_host_id().clone());
    let mut session = block_on(driver(selected.credential.clone()).open_session(
        selected.plan,
        open_request("correlation-session", selected.resource),
        services.clone(),
    ))
    .expect("Pi session opens");
    let error =
        block_on(session.start_turn(turn_request("correlation-turn", deadline()), services))
            .err()
            .expect("mismatched response fails");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.pi.rpc.response_command_mismatch"
    );
    assert!(!format!("{error:?}").contains("fixture private prompt"));
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

#[test]
fn active_and_completed_prompt_bounds_fail_without_reclassification() {
    let topology = ExecutionTopologyFixture::local();
    let fixture = FixtureHost::new(Scenario::Hold);
    let selected = selection_for_topology(&topology);
    let services = fixture.services(topology.execution_host_id().clone());
    let mut session = block_on(driver(selected.credential.clone()).open_session(
        selected.plan,
        open_request("busy-session", selected.resource),
        services.clone(),
    ))
    .expect("Pi session opens");
    let turn =
        block_on(session.start_turn(turn_request("busy-turn-1", deadline()), services.clone()))
            .expect("first Pi turn starts");
    let error = block_on(session.start_turn(turn_request("busy-turn-2", deadline()), services))
        .err()
        .expect("parallel prompt fails");
    assert_eq!(error.diagnostic().code(), "swallowtail.pi.rpc.turn_active");
    assert!(block_on(turn.close()) == CleanupOutcome::NotApplicable);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);

    let fixture = FixtureHost::new(Scenario::Complete);
    let selected = selection_for_topology(&topology);
    let services = fixture.services(topology.execution_host_id().clone());
    let mut session = block_on(driver(selected.credential.clone()).open_session(
        selected.plan,
        open_request("limit-session", selected.resource),
        services.clone(),
    ))
    .expect("Pi limit session opens");
    for index in 0..2 {
        let mut turn = block_on(session.start_turn(
            turn_request(&format!("limit-turn-{index}"), deadline()),
            services.clone(),
        ))
        .expect("bounded Pi turn starts");
        let terminal = block_on(turn.take_terminal_outcome().expect("terminal outcome"));
        assert_eq!(terminal.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    }
    let error =
        block_on(session.start_turn(turn_request("limit-turn-overflow", deadline()), services))
            .err()
            .expect("third completed prompt fails");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.pi.rpc.prompt_limit_reached"
    );
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

#[test]
fn provider_success_does_not_hide_process_cleanup_failure() {
    let topology = ExecutionTopologyFixture::local();
    let fixture = FixtureHost::new(Scenario::Complete).with_process_wait_failure();
    let selected = selection_for_topology(&topology);
    let services = fixture.services(topology.execution_host_id().clone());
    let mut session = block_on(driver(selected.credential.clone()).open_session(
        selected.plan,
        open_request("cleanup-session", selected.resource),
        services.clone(),
    ))
    .expect("Pi cleanup session opens");
    let mut turn = block_on(session.start_turn(turn_request("cleanup-turn", deadline()), services))
        .expect("Pi cleanup turn starts");
    let terminal = block_on(turn.take_terminal_outcome().expect("terminal outcome"));
    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    let cleanup = block_on(session.close());
    assert!(matches!(cleanup, CleanupOutcome::Failed(ref diagnostic)
        if diagnostic.code() == "swallowtail.pi.rpc.process_cleanup_failed"));
}

fn terminal_case(
    topology: &ExecutionTopologyFixture,
    scenario: Scenario,
    label: &str,
) -> (TerminalOutcome, CleanupOutcome) {
    let fixture = FixtureHost::new(scenario);
    let selected = selection_for_topology(topology);
    let services = fixture.services(topology.execution_host_id().clone());
    let mut session = block_on(driver(selected.credential.clone()).open_session(
        selected.plan,
        open_request(&format!("failure-session-{label}"), selected.resource),
        services.clone(),
    ))
    .expect("Pi failure session opens");
    let mut turn = block_on(session.start_turn(
        turn_request(&format!("failure-turn-{label}"), deadline()),
        services,
    ))
    .expect("Pi failure turn starts");
    let terminal = block_on(turn.take_terminal_outcome().expect("terminal outcome"));
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    let cleanup = block_on(session.close());
    (terminal, cleanup)
}

fn driver(credential: swallowtail_core::CredentialRef) -> PiRpcDriver {
    PiRpcDriver::new(
        EnvironmentRef::new("pi.fixture.environment").expect("valid environment"),
        credential,
    )
}

fn deadline() -> Deadline {
    Deadline::at(MonotonicInstant::from_ticks(100_000))
}
