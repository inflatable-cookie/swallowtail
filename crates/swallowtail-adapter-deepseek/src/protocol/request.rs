use super::private::PrivateContinuation;
use super::{ProtocolFailure, ProtocolFailureKind};
use crate::DeepSeekThinkingMode;
use crate::selection::DEEPSEEK_MODEL_ID;
use serde_json::{Value, json};
use swallowtail_core::ReasoningMode;
use swallowtail_protocol_openai_chat::{ChatMessage, ChatRequest, CodecLimits, encode_request};

#[derive(Clone)]
pub(crate) struct ToolSpec {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) parameters: Value,
}

pub(crate) fn encode_initial(
    user: &str,
    tools: &[ToolSpec],
    reasoning: &ReasoningMode,
) -> Result<Vec<u8>, ProtocolFailure> {
    encode(
        vec![ChatMessage::new("user", user)],
        tools,
        false,
        8_192,
        Some(reasoning),
        None,
    )
}

pub(crate) fn encode_structured(
    user: &str,
    maximum_output_tokens: u64,
    reasoning: Option<&ReasoningMode>,
    thinking_mode: Option<DeepSeekThinkingMode>,
) -> Result<Vec<u8>, ProtocolFailure> {
    encode(
        vec![ChatMessage::new("user", user)],
        &[],
        true,
        maximum_output_tokens,
        reasoning,
        thinking_mode,
    )
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
    reasoning_mode: &ReasoningMode,
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
        8_192,
        Some(reasoning_mode),
        None,
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
    reasoning_mode: &ReasoningMode,
) -> Result<Vec<u8>, ProtocolFailure> {
    let after_tool = encode_after_tool(
        first_user,
        tool_reasoning,
        call_id,
        tool_name,
        arguments,
        tool_result,
        tools,
        reasoning_mode,
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
    maximum_output_tokens: u64,
    reasoning: Option<&ReasoningMode>,
    thinking_mode: Option<DeepSeekThinkingMode>,
) -> Result<Vec<u8>, ProtocolFailure> {
    let mut request = ChatRequest::new(DEEPSEEK_MODEL_ID, messages, stream, stream);
    request
        .insert_extension("max_tokens", json!(maximum_output_tokens))
        .map_err(invalid)?;
    if let Some(reasoning) = reasoning {
        request
            .insert_extension("reasoning_effort", json!(reasoning.as_str()))
            .map_err(invalid)?;
    }
    request
        .insert_extension(
            "thinking",
            json!({"type": thinking_mode.map_or("enabled", |mode| mode.as_str())}),
        )
        .map_err(invalid)?;
    request
        .insert_extension(
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
        )
        .map_err(invalid)?;
    encode_request(&request, CodecLimits::default()).map_err(invalid)
}

fn invalid<T>(_error: T) -> ProtocolFailure {
    ProtocolFailure::new(ProtocolFailureKind::InvalidStructure)
}
