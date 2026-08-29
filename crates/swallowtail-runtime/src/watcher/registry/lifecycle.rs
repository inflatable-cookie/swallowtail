//! Lifecycle transitions for the pure watcher registry.

use super::WatcherRegistry;
use super::failure::{WatcherFailure, WatcherFailureKind};
use super::records::{
    WatcherRecord, WatcherSnapshot, WatcherStopAcknowledgement, WatcherWaitRepresentation,
};
use crate::ActivityId;
use swallowtail_core::{
    WatcherCleanupCause, WatcherId, WatcherLifecyclePhase, WatcherOperationData, WatcherOwningTurn,
    WatcherRequester, WatcherRevision, WatcherSummary, WatcherTerminalCause,
};

impl WatcherRegistry {
    /// Accepts a start request and retains an opaque watcher identity.
    ///
    /// Acceptance does not start host work. Allocated ids embed the owning-turn
    /// key so a stale id from another turn cannot alias later work.
    pub fn accept_start(
        &mut self,
        requester: WatcherRequester,
        _operation_data: WatcherOperationData,
    ) -> Result<WatcherSnapshot, WatcherFailure> {
        if self.records.len() >= self.maximum_watchers {
            return Err(WatcherFailure::new(WatcherFailureKind::CapacityExceeded));
        }
        let sequence = self.next_sequence;
        self.next_sequence = self.next_sequence.saturating_add(1);
        let watcher_id = WatcherId::new(format!(
            "{}/{}",
            self.owning_turn.as_str(),
            self.watcher_id(sequence)
        ))
        .map_err(|_| WatcherFailure::new(WatcherFailureKind::InvalidTransition))?;
        let activity_id = ActivityId::new(format!(
            "{}/{}",
            self.owning_turn.as_str(),
            self.activity_id(sequence)
        ))
        .map_err(|_| WatcherFailure::new(WatcherFailureKind::InvalidTransition))?;
        let record = WatcherRecord {
            activity_id,
            phase: WatcherLifecyclePhase::Accepted,
            terminal_cause: None,
            revision: WatcherRevision::initial(),
            summary: None,
            accepted_by: requester,
        };
        self.order.push(watcher_id.clone());
        self.records.insert(watcher_id.clone(), record);
        self.snapshot(&watcher_id)
    }

    /// Rejects a start before the host returns its watcher identity.
    ///
    /// This rollback is for a host that cannot finish binding the already-
    /// admitted operation. The host may have briefly marked the record
    /// running, but no caller can observe that record until `accept_start`
    /// succeeds. It never removes a terminal or joined watcher.
    pub fn reject_start(&mut self, watcher_id: &WatcherId) -> Result<(), WatcherFailure> {
        match self.record(watcher_id)?.phase {
            WatcherLifecyclePhase::Accepted | WatcherLifecyclePhase::Running => {}
            WatcherLifecyclePhase::Terminal | WatcherLifecyclePhase::Joined => {
                return Err(WatcherFailure::new(WatcherFailureKind::InvalidTransition));
            }
        }
        self.records.remove(watcher_id);
        self.order.retain(|current| current != watcher_id);
        Ok(())
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
}
