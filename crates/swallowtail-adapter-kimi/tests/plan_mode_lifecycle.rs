use crate::support;

use futures_executor::block_on;
use support::{CleanupEvent, FixtureHost, Scenario, plan_selection};
use swallowtail_adapter_kimi::KimiAcpDriver;
use swallowtail_core::{
    ExecutionHostId, HarnessMode, ResourceAccess, SessionProviderStatePolicy, SessionRef,
};
use swallowtail_runtime::{
    InteractiveSessionDriver, LoadSessionRequest, OpenSessionRequest, RequestId,
    ResumeSessionRequest, SessionAccessPolicy, SessionOptions, SessionPlanAgreement,
    SessionResumeBinding, WorkingResourceRef,
};

#[test]
fn provider_rejection_aborts_joined_work() {
    let host_id = ExecutionHostId::new("fixture.host.plan.provider-reject").expect("valid host id");
    let selected = plan_selection(host_id.clone(), "0.29.0");
    let host = FixtureHost::new(Scenario::PlanRejected);
    assert!(
        block_on(
            driver(selected.credential).open_session(
                selected.plan,
                OpenSessionRequest::new(
                    RequestId::new("kimi-plan-provider-rejected").expect("valid request"),
                    selected.resource,
                    None,
                    SessionPlanAgreement::explicit(
                        policy(),
                        Some(SessionProviderStatePolicy::Prohibited),
                        None,
                    ),
                )
                .with_options(SessionOptions::default().with_harness_mode(HarnessMode::Plan)),
                host.services(host_id),
            )
        )
        .is_err()
    );
    assert_eq!(host.cleanup_events(), joined_cleanup());
}

#[test]
fn load_and_resume_reject_harness_mode_before_host_effects() {
    for operation in ["load", "resume"] {
        let host_id =
            ExecutionHostId::new(format!("fixture.host.plan.{operation}")).expect("valid host id");
        let selected = plan_selection(host_id.clone(), "0.29.0");
        let binding = binding(&selected.plan, selected.resource.clone());
        let options = SessionOptions::default().with_harness_mode(HarnessMode::Plan);
        let host = FixtureHost::new(Scenario::PlanSuccess);
        let error = if operation == "load" {
            block_on(
                driver(selected.credential).load_session(
                    selected.plan,
                    LoadSessionRequest::new(
                        RequestId::new("kimi-plan-load").expect("valid request"),
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
            .expect("load harness mode rejects")
        } else {
            block_on(
                driver(selected.credential).resume_session(
                    selected.plan,
                    ResumeSessionRequest::new(
                        RequestId::new("kimi-plan-resume").expect("valid request"),
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
            .expect("resume harness mode rejects")
        };
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.kimi.acp.attachment_harness_mode_unsupported"
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
