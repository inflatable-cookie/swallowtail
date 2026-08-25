use serde_json::Value;
use swallowtail_core::{FailureClassification, FailureKind, FailureOrigin, FailureRecovery};
use swallowtail_runtime::TokenUsage;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum Event {
    MessageStart { id: String, usage: TokenUsage },
    ContentStart(ContentBlock),
    OutputDelta(String),
    InputJsonDelta(String),
    SignatureDelta(RedactedBytes),
    Citation,
    ContentStop,
    Usage(TokenUsage, String),
    MessageStop,
    Ping,
    ProviderFailed(ProviderErrorKind),
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ContentBlock {
    Text,
    ToolUse { id: String, name: String },
    SearchUse { id: String },
    SearchResult { tool_use_id: String },
    Thinking,
    RedactedThinking { data: RedactedBytes },
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct RedactedBytes(Vec<u8>);

impl RedactedBytes {
    fn from_str(value: &str) -> Self {
        Self(value.as_bytes().to_vec())
    }

    #[allow(dead_code)]
    pub(crate) fn as_str(&self) -> Result<&str, RuntimeFailure> {
        std::str::from_utf8(&self.0).map_err(|_| protocol_failure("private continuation encoding"))
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub(crate) fn clone_bytes(&self) -> Vec<u8> {
        self.0.clone()
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }
}

impl std::fmt::Debug for RedactedBytes {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("[redacted]")
    }
}

impl Drop for RedactedBytes {
    fn drop(&mut self) {
        self.0.fill(0);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ProviderErrorKind {
    Authentication,
    Billing,
    Permission,
    RateLimited,
    Overloaded,
    InvalidRequest,
    Other,
}

pub(crate) fn parse_event(frame: &SseFrame) -> Result<Event, RuntimeFailure> {
    let value: Value = parse_json(frame.data.as_bytes(), "stream event")?;
    if value.get("type").and_then(Value::as_str) != Some(frame.name.as_str()) {
        return Err(protocol_failure("stream event type"));
    }
    match frame.name.as_str() {
        "message_start" => Ok(Event::MessageStart {
            id: required_string(&value["message"], "id", "message id")?,
            usage: parse_usage(&value["message"]["usage"]),
        }),
        "content_block_start" => parse_content_start(&value["content_block"]),
        "content_block_delta" => parse_content_delta(&value["delta"]),
        "content_block_stop" => Ok(Event::ContentStop),
        "message_delta" => Ok(Event::Usage(
            parse_usage(&value["usage"]),
            value["delta"]["stop_reason"]
                .as_str()
                .ok_or_else(|| protocol_failure("message stop reason"))?
                .to_owned(),
        )),
        "message_stop" => Ok(Event::MessageStop),
        "ping" => Ok(Event::Ping),
        "error" => Ok(Event::ProviderFailed(classify_error(
            value["error"]["type"].as_str(),
        ))),
        _ => Ok(Event::Unknown),
    }
}

fn parse_content_start(value: &Value) -> Result<Event, RuntimeFailure> {
    let block = match value["type"].as_str() {
        Some("text") => ContentBlock::Text,
        Some("thinking") => {
            if value["thinking"].as_str() != Some("") || value["signature"].as_str() != Some("") {
                return Err(protocol_failure("omitted thinking start"));
            }
            ContentBlock::Thinking
        }
        Some("redacted_thinking") => ContentBlock::RedactedThinking {
            data: RedactedBytes::from_str(&required_string(value, "data", "redacted thinking data")?),
        },
        Some("tool_use") => ContentBlock::ToolUse {
            id: required_string(value, "id", "tool-use id")?,
            name: required_string(value, "name", "tool-use name")?,
        },
        Some("server_tool_use") if value["name"].as_str() == Some("web_search") => {
            ContentBlock::SearchUse {
                id: required_string(value, "id", "server tool-use id")?,
            }
        }
        Some("web_search_tool_result") => ContentBlock::SearchResult {
            tool_use_id: required_string(value, "tool_use_id", "search result tool-use id")?,
        },
        _ => return Err(protocol_failure("content-block semantics")),
    };
    Ok(Event::ContentStart(block))
}

fn parse_content_delta(value: &Value) -> Result<Event, RuntimeFailure> {
    match value["type"].as_str() {
        Some("text_delta") => value["text"]
            .as_str()
            .map(|text| Event::OutputDelta(text.to_owned()))
            .ok_or_else(|| protocol_failure("text delta")),
        Some("input_json_delta") => value["partial_json"]
            .as_str()
            .map(|text| Event::InputJsonDelta(text.to_owned()))
            .ok_or_else(|| protocol_failure("tool input delta")),
        Some("signature_delta") => value["signature"]
            .as_str()
            .filter(|value| !value.is_empty())
            .map(RedactedBytes::from_str)
            .map(Event::SignatureDelta)
            .ok_or_else(|| protocol_failure("thinking signature")),
        Some("thinking_delta") => Err(protocol_failure("omitted thinking delta")),
        Some("citations_delta") => Ok(Event::Citation),
        _ => Err(protocol_failure("content delta semantics")),
    }
}

fn required_string(
    value: &Value,
    field: &str,
    subject: &str,
) -> Result<String, RuntimeFailure> {
    value[field]
        .as_str()
        .filter(|value| !value.trim().is_empty())
        .map(str::to_owned)
        .ok_or_else(|| protocol_failure(subject))
}

fn parse_usage(value: &Value) -> TokenUsage {
    TokenUsage::new(
        value["input_tokens"].as_u64(),
        value["output_tokens"].as_u64(),
    )
    .with_cache_tokens(
        value["cache_read_input_tokens"].as_u64(),
        value["cache_creation_input_tokens"].as_u64(),
    )
}

pub(crate) fn classify_error(kind: Option<&str>) -> ProviderErrorKind {
    match kind {
        Some("authentication_error") => ProviderErrorKind::Authentication,
        Some("billing_error") => ProviderErrorKind::Billing,
        Some("permission_error") => ProviderErrorKind::Permission,
        Some("rate_limit_error") => ProviderErrorKind::RateLimited,
        Some("overloaded_error") => ProviderErrorKind::Overloaded,
        Some("invalid_request_error") => ProviderErrorKind::InvalidRequest,
        _ => ProviderErrorKind::Other,
    }
}

pub(crate) fn provider_failure(kind: ProviderErrorKind, operation: &str) -> RuntimeFailure {
    let (code, label) = match kind {
        ProviderErrorKind::Authentication => (
            "swallowtail.anthropic.authentication_rejected",
            "authentication was rejected",
        ),
        ProviderErrorKind::Billing => (
            "swallowtail.anthropic.billing_unavailable",
            "billing was unavailable",
        ),
        ProviderErrorKind::Permission => (
            "swallowtail.anthropic.permission_denied",
            "permission was denied",
        ),
        ProviderErrorKind::RateLimited => (
            "swallowtail.anthropic.rate_limited",
            "was rate limited",
        ),
        ProviderErrorKind::Overloaded => {
            ("swallowtail.anthropic.overloaded", "was overloaded")
        }
        ProviderErrorKind::InvalidRequest => (
            "swallowtail.anthropic.invalid_request",
            "rejected the request",
        ),
        ProviderErrorKind::Other => ("swallowtail.anthropic.provider_failed", "failed"),
    };
    failure(code, format!("Anthropic {operation} {label}"))
        .with_failure_classification(classification(kind))
}

const fn classification(kind: ProviderErrorKind) -> FailureClassification {
    let (failure_kind, recovery) = match kind {
        ProviderErrorKind::Authentication => (
            FailureKind::AuthenticationRejected,
            FailureRecovery::ReauthenticationRequired,
        ),
        ProviderErrorKind::Billing => (
            FailureKind::EntitlementUnavailable,
            FailureRecovery::SameRequestNotRetryable,
        ),
        ProviderErrorKind::Permission => (
            FailureKind::AuthorizationDenied,
            FailureRecovery::ConfigurationChangeRequired,
        ),
        ProviderErrorKind::RateLimited => (
            FailureKind::RateLimited,
            FailureRecovery::RetryMaySucceed,
        ),
        ProviderErrorKind::Overloaded => (
            FailureKind::ProviderUnavailable,
            FailureRecovery::RetryMaySucceed,
        ),
        ProviderErrorKind::InvalidRequest => (
            FailureKind::InvalidRequest,
            FailureRecovery::InputChangeRequired,
        ),
        ProviderErrorKind::Other => (FailureKind::Unknown, FailureRecovery::Unknown),
    };
    FailureClassification::new(FailureOrigin::Provider, failure_kind, recovery)
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct SseFrame {
    pub name: String,
    pub data: RedactedBytes,
}

impl std::fmt::Debug for SseFrame {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SseFrame")
            .field("name", &self.name)
            .field("data", &self.data)
            .finish()
    }
}

#[derive(Default)]
pub(crate) struct SseDecoder {
    buffer: Vec<u8>,
}

impl Drop for SseDecoder {
    fn drop(&mut self) {
        self.buffer.fill(0);
    }
}

impl SseDecoder {
    pub(crate) fn push(&mut self, chunk: &[u8]) -> Result<Vec<SseFrame>, RuntimeFailure> {
        if self.buffer.len().saturating_add(chunk.len()) > 1_048_576 {
            return Err(failure(
                "swallowtail.anthropic.sse_limit",
                "Anthropic SSE event exceeded its input limit",
            ));
        }
        self.buffer.extend_from_slice(chunk);
        let mut frames = Vec::new();
        while let Some(end) = boundary(&self.buffer) {
            let frame: Vec<_> = self.buffer.drain(..end).collect();
            let separator = if self.buffer.starts_with(b"\r\n\r\n") {
                4
            } else {
                2
            };
            self.buffer.drain(..separator);
            let decoded = decode_frame(&frame)?;
            let mut raw = frame;
            raw.fill(0);
            frames.push(decoded);
        }
        Ok(frames)
    }

    pub(crate) fn finish(mut self) -> Result<(), RuntimeFailure> {
        let disconnected = !self.buffer.iter().all(u8::is_ascii_whitespace);
        self.buffer.fill(0);
        if disconnected {
            Err(failure(
                "swallowtail.anthropic.sse_disconnected",
                "Anthropic SSE disconnected during an event",
            ))
        } else {
            Ok(())
        }
    }

    #[cfg(test)]
    fn leftover_is_zeroed(&self) -> bool {
        self.buffer.iter().all(|&byte| byte == 0)
    }

    #[cfg(test)]
    fn zeroize_leftover(&mut self) {
        self.buffer.fill(0);
    }
}

fn boundary(buffer: &[u8]) -> Option<usize> {
    buffer
        .windows(2)
        .position(|value| value == b"\n\n")
        .or_else(|| {
            buffer
                .windows(4)
                .position(|value| value == b"\r\n\r\n")
        })
}

fn decode_frame(frame: &[u8]) -> Result<SseFrame, RuntimeFailure> {
    let text = std::str::from_utf8(frame).map_err(|_| protocol_failure("SSE encoding"))?;
    let mut name = None;
    let mut data = Vec::new();
    for line in text.lines() {
        if let Some(value) = line.strip_prefix("event:") {
            name = Some(value.trim().to_owned());
        } else if let Some(value) = line.strip_prefix("data:") {
            if !data.is_empty() {
                data.push(b'\n');
            }
            data.extend_from_slice(value.trim_start().as_bytes());
        }
    }
    Ok(SseFrame {
        name: name.ok_or_else(|| protocol_failure("SSE event name"))?,
        data: RedactedBytes(data),
    })
}
