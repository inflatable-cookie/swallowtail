use super::fixtures::{
    access_profile, preparation_services, prepared, probe, profile_input, target,
};
use crate::discovery_support::FakeProcessService;
use crate::support::{FixtureHost, Scenario};
use futures_executor::block_on;
use swallowtail_adapter_kimi::{KimiPreparationInput, prepare_kimi};
use swallowtail_core::{
    AccessProfileId, AccessStatus, ConfiguredInstanceId, CredentialState, EndpointAuthorization,
    EntitlementState, ExecutionHostId, InstanceRevision, RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{
    EnvironmentRef, PreparationStage, PreparedAccessEvidence, RequestId, SessionOptions,
};

#[test]
fn exact_newer_evidence_and_preparation_failures_remain_visible_before_effects() {
    let host_id = ExecutionHostId::new("fixture.prepared.newer").unwrap();
    let host = FixtureHost::new(Scenario::ReasoningNewerSuccess);
    let prepared = prepared(&host, host_id.clone(), "0.30.0");
    assert!(!prepared.observation().is_qualified());
    let profile = prepared
        .prepare_session(profile_input(
            "newer",
            SessionOptions::default()
                .with_reasoning_mode(swallowtail_core::ReasoningMode::new("high").unwrap()),
        ))
        .expect("unverified newer Kimi prepares");
    assert!(!profile.evidence().observation().is_qualified());
    let session =
        block_on(profile.open_session(host.services(host_id))).expect("newer session opens");
    assert_eq!(
        block_on(session.close()),
        swallowtail_runtime::CleanupOutcome::Clean
    );

    let host_id = ExecutionHostId::new("fixture.prepared.rejected").unwrap();
    let operation_host = FixtureHost::new(Scenario::Complete);
    let input = KimiPreparationInput::new(
        ConfiguredInstanceId::new("kimi.prepared.rejected").unwrap(),
        InstanceRevision::new("1").unwrap(),
        host_id.clone(),
        target(),
        EnvironmentRef::new("kimi.prepared.state").unwrap(),
        access_profile(),
        PreparedAccessEvidence::caller_asserted(AccessStatus::new(
            AccessProfileId::new("different.access").unwrap(),
            CredentialState::Ready,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::IntegrationMaintainerSupported,
        )),
    );
    let (process, state) = FakeProcessService::completed("0.29.0\n");
    let result = block_on(prepare_kimi(
        input,
        probe(),
        preparation_services(&operation_host, host_id, process),
    ));
    let failure = result.expect_err("access mismatch rejects preparation");
    assert_eq!(failure.stage(), PreparationStage::AccessEvidence);
    assert!(!state.started());
    assert!(!operation_host.process_started());
}

#[test]
fn reasoning_is_explicit_for_new_and_rejected_for_load_or_resume() {
    let host_id = ExecutionHostId::new("fixture.prepared.reasoning").unwrap();
    let host = FixtureHost::new(Scenario::ReasoningEffortSuccess);
    let prepared = prepared(&host, host_id, "0.29.0");
    let profile = prepared
        .prepare_session(profile_input(
            "reasoning",
            SessionOptions::default()
                .with_reasoning_mode(swallowtail_core::ReasoningMode::new("high").unwrap()),
        ))
        .expect("reasoning session prepares");
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
            .load_request(RequestId::new("reasoning-load").unwrap(), binding.clone())
            .expect_err("load cannot redeclare reasoning")
            .stage(),
        PreparationStage::Preflight
    );
    assert_eq!(
        profile
            .resume_request(RequestId::new("reasoning-resume").unwrap(), binding)
            .expect_err("resume cannot redeclare reasoning")
            .stage(),
        PreparationStage::Preflight
    );
    assert!(!host.process_started());
}
