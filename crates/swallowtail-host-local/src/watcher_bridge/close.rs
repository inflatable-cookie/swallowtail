use super::state::{LiveLease, ProofArchive, drive};
use crate::operation_bridge::{BridgeProfile, OperationBridgeRegistry};
use std::sync::{Arc, Mutex};
use swallowtail_core::WatcherCleanupCause;
use swallowtail_runtime::{
    CancellationControl, CleanupOutcome, RuntimeFailure, WatcherBridgeAdmission,
};

pub(super) fn shutdown_live(
    registry: &Arc<OperationBridgeRegistry>,
    proof_archive: &Arc<Mutex<ProofArchive>>,
    live: &Arc<LiveLease>,
    cause: WatcherCleanupCause,
) -> Result<CleanupOutcome, RuntimeFailure> {
    if live.closed.swap(true, std::sync::atomic::Ordering::SeqCst) {
        return Ok(CleanupOutcome::NotApplicable);
    }
    {
        let mut gate = live.gate.lock().expect("watcher bridge gate lock poisoned");
        gate.admission = WatcherBridgeAdmission::Closed;
    }
    drop(live.cancel.request());
    crate::operation_bridge::wake_accept(live.bind_addr);
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
    let turn = live.turn.clone();
    let generation = live.generation;
    let kinds = live.proof.snapshot();
    proof_archive
        .lock()
        .expect("watcher bridge proof archive lock poisoned")
        .retire_proof(turn.clone(), kinds);
    registry.forget(BridgeProfile::Watcher, &turn, generation.get());
    let outcome = match drive(live.watcher.stop_and_join_all(turn.clone(), cause)) {
        Ok((_, outcome)) => Ok(outcome),
        Err(error)
            if matches!(
                error.diagnostic().code(),
                "swallowtail.local_watcher.turn_not_found"
                    | "swallowtail.local_watcher.turn_retired"
            ) =>
        {
            Ok(CleanupOutcome::NotApplicable)
        }
        Err(error) => Err(error),
    };
    let _ = drive(live.watcher.close_lifecycle_feed(turn));
    outcome
}
