//! Provider-neutral operation-scoped watcher records.
//!
//! These types describe identity, lifecycle, ownership, and redacted summaries.
//! They contain no process launch, executor, or provider tool authority.

mod identity;
mod lifecycle;
mod requester;
mod summary;

pub use identity::{
    InvalidWatcherRecord, MAX_WATCHER_ID_BYTES, MAX_WATCHER_OWNING_TURN_BYTES, WatcherId,
    WatcherOwningTurn,
};
pub use lifecycle::{WatcherLifecyclePhase, WatcherRevision, WatcherTerminalCause};
pub use requester::WatcherRequester;
pub use summary::{MAX_WATCHER_SUMMARY_BYTES, WatcherSummary};

/// Default positive bound for watchers owned by one turn.
pub const DEFAULT_MAX_WATCHERS_PER_TURN: usize = 32;
