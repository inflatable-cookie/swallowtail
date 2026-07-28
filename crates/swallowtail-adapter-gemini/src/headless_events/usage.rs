use serde_json::Value;
use swallowtail_runtime::TokenUsage;

pub(super) fn token_usage(payload: &Value) -> Option<TokenUsage> {
    let stats = payload.get("stats")?;
    Some(
        TokenUsage::new(
            Some(stats.get("input_tokens")?.as_u64()?),
            Some(stats.get("output_tokens")?.as_u64()?),
        )
        .with_cache_tokens(Some(stats.get("cached")?.as_u64()?), None)
        .with_cache_miss_input_tokens(Some(stats.get("input")?.as_u64()?)),
    )
}
