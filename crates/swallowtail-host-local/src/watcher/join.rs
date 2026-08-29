use super::{LocalWatcherEntry, LocalWatcherHostService};
use crate::output::failure;
use futures_executor::block_on;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use swallowtail_core::{WatcherId, WatcherLifecyclePhase, WatcherTerminalCause};
use swallowtail_runtime::{RuntimeFailure, RuntimeTurnId};

use super::support::{owning_turn, registry_failure, summary, turn_missing_failure};

impl LocalWatcherHostService {
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
        if let Some(error) = entry.join_error() {
            return Err(error);
        }
        if !entry.joined.load(Ordering::Acquire) {
            let task = entry
                .task
                .lock()
                .expect("local watcher task lock poisoned")
                .take();
            if let Some(task) = task {
                if let Err(error) = block_on(task.join()) {
                    entry.record_join_error(error.clone());
                    return Err(error);
                }
            } else {
                let error = failure(
                    "swallowtail.local_watcher.join_not_available",
                    "Local watcher task join is unavailable",
                );
                entry.record_join_error(error.clone());
                return Err(error);
            }
            let mut process_result = block_on(entry.process.wait());
            if process_result.is_err() {
                let _ = block_on(entry.lease.force_stop());
                process_result = block_on(entry.process.wait());
            }
            if let Err(error) = process_result {
                entry.record_join_error(error.clone());
                return Err(error);
            }
            if let Err(error) = block_on(entry.lease.prove_empty_and_join()) {
                entry.record_join_error(error.clone());
                return Err(error);
            }

            let owning_turn = owning_turn(turn)?;
            let mut state = self
                .state
                .lock()
                .expect("local watcher state lock poisoned");
            let turn_state = if state.active.contains_key(turn) {
                state
                    .active
                    .get_mut(turn)
                    .expect("active watcher turn was retained")
            } else {
                return Err(if state.is_retired(turn) {
                    super::support::turn_retired_failure()
                } else {
                    turn_missing_failure()
                });
            };
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
                    turn_state
                        .registry
                        .join(watcher_id)
                        .map_err(registry_failure)?;
                }
                WatcherLifecyclePhase::Terminal => {
                    turn_state
                        .registry
                        .join(watcher_id)
                        .map_err(registry_failure)?;
                }
                WatcherLifecyclePhase::Joined => {}
            }
            entry.joined.store(true, Ordering::Release);
            entry.join_signal.notify();
        }
        Ok(())
    }
}
