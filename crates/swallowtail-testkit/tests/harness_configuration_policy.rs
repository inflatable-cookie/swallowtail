use swallowtail_core::{HarnessConfigurationPosture, PreflightDimension};
use swallowtail_runtime::{OperationPolicy, validate_harness_configuration_policy};
use swallowtail_testkit::{PreflightFixtureCase, RuntimePreflightFixture};

#[test]
fn ambient_configuration_is_explicit_and_request_bound() {
    let fixture =
        RuntimePreflightFixture::for_case(PreflightFixtureCase::HarnessConfigurationAmbient);
    let plan = fixture.preflight().expect("ambient configuration passes");
    let policy = OperationPolicy::offline()
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);

    validate_harness_configuration_policy(&plan, &policy)
        .expect("request posture matches pure preflight");
    assert_eq!(
        plan.harness_configuration_posture(),
        Some(HarnessConfigurationPosture::Ambient)
    );
    assert_eq!(fixture.provider_side_effect_count(), 0);
}

#[test]
fn every_configuration_mismatch_rejects_before_provider_effects() {
    let fixture =
        RuntimePreflightFixture::for_case(PreflightFixtureCase::HarnessConfigurationMismatch);
    let failure = fixture
        .preflight()
        .expect_err("instance posture cannot substitute for the requirement");

    assert_eq!(
        failure.dimension(),
        PreflightDimension::HarnessConfiguration
    );
    assert_eq!(fixture.provider_side_effect_count(), 0);

    let ambient =
        RuntimePreflightFixture::for_case(PreflightFixtureCase::HarnessConfigurationAmbient);
    let plan = ambient.preflight().expect("ambient configuration passes");
    let omitted = OperationPolicy::offline();
    let failure = validate_harness_configuration_policy(&plan, &omitted)
        .expect_err("request policy cannot omit a bound posture");
    assert_eq!(
        failure.diagnostic().message(),
        "Harness configuration does not match the preflight-bound posture"
    );
    assert_eq!(ambient.provider_side_effect_count(), 0);
}

#[test]
fn direct_inference_rejects_harness_configuration() {
    let fixture = RuntimePreflightFixture::for_case(
        PreflightFixtureCase::DirectInferenceHarnessConfiguration,
    );
    let failure = fixture
        .preflight()
        .expect_err("direct inference cannot claim harness configuration");

    assert_eq!(
        failure.dimension(),
        PreflightDimension::HarnessConfiguration
    );
    assert_eq!(fixture.provider_side_effect_count(), 0);
}

#[test]
fn provider_suppression_requires_exact_interface_version_evidence() {
    let fixture = RuntimePreflightFixture::for_case(
        PreflightFixtureCase::ProviderSuppressedWithoutVersionEvidence,
    );
    let failure = fixture
        .preflight()
        .expect_err("provider suppression without exact version evidence cannot pass");

    assert_eq!(
        failure.dimension(),
        PreflightDimension::HarnessConfiguration
    );
    assert_eq!(
        failure.diagnostic().message(),
        "Provider-suppressed harness configuration requires exact interface-version evidence"
    );
    assert_eq!(fixture.provider_side_effect_count(), 0);
}

#[test]
fn host_scoped_configuration_waits_for_a_separate_lease_boundary() {
    let fixture =
        RuntimePreflightFixture::for_case(PreflightFixtureCase::HostScopedHarnessConfiguration);
    let failure = fixture
        .preflight()
        .expect_err("host-scoped configuration cannot run without a host lease");

    assert_eq!(
        failure.dimension(),
        PreflightDimension::HarnessConfiguration
    );
    assert_eq!(
        failure.diagnostic().message(),
        "Host-scoped harness configuration requires a separately bound host lease"
    );
    assert_eq!(fixture.provider_side_effect_count(), 0);
}

#[test]
fn configuration_posture_is_an_immutable_plan_binding() {
    let original = RuntimePreflightFixture::canonical();
    let plan = original.preflight().expect("unmigrated preflight passes");
    let changed = RuntimePreflightFixture::canonical()
        .with_instance_harness_configuration_posture(HarnessConfigurationPosture::Ambient);

    let failure = plan
        .validate_current(&changed.context())
        .expect_err("changing instance posture must stale the plan");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.preflight_plan_stale"
    );
    assert_eq!(original.provider_side_effect_count(), 0);
    assert_eq!(changed.provider_side_effect_count(), 0);
}

#[test]
fn unmigrated_routes_do_not_gain_an_ambient_alias() {
    let fixture = RuntimePreflightFixture::canonical();
    let plan = fixture.preflight().expect("unmigrated preflight passes");

    assert_eq!(plan.harness_configuration_posture(), None);
    validate_harness_configuration_policy(&plan, &OperationPolicy::offline())
        .expect("unbound plan and request remain consistently unbound");
}
