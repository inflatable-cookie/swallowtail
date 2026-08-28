//! Public records retained by the pure watcher registry.

use crate::ActivityId;
use swallowtail_core::{
    WatcherId, WatcherLifecyclePhase, WatcherOwningTurn, WatcherRequester, WatcherRevision,
    WatcherSummary, WatcherTerminalCause,
};

/// Idempotent acknowledgement for a stop request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WatcherStopAcknowledgement {
    /// This call recorded the first terminal stop.
    Stopped,
    /// Stop had already been recorded, or another terminal cause won the race.
    AlreadyTerminal(WatcherTerminalCause),
}

/// Pure representation of wait gating for one watcher.
///
/// The registry does not invent an executor or auto-wait policy. Callers use
/// this representation to decide when a reserved wait tool may resolve.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WatcherWaitRepresentation {
    /// The watcher is accepted or running.
    Pending,
    /// Terminal cause is recorded, but join has not completed.
    TerminalUnjoined(WatcherTerminalCause),
    /// Terminal and joined; wait may resolve successfully.
    Satisfied(WatcherTerminalCause),
    /// Wait was cancelled before satisfaction.
    Cancelled,
    /// Wait deadline elapsed before satisfaction.
    DeadlineExceeded,
}

/// Monotonic snapshot of one retained watcher.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WatcherSnapshot {
    watcher_id: WatcherId,
    owning_turn: WatcherOwningTurn,
    activity_id: ActivityId,
    phase: WatcherLifecyclePhase,
    terminal_cause: Option<WatcherTerminalCause>,
    revision: WatcherRevision,
    summary: Option<WatcherSummary>,
    accepted_by: WatcherRequester,
}

impl WatcherSnapshot {
    pub(super) fn from_record(
        watcher_id: WatcherId,
        owning_turn: WatcherOwningTurn,
        record: &WatcherRecord,
    ) -> Self {
        Self {
            watcher_id,
            owning_turn,
            activity_id: record.activity_id.clone(),
            phase: record.phase,
            terminal_cause: record.terminal_cause,
            revision: record.revision,
            summary: record.summary.clone(),
            accepted_by: record.accepted_by,
        }
    }

    #[must_use]
    /// Returns the opaque watcher identity.
    pub const fn watcher_id(&self) -> &WatcherId {
        &self.watcher_id
    }

    #[must_use]
    /// Returns the owning-turn key retained with this watcher.
    pub const fn owning_turn(&self) -> &WatcherOwningTurn {
        &self.owning_turn
    }

    #[must_use]
    /// Returns the presentation activity identity correlated to this watcher.
    pub const fn activity_id(&self) -> &ActivityId {
        &self.activity_id
    }

    #[must_use]
    /// Returns the portable lifecycle phase.
    pub const fn phase(&self) -> WatcherLifecyclePhase {
        self.phase
    }

    #[must_use]
    /// Returns the exact terminal cause when recorded.
    pub const fn terminal_cause(&self) -> Option<WatcherTerminalCause> {
        self.terminal_cause
    }

    #[must_use]
    /// Returns the monotonic lifecycle revision.
    pub const fn revision(&self) -> WatcherRevision {
        self.revision
    }

    #[must_use]
    /// Returns the bounded redacted summary when present.
    pub const fn summary(&self) -> Option<&WatcherSummary> {
        self.summary.as_ref()
    }

    #[must_use]
    /// Returns the requester that accepted the start.
    pub const fn accepted_by(&self) -> WatcherRequester {
        self.accepted_by
    }
}

#[derive(Clone, Debug)]
pub(super) struct WatcherRecord {
    pub(super) activity_id: ActivityId,
    pub(super) phase: WatcherLifecyclePhase,
    pub(super) terminal_cause: Option<WatcherTerminalCause>,
    pub(super) revision: WatcherRevision,
    pub(super) summary: Option<WatcherSummary>,
    pub(super) accepted_by: WatcherRequester,
}
