use super::{deadline, driver, make_host_id};
use crate::support::{
    SidecarFixtureHost, SidecarScenario, sidecar_open_request, sidecar_selection, turn_request,
};
use futures_executor::block_on;
use swallowtail_adapter_pi::PiSdkSidecarDriver;
use swallowtail_core::CredentialRef;
use swallowtail_runtime::{
    CancellationAcknowledgement, CleanupOutcome, EnvironmentRef, InteractiveSessionDriver,
    TerminalStatus,
};

#[test]
fn native_abort_is_idempotent_and_resolves_cancelled() {
    let host_id = make_host_id("pi.fixture.sdk-sidecar.cancel");
    let fixture = SidecarFixtureHost::new(SidecarScenario::Hold);
    let selected = sidecar_selection(host_id.clone());
    let services = fixture.services(host_id);
    let mut session = block_on(driver(selected.credential.clone()).open_session(
        selected.plan,
        sidecar_open_request("sidecar-session-cancel", selected.resource),
        services.clone(),
    ))
    .expect("sidecar session opens");
    let mut turn =
        block_on(session.start_turn(turn_request("sidecar-turn-cancel", deadline()), services))
            .expect("sidecar turn starts");

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
    assert!(
        fixture
            .inputs()
            .iter()
            .any(|value| value["command"] == "abort")
    );
}

#[test]
fn host_deadline_uses_native_abort_and_resolves_timed_out() {
    let host_id = make_host_id("pi.fixture.sdk-sidecar.deadline");
    let fixture = SidecarFixtureHost::new(SidecarScenario::Hold).with_immediate_time();
    let selected = sidecar_selection(host_id.clone());
    let services = fixture.services(host_id);
    let mut session = block_on(driver(selected.credential.clone()).open_session(
        selected.plan,
        sidecar_open_request("sidecar-session-timeout", selected.resource),
        services.clone(),
    ))
    .expect("sidecar session opens");
    let mut turn =
        block_on(session.start_turn(turn_request("sidecar-turn-timeout", deadline()), services))
            .expect("sidecar turn starts");

    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome exists"),
    );
    assert_eq!(terminal.status(), &TerminalStatus::TimedOut);
    assert!(
        fixture
            .inputs()
            .iter()
            .any(|value| value["command"] == "abort")
    );
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

#[test]
fn prompt_bounds_hold_without_reclassification() {
    let host_id = make_host_id("pi.fixture.sdk-sidecar.bounds");
    let fixture = SidecarFixtureHost::new(SidecarScenario::Hold);
    let selected = sidecar_selection(host_id.clone());
    let services = fixture.services(host_id);
    let mut session = block_on(driver(selected.credential.clone()).open_session(
        selected.plan,
        sidecar_open_request("sidecar-busy-session", selected.resource),
        services.clone(),
    ))
    .expect("sidecar session opens");
    let turn = block_on(session.start_turn(
        turn_request("sidecar-busy-turn-1", deadline()),
        services.clone(),
    ))
    .expect("first sidecar turn starts");
    let error =
        block_on(session.start_turn(turn_request("sidecar-busy-turn-2", deadline()), services))
            .err()
            .expect("parallel prompt fails");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.pi.sdk-sidecar.turn_active"
    );
    assert!(block_on(turn.close()) == CleanupOutcome::NotApplicable);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);

    let host_id = make_host_id("pi.fixture.sdk-sidecar.limit");
    let fixture = SidecarFixtureHost::new(SidecarScenario::Complete);
    let selected = sidecar_selection(host_id.clone());
    let services = fixture.services(host_id);
    let mut session = block_on(driver(selected.credential.clone()).open_session(
        selected.plan,
        sidecar_open_request("sidecar-limit-session", selected.resource),
        services.clone(),
    ))
    .expect("sidecar limit session opens");
    for index in 0..2 {
        let mut turn = block_on(session.start_turn(
            turn_request(&format!("sidecar-limit-turn-{index}"), deadline()),
            services.clone(),
        ))
        .expect("bounded sidecar turn starts");
        let terminal = block_on(turn.take_terminal_outcome().expect("terminal outcome"));
        assert_eq!(terminal.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    }
    let error = block_on(session.start_turn(
        turn_request("sidecar-limit-turn-overflow", deadline()),
        services,
    ))
    .err()
    .expect("third completed prompt fails");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.pi.sdk-sidecar.prompt_limit_reached"
    );
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

#[test]
fn preflight_mismatch_has_no_effect_and_process_cleanup_failure_surfaces() {
    let host_id = make_host_id("pi.fixture.sdk-sidecar.preflight");
    let fixture = SidecarFixtureHost::new(SidecarScenario::Complete);
    let selected = sidecar_selection(host_id.clone());
    let wrong_driver = PiSdkSidecarDriver::new(
        EnvironmentRef::new("pi.fixture.environment").expect("valid environment"),
        CredentialRef::new("pi.fixture.wrong").expect("valid credential"),
    );
    let error = block_on(wrong_driver.open_session(
        selected.plan,
        sidecar_open_request("sidecar-preflight-fail", selected.resource),
        fixture.services(host_id),
    ))
    .err()
    .expect("credential mismatch fails");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.pi.sdk-sidecar.request_plan_mismatch"
    );
    assert_eq!(fixture.credential_acquisitions(), 0);
    assert!(!fixture.process_started());

    let host_id = make_host_id("pi.fixture.sdk-sidecar.cleanup");
    let fixture = SidecarFixtureHost::new(SidecarScenario::Complete).with_process_wait_failure();
    let selected = sidecar_selection(host_id.clone());
    let services = fixture.services(host_id);
    let mut session = block_on(driver(selected.credential.clone()).open_session(
        selected.plan,
        sidecar_open_request("sidecar-cleanup-session", selected.resource),
        services.clone(),
    ))
    .expect("sidecar cleanup session opens");
    let mut turn =
        block_on(session.start_turn(turn_request("sidecar-cleanup-turn", deadline()), services))
            .expect("sidecar cleanup turn starts");
    let terminal = block_on(turn.take_terminal_outcome().expect("terminal outcome"));
    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    let cleanup = block_on(session.close());
    assert!(matches!(cleanup, CleanupOutcome::Failed(ref diagnostic)
        if diagnostic.code() == "swallowtail.pi.sdk-sidecar.process_cleanup_failed"));
}
