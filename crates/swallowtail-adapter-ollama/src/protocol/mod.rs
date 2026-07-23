use crate::failure::failure;
use serde::Deserialize;
use swallowtail_runtime::RuntimeFailure;

mod catalog;
mod chat;
mod ndjson;

pub use catalog::{ObservationBinding, parse_inventory, parse_model_detail, parse_version};
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
        encode_json(
            "/api/chat",
            &serde_json::json!({
                "model": model,
                "messages": [{
                    "role": "user",
                    "content": content
                }],
                "stream": true,
                "options": {
                    "num_predict": maximum
                }
            }),
        )
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
    let message = match response.status {
        400 => "Ollama rejected the native request",
        404 => "Ollama did not expose the selected model",
        429 => "Ollama rejected the request due to a provider limit",
        _ => "Ollama native request failed",
    };
    Err(failure("swallowtail.ollama.provider_failed", message))
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
