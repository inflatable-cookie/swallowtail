use swallowtail_adapter_kimi::{kimi_acp_claim, kimi_acp_descriptor};
use swallowtail_core::{
    DriverRole, ExecutionLayer, HostServiceKind, InterfaceVersion, OperationShape, ReasoningMode,
};
use swallowtail_runtime::SessionOptions;
use swallowtail_testkit::{
    ClosedSemanticWindowCase, ConformanceAssertion, SyntheticProfile,
    assert_closed_semantic_compatibility_window, assert_unverified_newer_execution,
    run_persistent_acp_profile,
};

#[test]
fn unchanged_persistent_acp_profile_covers_kimi_authority_boundaries() {
    let report = run_persistent_acp_profile();
    assert_eq!(report.profile(), SyntheticProfile::PersistentAcpHarness);
    for assertion in [
        ConformanceAssertion::PreflightBeforeSideEffects,
        ConformanceAssertion::NoImplicitFallback,
        ConformanceAssertion::Redaction,
        ConformanceAssertion::PersistentSessionLifecycle,
        ConformanceAssertion::ReplayPhase,
        ConformanceAssertion::WorkingResourceWriteCallback,
        ConformanceAssertion::AmbientHarnessAuthority,
        ConformanceAssertion::DelegatedAuthentication,
        ConformanceAssertion::HostTopologyPreserved,
    ] {
        assert!(report.covers(assertion));
    }
}

#[test]
fn qualified_kimi_milestones_compose_with_shared_compatibility_assertions() {
    let claim = kimi_acp_claim();
    let case = ClosedSemanticWindowCase::new(version("0.28.1"), version("0.38.0"))
        .with_accepted([
            version("0.29.0"),
            version("0.29.1"),
            version("0.29.2"),
            version("0.30.0"),
            version("0.31.0"),
            version("0.31.1"),
            version("0.32.0"),
            version("0.34.0"),
            version("0.36.0"),
            version("0.37.0"),
            version("0.37.2"),
            version("0.38.0"),
        ])
        .with_rejected([version("0.28.0"), version("0.28.2"), version("0.29.0-rc.1")]);
    assert_closed_semantic_compatibility_window(&claim, &case);
    assert_unverified_newer_execution(&claim, &version("0.38.1"));
}

#[test]
fn descriptor_keeps_discovery_access_and_isolation_separate() {
    let descriptor = kimi_acp_descriptor();
    assert!(descriptor.supports_role(DriverRole::Discovery));
    assert!(descriptor.supports_role(DriverRole::InteractiveSession));
    assert!(descriptor.supports_execution_layer(ExecutionLayer::HarnessInteraction));
    assert!(descriptor.supports_operation_shape(OperationShape::InteractiveSession));

    let discovery = descriptor
        .required_host_services(DriverRole::Discovery)
        .collect::<Vec<_>>();
    assert_eq!(discovery.len(), 3);
    for required in [
        HostServiceKind::Task,
        HostServiceKind::Time,
        HostServiceKind::Process,
    ] {
        assert!(discovery.contains(&required));
    }
    for forbidden in [
        HostServiceKind::Credential,
        HostServiceKind::WorkingResource,
        HostServiceKind::WorkingResourceIo,
        HostServiceKind::Network,
    ] {
        assert!(!discovery.contains(&forbidden));
    }

    let session = descriptor
        .required_host_services(DriverRole::InteractiveSession)
        .collect::<Vec<_>>();
    assert!(!session.contains(&HostServiceKind::Network));
}

#[test]
fn portable_reasoning_option_has_no_provider_configuration_record() {
    let options = SessionOptions::default()
        .with_reasoning_mode(ReasoningMode::new("high").expect("valid reasoning mode"));
    let debug = format!("{options:?}");
    assert!(!debug.contains("thinking"));
    assert!(!debug.contains("configOptions"));
    assert!(!debug.contains("set_config_option"));
}

#[test]
fn portable_plan_mode_option_has_no_provider_configuration_record() {
    let options = SessionOptions::default().with_harness_mode(swallowtail_core::HarnessMode::Plan);
    let debug = format!("{options:?}");
    assert!(!debug.contains("configOptions"));
    assert!(!debug.contains("set_config_option"));
    assert!(!debug.contains("auto"));
    assert!(!debug.contains("yolo"));
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("valid Kimi version")
}
