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
    drop(
        live.route
            .lock()
            .expect("watcher route lock poisoned")
            .take(),
    );
    let turn = live.turn.clone();
    let generation = live.generation;
    let kinds = live.proof.snapshot();
    proof_archive
        .lock()
        .expect("watcher bridge proof archive lock poisoned")
        .retire_proof(turn.clone(), kinds);
    registry.forget(BridgeProfile::Watcher, &turn, generation.get());
    registry.close_listener_if_idle(&turn);
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
