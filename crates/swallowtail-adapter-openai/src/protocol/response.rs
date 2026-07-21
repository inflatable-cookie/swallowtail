use crate::failure::malformed;
use serde_json::Value;
use swallowtail_runtime::{RuntimeFailure, TokenUsage};

#[derive(Debug)]
pub(crate) struct Response {
    pub status: u32,
    pub headers: std::collections::BTreeMap<String, String>,
    pub body: Vec<u8>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum BackgroundStatus {
    Queued,
    InProgress,
    Completed,
    Incomplete,
    Failed,
    Cancelled,
}

impl BackgroundStatus {
    pub(crate) fn parse(value: &str) -> Result<Self, RuntimeFailure> {
        match value {
            "queued" => Ok(Self::Queued),
            "in_progress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            "incomplete" => Ok(Self::Incomplete),
            "failed" => Ok(Self::Failed),
            "cancelled" => Ok(Self::Cancelled),
            _ => Err(malformed()),
        }
    }

    pub(crate) const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Completed | Self::Incomplete | Self::Failed | Self::Cancelled
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ResponseSnapshot {
    pub response_id: String,
    pub status: BackgroundStatus,
    pub output_text: Option<String>,
    pub usage: Option<TokenUsage>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ProviderFailureKind {
    Authentication,
    Permission,
    RateLimited,
    Quota,
    InvalidRequest,
    Other,
}

pub(crate) fn parse_snapshot(bytes: &[u8]) -> Result<ResponseSnapshot, RuntimeFailure> {
    let value: Value = serde_json::from_slice(bytes).map_err(|_| malformed())?;
    parse_snapshot_value(&value)
}

pub(crate) fn parse_snapshot_value(value: &Value) -> Result<ResponseSnapshot, RuntimeFailure> {
    let response_id = string(value, "/id")?.to_owned();
    if !response_id.starts_with("resp_") {
        return Err(malformed());
    }
    let status = BackgroundStatus::parse(string(value, "/status")?)?;
    let usage = value
        .get("usage")
        .filter(|value| !value.is_null())
        .map(parse_usage)
        .transpose()?;
    let output_text = if status == BackgroundStatus::Completed {
        Some(parse_output(value)?)
    } else {
        None
    };
    Ok(ResponseSnapshot {
        response_id,
        status,
        output_text,
        usage,
    })
}

pub(crate) fn parse_failure(bytes: &[u8]) -> Result<ProviderFailureKind, RuntimeFailure> {
    let value: Value = serde_json::from_slice(bytes).map_err(|_| malformed())?;
    Ok(match string(&value, "/error/code")? {
        "invalid_api_key" => ProviderFailureKind::Authentication,
        "insufficient_permissions" => ProviderFailureKind::Permission,
        "rate_limit_exceeded" => ProviderFailureKind::RateLimited,
        "insufficient_quota" => ProviderFailureKind::Quota,
        "invalid_request_error" => ProviderFailureKind::InvalidRequest,
        _ => ProviderFailureKind::Other,
    })
}

pub(crate) fn require_success(response: &Response) -> Result<(), RuntimeFailure> {
    if (200..300).contains(&response.status) {
        return Ok(());
    }
    let kind = parse_failure(&response.body).unwrap_or(ProviderFailureKind::Other);
    let (code, message) = match kind {
        ProviderFailureKind::Authentication => (
            "swallowtail.openai.authentication_rejected",
            "OpenAI public API authentication was rejected",
        ),
        ProviderFailureKind::Permission => (
            "swallowtail.openai.permission_denied",
            "OpenAI public API permission was denied",
        ),
        ProviderFailureKind::RateLimited => (
            "swallowtail.openai.rate_limited",
            "OpenAI public API request was rate limited",
        ),
        ProviderFailureKind::Quota => (
            "swallowtail.openai.quota_exhausted",
            "OpenAI public API quota was exhausted",
        ),
        ProviderFailureKind::InvalidRequest => (
            "swallowtail.openai.invalid_request",
            "OpenAI public API rejected the request",
        ),
        ProviderFailureKind::Other => (
            "swallowtail.openai.provider_failed",
            "OpenAI public API request failed",
        ),
    };
    Err(crate::failure::failure(code, message))
}

fn parse_usage(value: &Value) -> Result<TokenUsage, RuntimeFailure> {
    let input = integer(value, "/input_tokens")?;
    let output = integer(value, "/output_tokens")?;
    let total = integer(value, "/total_tokens")?;
    if input.checked_add(output) != Some(total) {
        return Err(malformed());
    }
    Ok(TokenUsage::new(Some(input), Some(output)))
}

fn parse_output(value: &Value) -> Result<String, RuntimeFailure> {
    let output = value
        .get("output")
        .and_then(Value::as_array)
        .ok_or_else(malformed)?;
    if output.len() != 1
        || string(&output[0], "/type")? != "message"
        || string(&output[0], "/role")? != "assistant"
    {
        return Err(malformed());
    }
    let content = output[0]
        .get("content")
        .and_then(Value::as_array)
        .ok_or_else(malformed)?;
    if content.len() != 1 || string(&content[0], "/type")? != "output_text" {
        return Err(malformed());
    }
    Ok(string(&content[0], "/text")?.to_owned())
}

pub(crate) fn string<'a>(value: &'a Value, pointer: &str) -> Result<&'a str, RuntimeFailure> {
    value
        .pointer(pointer)
        .and_then(Value::as_str)
        .ok_or_else(malformed)
}

pub(crate) fn integer(value: &Value, pointer: &str) -> Result<u64, RuntimeFailure> {
    value
        .pointer(pointer)
        .and_then(Value::as_u64)
        .ok_or_else(malformed)
}
