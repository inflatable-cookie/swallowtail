use swallowtail_core::PreflightDimension;
use swallowtail_testkit::{
    PreflightFixtureCase, assert_changed_revision_invalidates_plan,
    assert_preflight_rejection_without_side_effects, assert_successful_preflight_binding,
};

#[test]
fn every_dimensional_rejection_precedes_provider_work() {
    let cases = [
        (PreflightFixtureCase::MissingRole, PreflightDimension::Role),
        (
            PreflightFixtureCase::MissingHostService,
            PreflightDimension::HostService,
        ),
        (
            PreflightFixtureCase::MissingCapability,
            PreflightDimension::Capability,
        ),
        (
            PreflightFixtureCase::MissingConstraint,
            PreflightDimension::Constraint,
        ),
        (
            PreflightFixtureCase::MissingReasoningMode,
            PreflightDimension::Constraint,
        ),
        (
            PreflightFixtureCase::MissingExternalSearch,
            PreflightDimension::Capability,
        ),
        (
            PreflightFixtureCase::MissingSchemaService,
            PreflightDimension::HostService,
        ),
        (
            PreflightFixtureCase::MissingModelRoute,
            PreflightDimension::ModelRoute,
        ),
        (
            PreflightFixtureCase::RejectedAccess,
            PreflightDimension::Access,
        ),
        (
            PreflightFixtureCase::RejectedSupportAuthority,
            PreflightDimension::SupportAuthority,
        ),
        (
            PreflightFixtureCase::RejectedOwnership,
            PreflightDimension::Ownership,
        ),
        (
            PreflightFixtureCase::WrongExecutionHost,
            PreflightDimension::Topology,
        ),
        (
            PreflightFixtureCase::MissingExtension,
            PreflightDimension::Extension,
        ),
    ];

    for (case, dimension) in cases {
        assert_preflight_rejection_without_side_effects(case, dimension);
    }
}

#[test]
fn successful_plan_preserves_the_selected_dimensions() {
    assert_successful_preflight_binding();
}

#[test]
fn material_revision_change_makes_the_plan_stale() {
    assert_changed_revision_invalidates_plan();
}
