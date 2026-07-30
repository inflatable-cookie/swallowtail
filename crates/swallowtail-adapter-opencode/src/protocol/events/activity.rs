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
    let message_id = required_identity(properties, "messageID")?;
    let part_id = required_identity(properties, "partID")?;
    properties
        .get("delta")
        .and_then(Value::as_str)
        .map(|delta| Event::OutputDelta {
            message_id,
            part_id,
            text: delta.to_owned(),
        })
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
        Some("text") => {
            let message_id = required_identity(part, "messageID")?;
            let part_id = required_identity(part, "id")?;
            part.get("text")
                .and_then(Value::as_str)
                .map(|text| Event::OutputSnapshot {
                    message_id,
                    part_id,
                    text: text.to_owned(),
                })
                .ok_or_else(|| {
                    failure(
                        "swallowtail.opencode.event_invalid",
                        "OpenCode text part was invalid",
                    )
                })
        }
        Some("reasoning") => {
            let message_id = required_identity(part, "messageID")?;
            let part_id = required_identity(part, "id")?;
            let text = properties
                .get("delta")
                .or_else(|| part.get("text"))
                .and_then(Value::as_str)
                .ok_or_else(|| {
                    failure(
                        "swallowtail.opencode.event_invalid",
                        "OpenCode reasoning part was invalid",
                    )
                })?;
            Ok(Event::ReasoningSnapshot {
                message_id,
                part_id,
                text: text.to_owned(),
            })
        }
        Some("tool") => {
            let part_id = required_identity(part, "id")?;
            let call_id = required_identity(part, "callID")?;
            let name = required_identity(part, "tool")?;
            let status = part
                .get("state")
                .and_then(Value::as_object)
                .and_then(|state| state.get("status"))
                .and_then(Value::as_str)
                .and_then(|status| match status {
                    "pending" => Some(ToolStatus::Pending),
                    "running" => Some(ToolStatus::Running),
                    "completed" => Some(ToolStatus::Completed),
                    "error" => Some(ToolStatus::Failed),
                    _ => None,
                })
                .ok_or_else(|| {
                    failure(
                        "swallowtail.opencode.event_invalid",
                        "OpenCode tool state was invalid",
                    )
                })?;
            Ok(Event::ToolState {
                part_id,
                call_id,
                name,
                status,
            })
        }
        Some("step-finish") => {
            let part_id = part
                .get("id")
                .and_then(Value::as_str)
                .filter(|value| !value.is_empty())
                .ok_or_else(invalid_usage)?;
            parse_usage(part).map(|usage| Event::Usage(part_id.to_owned(), usage))
        }
        Some(kind) => {
            required_identity(part, "id")?;
            Ok(Event::Unknown(format!("message.part.updated.{kind}")))
        }
        None => Err(failure(
            "swallowtail.opencode.event_unknown",
            "OpenCode emitted an unsupported message part",
        )),
    }
}

fn required_identity(value: &Map<String, Value>, field: &str) -> Result<String, RuntimeFailure> {
    value
        .get(field)
        .and_then(Value::as_str)
        .filter(|value| {
            !value.is_empty()
                && value.len() <= 1024
                && value.bytes().all(|byte| {
                    byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.' | b'~')
                })
        })
        .map(str::to_owned)
        .ok_or_else(|| {
            failure(
                "swallowtail.opencode.event_invalid",
                "OpenCode event identity was invalid",
            )
        })
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
