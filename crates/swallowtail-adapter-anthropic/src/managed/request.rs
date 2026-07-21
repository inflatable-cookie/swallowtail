use super::{Request, Value};
use crate::failure::failure;
use serde_json::json;
use swallowtail_runtime::{OperationContent, RuntimeFailure, ToolDeclaration};

impl Request {
    pub(crate) fn agent(agent_id: &str, version: u64) -> Self {
        Self::get(
            format!("/v1/agents/{agent_id}"),
            vec![("version".to_owned(), version.to_string())],
        )
    }

    pub(crate) fn create_environment(name: &str) -> Self {
        Self::post(
            "/v1/environments".to_owned(),
            json!({
                "name": name,
                "config": {
                    "type": "cloud",
                    "networking": {
                        "type": "limited",
                        "allowed_hosts": [],
                        "allow_mcp_servers": false,
                        "allow_package_managers": false
                    }
                }
            }),
        )
    }

    pub(crate) fn create_session(
        agent_id: &str,
        agent_version: u64,
        model: &str,
        environment_id: &str,
        tools: &[&ToolDeclaration],
    ) -> Result<Self, RuntimeFailure> {
        let tools = tools
            .iter()
            .map(|tool| translate_tool(tool))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self::post(
            "/v1/sessions".to_owned(),
            json!({
                "agent": {
                    "type": "agent_with_overrides",
                    "id": agent_id,
                    "version": agent_version,
                    "model": model,
                    "tools": tools,
                    "mcp_servers": [],
                    "skills": []
                },
                "environment_id": environment_id,
                "resources": [],
                "vault_ids": []
            }),
        ))
    }

    pub(crate) fn session(session_id: &str) -> Self {
        Self::get(format!("/v1/sessions/{session_id}"), Vec::new())
    }

    pub(crate) fn stream(session_id: &str) -> Self {
        Self::get(
            format!("/v1/sessions/{session_id}/events/stream"),
            Vec::new(),
        )
    }

    pub(crate) fn history(session_id: &str) -> Self {
        Self::get(
            format!("/v1/sessions/{session_id}/events"),
            vec![
                ("limit".to_owned(), "1000".to_owned()),
                ("order".to_owned(), "asc".to_owned()),
            ],
        )
    }

    pub(crate) fn message(session_id: &str, content: &OperationContent) -> Self {
        Self::post(
            format!("/v1/sessions/{session_id}/events"),
            json!({
                "events": [{
                    "type": "user.message",
                    "content": [{"type": "text", "text": content.as_str()}]
                }]
            }),
        )
    }

    pub(crate) fn custom_tool_result(
        session_id: &str,
        custom_tool_use_id: &str,
        content: &OperationContent,
    ) -> Self {
        Self::post(
            format!("/v1/sessions/{session_id}/events"),
            json!({
                "events": [{
                    "type": "user.custom_tool_result",
                    "custom_tool_use_id": custom_tool_use_id,
                    "content": [{"type": "text", "text": content.as_str()}]
                }]
            }),
        )
    }

    pub(crate) fn interrupt(session_id: &str) -> Self {
        Self::post(
            format!("/v1/sessions/{session_id}/events"),
            json!({"events": [{"type": "user.interrupt"}]}),
        )
    }

    pub(crate) fn delete_session(session_id: &str) -> Self {
        Self::delete(format!("/v1/sessions/{session_id}"))
    }

    pub(crate) fn delete_environment(environment_id: &str) -> Self {
        Self::delete(format!("/v1/environments/{environment_id}"))
    }
}

fn translate_tool(tool: &ToolDeclaration) -> Result<Value, RuntimeFailure> {
    if tool.schema_media_type() != "application/schema+json"
        || tool.schema_dialect() != "json-schema-2020-12"
    {
        return Err(protocol_failure("custom tool schema metadata"));
    }
    let Some(bytes) = tool.input_schema().inline_bytes() else {
        return Err(protocol_failure("referenced custom tool schema"));
    };
    let schema: Value =
        serde_json::from_slice(bytes).map_err(|_| protocol_failure("custom tool schema"))?;
    if schema.get("type").and_then(Value::as_str) != Some("object") {
        return Err(protocol_failure("custom tool object schema"));
    }
    let Some(description) = tool.description() else {
        return Err(protocol_failure("custom tool description"));
    };
    if description.as_str().len() > 4096
        || tool.name().len() > 128
        || !tool
            .name()
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || matches!(character, '_' | '-'))
    {
        return Err(protocol_failure("custom tool bounds"));
    }
    Ok(json!({
        "type": "custom",
        "name": tool.name(),
        "description": description.as_str(),
        "input_schema": schema
    }))
}

fn protocol_failure(subject: &str) -> RuntimeFailure {
    failure(
        "swallowtail.anthropic.managed.protocol_invalid",
        format!("Anthropic Managed Agents {subject} was invalid"),
    )
}
