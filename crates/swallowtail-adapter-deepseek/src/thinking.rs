#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Exact adapter-local thinking-mode selection for a DeepSeek structured run.
///
/// This selection describes local dispatch only. It does not claim provider
/// acceptance, effective mode, or any continuation capability.
pub struct DeepSeekThinkingMode {
    _private: (),
}

impl DeepSeekThinkingMode {
    #[must_use]
    /// Returns the admitted non-thinking structured-run selection.
    pub const fn disabled() -> Self {
        Self { _private: () }
    }

    #[must_use]
    /// Returns the exact DeepSeek wire value for this selection.
    pub const fn as_str(self) -> &'static str {
        "disabled"
    }
}
