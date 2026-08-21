use super::{deadline, driver, make_host_id};
use crate::support::{
    CleanupEvent, SidecarFixtureHost, SidecarScenario, sidecar_open_request, sidecar_selection,
    turn_request,
};
use futures_executor::block_on;
use swallowtail_runtime::{CleanupOutcome, InteractiveSessionDriver, TerminalStatus};

#[test]
fn provider_transport_and_protocol_failures_remain_distinct() {
    for (scenario, expected_code, provider_failure) in [
        (
            SidecarScenario::ProviderFailure,
            "swallowtail.pi.sdk-sidecar.provider_failed",
            true,
        ),
        (
            SidecarScenario::Disconnect,
            "swallowtail.pi.sdk-sidecar.connection_ended",
            false,
        ),
        (
            SidecarScenario::Malformed,
            "swallowtail.pi.sdk-sidecar.protocol_failed",
            false,
        ),
        (
            SidecarScenario::UnknownEvent,
            "swallowtail.pi.sdk-sidecar.protocol_failed",
            false,
        ),
        (
            SidecarScenario::TerminalRecord,
            "swallowtail.pi.sdk-sidecar.terminal_record",
            false,
        ),
    ] {
        let host_id = make_host_id("pi.fixture.sdk-sidecar.failure");
        let fixture = SidecarFixtureHost::new(scenario);
        let selected = sidecar_selection(host_id.clone());
        let services = fixture.services(host_id);
        let mut session = block_on(driver(selected.credential.clone()).open_session(
            selected.plan,
            sidecar_open_request("sidecar-failure-session", selected.resource),
            services.clone(),
        ))
        .expect("sidecar session opens");
        let mut turn = block_on(
            session.start_turn(turn_request("sidecar-failure-turn", deadline()), services),
        )
        .expect("sidecar turn starts");
        let terminal = block_on(turn.take_terminal_outcome().expect("terminal outcome"));
        let diagnostic = match terminal.status() {
            TerminalStatus::ProviderFailed(diagnostic) if provider_failure => diagnostic,
            TerminalStatus::RuntimeFailed(diagnostic) if !provider_failure => diagnostic,
            status => panic!("unexpected sidecar terminal status: {status:?}"),
        };
        assert_eq!(diagnostic.code(), expected_code);
        if provider_failure {
            assert_eq!(
                diagnostic.failure_classification().origin(),
                swallowtail_core::FailureOrigin::Provider
            );
        }
        assert!(!format!("{terminal:?}").contains("fixture private prompt"));
        assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
        assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    }
}

#[test]
fn response_command_mismatch_fails_the_turn_and_still_joins() {
    let host_id = make_host_id("pi.fixture.sdk-sidecar.mismatch");
    let fixture = SidecarFixtureHost::new(SidecarScenario::ResponseMismatch);
    let selected = sidecar_selection(host_id.clone());
    let services = fixture.services(host_id);
    let mut session = block_on(driver(selected.credential.clone()).open_session(
        selected.plan,
        sidecar_open_request("sidecar-mismatch-session", selected.resource),
        services.clone(),
    ))
    .expect("sidecar session opens");
    let error =
        block_on(session.start_turn(turn_request("sidecar-mismatch-turn", deadline()), services))
            .err()
            .expect("mismatched response fails");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.pi.sdk-sidecar.response_command_mismatch"
    );
    assert!(!format!("{error:?}").contains("fixture private prompt"));
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

#[test]
fn bootstrap_identity_mismatch_fails_before_provider_work() {
    for (scenario, expected_code) in [
        (
            SidecarScenario::BootstrapCwdMismatch,
            "swallowtail.pi.sdk-sidecar.bootstrap_mismatch",
        ),
        (
            SidecarScenario::BootstrapVersionMismatch,
            "swallowtail.pi.sdk-sidecar.bootstrap_mismatch",
        ),
        (
            SidecarScenario::StateMismatch,
            "swallowtail.pi.sdk-sidecar.state_mismatch",
        ),
    ] {
        let host_id = make_host_id("pi.fixture.sdk-sidecar.startup");
        let fixture = SidecarFixtureHost::new(scenario);
        let selected = sidecar_selection(host_id.clone());
        let error = block_on(driver(selected.credential.clone()).open_session(
            selected.plan,
            sidecar_open_request("sidecar-startup-fail", selected.resource),
            fixture.services(host_id),
        ))
        .err()
        .expect("startup identity mismatch fails");
        assert_eq!(error.diagnostic().code(), expected_code);
        let inputs = fixture.inputs();
        assert!(
            inputs.iter().all(|value| value["command"] != "prompt"),
            "no provider work may start before identity proof"
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
