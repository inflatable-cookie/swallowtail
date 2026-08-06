use crate::{CodecLimits, ProtocolError, ProtocolErrorKind};
use serde_json::{Map, Value};
use std::fmt;

/// One bounded provider field not recognized by the structural codec.
#[derive(Clone, PartialEq)]
pub struct UnknownField {
    name: String,
    value: Value,
}

impl UnknownField {
    /// Returns the provider field name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the retained provider value.
    #[must_use]
    pub const fn value(&self) -> &Value {
        &self.value
    }
}

impl fmt::Debug for UnknownField {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("UnknownField")
            .field("name", &self.name)
            .field("value", &"<redacted>")
            .finish()
    }
}

/// One decoded compatible chat response payload.
#[derive(Clone, Debug, PartialEq)]
pub enum Payload {
    /// A streamed completion chunk.
    Chunk(Chunk),
    /// A provider error envelope.
    Error(ErrorEnvelope),
}

/// Provider-neutral structural view of one streamed completion chunk.
#[derive(Clone, Debug, PartialEq)]
pub struct Chunk {
    /// Optional provider response identity.
    pub id: Option<String>,
    /// Optional provider object discriminator.
    pub object: Option<String>,
    /// Optional provider creation timestamp.
    pub created: Option<u64>,
    /// Optional reported model identity.
    pub model: Option<String>,
    /// Bounded completion choices.
    pub choices: Vec<Choice>,
    /// Optional token usage reported by the provider.
    pub usage: Option<Usage>,
    /// Bounded unrecognized top-level provider fields.
    pub unknown_fields: Vec<UnknownField>,
}

/// One indexed streamed completion choice.
#[derive(Clone, Debug, PartialEq)]
pub struct Choice {
    /// Provider-supplied choice index.
    pub index: u64,
    /// Incremental content carried by this choice.
    pub delta: Delta,
    /// Optional provider finish reason.
    pub finish_reason: Option<String>,
    /// Bounded unrecognized choice fields.
    pub unknown_fields: Vec<UnknownField>,
}

/// Incremental role or text content from one streamed choice.
#[derive(Clone, Debug, PartialEq)]
pub struct Delta {
    /// Optional provider role.
    pub role: Option<String>,
    /// Optional incremental text content.
    pub content: Option<String>,
    /// Bounded unrecognized delta fields, including provider extensions.
    pub unknown_fields: Vec<UnknownField>,
}

/// Optional token accounting carried by a compatible provider.
#[derive(Clone, Debug, PartialEq)]
pub struct Usage {
    /// Input token count when reported.
    pub prompt_tokens: Option<u64>,
    /// Generated token count when reported.
    pub completion_tokens: Option<u64>,
    /// Total token count when reported.
    pub total_tokens: Option<u64>,
    /// Bounded unrecognized usage fields.
    pub unknown_fields: Vec<UnknownField>,
}

/// Provider error plus bounded unrecognized envelope fields.
#[derive(Clone, Debug, PartialEq)]
pub struct ErrorEnvelope {
    /// Decoded provider error body.
    pub error: ProviderError,
    /// Bounded unrecognized envelope fields.
    pub unknown_fields: Vec<UnknownField>,
}

/// Provider-originated error content; message text is not a safe diagnostic.
#[derive(Clone, PartialEq)]
pub struct ProviderError {
    /// Provider error type.
    pub kind: String,
    /// Optional provider error code in its original JSON form.
    pub code: Option<Value>,
    /// Provider message, redacted from `Debug` output.
    pub message: String,
    /// Bounded unrecognized provider-error fields.
    pub unknown_fields: Vec<UnknownField>,
}

impl fmt::Debug for ProviderError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProviderError")
            .field("kind", &self.kind)
            .field("code", &self.code)
            .field("message", &"<redacted>")
            .field("unknown_fields", &self.unknown_fields)
            .finish()
    }
}

/// Decodes one bounded compatible chat chunk or error document.
pub fn decode_payload(bytes: &[u8], limits: CodecLimits) -> Result<Payload, ProtocolError> {
    if bytes.len() > limits.maximum_wire_bytes() {
        return Err(ProtocolError::new(ProtocolErrorKind::WireLimitExceeded));
    }
    let value: Value = serde_json::from_slice(bytes)
        .map_err(|_| ProtocolError::new(ProtocolErrorKind::InvalidJson))?;
    let mut object = into_object(value, limits)?;
    if object.contains_key("error") {
        parse_error(&mut object, limits).map(Payload::Error)
    } else {
        parse_chunk(&mut object, limits).map(Payload::Chunk)
    }
}

fn parse_chunk(
    object: &mut Map<String, Value>,
    limits: CodecLimits,
) -> Result<Chunk, ProtocolError> {
    let choices = required_array(object, "choices")?;
    if choices.len() > limits.maximum_choices() {
        return Err(ProtocolError::new(ProtocolErrorKind::ChoiceLimitExceeded));
    }
    let choices = choices
        .into_iter()
        .map(|choice| parse_choice(choice, limits))
        .collect::<Result<_, _>>()?;
    let usage = object
        .remove("usage")
        .filter(|value| !value.is_null())
        .map(|value| parse_usage(value, limits))
        .transpose()?;
    Ok(Chunk {
        id: optional_string(object, "id", limits)?,
        object: optional_string(object, "object", limits)?,
        created: optional_u64(object, "created")?,
        model: optional_string(object, "model", limits)?,
        choices,
        usage,
        unknown_fields: unknowns(object, limits)?,
    })
}

fn parse_choice(value: Value, limits: CodecLimits) -> Result<Choice, ProtocolError> {
    let mut object = into_object(value, limits)?;
    let index = required_u64(&mut object, "index")?;
    let delta = object
        .remove("delta")
        .ok_or_else(invalid)
        .and_then(|value| parse_delta(value, limits))?;
    Ok(Choice {
        index,
        delta,
        finish_reason: optional_string(&mut object, "finish_reason", limits)?,
        unknown_fields: unknowns(&object, limits)?,
    })
}

fn parse_delta(value: Value, limits: CodecLimits) -> Result<Delta, ProtocolError> {
    let mut object = into_object(value, limits)?;
    Ok(Delta {
        role: optional_string(&mut object, "role", limits)?,
        content: optional_string(&mut object, "content", limits)?,
        unknown_fields: unknowns(&object, limits)?,
    })
}

fn parse_usage(value: Value, limits: CodecLimits) -> Result<Usage, ProtocolError> {
    let mut object = into_object(value, limits)?;
    Ok(Usage {
        prompt_tokens: optional_u64(&mut object, "prompt_tokens")?,
        completion_tokens: optional_u64(&mut object, "completion_tokens")?,
        total_tokens: optional_u64(&mut object, "total_tokens")?,
        unknown_fields: unknowns(&object, limits)?,
    })
}

fn parse_error(
    object: &mut Map<String, Value>,
    limits: CodecLimits,
) -> Result<ErrorEnvelope, ProtocolError> {
    let mut error = object
        .remove("error")
        .ok_or_else(invalid)
        .and_then(|value| into_object(value, limits))?;
    let kind = required_string(&mut error, "type", limits)?;
    let message = required_string(&mut error, "message", limits)?;
    let code = error.remove("code").filter(|value| !value.is_null());
    Ok(ErrorEnvelope {
        error: ProviderError {
            kind,
            code,
            message,
            unknown_fields: unknowns(&error, limits)?,
        },
        unknown_fields: unknowns(object, limits)?,
    })
}

fn into_object(value: Value, limits: CodecLimits) -> Result<Map<String, Value>, ProtocolError> {
    let object = value.as_object().cloned().ok_or_else(invalid)?;
    if object.len() > limits.maximum_fields() {
        return Err(ProtocolError::new(ProtocolErrorKind::FieldLimitExceeded));
    }
    Ok(object)
}

fn required_array(
    object: &mut Map<String, Value>,
    name: &str,
) -> Result<Vec<Value>, ProtocolError> {
    object
        .remove(name)
        .and_then(|value| value.as_array().cloned())
        .ok_or_else(invalid)
}

fn required_u64(object: &mut Map<String, Value>, name: &str) -> Result<u64, ProtocolError> {
    object
        .remove(name)
        .and_then(|value| value.as_u64())
        .ok_or_else(invalid)
}

fn optional_u64(object: &mut Map<String, Value>, name: &str) -> Result<Option<u64>, ProtocolError> {
    match object.remove(name) {
        None | Some(Value::Null) => Ok(None),
        Some(value) => value.as_u64().map(Some).ok_or_else(invalid),
    }
}

fn required_string(
    object: &mut Map<String, Value>,
    name: &str,
    limits: CodecLimits,
) -> Result<String, ProtocolError> {
    optional_string(object, name, limits)?.ok_or_else(invalid)
}

fn optional_string(
    object: &mut Map<String, Value>,
    name: &str,
    limits: CodecLimits,
) -> Result<Option<String>, ProtocolError> {
    match object.remove(name) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(value)) if value.len() <= limits.maximum_string_bytes() => {
            Ok(Some(value))
        }
        Some(Value::String(_)) => Err(ProtocolError::new(ProtocolErrorKind::StringLimitExceeded)),
        Some(_) => Err(invalid()),
    }
}

fn unknowns(
    object: &Map<String, Value>,
    limits: CodecLimits,
) -> Result<Vec<UnknownField>, ProtocolError> {
    if object.len() > limits.maximum_fields() {
        return Err(ProtocolError::new(ProtocolErrorKind::FieldLimitExceeded));
    }
    object
        .iter()
        .map(|(name, value)| {
            if name.len() > limits.maximum_string_bytes() {
                return Err(ProtocolError::new(ProtocolErrorKind::StringLimitExceeded));
            }
            Ok(UnknownField {
                name: name.clone(),
                value: value.clone(),
            })
        })
        .collect()
}

const fn invalid() -> ProtocolError {
    ProtocolError::new(ProtocolErrorKind::InvalidStructure)
}
