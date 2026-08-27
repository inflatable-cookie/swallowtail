use super::{driver, make_host_id};
use crate::support::{
    SidecarFixtureHost, SidecarScenario, reasoning_options, sidecar_open_request,
    sidecar_reasoning_selection, sidecar_selection,
};
use futures_executor::block_on;
use swallowtail_adapter_pi::{PiSdkSidecarSessionPreparation, prepare_pi_sdk_sidecar_session};
use swallowtail_core::{
    AccessProfileId, Capability, ConfiguredInstanceId, ExecutionHostId, InstanceRevision,
    InstanceTargetRef, ModelId, ModelRouteId, ModelRouteRevision, ProviderId,
};
use swallowtail_runtime::{
    CleanupOutcome, EnvironmentRef, InteractiveSessionDriver, PreparationFailure, RequestId,
    WorkingResourceRef,
};

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

#[test]
fn qualified_reasoning_prepares_and_dispatches_thinking_level() {
    let host_id = make_host_id("pi.fixture.sdk-sidecar.reasoning");
    let prepared = prepare_pi_sdk_sidecar_session(
        qualified_preparation(host_id.clone(), "sidecar-reasoning-prepare"),
        reasoning_options("medium"),
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
            .map(|mode| mode.as_str()),
        Some("medium")
    );

    let fixture = SidecarFixtureHost::new(SidecarScenario::Complete);
    let services = fixture.services(host_id);
    let session =
        block_on(prepared.open_session(services.clone())).expect("reasoning session opens");
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);

    let bootstrap = &fixture.inputs()[0];
    assert_eq!(bootstrap["params"]["thinkingLevel"], "medium");
    assert_eq!(bootstrap["params"]["provider"], "anthropic");
    assert_eq!(bootstrap["params"]["model"], "claude-opus-4-5");
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
fn thinking_level_mismatch_fails_before_provider_work() {
    let host_id = make_host_id("pi.fixture.sdk-sidecar.reasoning-mismatch");
    let fixture = SidecarFixtureHost::new(SidecarScenario::ThinkingMismatch);
    let selected = sidecar_reasoning_selection(host_id.clone(), "medium");
    let services = fixture.services(host_id);
    let error = block_on(
        driver(selected.credential.clone()).open_session(
            selected.plan,
            sidecar_open_request("sidecar-reasoning-mismatch", selected.resource)
                .with_options(reasoning_options("medium")),
            services,
        ),
    )
    .err()
    .expect("effective drift fails closed");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.pi.sdk-sidecar.bootstrap_mismatch"
    );
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
