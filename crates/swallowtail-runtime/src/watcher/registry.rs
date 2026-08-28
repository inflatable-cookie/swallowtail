use crate::{ActivityId, RuntimeTurnId};
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use swallowtail_core::{
    DEFAULT_MAX_WATCHERS_PER_TURN, WatcherCleanupCause, WatcherId, WatcherLifecyclePhase,
    WatcherOwningTurn, WatcherRequester, WatcherRevision, WatcherSummary, WatcherTerminalCause,
};

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
    const fn new(kind: WatcherFailureKind) -> Self {
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
struct WatcherRecord {
    activity_id: ActivityId,
    phase: WatcherLifecyclePhase,
    terminal_cause: Option<WatcherTerminalCause>,
    revision: WatcherRevision,
    summary: Option<WatcherSummary>,
    accepted_by: WatcherRequester,
}

/// Pure deterministic registry for one turn-owned watcher set.
///
/// The registry retains lifecycle truth only. It never launches work, selects a
/// provider route, or stores executable paths, commands, PIDs, or raw output.
pub struct WatcherRegistry {
    owning_turn: WatcherOwningTurn,
    runtime_turn: RuntimeTurnId,
    maximum_watchers: usize,
    next_sequence: u64,
    order: Vec<WatcherId>,
    records: BTreeMap<WatcherId, WatcherRecord>,
}

impl WatcherRegistry {
    /// Creates a registry for one runtime turn and positive watcher bound.
    pub fn new(
        runtime_turn: RuntimeTurnId,
        maximum_watchers: usize,
    ) -> Result<Self, WatcherFailure> {
        if maximum_watchers == 0 {
            return Err(WatcherFailure::new(WatcherFailureKind::InvalidCapacity));
        }
        let owning_turn = WatcherOwningTurn::new(runtime_turn.as_str().to_owned())
            .map_err(|_| WatcherFailure::new(WatcherFailureKind::ForeignIdentity))?;
        // Prove at least one turn-bound id fits the public byte bound.
        let _ = WatcherId::new(format!("{}/watcher-1", owning_turn.as_str()))
            .map_err(|_| WatcherFailure::new(WatcherFailureKind::InvalidCapacity))?;
        let _ = ActivityId::new(format!("{}/watcher-activity-1", owning_turn.as_str()))
            .map_err(|_| WatcherFailure::new(WatcherFailureKind::InvalidCapacity))?;
        Ok(Self {
            owning_turn,
            runtime_turn,
            maximum_watchers,
            next_sequence: 1,
            order: Vec::new(),
            records: BTreeMap::new(),
        })
    }

    /// Creates a registry using the default turn capacity.
    pub fn with_default_capacity(runtime_turn: RuntimeTurnId) -> Result<Self, WatcherFailure> {
        Self::new(runtime_turn, DEFAULT_MAX_WATCHERS_PER_TURN)
    }

    #[must_use]
    /// Returns the runtime turn that owns this registry.
    pub const fn runtime_turn(&self) -> &RuntimeTurnId {
        &self.runtime_turn
    }

    #[must_use]
    /// Returns the opaque owning-turn key.
    pub const fn owning_turn(&self) -> &WatcherOwningTurn {
        &self.owning_turn
    }

    #[must_use]
    /// Returns the configured positive watcher bound.
    pub const fn maximum_watchers(&self) -> usize {
        self.maximum_watchers
    }

    #[must_use]
    /// Returns the number of retained watchers.
    pub fn len(&self) -> usize {
        self.records.len()
    }

    #[must_use]
    /// Reports whether the registry retains no watchers.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Accepts a start request and retains an opaque watcher identity.
    ///
    /// Acceptance does not start host work. The caller supplies the optional
    /// summary; the registry allocates the watcher and activity identities.
    ///
    /// Allocated ids embed the owning-turn key so a stale id from another turn
    /// cannot alias a later registry's sequence-local names.
    pub fn accept_start(
        &mut self,
        requester: WatcherRequester,
        summary: Option<WatcherSummary>,
    ) -> Result<WatcherSnapshot, WatcherFailure> {
        if self.records.len() >= self.maximum_watchers {
            return Err(WatcherFailure::new(WatcherFailureKind::CapacityExceeded));
        }
        let sequence = self.next_sequence;
        self.next_sequence = self.next_sequence.saturating_add(1);
        let watcher_id = WatcherId::new(format!(
            "{}/watcher-{}",
            self.owning_turn.as_str(),
            sequence
        ))
        .map_err(|_| WatcherFailure::new(WatcherFailureKind::InvalidTransition))?;
        let activity_id = ActivityId::new(format!(
            "{}/watcher-activity-{}",
            self.owning_turn.as_str(),
            sequence
        ))
        .map_err(|_| WatcherFailure::new(WatcherFailureKind::InvalidTransition))?;
        let record = WatcherRecord {
            activity_id,
            phase: WatcherLifecyclePhase::Accepted,
            terminal_cause: None,
            revision: WatcherRevision::initial(),
            summary,
            accepted_by: requester,
        };
        self.order.push(watcher_id.clone());
        self.records.insert(watcher_id.clone(), record);
        self.snapshot(&watcher_id)
    }

    /// Transitions an accepted watcher to running.
    pub fn mark_running(
        &mut self,
        watcher_id: &WatcherId,
    ) -> Result<WatcherSnapshot, WatcherFailure> {
        let record = self.record_mut(watcher_id)?;
        if record.phase != WatcherLifecyclePhase::Accepted {
            return Err(WatcherFailure::new(WatcherFailureKind::InvalidTransition));
        }
        record.phase = WatcherLifecyclePhase::Running;
        record.revision = record.revision.next();
        self.snapshot(watcher_id)
    }

    /// Records the first terminal cause for an accepted or running watcher.
    pub fn complete(
        &mut self,
        watcher_id: &WatcherId,
        cause: WatcherTerminalCause,
        summary: Option<WatcherSummary>,
    ) -> Result<WatcherSnapshot, WatcherFailure> {
        let record = self.record_mut(watcher_id)?;
        match record.phase {
            WatcherLifecyclePhase::Accepted | WatcherLifecyclePhase::Running => {}
            WatcherLifecyclePhase::Terminal | WatcherLifecyclePhase::Joined => {
                return Err(WatcherFailure::new(WatcherFailureKind::AlreadyTerminal));
            }
        }
        record.phase = WatcherLifecyclePhase::Terminal;
        record.terminal_cause = Some(cause);
        if let Some(summary) = summary {
            record.summary = Some(summary);
        }
        record.revision = record.revision.next();
        self.snapshot(watcher_id)
    }

    /// Requests stop. Repeated stop is idempotent; races keep the first cause.
    /// Requests stop. Repeated stop is idempotent; races keep the first cause.
    ///
    /// The owning turn and watcher id are validated together before mutation so
    /// a stale id from another turn cannot stop current work.
    pub fn request_stop(
        &mut self,
        owning_turn: &WatcherOwningTurn,
        watcher_id: &WatcherId,
    ) -> Result<(WatcherStopAcknowledgement, WatcherSnapshot), WatcherFailure> {
        self.inspect(owning_turn, watcher_id)?;
        let record = self.record_mut(watcher_id)?;
        match record.phase {
            WatcherLifecyclePhase::Accepted | WatcherLifecyclePhase::Running => {
                record.phase = WatcherLifecyclePhase::Terminal;
                record.terminal_cause = Some(WatcherTerminalCause::Stopped);
                record.revision = record.revision.next();
                let snapshot = self.snapshot(watcher_id)?;
                Ok((WatcherStopAcknowledgement::Stopped, snapshot))
            }
            WatcherLifecyclePhase::Terminal | WatcherLifecyclePhase::Joined => {
                let cause = record
                    .terminal_cause
                    .ok_or_else(|| WatcherFailure::new(WatcherFailureKind::InvalidTransition))?;
                let snapshot = self.snapshot(watcher_id)?;
                Ok((WatcherStopAcknowledgement::AlreadyTerminal(cause), snapshot))
            }
        }
    }

    /// Marks a terminal watcher joined. Join is cleanup truth.
    pub fn join(&mut self, watcher_id: &WatcherId) -> Result<WatcherSnapshot, WatcherFailure> {
        let record = self.record_mut(watcher_id)?;
        match record.phase {
            WatcherLifecyclePhase::Terminal => {
                record.phase = WatcherLifecyclePhase::Joined;
                record.revision = record.revision.next();
                self.snapshot(watcher_id)
            }
            WatcherLifecyclePhase::Joined => {
                Err(WatcherFailure::new(WatcherFailureKind::AlreadyJoined))
            }
            WatcherLifecyclePhase::Accepted | WatcherLifecyclePhase::Running => {
                Err(WatcherFailure::new(WatcherFailureKind::InvalidTransition))
            }
        }
    }

    /// Stops and joins every non-joined watcher owned by this turn.
    ///
    /// Cleanup causes are restricted to cancel, timeout, stop, and failure.
    /// Successful completion cannot be assigned by bulk cleanup.
    pub fn stop_and_join_all(
        &mut self,
        cause: WatcherCleanupCause,
    ) -> Result<Vec<WatcherSnapshot>, WatcherFailure> {
        let terminal_cause = cause.terminal_cause();
        let ids = self.order.clone();
        let mut snapshots = Vec::with_capacity(ids.len());
        for id in ids {
            let phase = self.record(&id)?.phase;
            match phase {
                WatcherLifecyclePhase::Accepted | WatcherLifecyclePhase::Running => {
                    self.complete(&id, terminal_cause, None)?;
                    snapshots.push(self.join(&id)?);
                }
                WatcherLifecyclePhase::Terminal => {
                    snapshots.push(self.join(&id)?);
                }
                WatcherLifecyclePhase::Joined => {
                    snapshots.push(self.snapshot(&id)?);
                }
            }
        }
        Ok(snapshots)
    }

    /// Returns one watcher snapshot after ownership checks.
    pub fn inspect(
        &self,
        owning_turn: &WatcherOwningTurn,
        watcher_id: &WatcherId,
    ) -> Result<WatcherSnapshot, WatcherFailure> {
        self.require_owning_turn(owning_turn)?;
        self.snapshot(watcher_id)
    }

    /// Lists retained watchers in acceptance order for the owning turn.
    pub fn list(
        &self,
        owning_turn: &WatcherOwningTurn,
    ) -> Result<Vec<WatcherSnapshot>, WatcherFailure> {
        self.require_owning_turn(owning_turn)?;
        self.order.iter().map(|id| self.snapshot(id)).collect()
    }

    /// Returns the pure wait representation for one owned watcher.
    pub fn wait_representation(
        &self,
        owning_turn: &WatcherOwningTurn,
        watcher_id: &WatcherId,
    ) -> Result<WatcherWaitRepresentation, WatcherFailure> {
        self.require_owning_turn(owning_turn)?;
        let record = self.record(watcher_id)?;
        Ok(match record.phase {
            WatcherLifecyclePhase::Accepted | WatcherLifecyclePhase::Running => {
                WatcherWaitRepresentation::Pending
            }
            WatcherLifecyclePhase::Terminal => WatcherWaitRepresentation::TerminalUnjoined(
                record
                    .terminal_cause
                    .ok_or_else(|| WatcherFailure::new(WatcherFailureKind::InvalidTransition))?,
            ),
            WatcherLifecyclePhase::Joined => WatcherWaitRepresentation::Satisfied(
                record
                    .terminal_cause
                    .ok_or_else(|| WatcherFailure::new(WatcherFailureKind::InvalidTransition))?,
            ),
        })
    }

    /// Requires wait gating to be satisfied for one owned watcher.
    pub fn require_wait_satisfied(
        &self,
        owning_turn: &WatcherOwningTurn,
        watcher_id: &WatcherId,
    ) -> Result<WatcherTerminalCause, WatcherFailure> {
        match self.wait_representation(owning_turn, watcher_id)? {
            WatcherWaitRepresentation::Satisfied(cause) => Ok(cause),
            WatcherWaitRepresentation::Pending
            | WatcherWaitRepresentation::TerminalUnjoined(_)
            | WatcherWaitRepresentation::Cancelled
            | WatcherWaitRepresentation::DeadlineExceeded => {
                Err(WatcherFailure::new(WatcherFailureKind::WaitNotSatisfied))
            }
        }
    }

    /// Marks an outstanding wait as cancelled without mutating watcher state.
    pub fn represent_wait_cancelled(&self) -> WatcherWaitRepresentation {
        WatcherWaitRepresentation::Cancelled
    }

    /// Marks an outstanding wait as deadline-exceeded without mutating watcher state.
    pub fn represent_wait_deadline_exceeded(&self) -> WatcherWaitRepresentation {
        WatcherWaitRepresentation::DeadlineExceeded
    }

    /// Reports whether every owned watcher is terminal and joined.
    #[must_use]
    pub fn all_joined(&self) -> bool {
        self.records
            .values()
            .all(|record| record.phase == WatcherLifecyclePhase::Joined)
    }

    fn require_owning_turn(&self, owning_turn: &WatcherOwningTurn) -> Result<(), WatcherFailure> {
        if owning_turn == &self.owning_turn {
            Ok(())
        } else {
            Err(WatcherFailure::new(WatcherFailureKind::ForeignIdentity))
        }
    }

    fn record(&self, watcher_id: &WatcherId) -> Result<&WatcherRecord, WatcherFailure> {
        self.records
            .get(watcher_id)
            .ok_or_else(|| WatcherFailure::new(WatcherFailureKind::UnknownWatcher))
    }

    fn record_mut(&mut self, watcher_id: &WatcherId) -> Result<&mut WatcherRecord, WatcherFailure> {
        self.records
            .get_mut(watcher_id)
            .ok_or_else(|| WatcherFailure::new(WatcherFailureKind::UnknownWatcher))
    }

    fn snapshot(&self, watcher_id: &WatcherId) -> Result<WatcherSnapshot, WatcherFailure> {
        let record = self.record(watcher_id)?;
        Ok(WatcherSnapshot {
            watcher_id: watcher_id.clone(),
            owning_turn: self.owning_turn.clone(),
            activity_id: record.activity_id.clone(),
            phase: record.phase,
            terminal_cause: record.terminal_cause,
            revision: record.revision,
            summary: record.summary.clone(),
            accepted_by: record.accepted_by,
        })
    }
}
