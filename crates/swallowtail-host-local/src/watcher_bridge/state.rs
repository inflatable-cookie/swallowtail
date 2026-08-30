use crate::output::failure;
use std::collections::{BTreeMap, BTreeSet};
use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::JoinHandle;
use swallowtail_core::{ExecutionHostId, WatcherOwningTurn};
use swallowtail_runtime::{
    ImmediateCancellation, RuntimeFailure, RuntimeTurnId, ScopeId,
    WATCHER_BRIDGE_MAX_CONCURRENT_CONNECTIONS, WATCHER_BRIDGE_MAX_CORRELATION_IDS,
    WATCHER_BRIDGE_MAX_IN_FLIGHT_REQUESTS, WatcherBridgeAdmission, WatcherBridgeCompletionState,
    WatcherBridgeGeneration, WatcherHostService,
};

pub(super) struct BridgeRegistry {
    pub(super) next_generation: u64,
    pub(super) by_turn: BTreeMap<RuntimeTurnId, WatcherBridgeGeneration>,
    pub(super) live: BTreeMap<WatcherBridgeGeneration, Arc<LiveLease>>,
}

impl Default for BridgeRegistry {
    fn default() -> Self {
        Self {
            next_generation: 1,
            by_turn: BTreeMap::new(),
            live: BTreeMap::new(),
        }
    }
}

pub(super) struct LiveLease {
    pub(super) execution_host_id: ExecutionHostId,
    pub(super) scope: ScopeId,
    pub(super) turn: RuntimeTurnId,
    pub(super) generation: WatcherBridgeGeneration,
    pub(super) bind_addr: SocketAddr,
    pub(super) bearer: String,
    pub(super) watcher: Arc<dyn WatcherHostService>,
    pub(super) closed: AtomicBool,
    pub(super) in_flight: AtomicUsize,
    pub(super) connection_count: AtomicUsize,
    pub(super) cancel: ImmediateCancellation,
    pub(super) gate: Mutex<Gate>,
    pub(super) creating_changed: Condvar,
    pub(super) seen_ids: Mutex<BTreeSet<String>>,
    pub(super) connections: Mutex<Vec<JoinHandle<()>>>,
    pub(super) accept_thread: Mutex<Option<JoinHandle<()>>>,
}

pub(super) struct Gate {
    pub(super) admission: WatcherBridgeAdmission,
    pub(super) creating: usize,
}

impl LiveLease {
    pub(super) fn is_closed(&self) -> bool {
        self.closed.load(Ordering::SeqCst)
    }

    pub(super) fn matches(
        &self,
        host: &ExecutionHostId,
        scope: &ScopeId,
        turn: &RuntimeTurnId,
        generation: WatcherBridgeGeneration,
    ) -> Result<(), RuntimeFailure> {
        if self.is_closed() {
            return Err(closed_failure());
        }
        if &self.execution_host_id != host
            || &self.scope != scope
            || &self.turn != turn
            || self.generation != generation
        {
            return Err(foreign_failure());
        }
        Ok(())
    }

    pub(super) fn admit_connection(&self) -> Result<(), RuntimeFailure> {
        if self.is_closed() {
            return Err(closed_failure());
        }
        let current = self.connection_count.load(Ordering::SeqCst);
        if current >= WATCHER_BRIDGE_MAX_CONCURRENT_CONNECTIONS {
            return Err(busy_failure());
        }
        self.connection_count.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    pub(super) fn release_connection(&self) {
        self.connection_count.fetch_sub(1, Ordering::SeqCst);
    }

    pub(super) fn admit_request(&self, correlation: &str) -> Result<(), RuntimeFailure> {
        if self.is_closed() {
            return Err(closed_failure());
        }
        if self.in_flight.load(Ordering::SeqCst) >= WATCHER_BRIDGE_MAX_IN_FLIGHT_REQUESTS {
            return Err(busy_failure());
        }
        let mut seen = self
            .seen_ids
            .lock()
            .expect("watcher bridge correlation lock poisoned");
        if seen.len() >= WATCHER_BRIDGE_MAX_CORRELATION_IDS {
            return Err(busy_failure());
        }
        if !seen.insert(correlation.to_owned()) {
            return Err(duplicate_failure());
        }
        self.in_flight.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    pub(super) fn release_request(&self) {
        self.in_flight.fetch_sub(1, Ordering::SeqCst);
    }

    pub(super) fn begin_create(&self) -> Result<(), RuntimeFailure> {
        let mut gate = self.gate.lock().expect("watcher bridge gate lock poisoned");
        if gate.admission != WatcherBridgeAdmission::Open {
            return Err(if gate.admission == WatcherBridgeAdmission::Frozen {
                frozen_failure()
            } else {
                closed_failure()
            });
        }
        gate.creating += 1;
        Ok(())
    }

    pub(super) fn end_create(&self) {
        let mut gate = self.gate.lock().expect("watcher bridge gate lock poisoned");
        gate.creating = gate.creating.saturating_sub(1);
        self.creating_changed.notify_all();
    }

    pub(super) fn require_not_closed(&self) -> Result<WatcherBridgeAdmission, RuntimeFailure> {
        let gate = self.gate.lock().expect("watcher bridge gate lock poisoned");
        if gate.admission == WatcherBridgeAdmission::Closed {
            Err(closed_failure())
        } else {
            Ok(gate.admission)
        }
    }

    pub(super) fn completion_gate(&self) -> Result<WatcherBridgeCompletionState, RuntimeFailure> {
        loop {
            let snapshots = ready(self.watcher.list(owning_turn(&self.turn)?))?;
            let remaining = snapshots
                .into_iter()
                .filter(|snapshot| !snapshot.phase().is_joined())
                .collect();
            let mut gate = self.gate.lock().expect("watcher bridge gate lock poisoned");
            match gate.admission {
                WatcherBridgeAdmission::Closed => return Err(closed_failure()),
                WatcherBridgeAdmission::Frozen => {
                    return Ok(WatcherBridgeCompletionState::new(
                        WatcherBridgeAdmission::Frozen,
                        remaining,
                    ));
                }
                WatcherBridgeAdmission::Open if gate.creating > 0 => {
                    drop(
                        self.creating_changed
                            .wait(gate)
                            .expect("watcher bridge gate lock poisoned"),
                    );
                }
                WatcherBridgeAdmission::Open if remaining.is_empty() => {
                    gate.admission = WatcherBridgeAdmission::Frozen;
                    return Ok(WatcherBridgeCompletionState::new(
                        WatcherBridgeAdmission::Frozen,
                        Vec::new(),
                    ));
                }
                WatcherBridgeAdmission::Open => {
                    return Ok(WatcherBridgeCompletionState::new(
                        WatcherBridgeAdmission::Open,
                        remaining,
                    ));
                }
            }
        }
    }
}

pub(super) fn ready<T>(
    future: swallowtail_runtime::BoxFuture<'_, Result<T, RuntimeFailure>>,
) -> Result<T, RuntimeFailure> {
    let mut future = future;
    let waker = std::task::Waker::noop();
    let mut context = std::task::Context::from_waker(waker);
    match future.as_mut().poll(&mut context) {
        std::task::Poll::Ready(value) => value,
        std::task::Poll::Pending => Err(failure(
            "swallowtail.watcher_bridge.host_pending",
            "Watcher bridge host method did not complete synchronously",
        )),
    }
}

fn owning_turn(turn: &RuntimeTurnId) -> Result<WatcherOwningTurn, RuntimeFailure> {
    WatcherOwningTurn::new(turn.as_str().to_owned()).map_err(|_| {
        failure(
            "swallowtail.watcher_bridge.identity_rejected",
            "Watcher bridge rejected the bound turn identity",
        )
    })
}

pub(super) fn closed_failure() -> RuntimeFailure {
    failure(
        "swallowtail.watcher_bridge.closed",
        "Watcher bridge lease is closed",
    )
}

pub(super) fn frozen_failure() -> RuntimeFailure {
    failure(
        "swallowtail.watcher_bridge.admission_frozen",
        "Watcher bridge admission is frozen",
    )
}

pub(super) fn foreign_failure() -> RuntimeFailure {
    failure(
        "swallowtail.watcher_bridge.foreign_lease",
        "Watcher bridge request did not match the bound lease",
    )
}

pub(super) fn duplicate_failure() -> RuntimeFailure {
    failure(
        "swallowtail.watcher_bridge.duplicate_correlation",
        "Watcher bridge request correlation was already used",
    )
}

pub(super) fn busy_failure() -> RuntimeFailure {
    failure(
        "swallowtail.watcher_bridge.busy",
        "Watcher bridge rejected work beyond its positive bound",
    )
}

pub(super) fn unauthorized_failure() -> RuntimeFailure {
    failure(
        "swallowtail.watcher_bridge.unauthorized",
        "Watcher bridge request was not authenticated",
    )
}

pub(super) fn malformed_failure() -> RuntimeFailure {
    failure(
        "swallowtail.watcher_bridge.malformed",
        "Watcher bridge request was malformed",
    )
}

pub(super) fn oversized_failure() -> RuntimeFailure {
    failure(
        "swallowtail.watcher_bridge.oversized",
        "Watcher bridge request exceeded its positive bound",
    )
}

pub(super) fn unknown_failure() -> RuntimeFailure {
    failure(
        "swallowtail.watcher_bridge.unknown",
        "Watcher bridge request used an unknown protocol surface",
    )
}
