use serde_json::Value;

pub const MAX_FIXTURE_HTTP_BYTES: usize = 256 * 1024;
pub const MAX_FIXTURE_STREAM_BYTES: usize = 256 * 1024;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FixtureParseError {
    HttpTooLarge,
    StreamTooLarge,
    IncompleteFrame,
    MissingEvent,
    MissingData,
    InvalidJson,
    EventTypeMismatch,
    UnknownContentSemantics,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FixtureEventKind {
    MessageStart,
    ContentBlockStart,
    TextDelta(String),
    ContentBlockStop,
    Usage,
    MessageStop,
    Ping,
    ProviderError(String),
    UnknownTopLevel(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureEvent {
    name: String,
    data: Value,
    kind: FixtureEventKind,
}

impl FixtureEvent {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub const fn data(&self) -> &Value {
        &self.data
    }

    pub const fn kind(&self) -> &FixtureEventKind {
        &self.kind
    }
}

pub fn parse_http_json(input: &str) -> Result<Value, FixtureParseError> {
    if input.len() > MAX_FIXTURE_HTTP_BYTES {
        return Err(FixtureParseError::HttpTooLarge);
    }
    serde_json::from_str(input).map_err(|_| FixtureParseError::InvalidJson)
}

pub fn parse_sse(input: &str) -> Result<Vec<FixtureEvent>, FixtureParseError> {
    if input.len() > MAX_FIXTURE_STREAM_BYTES {
        return Err(FixtureParseError::StreamTooLarge);
    }
    if !input.ends_with("\n\n") {
        return Err(FixtureParseError::IncompleteFrame);
    }

    input
        .trim_end_matches('\n')
        .split("\n\n")
        .map(parse_frame)
        .collect()
}

fn parse_frame(frame: &str) -> Result<FixtureEvent, FixtureParseError> {
    let mut event = None;
    let mut data_lines = Vec::new();
    for line in frame.lines() {
        if line.starts_with(':') {
            continue;
        }
        if let Some(value) = line.strip_prefix("event:") {
            event = Some(value.trim().to_owned());
        } else if let Some(value) = line.strip_prefix("data:") {
            data_lines.push(value.trim_start());
        }
    }
    let name = event.ok_or(FixtureParseError::MissingEvent)?;
    if data_lines.is_empty() {
        return Err(FixtureParseError::MissingData);
    }
    let data: Value =
        serde_json::from_str(&data_lines.join("\n")).map_err(|_| FixtureParseError::InvalidJson)?;
    if data.get("type").and_then(Value::as_str) != Some(name.as_str()) {
        return Err(FixtureParseError::EventTypeMismatch);
    }
    let kind = classify(&name, &data)?;
    Ok(FixtureEvent { name, data, kind })
}

fn classify(name: &str, data: &Value) -> Result<FixtureEventKind, FixtureParseError> {
    match name {
        "message_start" => Ok(FixtureEventKind::MessageStart),
        "content_block_start" => {
            require_text_block(data)?;
            Ok(FixtureEventKind::ContentBlockStart)
        }
        "content_block_delta" => text_delta(data).map(FixtureEventKind::TextDelta),
        "content_block_stop" => Ok(FixtureEventKind::ContentBlockStop),
        "message_delta" => Ok(FixtureEventKind::Usage),
        "message_stop" => Ok(FixtureEventKind::MessageStop),
        "ping" => Ok(FixtureEventKind::Ping),
        "error" => Ok(FixtureEventKind::ProviderError(
            data.pointer("/error/type")
                .and_then(Value::as_str)
                .unwrap_or("unknown_error")
                .to_owned(),
        )),
        unknown => Ok(FixtureEventKind::UnknownTopLevel(unknown.to_owned())),
    }
}

fn require_text_block(data: &Value) -> Result<(), FixtureParseError> {
    if data.pointer("/content_block/type").and_then(Value::as_str) == Some("text") {
        Ok(())
    } else {
        Err(FixtureParseError::UnknownContentSemantics)
    }
}

fn text_delta(data: &Value) -> Result<String, FixtureParseError> {
    if data.pointer("/delta/type").and_then(Value::as_str) != Some("text_delta") {
        return Err(FixtureParseError::UnknownContentSemantics);
    }
    data.pointer("/delta/text")
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or(FixtureParseError::UnknownContentSemantics)
}
