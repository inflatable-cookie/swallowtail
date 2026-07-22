use serde_json::Value;
use swallowtail_protocol_openai_chat::{
    CodecLimits, Payload, SseDecoder, SseRecord, UnknownField, decode_payload,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum FixtureError {
    Invalid,
    ModelMismatch,
    UnknownSemantic,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum KimiEvent {
    RoleStart,
    Reasoning(String),
    Output(String),
    Finished(String),
    Usage {
        input: u64,
        output: u64,
        cached: u64,
    },
    Done,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum SafeErrorCategory {
    Authentication,
    Permission,
    ModelUnavailable,
    Quota,
    RateLimited,
    ServiceUnavailable,
}

pub(super) fn decode_kimi(bytes: &[u8]) -> Result<Vec<KimiEvent>, FixtureError> {
    let mut decoder = SseDecoder::default();
    let records = decoder.push(bytes).map_err(|_| FixtureError::Invalid)?;
    decoder.finish().map_err(|_| FixtureError::Invalid)?;
    let mut events = Vec::new();
    for record in records {
        match record {
            SseRecord::Done => events.push(KimiEvent::Done),
            SseRecord::Data(data) => map_payload(&data, &mut events)?,
        }
    }
    Ok(events)
}

fn map_payload(data: &[u8], events: &mut Vec<KimiEvent>) -> Result<(), FixtureError> {
    let Payload::Chunk(chunk) =
        decode_payload(data, CodecLimits::default()).map_err(|_| FixtureError::Invalid)?
    else {
        return Err(FixtureError::Invalid);
    };
    if chunk.model.as_deref() != Some("kimi-k3") {
        return Err(FixtureError::ModelMismatch);
    }
    if !chunk.unknown_fields.is_empty() || chunk.choices.len() != 1 {
        return Err(FixtureError::UnknownSemantic);
    }
    let choice = &chunk.choices[0];
    if !choice.unknown_fields.is_empty() {
        return Err(FixtureError::UnknownSemantic);
    }
    if choice.delta.role.as_deref() == Some("assistant") && choice.delta.content.is_none() {
        events.push(KimiEvent::RoleStart);
    }
    if let Some(reasoning) = one_unknown_string(&choice.delta.unknown_fields, "reasoning_content")?
    {
        events.push(KimiEvent::Reasoning(reasoning));
    } else if !choice.delta.unknown_fields.is_empty() {
        return Err(FixtureError::UnknownSemantic);
    }
    if let Some(content) = &choice.delta.content {
        events.push(KimiEvent::Output(content.clone()));
    }
    if let Some(reason) = &choice.finish_reason {
        events.push(KimiEvent::Finished(reason.clone()));
    }
    if let Some(usage) = &chunk.usage {
        let cached = one_unknown_u64(&usage.unknown_fields, "cached_tokens")?
            .ok_or(FixtureError::Invalid)?;
        events.push(KimiEvent::Usage {
            input: usage.prompt_tokens.ok_or(FixtureError::Invalid)?,
            output: usage.completion_tokens.ok_or(FixtureError::Invalid)?,
            cached,
        });
    }
    Ok(())
}

fn one_unknown_string(fields: &[UnknownField], name: &str) -> Result<Option<String>, FixtureError> {
    match fields {
        [] => Ok(None),
        [field] if field.name() == name => field
            .value()
            .as_str()
            .map(|value| Some(value.to_owned()))
            .ok_or(FixtureError::Invalid),
        _ => Err(FixtureError::UnknownSemantic),
    }
}

fn one_unknown_u64(fields: &[UnknownField], name: &str) -> Result<Option<u64>, FixtureError> {
    match fields {
        [] => Ok(None),
        [field] if field.name() == name => field
            .value()
            .as_u64()
            .map(Some)
            .ok_or(FixtureError::Invalid),
        _ => Err(FixtureError::UnknownSemantic),
    }
}

pub(super) fn map_error(fixture: &Value) -> Result<SafeErrorCategory, FixtureError> {
    let status = fixture["status"].as_u64().ok_or(FixtureError::Invalid)?;
    let encoded = serde_json::to_vec(&fixture["body"]).map_err(|_| FixtureError::Invalid)?;
    let Payload::Error(error) =
        decode_payload(&encoded, CodecLimits::default()).map_err(|_| FixtureError::Invalid)?
    else {
        return Err(FixtureError::Invalid);
    };
    match (status, error.error.kind.as_str()) {
        (401, "invalid_authentication_error" | "incorrect_api_key_error") => {
            Ok(SafeErrorCategory::Authentication)
        }
        (403, "permission_denied_error") => Ok(SafeErrorCategory::Permission),
        (404, "resource_not_found_error") => Ok(SafeErrorCategory::ModelUnavailable),
        (429, "exceeded_current_quota_error") => Ok(SafeErrorCategory::Quota),
        (429, "rate_limit_reached_error" | "engine_overloaded_error") => {
            Ok(SafeErrorCategory::RateLimited)
        }
        (500 | 503, "server_error" | "unexpected_output" | "server_unavailable") => {
            Ok(SafeErrorCategory::ServiceUnavailable)
        }
        _ => Err(FixtureError::Invalid),
    }
}
