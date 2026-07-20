use crate::{PreflightFixtureCase, RuntimePreflightFixture};
use swallowtail_core::PreflightDimension;

const PREFLIGHT_RULE: &str = "Contract 008 side-effect-free preflight";
const PLAN_RULE: &str = "Contract 008 immutable preflight binding";

pub fn assert_preflight_rejection_without_side_effects(
    case: PreflightFixtureCase,
    expected_dimension: PreflightDimension,
) {
    let fixture = RuntimePreflightFixture::for_case(case);
    let failure = fixture.preflight().unwrap_err();

    assert_eq!(
        failure.dimension(),
        expected_dimension,
        "{PREFLIGHT_RULE}: {case:?} rejected the wrong dimension"
    );
    assert_eq!(
        fixture.provider_side_effect_count(),
        0,
        "{PREFLIGHT_RULE}: {case:?} reached the provider boundary"
    );
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.preflight_rejected",
        "{PREFLIGHT_RULE}: {case:?} returned an unstable diagnostic code"
    );
}

pub fn assert_successful_preflight_binding() {
    let fixture = RuntimePreflightFixture::canonical();
    let plan = fixture
        .preflight()
        .unwrap_or_else(|error| panic!("{PLAN_RULE}: canonical preflight failed: {error}"));

    assert_eq!(plan.instance_id().as_str(), "fixture.instance.local");
    assert_eq!(plan.instance_revision().as_str(), "revision-1");
    assert_eq!(
        plan.instance_target_ref().as_host_value(),
        "fixture-host-target"
    );
    assert_eq!(
        plan.model_route_id().map(|route| route.as_str()),
        Some("fixture.route.model")
    );
    assert_eq!(
        plan.model_id().map(|model| model.as_str()),
        Some("fixture-model")
    );
    assert_eq!(
        plan.access_profile_id().as_str(),
        "fixture.access.subscription"
    );
    assert_eq!(plan.execution_host_id().as_str(), "fixture.host.local");
    assert_eq!(fixture.provider_side_effect_count(), 0);
    plan.validate_current(&fixture.context())
        .unwrap_or_else(|error| panic!("{PLAN_RULE}: unchanged plan was stale: {error}"));
}

pub fn assert_changed_revision_invalidates_plan() {
    let original = RuntimePreflightFixture::canonical();
    let plan = original
        .preflight()
        .unwrap_or_else(|error| panic!("{PLAN_RULE}: canonical preflight failed: {error}"));
    let changed = RuntimePreflightFixture::canonical().with_instance_revision("revision-2");
    let failure = plan
        .validate_current(&changed.context())
        .expect_err("Contract 008 immutable preflight binding: changed revision was accepted");

    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.preflight_plan_stale"
    );
    assert_eq!(original.provider_side_effect_count(), 0);
    assert_eq!(changed.provider_side_effect_count(), 0);
}
