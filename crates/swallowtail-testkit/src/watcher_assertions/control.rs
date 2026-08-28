use super::WATCHER_RULE;
use std::sync::{Arc, Mutex};
use swallowtail_core::{WatcherLifecyclePhase, WatcherOperationData, WatcherRequester};
use swallowtail_runtime::{
    ModelWatcherControl, OperatorWatcherControl, RuntimeTurnId, WatcherControlSurface,
    WatcherFailureKind, WatcherRegistry, WatcherStopAcknowledgement,
};

/// Proves model and operator roles share one registry and retain requester identity.
pub fn assert_watcher_model_operator_roles() {
    let turn = RuntimeTurnId::new("turn-roles").expect("turn is valid");
    let registry = Arc::new(Mutex::new(
        WatcherRegistry::new(turn, 4).expect("registry is valid"),
    ));
    let surface = WatcherControlSurface::new(Arc::clone(&registry));
    let model = surface.model();
    let operator = surface.operator();

    let model_start = model
        .accept_start(WatcherOperationData::new("model-operation").expect("operation data"))
        .expect("model accept");
    assert_eq!(model_start.accepted_by(), WatcherRequester::Model);
    let operator_start = operator
        .accept_start(WatcherOperationData::new("operator-operation").expect("operation data"))
        .expect("operator accept");
    assert_eq!(operator_start.accepted_by(), WatcherRequester::Operator);

    let owning = registry.lock().expect("lock").owning_turn().clone();
    let listed = model.list(&owning).expect("shared list");
    assert_eq!(listed.len(), 2);
    assert!(
        listed
            .iter()
            .any(|snapshot| snapshot.accepted_by() == WatcherRequester::Model)
    );
    assert!(
        listed
            .iter()
            .any(|snapshot| snapshot.accepted_by() == WatcherRequester::Operator)
    );

    let (stop, _) = operator
        .stop(&owning, model_start.watcher_id())
        .expect("operator can stop model-started watcher");
    assert_eq!(stop, WatcherStopAcknowledgement::Stopped);
}

/// Proves a stale watcher id from turn A cannot stop turn B work.
pub fn assert_watcher_stale_id_fails_closed() {
    let turn_a = RuntimeTurnId::new("turn-a").expect("turn a is valid");
    let turn_b = RuntimeTurnId::new("turn-b").expect("turn b is valid");
    let mut registry_a = WatcherRegistry::new(turn_a, 2).expect("registry a");
    let stale = registry_a
        .accept_start(
            WatcherRequester::Model,
            WatcherOperationData::new("stale-operation").expect("operation data"),
        )
        .expect("turn a accept");
    let stale_id = stale.watcher_id().clone();

    let registry_b = Arc::new(Mutex::new(
        WatcherRegistry::new(turn_b, 2).expect("registry b"),
    ));
    let surface = WatcherControlSurface::new(Arc::clone(&registry_b));
    let model = surface.model();
    let operator = surface.operator();
    let current = model
        .accept_start(WatcherOperationData::new("current-operation").expect("operation data"))
        .expect("turn b accept");
    let owning_b = registry_b.lock().expect("lock").owning_turn().clone();

    assert_ne!(
        stale_id.as_str(),
        current.watcher_id().as_str(),
        "{WATCHER_RULE}: turn-bound ids must not reuse sequence-local names"
    );

    let model_failure = model
        .stop(&owning_b, &stale_id)
        .expect_err("{WATCHER_RULE}: stale model stop must fail closed");
    assert_eq!(model_failure.kind(), WatcherFailureKind::UnknownWatcher);

    let operator_failure = operator
        .stop(&owning_b, &stale_id)
        .expect_err("{WATCHER_RULE}: stale operator stop must fail closed");
    assert_eq!(operator_failure.kind(), WatcherFailureKind::UnknownWatcher);

    let current_phase = registry_b
        .lock()
        .expect("lock")
        .inspect(&owning_b, current.watcher_id())
        .expect("current watcher remains")
        .phase();
    assert_eq!(
        current_phase,
        WatcherLifecyclePhase::Accepted,
        "{WATCHER_RULE}: stale stop must not mutate foreign turn work"
    );
}
