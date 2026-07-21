#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Event {
    Connected,
    Busy,
    OutputDelta(String),
    OutputSnapshot(String),
    Idle,
    Cancelled,
    ProviderFailed,
    StopAndAbort,
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
        "permission.asked" | "question.asked" => Ok(Event::StopAndAbort),
        _ => unreachable!("event kind was checked before correlation"),
    }
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
    if part.get("type").and_then(Value::as_str) != Some("text") {
        return Err(failure(
            "swallowtail.opencode.event_unknown",
            "OpenCode emitted an unsupported message part",
        ));
    }
    part.get("text")
        .and_then(Value::as_str)
        .map(|text| Event::OutputSnapshot(text.to_owned()))
        .ok_or_else(|| {
            failure(
                "swallowtail.opencode.event_invalid",
                "OpenCode text part was invalid",
            )
        })
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


