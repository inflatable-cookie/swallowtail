use swallowtail_core::{ModelId, ReasoningMode};
use swallowtail_runtime::{PreparationFailure, PreparationStage, RuntimeFailure};

/// Cursor-local Fast parameter for headless `--model` bracket syntax.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CursorHeadlessFast {
    /// Selects the standard (non-fast) variant (`fast=false`).
    Standard,
}

/// Cursor-local context-window parameter for headless `--model` bracket syntax.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CursorHeadlessContext {
    /// `context=300k` for qualified `claude-opus-5`.
    ThreeHundredK,
    /// `context=1m` for qualified `claude-opus-4-8`.
    OneMillion,
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub(crate) struct CursorHeadlessModelParameters {
    fast: Option<CursorHeadlessFast>,
    context: Option<CursorHeadlessContext>,
    effort: Option<ReasoningMode>,
}

/// Parsed base model and parameters from a plan-bound model id.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ParsedHeadlessModelId {
    pub(crate) base_model: String,
    pub(crate) parameters: CursorHeadlessModelParameters,
}

impl CursorHeadlessModelParameters {
    pub(crate) const fn empty() -> Self {
        Self {
            fast: None,
            context: None,
            effort: None,
        }
    }

    pub(crate) const fn is_empty(&self) -> bool {
        self.fast.is_none() && self.context.is_none() && self.effort.is_none()
    }

    pub(crate) fn with_fast(
        mut self,
        base_model: &str,
        fast: CursorHeadlessFast,
    ) -> Result<Self, PreparationFailure> {
        if !allows_fast(base_model, fast) {
            return Err(tuple_rejected("fast"));
        }
        self.fast = Some(fast);
        Ok(self)
    }

    pub(crate) fn with_context(
        mut self,
        base_model: &str,
        context: CursorHeadlessContext,
    ) -> Result<Self, PreparationFailure> {
        if !allows_context(base_model, context) {
            return Err(tuple_rejected("context"));
        }
        self.context = Some(context);
        Ok(self)
    }

    pub(crate) fn with_effort(
        mut self,
        base_model: &str,
        effort: ReasoningMode,
    ) -> Result<Self, PreparationFailure> {
        if !allows_effort(base_model, &effort) {
            return Err(tuple_rejected("effort"));
        }
        self.effort = Some(effort);
        Ok(self)
    }

    pub(crate) const fn effort(&self) -> Option<&ReasoningMode> {
        self.effort.as_ref()
    }
}

/// Returns whether the model id contains caller-assembled bracket parameter grammar.
#[must_use]
pub(crate) fn contains_parameter_grammar(model_id: &str) -> bool {
    model_id.contains('[')
        || model_id.contains(']')
        || model_id.contains('=')
        || model_id.contains(',')
}

/// Renders the exact `--model` value for the qualified base model and parameters.
pub(crate) fn render_model_id(
    base_model: &str,
    parameters: &CursorHeadlessModelParameters,
) -> Result<ModelId, PreparationFailure> {
    if parameters.is_empty() {
        return ModelId::new(base_model).map_err(|_| invalid_model_id());
    }
    validate_combination(base_model, parameters)?;
    let mut parts = Vec::new();
    if let Some(context) = parameters.context {
        parts.push(match context {
            CursorHeadlessContext::ThreeHundredK => "context=300k",
            CursorHeadlessContext::OneMillion => "context=1m",
        });
    }
    if let Some(effort) = parameters.effort.as_ref() {
        parts.push(match effort.as_str() {
            "high" => "effort=high",
            _ => return Err(tuple_rejected("effort")),
        });
    }
    if let Some(fast) = parameters.fast {
        parts.push(match fast {
            CursorHeadlessFast::Standard => "fast=false",
        });
    }
    let rendered = format!("{base_model}[{}]", parts.join(","));
    ModelId::new(&rendered).map_err(|_| invalid_model_id())
}

pub(crate) fn validate_combination(
    base_model: &str,
    parameters: &CursorHeadlessModelParameters,
) -> Result<(), PreparationFailure> {
    if let Some(fast) = parameters.fast
        && !allows_fast(base_model, fast)
    {
        return Err(tuple_rejected("fast"));
    }
    if let Some(context) = parameters.context
        && !allows_context(base_model, context)
    {
        return Err(tuple_rejected("context"));
    }
    if let Some(effort) = parameters.effort.as_ref()
        && !allows_effort(base_model, effort)
    {
        return Err(tuple_rejected("effort"));
    }
    Ok(())
}

pub(crate) fn validate_plain_model_id(model_id: &str) -> Result<(), PreparationFailure> {
    if contains_parameter_grammar(model_id) {
        return Err(super::prepared::failure(
            PreparationStage::Preflight,
            "swallowtail.cursor.headless.model_parameter_grammar_rejected",
            "Cursor headless rejects caller-assembled model-parameter bracket grammar",
        ));
    }
    Ok(())
}

/// Parses and validates a plan-bound model id against the Research 183 allowlist.
pub(crate) fn parse_plan_model_id(model_id: &str) -> Result<ParsedHeadlessModelId, RuntimeFailure> {
    if let Some(open) = model_id.find('[') {
        if !model_id.ends_with(']') || model_id[open + 1..].find('[').is_some() {
            return Err(model_parameter_rejected());
        }
        let base_model = &model_id[..open];
        if base_model.is_empty() {
            return Err(model_parameter_rejected());
        }
        let inner = &model_id[open + 1..model_id.len() - 1];
        let parameters = parse_parameter_suffix(base_model, inner)?;
        let rendered = render_model_id(base_model, &parameters).map_err(|_| model_parameter_rejected())?;
        if rendered.as_str() != model_id {
            return Err(model_parameter_rejected());
        }
        Ok(ParsedHeadlessModelId {
            base_model: base_model.to_owned(),
            parameters,
        })
    } else if contains_parameter_grammar(model_id) {
        Err(model_parameter_rejected())
    } else {
        Ok(ParsedHeadlessModelId {
            base_model: model_id.to_owned(),
            parameters: CursorHeadlessModelParameters::empty(),
        })
    }
}

fn parse_parameter_suffix(
    base_model: &str,
    inner: &str,
) -> Result<CursorHeadlessModelParameters, RuntimeFailure> {
    if inner.is_empty() {
        return Err(model_parameter_rejected());
    }
    const ORDER: [&str; 3] = ["context", "effort", "fast"];
    let mut parameters = CursorHeadlessModelParameters::empty();
    let mut last_index = None;
    for part in inner.split(',') {
        let Some((key, value)) = part.split_once('=') else {
            return Err(model_parameter_rejected());
        };
        let index = ORDER
            .iter()
            .position(|candidate| *candidate == key)
            .ok_or_else(model_parameter_rejected)?;
        if last_index.is_some_and(|previous| index <= previous) {
            return Err(model_parameter_rejected());
        }
        last_index = Some(index);
        parameters = match key {
            "context" => parameters
                .with_context(
                    base_model,
                    match value {
                        "300k" => CursorHeadlessContext::ThreeHundredK,
                        "1m" => CursorHeadlessContext::OneMillion,
                        _ => return Err(model_parameter_rejected()),
                    },
                )
                .map_err(|_| model_parameter_rejected())?,
            "effort" => parameters
                .with_effort(
                    base_model,
                    ReasoningMode::new(value).map_err(|_| model_parameter_rejected())?,
                )
                .map_err(|_| model_parameter_rejected())?,
            "fast" => parameters
                .with_fast(
                    base_model,
                    match value {
                        "false" => CursorHeadlessFast::Standard,
                        _ => return Err(model_parameter_rejected()),
                    },
                )
                .map_err(|_| model_parameter_rejected())?,
            _ => return Err(model_parameter_rejected()),
        };
    }
    Ok(parameters)
}

fn model_parameter_rejected() -> RuntimeFailure {
    crate::failure::failure(
        "swallowtail.cursor.headless.model_parameter_rejected",
        "Cursor headless model-parameter tuple is not qualified",
    )
}

fn allows_fast(base_model: &str, fast: CursorHeadlessFast) -> bool {
    matches!(
        (base_model, fast),
        ("claude-opus-4-8", CursorHeadlessFast::Standard)
            | ("composer-2.5", CursorHeadlessFast::Standard)
    )
}

fn allows_context(base_model: &str, context: CursorHeadlessContext) -> bool {
    matches!(
        (base_model, context),
        ("claude-opus-4-8", CursorHeadlessContext::OneMillion)
            | ("claude-opus-5", CursorHeadlessContext::ThreeHundredK)
    )
}

fn allows_effort(base_model: &str, effort: &ReasoningMode) -> bool {
    matches!(
        (base_model, effort.as_str()),
        ("claude-opus-4-8", "high") | ("claude-opus-5", "high")
    )
}

fn tuple_rejected(_parameter: &str) -> PreparationFailure {
    super::prepared::failure(
        PreparationStage::Preflight,
        "swallowtail.cursor.headless.model_parameter_rejected",
        "Cursor headless model-parameter tuple is not qualified",
    )
}

fn invalid_model_id() -> PreparationFailure {
    super::prepared::failure(
        PreparationStage::Preflight,
        "swallowtail.cursor.headless.model_id_rejected",
        "Cursor headless rendered model id is invalid",
    )
}

#[cfg(test)]
mod tests {
    use super::{
        CursorHeadlessContext, CursorHeadlessFast, CursorHeadlessModelParameters,
        contains_parameter_grammar, parse_plan_model_id, render_model_id, validate_plain_model_id,
    };
    use swallowtail_core::ReasoningMode;

    #[test]
    fn rejects_raw_parameter_grammar_in_plain_model_ids() {
        for model in [
            "claude-opus-4-8[context=1m]",
            "model[fast=true]",
            "model,extra",
            "model=value",
        ] {
            assert!(contains_parameter_grammar(model));
            assert!(validate_plain_model_id(model).is_err(), "{model}");
        }
        assert!(!contains_parameter_grammar("fixture-model"));
        assert!(validate_plain_model_id("fixture-model").is_ok());
    }

    #[test]
    fn renders_canonical_parameter_order() {
        let parameters = CursorHeadlessModelParameters::default()
            .with_fast("claude-opus-4-8", CursorHeadlessFast::Standard)
            .expect("fast")
            .with_context("claude-opus-4-8", CursorHeadlessContext::OneMillion)
            .expect("context")
            .with_effort(
                "claude-opus-4-8",
                ReasoningMode::new("high").expect("effort"),
            )
            .expect("effort");
        let rendered = render_model_id("claude-opus-4-8", &parameters).expect("render");
        assert_eq!(
            rendered.as_str(),
            "claude-opus-4-8[context=1m,effort=high,fast=false]"
        );
    }

    #[test]
    fn parse_plan_model_id_rejects_unqualified_and_noncanonical_suffixes() {
        for model in [
            "composer-2.5[fast=true]",
            "claude-opus-5[effort=high,context=300k]",
            "claude-opus-5[effort=high,effort=high]",
            "claude-opus-5[effort=low]",
        ] {
            assert!(parse_plan_model_id(model).is_err(), "{model}");
        }
        let parsed =
            parse_plan_model_id("claude-opus-4-8[context=1m,effort=high,fast=false]").expect("valid");
        assert_eq!(parsed.base_model, "claude-opus-4-8");
    }
}
