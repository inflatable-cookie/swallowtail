use crate::support::{
    FixtureHost, Scenario, close_session, open_request, selection_for_topology, turn_request,
};
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
                Scenario::SummarizationRetryDrift,
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
            if provider_failure {
                assert_eq!(
                    diagnostic.failure_classification().origin(),
                    swallowtail_core::FailureOrigin::Provider
                );
            } else if expected_code == "swallowtail.pi.rpc.retry_policy_drift" {
                assert_eq!(
                    diagnostic.failure_classification().origin(),
                    swallowtail_core::FailureOrigin::Harness
                );
            }
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
    let error = block_on(session.start_turn(
        turn_request("correlation-turn", deadline()),
        services.clone(),
    ))
    .err()
    .expect("mismatched response fails");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.pi.rpc.response_command_mismatch"
    );
    assert!(!format!("{error:?}").contains("fixture private prompt"));
    assert_eq!(
        block_on(close_session(session, services)),
        CleanupOutcome::Clean
    );
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
    let error =
        block_on(session.start_turn(turn_request("busy-turn-2", deadline()), services.clone()))
            .err()
            .expect("parallel prompt fails");
    assert_eq!(error.diagnostic().code(), "swallowtail.pi.rpc.turn_active");
    assert!(block_on(turn.close()) == CleanupOutcome::NotApplicable);
    assert_eq!(
        block_on(close_session(session, services)),
        CleanupOutcome::Clean
    );

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
    let error = block_on(session.start_turn(
        turn_request("limit-turn-overflow", deadline()),
        services.clone(),
    ))
    .err()
    .expect("third completed prompt fails");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.pi.rpc.prompt_limit_reached"
    );
    assert_eq!(
        block_on(close_session(session, services)),
        CleanupOutcome::Clean
    );
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
    let mut turn =
        block_on(session.start_turn(turn_request("cleanup-turn", deadline()), services.clone()))
            .expect("Pi cleanup turn starts");
    let terminal = block_on(turn.take_terminal_outcome().expect("terminal outcome"));
    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    let cleanup = block_on(close_session(session, services));
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
        services.clone(),
    ))
    .expect("Pi failure turn starts");
    let terminal = block_on(turn.take_terminal_outcome().expect("terminal outcome"));
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    let cleanup = block_on(close_session(session, services));
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

struct SteppingClock {
    calls: std::sync::atomic::AtomicUsize,
    expire_on_call: usize,
}

impl swallowtail_runtime::TimeService for SteppingClock {
    fn now(&self) -> MonotonicInstant {
        let call = self.calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
        if call >= self.expire_on_call {
            MonotonicInstant::from_ticks(100)
        } else {
            MonotonicInstant::from_ticks(1)
        }
    }

    fn wait_until(
        &self,
        _deadline: Deadline,
    ) -> swallowtail_runtime::BoxFuture<'static, swallowtail_runtime::DeadlineObservation> {
        Box::pin(std::future::pending())
    }
}

#[test]
fn rejected_abort_during_turn_close_blocks_subsequent_turns_on_the_session() {
    let topology = ExecutionTopologyFixture::local();
    let fixture = FixtureHost::new(Scenario::AbortRejected);
    let selected = selection_for_topology(&topology);
    let services = fixture.services(topology.execution_host_id().clone());
    let mut session = block_on(driver(selected.credential.clone()).open_session(
        selected.plan,
        open_request("abort-rejected-session", selected.resource),
        services.clone(),
    ))
    .expect("Pi session opens");
    let turn = block_on(session.start_turn(
        turn_request("abort-rejected-turn-1", deadline()),
        services.clone(),
    ))
    .expect("first turn starts");
    let close_outcome = block_on(turn.close());
    assert!(
        matches!(close_outcome, CleanupOutcome::Failed(ref diagnostic) if diagnostic.message().contains("cancel_rejected")),
        "expected cancel_rejected cleanup failure, got {close_outcome:?}"
    );
    let second_error = block_on(session.start_turn(
        turn_request("abort-rejected-turn-2", deadline()),
        services.clone(),
    ))
    .err()
    .expect("session must refuse a new turn after rejected abort");
    assert_eq!(
        second_error.diagnostic().code(),
        "swallowtail.pi.rpc.connection_closed"
    );
    let _ = block_on(close_session(session, services));
}

#[test]
fn explicit_rejected_cancellation_followed_by_close_wakes_deadline_and_blocks_new_turn() {
    let topology = ExecutionTopologyFixture::local();
    let fixture = FixtureHost::new(Scenario::AbortRejected);
    let selected = selection_for_topology(&topology);
    let services = fixture.services(topology.execution_host_id().clone());
    let mut session = block_on(driver(selected.credential.clone()).open_session(
        selected.plan,
        open_request("explicit-abort-rejected-session", selected.resource),
        services.clone(),
    ))
    .expect("Pi session opens");
    // Do not advance the fixture clock: close must complete without firing the deadline.
    let turn = block_on(session.start_turn(
        turn_request("explicit-abort-rejected-turn-1", deadline()),
        services.clone(),
    ))
    .expect("first turn starts");
    let cancel_error = block_on(turn.cancellation().request())
        .expect_err("explicit abort rejection must return an error");
    assert_eq!(
        cancel_error.diagnostic().code(),
        "swallowtail.pi.rpc.abort_rejected"
    );
    let close_outcome = block_on(turn.close());
    assert!(
        matches!(close_outcome, CleanupOutcome::Failed(ref diagnostic) if diagnostic.message().contains("cancel_rejected")),
        "close after prior rejected abort must still report cancel_rejected, got {close_outcome:?}"
    );
    let second_error = block_on(session.start_turn(
        turn_request("explicit-abort-rejected-turn-2", deadline()),
        services.clone(),
    ))
    .err()
    .expect("session must refuse a new turn after prior rejected abort");
    assert_eq!(
        second_error.diagnostic().code(),
        "swallowtail.pi.rpc.connection_closed"
    );
    let _ = block_on(close_session(session, services));
}

#[test]
fn startup_timeout_force_stops_unresponsive_rpc_child_before_releasing_leases() {
    use crate::support::CleanupEvent;
    use swallowtail_runtime::{OpenSessionRequest, RequestId};

    let topology = ExecutionTopologyFixture::local();
    let fixture = FixtureHost::new(Scenario::Complete)
        .with_held_command("set_auto_retry")
        .with_unresponsive_shutdown()
        .with_immediate_time();
    let selected = selection_for_topology(&topology);
    let request = OpenSessionRequest::from_plan(
        &selected.plan,
        RequestId::new("pi-rpc-startup-abort").expect("valid request"),
        selected.resource,
        Some(Deadline::at(MonotonicInstant::from_ticks(1_001))),
    )
    .expect("open request builds");
    let error = block_on(driver(selected.credential).open_session(
        selected.plan,
        request,
        fixture.services(topology.execution_host_id().clone()),
    ))
    .err()
    .expect("startup times out");

    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.pi.rpc.open_deadline_elapsed"
    );
    assert_eq!(fixture.inputs()[0]["type"], "set_auto_retry");
    assert_eq!(fixture.force_stops(), 1);
    assert_eq!(
        fixture.cleanup_events(),
        [
            CleanupEvent::ProcessWait,
            CleanupEvent::ResourceRelease,
            CleanupEvent::CredentialRelease
        ]
    );
}

#[test]
fn startup_deadline_enforced_before_and_after_synchronous_process_launch() {
    use crate::support::CleanupEvent;
    use std::sync::Arc;
    use swallowtail_core::{
        HarnessConfigurationPosture, ResourceAccess, SessionAccessPolicy,
        SessionProviderStatePolicy,
    };
    use swallowtail_runtime::{OpenSessionRequest, RequestId, SessionPlanAgreement};

    // 1. Expires during lease acquisition before ProcessService::start is called.
    {
        let topology = ExecutionTopologyFixture::local();
        let fixture = FixtureHost::new(Scenario::Complete);
        let selected = selection_for_topology(&topology);
        let services = fixture
            .services(topology.execution_host_id().clone())
            .with_time(Arc::new(SteppingClock {
                calls: std::sync::atomic::AtomicUsize::new(0),
                expire_on_call: 2,
            }));
        let request = OpenSessionRequest::new(
            RequestId::new("pi-rpc-expire-before-launch").expect("id"),
            selected.resource,
            Some(Deadline::at(MonotonicInstant::from_ticks(50))),
            SessionPlanAgreement::explicit(
                SessionAccessPolicy::ambient_harness(ResourceAccess::Read),
                Some(SessionProviderStatePolicy::Prohibited),
                Some(HarnessConfigurationPosture::ProviderSuppressed),
            ),
        );
        let error = block_on(driver(selected.credential.clone()).open_session(
            selected.plan,
            request,
            services,
        ))
        .err()
        .expect("expired deadline before launch must fail");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.pi.rpc.open_deadline_elapsed"
        );
        assert_eq!(
            fixture.cleanup_events(),
            [
                CleanupEvent::ResourceRelease,
                CleanupEvent::CredentialRelease,
            ],
            "process must not have been started when deadline elapsed before dispatch"
        );
    }

    // 2. Deadline expires during synchronous ProcessService::start (check 3):
    //    the ready handle is explicitly stopped and reaped before lease release,
    //    and handshake never runs.
    {
        let topology = ExecutionTopologyFixture::local();
        let fixture = FixtureHost::new(Scenario::Complete);
        let selected = selection_for_topology(&topology);
        let services = fixture
            .services(topology.execution_host_id().clone())
            .with_time(Arc::new(SteppingClock {
                calls: std::sync::atomic::AtomicUsize::new(0),
                expire_on_call: 3,
            }));
        let request = OpenSessionRequest::new(
            RequestId::new("pi-rpc-expire-after-launch").expect("id"),
            selected.resource,
            Some(Deadline::at(MonotonicInstant::from_ticks(50))),
            SessionPlanAgreement::explicit(
                SessionAccessPolicy::ambient_harness(ResourceAccess::Read),
                Some(SessionProviderStatePolicy::Prohibited),
                Some(HarnessConfigurationPosture::ProviderSuppressed),
            ),
        );
        let error = block_on(driver(selected.credential.clone()).open_session(
            selected.plan,
            request,
            services,
        ))
        .err()
        .expect("deadline elapsed after synchronous launch must fail");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.pi.rpc.open_deadline_elapsed"
        );
        assert!(
            fixture.inputs().is_empty(),
            "handshake must not run after late launch"
        );
        assert_eq!(
            fixture.cleanup_events(),
            [
                CleanupEvent::ProcessWait,
                CleanupEvent::ResourceRelease,
                CleanupEvent::CredentialRelease,
            ]
        );
    }
}
