use super::malformed_stream;
use serde_json::Value;
use swallowtail_runtime::{RuntimeFailure, TokenUsage};

pub(super) fn session_id(payload: &Value) -> Result<&str, RuntimeFailure> {
    payload
        .get("session_id")
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(malformed_stream)
}

pub(super) fn token_usage(payload: &Value) -> Option<TokenUsage> {
    Some(TokenUsage::new(
        Some(payload.pointer("/usage/input_tokens")?.as_u64()?),
        Some(payload.pointer("/usage/output_tokens")?.as_u64()?),
    ))
}
