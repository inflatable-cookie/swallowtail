use crate::selection::QwenPlanSelection;
use crate::validation::failure;
use swallowtail_core::{
    Capability, CapabilityConstraint, Diagnostic, HarnessMode, InterfaceVersion, PreflightPlan,
    SafeDiagnostic,
};
use swallowtail_runtime::{PreparationFailure, PreparationStage, RuntimeFailure};

pub(crate) const QUALIFIED_VERSIONS: [&str; 5] =
    ["0.21.15", "0.22.0", "0.22.1", "0.22.2", "0.22.3"];

pub(crate) fn supports(version: &InterfaceVersion) -> bool {
    QUALIFIED_VERSIONS.contains(&version.as_str())
}

pub(crate) fn validate_preparation(
    version: &InterfaceVersion,
    harness_mode: HarnessMode,
) -> Result<(), PreparationFailure> {
    if harness_mode == HarnessMode::Plan && supports(version) {
        Ok(())
    } else {
        Err(PreparationFailure::new(
            PreparationStage::Preflight,
            Diagnostic::new(SafeDiagnostic::new(
                "swallowtail.qwen.preparation.harness_mode_unsupported",
                "Qwen Plan requires exact maintained 0.21.15, 0.22.0, 0.22.1, 0.22.2, or 0.22.3",
            )),
        ))
    }
}

pub(crate) fn validate_runtime_binding(
    selection: &QwenPlanSelection,
    plan: &PreflightPlan,
    harness_mode: Option<HarnessMode>,
) -> Result<(), RuntimeFailure> {
    if binding_matches(selection, plan, harness_mode) {
        Ok(())
    } else {
        Err(failure(
            "swallowtail.qwen.headless.harness_mode_mismatch",
            "Qwen headless harness mode does not match its preflight-bound plan",
        ))
    }
}

pub(crate) fn approval_arg(harness_mode: Option<HarnessMode>) -> &'static str {
    match harness_mode {
        Some(HarnessMode::Plan) => "plan",
        None => "default",
        Some(_) => "default",
    }
}

fn binding_matches(
    selection: &QwenPlanSelection,
    plan: &PreflightPlan,
    harness_mode: Option<HarnessMode>,
) -> bool {
    let requirements = plan
        .requirements()
        .capabilities()
        .filter(|requirement| requirement.capability() == Capability::HarnessModeSelection)
        .collect::<Vec<_>>();
    match (requirements.as_slice(), harness_mode) {
        ([], None) => true,
        ([requirement], Some(HarnessMode::Plan)) => {
            let constraints = requirement.constraints().cloned().collect::<Vec<_>>();
            constraints == [CapabilityConstraint::HarnessMode(HarnessMode::Plan)]
                && supports(selection.version())
        }
        _ => false,
    }
}
