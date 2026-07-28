use serde_json::Value;
use swallowtail_runtime::TokenUsage;

pub(super) fn token_usage(payload: &Value) -> Option<TokenUsage> {
    let usage = payload.get("usage")?;
    Some(
        TokenUsage::new(
            Some(usage.get("input_tokens")?.as_u64()?),
            Some(usage.get("output_tokens")?.as_u64()?),
        )
        .with_cache_tokens(
            optional_u64(usage, "cache_read_input_tokens")?,
            optional_u64(usage, "cache_creation_input_tokens")?,
        ),
    )
}

fn optional_u64(value: &Value, key: &str) -> Option<Option<u64>> {
    match value.get(key) {
        None | Some(Value::Null) => Some(None),
        Some(value) => value.as_u64().map(Some),
    }
}
