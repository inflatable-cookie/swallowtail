use super::fixtures::{prepared, profile_input};
use crate::support::{FixtureHost, Scenario};
use swallowtail_core::{
    Capability, CapabilityConstraint, ExecutionHostId, HarnessIsolation, HarnessMode,
};
use swallowtail_runtime::{RequestId, SessionOptions};

#[test]
fn plan_mode_prepares_for_new_and_is_rejected_for_load_or_resume() {
    let host_id = ExecutionHostId::new("fixture.prepared.plan").unwrap();
    let host = FixtureHost::new(Scenario::PlanSuccess);
    let prepared = prepared(&host, host_id, "0.29.0");
    let profile = prepared
        .prepare_session(profile_input(
            "plan",
            SessionOptions::default().with_harness_mode(HarnessMode::Plan),
        ))
        .expect("plan-mode session prepares");
    assert_eq!(
        profile.request().options().harness_mode(),
        Some(HarnessMode::Plan)
    );
    assert_eq!(
        profile.plan().requirements().harness_isolation(),
        Some(HarnessIsolation::AmbientHost)
    );
    assert!(
        profile
            .plan()
            .requirements()
            .capabilities()
            .any(|requirement| {
                requirement.capability() == Capability::HarnessModeSelection
                    && requirement.constraints().any(|constraint| {
                        matches!(
                            constraint,
                            CapabilityConstraint::HarnessMode(HarnessMode::Plan)
                        )
                    })
            })
    );
    let binding = swallowtail_runtime::SessionResumeBinding::new(
        swallowtail_core::SessionRef::new("kimi-session-bound").unwrap(),
        profile.plan().instance_id().clone(),
        profile.plan().execution_host_id().clone(),
        profile.plan().model_route_id().unwrap().clone(),
        profile.plan().model_id().unwrap().clone(),
        profile.request().working_resource().unwrap().clone(),
        profile.request().access_policy().clone(),
    );
    assert_eq!(
        profile
            .load_request(RequestId::new("plan-load").unwrap(), binding.clone())
            .expect_err("load cannot redeclare harness mode")
            .diagnostic()
            .safe()
            .code(),
        "swallowtail.kimi.preparation.attachment_harness_mode_unsupported"
    );
    assert_eq!(
        profile
            .resume_request(RequestId::new("plan-resume").unwrap(), binding)
            .expect_err("resume cannot redeclare harness mode")
            .diagnostic()
            .safe()
            .code(),
        "swallowtail.kimi.preparation.attachment_harness_mode_unsupported"
    );
    assert!(!host.process_started());
}

#[test]
fn plan_mode_composes_with_admitted_reasoning_on_prepare() {
    let host_id = ExecutionHostId::new("fixture.prepared.plan-reasoning").unwrap();
    let host = FixtureHost::new(Scenario::ReasoningEffortSuccess);
    let prepared = prepared(&host, host_id, "0.29.0");
    let profile = prepared
        .prepare_session(profile_input(
            "plan-reasoning",
            SessionOptions::default()
                .with_reasoning_mode(swallowtail_core::ReasoningMode::new("max").unwrap())
                .with_harness_mode(HarnessMode::Plan),
        ))
        .expect("plan mode composes with admitted reasoning");
    assert_eq!(
        profile
            .request()
            .options()
            .reasoning_mode()
            .map(swallowtail_core::ReasoningMode::as_str),
        Some("max")
    );
    assert_eq!(
        profile.request().options().harness_mode(),
        Some(HarnessMode::Plan)
    );
    assert!(!host.process_started());
}
