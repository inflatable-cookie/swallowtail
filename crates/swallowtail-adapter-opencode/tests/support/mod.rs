use serde_json::Value;

pub const MAX_FIXTURE_STREAM_BYTES: usize = 64 * 1024;
pub const MAX_FIXTURE_HTTP_BYTES: usize = 128 * 1024;
const MAX_FIXTURE_FRAME_BYTES: usize = 16 * 1024;
const MAX_FIXTURE_FRAMES: usize = 64;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureError {
    StreamTooLarge,
    HttpTranscriptTooLarge,
    FrameTooLarge,
    TooManyFrames,
    IncompleteFrame,
    InvalidJson,
}

pub fn parse_http_json(input: &str) -> Result<Value, FixtureError> {
    if input.len() > MAX_FIXTURE_HTTP_BYTES {
        return Err(FixtureError::HttpTranscriptTooLarge);
    }
    serde_json::from_str(input).map_err(|_| FixtureError::InvalidJson)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EventDecision {
    Continue,
    Completed,
    Aborted,
    ProviderFailed,
    StopAndAbort,
    ProtocolFailed,
}

pub fn parse_sse(input: &str) -> Result<Vec<Value>, FixtureError> {
    if input.len() > MAX_FIXTURE_STREAM_BYTES {
        return Err(FixtureError::StreamTooLarge);
    }

    let mut frames = Vec::new();
    let mut data = Vec::new();
    let mut frame_bytes = 0;
    for line in input.lines() {
        let line = line.strip_suffix('\r').unwrap_or(line);
        if line.is_empty() {
            if data.is_empty() {
                continue;
            }
            frames.push(parse_frame(&data.join("\n"))?);
            if frames.len() > MAX_FIXTURE_FRAMES {
                return Err(FixtureError::TooManyFrames);
            }
            data.clear();
            frame_bytes = 0;
            continue;
        }
        if line.starts_with(':') {
            continue;
        }
        if let Some(value) = line.strip_prefix("data:") {
            let value = value.strip_prefix(' ').unwrap_or(value);
            frame_bytes += value.len();
            if frame_bytes > MAX_FIXTURE_FRAME_BYTES {
                return Err(FixtureError::FrameTooLarge);
            }
            data.push(value);
        }
    }

    if data.is_empty() {
        Ok(frames)
    } else {
        Err(FixtureError::IncompleteFrame)
    }
}

fn parse_frame(data: &str) -> Result<Value, FixtureError> {
    serde_json::from_str(data).map_err(|_| FixtureError::InvalidJson)
}

pub fn event_type(event: &Value) -> &str {
    event["type"].as_str().expect("fixture event has a type")
}

pub fn session_id(event: &Value) -> Option<&str> {
    event["properties"]["sessionID"].as_str()
}

pub fn decide(event: &Value) -> EventDecision {
    match event_type(event) {
        "server.connected" | "message.part.delta" | "message.part.updated" => {
            EventDecision::Continue
        }
        "session.status" if event["properties"]["status"]["type"] == "idle" => {
            EventDecision::Completed
        }
        "session.status" => EventDecision::Continue,
        "session.idle" => EventDecision::Completed,
        "session.error" if event["properties"]["error"]["name"] == "MessageAbortedError" => {
            EventDecision::Aborted
        }
        "session.error" => EventDecision::ProviderFailed,
        "permission.asked" | "question.asked" => EventDecision::StopAndAbort,
        _ => EventDecision::ProtocolFailed,
    }
}
