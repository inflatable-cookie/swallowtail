use crate::OllamaContextWindow;
use swallowtail_core::{Capability, CapabilityConstraint, PreflightPlan};
use swallowtail_runtime::RuntimeFailure;

pub(crate) fn context_window_capability_constraints(
    context_window: Option<OllamaContextWindow>,
) -> Vec<CapabilityConstraint> {
    context_window
        .map(|value| CapabilityConstraint::ContextLimit(u64::from(value.as_u32())))
        .into_iter()
        .collect()
}

pub(crate) fn plan_bound_context_window(
    plan: &PreflightPlan,
    capability: Capability,
) -> Option<u32> {
    plan.requirements()
        .capabilities()
        .find(|requirement| requirement.capability() == capability)
        .and_then(|requirement| {
            requirement.constraints().find_map(|constraint| {
                if let CapabilityConstraint::ContextLimit(value) = constraint {
                    u32::try_from(*value).ok()
                } else {
                    None
                }
            })
        })
}

pub(crate) fn validate_context_window_plan_binding(
    driver: Option<OllamaContextWindow>,
    plan: &PreflightPlan,
    capability: Capability,
) -> Result<(), RuntimeFailure> {
    let planned = plan_bound_context_window(plan, capability);
    match (planned, driver.map(OllamaContextWindow::as_u32)) {
        (None, None) => Ok(()),
        (Some(expected), Some(actual)) if expected == actual => Ok(()),
        _ => Err(crate::failure::failure(
            "swallowtail.ollama.context_window_binding_mismatch",
            "Ollama context window binding did not match preflight",
        )),
    }
}
