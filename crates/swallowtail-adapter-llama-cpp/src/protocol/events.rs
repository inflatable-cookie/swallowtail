use serde_json::Value;
use swallowtail_runtime::TokenUsage;

const MAX_SSE_BYTES: usize = 1_048_576;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum Event {
    RoleStart,
    OutputDelta(String),
    Finished(String),
    Usage(TokenUsage),
    Done,
    ProviderFailed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SseFrame {
    pub data: Vec<u8>,
}

#[derive(Default)]
pub(crate) struct SseDecoder {
    buffer: Vec<u8>,
}

impl SseDecoder {
    pub(crate) fn push(&mut self, chunk: &[u8]) -> Result<Vec<SseFrame>, RuntimeFailure> {
        if self.buffer.len().saturating_add(chunk.len()) > MAX_SSE_BYTES {
            return Err(failure(
                "swallowtail.llama_cpp.sse_limit",
                "llama.cpp SSE input exceeded its limit",
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
                "swallowtail.llama_cpp.sse_disconnected",
                "llama.cpp SSE disconnected during an event",
            ))
        }
    }
}

pub(crate) fn parse_event(frame: &SseFrame) -> Result<Event, RuntimeFailure> {
    if frame.data == b"[DONE]" {
        return Ok(Event::Done);
    }
    let value: Value = serde_json::from_slice(&frame.data)
        .map_err(|_| protocol_failure("stream event"))?;
    if value.get("error").is_some() {
        return Ok(Event::ProviderFailed);
    }
    let choices = value["choices"]
        .as_array()
        .ok_or_else(|| protocol_failure("stream choices"))?;
    if choices.is_empty() {
        let usage = &value["usage"];
        return Ok(Event::Usage(TokenUsage::new(
            usage["prompt_tokens"].as_u64(),
            usage["completion_tokens"].as_u64(),
        )));
    }
    if choices.len() != 1 {
        return Err(protocol_failure("stream choice count"));
    }
    let choice = &choices[0];
    let delta = &choice["delta"];
    if delta.get("tool_calls").is_some()
        || delta.get("function_call").is_some()
        || delta.get("reasoning_content").is_some()
    {
        return Err(failure(
            "swallowtail.llama_cpp.content_semantics_unsupported",
            "llama.cpp emitted content outside the observed text-only fixture",
        ));
    }
    if delta["role"].as_str() == Some("assistant") && delta["content"].is_null() {
        return Ok(Event::RoleStart);
    }
    if let Some(content) = delta["content"].as_str() {
        return Ok(Event::OutputDelta(content.to_owned()));
    }
    if let Some(reason) = choice["finish_reason"].as_str() {
        return Ok(Event::Finished(reason.to_owned()));
    }
    Err(protocol_failure("stream delta"))
}

fn boundary(buffer: &[u8]) -> Option<usize> {
    buffer
        .windows(2)
        .position(|value| value == b"\n\n")
        .or_else(|| {
            buffer
                .windows(4)
                .position(|value| value == b"\r\n\r\n")
        })
}

fn decode_frame(frame: &[u8]) -> Result<Option<SseFrame>, RuntimeFailure> {
    let text = std::str::from_utf8(frame).map_err(|_| protocol_failure("SSE encoding"))?;
    let mut data = Vec::new();
    for line in text.lines() {
        if let Some(value) = line.strip_prefix("data:") {
            if !data.is_empty() {
                data.push(b'\n');
            }
            data.extend_from_slice(value.trim_start().as_bytes());
        } else if !line.starts_with(':') {
            return Err(protocol_failure("SSE field"));
        }
    }
    if data.is_empty() && text.lines().all(|line| line.starts_with(':')) {
        return Ok(None);
    }
    if data.is_empty() {
        return Err(protocol_failure("SSE data"));
    }
    Ok(Some(SseFrame { data }))
}
