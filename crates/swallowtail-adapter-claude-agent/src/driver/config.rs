use serde_json::Value;
use swallowtail_core::ReasoningMode;
use swallowtail_runtime::RuntimeFailure;

use super::{failure, malformed};

const MODEL_CONFIG_ID: &str = "model";
const EFFORT_CONFIG_ID: &str = "effort";
const MODE_CONFIG_ID: &str = "mode";
const THOUGHT_LEVEL_CATEGORY: &str = "thought_level";
const MODE_CATEGORY: &str = "mode";
const WRITE_MODE_ID: &str = "acceptEdits";
const PLAN_MODE_ID: &str = "plan";

pub(super) fn parse_session_id(response: &Value) -> Result<String, RuntimeFailure> {
    response
        .get("sessionId")
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or_else(malformed)
}

pub(super) fn validate_legacy_model(
    response: &Value,
    expected: &str,
) -> Result<(), RuntimeFailure> {
    confirm_value(
        response,
        MODEL_CONFIG_ID,
        expected,
        "swallowtail.claude_agent.acp.model_mismatch",
        "Claude Agent session model does not match the preflight route",
    )
}

pub(super) fn validate_model_option(response: &Value) -> Result<(), RuntimeFailure> {
    let _ = select_option(response, MODEL_CONFIG_ID, None)?;
    Ok(())
}

pub(super) fn confirm_model(response: &Value, expected: &str) -> Result<(), RuntimeFailure> {
    confirm_value(
        response,
        MODEL_CONFIG_ID,
        expected,
        "swallowtail.claude_agent.acp.model_mismatch",
        "Claude Agent session model does not match the preflight route",
    )
}

pub(super) fn validate_reasoning_option(
    response: &Value,
    requested: &ReasoningMode,
) -> Result<(), RuntimeFailure> {
    let option = select_option(response, EFFORT_CONFIG_ID, Some(THOUGHT_LEVEL_CATEGORY))?;
    let supported = option
        .get("options")
        .and_then(Value::as_array)
        .ok_or_else(malformed)?
        .iter()
        .any(|candidate| {
            candidate.get("value").and_then(Value::as_str) == Some(requested.as_str())
        });
    if supported {
        Ok(())
    } else {
        Err(failure(
            "swallowtail.claude_agent.acp.reasoning_mode_unsupported",
            "Claude Agent model does not advertise the requested reasoning mode",
        ))
    }
}

pub(super) fn confirm_reasoning(
    response: &Value,
    requested: &ReasoningMode,
) -> Result<(), RuntimeFailure> {
    confirm_value(
        response,
        EFFORT_CONFIG_ID,
        requested.as_str(),
        "swallowtail.claude_agent.acp.reasoning_mismatch",
        "Claude Agent reasoning confirmation does not match the requested mode",
    )
}

pub(super) fn validate_plan_mode_option(response: &Value) -> Result<(), RuntimeFailure> {
    let option = select_option(response, MODE_CONFIG_ID, Some(MODE_CATEGORY))?;
    if option
        .get("options")
        .and_then(Value::as_array)
        .ok_or_else(malformed)?
        .iter()
        .any(|candidate| candidate.get("value").and_then(Value::as_str) == Some(PLAN_MODE_ID))
    {
        Ok(())
    } else {
        Err(failure(
            "swallowtail.claude_agent.acp.harness_mode_unsupported",
            "Claude Agent does not advertise the requested harness mode",
        ))
    }
}

pub(super) fn confirm_plan_mode(response: &Value) -> Result<(), RuntimeFailure> {
    confirm_value(
        response,
        MODE_CONFIG_ID,
        PLAN_MODE_ID,
        "swallowtail.claude_agent.acp.harness_mode_mismatch",
        "Claude Agent harness-mode confirmation does not match the requested mode",
    )
}

pub(super) fn validate_write_mode(response: &Value) -> Result<(), RuntimeFailure> {
    let modes = response
        .get("modes")
        .and_then(|modes| modes.get("availableModes"))
        .and_then(Value::as_array)
        .ok_or_else(malformed)?;
    if modes
        .iter()
        .any(|mode| mode.get("id").and_then(Value::as_str) == Some(WRITE_MODE_ID))
    {
        Ok(())
    } else {
        Err(failure(
            "swallowtail.claude_agent.acp.write_mode_unsupported",
            "Claude Agent session does not advertise the required edit-acceptance mode",
        ))
    }
}

fn confirm_value(
    response: &Value,
    config_id: &str,
    expected: &str,
    code: &'static str,
    message: &'static str,
) -> Result<(), RuntimeFailure> {
    let option = select_option(
        response,
        config_id,
        match config_id {
            EFFORT_CONFIG_ID => Some(THOUGHT_LEVEL_CATEGORY),
            MODE_CONFIG_ID => Some(MODE_CATEGORY),
            _ => None,
        },
    )?;
    if option.get("currentValue").and_then(Value::as_str) == Some(expected) {
        Ok(())
    } else {
        Err(failure(code, message))
    }
}

fn select_option<'a>(
    response: &'a Value,
    config_id: &str,
    category: Option<&str>,
) -> Result<&'a Value, RuntimeFailure> {
    let options = response
        .get("configOptions")
        .and_then(Value::as_array)
        .ok_or_else(malformed)?;
    let mut matches = options
        .iter()
        .filter(|option| option.get("id").and_then(Value::as_str) == Some(config_id));
    let option = matches.next().ok_or_else(|| {
        failure(
            "swallowtail.claude_agent.acp.config_option_missing",
            "Claude Agent did not advertise a required session config option",
        )
    })?;
    if matches.next().is_some()
        || option.get("type").and_then(Value::as_str) != Some("select")
        || category.is_some_and(|category| {
            option.get("category").and_then(Value::as_str) != Some(category)
        })
        || option.get("currentValue").and_then(Value::as_str).is_none()
    {
        return Err(malformed());
    }
    Ok(option)
}
