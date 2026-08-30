//! Local Contract 060 watcher HTTP bridge.

mod bearer;
mod close;
mod failure;
mod http;
mod listener;
mod proof;
mod protocol;
#[cfg(test)]
mod races;
mod state;

pub use proof::WatcherBridgeProofKind;

use crate::output::failure;
use bearer::generate_bearer;
use close::shutdown_live;
use failure::{closed_failure, foreign_failure, identity_failure};
use listener::{bind_loopback, endpoint_url, spawn_accept};
use proof::ProofLog;
use state::{BridgeRegistry, Gate, LiveLease, RequestBounds, SessionPhase};
use std::sync::atomic::{AtomicBool, AtomicUsize};
use std::sync::{Arc, Condvar, Mutex};
use std::time::Duration;
use swallowtail_core::{CancellationScope, ExecutionHostId, WatcherCleanupCause};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, ImmediateCancellation, RuntimeFailure, RuntimeTurnId, TimeService,
    WATCHER_BRIDGE_MAX_WAIT, WatcherBridgeAdmission, WatcherBridgeBearer,
    WatcherBridgeCompletionState, WatcherBridgeEndpoint, WatcherBridgeGeneration,
    WatcherBridgeHostService, WatcherBridgeLease, WatcherBridgeOpenRequest, WatcherBridgeToken,
    WatcherHostService,
};

pub(crate) struct LocalWatcherBridgeHostService {
    execution_host_id: ExecutionHostId,
    watcher: Arc<dyn WatcherHostService>,
    time: Arc<dyn TimeService>,
    wait_bound: Duration,
    state: Arc<Mutex<BridgeRegistry>>,
}

impl LocalWatcherBridgeHostService {
    pub(crate) fn new(
        execution_host_id: ExecutionHostId,
        watcher: Arc<dyn WatcherHostService>,
        time: Arc<dyn TimeService>,
    ) -> Self {
        Self {
            execution_host_id,
            watcher,
            time,
            wait_bound: WATCHER_BRIDGE_MAX_WAIT,
            state: Arc::new(Mutex::new(BridgeRegistry::default())),
        }
    }

    pub(crate) fn proof_facts(&self, turn: &RuntimeTurnId) -> Vec<WatcherBridgeProofKind> {
        let registry = self
            .state
            .lock()
            .expect("watcher bridge registry lock poisoned");
        if let Some(generation) = registry.by_turn.get(turn)
            && let Some(live) = registry.live.get(generation)
        {
            return live.proof.snapshot();
        }
        registry
            .retired_proof
            .get(turn)
            .cloned()
            .unwrap_or_default()
    }

    #[cfg(test)]
    pub(crate) fn with_wait_bound(mut self, wait_bound: Duration) -> Self {
        self.wait_bound = wait_bound;
        self
    }

    fn open_now(
        &self,
        request: WatcherBridgeOpenRequest,
    ) -> Result<WatcherBridgeLease, RuntimeFailure> {
        let (listener, addr) = bind_loopback()?;
        let bearer = generate_bearer()?;
        let token_secret = generate_bearer()?;
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
            token: WatcherBridgeToken::new(token_secret.as_str())
                .map_err(|_| identity_failure())?,
            watcher: Arc::clone(&self.watcher),
            closed: AtomicBool::new(false),
            connection_count: AtomicUsize::new(0),
            cancel: ImmediateCancellation::new(CancellationScope::ActiveTurn),
            time: Arc::clone(&self.time),
            wait_bound: self.wait_bound,
            gate: Mutex::new(Gate {
                admission: WatcherBridgeAdmission::Open,
                creating: 0,
            }),
            creating_changed: Condvar::new(),
            requests: Mutex::new(RequestBounds::default()),
            session: Mutex::new(SessionPhase::New),
            connections: Mutex::new(Vec::new()),
            accept_thread: Mutex::new(None),
            proof: ProofLog::new(),
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
        let close_live = Arc::clone(&live);
        Ok(WatcherBridgeLease::new(
            self.execution_host_id.clone(),
            request.scope().clone(),
            request.turn().clone(),
            generation,
            WatcherBridgeEndpoint::new(endpoint).map_err(|_| identity_failure())?,
            WatcherBridgeBearer::new(bearer.as_str()).map_err(|_| identity_failure())?,
        )
        .bind(
            WatcherBridgeToken::new(token_secret.as_str()).map_err(|_| identity_failure())?,
            move || {
                let _ = shutdown_live(close_state, close_live, WatcherCleanupCause::Cancelled);
            },
        ))
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
        if !lease.binding_matches(&live.token) {
            return Err(foreign_failure());
        }
        Ok(live)
    }

    fn close_now(
        &self,
        lease: WatcherBridgeLease,
        cause: WatcherCleanupCause,
    ) -> Result<CleanupOutcome, RuntimeFailure> {
        let live = self.live_for(&lease)?;
        shutdown_live(Arc::clone(&self.state), live, cause)
    }
}

impl WatcherBridgeHostService for LocalWatcherBridgeHostService {
    fn open(
        &self,
        request: WatcherBridgeOpenRequest,
    ) -> BoxFuture<'_, Result<WatcherBridgeLease, RuntimeFailure>> {
        let result = self.open_now(request);
        Box::pin(async move { result })
    }

    fn completion_gate(
        &self,
        lease: &WatcherBridgeLease,
    ) -> BoxFuture<'_, Result<WatcherBridgeCompletionState, RuntimeFailure>> {
        let result = self.live_for(lease).and_then(|live| live.completion_gate());
        Box::pin(async move { result })
    }

    fn close(
        &self,
        lease: WatcherBridgeLease,
        cause: WatcherCleanupCause,
    ) -> BoxFuture<'_, Result<CleanupOutcome, RuntimeFailure>> {
        let result = self.close_now(lease, cause);
        Box::pin(async move { result })
    }
}
