use swallowtail_core::{HarnessIsolation, PreflightDimension};
use swallowtail_runtime::{OperationPolicy, validate_harness_isolation_policy};
use swallowtail_testkit::{PreflightFixtureCase, RuntimePreflightFixture};

#[test]
fn structured_harness_isolation_is_bound_without_provider_effects() {
    let fixture = RuntimePreflightFixture::for_case(PreflightFixtureCase::HarnessIsolationAmbient);
    let plan = fixture
        .preflight()
        .expect("ambient harness preflight succeeds");
    let policy = OperationPolicy::offline().with_harness_isolation(HarnessIsolation::AmbientHost);

    validate_harness_isolation_policy(&plan, &policy)
        .expect("request posture matches pure preflight");
    assert_eq!(fixture.provider_side_effect_count(), 0);
}

#[test]
fn direct_inference_rejects_harness_isolation_during_pure_preflight() {
    let fixture =
        RuntimePreflightFixture::for_case(PreflightFixtureCase::DirectInferenceHarnessIsolation);
    let failure = fixture
        .preflight()
        .expect_err("direct inference cannot claim harness isolation");

    assert_eq!(failure.dimension(), PreflightDimension::HarnessIsolation);
    assert_eq!(fixture.provider_side_effect_count(), 0);
}

#[test]
fn mismatched_isolation_fails_before_provider_effects() {
    let fixture = RuntimePreflightFixture::for_case(PreflightFixtureCase::HarnessIsolationAmbient);
    let plan = fixture
        .preflight()
        .expect("ambient harness preflight succeeds");
    let mismatched =
        OperationPolicy::offline().with_harness_isolation(HarnessIsolation::ProviderEnforced);

    let failure = validate_harness_isolation_policy(&plan, &mismatched)
        .expect_err("enforced posture cannot satisfy an ambient binding");
    assert_eq!(
        failure.diagnostic().message(),
        "Harness isolation does not match the preflight-bound posture"
    );
    assert_eq!(fixture.provider_side_effect_count(), 0);
}

#[test]
fn legacy_structured_runs_remain_unbound_until_their_driver_migrates() {
    let fixture = RuntimePreflightFixture::canonical();
    let plan = fixture.preflight().expect("legacy preflight succeeds");

    validate_harness_isolation_policy(&plan, &OperationPolicy::offline())
        .expect("unmigrated request and plan remain consistently unbound");
    assert_eq!(fixture.provider_side_effect_count(), 0);
}
