use super::LocalWatcherHostService;
use crate::output::failure;
use std::sync::atomic::Ordering;
use swallowtail_core::{WatcherCleanupCause, WatcherId, WatcherLifecyclePhase};
use swallowtail_runtime::{
    CleanupOutcome, RuntimeFailure, RuntimeTurnId, WatcherStopAcknowledgement,
};

use super::support::{
    entry_missing_failure, owning_turn, registry_failure, request_process_stop, summary,
    turn_missing_failure,
};

impl LocalWatcherHostService {
    pub(super) fn request_stop_now(
        &self,
        owning_turn: swallowtail_core::WatcherOwningTurn,
        watcher_id: WatcherId,
    ) -> Result<(WatcherStopAcknowledgement, super::WatcherSnapshot), RuntimeFailure> {
        let (entry, acknowledgement, snapshot) = {
            let turn = super::support::runtime_turn(&owning_turn)?;
            let mut state = self
                .state
                .lock()
                .expect("local watcher state lock poisoned");
            let turn_state = if state.active.contains_key(&turn) {
                state
                    .active
                    .get_mut(&turn)
                    .expect("active watcher turn was retained")
            } else {
                return Err(if state.is_retired(&turn) {
                    super::support::turn_retired_failure()
                } else {
                    turn_missing_failure()
                });
            };
            if turn_state.closed {
                return Err(failure(
                    "swallowtail.local_watcher.turn_closed",
                    "Watcher turn cleanup has already closed this turn",
                ));
            }
            let (acknowledgement, snapshot) = turn_state
                .registry
                .request_stop(&owning_turn, &watcher_id)
                .map_err(registry_failure)?;
            let entry = turn_state
                .entries
                .get(&watcher_id)
                .cloned()
                .ok_or_else(entry_missing_failure)?;
            (entry, acknowledgement, snapshot)
        };
        if matches!(acknowledgement, WatcherStopAcknowledgement::Stopped) {
            request_process_stop(&entry.process)?;
        }
        Ok((acknowledgement, snapshot))
    }

    pub(super) fn stop_and_join_all_now(
        &self,
        turn: RuntimeTurnId,
        cause: WatcherCleanupCause,
    ) -> Result<(Vec<super::WatcherSnapshot>, CleanupOutcome), RuntimeFailure> {
        let owning_turn = owning_turn(&turn)?;
        let entries = {
            let mut state = self
                .state
                .lock()
                .expect("local watcher state lock poisoned");
            let turn_state = if state.active.contains_key(&turn) {
                state
                    .active
                    .get_mut(&turn)
                    .expect("active watcher turn was retained")
            } else {
                return Err(if state.is_retired(&turn) {
                    super::support::turn_retired_failure()
                } else {
                    turn_missing_failure()
                });
            };
            turn_state.closed = true;
            let snapshots = turn_state
                .registry
                .list(&owning_turn)
                .map_err(registry_failure)?;
            let mut entries = Vec::with_capacity(snapshots.len());
            for snapshot in snapshots {
                let needs_stop = matches!(
                    snapshot.phase(),
                    WatcherLifecyclePhase::Accepted | WatcherLifecyclePhase::Running
                );
                if needs_stop {
                    turn_state
                        .registry
                        .complete(
                            snapshot.watcher_id(),
                            cause.terminal_cause(),
                            Some(summary(cause.terminal_cause().as_str())),
                        )
                        .map_err(registry_failure)?;
                }
                let entry = turn_state
                    .entries
                    .get(snapshot.watcher_id())
                    .cloned()
                    .ok_or_else(entry_missing_failure)?;
                entries.push((snapshot.watcher_id().clone(), entry, needs_stop));
            }
            entries
        };

        let mut cleanup_error = None;
        for (_, entry, needs_stop) in &entries {
            if *needs_stop
                && let Err(error) = request_process_stop(&entry.process)
                && cleanup_error.is_none()
            {
                cleanup_error = Some(error);
            }
        }
        for (watcher_id, entry, _) in &entries {
            if let Err(error) = self.join_entry(&turn, watcher_id, entry)
                && cleanup_error.is_none()
            {
                cleanup_error = Some(error);
            }
        }

        let snapshots = {
            let state = self
                .state
                .lock()
                .expect("local watcher state lock poisoned");
            state
                .active
                .get(&turn)
                .ok_or_else(|| {
                    if state.is_retired(&turn) {
                        super::support::turn_retired_failure()
                    } else {
                        turn_missing_failure()
                    }
                })?
                .registry
                .list(&owning_turn)
                .map_err(registry_failure)?
        };
        let clean_cleanup = cleanup_error.is_none();
        let outcome = cleanup_error.map_or(CleanupOutcome::Clean, |error| {
            CleanupOutcome::Failed(error.diagnostic().clone())
        });
        if clean_cleanup
            && snapshots
                .iter()
                .all(|snapshot| snapshot.phase() == WatcherLifecyclePhase::Joined)
        {
            let mut state = self
                .state
                .lock()
                .expect("local watcher state lock poisoned");
            state.retire(&turn);
        }
        Ok((snapshots, outcome))
    }

    pub(super) fn finalize_turn_now(
        &self,
        turn: RuntimeTurnId,
    ) -> Result<CleanupOutcome, RuntimeFailure> {
        let mut state = self
            .state
            .lock()
            .expect("local watcher state lock poisoned");
        let Some(turn_state) = state.active.get(&turn) else {
            return Err(if state.is_retired(&turn) {
                super::support::turn_retired_failure()
            } else {
                turn_missing_failure()
            });
        };
        if turn_state.closed {
            return Err(failure(
                "swallowtail.local_watcher.turn_closed",
                "Watcher turn cleanup has already closed this turn",
            ));
        }
        if !turn_state.registry.all_joined()
            || turn_state
                .entries
                .values()
                .any(|entry| !entry.joined.load(Ordering::Acquire))
        {
            return Err(failure(
                "swallowtail.local_watcher.turn_not_joined",
                "Local watcher turn cannot retire before every watcher is joined",
            ));
        }
        state.retire(&turn);
        Ok(CleanupOutcome::Clean)
    }
}
