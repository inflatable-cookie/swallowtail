#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Event {
    Connected,
    Busy,
    OutputDelta(String),
    OutputSnapshot(String),
    Usage(String, TokenUsage),
    Idle,
    Cancelled,
    ProviderFailed,
    ProviderRequest(PendingProviderRequest),
    Foreign,
}

pub(crate) fn parse_event(data: &[u8], session_id: &str) -> Result<Event, RuntimeFailure> {
    let envelope: Value = parse_json(data, "event")?;
    let kind = envelope
        .get("type")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            failure(
                "swallowtail.opencode.event_invalid",
                "OpenCode event omitted its type",
            )
        })?;
    let properties = envelope
        .get("properties")
        .and_then(Value::as_object)
        .ok_or_else(|| {
            failure(
                "swallowtail.opencode.event_invalid",
                "OpenCode event omitted properties",
            )
        })?;
    if kind == "server.connected" {
        return Ok(Event::Connected);
    }
    if !matches!(
        kind,
        "session.status"
            | "session.idle"
            | "message.part.delta"
            | "message.part.updated"
            | "session.error"
            | "permission.asked"
            | "question.asked"
    ) {
        return Err(failure(
            "swallowtail.opencode.event_unknown",
            "OpenCode emitted an unsupported event type",
        ));
    }
    if properties.get("sessionID").and_then(Value::as_str) != Some(session_id) {
        return Ok(Event::Foreign);
    }
    match kind {
        "session.status" => parse_status(properties),
        "session.idle" => Ok(Event::Idle),
        "message.part.delta" => parse_delta(properties),
        "message.part.updated" => parse_part(properties),
        "session.error" => parse_error(properties),
        "permission.asked" => parse_permission(properties),
        "question.asked" => parse_question(properties),
        _ => unreachable!("event kind was checked before correlation"),
    }
}

fn parse_permission(properties: &Map<String, Value>) -> Result<Event, RuntimeFailure> {
    let id = provider_request_id(properties)?;
    let permission = properties
        .get("permission")
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty() && value.len() <= 4096)
        .ok_or_else(provider_request_invalid)?;
    let patterns = bounded_string_array(properties.get("patterns"))?;
    let payload = serde_json::to_vec(&json!({
        "permission": permission,
        "patterns": patterns,
        "replies": ["once", "reject"]
    }))
    .map_err(|_| provider_request_invalid())?;
    Ok(Event::ProviderRequest(PendingProviderRequest {
        id,
        kind: ProviderRequestKind::Permission,
        payload,
    }))
}

fn parse_question(properties: &Map<String, Value>) -> Result<Event, RuntimeFailure> {
    let id = provider_request_id(properties)?;
    let questions = properties
        .get("questions")
        .and_then(Value::as_array)
        .filter(|questions| !questions.is_empty() && questions.len() <= 32)
        .ok_or_else(provider_request_invalid)?;
    for question in questions {
        let object = question.as_object().ok_or_else(provider_request_invalid)?;
        for field in ["question", "header"] {
            if object
                .get(field)
                .and_then(Value::as_str)
                .is_none_or(|value| value.is_empty() || value.len() > 4096)
            {
                return Err(provider_request_invalid());
            }
        }
        let options = object
            .get("options")
            .and_then(Value::as_array)
            .filter(|options| !options.is_empty() && options.len() <= 32)
            .ok_or_else(provider_request_invalid)?;
        for option in options {
            let option = option.as_object().ok_or_else(provider_request_invalid)?;
            for field in ["label", "description"] {
                if option
                    .get(field)
                    .and_then(Value::as_str)
                    .is_none_or(|value| value.is_empty() || value.len() > 4096)
                {
                    return Err(provider_request_invalid());
                }
            }
        }
        if !matches!(object.get("multiple"), Some(Value::Bool(_))) {
            return Err(provider_request_invalid());
        }
    }
    let payload = serde_json::to_vec(&json!({"questions": questions}))
        .map_err(|_| provider_request_invalid())?;
    Ok(Event::ProviderRequest(PendingProviderRequest {
        id,
        kind: ProviderRequestKind::Question {
            count: questions.len(),
        },
        payload,
    }))
}

fn provider_request_id(properties: &Map<String, Value>) -> Result<String, RuntimeFailure> {
    properties
        .get("id")
        .and_then(Value::as_str)
        .filter(|id| {
            !id.is_empty()
                && id.len() <= 1024
                && id.bytes().all(|byte| {
                    byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.' | b'~')
                })
        })
        .map(str::to_owned)
        .ok_or_else(provider_request_invalid)
}

fn bounded_string_array(value: Option<&Value>) -> Result<Vec<&str>, RuntimeFailure> {
    let values = value
        .and_then(Value::as_array)
        .filter(|values| values.len() <= 32)
        .ok_or_else(provider_request_invalid)?;
    values
        .iter()
        .map(|value| {
            value
                .as_str()
                .filter(|value| !value.is_empty() && value.len() <= 4096)
                .ok_or_else(provider_request_invalid)
        })
        .collect()
}

fn provider_request_invalid() -> RuntimeFailure {
    failure(
        "swallowtail.opencode.provider_request_invalid",
        "OpenCode provider request was malformed",
    )
}

fn parse_status(properties: &Map<String, Value>) -> Result<Event, RuntimeFailure> {
    match properties
        .get("status")
        .and_then(Value::as_object)
        .and_then(|status| status.get("type"))
        .and_then(Value::as_str)
    {
        Some("busy") => Ok(Event::Busy),
        Some("idle") => Ok(Event::Idle),
        _ => Err(failure(
            "swallowtail.opencode.event_invalid",
            "OpenCode emitted an unsupported session status",
        )),
    }
}

fn parse_delta(properties: &Map<String, Value>) -> Result<Event, RuntimeFailure> {
    if properties.get("field").and_then(Value::as_str) != Some("text") {
        return Err(failure(
            "swallowtail.opencode.event_unknown",
            "OpenCode emitted an unsupported message delta",
        ));
    }
    properties
        .get("delta")
        .and_then(Value::as_str)
        .map(|delta| Event::OutputDelta(delta.to_owned()))
        .ok_or_else(|| {
            failure(
                "swallowtail.opencode.event_invalid",
                "OpenCode text delta was invalid",
            )
        })
}

fn parse_part(properties: &Map<String, Value>) -> Result<Event, RuntimeFailure> {
    let part = properties
        .get("part")
        .and_then(Value::as_object)
        .ok_or_else(|| {
            failure(
                "swallowtail.opencode.event_invalid",
                "OpenCode message part was invalid",
            )
        })?;
    match part.get("type").and_then(Value::as_str) {
        Some("text") => part
            .get("text")
            .and_then(Value::as_str)
            .map(|text| Event::OutputSnapshot(text.to_owned()))
            .ok_or_else(|| {
                failure(
                    "swallowtail.opencode.event_invalid",
                    "OpenCode text part was invalid",
                )
            }),
        Some("step-finish") => {
            let part_id = part
                .get("id")
                .and_then(Value::as_str)
                .filter(|value| !value.is_empty())
                .ok_or_else(invalid_usage)?;
            parse_usage(part).map(|usage| Event::Usage(part_id.to_owned(), usage))
        }
        _ => Err(failure(
            "swallowtail.opencode.event_unknown",
            "OpenCode emitted an unsupported message part",
        )),
    }
}

fn parse_usage(part: &Map<String, Value>) -> Result<TokenUsage, RuntimeFailure> {
    let tokens = part
        .get("tokens")
        .and_then(Value::as_object)
        .ok_or_else(invalid_usage)?;
    let cache = tokens
        .get("cache")
        .and_then(Value::as_object)
        .ok_or_else(invalid_usage)?;
    Ok(TokenUsage::new(
        required_usage(tokens, "input")?,
        required_usage(tokens, "output")?,
    )
    .with_reasoning_tokens(required_usage(tokens, "reasoning")?)
    .with_cache_tokens(
        required_usage(cache, "read")?,
        required_usage(cache, "write")?,
    ))
}

fn required_usage(
    value: &Map<String, Value>,
    field: &str,
) -> Result<Option<u64>, RuntimeFailure> {
    value
        .get(field)
        .and_then(Value::as_u64)
        .map(Some)
        .ok_or_else(invalid_usage)
}

fn invalid_usage() -> RuntimeFailure {
    failure(
        "swallowtail.opencode.event_invalid",
        "OpenCode token usage was invalid",
    )
}

fn parse_error(properties: &Map<String, Value>) -> Result<Event, RuntimeFailure> {
    match properties
        .get("error")
        .and_then(Value::as_object)
        .and_then(|error| error.get("name"))
        .and_then(Value::as_str)
    {
        Some("MessageAbortedError") => Ok(Event::Cancelled),
        Some(_) => Ok(Event::ProviderFailed),
        None => Err(failure(
            "swallowtail.opencode.event_invalid",
            "OpenCode session error was invalid",
        )),
    }
}

#[derive(Default)]
pub(crate) struct SseDecoder {
    buffer: Vec<u8>,
}

impl SseDecoder {
    pub(crate) fn push(&mut self, chunk: &[u8]) -> Result<Vec<Vec<u8>>, RuntimeFailure> {
        if self.buffer.len().saturating_add(chunk.len()) > 1_048_576 {
            return Err(failure(
                "swallowtail.opencode.sse_limit",
                "OpenCode SSE event exceeded the bounded input limit",
            ));
        }
        self.buffer.extend_from_slice(chunk);
        let mut events = Vec::new();
        while let Some(end) = find_boundary(&self.buffer) {
            let frame: Vec<u8> = self.buffer.drain(..end).collect();
            self.buffer.drain(..boundary_len(&self.buffer));
            if let Some(data) = frame_data(&frame)? {
                events.push(data);
            }
        }
        Ok(events)
    }

    pub(crate) fn finish(self) -> Result<(), RuntimeFailure> {
        if self.buffer.iter().all(u8::is_ascii_whitespace) {
            Ok(())
        } else {
            Err(failure(
                "swallowtail.opencode.sse_disconnected",
                "OpenCode SSE stream disconnected during an event",
            ))
        }
    }
}

fn find_boundary(buffer: &[u8]) -> Option<usize> {
    buffer
        .windows(2)
        .position(|window| window == b"\n\n")
        .or_else(|| buffer.windows(4).position(|window| window == b"\r\n\r\n"))
}

fn boundary_len(buffer: &[u8]) -> usize {
    if buffer.starts_with(b"\r\n\r\n") {
        4
    } else {
        2
    }
}

fn frame_data(frame: &[u8]) -> Result<Option<Vec<u8>>, RuntimeFailure> {
    let text = std::str::from_utf8(frame).map_err(|_| {
        failure(
            "swallowtail.opencode.sse_invalid",
            "OpenCode SSE stream was not valid UTF-8",
        )
    })?;
    let mut data = Vec::new();
    for line in text.lines() {
        if line.starts_with(':') || line.is_empty() {
            continue;
        }
        let Some(value) = line.strip_prefix("data:") else {
            continue;
        };
        if !data.is_empty() {
            data.push(b'\n');
        }
        data.extend_from_slice(value.strip_prefix(' ').unwrap_or(value).as_bytes());
    }
    Ok((!data.is_empty()).then_some(data))
}
