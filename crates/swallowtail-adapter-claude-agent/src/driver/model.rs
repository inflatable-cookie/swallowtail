use crate::failure::failure;
use serde_json::Value;
use swallowtail_runtime::{
    NegotiatedSessionModelOption, NegotiatedSessionModelOptions, RuntimeFailure,
};

const MODEL_OPTION_ID: &str = "model";

pub(crate) enum ClaudeAgentModelObservation {
    Exact(NegotiatedSessionModelOptions),
    Invalid(RuntimeFailure),
}

impl ClaudeAgentModelObservation {
    pub(crate) const fn exact(&self) -> Option<&NegotiatedSessionModelOptions> {
        match self {
            Self::Exact(options) => Some(options),
            Self::Invalid(_) => None,
        }
    }
}

pub(crate) fn observe_model_options(root: &Value) -> ClaudeAgentModelObservation {
    match parse_model_options(root) {
        Ok(options) => ClaudeAgentModelObservation::Exact(options),
        Err(error) => ClaudeAgentModelObservation::Invalid(error),
    }
}

fn parse_model_options(root: &Value) -> Result<NegotiatedSessionModelOptions, RuntimeFailure> {
    let options = root
        .get("configOptions")
        .and_then(Value::as_array)
        .ok_or_else(model_options_invalid)?;
    let matches = options
        .iter()
        .filter(|option| option.get("id").and_then(Value::as_str) == Some(MODEL_OPTION_ID))
        .collect::<Vec<_>>();
    let [option] = matches.as_slice() else {
        return Err(model_options_invalid());
    };
    if option.get("type").and_then(Value::as_str) != Some("select")
        || option.get("category").and_then(Value::as_str) != Some("model")
    {
        return Err(model_options_invalid());
    }
    let current = option
        .get("currentValue")
        .and_then(Value::as_str)
        .ok_or_else(model_options_invalid)?;
    let rows = option
        .get("options")
        .and_then(Value::as_array)
        .ok_or_else(model_options_invalid)?;
    let parsed = rows
        .iter()
        .map(|row| {
            let value = row
                .get("value")
                .and_then(Value::as_str)
                .ok_or_else(model_options_invalid)?;
            let display_name = match row.get("name") {
                Some(value) => Some(value.as_str().ok_or_else(model_options_invalid)?.to_owned()),
                None => None,
            };
            NegotiatedSessionModelOption::new(value, display_name)
                .map_err(|_| model_options_invalid())
        })
        .collect::<Result<Vec<_>, _>>()?;
    NegotiatedSessionModelOptions::new(current, parsed).map_err(|_| model_options_invalid())
}

fn model_options_invalid() -> RuntimeFailure {
    failure(
        "swallowtail.negotiated_model_options.invalid",
        "Harness returned invalid bounded negotiated model options",
    )
}
