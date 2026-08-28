mod accept;
mod cleanup;
mod process;
mod support;

use crate::host::LocalProcessHost;
use crate::task::LocalScopedTaskService;
use futures_executor::block_on;
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use swallowtail_core::{
    WatcherCleanupCause, WatcherId, WatcherOperationData, WatcherOwningTurn, WatcherRequester,
};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, JoinedTask, ProcessHandle, RuntimeFailure, RuntimeTurnId,
    WatcherHostService, WatcherSnapshot, WatcherStopAcknowledgement, WatcherWaitRepresentation,
};

pub(crate) struct LocalWatcherHostService {
    process_host: Arc<LocalProcessHost>,
    task_service: Arc<LocalScopedTaskService>,
    state: Arc<Mutex<BTreeMap<RuntimeTurnId, LocalWatcherTurn>>>,
    capacity: usize,
}

struct LocalWatcherTurn {
    registry: swallowtail_runtime::WatcherRegistry,
    entries: BTreeMap<WatcherId, Arc<LocalWatcherEntry>>,
    closed: bool,
}

struct LocalWatcherEntry {
    process: Arc<dyn ProcessHandle>,
    task: Mutex<Option<Box<dyn JoinedTask>>>,
    join_lock: Mutex<()>,
    joined: AtomicBool,
}

impl LocalWatcherHostService {
    pub(crate) fn new(
        process_host: Arc<LocalProcessHost>,
        task_service: Arc<LocalScopedTaskService>,
        capacity: usize,
    ) -> Self {
        Self {
            process_host,
            task_service,
            state: Arc::new(Mutex::new(BTreeMap::new())),
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

    fn wait(
        &self,
        owning_turn: WatcherOwningTurn,
        watcher_id: WatcherId,
    ) -> BoxFuture<'_, Result<WatcherWaitRepresentation, RuntimeFailure>> {
        let result = self.wait_now(owning_turn, watcher_id);
        Box::pin(async move { result })
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
}

impl Drop for LocalWatcherEntry {
    fn drop(&mut self) {
        if !self.joined.load(Ordering::Acquire) {
            let _ = block_on(self.process.force_stop());
        }
    }
}
