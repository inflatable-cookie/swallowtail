use serde_json::{Map, Value};
use swallowtail_runtime::RuntimeFailure;

use crate::failure::failure;

pub(super) fn decode_json_object(
    bytes: &[u8],
    maximum_bytes: usize,
) -> Result<Map<String, Value>, RuntimeFailure> {
    if bytes.is_empty() || bytes.len() > maximum_bytes {
        return Err(malformed());
    }
    match serde_json::from_slice(bytes).map_err(|_| malformed())? {
        Value::Object(object) => Ok(object),
        _ => Err(malformed()),
    }
}

pub(super) fn required_object<'a>(
    object: &'a Map<String, Value>,
    key: &str,
) -> Result<&'a Map<String, Value>, RuntimeFailure> {
    object
        .get(key)
        .and_then(Value::as_object)
        .ok_or_else(malformed)
}

pub(super) fn required_array<'a>(
    object: &'a Map<String, Value>,
    key: &str,
) -> Result<&'a Vec<Value>, RuntimeFailure> {
    object
        .get(key)
        .and_then(Value::as_array)
        .ok_or_else(malformed)
}

pub(super) fn required_string<'a>(
    object: &'a Map<String, Value>,
    key: &str,
) -> Result<&'a str, RuntimeFailure> {
    object
        .get(key)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(malformed)
}

pub(super) fn required_i64(object: &Map<String, Value>, key: &str) -> Result<i64, RuntimeFailure> {
    object
        .get(key)
        .and_then(Value::as_i64)
        .ok_or_else(malformed)
}

pub(super) fn required_u64(object: &Map<String, Value>, key: &str) -> Result<u64, RuntimeFailure> {
    object
        .get(key)
        .and_then(Value::as_u64)
        .ok_or_else(malformed)
}

pub(super) fn malformed() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.malformed_response",
        "Kimi local server returned malformed protocol data",
    )
}

pub(super) fn unsupported_event() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.event_unsupported",
        "Kimi local server returned an unsupported semantic event",
    )
}
