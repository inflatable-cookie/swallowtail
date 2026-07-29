use crate::activity::{
    AcpBoundedText, ActivityDecodeError, ActivityDecodeErrorKind, ActivityDecodeLimits,
};
use serde_json::{Map, Value};

pub(super) fn object(
    value: &Value,
    kind: ActivityDecodeErrorKind,
) -> Result<&Map<String, Value>, ActivityDecodeError> {
    value.as_object().ok_or_else(|| error(kind))
}

pub(super) fn required_array<'a>(
    object: &'a Map<String, Value>,
    field: &str,
    kind: ActivityDecodeErrorKind,
    limits: ActivityDecodeLimits,
) -> Result<&'a [Value], ActivityDecodeError> {
    let values = object
        .get(field)
        .and_then(Value::as_array)
        .ok_or_else(|| error(kind))?;
    ensure_collection_bound(values.len(), limits)?;
    Ok(values)
}

pub(super) fn optional_array<'a>(
    object: &'a Map<String, Value>,
    field: &str,
    kind: ActivityDecodeErrorKind,
    limits: ActivityDecodeLimits,
) -> Result<Option<&'a [Value]>, ActivityDecodeError> {
    match object.get(field) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Array(values)) => {
            ensure_collection_bound(values.len(), limits)?;
            Ok(Some(values))
        }
        Some(_) => Err(error(kind)),
    }
}

pub(super) fn ensure_collection_bound(
    len: usize,
    limits: ActivityDecodeLimits,
) -> Result<(), ActivityDecodeError> {
    if len > limits.maximum_collection_items() {
        Err(error(ActivityDecodeErrorKind::LimitExceeded))
    } else {
        Ok(())
    }
}

pub(super) fn required_text(
    object: &Map<String, Value>,
    field: &str,
    kind: ActivityDecodeErrorKind,
) -> Result<AcpBoundedText, ActivityDecodeError> {
    required_str(object, field, kind).map(|value| AcpBoundedText(value.to_owned()))
}

pub(super) fn optional_text(
    object: &Map<String, Value>,
    field: &str,
    kind: ActivityDecodeErrorKind,
) -> Result<Option<AcpBoundedText>, ActivityDecodeError> {
    match object.get(field) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(value)) => Ok(Some(AcpBoundedText(value.clone()))),
        Some(_) => Err(error(kind)),
    }
}

pub(super) fn required_identifier(
    object: &Map<String, Value>,
    field: &str,
    missing: ActivityDecodeErrorKind,
    limits: ActivityDecodeLimits,
) -> Result<AcpBoundedText, ActivityDecodeError> {
    let value = match object.get(field) {
        None => return Err(error(missing)),
        Some(Value::String(value)) => value,
        Some(_) => return Err(error(ActivityDecodeErrorKind::IdentifierInvalid)),
    };
    validate_identifier(value, limits)?;
    Ok(AcpBoundedText(value.to_owned()))
}

pub(super) fn optional_identifier(
    object: &Map<String, Value>,
    field: &str,
    limits: ActivityDecodeLimits,
) -> Result<Option<AcpBoundedText>, ActivityDecodeError> {
    match object.get(field) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(value)) => {
            validate_identifier(value, limits)?;
            Ok(Some(AcpBoundedText(value.clone())))
        }
        Some(_) => Err(error(ActivityDecodeErrorKind::IdentifierInvalid)),
    }
}

pub(super) fn validate_identifier(
    value: &str,
    limits: ActivityDecodeLimits,
) -> Result<(), ActivityDecodeError> {
    if value.trim().is_empty()
        || value.len() > limits.maximum_identifier_bytes()
        || value.chars().any(char::is_control)
    {
        Err(error(ActivityDecodeErrorKind::IdentifierInvalid))
    } else {
        Ok(())
    }
}

pub(super) fn required_str<'a>(
    object: &'a Map<String, Value>,
    field: &str,
    kind: ActivityDecodeErrorKind,
) -> Result<&'a str, ActivityDecodeError> {
    object
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| error(kind))
}

pub(super) fn required_u64(
    object: &Map<String, Value>,
    field: &str,
    kind: ActivityDecodeErrorKind,
) -> Result<u64, ActivityDecodeError> {
    object
        .get(field)
        .and_then(Value::as_u64)
        .ok_or_else(|| error(kind))
}

pub(super) const fn error(kind: ActivityDecodeErrorKind) -> ActivityDecodeError {
    ActivityDecodeError::new(kind)
}
