use crate::failure::failure;
use serde::{Deserialize, Serialize};
use swallowtail_core::{
    FailureClassification, FailureKind, FailureOrigin, FailureRecovery, ReasoningMode,
};
use swallowtail_runtime::{RuntimeFailure, SchemaDocument, StructuredOutputDescriptor};

mod catalog;
mod chat;
mod ndjson;

pub use catalog::{
    ObservationBinding, OllamaModelCapability, SelectedModelDetail, parse_inventory,
    parse_model_detail, parse_version,
};
pub use chat::{ChatDecoder, NativeEvent};

const MAX_CATALOG_MODELS: usize = 256;
const MAX_RESPONSE_BYTES: usize = 1_048_576;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Request {
    pub method: Method,
    pub path: &'static str,
    pub body: Option<Vec<u8>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Method {
    Get,
    Post,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct ChatMessage {
    role: &'static str,
    content: String,
}

impl ChatMessage {
    pub(crate) fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user",
            content: content.into(),
        }
    }

    pub(crate) fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: "assistant",
            content: content.into(),
        }
    }
}

impl Request {
    pub const fn version() -> Self {
        Self::get("/api/version")
    }

    pub const fn installed_models() -> Self {
        Self::get("/api/tags")
    }

    pub const fn running_models() -> Self {
        Self::get("/api/ps")
    }

    pub fn show(model: &str) -> Result<Self, RuntimeFailure> {
        encode_json(
            "/api/show",
            &serde_json::json!({
                "model": model
            }),
        )
    }

    pub fn chat(
        model: &str,
        content: &str,
        maximum_output_tokens: u64,
        reasoning: Option<&ReasoningMode>,
        structured_output: Option<&StructuredOutputDescriptor>,
    ) -> Result<Self, RuntimeFailure> {
        let messages = [ChatMessage::user(content)];
        Self::chat_with_messages(
            model,
            &messages,
            maximum_output_tokens,
            reasoning,
            structured_output,
        )
    }

    pub(crate) fn chat_history(
        model: &str,
        messages: &[ChatMessage],
        maximum_output_tokens: u64,
    ) -> Result<Self, RuntimeFailure> {
        Self::chat_with_messages(model, messages, maximum_output_tokens, None, None)
    }

    fn chat_with_messages(
        model: &str,
        messages: &[ChatMessage],
        maximum_output_tokens: u64,
        reasoning: Option<&ReasoningMode>,
        structured_output: Option<&StructuredOutputDescriptor>,
    ) -> Result<Self, RuntimeFailure> {
        let maximum = u32::try_from(maximum_output_tokens)
            .ok()
            .filter(|maximum| *maximum > 0)
            .ok_or_else(|| {
                failure(
                    "swallowtail.ollama.output_limit_invalid",
                    "Ollama maximum output tokens exceeded the supported request range",
                )
            })?;
        let mut request = serde_json::json!({
            "model": model,
            "messages": messages,
            "stream": true,
            "options": {
                "num_predict": maximum
            }
        });
        if let Some(reasoning) = reasoning {
            request["think"] = match reasoning.as_str() {
                "off" => serde_json::Value::Bool(false),
                value => serde_json::Value::String(value.to_owned()),
            };
        }
        if let Some(output) = structured_output {
            request["format"] = match output.document() {
                SchemaDocument::Inline(bytes) => serde_json::from_slice(bytes)
                    .map_err(|_| protocol_failure("structured-output schema"))?,
                SchemaDocument::Reference(_) => {
                    return Err(protocol_failure("structured-output schema"));
                }
            };
        }
        encode_json("/api/chat", &request)
    }

    const fn get(path: &'static str) -> Self {
        Self {
            method: Method::Get,
            path,
            body: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Response {
    pub status: u32,
    pub body: Vec<u8>,
}

pub fn require_success(
    response: &Response,
    _operation: &'static str,
) -> Result<(), RuntimeFailure> {
    if (200..300).contains(&response.status) {
        return Ok(());
    }
    let (message, kind, recovery) = match response.status {
        400 => (
            "Ollama rejected the native request",
            FailureKind::InvalidRequest,
            FailureRecovery::InputChangeRequired,
        ),
        404 => (
            "Ollama did not expose the selected model",
            FailureKind::ModelUnavailable,
            FailureRecovery::ConfigurationChangeRequired,
        ),
        429 => (
            "Ollama rejected the request due to a provider limit",
            FailureKind::RateLimited,
            FailureRecovery::RetryMaySucceed,
        ),
        _ => (
            "Ollama native request failed",
            FailureKind::Unknown,
            FailureRecovery::Unknown,
        ),
    };
    Err(
        failure("swallowtail.ollama.provider_failed", message).with_failure_classification(
            FailureClassification::new(FailureOrigin::Provider, kind, recovery),
        ),
    )
}

fn encode_json(path: &'static str, value: &serde_json::Value) -> Result<Request, RuntimeFailure> {
    let body = serde_json::to_vec(value).map_err(|_| protocol_failure("request"))?;
    if body.len() > MAX_RESPONSE_BYTES {
        return Err(limit_failure());
    }
    Ok(Request {
        method: Method::Post,
        path,
        body: Some(body),
    })
}

fn bounded_json<T: for<'de> Deserialize<'de>>(
    bytes: &[u8],
    subject: &'static str,
) -> Result<T, RuntimeFailure> {
    if bytes.len() > MAX_RESPONSE_BYTES {
        return Err(limit_failure());
    }
    serde_json::from_slice(bytes).map_err(|_| protocol_failure(subject))
}

fn protocol_failure(subject: &'static str) -> RuntimeFailure {
    let _ = subject;
    failure(
        "swallowtail.ollama.protocol_invalid",
        "Ollama native response was invalid",
    )
}

fn unsupported_semantics() -> RuntimeFailure {
    failure(
        "swallowtail.ollama.semantics_unsupported",
        "Ollama emitted semantics outside the qualified text-only subset",
    )
}

fn limit_failure() -> RuntimeFailure {
    failure(
        "swallowtail.ollama.protocol_limit",
        "Ollama native response exceeded a bounded protocol limit",
    )
}

#[cfg(test)]
mod tests;
