use super::private::PrivateContinuation;
use super::{ProtocolFailure, ProtocolFailureKind};
use crate::selection::DEEPSEEK_MODEL_ID;
use serde::Deserialize;
use swallowtail_core::DirectContinuationConfig;
use swallowtail_runtime::{
    DirectInferenceAttemptId, DirectToolArguments, DirectToolCall, DirectToolCallId,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ProviderFailureKind {
    InvalidRequest,
    Authentication,
    InsufficientBalance,
    AccountConcurrency,
    Provider,
    Overloaded,
}

pub(crate) fn classify_failure(status: u16) -> Option<ProviderFailureKind> {
    match status {
        400 => Some(ProviderFailureKind::InvalidRequest),
        401 => Some(ProviderFailureKind::Authentication),
        402 => Some(ProviderFailureKind::InsufficientBalance),
        429 => Some(ProviderFailureKind::AccountConcurrency),
        500 => Some(ProviderFailureKind::Provider),
        503 => Some(ProviderFailureKind::Overloaded),
        _ => None,
    }
}

pub(crate) struct ToolAttempt {
    pub(crate) call: DirectToolCall,
    pub(crate) reasoning: PrivateContinuation,
    pub(crate) usage: Usage,
}

impl std::fmt::Debug for ToolAttempt {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("ToolAttempt(<redacted>)")
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Usage {
    pub(crate) prompt_tokens: u64,
    pub(crate) completion_tokens: u64,
    pub(crate) total_tokens: u64,
    pub(crate) cache_hit_tokens: u64,
    pub(crate) cache_miss_tokens: u64,
}

pub(crate) fn parse_tool_attempt(
    bytes: &[u8],
    attempt_id: DirectInferenceAttemptId,
    config: &DirectContinuationConfig,
) -> Result<ToolAttempt, ProtocolFailure> {
    if bytes.len() as u64 > config.maximum_private_history_bytes().get() {
        return Err(ProtocolFailure::new(ProtocolFailureKind::BoundExceeded));
    }
    let response: Completion = serde_json::from_slice(bytes)
        .map_err(|_| ProtocolFailure::new(ProtocolFailureKind::InvalidStructure))?;
    if response.object != "chat.completion"
        || response.model != DEEPSEEK_MODEL_ID
        || response.choices.len() != 1
    {
        return Err(ProtocolFailure::new(
            if response.model != DEEPSEEK_MODEL_ID {
                ProtocolFailureKind::ModelMismatch
            } else {
                ProtocolFailureKind::InvalidStructure
            },
        ));
    }
    let choice = response.choices.into_iter().next().unwrap();
    if choice.index != 0
        || choice.finish_reason != "tool_calls"
        || choice.message.role != "assistant"
        || choice.message.content.is_some()
        || choice.message.tool_calls.len() != 1
    {
        return Err(ProtocolFailure::new(ProtocolFailureKind::InvalidStructure));
    }
    let tool = choice.message.tool_calls.into_iter().next().unwrap();
    if tool.kind != "function" {
        return Err(ProtocolFailure::new(ProtocolFailureKind::InvalidStructure));
    }
    let maximum_arguments = usize::try_from(config.maximum_tool_argument_bytes().get())
        .map_err(|_| ProtocolFailure::new(ProtocolFailureKind::BoundExceeded))?;
    let arguments =
        DirectToolArguments::new(tool.function.arguments.into_bytes(), maximum_arguments)
            .map_err(|_| ProtocolFailure::new(ProtocolFailureKind::BoundExceeded))?;
    let call = DirectToolCall::new(
        DirectToolCallId::new(tool.id)
            .map_err(|_| ProtocolFailure::new(ProtocolFailureKind::InvalidStructure))?,
        attempt_id,
        tool.function.name,
        arguments,
    )
    .map_err(|_| ProtocolFailure::new(ProtocolFailureKind::InvalidStructure))?;
    let reasoning = PrivateContinuation::new(
        choice.message.reasoning_content.into_bytes(),
        config.maximum_private_continuation_bytes(),
    )?;
    Ok(ToolAttempt {
        call,
        reasoning,
        usage: response.usage.into(),
    })
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Completion {
    #[serde(rename = "id")]
    _id: String,
    object: String,
    #[serde(rename = "created")]
    _created: u64,
    model: String,
    choices: Vec<Choice>,
    usage: WireUsage,
    #[serde(rename = "system_fingerprint")]
    _system_fingerprint: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Choice {
    index: u64,
    message: AssistantMessage,
    finish_reason: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AssistantMessage {
    role: String,
    content: Option<String>,
    reasoning_content: String,
    tool_calls: Vec<WireToolCall>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WireToolCall {
    id: String,
    #[serde(rename = "type")]
    kind: String,
    function: WireFunction,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WireFunction {
    name: String,
    arguments: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WireUsage {
    prompt_tokens: u64,
    completion_tokens: u64,
    total_tokens: u64,
    prompt_cache_hit_tokens: u64,
    prompt_cache_miss_tokens: u64,
}

impl From<WireUsage> for Usage {
    fn from(value: WireUsage) -> Self {
        Self {
            prompt_tokens: value.prompt_tokens,
            completion_tokens: value.completion_tokens,
            total_tokens: value.total_tokens,
            cache_hit_tokens: value.prompt_cache_hit_tokens,
            cache_miss_tokens: value.prompt_cache_miss_tokens,
        }
    }
}
