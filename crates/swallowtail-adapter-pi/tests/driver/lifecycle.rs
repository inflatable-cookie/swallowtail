use crate::support::{CleanupEvent, FixtureHost, Scenario, open_request, selection, turn_request};
use futures_executor::block_on;
use swallowtail_adapter_pi::PiRpcDriver;
use swallowtail_core::{CredentialRef, ExecutionHostId};
use swallowtail_runtime::{
    CancellationAcknowledgement, CleanupOutcome, Deadline, EnvironmentRef,
    InteractiveSessionDriver, MonotonicInstant, TerminalStatus,
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
    let mut turn = block_on(session.start_turn(turn_request("turn-cancel", deadline()), services))
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
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
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
    let mut turn = block_on(session.start_turn(turn_request("turn-timeout", deadline()), services))
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
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
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
