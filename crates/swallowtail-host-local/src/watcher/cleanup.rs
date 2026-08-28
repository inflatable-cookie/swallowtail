use super::{LocalWatcherEntry, LocalWatcherHostService};
use futures_executor::block_on;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use swallowtail_core::{
    WatcherCleanupCause, WatcherId, WatcherLifecyclePhase, WatcherTerminalCause,
};
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
            let turn_state = state.get_mut(&turn).ok_or_else(turn_missing_failure)?;
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
            let turn_state = state.get_mut(&turn).ok_or_else(turn_missing_failure)?;
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
                .get(&turn)
                .ok_or_else(turn_missing_failure)?
                .registry
                .list(&owning_turn)
                .map_err(registry_failure)?
        };
        let outcome = cleanup_error.map_or(CleanupOutcome::Clean, |error| {
            CleanupOutcome::Failed(error.diagnostic().clone())
        });
        Ok((snapshots, outcome))
    }

    pub(super) fn join_entry(
        &self,
        turn: &RuntimeTurnId,
        watcher_id: &WatcherId,
        entry: &Arc<LocalWatcherEntry>,
    ) -> Result<(), RuntimeFailure> {
        let _join_guard = entry
            .join_lock
            .lock()
            .expect("local watcher join lock poisoned");
        let mut task_error = None;
        if !entry.joined.load(Ordering::Acquire) {
            let task = entry
                .task
                .lock()
                .expect("local watcher task lock poisoned")
                .take();
            if let Some(task) = task
                && let Err(error) = block_on(task.join())
            {
                task_error = Some(error);
            }
            let mut process_result = block_on(entry.process.wait());
            if process_result.is_err() {
                let _ = block_on(entry.process.force_stop());
                process_result = block_on(entry.process.wait());
            }
            process_result?;
            entry.joined.store(true, Ordering::Release);

            let owning_turn = owning_turn(turn)?;
            let mut state = self
                .state
                .lock()
                .expect("local watcher state lock poisoned");
            let turn_state = state.get_mut(turn).ok_or_else(turn_missing_failure)?;
            let snapshot = turn_state
                .registry
                .inspect(&owning_turn, watcher_id)
                .map_err(registry_failure)?;
            match snapshot.phase() {
                WatcherLifecyclePhase::Accepted | WatcherLifecyclePhase::Running => {
                    turn_state
                        .registry
                        .complete(
                            watcher_id,
                            WatcherTerminalCause::Failed,
                            Some(summary("failed")),
                        )
                        .map_err(registry_failure)?;
                    let _ = turn_state.registry.join(watcher_id);
                }
                WatcherLifecyclePhase::Terminal => {
                    turn_state
                        .registry
                        .join(watcher_id)
                        .map_err(registry_failure)?;
                }
                WatcherLifecyclePhase::Joined => {}
            }
        }
        if let Some(error) = task_error {
            return Err(error);
        }
        Ok(())
    }
}
