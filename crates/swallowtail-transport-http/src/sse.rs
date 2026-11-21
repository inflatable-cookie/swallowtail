use crate::BodyDecoder;
use std::fmt;
use zeroize::Zeroize;

/// Default bound on one buffered, undecoded event.
pub const DEFAULT_EVENT_LIMIT: usize = 1_048_576;

/// Why an event stream could not be framed or decoded.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SseDecodeErrorKind {
    /// One framed event was not valid UTF-8.
    InvalidUtf8,
    /// A configured byte bound was exceeded.
    LimitExceeded,
    /// The stream ended inside a partially received event.
    IncompleteFrame,
}

/// Failure from shared event-stream framing.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SseDecodeError {
    kind: SseDecodeErrorKind,
}

impl SseDecodeError {
    /// Creates one classified framing failure.
    #[must_use]
    pub const fn new(kind: SseDecodeErrorKind) -> Self {
        Self { kind }
    }

    /// Returns the failure classification.
    #[must_use]
    pub const fn kind(self) -> SseDecodeErrorKind {
        self.kind
    }
}

impl fmt::Display for SseDecodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self.kind {
            SseDecodeErrorKind::InvalidUtf8 => "event stream was not valid UTF-8",
            SseDecodeErrorKind::LimitExceeded => "event stream exceeded its byte bound",
            SseDecodeErrorKind::IncompleteFrame => "event stream ended inside an event",
        })
    }
}

impl std::error::Error for SseDecodeError {}

/// Incremental, bounded framer for `\n\n`-delimited server-sent events.
///
/// Each pushed chunk yields every complete frame, including its blank-line
/// delimiter, normalised to line feeds. The framer retains no secret after
/// [`Drop`]: undecoded bytes are zeroized.
#[derive(Debug)]
pub struct SseFramer {
    pending: Vec<u8>,
    observed: usize,
    event_limit: Option<usize>,
    stream_limit: Option<usize>,
}

impl Default for SseFramer {
    fn default() -> Self {
        Self {
            pending: Vec::new(),
            observed: 0,
            event_limit: Some(DEFAULT_EVENT_LIMIT),
            stream_limit: None,
        }
    }
}

impl Drop for SseFramer {
    fn drop(&mut self) {
        self.pending.zeroize();
    }
}

impl SseFramer {
    /// Creates a framer bounded by [`DEFAULT_EVENT_LIMIT`] per event.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Bounds each event's raw bytes, including its delimiter.
    #[must_use]
    pub const fn with_event_limit(mut self, limit: usize) -> Self {
        self.event_limit = Some(limit);
        self
    }

    /// Removes the per-event bound; the stream bound still applies.
    #[must_use]
    pub const fn without_event_limit(mut self) -> Self {
        self.event_limit = None;
        self
    }

    /// Bounds the total bytes observed over the lifetime of the stream.
    #[must_use]
    pub const fn with_stream_limit(mut self, limit: usize) -> Self {
        self.stream_limit = Some(limit);
        self
    }

    /// Accepts one transport chunk and returns every complete frame.
    ///
    /// # Errors
    ///
    /// Returns [`SseDecodeErrorKind::LimitExceeded`] when either configured
    /// bound would be exceeded.
    pub fn push(&mut self, chunk: &[u8]) -> Result<Vec<Vec<u8>>, SseDecodeError> {
        self.observed = self.observed.saturating_add(chunk.len());
        if self.stream_limit.is_some_and(|limit| self.observed > limit) {
            return Err(SseDecodeError::new(SseDecodeErrorKind::LimitExceeded));
        }
        let mut frames = Vec::new();
        if let Some(limit) = self.event_limit {
            let mut remaining = chunk;
            while !remaining.is_empty() {
                let take = limit
                    .saturating_add(1)
                    .saturating_sub(self.pending.len())
                    .max(1)
                    .min(remaining.len());
                let (piece, rest) = remaining.split_at(take);
                remaining = rest;
                if self.pending.len().saturating_add(piece.len()) > limit
                    && !has_boundary_across(&self.pending, piece)
                {
                    return Err(SseDecodeError::new(SseDecodeErrorKind::LimitExceeded));
                }
                self.pending.extend_from_slice(piece);
                while let Some((end, separator)) = boundary(&self.pending) {
                    let frame_len = end + separator;
                    if frame_len > limit {
                        return Err(SseDecodeError::new(SseDecodeErrorKind::LimitExceeded));
                    }
                    let frame: Vec<u8> = self.pending.drain(..frame_len).collect();
                    frames.push(normalize_line_endings(frame));
                }
                if self.pending.len() > limit {
                    return Err(SseDecodeError::new(SseDecodeErrorKind::LimitExceeded));
                }
            }
        } else {
            self.pending.extend_from_slice(chunk);
            while let Some((end, separator)) = boundary(&self.pending) {
                let frame_len = end + separator;
                let frame: Vec<u8> = self.pending.drain(..frame_len).collect();
                frames.push(normalize_line_endings(frame));
            }
        }
        Ok(frames)
    }

    /// Verifies that the stream ended outside a partial event.
    ///
    /// # Errors
    ///
    /// Returns [`SseDecodeErrorKind::IncompleteFrame`] when undecoded
    /// non-whitespace bytes remain.
    pub fn finish(mut self) -> Result<(), SseDecodeError> {
        let incomplete = !self.pending.iter().all(u8::is_ascii_whitespace);
        self.pending.zeroize();
        if incomplete {
            Err(SseDecodeError::new(SseDecodeErrorKind::IncompleteFrame))
        } else {
            Ok(())
        }
    }
}

fn has_boundary_across(pending: &[u8], piece: &[u8]) -> bool {
    if boundary(piece).is_some() {
        return true;
    }
    let tail_len = pending.len().min(3);
    let head_len = piece.len().min(3);
    if tail_len == 0 || head_len == 0 {
        return false;
    }
    let mut seam = [0u8; 6];
    seam[..tail_len].copy_from_slice(&pending[pending.len() - tail_len..]);
    seam[tail_len..tail_len + head_len].copy_from_slice(&piece[..head_len]);
    boundary(&seam[..tail_len + head_len]).is_some()
}

fn boundary(buffer: &[u8]) -> Option<(usize, usize)> {
    let line_feed = buffer
        .windows(2)
        .position(|window| window == b"\n\n")
        .map(|end| (end, 2));
    let carriage_return = buffer
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .map(|end| (end, 4));
    match (line_feed, carriage_return) {
        (Some(left), Some(right)) => Some(if left.0 <= right.0 { left } else { right }),
        (Some(found), None) | (None, Some(found)) => Some(found),
        (None, None) => None,
    }
}

fn normalize_line_endings(frame: Vec<u8>) -> Vec<u8> {
    if !frame.contains(&b'\r') {
        return frame;
    }
    let mut normalized = Vec::with_capacity(frame.len());
    let mut index = 0;
    while index < frame.len() {
        if frame[index] == b'\r' && frame.get(index + 1) == Some(&b'\n') {
            index += 1;
            continue;
        }
        normalized.push(frame[index]);
        index += 1;
    }
    normalized
}

/// One decoded server-sent event with a redacted payload.
#[derive(Clone, Eq, PartialEq)]
pub struct SseFrame {
    name: Option<String>,
    data: Vec<u8>,
}

impl SseFrame {
    /// Returns the event name, when the frame declared one.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the bounded payload bytes.
    #[must_use]
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Splits the frame into its name and payload.
    #[must_use]
    pub fn into_parts(mut self) -> (Option<String>, Vec<u8>) {
        let data = std::mem::take(&mut self.data);
        (self.name.take(), data)
    }
}

impl Drop for SseFrame {
    fn drop(&mut self) {
        self.data.zeroize();
        if let Some(name) = self.name.as_mut() {
            name.zeroize();
        }
    }
}

impl fmt::Debug for SseFrame {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SseFrame")
            .field("name", &self.name)
            .field("data", &"<redacted>")
            .finish()
    }
}

/// Incremental, bounded server-sent event decoder.
///
/// Concrete adapters keep their provider event grammar and treat this decoder
/// as the one framing authority shared across every HTTP/SSE adapter.
#[derive(Debug, Default)]
pub struct SseDecoder {
    framer: SseFramer,
}

impl SseDecoder {
    /// Creates a decoder bounded by [`DEFAULT_EVENT_LIMIT`] per event.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a decoder over one explicit framer configuration.
    #[must_use]
    pub const fn with_framer(framer: SseFramer) -> Self {
        Self { framer }
    }

    /// Accepts one transport chunk and returns every complete decoded frame.
    ///
    /// Frames without an event name or data payload (comments, keep-alive
    /// noise, and blank separators) are skipped rather than surfaced.
    ///
    /// # Errors
    ///
    /// Returns a classified [`SseDecodeError`] when a frame is not UTF-8 or a
    /// configured bound would be exceeded.
    pub fn push(&mut self, chunk: &[u8]) -> Result<Vec<SseFrame>, SseDecodeError> {
        let mut frames = Vec::new();
        for frame in self.framer.push(chunk)? {
            if let Some(frame) = decode_frame(&frame)? {
                frames.push(frame);
            }
        }
        Ok(frames)
    }

    /// Verifies that the stream ended outside a partial event.
    ///
    /// # Errors
    ///
    /// Returns [`SseDecodeErrorKind::IncompleteFrame`] when undecoded
    /// non-whitespace bytes remain.
    pub fn finish(self) -> Result<(), SseDecodeError> {
        self.framer.finish()
    }
}

impl BodyDecoder for SseDecoder {
    type Item = SseFrame;
    type Error = SseDecodeError;

    fn push(&mut self, chunk: &[u8]) -> Result<Vec<Self::Item>, Self::Error> {
        Self::push(self, chunk)
    }

    fn finish(self) -> Result<(), Self::Error> {
        Self::finish(self)
    }
}

fn decode_frame(frame: &[u8]) -> Result<Option<SseFrame>, SseDecodeError> {
    let body = frame.strip_suffix(b"\n\n").unwrap_or(frame);
    let text = std::str::from_utf8(body)
        .map_err(|_| SseDecodeError::new(SseDecodeErrorKind::InvalidUtf8))?;
    let mut name = None;
    let mut data = Vec::new();
    let mut observed_data = false;
    for line in text.lines() {
        if line.starts_with(':') {
            continue;
        }
        if let Some(value) = line.strip_prefix("event:") {
            name = Some(value.trim().to_owned());
        } else if let Some(value) = line.strip_prefix("data:") {
            if observed_data {
                data.push(b'\n');
            }
            observed_data = true;
            let value = value.strip_prefix(' ').unwrap_or(value);
            data.extend_from_slice(value.as_bytes());
        }
    }
    if name.is_none() && !observed_data {
        return Ok(None);
    }
    Ok(Some(SseFrame { name, data }))
}
