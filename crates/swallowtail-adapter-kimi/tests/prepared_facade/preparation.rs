use super::fixtures::{
    access_profile, access_status, preparation_services, prepared, probe, profile_input, target,
};
use crate::discovery_support::FakeProcessService;
use crate::support::{FixtureHost, Scenario};
use futures_executor::block_on;
use swallowtail_adapter_kimi::{
    KimiCodePreparationInput, KimiCodePreparationProbe, KimiCodePreparedDriver,
    KimiCodePreparedIntegration, KimiPreparationInput, prepare_kimi, prepare_kimi_code,
};
use swallowtail_core::{
    AccessProfileId, AccessStatus, ConfiguredInstanceId, CredentialState, EndpointAuthorization,
    EntitlementState, ExecutionHostId, InstanceRevision, RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{
    Deadline, DiscoveryCancellation, EnvironmentRef, MonotonicInstant, PreparationStage,
    PreparedAccessEvidence, RequestId, ScopeId, SessionOptions, WorkingResourceRef,
};

#[test]
fn installed_solution_facade_keeps_acp_and_headless_selection_explicit() {
    for driver in [
        KimiCodePreparedDriver::Acp,
        KimiCodePreparedDriver::Headless,
    ] {
        let suffix = match driver {
            KimiCodePreparedDriver::Acp => "acp",
            KimiCodePreparedDriver::Headless => "headless",
        };
        let host_id = ExecutionHostId::new(format!("fixture.kimi.code.{suffix}")).unwrap();
        let operation_host = FixtureHost::new(Scenario::Complete);
        let input = KimiCodePreparationInput::new(
            driver,
            ConfiguredInstanceId::new(format!("kimi.code.{suffix}")).unwrap(),
            InstanceRevision::new("1").unwrap(),
            host_id.clone(),
            target(),
            EnvironmentRef::new("kimi.code.environment").unwrap(),
            access_profile(),
            PreparedAccessEvidence::caller_asserted(access_status()),
        );
        let probe = KimiCodePreparationProbe::new(
            RequestId::new(format!("kimi-code-probe-{suffix}")).unwrap(),
            ScopeId::new(format!("kimi-code-probe-{suffix}")).unwrap(),
            Deadline::at(MonotonicInstant::from_ticks(100)),
            DiscoveryCancellation::new(),
        );
        let (process, _) = FakeProcessService::completed("0.29.2\n");
        let prepared = block_on(prepare_kimi_code(
            input,
            probe,
            preparation_services(&operation_host, host_id, process),
        ))
        .expect("selected Kimi Code route prepares");
        assert_eq!(prepared.driver(), driver);
        assert!(matches!(
            (driver, prepared),
            (
                KimiCodePreparedDriver::Acp,
                KimiCodePreparedIntegration::Acp(_)
            ) | (
                KimiCodePreparedDriver::Headless,
                KimiCodePreparedIntegration::Headless(_)
            )
        ));
    }

    let host_id = ExecutionHostId::new("fixture.kimi.code.headless-state").unwrap();
    let operation_host = FixtureHost::new(Scenario::Complete);
    let input = KimiCodePreparationInput::new(
        KimiCodePreparedDriver::Headless,
        ConfiguredInstanceId::new("kimi.code.headless-state").unwrap(),
        InstanceRevision::new("1").unwrap(),
        host_id.clone(),
        target(),
        EnvironmentRef::new("kimi.code.environment").unwrap(),
        access_profile(),
        PreparedAccessEvidence::caller_asserted(access_status()),
    )
    .with_state_root(WorkingResourceRef::new("kimi.code.state-root").unwrap());
    let probe = KimiCodePreparationProbe::new(
        RequestId::new("kimi-code-headless-state").unwrap(),
        ScopeId::new("kimi-code-headless-state").unwrap(),
        Deadline::at(MonotonicInstant::from_ticks(100)),
        DiscoveryCancellation::new(),
    );
    let (process, state) = FakeProcessService::completed("0.29.2\n");
    let failure = block_on(prepare_kimi_code(
        input,
        probe,
        preparation_services(&operation_host, host_id, process),
    ))
    .expect_err("headless route rejects reusable session-state authority");
    assert_eq!(failure.stage(), PreparationStage::TargetSelection);
    assert!(!state.started());
}

#[test]
fn exact_newer_points_fail_closed_before_preparation_effects() {
    let host_id = ExecutionHostId::new("fixture.prepared.newer").unwrap();
    let operation_host = FixtureHost::new(Scenario::Complete);
    let input = KimiPreparationInput::new(
        ConfiguredInstanceId::new("kimi.prepared.newer").unwrap(),
        InstanceRevision::new("1").unwrap(),
        host_id.clone(),
        target(),
        EnvironmentRef::new("kimi.prepared.state").unwrap(),
        access_profile(),
        PreparedAccessEvidence::caller_asserted(access_status()),
    );
    let (process, state) = FakeProcessService::completed("0.39.2\n");
    let failure = block_on(prepare_kimi(
        input,
        probe(),
        preparation_services(&operation_host, host_id, process),
    ))
    .expect_err("QualifiedOnly ACP refuses 0.39.2");
    assert_eq!(
        failure.stage(),
        PreparationStage::CompatibilityClassification
    );
    assert!(state.waited());
    assert!(!operation_host.process_started());

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

#[test]
fn extended_effort_modes_follow_declared_effort_behavior_gate() {
    use swallowtail_core::ReasoningMode;
    use swallowtail_testkit::assert_negotiated_reasoning_setup_contract;

    let host_id = ExecutionHostId::new("fixture.prepared.legacy-extended").unwrap();
    let host = FixtureHost::new(Scenario::ReasoningLegacySuccess);
    let legacy = prepared(&host, host_id, "0.28.1");
    for mode in ["xhigh", "max"] {
        let failure = legacy
            .prepare_session(profile_input(
                &format!("legacy-{mode}"),
                SessionOptions::default().with_reasoning_mode(ReasoningMode::new(mode).unwrap()),
            ))
            .expect_err("legacy boolean ACP rejects extended effort before host effects");
        assert_eq!(failure.stage(), PreparationStage::Preflight);
        assert_eq!(
            failure.diagnostic().safe().code(),
            "swallowtail.kimi.preparation.reasoning_mode_unsupported"
        );
    }
    assert!(!host.process_started());

    let host_id = ExecutionHostId::new("fixture.prepared.declared-extended").unwrap();
    let host = FixtureHost::new(Scenario::ReasoningEffortSuccess);
    let declared = prepared(&host, host_id, "0.29.0");
    for mode in ["xhigh", "max"] {
        let requested = ReasoningMode::new(mode).unwrap();
        let profile = declared
            .prepare_session(profile_input(
                &format!("declared-{mode}"),
                SessionOptions::default().with_reasoning_mode(requested.clone()),
            ))
            .expect("declared-effort ACP prepares extended effort");
        assert_eq!(
            profile.request().options().reasoning_mode(),
            Some(&requested)
        );
        assert_negotiated_reasoning_setup_contract(
            profile.plan(),
            requested,
            ReasoningMode::new("different").unwrap(),
        );
    }
    assert!(!host.process_started());

    let host_id = ExecutionHostId::new("fixture.prepared.foreign-effort").unwrap();
    let host = FixtureHost::new(Scenario::ReasoningEffortExtended);
    let declared = prepared(&host, host_id, "0.29.0");
    let failure = declared
        .prepare_session(profile_input(
            "foreign-ultra",
            SessionOptions::default().with_reasoning_mode(ReasoningMode::new("ultra").unwrap()),
        ))
        .expect_err("foreign advertised effort remains non-admissible");
    assert_eq!(failure.stage(), PreparationStage::Preflight);
    assert_eq!(
        failure.diagnostic().safe().code(),
        "swallowtail.kimi.preparation.reasoning_mode_unsupported"
    );
    assert!(!host.process_started());
}
