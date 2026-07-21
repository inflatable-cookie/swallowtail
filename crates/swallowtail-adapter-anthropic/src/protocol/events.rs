use serde_json::Value;
use swallowtail_runtime::TokenUsage;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum Event {
    MessageStart(TokenUsage),
    ContentStart,
    OutputDelta(String),
    ContentStop,
    Usage(TokenUsage),
    MessageStop,
    Ping,
    ProviderFailed(ProviderErrorKind),
    Unknown,
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
    let value: Value = parse_json(&frame.data, "stream event")?;
    if value.get("type").and_then(Value::as_str) != Some(frame.name.as_str()) {
        return Err(protocol_failure("stream event type"));
    }
    match frame.name.as_str() {
        "message_start" => Ok(Event::MessageStart(parse_usage(&value["message"]["usage"]))),
        "content_block_start" => {
            require_kind(&value["content_block"], "text")?;
            Ok(Event::ContentStart)
        }
        "content_block_delta" => {
            require_kind(&value["delta"], "text_delta")?;
            value["delta"]["text"]
                .as_str()
                .map(|text| Event::OutputDelta(text.to_owned()))
                .ok_or_else(|| protocol_failure("text delta"))
        }
        "content_block_stop" => Ok(Event::ContentStop),
        "message_delta" => Ok(Event::Usage(parse_usage(&value["usage"]))),
        "message_stop" => Ok(Event::MessageStop),
        "ping" => Ok(Event::Ping),
        "error" => Ok(Event::ProviderFailed(classify_error(
            value["error"]["type"].as_str(),
        ))),
        _ => Ok(Event::Unknown),
    }
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
}

fn require_kind(value: &Value, expected: &str) -> Result<(), RuntimeFailure> {
    if value["type"].as_str() == Some(expected) {
        Ok(())
    } else {
        Err(failure(
            "swallowtail.anthropic.content_semantics_unknown",
            "Anthropic emitted unsupported content semantics",
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SseFrame {
    pub name: String,
    pub data: Vec<u8>,
}

#[derive(Default)]
pub(crate) struct SseDecoder {
    buffer: Vec<u8>,
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
            frames.push(decode_frame(&frame)?);
        }
        Ok(frames)
    }

    pub(crate) fn finish(self) -> Result<(), RuntimeFailure> {
        if self.buffer.iter().all(u8::is_ascii_whitespace) {
            Ok(())
        } else {
            Err(failure(
                "swallowtail.anthropic.sse_disconnected",
                "Anthropic SSE disconnected during an event",
            ))
        }
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
        data,
    })
}
