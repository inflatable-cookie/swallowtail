use super::*;
use futures_executor::block_on;
use std::sync::mpsc::RecvTimeoutError;
use std::time::{Duration, Instant};
use swallowtail_runtime::{CancellationControl, DiscoveryCancellation};

#[test]
fn relinquish_returns_while_stalled_and_reaps_after_completion() {
    let service = LocalScopedTaskService::new(host("fixture.host.relinquish"));
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
}

#[test]
fn relinquish_fails_closed_for_wrong_authority_and_repeat_transfer() {
    let owning_service = LocalScopedTaskService::new(host("fixture.host.owner"));
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
}

#[test]
fn finished_tasks_remain_ordinary_joins() {
    let service = LocalScopedTaskService::new(host("fixture.host.finished"));
    let scope = scope("finished");
    let (finished_sender, finished_receiver) = mpsc::channel();
    let mut task = Some(
        service
            .spawn(
                scope.clone(),
                Box::pin(async move {
                    finished_sender
                        .send(())
                        .expect("test still receives completion");
                }),
            )
            .expect("task starts"),
    );
    finished_receiver
        .recv_timeout(Duration::from_secs(1))
        .expect("task completes");
    wait_until(|| task.as_ref().is_some_and(|task| task.is_finished()));

    let failure = service
        .relinquish(&scope, &mut task)
        .expect_err("finished task is not relabeled as relinquished");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.local_task.already_finished"
    );
    block_on(task.take().expect("caller retains task").join())
        .expect("finished task joins normally");
}

fn wait_until(predicate: impl Fn() -> bool) {
    let deadline = Instant::now() + Duration::from_secs(1);
    while !predicate() {
        assert!(Instant::now() < deadline, "condition was not observed");
        thread::yield_now();
    }
}

fn host(value: &str) -> ExecutionHostId {
    ExecutionHostId::new(value).expect("execution host id is valid")
}

fn scope(value: &str) -> ScopeId {
    ScopeId::new(value).expect("scope id is valid")
}
