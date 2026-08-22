use crate::failure::failure;
use swallowtail_core::{Capability, CapabilityConstraint, ModelId, PreflightPlan, ReasoningMode};
use swallowtail_runtime::{PreparationFailure, PreparationStage, RuntimeFailure};

const QUALIFIED_MODEL_ID: &str = "claude-opus-4-7";

pub(crate) fn validate_preparation(
    model: &ModelId,
    reasoning: &ReasoningMode,
) -> Result<(), PreparationFailure> {
    if supports(model, reasoning) {
        Ok(())
    } else {
        Err(crate::prepared::failure(
            PreparationStage::Preflight,
            "swallowtail.anthropic.preparation.reasoning_unsupported",
            "Anthropic effort requires the exact qualified model and effort value",
        ))
    }
}

pub(crate) fn validate_runtime_binding(
    plan: &PreflightPlan,
    reasoning: Option<&ReasoningMode>,
) -> Result<(), RuntimeFailure> {
    if binding_matches(plan, reasoning) {
        Ok(())
    } else {
        Err(failure(
            "swallowtail.anthropic.generation_control_mismatch",
            "Anthropic effort did not match the exact preflight selection",
        ))
    }
}

pub(crate) fn supports(model: &ModelId, reasoning: &ReasoningMode) -> bool {
    model.as_str() == QUALIFIED_MODEL_ID
        && matches!(
            reasoning.as_str(),
            "low" | "medium" | "high" | "xhigh" | "max"
        )
}

fn binding_matches(plan: &PreflightPlan, reasoning: Option<&ReasoningMode>) -> bool {
    let requirements = plan
        .requirements()
        .capabilities()
        .filter(|requirement| requirement.capability() == Capability::ReasoningSelection)
        .collect::<Vec<_>>();
    match (requirements.as_slice(), reasoning) {
        ([], None) => true,
        ([requirement], Some(reasoning)) => {
            let constraints = requirement.constraints().cloned().collect::<Vec<_>>();
            constraints == [CapabilityConstraint::ReasoningMode(reasoning.clone())]
                && plan
                    .model_id()
                    .is_some_and(|model| supports(model, reasoning))
        }
        _ => false,
    }
}
