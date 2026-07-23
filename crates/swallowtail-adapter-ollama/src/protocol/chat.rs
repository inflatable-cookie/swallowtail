use super::ndjson::NdjsonDecoder;
use super::{bounded_json, protocol_failure, unsupported_semantics};
use serde::Deserialize;
use std::collections::BTreeMap;
use swallowtail_runtime::{RuntimeFailure, TokenUsage};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NativeEvent {
    OutputDelta(String),
    Finished(String),
    Usage(TokenUsage),
    ProviderFailed,
}

pub struct ChatDecoder {
    inner: NdjsonDecoder,
    expected_model: String,
    terminal: bool,
}

impl ChatDecoder {
    pub fn new(expected_model: impl Into<String>) -> Self {
        Self {
            inner: NdjsonDecoder::default(),
            expected_model: expected_model.into(),
            terminal: false,
        }
    }

    pub fn push(&mut self, chunk: &[u8]) -> Result<Vec<NativeEvent>, RuntimeFailure> {
        let lines = self.inner.push(chunk)?;
        let mut events = Vec::new();
        for line in lines {
            if self.terminal {
                return Err(protocol_failure("event after terminal"));
            }
            let mut decoded = decode_record(&line, &self.expected_model)?;
            if decoded.iter().any(|event| {
                matches!(
                    event,
                    NativeEvent::Finished(_) | NativeEvent::ProviderFailed
                )
            }) {
                self.terminal = true;
            }
            events.append(&mut decoded);
        }
        Ok(events)
    }

    pub fn finish(self) -> Result<(), RuntimeFailure> {
        self.inner.finish()?;
        if self.terminal {
            Ok(())
        } else {
            Err(crate::failure::failure(
                "swallowtail.ollama.stream_disconnected",
                "Ollama native stream ended before a terminal record",
            ))
        }
    }
}

fn decode_record(bytes: &[u8], expected_model: &str) -> Result<Vec<NativeEvent>, RuntimeFailure> {
    let value: serde_json::Value = bounded_json(bytes, "stream record")?;
    if value.get("error").is_some() {
        let error: ErrorRecord =
            serde_json::from_value(value).map_err(|_| protocol_failure("stream error record"))?;
        if error.error.trim().is_empty() || !error.extra.is_empty() {
            return Err(protocol_failure("stream error record"));
        }
        return Ok(vec![NativeEvent::ProviderFailed]);
    }
    let record: ChatRecord =
        serde_json::from_value(value).map_err(|_| protocol_failure("chat record"))?;
    if record.model != expected_model
        || record.created_at.trim().is_empty()
        || record.remote_model.is_some()
        || record.remote_host.is_some()
        || !record.extra.is_empty()
        || record.message.role != "assistant"
        || record.message.thinking.is_some()
        || record.message.images.is_some()
        || record.message.tool_calls.is_some()
        || record.message.tool_name.is_some()
        || record.message.tool_call_id.is_some()
        || !record.message.extra.is_empty()
    {
        return Err(unsupported_semantics());
    }
    if record.done {
        let reason = record
            .done_reason
            .filter(|reason| matches!(reason.as_str(), "stop" | "length"))
            .ok_or_else(unsupported_semantics)?;
        if !record.message.content.is_empty() {
            return Err(protocol_failure("terminal content"));
        }
        if record.total_duration.is_none()
            || record.load_duration.is_none()
            || record.prompt_eval_duration.is_none()
            || record.eval_duration.is_none()
        {
            return Err(protocol_failure("terminal metrics"));
        }
        let input = record
            .prompt_eval_count
            .ok_or_else(|| protocol_failure("terminal usage"))?;
        let output = record
            .eval_count
            .ok_or_else(|| protocol_failure("terminal usage"))?;
        return Ok(vec![
            NativeEvent::Finished(reason),
            NativeEvent::Usage(TokenUsage::new(Some(input), Some(output))),
        ]);
    }
    if record.done_reason.is_some()
        || record.prompt_eval_count.is_some()
        || record.eval_count.is_some()
        || record.total_duration.is_some()
        || record.load_duration.is_some()
        || record.prompt_eval_duration.is_some()
        || record.eval_duration.is_some()
        || record.message.content.is_empty()
    {
        return Err(protocol_failure("output delta"));
    }
    Ok(vec![NativeEvent::OutputDelta(record.message.content)])
}

#[derive(Deserialize)]
struct ErrorRecord {
    error: String,
    #[serde(flatten)]
    extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Deserialize)]
struct ChatRecord {
    model: String,
    created_at: String,
    message: Message,
    done: bool,
    #[serde(default)]
    done_reason: Option<String>,
    #[serde(default)]
    prompt_eval_count: Option<u64>,
    #[serde(default)]
    eval_count: Option<u64>,
    #[serde(default)]
    total_duration: Option<u64>,
    #[serde(default)]
    load_duration: Option<u64>,
    #[serde(default)]
    prompt_eval_duration: Option<u64>,
    #[serde(default)]
    eval_duration: Option<u64>,
    #[serde(default)]
    remote_model: Option<String>,
    #[serde(default)]
    remote_host: Option<String>,
    #[serde(flatten)]
    extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Deserialize)]
struct Message {
    role: String,
    content: String,
    #[serde(default)]
    thinking: Option<serde_json::Value>,
    #[serde(default)]
    images: Option<serde_json::Value>,
    #[serde(default)]
    tool_calls: Option<serde_json::Value>,
    #[serde(default)]
    tool_name: Option<serde_json::Value>,
    #[serde(default)]
    tool_call_id: Option<serde_json::Value>,
    #[serde(flatten)]
    extra: BTreeMap<String, serde_json::Value>,
}
