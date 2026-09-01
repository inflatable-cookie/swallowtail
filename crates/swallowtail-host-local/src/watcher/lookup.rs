use super::{LocalWatcherEntry, LocalWatcherHostService};
use crate::output::failure;
use std::sync::Arc;
use swallowtail_core::{WatcherId, WatcherOwningTurn};
use swallowtail_runtime::{RuntimeFailure, RuntimeTurnId, WatcherSnapshot, WatcherWaitOptions};

use super::support::{entry_missing_failure, registry_failure, runtime_turn, turn_missing_failure};

impl LocalWatcherHostService {
    pub(super) fn inspect_now(
        &self,
        owning_turn: WatcherOwningTurn,
        watcher_id: WatcherId,
    ) -> Result<WatcherSnapshot, RuntimeFailure> {
        let state = self
            .state
            .lock()
            .expect("local watcher state lock poisoned");
        let turn = runtime_turn(&owning_turn)?;
        let turn_state = state.active.get(&turn).ok_or_else(|| {
            if state.is_retired(&turn) {
                super::support::turn_retired_failure()
            } else {
                turn_missing_failure()
            }
        })?;
        if turn_state.closed {
            return Err(failure(
                "swallowtail.local_watcher.turn_closed",
                "Watcher turn cleanup has already closed this turn",
            ));
        }
        turn_state
            .registry
            .inspect(&owning_turn, &watcher_id)
            .map_err(registry_failure)
    }

    pub(super) fn list_now(
        &self,
        owning_turn: WatcherOwningTurn,
    ) -> Result<Vec<WatcherSnapshot>, RuntimeFailure> {
        let state = self
            .state
            .lock()
            .expect("local watcher state lock poisoned");
        let turn = runtime_turn(&owning_turn)?;
        let turn_state = state.active.get(&turn).ok_or_else(|| {
            if state.is_retired(&turn) {
                super::support::turn_retired_failure()
            } else {
                turn_missing_failure()
            }
        })?;
        if turn_state.closed {
            return Err(failure(
                "swallowtail.local_watcher.turn_closed",
                "Watcher turn cleanup has already closed this turn",
            ));
        }
        turn_state
            .registry
            .list(&owning_turn)
            .map_err(registry_failure)
    }

    pub(super) fn lookup(
        &self,
        owning_turn: &WatcherOwningTurn,
        watcher_id: &WatcherId,
    ) -> Result<(RuntimeTurnId, Arc<LocalWatcherEntry>), RuntimeFailure> {
        let turn = runtime_turn(owning_turn)?;
        let state = self
            .state
            .lock()
            .expect("local watcher state lock poisoned");
        let turn_state = state.active.get(&turn).ok_or_else(|| {
            if state.is_retired(&turn) {
                super::support::turn_retired_failure()
            } else {
                turn_missing_failure()
            }
        })?;
        if turn_state.closed {
            return Err(failure(
                "swallowtail.local_watcher.turn_closed",
                "Watcher turn cleanup has already closed this turn",
            ));
        }
        turn_state
            .registry
            .inspect(owning_turn, watcher_id)
            .map_err(registry_failure)?;
        let entry = turn_state
            .entries
            .get(watcher_id)
            .cloned()
            .ok_or_else(entry_missing_failure)?;
        Ok((turn, entry))
    }

    pub(super) fn prepare_wait<'a>(
        &self,
        owning_turn: WatcherOwningTurn,
        watcher_id: WatcherId,
        options: WatcherWaitOptions<'a>,
    ) -> Result<super::wait::LocalWatcherWait<'a>, RuntimeFailure> {
        let (turn, entry) = self.lookup(&owning_turn, &watcher_id)?;
        Ok(super::wait::LocalWatcherWait::new(
            Arc::clone(&self.state),
            turn,
            owning_turn,
            watcher_id,
            entry,
            options,
        ))
    }
}
