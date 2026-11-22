use super::{deadline, driver, make_host_id};
use crate::support::{
    CleanupEvent, SidecarFixtureHost, SidecarScenario, close_session, sidecar_open_request,
    sidecar_selection, turn_request,
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
        let mut turn = block_on(session.start_turn(
            turn_request("sidecar-failure-turn", deadline()),
            services.clone(),
        ))
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
        assert_eq!(
            block_on(close_session(session, services)),
            CleanupOutcome::Clean
        );
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
    let error = block_on(session.start_turn(
        turn_request("sidecar-mismatch-turn", deadline()),
        services.clone(),
    ))
    .err()
    .expect("mismatched response fails");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.pi.sdk-sidecar.response_command_mismatch"
    );
    assert!(!format!("{error:?}").contains("fixture private prompt"));
    assert_eq!(
        block_on(close_session(session, services)),
        CleanupOutcome::Clean
    );
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

struct SteppingClock {
    calls: std::sync::atomic::AtomicUsize,
    expire_on_call: usize,
}

impl swallowtail_runtime::TimeService for SteppingClock {
    fn now(&self) -> swallowtail_runtime::MonotonicInstant {
        let call = self.calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
        if call >= self.expire_on_call {
            swallowtail_runtime::MonotonicInstant::from_ticks(100)
        } else {
            swallowtail_runtime::MonotonicInstant::from_ticks(1)
        }
    }

    fn wait_until(
        &self,
        _deadline: swallowtail_runtime::Deadline,
    ) -> swallowtail_runtime::BoxFuture<'static, swallowtail_runtime::DeadlineObservation> {
        Box::pin(std::future::pending())
    }
}

#[test]
fn sidecar_rejected_abort_during_turn_close_blocks_subsequent_turns() {
    let host_id = make_host_id("pi.fixture.sdk-sidecar.abort-rejected");
    let fixture = SidecarFixtureHost::new(SidecarScenario::AbortRejected);
    let selected = sidecar_selection(host_id.clone());
    let services = fixture.services(host_id);
    let mut session = block_on(driver(selected.credential.clone()).open_session(
        selected.plan,
        sidecar_open_request("sidecar-abort-rejected-session", selected.resource),
        services.clone(),
    ))
    .expect("sidecar session opens");
    let turn = block_on(session.start_turn(
        turn_request("sidecar-abort-rejected-turn-1", deadline()),
        services.clone(),
    ))
    .expect("first sidecar turn starts");
    let close_outcome = block_on(turn.close());
    assert!(
        matches!(close_outcome, CleanupOutcome::Failed(ref diagnostic) if diagnostic.message().contains("cancel_rejected")),
        "expected cancel_rejected cleanup failure, got {close_outcome:?}"
    );
    let second_error = block_on(session.start_turn(
        turn_request("sidecar-abort-rejected-turn-2", deadline()),
        services.clone(),
    ))
    .err()
    .expect("sidecar session must refuse a new turn after rejected abort");
    assert_eq!(
        second_error.diagnostic().code(),
        "swallowtail.pi.sdk-sidecar.cancel_rejected"
    );
    let _ = block_on(close_session(session, services));
}

#[test]
fn sidecar_explicit_rejected_cancellation_followed_by_close_wakes_deadline_and_blocks_new_turn() {
    let host_id = make_host_id("pi.fixture.sdk-sidecar.explicit-abort-rejected");
    let fixture = SidecarFixtureHost::new(SidecarScenario::AbortRejected);
    let selected = sidecar_selection(host_id.clone());
    let services = fixture.services(host_id);
    let mut session = block_on(driver(selected.credential.clone()).open_session(
        selected.plan,
        sidecar_open_request("sidecar-explicit-abort-rejected-session", selected.resource),
        services.clone(),
    ))
    .expect("sidecar session opens");
    // Do not advance the fixture clock: close must complete without firing the deadline.
    let turn = block_on(session.start_turn(
        turn_request("sidecar-explicit-abort-rejected-turn-1", deadline()),
        services.clone(),
    ))
    .expect("first sidecar turn starts");
    let cancel_error = block_on(turn.cancellation().request())
        .expect_err("explicit sidecar abort rejection must return an error");
    assert_eq!(
        cancel_error.diagnostic().code(),
        "swallowtail.pi.sdk-sidecar.abort_rejected"
    );
    let close_outcome = block_on(turn.close());
    assert!(
        matches!(close_outcome, CleanupOutcome::Failed(ref diagnostic) if diagnostic.message().contains("cancel_rejected")),
        "close after prior rejected abort must still report cancel_rejected, got {close_outcome:?}"
    );
    let second_error = block_on(session.start_turn(
        turn_request("sidecar-explicit-abort-rejected-turn-2", deadline()),
        services.clone(),
    ))
    .err()
    .expect("sidecar session must refuse a new turn after prior rejected abort");
    assert_eq!(
        second_error.diagnostic().code(),
        "swallowtail.pi.sdk-sidecar.cancel_rejected"
    );
    let _ = block_on(close_session(session, services));
}

#[test]
fn sidecar_startup_deadline_enforced_before_and_after_synchronous_launch() {
    use std::sync::Arc;
    use swallowtail_core::{
        HarnessConfigurationPosture, ResourceAccess, SessionAccessPolicy,
        SessionProviderStatePolicy,
    };
    use swallowtail_runtime::{
        Deadline, MonotonicInstant, OpenSessionRequest, RequestId, SessionPlanAgreement,
    };

    // 1. Expires during lease acquisition before ProcessService::start is called.
    {
        let host_id = make_host_id("pi.fixture.sdk-sidecar.expire-before-launch");
        let fixture = SidecarFixtureHost::new(SidecarScenario::Complete);
        let selected = sidecar_selection(host_id.clone());
        let services = fixture.services(host_id).with_time(Arc::new(SteppingClock {
            calls: std::sync::atomic::AtomicUsize::new(0),
            expire_on_call: 2,
        }));
        let request = OpenSessionRequest::new(
            RequestId::new("sidecar-expire-before-launch").expect("id"),
            selected.resource,
            Some(Deadline::at(MonotonicInstant::from_ticks(50))),
            SessionPlanAgreement::explicit(
                SessionAccessPolicy::ambient_harness(ResourceAccess::Read),
                Some(SessionProviderStatePolicy::DurableProviderSessionPreserved),
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
            "swallowtail.pi.sdk-sidecar.open_deadline_elapsed"
        );
        assert_eq!(
            fixture.cleanup_events(),
            [
                CleanupEvent::ResourceRelease,
                CleanupEvent::CredentialRelease,
            ],
            "sidecar process must not have been started when deadline elapsed before dispatch"
        );
    }

    // 2. Deadline expires during synchronous ProcessService::start (check 3):
    //    the ready handle is explicitly stopped and reaped before lease release,
    //    and bootstrap never runs.
    {
        let host_id = make_host_id("pi.fixture.sdk-sidecar.expire-after-launch");
        let fixture = SidecarFixtureHost::new(SidecarScenario::Complete);
        let selected = sidecar_selection(host_id.clone());
        let services = fixture.services(host_id).with_time(Arc::new(SteppingClock {
            calls: std::sync::atomic::AtomicUsize::new(0),
            expire_on_call: 3,
        }));
        let request = OpenSessionRequest::new(
            RequestId::new("sidecar-expire-after-launch").expect("id"),
            selected.resource,
            Some(Deadline::at(MonotonicInstant::from_ticks(50))),
            SessionPlanAgreement::explicit(
                SessionAccessPolicy::ambient_harness(ResourceAccess::Read),
                Some(SessionProviderStatePolicy::DurableProviderSessionPreserved),
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
            "swallowtail.pi.sdk-sidecar.open_deadline_elapsed"
        );
        assert!(
            fixture.inputs().is_empty(),
            "bootstrap must not run after late launch"
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
