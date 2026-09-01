use super::{LocalWatcherEntry, LocalWatcherHostService, LocalWatcherTurn};
use crate::output::failure;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use swallowtail_core::{WatcherId, WatcherOperationData, WatcherOwningTurn, WatcherRequester};
use swallowtail_runtime::{
    ProcessHandle, ProcessService, RuntimeFailure, RuntimeTurnId, WatcherRegistry, WatcherSnapshot,
    WatcherWaitOptions,
};

use super::support::{
    cleanup_process, entry_missing_failure, registry_failure, runtime_turn, turn_missing_failure,
    watcher_scope,
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
        if state.is_retired(&turn) {
            return Err(super::support::turn_retired_failure());
        }
        if let Some(turn_state) = state.active.get(&turn)
            && turn_state.closed
        {
            return Err(failure(
                "swallowtail.local_watcher.turn_closed",
                "Watcher turn cleanup has already closed this turn",
            ));
        }
        if !state.active.contains_key(&turn) {
            let namespace = state.next_namespace;
            state.next_namespace = state.next_namespace.checked_add(1).ok_or_else(|| {
                failure(
                    "swallowtail.local_watcher.identity_exhausted",
                    "Local watcher identity namespace is exhausted",
                )
            })?;
            let registry =
                WatcherRegistry::new_with_namespace(turn.clone(), self.capacity, namespace)
                    .map_err(registry_failure)?;
            state.active.insert(
                turn.clone(),
                LocalWatcherTurn {
                    registry,
                    entries: BTreeMap::new(),
                    closed: false,
                },
            );
        }
        let accepted = {
            let turn_state = state
                .active
                .get_mut(&turn)
                .expect("local watcher turn was inserted before acceptance");
            turn_state
                .registry
                .accept_start(requester, operation_data)
                .map_err(registry_failure)?
        };
        state.publish(&turn, accepted.clone())?;
        let watcher_id = accepted.watcher_id().clone();
        let process =
            match super::support::drive_future(self.process_host.start(scope.clone(), request)) {
                Ok(process) => Arc::<dyn ProcessHandle>::from(process),
                Err(error) => {
                    if let Some(turn_state) = state.active.get_mut(&turn) {
                        let _ = turn_state.registry.reject_start(&watcher_id);
                    }
                    state.remove_empty_turn(&turn);
                    return Err(error);
                }
            };
        if let Err(error) = {
            let turn_state = state
                .active
                .get_mut(&turn)
                .expect("local watcher turn remains during start binding");
            turn_state.registry.mark_running(&watcher_id)
        } {
            return self.rollback_unbound_start(
                &mut state,
                &turn,
                &watcher_id,
                process,
                registry_failure(error),
            );
        }

        let entry = Arc::new(LocalWatcherEntry {
            process,
            task: Mutex::new(None),
            join_lock: Mutex::new(()),
            joined: std::sync::atomic::AtomicBool::new(false),
            join_error: Mutex::new(None),
            join_signal: super::JoinSignal::default(),
        });
        let monitor_state = Arc::clone(&self.state);
        let monitor_turn = turn.clone();
        let monitor_id = watcher_id.clone();
        let monitor_entry = Arc::clone(&entry);
        let task = match self.task_service.spawn(
            scope,
            Box::pin(super::process::monitor_watcher(
                monitor_state,
                monitor_turn,
                monitor_id,
                monitor_entry,
            )),
        ) {
            Ok(task) => task,
            Err(error) => {
                let process = Arc::clone(&entry.process);
                return self.rollback_unbound_start(&mut state, &turn, &watcher_id, process, error);
            }
        };
        *entry.task.lock().expect("local watcher task lock poisoned") = Some(task);
        let running = {
            let turn_state = state
                .active
                .get_mut(&turn)
                .expect("local watcher turn remains after monitor spawn");
            turn_state.entries.insert(watcher_id.clone(), entry);
            turn_state
                .registry
                .inspect(turn_state.registry.owning_turn(), &watcher_id)
                .map_err(registry_failure)?
        };
        state.publish(&turn, running.clone())?;
        Ok(running)
    }

    fn rollback_unbound_start(
        &self,
        state: &mut super::LocalWatcherState,
        turn: &RuntimeTurnId,
        watcher_id: &WatcherId,
        process: Arc<dyn ProcessHandle>,
        original_error: RuntimeFailure,
    ) -> Result<WatcherSnapshot, RuntimeFailure> {
        match cleanup_process(&process) {
            Ok(()) => {
                if let Some(turn_state) = state.active.get_mut(turn) {
                    let _ = turn_state.registry.reject_start(watcher_id);
                }
                state.remove_empty_turn(turn);
                Err(original_error)
            }
            Err(cleanup_error) => {
                let turn_state = state
                    .active
                    .get_mut(turn)
                    .expect("watcher turn remains while process cleanup failed");
                let entry = Arc::new(LocalWatcherEntry {
                    process,
                    task: Mutex::new(None),
                    join_lock: Mutex::new(()),
                    joined: std::sync::atomic::AtomicBool::new(false),
                    join_error: Mutex::new(Some(cleanup_error.clone())),
                    join_signal: super::JoinSignal::default(),
                });
                turn_state.entries.insert(watcher_id.clone(), entry);
                Err(cleanup_error)
            }
        }
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
