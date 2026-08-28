//! Optional Contract 010 watcher host port.
//!
//! Registration alone starts nothing and grants no arbitrary process or PID
//! authority. Card 009 owns host-local execution binding.

use crate::{BoxFuture, CleanupOutcome, RuntimeFailure, RuntimeTurnId};
use swallowtail_core::{WatcherId, WatcherOwningTurn, WatcherSummary, WatcherTerminalCause};

use super::{WatcherSnapshot, WatcherStopAcknowledgement, WatcherWaitRepresentation};

/// Host boundary for turn-scoped watcher operations.
///
/// The portable core registers this optional kind without authorizing start.
/// Concrete hosts implement start, wait, stop, and join under host policy.
pub trait WatcherHostService: Send + Sync {
    /// Accepts one host-authorized watcher request for the active turn.
    ///
    /// Implementations interpret bounded operation data under host policy. The
    /// portable surface never carries an executable path, shell command, PID,
    /// or permission grant.
    fn accept_start(
        &self,
        turn: RuntimeTurnId,
        summary: Option<WatcherSummary>,
    ) -> BoxFuture<'_, Result<WatcherSnapshot, RuntimeFailure>>;

    /// Returns the latest monotonic snapshot for one owned watcher.
    fn inspect(
        &self,
        owning_turn: WatcherOwningTurn,
        watcher_id: WatcherId,
    ) -> BoxFuture<'_, Result<WatcherSnapshot, RuntimeFailure>>;

    /// Lists the active turn's bounded watcher set.
    fn list(
        &self,
        owning_turn: WatcherOwningTurn,
    ) -> BoxFuture<'_, Result<Vec<WatcherSnapshot>, RuntimeFailure>>;

    /// Returns wait gating truth until terminal and joined, or turn end.
    fn wait(
        &self,
        owning_turn: WatcherOwningTurn,
        watcher_id: WatcherId,
    ) -> BoxFuture<'_, Result<WatcherWaitRepresentation, RuntimeFailure>>;

    /// Requests idempotent stop for one owned watcher.
    fn request_stop(
        &self,
        owning_turn: WatcherOwningTurn,
        watcher_id: WatcherId,
    ) -> BoxFuture<'_, Result<(WatcherStopAcknowledgement, WatcherSnapshot), RuntimeFailure>>;

    /// Stops and joins every watcher owned by a cancelled or timed-out turn.
    fn stop_and_join_all(
        &self,
        turn: RuntimeTurnId,
        cause: WatcherTerminalCause,
    ) -> BoxFuture<'_, Result<(Vec<WatcherSnapshot>, CleanupOutcome), RuntimeFailure>>;
}
