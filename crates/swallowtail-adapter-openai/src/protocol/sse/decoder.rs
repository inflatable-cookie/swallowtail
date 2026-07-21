const MAX_EVENT_BYTES: usize = 1_048_576;

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
        if self.buffer.len().saturating_add(chunk.len()) > MAX_EVENT_BYTES {
            return Err(failure(
                "swallowtail.openai.sse_limit",
                "OpenAI Responses SSE event exceeded its input limit",
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
            if let Some(frame) = decode_frame(&frame)? {
                frames.push(frame);
            }
        }
        Ok(frames)
    }

    pub(crate) fn finish(self) -> Result<(), RuntimeFailure> {
        if self.buffer.iter().all(u8::is_ascii_whitespace) {
            Ok(())
        } else {
            Err(failure(
                "swallowtail.openai.sse_disconnected",
                "OpenAI Responses SSE disconnected during an event",
            ))
        }
    }
}

fn boundary(buffer: &[u8]) -> Option<usize> {
    buffer
        .windows(2)
        .position(|value| value == b"\n\n")
        .or_else(|| buffer.windows(4).position(|value| value == b"\r\n\r\n"))
}

fn decode_frame(frame: &[u8]) -> Result<Option<SseFrame>, RuntimeFailure> {
    let text = std::str::from_utf8(frame).map_err(|_| malformed())?;
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
        Some(name) => Ok(Some(SseFrame { name, data })),
        None if text
            .lines()
            .all(|line| line.trim().is_empty() || line.starts_with(':')) =>
        {
            Ok(None)
        }
        None => Err(malformed()),
    }
}
