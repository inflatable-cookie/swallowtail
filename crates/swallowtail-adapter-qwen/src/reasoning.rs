use crate::validation::failure;
use swallowtail_core::{
    Capability, CapabilityConstraint, Diagnostic, InterfaceVersion, ModelId, PreflightPlan,
    ProviderId, ReasoningMode, SafeDiagnostic,
};
use swallowtail_runtime::{PreparationFailure, PreparationStage, RuntimeFailure};

pub(crate) const QUALIFIED_PROVIDER_ID: &str = "alibaba-modelstudio";
pub(crate) const QUALIFIED_VERSION: &str = "0.21.15";
pub(crate) const QUALIFIED_MODELS: [&str; 2] = ["qwen3.8-max", "qwen3.8-max-preview"];

pub(crate) fn validate_preparation(
    version: &InterfaceVersion,
    provider: &ProviderId,
    model: &ModelId,
    reasoning: &ReasoningMode,
) -> Result<(), PreparationFailure> {
    if supports(version, provider, model, reasoning) {
        Ok(())
    } else {
        Err(PreparationFailure::new(
            PreparationStage::Preflight,
            Diagnostic::new(SafeDiagnostic::new(
                "swallowtail.qwen.preparation.reasoning_unsupported",
                "Qwen reasoning requires the exact qualified package, provider, model, and mode",
            )),
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
            "swallowtail.qwen.headless.reasoning_plan_mismatch",
            "Qwen reasoning did not match the exact preflight selection",
        ))
    }
}

pub(crate) fn supports(
    version: &InterfaceVersion,
    provider: &ProviderId,
    model: &ModelId,
    reasoning: &ReasoningMode,
) -> bool {
    version.as_str() == QUALIFIED_VERSION
        && provider.as_str() == QUALIFIED_PROVIDER_ID
        && QUALIFIED_MODELS.contains(&model.as_str())
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
                    .interface_versions()
                    .any(|binding| binding.version().as_str() == QUALIFIED_VERSION)
                && plan
                    .provider_id()
                    .is_some_and(|provider| provider.as_str() == QUALIFIED_PROVIDER_ID)
                && plan
                    .model_id()
                    .is_some_and(|model| QUALIFIED_MODELS.contains(&model.as_str()))
                && plan
                    .model_id()
                    .zip(plan.provider_id())
                    .is_some_and(|(model, provider)| {
                        supports(
                            &InterfaceVersion::new(QUALIFIED_VERSION)
                                .expect("static Qwen version is valid"),
                            provider,
                            model,
                            reasoning,
                        )
                    })
        }
        _ => false,
    }
}
