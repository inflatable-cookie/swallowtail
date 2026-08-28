//! Failure classification for the pure watcher registry.

use std::error::Error;
use std::fmt;

/// Stable reason a pure watcher registry transition failed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WatcherFailureKind {
    /// Capacity was zero or otherwise invalid.
    InvalidCapacity,
    /// The requested id or turn does not belong to this registry.
    ForeignIdentity,
    /// The watcher id is unknown in the active turn.
    UnknownWatcher,
    /// Accepting another watcher would exceed the turn bound.
    CapacityExceeded,
    /// The requested lifecycle transition regresses or duplicates state.
    InvalidTransition,
    /// A terminal cause was already recorded.
    AlreadyTerminal,
    /// The watcher has already been joined.
    AlreadyJoined,
    /// Wait cannot be satisfied because the watcher is not terminal and joined.
    WaitNotSatisfied,
}

/// Safe failure returned by the pure watcher registry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WatcherFailure {
    kind: WatcherFailureKind,
}

impl WatcherFailure {
    pub(super) const fn new(kind: WatcherFailureKind) -> Self {
        Self { kind }
    }

    #[must_use]
    /// Returns the stable failure classification.
    pub const fn kind(self) -> WatcherFailureKind {
        self.kind
    }
}

impl fmt::Display for WatcherFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self.kind {
            WatcherFailureKind::InvalidCapacity => "Watcher registry requires a positive capacity",
            WatcherFailureKind::ForeignIdentity => {
                "Watcher operation rejected a foreign or stale identity"
            }
            WatcherFailureKind::UnknownWatcher => "Watcher id is unknown for the owning turn",
            WatcherFailureKind::CapacityExceeded => {
                "Watcher registry exceeded its configured capacity"
            }
            WatcherFailureKind::InvalidTransition => {
                "Watcher lifecycle rejected an invalid transition"
            }
            WatcherFailureKind::AlreadyTerminal => {
                "Watcher already recorded an exact terminal cause"
            }
            WatcherFailureKind::AlreadyJoined => "Watcher has already been joined",
            WatcherFailureKind::WaitNotSatisfied => {
                "Watcher wait requires terminal and joined state"
            }
        };
        formatter.write_str(message)
    }
}

impl Error for WatcherFailure {}
