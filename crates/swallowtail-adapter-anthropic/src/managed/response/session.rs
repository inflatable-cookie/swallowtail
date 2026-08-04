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
    Ok(parse_session_snapshot(
        input,
        expected_environment_id,
        expected_agent_id,
        expected_version,
        expected_model,
    )?
    .id)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ManagedSessionStatus {
    Running,
    Idle,
    Terminated,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ManagedSessionSnapshot {
    pub id: String,
    pub status: ManagedSessionStatus,
    pub usage: Option<TokenUsage>,
}

pub(crate) fn parse_session_snapshot(
    input: &[u8],
    expected_environment_id: &str,
    expected_agent_id: &str,
    expected_version: u64,
    expected_model: &str,
) -> Result<ManagedSessionSnapshot, RuntimeFailure> {
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
    let status = match value.get("status").and_then(Value::as_str) {
        Some("running") => ManagedSessionStatus::Running,
        Some("idle") => ManagedSessionStatus::Idle,
        Some("terminated") => ManagedSessionStatus::Terminated,
        _ => return Err(protocol_failure("session status")),
    };
    let usage = value.get("usage").map(parse_usage).transpose()?;
    Ok(ManagedSessionSnapshot { id, status, usage })
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
    parse_usage(usage)
}

fn parse_usage(usage: &Value) -> Result<TokenUsage, RuntimeFailure> {
    let input = usage.get("input_tokens").and_then(Value::as_u64);
    let output = usage.get("output_tokens").and_then(Value::as_u64);
    if input.is_none() && output.is_none() {
        return Err(protocol_failure("session usage"));
    }
    Ok(TokenUsage::new(input, output))
}
