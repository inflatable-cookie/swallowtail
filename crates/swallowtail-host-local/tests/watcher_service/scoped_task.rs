use super::support::block_on;
use super::{operation_data, runtime_turn, watcher_host, watcher_owning_turn};
use std::sync::{Arc, Mutex};
use swallowtail_core::{WatcherCleanupCause, WatcherLifecyclePhase, WatcherRequester};
use swallowtail_runtime::{CleanupOutcome, ScopeId, ScopedTaskService, WatcherSnapshot};

#[test]
fn watcher_host_methods_succeed_inside_a_scoped_task_executor() {
    let local = watcher_host("sleep", 2);
    let watcher = Arc::clone(
        local
            .services()
            .watcher()
            .expect("local composition includes watcher"),
    );
    let task_service = local.task_service().clone();
    let outcome = Arc::new(Mutex::new(None));
    let outcome_for_task = Arc::clone(&outcome);
    let task_watcher = Arc::clone(&watcher);

    let task = task_service
        .spawn(
            ScopeId::new("swallowtail-watcher-scoped-task-proof").expect("scope"),
            Box::pin(async move {
                let started = task_watcher
                    .accept_start(
                        runtime_turn("turn-scoped-executor"),
                        WatcherRequester::Model,
                        operation_data("sleep-operation"),
                    )
                    .await
                    .expect("accept_start from scoped task returns RuntimeFailure or snapshot");
                let watcher_id = started.watcher_id().clone();
                let owning = watcher_owning_turn("turn-scoped-executor");
                let listed = task_watcher
                    .list(owning.clone())
                    .await
                    .expect("list from scoped task stays observable");
                assert_eq!(listed.len(), 1);
                assert_eq!(listed[0].watcher_id(), &watcher_id);

                let (stopped, snapshot) = task_watcher
                    .request_stop(owning, watcher_id.clone())
                    .await
                    .expect("request_stop from scoped task");
                assert_eq!(
                    stopped,
                    swallowtail_runtime::WatcherStopAcknowledgement::Stopped
                );
                assert_eq!(snapshot.phase(), WatcherLifecyclePhase::Terminal);

                let (snapshots, cleanup) = task_watcher
                    .stop_and_join_all(
                        runtime_turn("turn-scoped-executor"),
                        WatcherCleanupCause::Cancelled,
                    )
                    .await
                    .expect("stop_and_join_all from scoped task");
                assert_eq!(cleanup, CleanupOutcome::Clean);
                assert_eq!(snapshots.len(), 1);
                assert_eq!(snapshots[0].phase(), WatcherLifecyclePhase::Joined);

                *outcome_for_task.lock().expect("outcome lock") =
                    Some((started, snapshots[0].clone()));
            }),
        )
        .expect("scoped task starts");

    block_on(task.join()).expect("scoped task joins without nested-executor panic");
    let (started, joined) = outcome
        .lock()
        .expect("outcome lock")
        .take()
        .expect("scoped task recorded watcher outcomes");
    assert_eq!(started.phase(), WatcherLifecyclePhase::Running);
    assert_eq!(joined.phase(), WatcherLifecyclePhase::Joined);

    let retired = block_on(watcher.list(watcher_owning_turn("turn-scoped-executor")))
        .expect_err("joined cleanup retires the turn exactly once");
    assert_eq!(
        retired.diagnostic().code(),
        "swallowtail.local_watcher.turn_retired"
    );

    // Ordinary calls outside a scoped task keep their existing ready path.
    let outside = block_on(watcher.accept_start(
        runtime_turn("turn-outside-executor"),
        WatcherRequester::Operator,
        operation_data("sleep-operation"),
    ))
    .expect("accept_start outside a scoped task still works");
    assert!(matches!(
        outside.phase(),
        WatcherLifecyclePhase::Accepted | WatcherLifecyclePhase::Running
    ));
    let (_, cleanup) = block_on(watcher.stop_and_join_all(
        runtime_turn("turn-outside-executor"),
        WatcherCleanupCause::Cancelled,
    ))
    .expect("outside stop_and_join_all still cleans once");
    assert_eq!(cleanup, CleanupOutcome::Clean);
    let _: WatcherSnapshot = outside;
}
