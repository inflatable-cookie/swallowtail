mod accept;
mod cleanup;
mod join;
mod process;
mod support;
#[cfg(test)]
mod tests;
mod wait;

use crate::host::LocalProcessHost;
use crate::task::LocalScopedTaskService;
use futures_executor::block_on;
use std::collections::{BTreeMap, VecDeque};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::Waker;
use swallowtail_core::{
    WatcherCleanupCause, WatcherId, WatcherOperationData, WatcherOwningTurn, WatcherRequester,
};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, JoinedTask, ProcessHandle, RuntimeFailure, RuntimeTurnId,
    ScopedTaskService, WatcherHostService, WatcherSnapshot, WatcherStopAcknowledgement,
    WatcherWaitOptions, WatcherWaitRepresentation,
};

pub(super) const MAX_RETIRED_TURNS: usize = 64;

pub(crate) struct LocalWatcherHostService {
    process_host: Arc<LocalProcessHost>,
    task_service: Arc<dyn ScopedTaskService>,
    state: Arc<Mutex<LocalWatcherState>>,
    capacity: usize,
}

pub(super) struct LocalWatcherState {
    pub(super) active: BTreeMap<RuntimeTurnId, LocalWatcherTurn>,
    pub(super) retired: VecDeque<RuntimeTurnId>,
    pub(super) next_namespace: u64,
}

pub(super) struct LocalWatcherTurn {
    registry: swallowtail_runtime::WatcherRegistry,
    entries: BTreeMap<WatcherId, Arc<LocalWatcherEntry>>,
    closed: bool,
}

struct LocalWatcherEntry {
    process: Arc<dyn ProcessHandle>,
    task: Mutex<Option<Box<dyn JoinedTask>>>,
    join_lock: Mutex<()>,
    joined: AtomicBool,
    join_error: Mutex<Option<RuntimeFailure>>,
    join_signal: JoinSignal,
}

impl Default for LocalWatcherState {
    fn default() -> Self {
        Self {
            active: BTreeMap::new(),
            retired: VecDeque::new(),
            next_namespace: 1,
        }
    }
}

impl LocalWatcherState {
    pub(super) fn is_retired(&self, turn: &RuntimeTurnId) -> bool {
        self.retired.iter().any(|retired| retired == turn)
    }

    pub(super) fn retire(&mut self, turn: &RuntimeTurnId) {
        self.active.remove(turn);
        if !self.is_retired(turn) {
            self.retired.push_back(turn.clone());
        }
        while self.retired.len() > MAX_RETIRED_TURNS {
            self.retired.pop_front();
        }
    }

    pub(super) fn remove_empty_turn(&mut self, turn: &RuntimeTurnId) {
        if self.active.get(turn).is_some_and(|turn_state| {
            turn_state.registry.is_empty() && turn_state.entries.is_empty()
        }) {
            self.active.remove(turn);
        }
    }
}

impl LocalWatcherEntry {
    pub(super) fn record_join_error(&self, error: RuntimeFailure) {
        let mut join_error = self
            .join_error
            .lock()
            .expect("local watcher join error lock poisoned");
        if join_error.is_none() {
            *join_error = Some(error);
        }
        drop(join_error);
        self.join_signal.notify();
    }

    pub(super) fn join_error(&self) -> Option<RuntimeFailure> {
        self.join_error
            .lock()
            .expect("local watcher join error lock poisoned")
            .clone()
    }
}

#[derive(Default)]
pub(super) struct JoinSignal {
    finished: AtomicBool,
    wakers: Mutex<Vec<Waker>>,
}

impl JoinSignal {
    pub(super) fn notify(&self) {
        self.finished.store(true, Ordering::Release);
        let wakers = self
            .wakers
            .lock()
            .expect("local watcher waker lock poisoned")
            .drain(..)
            .collect::<Vec<_>>();
        for waker in wakers {
            waker.wake();
        }
    }

    pub(super) fn register(&self, waker: &Waker) {
        if self.finished.load(Ordering::Acquire) {
            waker.wake_by_ref();
            return;
        }
        let mut registered = self
            .wakers
            .lock()
            .expect("local watcher waker lock poisoned");
        if !registered.iter().any(|current| current.will_wake(waker)) {
            registered.push(waker.clone());
        }
        if self.finished.load(Ordering::Acquire) {
            let wakers = registered.drain(..).collect::<Vec<_>>();
            drop(registered);
            for waker in wakers {
                waker.wake();
            }
        }
    }
}

impl LocalWatcherHostService {
    pub(crate) fn new(
        process_host: Arc<LocalProcessHost>,
        task_service: Arc<LocalScopedTaskService>,
        capacity: usize,
    ) -> Self {
        Self::new_with_task_service(process_host, task_service, capacity)
    }

    pub(crate) fn new_with_task_service(
        process_host: Arc<LocalProcessHost>,
        task_service: Arc<dyn ScopedTaskService>,
        capacity: usize,
    ) -> Self {
        Self {
            process_host,
            task_service,
            state: Arc::new(Mutex::new(LocalWatcherState::default())),
            capacity,
        }
    }
}

impl WatcherHostService for LocalWatcherHostService {
    fn accept_start(
        &self,
        turn: RuntimeTurnId,
        requester: WatcherRequester,
        operation_data: WatcherOperationData,
    ) -> BoxFuture<'_, Result<WatcherSnapshot, RuntimeFailure>> {
        let result = self.accept_start_now(turn, requester, operation_data);
        Box::pin(async move { result })
    }

    fn inspect(
        &self,
        owning_turn: WatcherOwningTurn,
        watcher_id: WatcherId,
    ) -> BoxFuture<'_, Result<WatcherSnapshot, RuntimeFailure>> {
        let result = self.inspect_now(owning_turn, watcher_id);
        Box::pin(async move { result })
    }

    fn list(
        &self,
        owning_turn: WatcherOwningTurn,
    ) -> BoxFuture<'_, Result<Vec<WatcherSnapshot>, RuntimeFailure>> {
        let result = self.list_now(owning_turn);
        Box::pin(async move { result })
    }

    fn wait<'a>(
        &'a self,
        owning_turn: WatcherOwningTurn,
        watcher_id: WatcherId,
        options: WatcherWaitOptions<'a>,
    ) -> BoxFuture<'a, Result<WatcherWaitRepresentation, RuntimeFailure>> {
        match self.prepare_wait(owning_turn, watcher_id, options) {
            Ok(wait) => Box::pin(wait),
            Err(error) => Box::pin(async move { Err(error) }),
        }
    }

    fn request_stop(
        &self,
        owning_turn: WatcherOwningTurn,
        watcher_id: WatcherId,
    ) -> BoxFuture<'_, Result<(WatcherStopAcknowledgement, WatcherSnapshot), RuntimeFailure>> {
        let result = self.request_stop_now(owning_turn, watcher_id);
        Box::pin(async move { result })
    }

    fn stop_and_join_all(
        &self,
        turn: RuntimeTurnId,
        cause: WatcherCleanupCause,
    ) -> BoxFuture<'_, Result<(Vec<WatcherSnapshot>, CleanupOutcome), RuntimeFailure>> {
        let result = self.stop_and_join_all_now(turn, cause);
        Box::pin(async move { result })
    }

    fn finalize_turn(
        &self,
        turn: RuntimeTurnId,
    ) -> BoxFuture<'_, Result<CleanupOutcome, RuntimeFailure>> {
        let result = self.finalize_turn_now(turn);
        Box::pin(async move { result })
    }
}

impl Drop for LocalWatcherEntry {
    fn drop(&mut self) {
        if !self.joined.load(Ordering::Acquire) {
            let _ = block_on(self.process.force_stop());
        }
    }
}
