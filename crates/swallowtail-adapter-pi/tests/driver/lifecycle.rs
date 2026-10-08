use crate::support::{
    CleanupEvent, FixtureHost, Scenario, close_session, open_request, selection,
    selection_at_version, turn_request,
};
use futures_executor::block_on;
use swallowtail_adapter_pi::PiRpcDriver;
use swallowtail_core::{CredentialRef, ExecutionHostId, HarnessMessageClass};
use swallowtail_runtime::{
    CancellationAcknowledgement, CleanupOutcome, Deadline, EnvironmentRef,
    HarnessCommandAcknowledgement, HarnessCommandId, HarnessScheduledMessage,
    InteractiveSessionDriver, MonotonicInstant, OperationContent, TerminalStatus,
};

#[test]
fn native_abort_is_idempotent_and_resolves_cancelled() {
    let host_id = make_host_id("pi.fixture.host");
    let fixture = FixtureHost::new(Scenario::Hold);
    let selected = selection(host_id.clone());
    let services = fixture.services(host_id);
    let mut session = block_on(driver(selected.credential.clone()).open_session(
        selected.plan,
        open_request("session-cancel", selected.resource),
        services.clone(),
    ))
    .expect("Pi session opens");
    let mut turn =
        block_on(session.start_turn(turn_request("turn-cancel", deadline()), services.clone()))
            .expect("Pi turn starts");

    assert_eq!(
        block_on(turn.cancellation().request()).expect("abort request succeeds"),
        CancellationAcknowledgement::Requested
    );
    assert_eq!(
        block_on(turn.cancellation().request()).expect("repeat abort is classified"),
        CancellationAcknowledgement::AlreadyRequested
    );
    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome exists"),
    );
    assert_eq!(terminal.status(), &TerminalStatus::Cancelled);
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(
        block_on(close_session(session, services)),
        CleanupOutcome::Clean
    );
}

#[test]
fn host_deadline_uses_native_abort_and_resolves_timed_out() {
    let host_id = make_host_id("pi.fixture.host");
    let fixture = FixtureHost::new(Scenario::Hold).with_immediate_time();
    let selected = selection(host_id.clone());
    let services = fixture.services(host_id);
    let mut session = block_on(driver(selected.credential.clone()).open_session(
        selected.plan,
        open_request("session-timeout", selected.resource),
        services.clone(),
    ))
    .expect("Pi session opens");
    let mut turn =
        block_on(session.start_turn(turn_request("turn-timeout", deadline()), services.clone()))
            .expect("Pi turn starts");

    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome exists"),
    );
    assert_eq!(terminal.status(), &TerminalStatus::TimedOut);
    assert!(
        fixture
            .inputs()
            .iter()
            .any(|value| value["type"] == "abort")
    );
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(
        block_on(close_session(session, services)),
        CleanupOutcome::Clean
    );
}

#[test]
fn preflight_mismatch_has_no_effect_and_startup_mismatch_cleans_up() {
    let host_id = make_host_id("pi.fixture.host");
    let fixture = FixtureHost::new(Scenario::Complete);
    let selected = selection(host_id.clone());
    let wrong_driver = driver(CredentialRef::new("pi.fixture.wrong").expect("valid credential"));
    let error = block_on(wrong_driver.open_session(
        selected.plan,
        open_request("session-preflight-fail", selected.resource),
        fixture.services(host_id),
    ))
    .err()
    .expect("credential mismatch fails");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.pi.rpc.request_plan_mismatch"
    );
    assert_eq!(fixture.credential_acquisitions(), 0);
    assert!(!fixture.process_started());

    let host_id = make_host_id("pi.fixture.host-startup");
    let fixture = FixtureHost::new(Scenario::StateMismatch);
    let selected = selection(host_id.clone());
    let error = block_on(driver(selected.credential.clone()).open_session(
        selected.plan,
        open_request("session-startup-fail", selected.resource),
        fixture.services(host_id),
    ))
    .err()
    .expect("startup state mismatch fails");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.pi.rpc.state_mismatch"
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

#[test]
fn current_command_dispositions_fail_closed_and_do_not_leave_a_running_turn() {
    for (index, scenario, expected_code) in [
        (
            "handled",
            Scenario::PromptHandled,
            "swallowtail.pi.rpc.prompt_handled",
        ),
        (
            "unknown",
            Scenario::PromptInvalidDisposition,
            "swallowtail.pi.rpc.response_disposition_invalid",
        ),
        (
            "missing",
            Scenario::PromptMissingDisposition,
            "swallowtail.pi.rpc.response_disposition_invalid",
        ),
    ] {
        let host_id = make_host_id(&format!("pi.fixture.prompt-disposition.{index}"));
        let fixture = FixtureHost::new(scenario);
        let selected = selection_at_version(host_id.clone(), "1.1.0");
        let services = fixture.services(host_id);
        let mut session = block_on(driver(selected.credential.clone()).open_session(
            selected.plan,
            open_request("session-disposition", selected.resource),
            services.clone(),
        ))
        .expect("current Pi session opens");

        let error = block_on(session.start_turn(
            turn_request("turn-disposition", deadline()),
            services.clone(),
        ))
        .err()
        .expect("handled or malformed response never returns a turn handle");
        assert_eq!(error.diagnostic().code(), expected_code);
        assert_eq!(
            block_on(close_session(session, services)),
            CleanupOutcome::Clean
        );
    }
}

#[test]
fn handled_steering_and_follow_up_release_their_scheduling_slots() {
    let host_id = make_host_id("pi.fixture.scheduled-disposition");
    let fixture = FixtureHost::new(Scenario::ScheduledHandled);
    let selected = selection_at_version(host_id.clone(), "1.1.0");
    let services = fixture.services(host_id);
    let mut session = block_on(driver(selected.credential.clone()).open_session(
        selected.plan,
        open_request("session-scheduled-disposition", selected.resource),
        services.clone(),
    ))
    .expect("current Pi session opens");
    let mut turn = block_on(session.start_turn(
        turn_request("turn-scheduled-disposition", deadline()),
        services.clone(),
    ))
    .expect("current Pi prompt starts");

    for (id, class) in [
        ("steer-first", HarnessMessageClass::Steering),
        ("steer-second", HarnessMessageClass::Steering),
        ("follow-first", HarnessMessageClass::FollowUp),
        ("follow-second", HarnessMessageClass::FollowUp),
    ] {
        let response = block_on(turn.schedule_harness_message(scheduled(id, class)))
            .expect("handled command receives a typed acknowledgement");
        assert_eq!(
            response.acknowledgement(),
            HarnessCommandAcknowledgement::Rejected
        );
    }
    let inputs = fixture.inputs();
    assert_eq!(
        inputs
            .iter()
            .filter(|value| value["type"] == "steer")
            .count(),
        2
    );
    assert_eq!(
        inputs
            .iter()
            .filter(|value| value["type"] == "follow_up")
            .count(),
        2
    );

    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(
        block_on(close_session(session, services)),
        CleanupOutcome::Clean
    );
}

#[test]
fn current_aborted_settlement_resolves_cancelled_and_missing_state_fails_closed() {
    for (suffix, scenario, expected_status) in [
        (
            "completed",
            Scenario::CurrentComplete,
            TerminalStatus::Completed,
        ),
        ("aborted", Scenario::AgentAborted, TerminalStatus::Cancelled),
        (
            "missing",
            Scenario::SettledMissingAborted,
            TerminalStatus::RuntimeFailed(swallowtail_core::SafeDiagnostic::new(
                "swallowtail.pi.rpc.settled_aborted_missing",
                "Pi RPC omitted the qualified agent-settled abort state",
            )),
        ),
    ] {
        let host_id = make_host_id(&format!("pi.fixture.settled.{suffix}"));
        let fixture = FixtureHost::new(scenario);
        let selected = selection_at_version(host_id.clone(), "1.1.0");
        let services = fixture.services(host_id);
        let mut session = block_on(driver(selected.credential.clone()).open_session(
            selected.plan,
            open_request("session-settled", selected.resource),
            services.clone(),
        ))
        .expect("current Pi session opens");
        let mut turn = block_on(
            session.start_turn(turn_request("turn-settled", deadline()), services.clone()),
        )
        .expect("current Pi prompt starts");
        let terminal = block_on(
            turn.take_terminal_outcome()
                .expect("terminal outcome exists"),
        );
        assert_eq!(terminal.status(), &expected_status);
        assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
        assert_eq!(
            block_on(close_session(session, services)),
            CleanupOutcome::Clean
        );
    }
}

fn driver(credential: CredentialRef) -> PiRpcDriver {
    PiRpcDriver::new(
        EnvironmentRef::new("pi.fixture.environment").expect("valid environment"),
        credential,
    )
}

fn make_host_id(value: &str) -> ExecutionHostId {
    ExecutionHostId::new(value).expect("valid host")
}

fn deadline() -> Deadline {
    Deadline::at(MonotonicInstant::from_ticks(1_000))
}

fn scheduled(id: &str, class: HarnessMessageClass) -> HarnessScheduledMessage {
    HarnessScheduledMessage::new(
        HarnessCommandId::new(id).expect("valid command id"),
        class,
        OperationContent::new("fixture scheduled input").expect("valid content"),
    )
}
