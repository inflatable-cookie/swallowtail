use swallowtail_core::ModelId;
use swallowtail_runtime::{PreparationFailure, PreparationStage, RuntimeFailure};

const QUALIFIED_MODEL_ID: &str = "claude-opus-4-7";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Exact adapter-local Anthropic Messages thinking-mode selection.
///
/// This selection describes local dispatch only. It does not claim provider
/// acceptance, effective thinking depth, or readable thought content.
pub struct AnthropicThinkingMode {
    _private: (),
}

impl AnthropicThinkingMode {
    #[must_use]
    /// Returns the admitted adaptive omitted-display selection.
    pub const fn adaptive() -> Self {
        Self { _private: () }
    }
}

pub(crate) fn validate_preparation(
    model: &ModelId,
    _mode: AnthropicThinkingMode,
) -> Result<(), PreparationFailure> {
    if supports(model) {
        Ok(())
    } else {
        Err(crate::prepared::failure(
            PreparationStage::Preflight,
            "swallowtail.anthropic.preparation.thinking_unsupported",
            "Anthropic adaptive thinking requires the exact qualified model",
        ))
    }
}

pub(crate) fn validate_runtime_binding(
    model: Option<&ModelId>,
    thinking: Option<AnthropicThinkingMode>,
) -> Result<(), RuntimeFailure> {
    match thinking {
        None => Ok(()),
        Some(_) if model.is_some_and(supports) => Ok(()),
        Some(_) => Err(crate::failure::failure(
            "swallowtail.anthropic.thinking_binding_rejected",
            "Anthropic adaptive thinking did not match the exact qualified model",
        )),
    }
}

pub(crate) fn supports(model: &ModelId) -> bool {
    model.as_str() == QUALIFIED_MODEL_ID
}
