mod support;

use futures_executor::block_on;
use support::{CleanupEvent, FixtureHost, Scenario, reasoning_selection, version_selection};
use swallowtail_adapter_kimi::KimiAcpDriver;
use swallowtail_core::{
    ExecutionHostId, ReasoningMode, ResourceAccess, SessionProviderStatePolicy, SessionRef,
};
use swallowtail_runtime::{
    InteractiveSessionDriver, LoadSessionRequest, OpenSessionRequest, RequestId,
    ResumeSessionRequest, SessionAccessPolicy, SessionOptions, SessionPlanAgreement,
    SessionResumeBinding, WorkingResourceRef,
};

#[test]
fn provider_rejection_and_handshake_drift_abort_joined_work() {
    let host_id =
        ExecutionHostId::new("fixture.host.reasoning.provider-reject").expect("valid host id");
    let selected = reasoning_selection(host_id.clone(), "0.29.0", "high");
    let host = FixtureHost::new(Scenario::ReasoningRejected);
    assert!(
        block_on(driver(selected.credential).open_session(
            selected.plan,
            reasoning_open_request("kimi-provider-rejected", selected.resource),
            host.services(host_id),
        ))
        .is_err()
    );
    assert_eq!(host.cleanup_events(), joined_cleanup());

    let host_id =
        ExecutionHostId::new("fixture.host.reasoning.handshake-drift").expect("valid host id");
    let selected = version_selection(host_id.clone(), "0.29.0");
    let host = FixtureHost::new(Scenario::ReasoningLegacySuccess);
    let error = block_on(driver(selected.credential).open_session(
        selected.plan,
        empty_open_request("kimi-handshake-drift", selected.resource),
        host.services(host_id),
    ))
    .err()
    .expect("handshake drift rejects");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.kimi.acp.agent_version_rejected"
    );
    assert_eq!(host.wire_methods(), ["initialize"]);
    assert_eq!(host.cleanup_events(), joined_cleanup());
}

#[test]
fn load_and_resume_reject_reasoning_before_host_effects() {
    for operation in ["load", "resume"] {
        let host_id = ExecutionHostId::new(format!("fixture.host.reasoning.{operation}"))
            .expect("valid host id");
        let selected = reasoning_selection(host_id.clone(), "0.29.0", "high");
        let binding = binding(&selected.plan, selected.resource.clone());
        let options = reasoning_options();
        let host = FixtureHost::new(Scenario::ReasoningEffortSuccess);
        let error = if operation == "load" {
            block_on(
                driver(selected.credential).load_session(
                    selected.plan,
                    LoadSessionRequest::new(
                        RequestId::new("kimi-reasoning-load").expect("valid request"),
                        binding,
                        selected.resource,
                        None,
                        SessionPlanAgreement::explicit(
                            policy(),
                            Some(SessionProviderStatePolicy::Prohibited),
                            None,
                        ),
                    )
                    .with_options(options),
                    host.services(host_id),
                ),
            )
            .err()
            .expect("load reasoning rejects")
        } else {
            block_on(
                driver(selected.credential).resume_session(
                    selected.plan,
                    ResumeSessionRequest::new(
                        RequestId::new("kimi-reasoning-resume").expect("valid request"),
                        binding,
                        selected.resource,
                        None,
                        SessionPlanAgreement::explicit(
                            policy(),
                            Some(SessionProviderStatePolicy::Prohibited),
                            None,
                        ),
                    )
                    .with_options(options),
                    host.services(host_id),
                ),
            )
            .err()
            .expect("resume reasoning rejects")
        };
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.negotiated_reasoning.lifecycle_rejected"
        );
        assert_eq!(host.credential_acquisitions(), 0);
        assert!(!host.process_started());
    }
}

fn driver(credential: swallowtail_core::CredentialRef) -> KimiAcpDriver {
    KimiAcpDriver::new(
        swallowtail_runtime::EnvironmentRef::new("kimi.fixture.isolated-state")
            .expect("valid environment"),
        credential,
    )
}

fn empty_open_request(id: &str, resource: WorkingResourceRef) -> OpenSessionRequest {
    OpenSessionRequest::new(
        RequestId::new(id).expect("valid request"),
        resource,
        None,
        SessionPlanAgreement::explicit(
            policy(),
            Some(SessionProviderStatePolicy::Prohibited),
            None,
        ),
    )
}

fn reasoning_open_request(id: &str, resource: WorkingResourceRef) -> OpenSessionRequest {
    empty_open_request(id, resource).with_options(reasoning_options())
}

fn reasoning_options() -> SessionOptions {
    SessionOptions::default().with_reasoning_mode(ReasoningMode::new("high").expect("valid mode"))
}

fn binding(
    plan: &swallowtail_core::PreflightPlan,
    resource: WorkingResourceRef,
) -> SessionResumeBinding {
    SessionResumeBinding::new(
        SessionRef::new("kimi-session-bound").expect("valid session"),
        plan.instance_id().clone(),
        plan.execution_host_id().clone(),
        plan.model_route_id().expect("route exists").clone(),
        plan.model_id().expect("model exists").clone(),
        resource,
        policy(),
    )
}

fn policy() -> SessionAccessPolicy {
    SessionAccessPolicy::ambient_harness(ResourceAccess::ReadWrite)
}

fn joined_cleanup() -> [CleanupEvent; 3] {
    [
        CleanupEvent::ProcessWait,
        CleanupEvent::ResourceRelease,
        CleanupEvent::CredentialRelease,
    ]
}
