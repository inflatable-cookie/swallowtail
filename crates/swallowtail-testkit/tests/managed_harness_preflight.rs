use swallowtail_core::PreflightDimension;
use swallowtail_testkit::{ManagedHarnessPreflightCase, ManagedHarnessPreflightFixture};

#[test]
fn exact_managed_harness_policy_passes_without_provider_effects() {
    let fixture = ManagedHarnessPreflightFixture::for_case(ManagedHarnessPreflightCase::Canonical);
    let plan = fixture.preflight().expect("canonical preflight passes");

    assert_eq!(
        plan.model_id().map(|model| model.as_str()),
        Some("fixture-managed-model")
    );
    assert_eq!(fixture.provider_side_effect_count(), 0);
}

#[test]
fn missing_retention_recovery_or_owned_resource_rejects_before_effects() {
    let cases = [
        (
            ManagedHarnessPreflightCase::MissingDurableRetention,
            PreflightDimension::Capability,
        ),
        (
            ManagedHarnessPreflightCase::MissingManagedRecovery,
            PreflightDimension::Capability,
        ),
        (
            ManagedHarnessPreflightCase::MissingOwnedEnvironment,
            PreflightDimension::Constraint,
        ),
        (
            ManagedHarnessPreflightCase::MissingOwnedSession,
            PreflightDimension::Constraint,
        ),
    ];

    for (case, expected) in cases {
        let fixture = ManagedHarnessPreflightFixture::for_case(case);
        let failure = fixture.preflight().expect_err("weaker policy must fail");
        assert_eq!(failure.dimension(), expected);
        assert_eq!(fixture.provider_side_effect_count(), 0);
    }
}
