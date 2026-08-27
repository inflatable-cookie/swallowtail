//! Exact Pi SDK-sidecar reasoning-selection membership for Research 228.
//!
//! Bootstrap resolves models through `ModelRuntime.getModel(provider, id)` against
//! the static bundled `@earendil-works/pi-ai@0.84.2` corpus. Auth filters runtime
//! availability; it does not change the frozen capability metadata used here.

use crate::sidecar::failure::failure;
use crate::sidecar::prepared::preparation_failure;
use swallowtail_core::{
    Capability, CapabilityConstraint, ModelId, PreflightPlan, ProviderId, ReasoningMode,
};
use swallowtail_runtime::{PreparationFailure, PreparationStage, RuntimeFailure, SessionOptions};

pub(crate) const QUALIFIED_PROVIDER: &str = "anthropic";
pub(crate) const QUALIFIED_MODEL: &str = "claude-opus-4-5";

const ADMITTED_MODES: &[&str] = &["off", "minimal", "low", "medium", "high"];

pub(crate) fn validate_preparation(
    provider: &ProviderId,
    model: &ModelId,
    reasoning: &ReasoningMode,
) -> Result<(), PreparationFailure> {
    if supports(provider, model, reasoning) {
        Ok(())
    } else {
        Err(preparation_failure(
            PreparationStage::Preflight,
            "swallowtail.pi.sdk-sidecar.preparation.reasoning_unsupported",
            "Pi SDK sidecar reasoning selection requires the exact qualified provider, model, and thinking level",
        ))
    }
}

pub(crate) fn validate_options(options: &SessionOptions) -> Result<(), PreparationFailure> {
    if has_unsupported_options(options) {
        return Err(preparation_failure(
            PreparationStage::Preflight,
            "swallowtail.pi.sdk-sidecar.preparation.session_options_unsupported",
            "Pi SDK sidecar prepared sessions support reasoning selection only",
        ));
    }
    Ok(())
}

pub(crate) fn has_unsupported_options(options: &SessionOptions) -> bool {
    options.developer_instructions().is_some()
        || options.harness_mode().is_some()
        || options.tools().len() > 0
        || options.idioms().is_some()
}

pub(crate) fn validate_open_options(
    plan: &PreflightPlan,
    options: &SessionOptions,
) -> Result<(), RuntimeFailure> {
    if binding_matches(plan, options.reasoning_mode()) {
        Ok(())
    } else {
        Err(failure(
            "swallowtail.pi.sdk-sidecar.request_plan_mismatch",
            "Pi SDK sidecar session options do not match the preflight-bound reasoning selection",
        ))
    }
}

pub(crate) fn supports(provider: &ProviderId, model: &ModelId, reasoning: &ReasoningMode) -> bool {
    provider.as_str() == QUALIFIED_PROVIDER
        && model.as_str() == QUALIFIED_MODEL
        && ADMITTED_MODES.contains(&reasoning.as_str())
}

pub(crate) fn binding_matches(plan: &PreflightPlan, reasoning: Option<&ReasoningMode>) -> bool {
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
                && plan.provider_id().is_some_and(|provider| {
                    plan.model_id()
                        .is_some_and(|model| supports(provider, model, reasoning))
                })
        }
        _ => false,
    }
}

pub(crate) fn thinking_level(reasoning: &ReasoningMode) -> Option<&str> {
    supports(
        &ProviderId::new(QUALIFIED_PROVIDER).expect("static provider id"),
        &ModelId::new(QUALIFIED_MODEL).expect("static model id"),
        reasoning,
    )
    .then_some(reasoning.as_str())
}

pub(crate) fn expected_thinking_level(
    plan: &PreflightPlan,
    options: &SessionOptions,
) -> Result<Option<String>, RuntimeFailure> {
    validate_open_options(plan, options)?;
    Ok(options
        .reasoning_mode()
        .and_then(|mode| thinking_level(mode).map(str::to_owned)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn qualified_ids() -> (ProviderId, ModelId) {
        (
            ProviderId::new(QUALIFIED_PROVIDER).expect("provider"),
            ModelId::new(QUALIFIED_MODEL).expect("model"),
        )
    }

    #[test]
    fn admitted_modes_match_research_228() {
        let (provider, model) = qualified_ids();
        for mode in ADMITTED_MODES {
            assert!(supports(
                &provider,
                &model,
                &ReasoningMode::new(*mode).expect("mode")
            ));
        }
        assert!(!supports(
            &provider,
            &model,
            &ReasoningMode::new("xhigh").expect("mode")
        ));
    }
}
