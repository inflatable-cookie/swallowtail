use std::sync::{Arc, Mutex};
use swallowtail_core::{
    WatcherId, WatcherLifecyclePhase, WatcherOwningTurn, WatcherRequester, WatcherSummary,
    WatcherTerminalCause,
};
use swallowtail_runtime::{
    ActivityKind, ActivityLifecyclePhase, ActivityStatus, ModelWatcherControl,
    OperatorWatcherControl, RuntimeTurnId, WatcherControlSurface, WatcherFailureKind,
    WatcherRegistry, WatcherStopAcknowledgement, WatcherWaitRepresentation,
    project_watcher_activity,
};

const WATCHER_RULE: &str = "Contract 059 portable watcher lifecycle";

/// Proves watcher ids and summaries stay redacted in default formatting.
pub fn assert_watcher_identity_redaction(watcher_id: &WatcherId, summary: &WatcherSummary) {
    let id_value = watcher_id.as_str();
    let summary_value = summary.as_str();
    assert!(
        !format!("{watcher_id:?}").contains(id_value),
        "{WATCHER_RULE}: WatcherId debug exposed its value"
    );
    assert!(
        !format!("{watcher_id}").contains(id_value),
        "{WATCHER_RULE}: WatcherId display exposed its value"
    );
    assert!(
        !format!("{summary:?}").contains(summary_value),
        "{WATCHER_RULE}: WatcherSummary debug exposed its value"
    );
    assert!(
        !format!("{summary}").contains(summary_value),
        "{WATCHER_RULE}: WatcherSummary display exposed its value"
    );
}

/// Proves foreign and unknown watcher identities fail closed.
pub fn assert_watcher_ownership_rejection(registry: &WatcherRegistry) {
    let foreign = WatcherOwningTurn::new("foreign-turn").expect("foreign turn is valid");
    let unknown = WatcherId::new("missing-watcher").expect("watcher id is valid");
    let failure = registry
        .inspect(&foreign, &unknown)
        .expect_err("{WATCHER_RULE}: foreign turn must fail");
    assert_eq!(failure.kind(), WatcherFailureKind::ForeignIdentity);

    let owned = registry.owning_turn().clone();
    let failure = registry
        .inspect(&owned, &unknown)
        .expect_err("{WATCHER_RULE}: unknown watcher must fail");
    assert_eq!(failure.kind(), WatcherFailureKind::UnknownWatcher);
}

/// Proves capacity bounds reject additional accepted starts.
pub fn assert_watcher_capacity_bound(mut registry: WatcherRegistry) {
    let capacity = registry.maximum_watchers();
    for _ in 0..capacity {
        registry
            .accept_start(WatcherRequester::Model, None)
            .expect("{WATCHER_RULE}: accepted start within bound");
    }
    let failure = registry
        .accept_start(WatcherRequester::Operator, None)
        .expect_err("{WATCHER_RULE}: over-capacity start must fail");
    assert_eq!(failure.kind(), WatcherFailureKind::CapacityExceeded);
}

/// Proves accepted → running → terminal → joined order and race behavior.
pub fn assert_watcher_lifecycle_transitions() {
    let turn = RuntimeTurnId::new("turn-lifecycle").expect("turn is valid");
    let mut registry = WatcherRegistry::new(turn, 4).expect("registry is valid");
    let accepted = registry
        .accept_start(
            WatcherRequester::Model,
            Some(WatcherSummary::new("accepted").expect("summary is valid")),
        )
        .expect("accept succeeds");
    assert_eq!(accepted.phase(), WatcherLifecyclePhase::Accepted);
    assert_eq!(accepted.revision().get(), 1);

    let running = registry
        .mark_running(accepted.watcher_id())
        .expect("running succeeds");
    assert_eq!(running.phase(), WatcherLifecyclePhase::Running);
    assert_eq!(running.revision().get(), 2);

    let completed = registry
        .complete(
            accepted.watcher_id(),
            WatcherTerminalCause::Completed,
            Some(WatcherSummary::new("done").expect("summary is valid")),
        )
        .expect("complete succeeds");
    assert_eq!(completed.phase(), WatcherLifecyclePhase::Terminal);
    assert_eq!(
        completed.terminal_cause(),
        Some(WatcherTerminalCause::Completed)
    );

    let (stop, _) = registry
        .request_stop(accepted.watcher_id())
        .expect("repeated stop is idempotent");
    assert_eq!(
        stop,
        WatcherStopAcknowledgement::AlreadyTerminal(WatcherTerminalCause::Completed)
    );

    let joined = registry.join(accepted.watcher_id()).expect("join succeeds");
    assert_eq!(joined.phase(), WatcherLifecyclePhase::Joined);
    assert!(registry.all_joined());
}

/// Proves stop-versus-completion races keep the first terminal cause.
pub fn assert_watcher_completion_stop_race() {
    let turn = RuntimeTurnId::new("turn-race").expect("turn is valid");
    let mut registry = WatcherRegistry::new(turn, 2).expect("registry is valid");
    let first = registry
        .accept_start(WatcherRequester::Model, None)
        .expect("accept succeeds");
    registry
        .mark_running(first.watcher_id())
        .expect("running succeeds");
    let (stop, stopped) = registry
        .request_stop(first.watcher_id())
        .expect("stop wins");
    assert_eq!(stop, WatcherStopAcknowledgement::Stopped);
    assert_eq!(
        stopped.terminal_cause(),
        Some(WatcherTerminalCause::Stopped)
    );
    let failure = registry
        .complete(first.watcher_id(), WatcherTerminalCause::Completed, None)
        .expect_err("{WATCHER_RULE}: second terminal must fail");
    assert_eq!(failure.kind(), WatcherFailureKind::AlreadyTerminal);
}

/// Proves wait representation, cancellation, and deadline markers.
pub fn assert_watcher_wait_representation() {
    let turn = RuntimeTurnId::new("turn-wait").expect("turn is valid");
    let mut registry = WatcherRegistry::new(turn, 2).expect("registry is valid");
    let owning = registry.owning_turn().clone();
    let accepted = registry
        .accept_start(WatcherRequester::Operator, None)
        .expect("accept succeeds");
    assert_eq!(
        registry
            .wait_representation(&owning, accepted.watcher_id())
            .expect("wait inspects"),
        WatcherWaitRepresentation::Pending
    );
    registry
        .complete(accepted.watcher_id(), WatcherTerminalCause::Failed, None)
        .expect("terminal");
    assert_eq!(
        registry
            .wait_representation(&owning, accepted.watcher_id())
            .expect("wait inspects"),
        WatcherWaitRepresentation::TerminalUnjoined(WatcherTerminalCause::Failed)
    );
    registry.join(accepted.watcher_id()).expect("join");
    assert_eq!(
        registry
            .wait_representation(&owning, accepted.watcher_id())
            .expect("wait inspects"),
        WatcherWaitRepresentation::Satisfied(WatcherTerminalCause::Failed)
    );
    assert_eq!(
        registry
            .require_wait_satisfied(&owning, accepted.watcher_id())
            .expect("satisfied wait"),
        WatcherTerminalCause::Failed
    );
    assert_eq!(
        registry.represent_wait_cancelled(),
        WatcherWaitRepresentation::Cancelled
    );
    assert_eq!(
        registry.represent_wait_deadline_exceeded(),
        WatcherWaitRepresentation::DeadlineExceeded
    );
}

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
        .accept_start(Some(WatcherSummary::new("model").expect("summary")))
        .expect("model accept");
    assert_eq!(model_start.accepted_by(), WatcherRequester::Model);
    let operator_start = operator
        .accept_start(Some(WatcherSummary::new("operator").expect("summary")))
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

/// Proves host-watcher activity projection stays on the turn stream vocabulary.
pub fn assert_watcher_activity_projection() {
    let turn = RuntimeTurnId::new("turn-activity").expect("turn is valid");
    let mut registry = WatcherRegistry::new(turn.clone(), 1).expect("registry is valid");
    let accepted = registry
        .accept_start(WatcherRequester::Model, None)
        .expect("accept");
    let observation = project_watcher_activity(&turn, &accepted).expect("projection succeeds");
    assert_eq!(*observation.kind(), ActivityKind::HostWatcher);
    assert_eq!(observation.phase(), ActivityLifecyclePhase::Started);
    assert_eq!(observation.status(), ActivityStatus::Pending);
    assert_eq!(observation.activity_id(), accepted.activity_id());
}

/// Runs the Contract 059 portable watcher assertion pack.
pub fn assert_portable_watcher_lifecycle_contract() {
    assert_watcher_identity_redaction(
        &WatcherId::new("watcher-opaque").expect("id"),
        &WatcherSummary::new("bounded progress").expect("summary"),
    );

    let turn = RuntimeTurnId::new("turn-pack").expect("turn is valid");
    let registry = WatcherRegistry::new(turn.clone(), 2).expect("registry");
    assert_watcher_ownership_rejection(&registry);
    assert_watcher_capacity_bound(WatcherRegistry::new(turn, 1).expect("capacity registry"));
    assert_watcher_lifecycle_transitions();
    assert_watcher_completion_stop_race();
    assert_watcher_wait_representation();
    assert_watcher_model_operator_roles();
    assert_watcher_activity_projection();
}

#[cfg(test)]
mod tests {
    use super::assert_portable_watcher_lifecycle_contract;

    #[test]
    fn portable_watcher_lifecycle_contract_holds() {
        assert_portable_watcher_lifecycle_contract();
    }
}
