use super::{driver, make_host_id};
use crate::support::{
    FIXTURE_SESSION_REF, SidecarFixtureHost, SidecarScenario, reasoning_options,
    sidecar_open_request, sidecar_reasoning_selection, sidecar_selection,
};
use futures_executor::block_on;
use swallowtail_adapter_pi::{PiSdkSidecarSessionPreparation, prepare_pi_sdk_sidecar_session};
use swallowtail_core::{
    AccessProfileId, Capability, ConfiguredInstanceId, ExecutionHostId, InstanceRevision,
    InstanceTargetRef, ModelId, ModelRouteId, ModelRouteRevision, ProviderId, ResourceAccess,
    SessionAccessPolicy, SessionProviderStatePolicy, SessionRef,
};
use swallowtail_runtime::{
    CleanupOutcome, EnvironmentRef, InteractiveSessionDriver, LoadSessionRequest,
    PreparationFailure, RequestId, ResumeSessionRequest, SessionPlanAgreement, SessionReplayKind,
    SessionResumeBinding, WorkingResourceRef,
};

const ADMITTED_MODES: [&str; 5] = ["off", "minimal", "low", "medium", "high"];

fn qualified_preparation(
    host: ExecutionHostId,
    request_id: &str,
) -> PiSdkSidecarSessionPreparation {
    PiSdkSidecarSessionPreparation::new(
        ConfiguredInstanceId::new("pi.fixture.sdk-sidecar.instance").expect("valid instance"),
        InstanceRevision::new("fixture-revision").expect("valid revision"),
        host,
        InstanceTargetRef::new("pi.fixture.pinned-launch-recipe").expect("valid target"),
        EnvironmentRef::new("pi.fixture.environment").expect("valid environment"),
        swallowtail_core::CredentialRef::new("pi.fixture.delegated-auth")
            .expect("valid credential"),
        AccessProfileId::new("pi.fixture.harness-auth").expect("valid access"),
        ModelRouteId::new("pi.fixture.route").expect("valid route"),
        ModelRouteRevision::new("fixture-route-revision").expect("valid route revision"),
        ProviderId::new("anthropic").expect("valid provider"),
        ModelId::new("claude-opus-4-5").expect("valid model"),
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

fn reasoning_binding(plan: &swallowtail_core::PreflightPlan) -> SessionResumeBinding {
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

#[test]
fn admitted_modes_prepare_bootstrap_and_confirm_effective_state() {
    for mode in ADMITTED_MODES {
        let host_id = make_host_id(&format!("pi.fixture.sdk-sidecar.reasoning-{mode}"));
        let prepared = prepare_pi_sdk_sidecar_session(
            qualified_preparation(host_id.clone(), &format!("sidecar-reasoning-{mode}")),
            reasoning_options(mode),
        )
        .expect("qualified reasoning prepares");
        assert!(
            prepared
                .plan()
                .requirements()
                .capabilities()
                .any(|required| required.capability() == Capability::ReasoningSelection)
        );
        assert_eq!(
            prepared
                .request()
                .options()
                .reasoning_mode()
                .map(|selected| selected.as_str()),
            Some(mode)
        );

        let fixture = SidecarFixtureHost::new(SidecarScenario::Complete);
        let services = fixture.services(host_id);
        let session = block_on(prepared.open_session(services))
            .unwrap_or_else(|error| panic!("{mode} reasoning session opens: {error:?}"));
        assert_eq!(block_on(session.close()), CleanupOutcome::Clean);

        let bootstrap = &fixture.inputs()[0];
        assert_eq!(bootstrap["params"]["thinkingLevel"], mode);
        assert_eq!(bootstrap["params"]["provider"], "anthropic");
        assert_eq!(bootstrap["params"]["model"], "claude-opus-4-5");
        let inputs = fixture.inputs();
        let commands: Vec<&str> = inputs
            .iter()
            .filter_map(|value| value["command"].as_str())
            .collect();
        assert!(
            commands.contains(&"state"),
            "{mode} should confirm effective state"
        );
    }
}

#[test]
fn omission_retains_exact_bootstrap_without_thinking_level() {
    let host_id = make_host_id("pi.fixture.sdk-sidecar.reasoning-omit");
    let fixture = SidecarFixtureHost::new(SidecarScenario::Complete);
    let selected = sidecar_selection(host_id.clone());
    let services = fixture.services(host_id);
    let session = block_on(driver(selected.credential.clone()).open_session(
        selected.plan,
        sidecar_open_request("sidecar-reasoning-omit", selected.resource),
        services,
    ))
    .expect("omission session opens");
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    let bootstrap = &fixture.inputs()[0];
    assert!(bootstrap["params"].get("thinkingLevel").is_none());
}

#[test]
fn load_transports_reasoning_through_switch_replay_and_state() {
    let mode = "high";
    let host_id = make_host_id("pi.fixture.sdk-sidecar.reasoning-load");
    let fixture = SidecarFixtureHost::new(SidecarScenario::Complete);
    let prepared = prepare_pi_sdk_sidecar_session(
        qualified_preparation(host_id.clone(), "sidecar-reasoning-load"),
        reasoning_options(mode),
    )
    .expect("reasoning session prepares");
    let binding = reasoning_binding(prepared.plan());
    let loaded = block_on(
        prepared
            .load_session(
                RequestId::new("sidecar-reasoning-load-op").expect("valid request"),
                binding,
                fixture.services(host_id),
            )
            .expect("load request builds"),
    )
    .expect("reasoning session loads");
    let (replay, session) = loaded.into_parts();
    assert_eq!(
        replay.iter().map(|item| item.kind()).collect::<Vec<_>>(),
        [
            SessionReplayKind::UserMessage,
            SessionReplayKind::AgentReasoning,
            SessionReplayKind::AgentMessage,
            SessionReplayKind::ToolCall,
            SessionReplayKind::ToolCallUpdate,
        ]
    );
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);

    let bootstrap = &fixture.inputs()[0];
    assert_eq!(bootstrap["params"]["thinkingLevel"], mode);
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
}

#[test]
fn resume_attaches_reasoning_without_replay() {
    let mode = "low";
    let host_id = make_host_id("pi.fixture.sdk-sidecar.reasoning-resume");
    let fixture = SidecarFixtureHost::new(SidecarScenario::Complete);
    let prepared = prepare_pi_sdk_sidecar_session(
        qualified_preparation(host_id.clone(), "sidecar-reasoning-resume"),
        reasoning_options(mode),
    )
    .expect("reasoning session prepares");
    let binding = reasoning_binding(prepared.plan());
    let session = block_on(
        prepared
            .resume_session(
                RequestId::new("sidecar-reasoning-resume-op").expect("valid request"),
                binding,
                fixture.services(host_id),
            )
            .expect("resume request builds"),
    )
    .expect("reasoning session resumes");
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);

    let inputs = fixture.inputs();
    let commands: Vec<&str> = inputs
        .iter()
        .filter_map(|value| value["command"].as_str())
        .collect();
    assert_eq!(commands, ["bootstrap", "session_switch", "state", "close"]);
    assert_eq!(inputs[0]["params"]["thinkingLevel"], mode);
}

#[test]
fn bootstrap_thinking_level_mismatch_fails_before_provider_work() {
    let host_id = make_host_id("pi.fixture.sdk-sidecar.reasoning-bootstrap-mismatch");
    let fixture = SidecarFixtureHost::new(SidecarScenario::ThinkingBootstrapMismatch);
    let selected = sidecar_reasoning_selection(host_id.clone(), "medium");
    let services = fixture.services(host_id);
    let error = block_on(
        driver(selected.credential.clone()).open_session(
            selected.plan,
            sidecar_open_request("sidecar-reasoning-bootstrap-mismatch", selected.resource)
                .with_options(reasoning_options("medium")),
            services,
        ),
    )
    .err()
    .expect("bootstrap drift fails closed");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.pi.sdk-sidecar.bootstrap_mismatch"
    );
    let inputs = fixture.inputs();
    let commands: Vec<&str> = inputs
        .iter()
        .filter_map(|value| value["command"].as_str())
        .collect();
    assert_eq!(commands, ["bootstrap"]);
}

#[test]
fn post_switch_state_thinking_drift_fails_before_readiness() {
    for scenario in [
        SidecarScenario::ThinkingStateMismatch,
        SidecarScenario::ThinkingStateMissing,
    ] {
        for attach in ["load", "resume"] {
            let host_id = make_host_id(&format!(
                "pi.fixture.sdk-sidecar.reasoning-state-{attach}-{scenario:?}"
            ));
            let fixture = SidecarFixtureHost::new(scenario);
            let selected = sidecar_reasoning_selection(host_id.clone(), "medium");
            let binding = reasoning_binding(&selected.plan);
            let services = fixture.services(host_id);
            let error = if attach == "load" {
                block_on(
                    driver(selected.credential.clone()).load_session(
                        selected.plan,
                        LoadSessionRequest::new(
                            RequestId::new("sidecar-reasoning-state-load").expect("valid request"),
                            binding,
                            selected.resource.clone(),
                            None,
                            agreement(),
                        )
                        .with_options(reasoning_options("medium")),
                        services,
                    ),
                )
                .err()
            } else {
                block_on(
                    driver(selected.credential.clone()).resume_session(
                        selected.plan,
                        ResumeSessionRequest::new(
                            RequestId::new("sidecar-reasoning-state-resume")
                                .expect("valid request"),
                            binding,
                            selected.resource.clone(),
                            None,
                            agreement(),
                        )
                        .with_options(reasoning_options("medium")),
                        services,
                    ),
                )
                .err()
            }
            .expect("state drift fails closed");
            assert_eq!(
                error.diagnostic().code(),
                "swallowtail.pi.sdk-sidecar.state_mismatch",
                "{scenario:?} {attach}"
            );
            let inputs = fixture.inputs();
            let commands: Vec<&str> = inputs
                .iter()
                .filter_map(|value| value["command"].as_str())
                .collect();
            assert!(
                commands.contains(&"session_switch"),
                "{scenario:?} {attach} should reach switch before state check"
            );
            assert!(
                !commands.contains(&"close"),
                "{scenario:?} {attach} should fail before session handle"
            );
        }
    }
}

#[test]
fn open_session_state_thinking_drift_fails_after_bootstrap() {
    let host_id = make_host_id("pi.fixture.sdk-sidecar.reasoning-open-state");
    let fixture = SidecarFixtureHost::new(SidecarScenario::ThinkingStateMismatch);
    let selected = sidecar_reasoning_selection(host_id.clone(), "medium");
    let services = fixture.services(host_id);
    let error = block_on(
        driver(selected.credential.clone()).open_session(
            selected.plan,
            sidecar_open_request("sidecar-reasoning-open-state", selected.resource)
                .with_options(reasoning_options("medium")),
            services,
        ),
    )
    .err()
    .expect("open state drift fails closed");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.pi.sdk-sidecar.state_mismatch"
    );
    let inputs = fixture.inputs();
    let commands: Vec<&str> = inputs
        .iter()
        .filter_map(|value| value["command"].as_str())
        .collect();
    assert_eq!(commands, ["bootstrap", "state"]);
}

#[test]
fn request_plan_disagreement_rejects_before_process_effects() {
    let host_id = make_host_id("pi.fixture.sdk-sidecar.reasoning-plan-mismatch");
    let fixture = SidecarFixtureHost::new(SidecarScenario::Complete);
    let selected = sidecar_reasoning_selection(host_id.clone(), "medium");
    let services = fixture.services(host_id);
    let error = block_on(
        driver(selected.credential.clone()).open_session(
            selected.plan,
            sidecar_open_request("sidecar-reasoning-plan-mismatch", selected.resource)
                .with_options(reasoning_options("high")),
            services,
        ),
    )
    .err()
    .expect("request/plan disagreement fails closed");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.pi.sdk-sidecar.request_plan_mismatch"
    );
    assert!(!fixture.process_started());
}

#[test]
fn unsupported_reasoning_rejects_at_preparation() {
    let host_id = make_host_id("pi.fixture.sdk-sidecar.reasoning-unsupported");
    let error = prepare_pi_sdk_sidecar_session(
        qualified_preparation(host_id, "sidecar-reasoning-unsupported"),
        reasoning_options("xhigh"),
    )
    .err()
    .expect("unsupported mode rejects before effects");
    assert!(matches!(error, PreparationFailure { .. }));
}

#[test]
fn unsupported_foreign_model_rejects_at_preparation() {
    let host_id = make_host_id("pi.fixture.sdk-sidecar.reasoning-foreign-model");
    let preparation = PiSdkSidecarSessionPreparation::new(
        ConfiguredInstanceId::new("pi.fixture.sdk-sidecar.instance").expect("valid instance"),
        InstanceRevision::new("fixture-revision").expect("valid revision"),
        host_id,
        InstanceTargetRef::new("pi.fixture.pinned-launch-recipe").expect("valid target"),
        EnvironmentRef::new("pi.fixture.environment").expect("valid environment"),
        swallowtail_core::CredentialRef::new("pi.fixture.delegated-auth")
            .expect("valid credential"),
        AccessProfileId::new("pi.fixture.harness-auth").expect("valid access"),
        ModelRouteId::new("pi.fixture.route").expect("valid route"),
        ModelRouteRevision::new("fixture-route-revision").expect("valid route revision"),
        ProviderId::new("anthropic").expect("valid provider"),
        ModelId::new("claude-opus-4-7").expect("valid model"),
        WorkingResourceRef::new("pi.fixture.workspace").expect("valid resource"),
        RequestId::new("sidecar-reasoning-foreign").expect("valid request"),
    );
    let error = prepare_pi_sdk_sidecar_session(preparation, reasoning_options("medium"))
        .err()
        .expect("foreign model rejects before effects");
    assert!(matches!(error, PreparationFailure { .. }));
}
