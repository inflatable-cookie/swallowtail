use super::*;
use futures_executor::block_on;
use std::sync::mpsc::{self, RecvTimeoutError};
use std::time::{Duration, Instant};
use swallowtail_runtime::ScopedTaskService;

mod relinquishment;

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
