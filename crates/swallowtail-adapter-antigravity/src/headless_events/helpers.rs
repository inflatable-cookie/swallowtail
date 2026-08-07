use serde_json::Value;
use swallowtail_runtime::{RuntimeFailure, TokenUsage};

pub(super) fn token_usage(value: &Value) -> Result<Option<TokenUsage>, RuntimeFailure> {
    let Some(usage) = value.get("usage") else {
        return Ok(None);
    };
    let input = required_u64(usage, "input_tokens")?;
    let output = required_u64(usage, "output_tokens")?;
    let thinking = required_u64(usage, "thinking_tokens")?;
    let cache_read = required_u64(usage, "cache_read_tokens")?;
    let total = required_u64(usage, "total_tokens")?;
    if input.checked_add(output) != Some(total) {
        return Err(malformed_stream());
    }
    Ok(Some(
        TokenUsage::new(Some(input), Some(output))
            .with_reasoning_tokens(Some(thinking))
            .with_cache_tokens(Some(cache_read), None),
    ))
}

pub(super) fn subagents(value: &Value) -> Result<Option<Vec<super::SubagentEvidence>>, RuntimeFailure> {
    let Some(info) = value.get("subagent_info") else {
        return Ok(None);
    };
    let children = info
        .get("subagents")
        .and_then(Value::as_array)
        .ok_or_else(malformed_stream)?;
    if children.is_empty() || children.len() > super::MAXIMUM_SUBAGENTS {
        return Err(malformed_stream());
    }
    children
        .iter()
        .map(|child| {
            let id = bounded_identity(required_text(child, "conversation_id")?)?.to_owned();
            let label = child
                .get("type_name")
                .or_else(|| child.get("role"))
                .and_then(Value::as_str)
                .map(bounded_label)
                .transpose()?
                .map(str::to_owned);
            Ok((id, label))
        })
        .collect::<Result<Vec<_>, RuntimeFailure>>()
        .map(Some)
}

pub(super) fn required_text<'a>(value: &'a Value, key: &str) -> Result<&'a str, RuntimeFailure> {
    value
        .get(key)
        .and_then(Value::as_str)
        .ok_or_else(malformed_stream)
}

pub(super) fn required_u64(value: &Value, key: &str) -> Result<u64, RuntimeFailure> {
    value
        .get(key)
        .and_then(Value::as_u64)
        .ok_or_else(malformed_stream)
}

pub(super) fn bounded_identity(value: &str) -> Result<&str, RuntimeFailure> {
    if value.is_empty() || value.len() > 256 || value.chars().any(char::is_control) {
        Err(malformed_stream())
    } else {
        Ok(value)
    }
}

pub(super) fn bounded_label(value: &str) -> Result<&str, RuntimeFailure> {
    if value.trim().is_empty() || value.len() > 256 || value.chars().any(char::is_control) {
        Err(malformed_stream())
    } else {
        Ok(value)
    }
}

pub(super) fn bounded_text(value: &str) -> Result<&str, RuntimeFailure> {
    if value.len() > super::MAXIMUM_OUTPUT_BYTES {
        Err(stream_limit())
    } else {
        Ok(value)
    }
}

pub(super) fn trim_newline(line: &[u8]) -> &[u8] {
    let line = line.strip_suffix(b"\n").unwrap_or(line);
    line.strip_suffix(b"\r").unwrap_or(line)
}

pub(super) fn malformed_stream() -> RuntimeFailure {
    crate::failure::failure(
        "swallowtail.antigravity.headless.malformed_stream",
        "Antigravity emitted malformed headless stream output",
    )
}

pub(super) fn conversation_mismatch() -> RuntimeFailure {
    crate::failure::failure(
        "swallowtail.antigravity.headless.conversation_mismatch",
        "Antigravity returned a different conversation identity",
    )
}

pub(super) fn stream_limit() -> RuntimeFailure {
    crate::failure::failure(
        "swallowtail.antigravity.headless.stream_limit",
        "Antigravity exceeded the bounded headless stream limit",
    )
}
