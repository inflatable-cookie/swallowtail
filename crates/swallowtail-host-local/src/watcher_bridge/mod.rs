//! Local Contract 060 watcher HTTP bridge.

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

pub(crate) use state::LiveLease as LiveWatcherLease;

use crate::operation_bridge::{
    BridgeLease, BridgeLeaseOwner, BridgeProfile, OperationBridgeCleanupCause,
    OperationBridgeRegistry, generate_operation_secret,
};
use crate::output::failure;
use close::shutdown_live;
use failure::{closed_failure, foreign_failure, identity_failure};
pub(crate) use http::{
    configure_stream, constant_time_eq, read_request as read_private_request, write_response,
};
pub(crate) use listener::{bind_loopback, wake_accept};
use listener::{endpoint_url, spawn_accept};
use proof::ProofLog;
use state::{Gate, LiveLease, ProofArchive, RequestBounds, SessionPhase};
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
    registry: Arc<OperationBridgeRegistry>,
    proof_archive: Arc<Mutex<ProofArchive>>,
}

/// Watcher-profile teardown driven by the one shared registry.
struct WatcherLeaseOwner {
    registry: Arc<OperationBridgeRegistry>,
    proof_archive: Arc<Mutex<ProofArchive>>,
    live: Arc<LiveLease>,
}

impl BridgeLeaseOwner for WatcherLeaseOwner {
    fn freeze(&self) {
        self.live.freeze_admission();
    }

    fn join_and_release(
        &self,
        cause: OperationBridgeCleanupCause,
    ) -> Result<CleanupOutcome, RuntimeFailure> {
        shutdown_live(
            &self.registry,
            &self.proof_archive,
            &self.live,
            watcher_cause(cause),
        )
    }
}

pub(crate) const fn watcher_cause(cause: OperationBridgeCleanupCause) -> WatcherCleanupCause {
    match cause {
        OperationBridgeCleanupCause::Completion | OperationBridgeCleanupCause::ExplicitClose => {
            WatcherCleanupCause::Stopped
        }
        OperationBridgeCleanupCause::Cancellation => WatcherCleanupCause::Cancelled,
        OperationBridgeCleanupCause::Deadline => WatcherCleanupCause::TimedOut,
        OperationBridgeCleanupCause::ProviderFailure
        | OperationBridgeCleanupCause::TransportFailure => WatcherCleanupCause::Failed,
    }
}

impl LocalWatcherBridgeHostService {
    pub(crate) fn new(
        execution_host_id: ExecutionHostId,
        watcher: Arc<dyn WatcherHostService>,
        time: Arc<dyn TimeService>,
        registry: Arc<OperationBridgeRegistry>,
    ) -> Self {
        Self {
            execution_host_id,
            watcher,
            time,
            wait_bound: WATCHER_BRIDGE_MAX_WAIT,
            registry,
            proof_archive: Arc::new(Mutex::new(ProofArchive::default())),
        }
    }

    pub(crate) fn proof_facts(&self, turn: &RuntimeTurnId) -> Vec<WatcherBridgeProofKind> {
        for (profile, generation) in self.registry.generations_for_turn(turn) {
            if profile == BridgeProfile::Watcher
                && let Some(live) = self.registry.watcher_lease(generation)
            {
                return live.proof.snapshot();
            }
        }
        self.proof_archive
            .lock()
            .expect("watcher bridge proof archive lock poisoned")
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
        let bearer = generate_operation_secret()?;
        let token_secret = generate_operation_secret()?;
        let endpoint = endpoint_url(addr);
        let reserved = self
            .registry
            .reserve(BridgeProfile::Watcher, request.turn())
            .ok_or_else(|| {
                failure(
                    "swallowtail.watcher_bridge.already_open",
                    "Watcher bridge already has an open lease for this turn",
                )
            })?;
        let generation = WatcherBridgeGeneration::new(reserved).ok_or_else(identity_failure)?;
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
        self.registry.attach(
            generation.get(),
            BridgeProfile::Watcher,
            request.turn().clone(),
            BridgeLease::Watcher(Arc::clone(&live)),
            Arc::new(WatcherLeaseOwner {
                registry: Arc::clone(&self.registry),
                proof_archive: Arc::clone(&self.proof_archive),
                live: Arc::clone(&live),
            }),
        );
        let close_registry = Arc::clone(&self.registry);
        let close_archive = Arc::clone(&self.proof_archive);
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
                let _ = shutdown_live(
                    &close_registry,
                    &close_archive,
                    &close_live,
                    WatcherCleanupCause::Cancelled,
                );
            },
        ))
    }

    fn forget_generation(&self, turn: &RuntimeTurnId, generation: WatcherBridgeGeneration) {
        self.registry
            .forget(BridgeProfile::Watcher, turn, generation.get());
    }

    fn live_for(&self, lease: &WatcherBridgeLease) -> Result<Arc<LiveLease>, RuntimeFailure> {
        if lease.execution_host_id() != &self.execution_host_id {
            return Err(foreign_failure());
        }
        let live = self
            .registry
            .watcher_lease(lease.generation().get())
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
        shutdown_live(&self.registry, &self.proof_archive, &live, cause)
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
