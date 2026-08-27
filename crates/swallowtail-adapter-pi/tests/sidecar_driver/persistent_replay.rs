use super::{driver, make_host_id};
use crate::support::{
    CleanupEvent, FIXTURE_SESSION_REF, SidecarFixtureHost, SidecarScenario, sidecar_selection,
};
use futures_executor::block_on;
use swallowtail_adapter_pi::{PiSdkSidecarSessionPreparation, prepare_pi_sdk_sidecar_session};
use swallowtail_core::{
    AccessProfileId, ConfiguredInstanceId, ExecutionHostId, InstanceRevision, InstanceTargetRef,
    ModelId, ModelRouteId, ModelRouteRevision, ProviderId, ResourceAccess, SessionAccessPolicy,
    SessionProviderStatePolicy, SessionRef,
};
use swallowtail_runtime::{
    CleanupOutcome, Deadline, EnvironmentRef, InteractiveSessionDriver, LoadSessionRequest,
    MonotonicInstant, RequestId, ResumeSessionRequest, SessionOptions, SessionPlanAgreement,
    SessionResumeBinding, WorkingResourceRef,
};

fn preparation(host: ExecutionHostId, request_id: &str) -> PiSdkSidecarSessionPreparation {
    PiSdkSidecarSessionPreparation::new(
        ConfiguredInstanceId::new("pi.fixture.sdk-sidecar.instance").expect("valid instance"),
        InstanceRevision::new("fixture-revision").expect("valid revision"),
        host,
        InstanceTargetRef::new("pi.fixture.pinned-launch-recipe").expect("valid target"),
        EnvironmentRef::new("pi.fixture.environment").expect("valid environment"),
        swallowtail_core::CredentialRef::new("pi.fixture.delegated-auth")
            .expect("valid credential"),
        AccessProfileId::new("pi.fixture.harness-auth").expect("valid access id"),
        ModelRouteId::new("pi.fixture.route").expect("valid route"),
        ModelRouteRevision::new("fixture-route-revision").expect("valid route revision"),
        ProviderId::new("fixture-provider").expect("valid provider"),
        ModelId::new("fixture-model").expect("valid model"),
        WorkingResourceRef::new("pi.fixture.workspace").expect("valid resource"),
        RequestId::new(request_id).expect("valid request"),
    )
}

fn ambient_read() -> SessionAccessPolicy {
    SessionAccessPolicy::ambient_harness(ResourceAccess::Read)
}

fn agreement() -> SessionPlanAgreement {
    SessionPlanAgreement::explicit(
        ambient_read(),
        Some(SessionProviderStatePolicy::DurableProviderSessionPreserved),
        Some(swallowtail_core::HarnessConfigurationPosture::ProviderSuppressed),
    )
}

#[test]
fn load_transports_bounded_ordered_replay_before_readiness() {
    let host_id = make_host_id("pi.fixture.sdk-sidecar.load");
    let fixture = SidecarFixtureHost::new(SidecarScenario::Complete);
    let prepared = prepare_pi_sdk_sidecar_session(
        preparation(host_id.clone(), "sidecar-load"),
        SessionOptions::default(),
    )
    .expect("sidecar session prepares");
    let binding = SessionResumeBinding::new(
        SessionRef::new(FIXTURE_SESSION_REF).expect("valid session ref"),
        prepared.plan().instance_id().clone(),
        prepared.plan().execution_host_id().clone(),
        prepared.plan().model_route_id().expect("route").clone(),
        prepared.plan().model_id().expect("model").clone(),
        prepared
            .request()
            .working_resource()
            .expect("resource")
            .clone(),
        ambient_read(),
    );
    let loaded = block_on(
        prepared
            .load_session(
                RequestId::new("sidecar-load-op").expect("valid request"),
                binding,
                fixture.services(host_id),
            )
            .expect("load request builds"),
    )
    .expect("sidecar session loads");
    let (replay, session) = loaded.into_parts();

    let kinds: Vec<_> = replay.iter().map(|item| item.kind()).collect();
    assert_eq!(
        kinds,
        [
            swallowtail_runtime::SessionReplayKind::UserMessage,
            swallowtail_runtime::SessionReplayKind::AgentReasoning,
            swallowtail_runtime::SessionReplayKind::AgentMessage,
            swallowtail_runtime::SessionReplayKind::ToolCall,
            swallowtail_runtime::SessionReplayKind::ToolCallUpdate,
        ]
    );
    let sequences: Vec<_> = replay.iter().map(|item| item.sequence()).collect();
    assert_eq!(sequences, [0, 1, 2, 3, 4]);
    assert_eq!(
        replay[0].content().map(|content| content.as_str()),
        Some("fixture question")
    );
    assert_eq!(
        replay[2].content().map(|content| content.as_str()),
        Some("fixture answer")
    );
    assert!(
        replay
            .iter()
            .all(|item| item.provider_session_ref().as_provider_value() == FIXTURE_SESSION_REF)
    );
    let debug = format!("{replay:?}");
    assert!(!debug.contains(FIXTURE_SESSION_REF));
    assert!(!debug.contains("fixture answer"));

    assert_eq!(
        session
            .provider_session_ref()
            .map(|reference| reference.as_provider_value()),
        Some(FIXTURE_SESSION_REF)
    );
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    assert_eq!(
        fixture.cleanup_events(),
        [
            CleanupEvent::ProcessWait,
            CleanupEvent::ResourceRelease,
            CleanupEvent::CredentialRelease,
        ]
    );
    let inputs = fixture.inputs();
    let commands: Vec<&str> = inputs
        .iter()
        .filter_map(|value| value["command"].as_str())
        .collect();
    assert_eq!(
        commands,
        [
            "bootstrap",
            "session_switch",
            "session_replay",
            "state",
            "close"
        ]
    );
    assert_eq!(
        inputs[1]["params"]["expectedCwd"],
        "/fixture/pi-sidecar-workspace"
    );
    assert_eq!(inputs[1]["params"]["sessionRef"], FIXTURE_SESSION_REF);
    assert_eq!(inputs[2]["params"]["maxItems"], 1024);
}

#[test]
fn resume_attaches_without_any_replay_phase() {
    let host_id = make_host_id("pi.fixture.sdk-sidecar.resume");
    let fixture = SidecarFixtureHost::new(SidecarScenario::Complete);
    let prepared = prepare_pi_sdk_sidecar_session(
        preparation(host_id.clone(), "sidecar-resume"),
        SessionOptions::default(),
    )
    .expect("sidecar session prepares");
    let binding = fixture_binding(prepared.plan());
    let session = block_on(
        prepared
            .resume_session(
                RequestId::new("sidecar-resume-op").expect("valid request"),
                binding,
                fixture.services(host_id),
            )
            .expect("resume request builds"),
    )
    .expect("sidecar session resumes");
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    let inputs = fixture.inputs();
    let commands: Vec<&str> = inputs
        .iter()
        .filter_map(|value| value["command"].as_str())
        .collect();
    assert_eq!(commands, ["bootstrap", "session_switch", "state", "close"]);
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
fn load_and_resume_reject_switch_drift_before_readiness() {
    for (scenario, code) in [
        (
            SidecarScenario::SessionNotFound,
            "swallowtail.pi.sdk-sidecar.switch_rejected",
        ),
        (
            SidecarScenario::SwitchCwdMismatch,
            "swallowtail.pi.sdk-sidecar.switch_cwd_mismatch",
        ),
        (
            SidecarScenario::SessionSubstituted,
            "swallowtail.pi.sdk-sidecar.session_substituted",
        ),
        (
            SidecarScenario::ReplayFailure,
            "swallowtail.pi.sdk-sidecar.replay_rejected",
        ),
        (
            SidecarScenario::ReplaySequenceGap,
            "swallowtail.pi.sdk-sidecar.replay_sequence_gap",
        ),
        (
            SidecarScenario::ReplayCountMismatch,
            "swallowtail.pi.sdk-sidecar.replay_incomplete",
        ),
        (
            SidecarScenario::ReplayOverflow,
            "swallowtail.pi.sdk-sidecar.replay_overflow",
        ),
    ] {
        let host_id = make_host_id("pi.fixture.sdk-sidecar.load-fail");
        let fixture = SidecarFixtureHost::new(scenario);
        let selected = sidecar_selection(host_id.clone());
        let binding = fixture_binding(&selected.plan);
        let error = block_on(driver(selected.credential.clone()).load_session(
            selected.plan,
            LoadSessionRequest::new(
                RequestId::new("sidecar-load-fail").expect("valid request"),
                binding,
                selected.resource.clone(),
                None,
                agreement(),
            ),
            fixture.services(host_id),
        ))
        .err()
        .expect("load drift fails");
        assert_eq!(error.diagnostic().code(), code, "{scenario:?}");
        assert!(!format!("{error:?}").contains(FIXTURE_SESSION_REF));
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

#[test]
fn replay_items_after_the_replay_response_fail_the_load() {
    // The stray replay item may be caught by the armed collector (count
    // mismatch) or by the disarmed pump (unexpected replay); both fail the
    // load before readiness and return no handle.
    let host_id = make_host_id("pi.fixture.sdk-sidecar.late-replay");
    let fixture = SidecarFixtureHost::new(SidecarScenario::ReplayAfterResponse);
    let selected = sidecar_selection(host_id.clone());
    let binding = fixture_binding(&selected.plan);
    let error = block_on(driver(selected.credential.clone()).load_session(
        selected.plan,
        LoadSessionRequest::new(
            RequestId::new("sidecar-late-replay").expect("valid request"),
            binding,
            selected.resource.clone(),
            None,
            agreement(),
        ),
        fixture.services(host_id),
    ))
    .err()
    .expect("late replay evidence fails the load");
    assert!(
        matches!(
            error.diagnostic().code(),
            "swallowtail.pi.sdk-sidecar.replay_unexpected"
                | "swallowtail.pi.sdk-sidecar.replay_incomplete"
        ),
        "unexpected code {}",
        error.diagnostic().code()
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
fn resume_fails_closed_on_replay_evidence() {
    let host_id = make_host_id("pi.fixture.sdk-sidecar.resume-replay");
    let fixture = SidecarFixtureHost::new(SidecarScenario::ReplayDuringResume);
    let selected = sidecar_selection(host_id.clone());
    let binding = fixture_binding(&selected.plan);
    let error = block_on(driver(selected.credential.clone()).resume_session(
        selected.plan,
        ResumeSessionRequest::new(
            RequestId::new("sidecar-resume-replay").expect("valid request"),
            binding,
            selected.resource.clone(),
            None,
            agreement(),
        ),
        fixture.services(host_id),
    ))
    .err()
    .expect("replay during resume fails");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.pi.sdk-sidecar.replay_unexpected"
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
fn deadline_during_replay_stops_the_load_without_a_handle() {
    let host_id = make_host_id("pi.fixture.sdk-sidecar.load-cancel");
    let fixture = SidecarFixtureHost::new(SidecarScenario::HoldReplay);
    let selected = sidecar_selection(host_id.clone());
    let binding = fixture_binding(&selected.plan);
    let request = LoadSessionRequest::new(
        RequestId::new("sidecar-load-cancel").expect("valid request"),
        binding,
        selected.resource.clone(),
        Some(Deadline::at(MonotonicInstant::from_ticks(500))),
        agreement(),
    );
    let services = fixture.services(host_id);
    let driver = driver(selected.credential.clone());
    let worker = std::thread::spawn(move || {
        block_on(driver.load_session(selected.plan, request, services))
            .err()
            .expect("cancelled load fails")
            .diagnostic()
            .code()
    });
    fixture.wait_for_command("session_replay");
    fixture.advance_time(500);
    let code = worker.join().expect("load thread joins");
    assert_eq!(code, "swallowtail.pi.sdk-sidecar.attach_timed_out");
    assert_eq!(
        fixture.cleanup_events(),
        [
            CleanupEvent::ProcessWait,
            CleanupEvent::ResourceRelease,
            CleanupEvent::CredentialRelease,
        ]
    );
}

fn fixture_binding(plan: &swallowtail_core::PreflightPlan) -> SessionResumeBinding {
    SessionResumeBinding::new(
        SessionRef::new(FIXTURE_SESSION_REF).expect("valid session ref"),
        plan.instance_id().clone(),
        plan.execution_host_id().clone(),
        plan.model_route_id().expect("route").clone(),
        plan.model_id().expect("model").clone(),
        WorkingResourceRef::new("pi.fixture.workspace").expect("valid resource"),
        ambient_read(),
    )
}
