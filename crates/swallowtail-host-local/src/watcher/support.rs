use crate::output::failure;
use futures_executor::block_on;
use std::sync::Arc;
use swallowtail_core::{WatcherOwningTurn, WatcherSummary};
use swallowtail_runtime::{
    ProcessHandle, RuntimeFailure, RuntimeTurnId, ScopeId, WatcherFailure, WatcherFailureKind,
};

pub(super) fn cleanup_process(process: &Arc<dyn ProcessHandle>) {
    let _ = block_on(process.force_stop());
    let _ = block_on(process.wait());
}

pub(super) fn request_process_stop(process: &Arc<dyn ProcessHandle>) -> Result<(), RuntimeFailure> {
    match block_on(process.request_stop()) {
        Ok(()) => Ok(()),
        Err(graceful_error) => match block_on(process.force_stop()) {
            Ok(()) => Ok(()),
            Err(_) => Err(graceful_error),
        },
    }
}

pub(super) fn watcher_scope(turn: &RuntimeTurnId) -> Result<ScopeId, RuntimeFailure> {
    ScopeId::new(format!("swallowtail-watcher-{}", turn.as_str())).map_err(|_| {
        failure(
            "swallowtail.local_watcher.scope_invalid",
            "Local watcher could not derive an operation scope",
        )
    })
}

pub(super) fn runtime_turn(
    owning_turn: &WatcherOwningTurn,
) -> Result<RuntimeTurnId, RuntimeFailure> {
    RuntimeTurnId::new(owning_turn.as_str().to_owned()).map_err(|_| {
        failure(
            "swallowtail.local_watcher.identity_rejected",
            "Local watcher rejected an invalid owning-turn identity",
        )
    })
}

pub(super) fn owning_turn(turn: &RuntimeTurnId) -> Result<WatcherOwningTurn, RuntimeFailure> {
    WatcherOwningTurn::new(turn.as_str().to_owned()).map_err(|_| {
        failure(
            "swallowtail.local_watcher.identity_rejected",
            "Local watcher rejected an invalid runtime-turn identity",
        )
    })
}

pub(super) fn turn_missing_failure() -> RuntimeFailure {
    failure(
        "swallowtail.local_watcher.turn_not_found",
        "Local watcher turn state is not available",
    )
}

pub(super) fn entry_missing_failure() -> RuntimeFailure {
    failure(
        "swallowtail.local_watcher.watcher_not_found",
        "Local watcher state is not available",
    )
}

pub(super) fn registry_failure(error: WatcherFailure) -> RuntimeFailure {
    let (code, message) = match error.kind() {
        WatcherFailureKind::InvalidCapacity | WatcherFailureKind::CapacityExceeded => (
            "swallowtail.local_watcher.capacity_rejected",
            "Local watcher capacity rejected the operation",
        ),
        WatcherFailureKind::ForeignIdentity | WatcherFailureKind::UnknownWatcher => (
            "swallowtail.local_watcher.identity_rejected",
            "Local watcher rejected a foreign or stale identity",
        ),
        WatcherFailureKind::InvalidTransition
        | WatcherFailureKind::AlreadyTerminal
        | WatcherFailureKind::AlreadyJoined
        | WatcherFailureKind::WaitNotSatisfied => (
            "swallowtail.local_watcher.lifecycle_rejected",
            "Local watcher lifecycle rejected the operation",
        ),
    };
    failure(code, message)
}

pub(super) fn summary(value: &'static str) -> WatcherSummary {
    WatcherSummary::new(value).expect("local watcher summary is valid")
}
