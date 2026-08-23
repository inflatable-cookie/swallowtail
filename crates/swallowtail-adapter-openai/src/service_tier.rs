/// Exact dispatch-only Responses service-tier selection for OpenAI background.
///
/// The only admitted value is standard processing, encoded as
/// `service_tier: "default"`. Swallowtail claims requested, planned, and
/// dispatched state only. It does not observe, claim, or infer the returned
/// processing tier, price, latency, capacity, or project setting.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OpenAiBackgroundServiceTier {
    _private: (),
}

impl OpenAiBackgroundServiceTier {
    /// Selects standard pricing and performance for the exact GPT-5.6 route.
    #[must_use]
    pub const fn standard() -> Self {
        Self { _private: () }
    }

    /// Returns the canonical Responses request spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        "default"
    }

    /// Parses only the admitted canonical value `default`.
    ///
    /// `auto`, `flex`, `priority`, `fast`, `ultrafast`, `scale`, aliases, and
    /// unknown strings return `None`.
    #[must_use]
    pub fn parse(value: &str) -> Option<Self> {
        (value == "default").then_some(Self::standard())
    }
}
