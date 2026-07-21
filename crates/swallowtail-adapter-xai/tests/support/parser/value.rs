use serde_json::Value;

use super::{FixtureError, TurnEvidence};

pub(super) fn completed_output(value: &Value) -> Result<String, FixtureError> {
    let output = value
        .pointer("/response/output")
        .and_then(Value::as_array)
        .ok_or(FixtureError::MissingField)?;
    if output.len() != 1
        || string(&output[0], "/type")? != "message"
        || string(&output[0], "/role")? != "assistant"
    {
        return Err(FixtureError::InvalidField);
    }
    let content = output[0]
        .get("content")
        .and_then(Value::as_array)
        .ok_or(FixtureError::MissingField)?;
    if content.len() != 1 || string(&content[0], "/type")? != "output_text" {
        return Err(FixtureError::InvalidField);
    }
    Ok(string(&content[0], "/text")?.to_owned())
}

pub(super) fn optional_usage(
    value: &Value,
    pointer: &str,
) -> Result<Option<TurnEvidence>, FixtureError> {
    match value.pointer(pointer) {
        None | Some(Value::Null) => Ok(None),
        Some(_) => usage(value, pointer).map(Some),
    }
}

pub(super) fn usage(value: &Value, pointer: &str) -> Result<TurnEvidence, FixtureError> {
    let usage = value.pointer(pointer).ok_or(FixtureError::MissingField)?;
    let input_tokens = integer(usage, "/input_tokens")?;
    let output_tokens = integer(usage, "/output_tokens")?;
    let total_tokens = integer(usage, "/total_tokens")?;
    if input_tokens.checked_add(output_tokens) != Some(total_tokens) {
        return Err(FixtureError::InvalidField);
    }
    Ok(TurnEvidence {
        input_tokens,
        output_tokens,
        total_tokens,
        cost_in_usd_ticks: integer(usage, "/cost_in_usd_ticks")?,
    })
}

pub(super) fn string<'a>(value: &'a Value, pointer: &str) -> Result<&'a str, FixtureError> {
    value
        .pointer(pointer)
        .and_then(Value::as_str)
        .ok_or(FixtureError::MissingField)
}

fn integer(value: &Value, pointer: &str) -> Result<u64, FixtureError> {
    value
        .pointer(pointer)
        .and_then(Value::as_u64)
        .ok_or(FixtureError::InvalidField)
}
