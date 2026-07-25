use super::{SessionLifecycleOperation, prepare_from_requirements};
use crate::SessionOptions;
use swallowtail_core::{
    AccessProfileId, AccessRequirement, Capability, CapabilityConstraint, CapabilityRequirement,
    DriverRole, ExecutionHostId, ExecutionLayer, OperationRequirements, OperationShape,
    ReasoningMode,
};

#[test]
fn exact_open_selection_requires_matching_request_plan_and_confirmation() {
    let mode = reasoning("high");
    let options = SessionOptions::default().with_reasoning_mode(mode.clone());
    let setup = prepare_from_requirements(
        &requirements([reasoning_requirement("high")]),
        SessionLifecycleOperation::Open,
        &options,
    )
    .expect("matching setup is valid")
    .expect("reasoning setup is present");

    assert_eq!(setup.operation(), SessionLifecycleOperation::Open);
    assert_eq!(setup.requested(), &mode);
    let effective = setup
        .confirm(mode.clone())
        .expect("exact effective mode confirms");
    assert_eq!(effective.requested(), &mode);
    assert_eq!(effective.effective(), &mode);
}

#[test]
fn empty_selection_preserves_an_unbound_session() {
    let setup = prepare_from_requirements(
        &requirements([]),
        SessionLifecycleOperation::Open,
        &SessionOptions::default(),
    )
    .expect("empty setup is valid");

    assert!(setup.is_none());
}

#[test]
fn missing_ambiguous_and_changed_plan_modes_reject() {
    let options = SessionOptions::default().with_reasoning_mode(reasoning("high"));
    for requirements in [
        requirements([]),
        requirements([reasoning_requirement("low")]),
        requirements([reasoning_requirement("high"), reasoning_requirement("high")]),
        requirements([CapabilityRequirement::new(
            Capability::ReasoningSelection,
            [
                CapabilityConstraint::reasoning_mode(reasoning("low")),
                CapabilityConstraint::reasoning_mode(reasoning("high")),
            ],
        )]),
    ] {
        let error =
            prepare_from_requirements(&requirements, SessionLifecycleOperation::Open, &options)
                .expect_err("non-exact plan agreement rejects");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.negotiated_reasoning.plan_mismatch"
        );
    }

    let error = prepare_from_requirements(
        &requirements([reasoning_requirement("high")]),
        SessionLifecycleOperation::Open,
        &SessionOptions::default(),
    )
    .expect_err("missing request selection rejects");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.negotiated_reasoning.plan_mismatch"
    );
}

#[test]
fn load_resume_wrong_shape_and_effective_drift_reject() {
    let options = SessionOptions::default().with_reasoning_mode(reasoning("high"));
    for operation in [
        SessionLifecycleOperation::Load,
        SessionLifecycleOperation::Resume,
    ] {
        let error = prepare_from_requirements(
            &requirements([reasoning_requirement("high")]),
            operation,
            &options,
        )
        .expect_err("persistent lifecycle mutation rejects");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.negotiated_reasoning.lifecycle_rejected"
        );
    }

    let wrong_shape = OperationRequirements::new(
        ExecutionLayer::DirectModelInference,
        OperationShape::StructuredRun,
        DriverRole::StructuredRun,
        ExecutionHostId::new("fixture.host").expect("valid host"),
        AccessRequirement::new(AccessProfileId::new("fixture.access").expect("valid access")),
    )
    .with_capabilities([reasoning_requirement("high")]);
    let error = prepare_from_requirements(&wrong_shape, SessionLifecycleOperation::Open, &options)
        .expect_err("wrong operation shape rejects");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.negotiated_reasoning.operation_rejected"
    );

    let setup = prepare_from_requirements(
        &requirements([reasoning_requirement("high")]),
        SessionLifecycleOperation::Open,
        &options,
    )
    .expect("matching setup is valid")
    .expect("setup exists");
    let error = setup
        .confirm(reasoning("medium"))
        .expect_err("effective drift rejects");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.negotiated_reasoning.effective_mismatch"
    );
}

fn requirements(
    capabilities: impl IntoIterator<Item = CapabilityRequirement>,
) -> OperationRequirements {
    OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        OperationShape::InteractiveSession,
        DriverRole::InteractiveSession,
        ExecutionHostId::new("fixture.host").expect("valid host"),
        AccessRequirement::new(AccessProfileId::new("fixture.access").expect("valid access")),
    )
    .with_capabilities(capabilities)
}

fn reasoning_requirement(mode: &str) -> CapabilityRequirement {
    CapabilityRequirement::new(
        Capability::ReasoningSelection,
        [CapabilityConstraint::reasoning_mode(reasoning(mode))],
    )
}

fn reasoning(mode: &str) -> ReasoningMode {
    ReasoningMode::new(mode).expect("valid reasoning mode")
}
