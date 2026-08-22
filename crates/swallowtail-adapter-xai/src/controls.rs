use crate::failure::failure;
use std::num::NonZeroU64;
use swallowtail_core::{Capability, CapabilityConstraint, ModelId, PreflightPlan, ReasoningMode};
use swallowtail_runtime::{PreparationFailure, PreparationStage, RuntimeFailure};

pub(crate) const MAX_OUTPUT_TOKENS: u64 = i32::MAX as u64;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct GenerationControls {
    pub(crate) reasoning: Option<ReasoningMode>,
    pub(crate) maximum_output_tokens: Option<NonZeroU64>,
}

pub(crate) fn validate_preparation(
    model: &ModelId,
    reasoning: Option<&ReasoningMode>,
    maximum_output_tokens: Option<NonZeroU64>,
) -> Result<(), PreparationFailure> {
    if reasoning.is_none() && maximum_output_tokens.is_none() {
        return Ok(());
    }
    if !is_qualified_model(model) {
        return Err(preparation_failure(
            "swallowtail.xai.preparation.model_controls_unsupported",
            "xAI generation controls require the exact grok-4.5 or grok-4.6 model id",
        ));
    }
    if maximum_output_tokens.is_some_and(|value| value.get() > MAX_OUTPUT_TOKENS) {
        return Err(preparation_failure(
            "swallowtail.xai.preparation.output_limit_invalid",
            "xAI maximum output tokens must fit the positive Responses int32 range",
        ));
    }
    if let Some(reasoning) = reasoning
        && !supports_reasoning(model, reasoning)
    {
        return Err(preparation_failure(
            "swallowtail.xai.preparation.reasoning_unsupported",
            "xAI reasoning selection is not qualified for the exact model id",
        ));
    }
    Ok(())
}

pub(crate) fn validate_request(
    plan: &PreflightPlan,
    reasoning: Option<&ReasoningMode>,
    maximum_output_tokens: Option<NonZeroU64>,
) -> Result<GenerationControls, RuntimeFailure> {
    let planned = from_plan(plan)?;
    if planned.reasoning.as_ref() != reasoning
        || planned.maximum_output_tokens != maximum_output_tokens
    {
        return Err(failure(
            "swallowtail.xai.generation_control_mismatch",
            "xAI generation controls differed between the request and preflight plan",
        ));
    }
    Ok(planned)
}

pub(crate) fn from_plan(plan: &PreflightPlan) -> Result<GenerationControls, RuntimeFailure> {
    let reasoning = match one_constraint(plan, Capability::ReasoningSelection)? {
        None => None,
        Some(CapabilityConstraint::ReasoningMode(mode)) => Some(mode),
        Some(_) => return Err(plan_controls_failure()),
    };
    let maximum_output_tokens = match one_constraint(plan, Capability::OutputTokenLimit)? {
        None => None,
        Some(CapabilityConstraint::OutputTokenMaximum(value)) => {
            NonZeroU64::new(value).filter(|value| value.get() <= MAX_OUTPUT_TOKENS)
        }
        Some(_) => return Err(plan_controls_failure()),
    };
    let model = plan.model_id().ok_or_else(plan_controls_failure)?;
    if (reasoning.is_some() || maximum_output_tokens.is_some())
        && (!is_qualified_model(model)
            || reasoning
                .as_ref()
                .is_some_and(|mode| !supports_reasoning(model, mode)))
    {
        return Err(plan_controls_failure());
    }
    if (reasoning.is_some() || maximum_output_tokens.is_some())
        && plan
            .requirements()
            .capabilities()
            .any(|requirement| requirement.capability() == Capability::OutputTokenLimit)
        && maximum_output_tokens.is_none()
    {
        return Err(plan_controls_failure());
    }
    Ok(GenerationControls {
        reasoning,
        maximum_output_tokens,
    })
}

pub(crate) fn with_capabilities(
    mut capabilities: swallowtail_core::CapabilityProfile,
    reasoning: Option<&ReasoningMode>,
    maximum_output_tokens: Option<NonZeroU64>,
) -> swallowtail_core::CapabilityProfile {
    let mut requirements = capabilities
        .iter()
        .map(|(capability, constraints)| {
            swallowtail_core::CapabilityRequirement::new(capability, constraints.iter().cloned())
        })
        .collect::<Vec<_>>();
    if let Some(reasoning) = reasoning {
        requirements.push(swallowtail_core::CapabilityRequirement::new(
            Capability::ReasoningSelection,
            [CapabilityConstraint::ReasoningMode(reasoning.clone())],
        ));
    }
    if let Some(maximum_output_tokens) = maximum_output_tokens {
        requirements.push(swallowtail_core::CapabilityRequirement::new(
            Capability::OutputTokenLimit,
            [CapabilityConstraint::OutputTokenMaximum(
                maximum_output_tokens.get(),
            )],
        ));
    }
    capabilities = swallowtail_core::CapabilityProfile::new(requirements);
    capabilities
}

pub(crate) fn is_qualified_model(model: &ModelId) -> bool {
    matches!(model.as_str(), "grok-4.5" | "grok-4.6")
}

fn supports_reasoning(model: &ModelId, reasoning: &ReasoningMode) -> bool {
    match model.as_str() {
        "grok-4.5" => matches!(reasoning.as_str(), "low" | "medium" | "high"),
        "grok-4.6" => matches!(reasoning.as_str(), "low" | "medium" | "high" | "xhigh"),
        _ => false,
    }
}

fn one_constraint(
    plan: &PreflightPlan,
    capability: Capability,
) -> Result<Option<CapabilityConstraint>, RuntimeFailure> {
    let mut requirements = plan
        .requirements()
        .capabilities()
        .filter(|requirement| requirement.capability() == capability);
    let Some(requirement) = requirements.next() else {
        return Ok(None);
    };
    if requirements.next().is_some() {
        return Err(plan_controls_failure());
    }
    let mut constraints = requirement.constraints();
    let Some(constraint) = constraints.next() else {
        return Err(plan_controls_failure());
    };
    if constraints.next().is_some() {
        return Err(plan_controls_failure());
    }
    Ok(Some(constraint.clone()))
}

fn preparation_failure(code: &'static str, message: &'static str) -> PreparationFailure {
    PreparationFailure::new(
        PreparationStage::Preflight,
        swallowtail_core::Diagnostic::new(swallowtail_core::SafeDiagnostic::new(code, message)),
    )
}

fn plan_controls_failure() -> RuntimeFailure {
    failure(
        "swallowtail.xai.generation_control_mismatch",
        "xAI generation-control constraints were malformed or unsupported",
    )
}
