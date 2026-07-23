use super::private::PrivateContinuation;
use super::{ProtocolFailure, ProtocolFailureKind};
use crate::selection::DEEPSEEK_MODEL_ID;
use serde_json::{Value, json};
use swallowtail_protocol_openai_chat::{ChatMessage, ChatRequest, CodecLimits, encode_request};

#[derive(Clone)]
pub(crate) struct ToolSpec {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) parameters: Value,
}

pub(crate) fn encode_initial(user: &str, tools: &[ToolSpec]) -> Result<Vec<u8>, ProtocolFailure> {
    encode(vec![ChatMessage::new("user", user)], tools, false)
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn encode_after_tool(
    user: &str,
    reasoning: &PrivateContinuation,
    call_id: &str,
    tool_name: &str,
    arguments: &str,
    tool_result: &str,
    tools: &[ToolSpec],
) -> Result<Vec<u8>, ProtocolFailure> {
    let mut assistant = ChatMessage::without_content("assistant");
    assistant
        .insert_extension("reasoning_content", json!(reasoning.as_str()))
        .map_err(invalid)?;
    assistant
        .insert_extension(
            "tool_calls",
            json!([{
                "id": call_id,
                "type": "function",
                "function": {"name": tool_name, "arguments": arguments}
            }]),
        )
        .map_err(invalid)?;
    let mut result = ChatMessage::new("tool", tool_result);
    result
        .insert_extension("tool_call_id", json!(call_id))
        .map_err(invalid)?;
    encode(
        vec![ChatMessage::new("user", user), assistant, result],
        tools,
        true,
    )
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn encode_later_user(
    first_user: &str,
    tool_reasoning: &PrivateContinuation,
    call_id: &str,
    tool_name: &str,
    arguments: &str,
    tool_result: &str,
    final_reasoning: &PrivateContinuation,
    first_answer: &str,
    next_user: &str,
    tools: &[ToolSpec],
) -> Result<Vec<u8>, ProtocolFailure> {
    let after_tool = encode_after_tool(
        first_user,
        tool_reasoning,
        call_id,
        tool_name,
        arguments,
        tool_result,
        tools,
    )?;
    let mut value: Value = serde_json::from_slice(&after_tool).map_err(invalid)?;
    let messages = value["messages"]
        .as_array_mut()
        .ok_or_else(|| ProtocolFailure::new(ProtocolFailureKind::InvalidStructure))?;
    messages.push(json!({
        "role": "assistant",
        "content": first_answer,
        "reasoning_content": final_reasoning.as_str()
    }));
    messages.push(json!({"role":"user", "content":next_user}));
    serde_json::to_vec(&value).map_err(invalid)
}

fn encode(
    messages: Vec<ChatMessage>,
    tools: &[ToolSpec],
    stream: bool,
) -> Result<Vec<u8>, ProtocolFailure> {
    let mut request = ChatRequest::new(DEEPSEEK_MODEL_ID, messages, stream, stream);
    for (name, value) in [
        ("max_tokens", json!(8_192)),
        ("reasoning_effort", json!("high")),
        ("thinking", json!({"type":"enabled"})),
        (
            "tools",
            Value::Array(
                tools
                    .iter()
                    .map(|tool| {
                        json!({
                            "type": "function",
                            "function": {
                                "name": tool.name,
                                "description": tool.description,
                                "parameters": tool.parameters
                            }
                        })
                    })
                    .collect(),
            ),
        ),
    ] {
        request.insert_extension(name, value).map_err(invalid)?;
    }
    encode_request(&request, CodecLimits::default()).map_err(invalid)
}

fn invalid<T>(_error: T) -> ProtocolFailure {
    ProtocolFailure::new(ProtocolFailureKind::InvalidStructure)
}
