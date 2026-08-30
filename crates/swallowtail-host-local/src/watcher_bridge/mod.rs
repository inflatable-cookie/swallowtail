//! Local Contract 060 watcher HTTP bridge.

mod bearer;
mod http;
mod listener;
mod protocol;
mod state;

use crate::output::failure;
use bearer::generate_bearer;
use listener::{bind_loopback, endpoint_url, spawn_accept, wake_accept};
use state::{BridgeRegistry, Gate, LiveLease, closed_failure, foreign_failure};
use std::sync::{Arc, Condvar, Mutex};
use swallowtail_core::{CancellationScope, ExecutionHostId, WatcherCleanupCause};
use swallowtail_runtime::{
    BoxFuture, CancellationControl, CleanupOutcome, ImmediateCancellation, RuntimeFailure,
    RuntimeTurnId, WatcherBridgeAdmission, WatcherBridgeBearer, WatcherBridgeCompletionState,
    WatcherBridgeEndpoint, WatcherBridgeGeneration, WatcherBridgeHostService, WatcherBridgeLease,
    WatcherBridgeOpenRequest, WatcherHostService,
};

pub(crate) struct LocalWatcherBridgeHostService {
    execution_host_id: ExecutionHostId,
    watcher: Arc<dyn WatcherHostService>,
    state: Arc<Mutex<BridgeRegistry>>,
}

impl LocalWatcherBridgeHostService {
    pub(crate) fn new(
        execution_host_id: ExecutionHostId,
        watcher: Arc<dyn WatcherHostService>,
    ) -> Self {
        Self {
            execution_host_id,
            watcher,
            state: Arc::new(Mutex::new(BridgeRegistry::default())),
        }
    }

    fn open_now(
        &self,
        request: WatcherBridgeOpenRequest,
    ) -> Result<WatcherBridgeLease, RuntimeFailure> {
        if request.scope().as_str().is_empty() {
            return Err(identity_failure());
        }
        let (listener, addr) = bind_loopback()?;
        let bearer = generate_bearer()?;
        let endpoint = endpoint_url(addr);
        let generation = {
            let mut registry = self
                .state
                .lock()
                .expect("watcher bridge registry lock poisoned");
            if registry.by_turn.contains_key(request.turn()) {
                return Err(failure(
                    "swallowtail.watcher_bridge.already_open",
                    "Watcher bridge already has an open lease for this turn",
                ));
            }
            let generation = WatcherBridgeGeneration::new(registry.next_generation)
                .ok_or_else(identity_failure)?;
            registry.next_generation = registry.next_generation.saturating_add(1);
            registry.by_turn.insert(request.turn().clone(), generation);
            generation
        };
        let live = Arc::new(LiveLease {
            execution_host_id: self.execution_host_id.clone(),
            scope: request.scope().clone(),
            turn: request.turn().clone(),
            generation,
            bind_addr: addr,
            bearer: bearer.clone(),
            watcher: Arc::clone(&self.watcher),
            closed: std::sync::atomic::AtomicBool::new(false),
            in_flight: std::sync::atomic::AtomicUsize::new(0),
            connection_count: std::sync::atomic::AtomicUsize::new(0),
            cancel: ImmediateCancellation::new(CancellationScope::ActiveTurn),
            gate: Mutex::new(Gate {
                admission: WatcherBridgeAdmission::Open,
                creating: 0,
            }),
            creating_changed: Condvar::new(),
            seen_ids: Mutex::new(std::collections::BTreeSet::new()),
            connections: Mutex::new(Vec::new()),
            accept_thread: Mutex::new(None),
        });
        if let Err(error) = spawn_accept(Arc::clone(&live), listener) {
            self.forget_generation(request.turn(), generation);
            return Err(error);
        }
        self.state
            .lock()
            .expect("watcher bridge registry lock poisoned")
            .live
            .insert(generation, Arc::clone(&live));
        let close_state = Arc::clone(&self.state);
        let close_watcher = Arc::clone(&self.watcher);
        let close_turn = request.turn().clone();
        Ok(WatcherBridgeLease::new(
            self.execution_host_id.clone(),
            request.scope().clone(),
            request.turn().clone(),
            generation,
            WatcherBridgeEndpoint::new(endpoint).map_err(|_| identity_failure())?,
            WatcherBridgeBearer::new(bearer).map_err(|_| identity_failure())?,
        )
        .with_defensive_cleanup(move || {
            shutdown_generation(
                close_state,
                close_watcher,
                close_turn,
                generation,
                WatcherCleanupCause::Cancelled,
            );
        }))
    }

    fn forget_generation(&self, turn: &RuntimeTurnId, generation: WatcherBridgeGeneration) {
        let mut registry = self
            .state
            .lock()
            .expect("watcher bridge registry lock poisoned");
        registry.by_turn.remove(turn);
        registry.live.remove(&generation);
    }

    fn live_for(&self, lease: &WatcherBridgeLease) -> Result<Arc<LiveLease>, RuntimeFailure> {
        if lease.execution_host_id() != &self.execution_host_id {
            return Err(foreign_failure());
        }
        let registry = self
            .state
            .lock()
            .expect("watcher bridge registry lock poisoned");
        let live = registry
            .live
            .get(&lease.generation())
            .cloned()
            .ok_or_else(closed_failure)?;
        live.matches(
            lease.execution_host_id(),
            lease.scope(),
            lease.turn(),
            lease.generation(),
        )?;
        Ok(live)
    }
}

impl WatcherBridgeHostService for LocalWatcherBridgeHostService {
    fn open(
        &self,
        request: WatcherBridgeOpenRequest,
    ) -> BoxFuture<'_, Result<WatcherBridgeLease, RuntimeFailure>> {
        Box::pin(async move { self.open_now(request) })
    }

    fn completion_gate(
        &self,
        lease: &WatcherBridgeLease,
    ) -> BoxFuture<'_, Result<WatcherBridgeCompletionState, RuntimeFailure>> {
        let live = self.live_for(lease);
        Box::pin(async move { live?.completion_gate() })
    }

    fn close(
        &self,
        mut lease: WatcherBridgeLease,
        cause: WatcherCleanupCause,
    ) -> BoxFuture<'_, Result<CleanupOutcome, RuntimeFailure>> {
        let _ = lease.take_defensive_cleanup();
        let state = Arc::clone(&self.state);
        let watcher = Arc::clone(&self.watcher);
        let turn = lease.turn().clone();
        let generation = lease.generation();
        shutdown_generation(state, watcher, turn, generation, cause);
        Box::pin(async move { Ok(CleanupOutcome::Clean) })
    }
}

fn shutdown_generation(
    state: Arc<Mutex<BridgeRegistry>>,
    watcher: Arc<dyn WatcherHostService>,
    turn: RuntimeTurnId,
    generation: WatcherBridgeGeneration,
    cause: WatcherCleanupCause,
) {
    let live = {
        let mut registry = state.lock().expect("watcher bridge registry lock poisoned");
        registry.by_turn.remove(&turn);
        registry.live.remove(&generation)
    };
    let Some(live) = live else {
        return;
    };
    if live.closed.swap(true, std::sync::atomic::Ordering::SeqCst) {
        return;
    }
    {
        let mut gate = live.gate.lock().expect("watcher bridge gate lock poisoned");
        gate.admission = WatcherBridgeAdmission::Closed;
    }
    drop(live.cancel.request());
    wake_accept(live.bind_addr);
    let accept = live
        .accept_thread
        .lock()
        .expect("watcher bridge accept thread lock poisoned")
        .take();
    if let Some(accept) = accept {
        let _ = accept.join();
    }
    let connections = std::mem::take(
        &mut *live
            .connections
            .lock()
            .expect("watcher bridge connection lock poisoned"),
    );
    for connection in connections {
        let _ = connection.join();
    }
    drop(watcher.stop_and_join_all(turn, cause));
}

fn identity_failure() -> RuntimeFailure {
    failure(
        "swallowtail.watcher_bridge.identity_rejected",
        "Watcher bridge rejected a required identity",
    )
}
