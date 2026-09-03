use crate::support::{
    CleanupEvent, FixtureHost, Scenario, close_session, open_request, selection, turn_request,
};
use futures_executor::block_on;
use swallowtail_adapter_oh_my_pi::OhMyPiRpcDriver;
use swallowtail_core::ExecutionHostId;
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
    let mut session = block_on(driver().open_session(
        selected.plan,
        open_request("session-cancel", selected.resource),
        services.clone(),
    ))
    .expect("OhMyPi session opens");
    let mut turn =
        block_on(session.start_turn(turn_request("turn-cancel", deadline()), services.clone()))
            .expect("OhMyPi turn starts");

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
    let mut session = block_on(driver().open_session(
        selected.plan,
        open_request("session-timeout", selected.resource),
        services.clone(),
    ))
    .expect("OhMyPi session opens");
    let mut turn =
        block_on(session.start_turn(turn_request("turn-timeout", deadline()), services.clone()))
            .expect("OhMyPi turn starts");

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
fn startup_mismatch_cleans_up() {
    let host_id = make_host_id("pi.fixture.host-startup");
    let fixture = FixtureHost::new(Scenario::StateMismatch);
    let selected = selection(host_id.clone());
    let error = block_on(driver().open_session(
        selected.plan,
        open_request("session-startup-fail", selected.resource),
        fixture.services(host_id),
    ))
    .err()
    .expect("startup state mismatch fails");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.oh_my_pi.rpc.state_mismatch"
    );
    assert_eq!(
        fixture.cleanup_events(),
        [CleanupEvent::ProcessWait, CleanupEvent::ResourceRelease,]
    );
}

fn driver() -> OhMyPiRpcDriver {
    OhMyPiRpcDriver::new(EnvironmentRef::new("pi.fixture.environment").expect("valid environment"))
}

fn make_host_id(value: &str) -> ExecutionHostId {
    ExecutionHostId::new(value).expect("valid host")
}

fn deadline() -> Deadline {
    Deadline::at(MonotonicInstant::from_ticks(1_000))
}
