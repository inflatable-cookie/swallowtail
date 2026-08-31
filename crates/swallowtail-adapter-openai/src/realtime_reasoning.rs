use swallowtail_core::ReasoningMode;

/// Maps one exact portable mode to its Realtime session wire value.
///
/// Only Research 236's five session-scoped values map. Nothing is clamped,
/// aliased, defaulted, or translated to a numeric budget.
pub(crate) fn session_effort(mode: &ReasoningMode) -> Option<&'static str> {
    Some(match mode.as_str() {
        "minimal" => "minimal",
        "low" => "low",
        "medium" => "medium",
        "high" => "high",
        "xhigh" => "xhigh",
        _ => return None,
    })
}

/// Reports whether the provider acknowledged a well-formed session effort.
pub(crate) fn is_session_effort(value: &str) -> bool {
    matches!(value, "minimal" | "low" | "medium" | "high" | "xhigh")
}
