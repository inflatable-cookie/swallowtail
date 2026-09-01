use swallowtail_runtime::{NegotiatedSessionModelOption, NegotiatedSessionModelOptions};

const MODEL_OPTION_ID: &str = "model";

fn observe_model_options(root: &Value) -> ClineModelObservation {
    match parse_model_options(root) {
        Ok(Some(options)) => ClineModelObservation::Exact(options),
        Ok(None) => ClineModelObservation::Absent,
        Err(error) => ClineModelObservation::Invalid(error),
    }
}

fn parse_model_options(
    root: &Value,
) -> Result<Option<NegotiatedSessionModelOptions>, RuntimeFailure> {
    let Some(config_options) = root.get("configOptions") else {
        return Ok(None);
    };
    let options = config_options.as_array().ok_or_else(model_options_invalid)?;
    let matches = options
        .iter()
        .filter(|option| option.get("id").and_then(Value::as_str) == Some(MODEL_OPTION_ID))
        .collect::<Vec<_>>();
    let [option] = matches.as_slice() else {
        return if matches.is_empty() {
            Ok(None)
        } else {
            Err(model_options_invalid())
        };
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
                Some(value) => Some(
                    value
                        .as_str()
                        .ok_or_else(model_options_invalid)?
                        .to_owned(),
                ),
                None => None,
            };
            NegotiatedSessionModelOption::new(value, display_name)
                .map_err(|_| model_options_invalid())
        })
        .collect::<Result<Vec<_>, _>>()?;
    NegotiatedSessionModelOptions::new(current, parsed)
        .map(Some)
        .map_err(|_| model_options_invalid())
}

fn model_options_invalid() -> RuntimeFailure {
    failure(
        "swallowtail.negotiated_model_options.invalid",
        "Harness returned invalid bounded negotiated model options",
    )
}
