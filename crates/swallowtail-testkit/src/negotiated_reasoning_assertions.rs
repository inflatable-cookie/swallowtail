use swallowtail_core::{PreflightPlan, ReasoningMode};
use swallowtail_runtime::{
    SessionLifecycleOperation, SessionOptions, prepare_negotiated_reasoning_setup,
};

pub fn assert_negotiated_reasoning_setup_contract(
    plan: &PreflightPlan,
    requested: ReasoningMode,
    different_effective: ReasoningMode,
) {
    let options = SessionOptions::default().with_reasoning_mode(requested.clone());
    let setup = prepare_negotiated_reasoning_setup(plan, SessionLifecycleOperation::Open, &options)
        .expect("exact request and plan reasoning agree")
        .expect("reasoning setup is present");
    assert_eq!(setup.requested(), &requested);
    assert_eq!(
        setup
            .clone()
            .confirm(requested.clone())
            .expect("exact effective value confirms")
            .effective(),
        &requested
    );
    assert_eq!(
        setup
            .confirm(different_effective)
            .expect_err("effective drift rejects")
            .diagnostic()
            .code(),
        "swallowtail.negotiated_reasoning.effective_mismatch"
    );

    for operation in [
        SessionLifecycleOperation::Load,
        SessionLifecycleOperation::Resume,
    ] {
        assert_eq!(
            prepare_negotiated_reasoning_setup(plan, operation, &options)
                .expect_err("persistent-session mutation rejects")
                .diagnostic()
                .code(),
            "swallowtail.negotiated_reasoning.lifecycle_rejected"
        );
    }
}
