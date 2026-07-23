use super::private::{PrivateAccumulator, PrivateContinuation};
use super::response::Usage;
use super::{ProtocolFailure, ProtocolFailureKind};
use crate::selection::DEEPSEEK_MODEL_ID;
use serde_json::Value;
use swallowtail_core::DirectContinuationConfig;
use swallowtail_protocol_openai_chat::{
    CodecLimits, Payload, SseDecoder, SseRecord, UnknownField, decode_payload,
};

pub(crate) struct FinalAttempt {
    pub(crate) output: String,
    pub(crate) reasoning: PrivateContinuation,
    pub(crate) finish_reason: String,
    pub(crate) usage: Usage,
}

impl std::fmt::Debug for FinalAttempt {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("FinalAttempt(<redacted>)")
    }
}

#[derive(Debug)]
pub(crate) enum FinalStreamUpdate {
    Output(String),
    Usage(Usage),
    Finished(String),
}

pub(crate) struct FinalStreamParser {
    decoder: Option<SseDecoder>,
    maximum_records: usize,
    maximum_private_bytes: std::num::NonZeroU64,
    records: usize,
    output: String,
    reasoning: PrivateAccumulator,
    finish_reason: Option<String>,
    usage: Option<Usage>,
    done: bool,
}

impl FinalStreamParser {
    pub(crate) fn new(config: &DirectContinuationConfig) -> Self {
        Self {
            decoder: Some(SseDecoder::default()),
            maximum_records: config.maximum_stream_records_per_attempt().get() as usize,
            maximum_private_bytes: config.maximum_private_continuation_bytes(),
            records: 0,
            output: String::new(),
            reasoning: PrivateAccumulator::default(),
            finish_reason: None,
            usage: None,
            done: false,
        }
    }

    pub(crate) fn push(&mut self, bytes: &[u8]) -> Result<Vec<FinalStreamUpdate>, ProtocolFailure> {
        let records = self
            .decoder
            .as_mut()
            .expect("active decoder exists")
            .push(bytes)
            .map_err(|_| ProtocolFailure::new(ProtocolFailureKind::InvalidStructure))?;
        let mut updates = Vec::new();
        for record in records {
            self.records += 1;
            if self.records > self.maximum_records {
                return Err(ProtocolFailure::new(ProtocolFailureKind::BoundExceeded));
            }
            self.apply(record, &mut updates)?;
        }
        Ok(updates)
    }

    pub(crate) fn finish(mut self) -> Result<FinalAttempt, ProtocolFailure> {
        self.decoder
            .take()
            .expect("active decoder exists")
            .finish()
            .map_err(|_| ProtocolFailure::new(ProtocolFailureKind::IncompleteStream))?;
        if !self.done
            || self.output.is_empty()
            || self.usage.is_none()
            || self.finish_reason.is_none()
        {
            return Err(ProtocolFailure::new(ProtocolFailureKind::IncompleteStream));
        }
        Ok(FinalAttempt {
            output: self.output,
            reasoning: PrivateContinuation::new(self.reasoning.take(), self.maximum_private_bytes)?,
            finish_reason: self.finish_reason.take().expect("validated finish reason"),
            usage: self.usage.expect("validated usage"),
        })
    }

    fn apply(
        &mut self,
        record: SseRecord,
        updates: &mut Vec<FinalStreamUpdate>,
    ) -> Result<(), ProtocolFailure> {
        match record {
            SseRecord::Done if !self.done => self.done = true,
            SseRecord::Done | SseRecord::Data(_) if self.done => {
                return Err(ProtocolFailure::new(ProtocolFailureKind::InvalidStructure));
            }
            SseRecord::Done => {
                return Err(ProtocolFailure::new(ProtocolFailureKind::InvalidStructure));
            }
            SseRecord::Data(data) => {
                let Payload::Chunk(chunk) = decode_payload(&data, CodecLimits::default())
                    .map_err(|_| ProtocolFailure::new(ProtocolFailureKind::InvalidStructure))?
                else {
                    return Err(ProtocolFailure::new(ProtocolFailureKind::ProviderFailure));
                };
                if chunk.model.as_deref() != Some(DEEPSEEK_MODEL_ID) {
                    return Err(ProtocolFailure::new(ProtocolFailureKind::ModelMismatch));
                }
                allow_unknowns(&chunk.unknown_fields, &["system_fingerprint"])?;
                if let Some(wire_usage) = chunk.usage {
                    allow_unknowns(
                        &wire_usage.unknown_fields,
                        &["prompt_cache_hit_tokens", "prompt_cache_miss_tokens"],
                    )?;
                    let usage = Usage {
                        prompt_tokens: wire_usage.prompt_tokens.ok_or_else(invalid)?,
                        completion_tokens: wire_usage.completion_tokens.ok_or_else(invalid)?,
                        total_tokens: wire_usage.total_tokens.ok_or_else(invalid)?,
                        cache_hit_tokens: unknown_u64(
                            &wire_usage.unknown_fields,
                            "prompt_cache_hit_tokens",
                        )?,
                        cache_miss_tokens: unknown_u64(
                            &wire_usage.unknown_fields,
                            "prompt_cache_miss_tokens",
                        )?,
                    };
                    self.usage = Some(usage);
                    updates.push(FinalStreamUpdate::Usage(usage));
                }
                for choice in chunk.choices {
                    self.apply_choice(choice, updates)?;
                }
            }
        }
        Ok(())
    }

    fn apply_choice(
        &mut self,
        choice: swallowtail_protocol_openai_chat::Choice,
        updates: &mut Vec<FinalStreamUpdate>,
    ) -> Result<(), ProtocolFailure> {
        if choice.index != 0 || !choice.unknown_fields.is_empty() {
            return Err(ProtocolFailure::new(
                ProtocolFailureKind::UnknownSemanticField,
            ));
        }
        allow_unknowns(&choice.delta.unknown_fields, &["reasoning_content"])?;
        if let Some(content) = choice.delta.content {
            self.output.push_str(&content);
            updates.push(FinalStreamUpdate::Output(content));
        }
        if let Some(value) =
            unknown_optional_string(&choice.delta.unknown_fields, "reasoning_content")?
        {
            self.reasoning.push(value, self.maximum_private_bytes)?;
        }
        if let Some(reason) = choice.finish_reason {
            if !matches!(
                reason.as_str(),
                "stop" | "length" | "content_filter" | "insufficient_system_resource"
            ) {
                return Err(ProtocolFailure::new(
                    ProtocolFailureKind::UnknownSemanticField,
                ));
            }
            self.finish_reason = Some(reason.clone());
            updates.push(FinalStreamUpdate::Finished(reason));
        }
        Ok(())
    }
}

#[cfg(test)]
pub(crate) fn parse_final_stream(
    bytes: &[u8],
    config: &DirectContinuationConfig,
) -> Result<FinalAttempt, ProtocolFailure> {
    let mut parser = FinalStreamParser::new(config);
    for fragment in bytes.chunks(7) {
        let _ = parser.push(fragment)?;
    }
    parser.finish()
}

fn allow_unknowns(fields: &[UnknownField], allowed: &[&str]) -> Result<(), ProtocolFailure> {
    if fields.iter().all(|field| allowed.contains(&field.name())) {
        Ok(())
    } else {
        Err(ProtocolFailure::new(
            ProtocolFailureKind::UnknownSemanticField,
        ))
    }
}

fn unknown_u64(fields: &[UnknownField], name: &str) -> Result<u64, ProtocolFailure> {
    fields
        .iter()
        .find(|field| field.name() == name)
        .and_then(|field| field.value().as_u64())
        .ok_or_else(invalid)
}

fn unknown_optional_string<'a>(
    fields: &'a [UnknownField],
    name: &str,
) -> Result<Option<&'a str>, ProtocolFailure> {
    match fields.iter().find(|field| field.name() == name) {
        None => Ok(None),
        Some(field) => match field.value() {
            Value::Null => Ok(None),
            Value::String(value) => Ok(Some(value)),
            _ => Err(invalid()),
        },
    }
}

fn invalid() -> ProtocolFailure {
    ProtocolFailure::new(ProtocolFailureKind::InvalidStructure)
}
