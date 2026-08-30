mod live;

use super::failure::identity_failure;
use super::proof::{ProofLog, WatcherBridgeProofKind};
use std::collections::{BTreeMap, BTreeSet};
use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, AtomicUsize};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;
use swallowtail_core::{ExecutionHostId, WatcherOwningTurn};
use swallowtail_runtime::{
    BoxFuture, ImmediateCancellation, RuntimeFailure, RuntimeTurnId, ScopeId, TimeService,
    WatcherBridgeAdmission, WatcherBridgeGeneration, WatcherBridgeToken, WatcherHostService,
};
use zeroize::Zeroizing;

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum SessionPhase {
    New,
    Handshake,
    Ready,
}

pub(super) struct LiveLease {
    pub(super) execution_host_id: ExecutionHostId,
    pub(super) scope: ScopeId,
    pub(super) turn: RuntimeTurnId,
    pub(super) generation: WatcherBridgeGeneration,
    pub(super) bind_addr: SocketAddr,
    pub(super) bearer: Zeroizing<String>,
    pub(super) token: WatcherBridgeToken,
    pub(super) watcher: Arc<dyn WatcherHostService>,
    pub(super) closed: AtomicBool,
    pub(super) connection_count: AtomicUsize,
    pub(super) cancel: ImmediateCancellation,
    pub(super) time: Arc<dyn TimeService>,
    pub(super) wait_bound: Duration,
    pub(super) gate: Mutex<Gate>,
    pub(super) creating_changed: Condvar,
    pub(super) requests: Mutex<RequestBounds>,
    pub(super) session: Mutex<SessionPhase>,
    pub(super) connections: Mutex<Vec<JoinHandle<()>>>,
    pub(super) accept_thread: Mutex<Option<JoinHandle<()>>>,
    pub(super) proof: ProofLog,
}

pub(super) struct Gate {
    pub(super) admission: WatcherBridgeAdmission,
    pub(super) creating: usize,
}

#[derive(Default)]
pub(super) struct RequestBounds {
    in_flight: usize,
    seen: BTreeSet<String>,
}

pub(super) fn drive<T>(
    future: BoxFuture<'_, Result<T, RuntimeFailure>>,
) -> Result<T, RuntimeFailure> {
    futures_executor::block_on(future)
}

pub(super) fn owning_turn(turn: &RuntimeTurnId) -> Result<WatcherOwningTurn, RuntimeFailure> {
    WatcherOwningTurn::new(turn.as_str().to_owned()).map_err(|_| identity_failure())
}

impl LiveLease {
    pub(super) fn record_proof(&self, kind: WatcherBridgeProofKind) {
        self.proof.record(kind);
    }
}

fn reap_finished(connections: &mut Vec<JoinHandle<()>>) {
    let mut index = 0;
    while index < connections.len() {
        if connections[index].is_finished() {
            let handle = connections.swap_remove(index);
            let _ = handle.join();
        } else {
            index += 1;
        }
    }
}
