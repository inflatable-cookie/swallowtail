use crate::failure::failure;
use base64::Engine;
use serde_json::Value;
use swallowtail_runtime::{
    ProviderCancellationOutcome, RateLimitKind, RateLimitObservation, RuntimeFailure, TokenUsage,
};
use zeroize::Zeroize;

mod stream;

pub(crate) use stream::RealtimeServerStream;

pub(crate) struct ProviderAudio(Vec<u8>);

impl ProviderAudio {
    pub(crate) fn bytes(&self) -> &[u8] {
        &self.0
    }
}

impl std::fmt::Debug for ProviderAudio {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_tuple("ProviderAudio")
            .field(&format_args!("<redacted:{} bytes>", self.0.len()))
            .finish()
    }
}

impl Drop for ProviderAudio {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

pub(crate) enum RealtimeServerEvent {
    SessionConfigured,
    InputCommitted,
    ResponseStarted(String),
    AudioDelta {
        response_id: String,
        audio: ProviderAudio,
    },
    AudioCompleted {
        response_id: String,
    },
    TranscriptDelta {
        response_id: String,
        transcript: String,
    },
    TranscriptCompleted {
        response_id: String,
        transcript: String,
    },
    Usage {
        response_id: String,
        usage: TokenUsage,
        cancelled: Option<ProviderCancellationOutcome>,
    },
    RateLimits(Vec<RateLimitObservation>),
    ProviderFailed,
}

impl std::fmt::Debug for RealtimeServerEvent {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::SessionConfigured => "RealtimeServerEvent::SessionConfigured",
            Self::InputCommitted => "RealtimeServerEvent::InputCommitted",
            Self::ResponseStarted(_) => "RealtimeServerEvent::ResponseStarted(<redacted>)",
            Self::AudioDelta { .. } => "RealtimeServerEvent::AudioDelta(<redacted>)",
            Self::AudioCompleted { .. } => "RealtimeServerEvent::AudioCompleted(<redacted>)",
            Self::TranscriptDelta { .. } => "RealtimeServerEvent::TranscriptDelta(<redacted>)",
            Self::TranscriptCompleted { .. } => {
                "RealtimeServerEvent::TranscriptCompleted(<redacted>)"
            }
            Self::Usage { .. } => "RealtimeServerEvent::Usage(<redacted>)",
            Self::RateLimits(_) => "RealtimeServerEvent::RateLimits",
            Self::ProviderFailed => "RealtimeServerEvent::ProviderFailed",
        })
    }
}

pub(crate) fn parse_server_event(bytes: &[u8]) -> Result<RealtimeServerEvent, RuntimeFailure> {
    let value: Value = serde_json::from_slice(bytes).map_err(|_| malformed())?;
    let kind = text(&value, "type")?;
    match kind {
        "session.created" | "session.updated" => parse_session(&value),
        "input_audio_buffer.committed" => Ok(RealtimeServerEvent::InputCommitted),
        "response.created" => Ok(RealtimeServerEvent::ResponseStarted(
            nested_text(&value, &["response", "id"])?.to_owned(),
        )),
        "response.output_audio.delta" => Ok(RealtimeServerEvent::AudioDelta {
            response_id: text(&value, "response_id")?.to_owned(),
            audio: ProviderAudio(
                base64::engine::general_purpose::STANDARD
                    .decode(text(&value, "delta")?)
                    .map_err(|_| malformed())?,
            ),
        }),
        "response.output_audio.done" => Ok(RealtimeServerEvent::AudioCompleted {
            response_id: text(&value, "response_id")?.to_owned(),
        }),
        "response.output_audio_transcript.delta" => Ok(RealtimeServerEvent::TranscriptDelta {
            response_id: text(&value, "response_id")?.to_owned(),
            transcript: text(&value, "delta")?.to_owned(),
        }),
        "response.output_audio_transcript.done" => Ok(RealtimeServerEvent::TranscriptCompleted {
            response_id: text(&value, "response_id")?.to_owned(),
            transcript: text(&value, "transcript")?.to_owned(),
        }),
        "response.done" => parse_done(&value),
        "rate_limits.updated" => parse_rate_limits(&value),
        "error" => Ok(RealtimeServerEvent::ProviderFailed),
        _ => Err(failure(
            "swallowtail.openai.realtime_event_unknown",
            "OpenAI Realtime event has unknown semantics",
        )),
    }
}

fn parse_session(value: &Value) -> Result<RealtimeServerEvent, RuntimeFailure> {
    if nested_text(value, &["session", "model"])? != "gpt-realtime-2.1"
        || nested_text(value, &["session", "audio", "input", "format", "type"])? != "audio/pcm"
        || nested_u64(value, &["session", "audio", "input", "format", "rate"])? != 24_000
        || nested_text(value, &["session", "audio", "output", "format", "type"])? != "audio/pcm"
        || nested_u64(value, &["session", "audio", "output", "format", "rate"])? != 24_000
    {
        return Err(failure(
            "swallowtail.openai.realtime_format_drift",
            "OpenAI Realtime session format does not match preflight",
        ));
    }
    Ok(RealtimeServerEvent::SessionConfigured)
}

fn parse_done(value: &Value) -> Result<RealtimeServerEvent, RuntimeFailure> {
    let response = value.get("response").ok_or_else(malformed)?;
    let status = text(response, "status")?;
    let cancelled = match status {
        "completed" => None,
        "cancelled" => Some(ProviderCancellationOutcome::Confirmed),
        "failed" | "incomplete" => return Ok(RealtimeServerEvent::ProviderFailed),
        _ => return Err(malformed()),
    };
    let usage = response.get("usage").ok_or_else(malformed)?;
    Ok(RealtimeServerEvent::Usage {
        response_id: text(response, "id")?.to_owned(),
        usage: TokenUsage::new(
            usage.get("input_tokens").and_then(Value::as_u64),
            usage.get("output_tokens").and_then(Value::as_u64),
        ),
        cancelled,
    })
}

fn parse_rate_limits(value: &Value) -> Result<RealtimeServerEvent, RuntimeFailure> {
    let entries = value
        .get("rate_limits")
        .and_then(Value::as_array)
        .ok_or_else(malformed)?;
    let mut observations = Vec::with_capacity(entries.len());
    for entry in entries {
        let kind = match text(entry, "name")? {
            "requests" => RateLimitKind::Requests,
            "tokens" => RateLimitKind::Tokens,
            _ => return Err(malformed()),
        };
        let reset = entry
            .get("reset_seconds")
            .and_then(Value::as_f64)
            .map(|seconds| (seconds * 1_000.0) as u64);
        observations.push(RateLimitObservation::new(
            kind,
            entry.get("limit").and_then(Value::as_u64),
            entry.get("remaining").and_then(Value::as_u64),
            reset,
        ));
    }
    Ok(RealtimeServerEvent::RateLimits(observations))
}

fn text<'a>(value: &'a Value, key: &str) -> Result<&'a str, RuntimeFailure> {
    value.get(key).and_then(Value::as_str).ok_or_else(malformed)
}

fn nested_text<'a>(value: &'a Value, path: &[&str]) -> Result<&'a str, RuntimeFailure> {
    path.iter()
        .try_fold(value, |current, key| current.get(key).ok_or_else(malformed))?
        .as_str()
        .ok_or_else(malformed)
}

fn nested_u64(value: &Value, path: &[&str]) -> Result<u64, RuntimeFailure> {
    path.iter()
        .try_fold(value, |current, key| current.get(key).ok_or_else(malformed))?
        .as_u64()
        .ok_or_else(malformed)
}

fn malformed() -> RuntimeFailure {
    failure(
        "swallowtail.openai.realtime_protocol_malformed",
        "OpenAI Realtime event is malformed",
    )
}
