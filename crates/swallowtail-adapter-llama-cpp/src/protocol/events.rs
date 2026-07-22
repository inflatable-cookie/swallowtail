use swallowtail_protocol_openai_chat::{
    Payload, ProtocolError, ProtocolErrorKind, SseRecord, decode_payload,
};
use swallowtail_runtime::TokenUsage;

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
    record: SseRecord,
}

#[derive(Default)]
pub(crate) struct SseDecoder {
    inner: swallowtail_protocol_openai_chat::SseDecoder,
}

impl SseDecoder {
    pub(crate) fn push(&mut self, chunk: &[u8]) -> Result<Vec<SseFrame>, RuntimeFailure> {
        self.inner
            .push(chunk)
            .map(|records| {
                records
                    .into_iter()
                    .map(|record| SseFrame { record })
                    .collect()
            })
            .map_err(map_sse_failure)
    }

    pub(crate) fn finish(self) -> Result<(), RuntimeFailure> {
        self.inner.finish().map_err(map_sse_failure)
    }
}

pub(crate) fn parse_event(frame: &SseFrame) -> Result<Event, RuntimeFailure> {
    let SseRecord::Data(data) = &frame.record else {
        return Ok(Event::Done);
    };
    let payload = decode_payload(
        data,
        swallowtail_protocol_openai_chat::CodecLimits::default(),
    )
        .map_err(|_| protocol_failure("stream event"))?;
    let Payload::Chunk(chunk) = payload else {
        return Ok(Event::ProviderFailed);
    };
    if !chunk.unknown_fields.is_empty() {
        return Err(protocol_failure("stream fields"));
    }
    if chunk.choices.is_empty() {
        let usage = chunk
            .usage
            .filter(|usage| usage.unknown_fields.is_empty())
            .ok_or_else(|| protocol_failure("stream usage"))?;
        return Ok(Event::Usage(TokenUsage::new(
            usage.prompt_tokens,
            usage.completion_tokens,
        )));
    }
    if chunk.choices.len() != 1 {
        return Err(protocol_failure("stream choice count"));
    }
    let choice = &chunk.choices[0];
    if !choice.unknown_fields.is_empty() || !choice.delta.unknown_fields.is_empty() {
        return Err(failure(
            "swallowtail.llama_cpp.content_semantics_unsupported",
            "llama.cpp emitted content outside the observed text-only fixture",
        ));
    }
    if choice.delta.role.as_deref() == Some("assistant") && choice.delta.content.is_none() {
        return Ok(Event::RoleStart);
    }
    if let Some(content) = &choice.delta.content {
        return Ok(Event::OutputDelta(content.clone()));
    }
    if let Some(reason) = &choice.finish_reason {
        return Ok(Event::Finished(reason.clone()));
    }
    Err(protocol_failure("stream delta"))
}

fn map_sse_failure(error: ProtocolError) -> RuntimeFailure {
    match error.kind() {
        ProtocolErrorKind::BufferLimitExceeded | ProtocolErrorKind::WireLimitExceeded => failure(
            "swallowtail.llama_cpp.sse_limit",
            "llama.cpp SSE input exceeded its limit",
        ),
        ProtocolErrorKind::IncompleteRecord => failure(
            "swallowtail.llama_cpp.sse_disconnected",
            "llama.cpp SSE disconnected during an event",
        ),
        _ => protocol_failure("SSE framing"),
    }
}
