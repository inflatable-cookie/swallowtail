const MAX_EVENT_BYTES: usize = 1_048_576;

#[derive(Clone, Eq, PartialEq)]
pub struct SseFrame {
    name: String,
    data: Vec<u8>,
}

impl SseFrame {
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub(crate) fn data(&self) -> &[u8] {
        &self.data
    }
}

impl std::fmt::Debug for SseFrame {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SseFrame")
            .field("name", &self.name)
            .field("data", &"<redacted>")
            .finish()
    }
}

#[derive(Default)]
pub struct SseDecoder {
    buffer: Vec<u8>,
}

impl SseDecoder {
    pub fn push(&mut self, chunk: &[u8]) -> Result<Vec<SseFrame>, AlibabaProtocolFailure> {
        if self.buffer.len().saturating_add(chunk.len()) > MAX_EVENT_BYTES {
            return Err(AlibabaProtocolFailure::invalid("SSE event bound"));
        }
        self.buffer.extend_from_slice(chunk);
        let mut frames = Vec::new();
        while let Some((end, separator)) = boundary(&self.buffer) {
            let frame: Vec<_> = self.buffer.drain(..end).collect();
            self.buffer.drain(..separator);
            if let Some(frame) = decode_frame(&frame)? {
                frames.push(frame);
            }
        }
        Ok(frames)
    }

    pub fn finish(self) -> Result<(), AlibabaProtocolFailure> {
        if self.buffer.iter().all(u8::is_ascii_whitespace) {
            Ok(())
        } else {
            Err(AlibabaProtocolFailure::invalid("disconnected SSE event"))
        }
    }
}

fn boundary(buffer: &[u8]) -> Option<(usize, usize)> {
    let lf = buffer.windows(2).position(|value| value == b"\n\n").map(|end| (end, 2));
    let crlf = buffer
        .windows(4)
        .position(|value| value == b"\r\n\r\n")
        .map(|end| (end, 4));
    match (lf, crlf) {
        (Some(left), Some(right)) => Some(if left.0 < right.0 { left } else { right }),
        (Some(value), None) | (None, Some(value)) => Some(value),
        (None, None) => None,
    }
}

fn decode_frame(frame: &[u8]) -> Result<Option<SseFrame>, AlibabaProtocolFailure> {
    let text = std::str::from_utf8(frame)
        .map_err(|_| AlibabaProtocolFailure::invalid("SSE UTF-8"))?;
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
    match name {
        Some(name) if !name.is_empty() && !data.is_empty() => Ok(Some(SseFrame { name, data })),
        None if text.lines().all(|line| line.trim().is_empty() || line.starts_with(':')) => Ok(None),
        _ => Err(AlibabaProtocolFailure::invalid("SSE frame")),
    }
}
