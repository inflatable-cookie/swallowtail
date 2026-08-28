use super::WATCHER_RULE;
use swallowtail_core::{WatcherRequester, WatcherTerminalCause};
use swallowtail_runtime::{
    ActivityKind, ActivityLifecyclePhase, ActivityStatus, RuntimeTurnId, WatcherActivityProjection,
    WatcherActivityProjectionFailure, WatcherRegistry, project_watcher_activity,
};

/// Proves host-watcher activity projection across the full lifecycle without duplicate completion.
pub fn assert_watcher_activity_projection() {
    let turn = RuntimeTurnId::new("turn-activity").expect("turn is valid");
    let foreign = RuntimeTurnId::new("turn-foreign").expect("foreign turn is valid");
    let mut registry = WatcherRegistry::new(turn.clone(), 1).expect("registry is valid");
    let accepted = registry
        .accept_start(WatcherRequester::Model, None)
        .expect("accept");

    assert_eq!(
        project_watcher_activity(&foreign, &accepted),
        Err(WatcherActivityProjectionFailure::ForeignIdentity)
    );

    let WatcherActivityProjection::Activity(started) =
        project_watcher_activity(&turn, &accepted).expect("accepted projection")
    else {
        panic!("{WATCHER_RULE}: accepted must project as activity");
    };
    assert_eq!(*started.kind(), ActivityKind::HostWatcher);
    assert_eq!(started.phase(), ActivityLifecyclePhase::Started);
    assert_eq!(started.status(), ActivityStatus::Pending);
    assert_eq!(started.activity_id(), accepted.activity_id());

    let running = registry
        .mark_running(accepted.watcher_id())
        .expect("running");
    let WatcherActivityProjection::Activity(updated) =
        project_watcher_activity(&turn, &running).expect("running projection")
    else {
        panic!("{WATCHER_RULE}: running must project as activity");
    };
    assert_eq!(updated.phase(), ActivityLifecyclePhase::Updated);
    assert_eq!(updated.status(), ActivityStatus::InProgress);

    let terminal = registry
        .complete(accepted.watcher_id(), WatcherTerminalCause::Completed, None)
        .expect("terminal");
    let WatcherActivityProjection::Activity(completed) =
        project_watcher_activity(&turn, &terminal).expect("terminal projection")
    else {
        panic!("{WATCHER_RULE}: terminal must project as activity");
    };
    assert_eq!(completed.phase(), ActivityLifecyclePhase::Completed);
    assert_eq!(completed.status(), ActivityStatus::Completed);

    let joined = registry.join(accepted.watcher_id()).expect("join");
    let WatcherActivityProjection::Joined {
        activity_id,
        terminal_cause,
        revision,
    } = project_watcher_activity(&turn, &joined).expect("joined projection")
    else {
        panic!("{WATCHER_RULE}: joined must not emit a second completed activity");
    };
    assert_eq!(&activity_id, joined.activity_id());
    assert_eq!(terminal_cause, WatcherTerminalCause::Completed);
    assert_eq!(revision, joined.revision());
}
