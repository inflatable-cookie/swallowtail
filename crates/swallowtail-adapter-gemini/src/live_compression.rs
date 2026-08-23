/// Exact default-only sliding-window context compression for Gemini Live.
///
/// The selected wire shape is `contextWindowCompression.slidingWindow` with
/// an empty object. Explicit trigger and target token values remain outside
/// the admitted adapter surface until their model-specific domain is closed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GeminiLiveContextWindowCompression;

impl GeminiLiveContextWindowCompression {
    /// Selects the provider-described sliding window with provider defaults.
    #[must_use]
    pub const fn sliding_window() -> Self {
        Self
    }
}
