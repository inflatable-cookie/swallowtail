//! Optional Contract 010 watcher host port.
//!
//! Registration alone starts nothing and grants no arbitrary process or PID
//! authority. Card 009 owns host-local execution binding.

use crate::{BoxFuture, CleanupOutcome, DeadlineObservation, RuntimeFailure, RuntimeTurnId};
use std::fmt;
use std::task::Context;
use swallowtail_core::{
    WatcherCleanupCause, WatcherId, WatcherOperationData, WatcherOwningTurn, WatcherRequester,
};

use super::{WatcherSnapshot, WatcherStopAcknowledgement, WatcherWaitRepresentation};

/// Optional live controls observed by one watcher wait.
///
/// Callers provide futures from their operation-scoped cancellation and time
/// services. The host polls these futures alongside joined watcher state and
/// returns [`WatcherWaitRepresentation::Cancelled`] or
/// [`WatcherWaitRepresentation::DeadlineExceeded`] before satisfaction.
pub struct WatcherWaitOptions<'a> {
    cancellation: Option<BoxFuture<'a, ()>>,
    deadline: Option<BoxFuture<'a, DeadlineObservation>>,
}

impl<'a> WatcherWaitOptions<'a> {
    /// Creates a wait with no cancellation or deadline control.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            cancellation: None,
            deadline: None,
        }
    }

    /// Adds the future that resolves when the owning operation is cancelled.
    #[must_use]
    pub fn with_cancellation(mut self, cancellation: BoxFuture<'a, ()>) -> Self {
        self.cancellation = Some(cancellation);
        self
    }

    /// Adds the future that resolves when the owning operation deadline is observed.
    #[must_use]
    pub fn with_deadline(mut self, deadline: BoxFuture<'a, DeadlineObservation>) -> Self {
        self.deadline = Some(deadline);
        self
    }
}

impl Default for WatcherWaitOptions<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for WatcherWaitOptions<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WatcherWaitOptions")
            .field("has_cancellation", &self.cancellation.is_some())
            .field("has_deadline", &self.deadline.is_some())
            .finish()
    }
}

impl WatcherWaitOptions<'_> {
    /// Polls the live controls and returns the first terminal wait outcome.
    ///
    /// `None` means that every configured control is still pending, or that
    /// no control was configured. The watcher host remains responsible for
    /// polling its joined state in that case.
    pub fn poll(&mut self, context: &mut Context<'_>) -> Option<WatcherWaitRepresentation> {
        if self
            .cancellation
            .as_mut()
            .is_some_and(|future| future.as_mut().poll(context).is_ready())
        {
            return Some(WatcherWaitRepresentation::Cancelled);
        }
        if self
            .deadline
            .as_mut()
            .is_some_and(|future| future.as_mut().poll(context).is_ready())
        {
            return Some(WatcherWaitRepresentation::DeadlineExceeded);
        }
        None
    }
}

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
        requester: WatcherRequester,
        operation_data: WatcherOperationData,
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
    fn wait<'a>(
        &'a self,
        owning_turn: WatcherOwningTurn,
        watcher_id: WatcherId,
        options: WatcherWaitOptions<'a>,
    ) -> BoxFuture<'a, Result<WatcherWaitRepresentation, RuntimeFailure>>;

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
        cause: WatcherCleanupCause,
    ) -> BoxFuture<'_, Result<(Vec<WatcherSnapshot>, CleanupOutcome), RuntimeFailure>>;

    /// Retires a successfully completed turn after every watcher is joined.
    ///
    /// This does not stop active work. It is the explicit successful-turn
    /// finalization seam; callers must use cancellation or deadline cleanup
    /// when work remains active.
    fn finalize_turn(
        &self,
        turn: RuntimeTurnId,
    ) -> BoxFuture<'_, Result<CleanupOutcome, RuntimeFailure>>;

    /// Opens one lossless lifecycle feed for the owning turn.
    ///
    /// Observation must start before watcher creation. The feed retains
    /// accepted, running, and terminal snapshots independently of provider
    /// stdout. Duplicate observers for the same turn fail closed.
    fn open_lifecycle_feed(
        &self,
        turn: RuntimeTurnId,
    ) -> BoxFuture<'_, Result<super::WatcherLifecycleSubscription, RuntimeFailure>>;

    /// Closes and releases the turn-scoped lifecycle feed.
    ///
    /// Idempotent. Must run on every start-failure path so a later attempt for
    /// the same turn is not rejected as a duplicate observer.
    fn close_lifecycle_feed(
        &self,
        turn: RuntimeTurnId,
    ) -> BoxFuture<'_, Result<(), RuntimeFailure>>;
}
