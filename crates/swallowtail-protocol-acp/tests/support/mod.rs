use serde_json::Value;

pub const MAX_FRAME_BYTES: usize = 64 * 1024;
pub const MAX_TRANSCRIPT_BYTES: usize = 256 * 1024;
pub const MAX_TRANSCRIPT_FRAMES: usize = 64;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    ClientToAgent,
    AgentToClient,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Frame {
    direction: Direction,
    message: Value,
}

impl Frame {
    pub const fn direction(&self) -> Direction {
        self.direction
    }

    pub const fn message(&self) -> &Value {
        &self.message
    }

    pub fn method(&self) -> Option<&str> {
        self.message.get("method").and_then(Value::as_str)
    }

    pub fn id(&self) -> Option<&Value> {
        self.message.get("id")
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParseError {
    TranscriptTooLarge,
    FrameTooLarge,
    TooManyFrames,
    IncompleteFrame,
    InvalidJson,
    InvalidTranscriptDirection,
    InvalidJsonRpcVersion,
    InvalidMessageShape,
}

pub fn parse_transcript(input: &str) -> Result<Vec<Frame>, ParseError> {
    if input.len() > MAX_TRANSCRIPT_BYTES {
        return Err(ParseError::TranscriptTooLarge);
    }
    if !input.is_empty() && !input.ends_with('\n') {
        return Err(ParseError::IncompleteFrame);
    }

    let mut frames = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            return Err(ParseError::InvalidMessageShape);
        }
        if line.len() > MAX_FRAME_BYTES {
            return Err(ParseError::FrameTooLarge);
        }
        if frames.len() == MAX_TRANSCRIPT_FRAMES {
            return Err(ParseError::TooManyFrames);
        }
        let record: Value = match serde_json::from_str(line) {
            Ok(record) => record,
            Err(error) if error.is_eof() => return Err(ParseError::IncompleteFrame),
            Err(_) => return Err(ParseError::InvalidJson),
        };
        let direction = match record.get("direction").and_then(Value::as_str) {
            Some("client_to_agent") => Direction::ClientToAgent,
            Some("agent_to_client") => Direction::AgentToClient,
            _ => return Err(ParseError::InvalidTranscriptDirection),
        };
        let message = record
            .get("message")
            .cloned()
            .ok_or(ParseError::InvalidMessageShape)?;
        if message.get("jsonrpc").and_then(Value::as_str) != Some("2.0") {
            return Err(ParseError::InvalidJsonRpcVersion);
        }
        if message.get("method").is_none()
            && message.get("result").is_none()
            && message.get("error").is_none()
        {
            return Err(ParseError::InvalidMessageShape);
        }
        frames.push(Frame { direction, message });
    }
    Ok(frames)
}

pub fn parse_json(input: &str) -> Value {
    assert!(
        input.len() <= MAX_TRANSCRIPT_BYTES,
        "fixture JSON is bounded"
    );
    serde_json::from_str(input).expect("fixture JSON is valid")
}

pub fn methods(frames: &[Frame]) -> Vec<&str> {
    frames.iter().filter_map(Frame::method).collect()
}
