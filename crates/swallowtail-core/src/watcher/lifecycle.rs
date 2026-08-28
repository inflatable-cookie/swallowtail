use std::fmt;
use std::num::NonZeroU64;

/// Monotonic lifecycle revision for one watcher.
///
/// Revisions start at one on accepted start and increase on every retained
/// state change. They never decrease.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WatcherRevision(NonZeroU64);

impl WatcherRevision {
    /// Creates the first revision assigned to an accepted watcher.
    #[must_use]
    pub const fn initial() -> Self {
        Self(NonZeroU64::MIN)
    }

    /// Creates a revision from an exact positive counter.
    pub const fn new(value: u64) -> Option<Self> {
        match NonZeroU64::new(value) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    #[must_use]
    /// Returns the raw monotonic counter.
    pub const fn get(self) -> u64 {
        self.0.get()
    }

    #[must_use]
    /// Returns the next revision after a retained state change.
    pub const fn next(self) -> Self {
        match self.0.checked_add(1) {
            Some(value) => Self(value),
            None => self,
        }
    }
}

impl fmt::Debug for WatcherRevision {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("WatcherRevision")
            .field(&self.get())
            .finish()
    }
}

impl fmt::Display for WatcherRevision {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.get())
    }
}

/// Portable phase of one operation-scoped watcher.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WatcherLifecyclePhase {
    /// Host accepted the start request before work began.
    Accepted,
    /// Host-owned work is in progress.
    Running,
    /// Exactly one terminal cause has been recorded.
    Terminal,
    /// Cleanup has joined the watcher; this is cleanup truth, not a process result.
    Joined,
}

impl WatcherLifecyclePhase {
    #[must_use]
    /// Reports whether the phase has recorded a terminal cause.
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Terminal | Self::Joined)
    }

    #[must_use]
    /// Reports whether wait and completion gating treat the watcher as finished.
    pub const fn is_joined(self) -> bool {
        matches!(self, Self::Joined)
    }
}

/// Exact cause of the first terminal transition for one watcher.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WatcherTerminalCause {
    /// Host-owned work finished successfully.
    Completed,
    /// Host-owned work failed.
    Failed,
    /// The owning turn or an explicit cancel path cancelled the watcher.
    Cancelled,
    /// A host or turn deadline expired.
    TimedOut,
    /// Model or operator stop reached terminal state first.
    Stopped,
}

impl WatcherTerminalCause {
    #[must_use]
    /// Returns a stable public label without host or provider payload detail.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
            Self::TimedOut => "timed_out",
            Self::Stopped => "stopped",
        }
    }
}

impl fmt::Display for WatcherTerminalCause {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}
