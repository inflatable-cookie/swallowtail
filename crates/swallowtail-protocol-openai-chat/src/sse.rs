use crate::{CodecLimits, ProtocolError, ProtocolErrorKind};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SseRecord {
    Data(Vec<u8>),
    Done,
}

pub struct SseDecoder {
    limits: CodecLimits,
    pending: Vec<u8>,
}

impl SseDecoder {
    #[must_use]
    pub fn new(limits: CodecLimits) -> Self {
        Self {
            limits,
            pending: Vec::new(),
        }
    }

    pub fn push(&mut self, input: &[u8]) -> Result<Vec<SseRecord>, ProtocolError> {
        if self.pending.len().saturating_add(input.len()) > self.limits.maximum_wire_bytes() {
            return Err(ProtocolError::new(ProtocolErrorKind::BufferLimitExceeded));
        }
        self.pending.extend_from_slice(input);
        let mut records = Vec::new();
        while let Some((end, separator_length)) = boundary(&self.pending) {
            if end > self.limits.maximum_wire_bytes() {
                return Err(ProtocolError::new(ProtocolErrorKind::WireLimitExceeded));
            }
            let frame: Vec<_> = self.pending.drain(..end).collect();
            self.pending.drain(..separator_length);
            if let Some(record) = decode_frame(&frame)? {
                records.push(record);
            }
        }
        Ok(records)
    }

    pub fn finish(self) -> Result<(), ProtocolError> {
        if self.pending.iter().all(u8::is_ascii_whitespace) {
            Ok(())
        } else {
            Err(ProtocolError::new(ProtocolErrorKind::IncompleteRecord))
        }
    }
}

impl Default for SseDecoder {
    fn default() -> Self {
        Self::new(CodecLimits::default())
    }
}

fn boundary(buffer: &[u8]) -> Option<(usize, usize)> {
    let line_feed = buffer
        .windows(2)
        .position(|window| window == b"\n\n")
        .map(|position| (position, 2));
    let carriage_return = buffer
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .map(|position| (position, 4));
    match (line_feed, carriage_return) {
        (Some(left), Some(right)) => Some(if left.0 <= right.0 { left } else { right }),
        (Some(found), None) | (None, Some(found)) => Some(found),
        (None, None) => None,
    }
}

fn decode_frame(frame: &[u8]) -> Result<Option<SseRecord>, ProtocolError> {
    let text = std::str::from_utf8(frame)
        .map_err(|_| ProtocolError::new(ProtocolErrorKind::InvalidUtf8))?;
    let mut data = Vec::new();
    let mut observed_data = false;
    for line in text.lines() {
        if line.starts_with(':') {
            continue;
        }
        let Some(value) = line.strip_prefix("data:") else {
            return Err(ProtocolError::new(ProtocolErrorKind::UnsupportedSseField));
        };
        if observed_data {
            data.push(b'\n');
        }
        observed_data = true;
        data.extend_from_slice(value.trim_start().as_bytes());
    }
    if !observed_data {
        return Ok(None);
    }
    if data.is_empty() {
        return Err(ProtocolError::new(ProtocolErrorKind::InvalidStructure));
    }
    if data == b"[DONE]" {
        Ok(Some(SseRecord::Done))
    } else {
        Ok(Some(SseRecord::Data(data)))
    }
}
