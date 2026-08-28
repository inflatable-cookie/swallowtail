use super::WATCHER_RULE;
use swallowtail_core::{
    WatcherCleanupCause, WatcherLifecyclePhase, WatcherRequester, WatcherSummary,
    WatcherTerminalCause,
};
use swallowtail_runtime::{
    RuntimeTurnId, WatcherFailureKind, WatcherRegistry, WatcherStopAcknowledgement,
    WatcherWaitRepresentation,
};

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

    let owning = registry.owning_turn().clone();
    let (stop, _) = registry
        .request_stop(&owning, accepted.watcher_id())
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
    let owning = registry.owning_turn().clone();
    let (stop, stopped) = registry
        .request_stop(&owning, first.watcher_id())
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

/// Proves cleanup cannot assign successful completion and uses cleanup causes only.
pub fn assert_watcher_cleanup_rejects_completed() {
    for cause in [
        WatcherCleanupCause::Cancelled,
        WatcherCleanupCause::TimedOut,
        WatcherCleanupCause::Stopped,
        WatcherCleanupCause::Failed,
    ] {
        assert_ne!(
            cause.terminal_cause(),
            WatcherTerminalCause::Completed,
            "{WATCHER_RULE}: cleanup cause must not map to Completed"
        );
    }

    let turn = RuntimeTurnId::new("turn-cleanup").expect("turn is valid");
    let mut registry = WatcherRegistry::new(turn, 2).expect("registry is valid");
    let accepted = registry
        .accept_start(WatcherRequester::Model, None)
        .expect("accept");
    registry
        .mark_running(accepted.watcher_id())
        .expect("running");
    let joined = registry
        .stop_and_join_all(WatcherCleanupCause::Cancelled)
        .expect("cleanup");
    assert_eq!(joined.len(), 1);
    assert_eq!(joined[0].phase(), WatcherLifecyclePhase::Joined);
    assert_eq!(
        joined[0].terminal_cause(),
        Some(WatcherTerminalCause::Cancelled)
    );
    assert_ne!(
        joined[0].terminal_cause(),
        Some(WatcherTerminalCause::Completed)
    );
}
