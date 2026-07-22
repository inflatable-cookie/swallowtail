use crate::failure::failure;
use serde_json::Value;
use swallowtail_runtime::TokenUsage;
use swallowtail_runtime::ToolDeclaration;
use swallowtail_runtime::{OwnedRemoteResourceKind, RemoteResourceDeletionOutcome, RuntimeFailure};

pub(crate) fn validate_agent(
    input: &[u8],
    expected_id: &str,
    expected_version: u64,
    expected_model: &str,
) -> Result<(), RuntimeFailure> {
    let value = response(input, "agent response")?;
    if value.pointer("/id").and_then(Value::as_str) != Some(expected_id)
        || value.pointer("/version").and_then(Value::as_u64) != Some(expected_version)
        || model_id(&value) != Some(expected_model)
        || !value.pointer("/multiagent").is_none_or(Value::is_null)
    {
        return Err(protocol_failure("agent binding"));
    }
    Ok(())
}

#[cfg(test)]
pub(crate) fn validate_environment(input: &[u8], expected_id: &str) -> Result<(), RuntimeFailure> {
    let actual = parse_environment(input)?;
    if actual != expected_id {
        return Err(protocol_failure("environment binding"));
    }
    Ok(())
}

pub(crate) fn parse_environment(input: &[u8]) -> Result<String, RuntimeFailure> {
    let value = response(input, "environment response")?;
    let networking = value
        .pointer("/config/networking")
        .ok_or_else(|| protocol_failure("environment networking"))?;
    let empty_hosts = networking
        .get("allowed_hosts")
        .and_then(Value::as_array)
        .is_some_and(Vec::is_empty);
    let id = required_text(&value, "/id", "environment identity")?;
    if value.pointer("/config/type").and_then(Value::as_str) != Some("cloud")
        || networking.get("type").and_then(Value::as_str) != Some("limited")
        || !empty_hosts
        || networking.get("allow_mcp_servers").and_then(Value::as_bool) != Some(false)
        || networking
            .get("allow_package_managers")
            .and_then(Value::as_bool)
            != Some(false)
    {
        return Err(protocol_failure("environment binding"));
    }
    Ok(id)
}

#[cfg(test)]
pub(crate) fn validate_session(
    input: &[u8],
    expected_id: &str,
    expected_environment_id: &str,
    expected_agent_id: &str,
    expected_version: u64,
    expected_model: &str,
) -> Result<(), RuntimeFailure> {
    let actual = parse_session(
        input,
        expected_environment_id,
        expected_agent_id,
        expected_version,
        expected_model,
    )?;
    if actual != expected_id {
        return Err(protocol_failure("session binding"));
    }
    Ok(())
}

pub(crate) fn parse_session(
    input: &[u8],
    expected_environment_id: &str,
    expected_agent_id: &str,
    expected_version: u64,
    expected_model: &str,
) -> Result<String, RuntimeFailure> {
    let value = response(input, "session response")?;
    let id = required_text(&value, "/id", "session identity")?;
    let agent = value
        .get("agent")
        .ok_or_else(|| protocol_failure("session agent"))?;
    let custom_tools_only = agent
        .get("tools")
        .and_then(Value::as_array)
        .is_some_and(|tools| {
            tools
                .iter()
                .all(|tool| tool.get("type").and_then(Value::as_str) == Some("custom"))
        });
    let empty_mcp = agent
        .get("mcp_servers")
        .and_then(Value::as_array)
        .is_some_and(Vec::is_empty);
    let empty_skills = agent
        .get("skills")
        .and_then(Value::as_array)
        .is_some_and(Vec::is_empty);
    if value.pointer("/environment_id").and_then(Value::as_str) != Some(expected_environment_id)
        || agent.get("id").and_then(Value::as_str) != Some(expected_agent_id)
        || agent.get("version").and_then(Value::as_u64) != Some(expected_version)
        || model_id(agent) != Some(expected_model)
        || !agent.get("multiagent").is_none_or(Value::is_null)
        || !custom_tools_only
        || !empty_mcp
        || !empty_skills
    {
        return Err(protocol_failure("session binding"));
    }
    Ok(id)
}

pub(crate) fn parse_session_with_tools(
    input: &[u8],
    expected_environment_id: &str,
    expected_agent_id: &str,
    expected_version: u64,
    expected_model: &str,
    expected_tools: &[&ToolDeclaration],
) -> Result<String, RuntimeFailure> {
    let id = parse_session(
        input,
        expected_environment_id,
        expected_agent_id,
        expected_version,
        expected_model,
    )?;
    let value = response(input, "session tool response")?;
    let tools = value
        .pointer("/agent/tools")
        .and_then(Value::as_array)
        .ok_or_else(|| protocol_failure("session tools"))?;
    let actual = tools
        .iter()
        .map(|tool| {
            tool.get("name")
                .and_then(Value::as_str)
                .ok_or_else(|| protocol_failure("session tool identity"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let expected = expected_tools
        .iter()
        .map(|tool| tool.name())
        .collect::<Vec<_>>();
    if actual != expected {
        return Err(protocol_failure("session tool binding"));
    }
    Ok(id)
}

pub(crate) fn parse_session_usage(input: &[u8]) -> Result<TokenUsage, RuntimeFailure> {
    let value = response(input, "session usage response")?;
    let usage = value
        .get("usage")
        .ok_or_else(|| protocol_failure("session usage"))?;
    let input = usage.get("input_tokens").and_then(Value::as_u64);
    let output = usage.get("output_tokens").and_then(Value::as_u64);
    if input.is_none() && output.is_none() {
        return Err(protocol_failure("session usage"));
    }
    Ok(TokenUsage::new(input, output))
}

pub(crate) fn parse_deletion(
    input: &[u8],
    expected_id: &str,
    kind: OwnedRemoteResourceKind,
) -> Result<RemoteResourceDeletionOutcome, RuntimeFailure> {
    let value = response(input, "deletion response")?;
    let expected_type = match kind {
        OwnedRemoteResourceKind::Environment => "environment_deleted",
        OwnedRemoteResourceKind::Session => "session_deleted",
        OwnedRemoteResourceKind::Conversation | OwnedRemoteResourceKind::ConversationItems => {
            return Err(protocol_failure("unsupported deletion resource"));
        }
    };
    if value.pointer("/id").and_then(Value::as_str) == Some(expected_id)
        && value.pointer("/type").and_then(Value::as_str) == Some(expected_type)
    {
        Ok(RemoteResourceDeletionOutcome::Confirmed)
    } else {
        Err(protocol_failure("deletion confirmation"))
    }
}

fn response(input: &[u8], subject: &str) -> Result<Value, RuntimeFailure> {
    if input.len() > 512 * 1024 {
        return Err(protocol_failure(subject));
    }
    serde_json::from_slice(input).map_err(|_| protocol_failure(subject))
}

fn model_id(value: &Value) -> Option<&str> {
    value.get("model").and_then(|model| {
        model
            .as_str()
            .or_else(|| model.get("id").and_then(Value::as_str))
    })
}

fn required_text(value: &Value, pointer: &str, subject: &str) -> Result<String, RuntimeFailure> {
    value
        .pointer(pointer)
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .map(str::to_owned)
        .ok_or_else(|| protocol_failure(subject))
}

fn protocol_failure(subject: &str) -> RuntimeFailure {
    failure(
        "swallowtail.anthropic.managed.protocol_invalid",
        format!("Anthropic Managed Agents {subject} was invalid"),
    )
}
