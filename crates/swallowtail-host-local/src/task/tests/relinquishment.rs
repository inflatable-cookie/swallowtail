use super::*;
use crate::{LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{CancellationControl, DiscoveryCancellation, TaskRelinquishOutcome};

#[test]
fn returns_while_stalled_and_reaps_after_completion() {
    let (service, owner) = owned_service("fixture.host.relinquish");
    let scope = scope("stalled");
    let cancellation = DiscoveryCancellation::new();
    let task_cancellation = cancellation.clone();
    let (started_sender, started_receiver) = mpsc::channel();
    let task = service
        .spawn_task(
            scope.clone(),
            Box::pin(async move {
                started_sender.send(()).expect("test still receives start");
                task_cancellation.wait_requested().await;
            }),
        )
        .expect("task starts");
    let reaped = Arc::clone(&task.reaped);
    let mut task = Some(Box::new(task) as Box<dyn JoinedTask>);
    started_receiver
        .recv_timeout(Duration::from_secs(1))
        .expect("task reaches its stall");

    let relinquish_service = service.clone();
    let relinquish_scope = scope.clone();
    let (result_sender, result_receiver) = mpsc::channel();
    let relinquisher = thread::spawn(move || {
        let result = relinquish_service.relinquish(&relinquish_scope, &mut task);
        result_sender
            .send((result, task))
            .expect("test still receives relinquishment");
    });
    let (result, task) = match result_receiver.recv_timeout(Duration::from_secs(1)) {
        Ok(result) => result,
        Err(RecvTimeoutError::Timeout) => {
            block_on(cancellation.request()).expect("stalled task is released");
            relinquisher.join().expect("relinquishment thread joins");
            panic!("relinquishment waited for the stalled task");
        }
        Err(RecvTimeoutError::Disconnected) => {
            panic!("relinquishment thread ended without a result")
        }
    };
    assert_eq!(
        result.expect("owning host accepts unfinished task"),
        TaskRelinquishOutcome::AcceptedForReap
    );
    assert!(
        task.is_none(),
        "successful transfer clears caller ownership"
    );
    assert!(!reaped.load(Ordering::Acquire));

    block_on(cancellation.request()).expect("stalled task is released");
    wait_until(|| reaped.load(Ordering::Acquire));
    relinquisher.join().expect("relinquishment thread joins");
    owner.shutdown().expect("outer host joins the reaper");
}

#[test]
fn captured_service_clone_cannot_own_or_deadlock_reaper_shutdown() {
    let local = LocalProcessHost::builder(LocalProcessLimits::default())
        .build_services(host("fixture.host.captured-service"));
    let service = local.task_service().as_ref().clone();
    let captured_service = service.clone();
    let scope = scope("captured-service");
    let cancellation = DiscoveryCancellation::new();
    let task_cancellation = cancellation.clone();
    let (started_sender, started_receiver) = mpsc::channel();
    let (task_finished_sender, task_finished_receiver) = mpsc::channel();
    let task = service
        .spawn_task(
            scope.clone(),
            Box::pin(async move {
                started_sender.send(()).expect("test still receives start");
                task_cancellation.wait_requested().await;
                drop(captured_service);
                task_finished_sender
                    .send(())
                    .expect("test still observes task completion");
            }),
        )
        .expect("task starts");
    let reaped = Arc::clone(&task.reaped);
    let mut task = Some(Box::new(task) as Box<dyn JoinedTask>);
    started_receiver
        .recv_timeout(Duration::from_secs(1))
        .expect("task reaches its stall");
    assert_eq!(
        service
            .relinquish(&scope, &mut task)
            .expect("owning host accepts unfinished task"),
        TaskRelinquishOutcome::AcceptedForReap
    );
    drop(service);

    let (shutdown_started_sender, shutdown_started_receiver) = mpsc::channel();
    let (shutdown_finished_sender, shutdown_finished_receiver) = mpsc::channel();
    let shutdown = thread::spawn(move || {
        shutdown_started_sender
            .send(())
            .expect("test still observes outer shutdown start");
        let result = local.shutdown_task_reapers();
        shutdown_finished_sender
            .send(result)
            .expect("test still observes outer shutdown completion");
    });
    shutdown_started_receiver
        .recv_timeout(Duration::from_secs(1))
        .expect("outer shutdown starts");
    assert_eq!(
        shutdown_finished_receiver.recv_timeout(Duration::from_millis(100)),
        Err(RecvTimeoutError::Timeout),
        "outer shutdown returned before its stalled reaper completed"
    );

    block_on(cancellation.request()).expect("stalled task is released");
    task_finished_receiver
        .recv_timeout(Duration::from_secs(1))
        .expect("task drops its captured service clone without deadlock");
    shutdown_finished_receiver
        .recv_timeout(Duration::from_secs(1))
        .expect("outer owner joins its reaper")
        .expect("reaper shutdown succeeds");
    assert!(reaped.load(Ordering::Acquire));
    shutdown.join().expect("shutdown thread joins");
}

#[test]
fn fails_closed_for_wrong_authority_and_repeat_transfer() {
    let (owning_service, owner) = owned_service("fixture.host.owner");
    let other_service = LocalScopedTaskService::new(host("fixture.host.other"));
    let owning_scope = scope("owned");
    let wrong_scope = scope("wrong");
    let cancellation = DiscoveryCancellation::new();
    let task_cancellation = cancellation.clone();
    let task = owning_service
        .spawn_task(
            owning_scope.clone(),
            Box::pin(async move {
                task_cancellation.wait_requested().await;
            }),
        )
        .expect("task starts");
    let reaped = Arc::clone(&task.reaped);
    let mut task = Some(Box::new(task) as Box<dyn JoinedTask>);

    let failure = owning_service
        .relinquish(&wrong_scope, &mut task)
        .expect_err("wrong scope is rejected");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.local_task.scope_mismatch"
    );
    assert!(task.is_some());

    let failure = other_service
        .relinquish(&owning_scope, &mut task)
        .expect_err("wrong host is rejected");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.local_task.execution_host_mismatch"
    );
    assert!(task.is_some());

    assert_eq!(
        owning_service
            .relinquish(&owning_scope, &mut task)
            .expect("exact owner accepts task"),
        TaskRelinquishOutcome::AcceptedForReap
    );
    let failure = owning_service
        .relinquish(&owning_scope, &mut task)
        .expect_err("repeat transfer is rejected");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.local_task.already_relinquished"
    );

    block_on(cancellation.request()).expect("relinquished task is released");
    wait_until(|| reaped.load(Ordering::Acquire));
    owner.shutdown().expect("outer host joins the reaper");
}

#[test]
fn retains_task_when_the_owning_reaper_lifecycle_is_gone() {
    let (service, owner) = owned_service("fixture.host.gone-owner");
    let scope = scope("gone-owner");
    let cancellation = DiscoveryCancellation::new();
    let task_cancellation = cancellation.clone();
    let task = service
        .spawn_task(
            scope.clone(),
            Box::pin(async move {
                task_cancellation.wait_requested().await;
            }),
        )
        .expect("task starts");
    let mut task = Some(Box::new(task) as Box<dyn JoinedTask>);
    drop(owner);

    let failure = service
        .relinquish(&scope, &mut task)
        .expect_err("a service cannot accept for its gone outer owner");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.local_task.reaper_unavailable"
    );
    assert!(task.is_some(), "failed handoff retains caller ownership");

    block_on(cancellation.request()).expect("retained task is released");
    block_on(task.take().expect("caller still owns task").join())
        .expect("caller joins retained task normally");
}

#[test]
fn retains_task_when_reaper_handoff_is_closed() {
    let (service, owner) = owned_service("fixture.host.closed-owner");
    owner.shutdown().expect("outer owner shuts down cleanly");
    let scope = scope("closed-owner");
    let cancellation = DiscoveryCancellation::new();
    let task_cancellation = cancellation.clone();
    let task = service
        .spawn_task(
            scope.clone(),
            Box::pin(async move {
                task_cancellation.wait_requested().await;
            }),
        )
        .expect("ordinary task starts after reaper shutdown");
    let mut task = Some(Box::new(task) as Box<dyn JoinedTask>);

    let failure = service
        .relinquish(&scope, &mut task)
        .expect_err("shut-down reaper rejects task handoff");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.local_task.reaper_shutdown"
    );
    assert!(task.is_some(), "failed handoff restores caller ownership");

    block_on(cancellation.request()).expect("retained task is released");
    block_on(task.take().expect("caller still owns task").join())
        .expect("caller joins retained task normally");
}

fn owned_service(value: &str) -> (LocalScopedTaskService, LocalTaskReaperOwner) {
    LocalScopedTaskService::with_reaper_owner(host(value))
}
