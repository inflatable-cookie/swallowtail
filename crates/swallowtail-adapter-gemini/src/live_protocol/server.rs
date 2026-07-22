use super::handle::ProviderSessionHandle;
use crate::failure::failure;
use base64::Engine;
use serde_json::Value;
use swallowtail_runtime::{RuntimeFailure, TokenUsage};
use zeroize::Zeroize;

pub(crate) struct ProviderAudio(Vec<u8>);

impl ProviderAudio {
    pub(crate) fn bytes(&self) -> &[u8] {
        &self.0
    }
}

impl std::fmt::Debug for ProviderAudio {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("ProviderAudio(<redacted>)")
    }
}

impl Drop for ProviderAudio {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

pub(crate) enum ServerEvent {
    SetupComplete,
    Audio(ProviderAudio),
    Transcript(String),
    Usage(TokenUsage),
    ResumptionUpdate {
        resumable: bool,
        handle: Option<ProviderSessionHandle>,
    },
    GoAway(String),
    GenerationComplete,
    TurnComplete,
    Structural,
    ProviderFailed,
}

impl std::fmt::Debug for ServerEvent {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::SetupComplete => "ServerEvent::SetupComplete",
            Self::Audio(_) => "ServerEvent::Audio(<redacted>)",
            Self::Transcript(_) => "ServerEvent::Transcript(<redacted>)",
            Self::Usage(_) => "ServerEvent::Usage(<redacted>)",
            Self::ResumptionUpdate { .. } => "ServerEvent::ResumptionUpdate(<redacted>)",
            Self::GoAway(_) => "ServerEvent::GoAway(<redacted>)",
            Self::GenerationComplete => "ServerEvent::GenerationComplete",
            Self::TurnComplete => "ServerEvent::TurnComplete",
            Self::Structural => "ServerEvent::Structural",
            Self::ProviderFailed => "ServerEvent::ProviderFailed",
        })
    }
}

pub(crate) fn parse_server_frame(bytes: &[u8]) -> Result<Vec<ServerEvent>, RuntimeFailure> {
    let value: Value = serde_json::from_slice(bytes).map_err(|_| malformed())?;
    let object = value.as_object().ok_or_else(malformed)?;
    let mut events = Vec::new();

    if object.contains_key("setupComplete") {
        events.push(ServerEvent::SetupComplete);
    }
    if let Some(update) = object.get("sessionResumptionUpdate") {
        let resumable = boolean(update, "resumable")?;
        let handle = update
            .get("newHandle")
            .and_then(Value::as_str)
            .filter(|value| !value.is_empty())
            .map(|value| ProviderSessionHandle::new(value.to_owned()));
        if resumable != handle.is_some() {
            return Err(malformed());
        }
        events.push(ServerEvent::ResumptionUpdate { resumable, handle });
    }
    if let Some(go_away) = object.get("goAway") {
        events.push(ServerEvent::GoAway(text(go_away, "timeLeft")?.to_owned()));
    }
    if let Some(content) = object.get("serverContent") {
        parse_content(content, &mut events)?;
    }
    if let Some(usage) = object.get("usageMetadata") {
        events.push(ServerEvent::Usage(TokenUsage::new(
            usage.get("promptTokenCount").and_then(Value::as_u64),
            usage.get("responseTokenCount").and_then(Value::as_u64),
        )));
    }
    if object.contains_key("error") {
        events.push(ServerEvent::ProviderFailed);
    }
    if events.is_empty() {
        return Err(failure(
            "swallowtail.gemini.live_event_unknown",
            "Gemini Live event has unknown semantics",
        ));
    }
    Ok(events)
}

fn parse_content(content: &Value, events: &mut Vec<ServerEvent>) -> Result<(), RuntimeFailure> {
    let initial_event_count = events.len();
    if let Some(parts) = content
        .get("modelTurn")
        .and_then(|turn| turn.get("parts"))
        .and_then(Value::as_array)
    {
        for part in parts {
            let data = part.get("inlineData").ok_or_else(malformed)?;
            if text(data, "mimeType")? != "audio/pcm;rate=24000" {
                return Err(failure(
                    "swallowtail.gemini.live_format_drift",
                    "Gemini Live output format does not match preflight",
                ));
            }
            let bytes = base64::engine::general_purpose::STANDARD
                .decode(text(data, "data")?)
                .map_err(|_| malformed())?;
            events.push(ServerEvent::Audio(ProviderAudio(bytes)));
        }
    }
    if let Some(transcript) = content.get("outputTranscription") {
        events.push(ServerEvent::Transcript(
            text(transcript, "text")?.to_owned(),
        ));
    }
    if content.get("generationComplete").and_then(Value::as_bool) == Some(true) {
        events.push(ServerEvent::GenerationComplete);
    }
    if content.get("turnComplete").and_then(Value::as_bool) == Some(true) {
        events.push(ServerEvent::TurnComplete);
    }
    if events.len() == initial_event_count {
        events.push(ServerEvent::Structural);
    }
    Ok(())
}

fn text<'a>(value: &'a Value, key: &str) -> Result<&'a str, RuntimeFailure> {
    value.get(key).and_then(Value::as_str).ok_or_else(malformed)
}

fn boolean(value: &Value, key: &str) -> Result<bool, RuntimeFailure> {
    value
        .get(key)
        .and_then(Value::as_bool)
        .ok_or_else(malformed)
}

fn malformed() -> RuntimeFailure {
    failure(
        "swallowtail.gemini.live_protocol_malformed",
        "Gemini Live event is malformed",
    )
}
