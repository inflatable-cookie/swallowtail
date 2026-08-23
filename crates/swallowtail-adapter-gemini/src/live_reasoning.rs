use swallowtail_core::ReasoningMode;

/// Exact level the fixed route dispatches when no caller selection is present.
pub(crate) const OMITTED_THINKING_LEVEL: &str = "MINIMAL";

/// Maps one exact portable mode to its qualified setup value.
///
/// Only the values Research 193 admits for `gemini-3.1-flash-live-preview`
/// map. Nothing is clamped, aliased, defaulted, or translated to a budget.
pub(crate) fn thinking_level(mode: &ReasoningMode) -> Option<&'static str> {
    Some(match mode.as_str() {
        "minimal" => "MINIMAL",
        "low" => "LOW",
        "medium" => "MEDIUM",
        "high" => "HIGH",
        _ => return None,
    })
}

/// Returns the setup value for an optional selection without inventing one.
pub(crate) fn setup_thinking_level(mode: Option<&ReasoningMode>) -> Option<&'static str> {
    match mode {
        None => Some(OMITTED_THINKING_LEVEL),
        Some(mode) => thinking_level(mode),
    }
}
