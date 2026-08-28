use super::{LocalWatcherEntry, LocalWatcherHostService, LocalWatcherTurn};
use crate::output::failure;
use futures_executor::block_on;
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::sync::{Arc, Mutex};
use swallowtail_core::{WatcherId, WatcherOperationData, WatcherOwningTurn, WatcherRequester};
use swallowtail_runtime::{
    ProcessService, RuntimeFailure, RuntimeTurnId, ScopedTaskService, WatcherRegistry,
    WatcherSnapshot, WatcherWaitRepresentation,
};

use super::process;
use super::support::{
    entry_missing_failure, registry_failure, runtime_turn, turn_missing_failure, watcher_scope,
};

impl LocalWatcherHostService {
    pub(super) fn accept_start_now(
        &self,
        turn: RuntimeTurnId,
        requester: WatcherRequester,
        operation_data: WatcherOperationData,
    ) -> Result<WatcherSnapshot, RuntimeFailure> {
        let request = self
            .process_host
            .approvals
            .watcher_operations
            .get(&operation_data)
            .cloned()
            .ok_or_else(|| {
                failure(
                    "swallowtail.local_watcher.operation_not_approved",
                    "Watcher operation data is not approved by the local host",
                )
            })?;
        let scope = watcher_scope(&turn)?;
        let mut state = self
            .state
            .lock()
            .expect("local watcher state lock poisoned");
        let turn_state = match state.entry(turn.clone()) {
            Entry::Occupied(entry) => {
                if entry.get().closed {
                    return Err(failure(
                        "swallowtail.local_watcher.turn_closed",
                        "Watcher turn cleanup has already closed this turn",
                    ));
                }
                entry.into_mut()
            }
            Entry::Vacant(entry) => {
                let registry =
                    WatcherRegistry::new(turn.clone(), self.capacity).map_err(registry_failure)?;
                entry.insert(LocalWatcherTurn {
                    registry,
                    entries: BTreeMap::new(),
                    closed: false,
                })
            }
        };

        let accepted = turn_state
            .registry
            .accept_start(requester, operation_data)
            .map_err(registry_failure)?;
        let watcher_id = accepted.watcher_id().clone();
        let process = match block_on(self.process_host.start(scope.clone(), request)) {
            Ok(process) => Arc::from(process),
            Err(error) => {
                let _ = turn_state.registry.reject_start(&watcher_id);
                return Err(error);
            }
        };
        if let Err(error) = turn_state.registry.mark_running(&watcher_id) {
            super::support::cleanup_process(&process);
            let _ = turn_state.registry.reject_start(&watcher_id);
            return Err(registry_failure(error));
        }

        let monitor_state = Arc::clone(&self.state);
        let monitor_turn = turn.clone();
        let monitor_id = watcher_id.clone();
        let monitor_process = Arc::clone(&process);
        let task = match self.task_service.spawn(
            scope,
            Box::pin(process::monitor_watcher(
                monitor_state,
                monitor_turn,
                monitor_id,
                monitor_process,
            )),
        ) {
            Ok(task) => task,
            Err(error) => {
                super::support::cleanup_process(&process);
                let _ = turn_state.registry.complete(
                    &watcher_id,
                    swallowtail_core::WatcherTerminalCause::Failed,
                    Some(super::support::summary("failed")),
                );
                let _ = turn_state.registry.join(&watcher_id);
                return Err(error);
            }
        };
        turn_state.entries.insert(
            watcher_id,
            Arc::new(LocalWatcherEntry {
                process,
                task: Mutex::new(Some(task)),
                join_lock: Mutex::new(()),
                joined: std::sync::atomic::AtomicBool::new(false),
            }),
        );
        turn_state
            .registry
            .inspect(turn_state.registry.owning_turn(), accepted.watcher_id())
            .map_err(registry_failure)
    }

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
        state
            .get(&turn)
            .ok_or_else(turn_missing_failure)?
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
        state
            .get(&turn)
            .ok_or_else(turn_missing_failure)?
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
        let turn_state = state.get(&turn).ok_or_else(turn_missing_failure)?;
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

    pub(super) fn wait_now(
        &self,
        owning_turn: WatcherOwningTurn,
        watcher_id: WatcherId,
    ) -> Result<WatcherWaitRepresentation, RuntimeFailure> {
        let (turn, entry) = self.lookup(&owning_turn, &watcher_id)?;
        self.join_entry(&turn, &watcher_id, &entry)?;
        let state = self
            .state
            .lock()
            .expect("local watcher state lock poisoned");
        let representation = state
            .get(&turn)
            .ok_or_else(turn_missing_failure)?
            .registry
            .wait_representation(&owning_turn, &watcher_id)
            .map_err(registry_failure)?;
        if matches!(representation, WatcherWaitRepresentation::Satisfied(_)) {
            Ok(representation)
        } else {
            Err(failure(
                "swallowtail.local_watcher.wait_not_joined",
                "Local watcher cleanup did not reach joined truth",
            ))
        }
    }
}
