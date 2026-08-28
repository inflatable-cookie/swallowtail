/// Identity of the control path that issued a watcher operation.
///
/// Model and operator paths retain separate requester identity over one shared
/// registry state. They do not create separate watcher records.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WatcherRequester {
    /// Reserved model-facing watcher tool family.
    Model,
    /// Consumer-facing turn control surface.
    Operator,
}

impl WatcherRequester {
    #[must_use]
    /// Returns a stable public label for diagnostics and assertions.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Model => "model",
            Self::Operator => "operator",
        }
    }
}
