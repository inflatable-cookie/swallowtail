use super::support::block_on;
use super::{operation_data, runtime_turn, watcher_host, watcher_owning_turn};
use std::future::Future;
use std::task::{Context, Poll, Waker};
use std::time::Duration;
use swallowtail_core::{
    WatcherCleanupCause, WatcherLifecyclePhase, WatcherRequester, WatcherTerminalCause,
};
use swallowtail_runtime::{CleanupOutcome, WatcherWaitOptions, WatcherWaitRepresentation};

#[test]
fn watcher_stop_is_idempotent_and_wait_reports_the_joined_stop() {
    let local = watcher_host("sleep", 2);
    let watcher = local
        .services()
        .watcher()
        .expect("local composition includes watcher");
    let turn = runtime_turn("turn-stop");
    let owning_turn = watcher_owning_turn("turn-stop");
    let watcher_id = block_on(watcher.accept_start(
        turn,
        WatcherRequester::Model,
        operation_data("sleep-operation"),
    ))
    .expect("stoppable watcher starts")
    .watcher_id()
    .clone();

    let (acknowledgement, stopped) =
        block_on(watcher.request_stop(owning_turn.clone(), watcher_id.clone()))
            .expect("first stop is accepted");
    assert_eq!(
        acknowledgement,
        swallowtail_runtime::WatcherStopAcknowledgement::Stopped
    );
    assert_eq!(stopped.phase(), WatcherLifecyclePhase::Terminal);
    assert_eq!(
        stopped.terminal_cause(),
        Some(WatcherTerminalCause::Stopped)
    );
    let (repeat, repeated_snapshot) =
        block_on(watcher.request_stop(owning_turn.clone(), watcher_id.clone()))
            .expect("repeated stop is idempotent");
    assert_eq!(
        repeat,
        swallowtail_runtime::WatcherStopAcknowledgement::AlreadyTerminal(
            WatcherTerminalCause::Stopped
        )
    );
    assert_eq!(repeated_snapshot.phase(), WatcherLifecyclePhase::Terminal);
    assert_eq!(
        block_on(watcher.wait(owning_turn, watcher_id, WatcherWaitOptions::default(),))
            .expect("stop joins before wait resolves"),
        WatcherWaitRepresentation::Satisfied(WatcherTerminalCause::Stopped)
    );
}

#[test]
fn watcher_operator_stop_is_idempotent_against_the_same_registry() {
    let local = watcher_host("sleep", 2);
    let watcher = local
        .services()
        .watcher()
        .expect("local composition includes watcher");
    let turn = runtime_turn("turn-operator-stop");
    let owning_turn = watcher_owning_turn("turn-operator-stop");
    let watcher_id = block_on(watcher.accept_start(
        turn,
        WatcherRequester::Operator,
        operation_data("sleep-operation"),
    ))
    .expect("operator-started watcher starts")
    .watcher_id()
    .clone();

    let (acknowledgement, stopped) =
        block_on(watcher.request_stop(owning_turn.clone(), watcher_id.clone()))
            .expect("operator stop is accepted");
    assert_eq!(
        acknowledgement,
        swallowtail_runtime::WatcherStopAcknowledgement::Stopped
    );
    assert_eq!(stopped.phase(), WatcherLifecyclePhase::Terminal);
    assert_eq!(
        stopped.terminal_cause(),
        Some(WatcherTerminalCause::Stopped)
    );
    let (repeat, repeated_snapshot) =
        block_on(watcher.request_stop(owning_turn.clone(), watcher_id.clone()))
            .expect("repeated operator stop is idempotent");
    assert_eq!(
        repeat,
        swallowtail_runtime::WatcherStopAcknowledgement::AlreadyTerminal(
            WatcherTerminalCause::Stopped
        )
    );
    assert_eq!(repeated_snapshot.phase(), WatcherLifecyclePhase::Terminal);
    assert_eq!(
        block_on(watcher.wait(owning_turn, watcher_id, WatcherWaitOptions::default(),))
            .expect("operator stop joins before wait resolves"),
        WatcherWaitRepresentation::Satisfied(WatcherTerminalCause::Stopped)
    );
}

#[test]
fn watcher_capacity_and_foreign_stop_fail_closed() {
    let local = watcher_host("sleep", 1);
    let watcher = local
        .services()
        .watcher()
        .expect("local composition includes watcher");
    let operation = operation_data("sleep-operation");
    let turn = runtime_turn("turn-capacity");
    let first =
        block_on(watcher.accept_start(turn.clone(), WatcherRequester::Operator, operation.clone()))
            .expect("first watcher starts");
    let capacity = block_on(watcher.accept_start(turn, WatcherRequester::Model, operation))
        .expect_err("second watcher must exceed the host turn bound");
    assert_eq!(
        capacity.diagnostic().code(),
        "swallowtail.local_watcher.capacity_rejected"
    );

    let foreign_started = block_on(watcher.accept_start(
        runtime_turn("turn-foreign"),
        WatcherRequester::Model,
        operation_data("sleep-operation"),
    ))
    .expect("foreign turn watcher starts");
    let foreign = block_on(watcher.request_stop(
        watcher_owning_turn("turn-foreign"),
        first.watcher_id().clone(),
    ))
    .expect_err("foreign owning turn cannot stop a watcher");
    assert_eq!(
        foreign.diagnostic().code(),
        "swallowtail.local_watcher.identity_rejected"
    );

    let (snapshots, cleanup) = block_on(watcher.stop_and_join_all(
        runtime_turn("turn-capacity"),
        WatcherCleanupCause::Cancelled,
    ))
    .expect("turn cleanup joins the owned watcher");
    assert_eq!(cleanup, CleanupOutcome::Clean);
    assert_eq!(snapshots.len(), 1);
    assert_eq!(snapshots[0].phase(), WatcherLifecyclePhase::Joined);
    assert_eq!(
        snapshots[0].terminal_cause(),
        Some(WatcherTerminalCause::Cancelled)
    );
    let (_, foreign_cleanup) = block_on(
        watcher.stop_and_join_all(runtime_turn("turn-foreign"), WatcherCleanupCause::Cancelled),
    )
    .expect("foreign turn cleanup joins its own watcher");
    assert_eq!(foreign_cleanup, CleanupOutcome::Clean);
    assert_eq!(foreign_started.accepted_by(), WatcherRequester::Model);
}

#[test]
fn watcher_stop_and_join_retires_owned_identities() {
    let local = watcher_host("sleep", 2);
    let watcher = local
        .services()
        .watcher()
        .expect("local composition includes watcher");
    let turn = runtime_turn("turn-tree");
    let owning_turn = watcher_owning_turn("turn-tree");
    let watcher_id = block_on(watcher.accept_start(
        turn.clone(),
        WatcherRequester::Operator,
        operation_data("sleep-operation"),
    ))
    .expect("managed watcher starts")
    .watcher_id()
    .clone();

    let (snapshots, cleanup) =
        block_on(watcher.stop_and_join_all(turn, WatcherCleanupCause::TimedOut))
            .expect("deadline cleanup stops and joins managed work");
    assert_eq!(cleanup, CleanupOutcome::Clean);
    assert_eq!(snapshots[0].watcher_id(), &watcher_id);
    assert_eq!(snapshots[0].phase(), WatcherLifecyclePhase::Joined);
    assert_eq!(
        snapshots[0].terminal_cause(),
        Some(WatcherTerminalCause::TimedOut)
    );
    let failure = block_on(watcher.inspect(owning_turn.clone(), watcher_id.clone()))
        .expect_err("retired turn rejects stale inspect controls");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.local_watcher.turn_retired"
    );
    let failure = block_on(watcher.list(owning_turn.clone()))
        .expect_err("retired turn rejects stale list controls");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.local_watcher.turn_retired"
    );
    let failure = block_on(watcher.wait(
        owning_turn.clone(),
        watcher_id.clone(),
        WatcherWaitOptions::default(),
    ))
    .expect_err("retired turn rejects stale wait controls");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.local_watcher.turn_retired"
    );
    let failure = block_on(watcher.request_stop(owning_turn, watcher_id))
        .expect_err("retired turn rejects stale stop controls");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.local_watcher.turn_retired"
    );
}

#[test]
fn watcher_wait_is_pending_before_poll_and_drop_allows_deadline_cleanup() {
    let local = watcher_host("sleep", 2);
    let watcher = local
        .services()
        .watcher()
        .expect("local composition includes watcher");
    let turn = runtime_turn("turn-wait-pending");
    let owning_turn = watcher_owning_turn("turn-wait-pending");
    let watcher_id = block_on(watcher.accept_start(
        turn.clone(),
        WatcherRequester::Model,
        operation_data("sleep-operation"),
    ))
    .expect("watcher starts")
    .watcher_id()
    .clone();

    let started = std::time::Instant::now();
    let mut wait = Box::pin(watcher.wait(owning_turn, watcher_id, WatcherWaitOptions::default()));
    let waker = Waker::noop();
    let mut context = Context::from_waker(waker);
    assert!(matches!(wait.as_mut().poll(&mut context), Poll::Pending));
    assert!(
        started.elapsed() < Duration::from_millis(100),
        "wait construction and its first poll must not join synchronously"
    );
    drop(wait);

    let (snapshots, cleanup) =
        block_on(watcher.stop_and_join_all(turn, WatcherCleanupCause::TimedOut))
            .expect("deadline cleanup joins the dropped wait's watcher");
    assert_eq!(cleanup, CleanupOutcome::Clean);
    assert_eq!(snapshots[0].phase(), WatcherLifecyclePhase::Joined);
}

#[test]
fn watcher_process_groups_are_foreign_safe() {
    let local = watcher_host("sleep", 2);
    let watcher = local
        .services()
        .watcher()
        .expect("local composition includes watcher");
    let first_turn = runtime_turn("turn-foreign-group-a");
    let second_turn = runtime_turn("turn-foreign-group-b");
    let first = block_on(watcher.accept_start(
        first_turn.clone(),
        WatcherRequester::Model,
        operation_data("sleep-operation"),
    ))
    .expect("first watcher starts");
    let second = block_on(watcher.accept_start(
        second_turn.clone(),
        WatcherRequester::Model,
        operation_data("sleep-operation"),
    ))
    .expect("second watcher starts");

    let (_, first_cleanup) =
        block_on(watcher.stop_and_join_all(first_turn, WatcherCleanupCause::Cancelled))
            .expect("first group cleans up");
    assert_eq!(first_cleanup, CleanupOutcome::Clean);
    let (acknowledgement, snapshot) = block_on(watcher.request_stop(
        watcher_owning_turn("turn-foreign-group-b"),
        second.watcher_id().clone(),
    ))
    .expect("first group cleanup cannot stop the foreign group");
    assert_eq!(
        acknowledgement,
        swallowtail_runtime::WatcherStopAcknowledgement::Stopped
    );
    assert_eq!(snapshot.watcher_id(), second.watcher_id());
    let (_, second_cleanup) =
        block_on(watcher.stop_and_join_all(second_turn, WatcherCleanupCause::Cancelled))
            .expect("second group cleans up");
    assert_eq!(second_cleanup, CleanupOutcome::Clean);
    assert_ne!(first.watcher_id(), second.watcher_id());
}

#[cfg(unix)]
#[test]
fn watcher_cleanup_stops_cooperative_child_processes() {
    use std::time::{Duration, Instant};
    use swallowtail_core::ExecutionHostId;
    use swallowtail_host_local::{LocalProcessHost, LocalProcessLimits};
    use swallowtail_runtime::{ProcessRequest, WorkingResourceRef};

    let resource_directory = super::support::temporary_resource();
    let executable = super::support::executable_ref();
    let environment = super::support::environment_ref();
    let resource = WorkingResourceRef::new("fixture.watcher.cooperative-child")
        .expect("working resource is valid");
    let operation = super::operation_data("sleep-with-cooperative-child-operation");
    let request = ProcessRequest::new(executable.clone())
        .with_arguments(super::support::fixture_arguments())
        .with_environment([environment.clone()])
        .with_working_resource(resource.clone());
    let local = LocalProcessHost::builder(LocalProcessLimits::default())
        .with_watcher_capacity(1)
        .approve_executable(
            executable,
            std::env::current_exe().expect("watcher fixture executable"),
        )
        .approve_environment(
            environment,
            super::support::fixture_environment("sleep-with-cooperative-child"),
        )
        .approve_working_resource(resource, resource_directory.clone())
        .approve_watcher_operation(operation.clone(), request)
        .build_services(
            ExecutionHostId::new("fixture.host.watcher.cooperative-child")
                .expect("watcher host id is valid"),
        );
    let watcher = local
        .services()
        .watcher()
        .expect("local composition includes watcher");
    let turn = runtime_turn("turn-cooperative-child");
    let _watcher_id =
        block_on(watcher.accept_start(turn.clone(), WatcherRequester::Operator, operation))
            .expect("cooperative-child watcher starts")
            .watcher_id()
            .clone();

    let pid_path = resource_directory.join("cooperative-child.pid");
    let deadline = Instant::now() + Duration::from_secs(2);
    let pid = loop {
        if let Ok(contents) = std::fs::read_to_string(&pid_path) {
            break contents
                .trim()
                .parse::<u32>()
                .expect("cooperative child pid is numeric");
        }
        assert!(
            Instant::now() < deadline,
            "cooperative child pid file was not written"
        );
        std::thread::sleep(Duration::from_millis(10));
    };
    assert!(
        process_is_alive(pid),
        "cooperative child is live before watcher cleanup"
    );

    let (snapshots, cleanup) =
        block_on(watcher.stop_and_join_all(turn, WatcherCleanupCause::Cancelled))
            .expect("watcher cleanup joins the managed process group");
    assert_eq!(cleanup, CleanupOutcome::Clean);
    assert_eq!(snapshots[0].phase(), WatcherLifecyclePhase::Joined);
    assert!(
        !process_is_alive(pid),
        "cooperative child is stopped with the owned process group"
    );
    std::fs::remove_dir_all(resource_directory).expect("fixture resource is removed");
}

#[cfg(unix)]
fn process_is_alive(pid: u32) -> bool {
    std::process::Command::new("/bin/ps")
        .args(["-p", &pid.to_string(), "-o", "pid="])
        .output()
        .map(|output| output.status.success() && !output.stdout.iter().all(u8::is_ascii_whitespace))
        .unwrap_or(false)
}
