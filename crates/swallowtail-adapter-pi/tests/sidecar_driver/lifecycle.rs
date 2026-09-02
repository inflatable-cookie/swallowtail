use super::{deadline, driver, make_host_id};
use crate::support::{
    CleanupEvent, SidecarFixtureHost, SidecarScenario, close_session, sidecar_open_request,
    sidecar_selection, sidecar_selection_with_attachments, turn_request,
};
use futures_executor::block_on;
use swallowtail_adapter_pi::PiSdkSidecarDriver;
use swallowtail_core::CredentialRef;
use swallowtail_runtime::{
    AttachmentDescriptor, AttachmentRef, AttachmentRole, CancellationAcknowledgement,
    CleanupOutcome, EnvironmentRef, InteractiveSessionDriver, TerminalStatus,
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
    let mut turn = block_on(session.start_turn(
        turn_request("sidecar-turn-cancel", deadline()),
        services.clone(),
    ))
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
    assert_eq!(
        block_on(close_session(session, services)),
        CleanupOutcome::Clean
    );
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
    let mut turn = block_on(session.start_turn(
        turn_request("sidecar-turn-timeout", deadline()),
        services.clone(),
    ))
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
    let cleanup = block_on(close_session(session, services));
    assert_eq!(
        cleanup
            .diagnostic()
            .map(swallowtail_core::SafeDiagnostic::code),
        Some("swallowtail.session_cleanup.deadline_expired")
    );
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
    let error = block_on(session.start_turn(
        turn_request("sidecar-busy-turn-2", deadline()),
        services.clone(),
    ))
    .err()
    .expect("parallel prompt fails");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.pi.sdk-sidecar.turn_active"
    );
    assert!(block_on(turn.close()) == CleanupOutcome::NotApplicable);
    assert_eq!(
        block_on(close_session(session, services)),
        CleanupOutcome::Clean
    );

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
        services.clone(),
    ))
    .err()
    .expect("third completed prompt fails");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.pi.sdk-sidecar.prompt_limit_reached"
    );
    assert_eq!(
        block_on(close_session(session, services)),
        CleanupOutcome::Clean
    );
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
    let mut turn = block_on(session.start_turn(
        turn_request("sidecar-cleanup-turn", deadline()),
        services.clone(),
    ))
    .expect("sidecar cleanup turn starts");
    let terminal = block_on(turn.take_terminal_outcome().expect("terminal outcome"));
    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    let cleanup = block_on(close_session(session, services));
    assert!(matches!(cleanup, CleanupOutcome::Failed(ref diagnostic)
        if diagnostic.code() == "swallowtail.pi.sdk-sidecar.process_cleanup_failed"));

    let host_id = make_host_id("pi.fixture.sdk-sidecar.nonzero-exit");
    let fixture = SidecarFixtureHost::new(SidecarScenario::Complete).with_process_exit_failure();
    let selected = sidecar_selection(host_id.clone());
    let services = fixture.services(host_id);
    let session = block_on(driver(selected.credential.clone()).open_session(
        selected.plan,
        sidecar_open_request("sidecar-nonzero-session", selected.resource),
        services.clone(),
    ))
    .expect("sidecar non-zero session opens");
    let cleanup = block_on(close_session(session, services));
    assert!(matches!(cleanup, CleanupOutcome::Failed(ref diagnostic)
        if diagnostic.code() == "swallowtail.pi.sdk-sidecar.process_cleanup_failed"));
}

#[test]
fn deadline_task_spawn_failure_clears_turn_and_releases_attachment() {
    let host_id = make_host_id("pi.fixture.sdk-sidecar.deadline-spawn");
    let fixture =
        SidecarFixtureHost::new(SidecarScenario::Complete).with_deadline_task_spawn_failure();
    let selected = sidecar_selection_with_attachments(host_id.clone());
    let services = fixture.services(host_id);
    let mut session = block_on(driver(selected.credential.clone()).open_session(
        selected.plan,
        sidecar_open_request("sidecar-deadline-spawn-session", selected.resource),
        services.clone(),
    ))
    .expect("sidecar session opens");
    let attachment = AttachmentDescriptor::new(
        AttachmentRef::new("pi.fixture.deadline-spawn-image").expect("valid attachment"),
        "image/png",
        AttachmentRole::Input,
    )
    .expect("valid descriptor")
    .with_known_length(8);
    let error = block_on(session.start_turn(
        turn_request("sidecar-deadline-spawn-fail", deadline()).with_attachments([attachment]),
        services.clone(),
    ))
    .err()
    .expect("deadline task spawn fails");
    assert_eq!(error.diagnostic().code(), "fixture.pi_sdk_sidecar.failed");
    assert_eq!(
        fixture
            .cleanup_events()
            .iter()
            .filter(|event| **event == CleanupEvent::AttachmentRelease)
            .count(),
        1
    );

    let mut turn = block_on(session.start_turn(
        turn_request("sidecar-deadline-spawn-retry", deadline()),
        services.clone(),
    ))
    .expect("rolled-back session accepts a later turn");
    assert_eq!(
        block_on(turn.take_terminal_outcome().expect("terminal outcome")).status(),
        &TerminalStatus::Completed
    );
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(
        block_on(close_session(session, services)),
        CleanupOutcome::Clean
    );
}
