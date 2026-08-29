use super::super::{JoinSignal, LocalWatcherHostService};
use super::containment::TestContainmentBackend;
use crate::host::LocalProcessHost;
use crate::task::LocalScopedTaskService;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::{Context, Poll, Wake, Waker};
use std::thread;
use std::time::{Duration, Instant};
use swallowtail_core::{
    ExecutionHostId, WatcherOperationData, WatcherOwningTurn, WatcherRequester,
};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, JoinedTask, ProcessRequest, RuntimeFailure, RuntimeTurnId, ScopeId,
    ScopedTaskService, WatcherHostService, WatcherWaitOptions,
};

struct PanicAfterTaskService {
    delegate: LocalScopedTaskService,
}

impl ScopedTaskService for PanicAfterTaskService {
    fn spawn(
        &self,
        scope: ScopeId,
        task: BoxFuture<'static, ()>,
    ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
        self.delegate.spawn(
            scope,
            Box::pin(async move {
                thread::sleep(Duration::from_millis(50));
                task.await;
                panic!("fixture monitor task panic");
            }),
        )
    }
}

struct CountingWake(Arc<AtomicUsize>);

impl Wake for CountingWake {
    fn wake(self: Arc<Self>) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

#[test]
fn watcher_join_signal_wakes_all_registered_waiters() {
    let first_wakes = Arc::new(AtomicUsize::new(0));
    let second_wakes = Arc::new(AtomicUsize::new(0));
    let first_waker = Waker::from(Arc::new(CountingWake(Arc::clone(&first_wakes))));
    let second_waker = Waker::from(Arc::new(CountingWake(Arc::clone(&second_wakes))));
    let signal = JoinSignal::default();

    signal.register(&first_waker);
    signal.register(&second_waker);
    signal.notify();

    assert_eq!(first_wakes.load(Ordering::SeqCst), 1);
    assert_eq!(second_wakes.load(Ordering::SeqCst), 1);
}

#[test]
fn concurrent_waiters_are_all_woken_when_the_monitor_finishes() {
    let execution_host =
        ExecutionHostId::new("fixture.watcher.concurrent-wait").expect("host id is valid");
    let executable =
        swallowtail_runtime::ExecutableRef::new("fixture.sleep").expect("executable is valid");
    let operation =
        WatcherOperationData::new("fixture.watcher.concurrent-wait.sleep").expect("operation");
    let process_host = Arc::new(
        LocalProcessHost::builder(crate::limits::LocalProcessLimits::default())
            .approve_executable(executable.clone(), "/bin/sleep")
            .approve_watcher_operation(
                operation.clone(),
                ProcessRequest::new(executable).with_arguments(["30".to_owned()]),
            )
            .with_process_containment_factory(|host| Arc::new(TestContainmentBackend::new(host)))
            .build(),
    );
    let containment = process_host.process_containment().cloned();
    let watcher = LocalWatcherHostService::new(
        process_host,
        Arc::new(LocalScopedTaskService::new(execution_host)),
        2,
        containment,
    );
    let turn = RuntimeTurnId::new("fixture.watcher.concurrent-wait.turn").expect("turn");
    let owning_turn = WatcherOwningTurn::new(turn.as_str()).expect("owning turn");
    let watcher_id = futures_executor::block_on(watcher.accept_start(
        turn.clone(),
        WatcherRequester::Model,
        operation,
    ))
    .expect("watcher starts")
    .watcher_id()
    .clone();

    let first_wakes = Arc::new(AtomicUsize::new(0));
    let second_wakes = Arc::new(AtomicUsize::new(0));
    let first_waker = Waker::from(Arc::new(CountingWake(Arc::clone(&first_wakes))));
    let second_waker = Waker::from(Arc::new(CountingWake(Arc::clone(&second_wakes))));
    let request_watcher_id = watcher_id.clone();
    let mut first_wait = Box::pin(watcher.wait(
        owning_turn.clone(),
        watcher_id.clone(),
        WatcherWaitOptions::default(),
    ));
    let mut second_wait =
        Box::pin(watcher.wait(owning_turn, watcher_id, WatcherWaitOptions::default()));
    let mut first_context = Context::from_waker(&first_waker);
    let mut second_context = Context::from_waker(&second_waker);
    assert!(matches!(
        first_wait.as_mut().poll(&mut first_context),
        Poll::Pending
    ));
    assert!(matches!(
        second_wait.as_mut().poll(&mut second_context),
        Poll::Pending
    ));

    futures_executor::block_on(watcher.request_stop(
        WatcherOwningTurn::new(turn.as_str()).expect("owning turn"),
        request_watcher_id,
    ))
    .expect("stop request is accepted");
    let deadline = Instant::now() + Duration::from_secs(3);
    while (first_wakes.load(Ordering::SeqCst) == 0 || second_wakes.load(Ordering::SeqCst) == 0)
        && Instant::now() < deadline
    {
        thread::sleep(Duration::from_millis(10));
    }
    assert!(
        first_wakes.load(Ordering::SeqCst) > 0,
        "first wait was woken"
    );
    assert!(
        second_wakes.load(Ordering::SeqCst) > 0,
        "second wait was woken"
    );

    assert_eq!(
        futures_executor::block_on(first_wait).expect("first wait joins"),
        swallowtail_runtime::WatcherWaitRepresentation::Satisfied(
            swallowtail_core::WatcherTerminalCause::Stopped,
        )
    );
    assert_eq!(
        futures_executor::block_on(second_wait).expect("second wait observes joined truth"),
        swallowtail_runtime::WatcherWaitRepresentation::Satisfied(
            swallowtail_core::WatcherTerminalCause::Stopped,
        )
    );
    assert_eq!(
        futures_executor::block_on(watcher.finalize_turn(turn)).expect("turn is joined"),
        CleanupOutcome::Clean
    );
}

#[test]
fn panicking_monitor_wakes_wait_and_surfaces_join_failure() {
    let execution_host = ExecutionHostId::new("fixture.watcher.panic").expect("host id is valid");
    let executable =
        swallowtail_runtime::ExecutableRef::new("fixture.exit").expect("executable is valid");
    let operation =
        WatcherOperationData::new("fixture.watcher.panic.exit").expect("operation is valid");
    let process_host = Arc::new(
        LocalProcessHost::builder(crate::limits::LocalProcessLimits::default())
            .approve_executable(executable.clone(), "/bin/sh")
            .approve_watcher_operation(
                operation.clone(),
                ProcessRequest::new(executable)
                    .with_arguments(["-c".to_owned(), "exit 0".to_owned()]),
            )
            .with_process_containment_factory(|host| Arc::new(TestContainmentBackend::new(host)))
            .build(),
    );
    let task_service = Arc::new(PanicAfterTaskService {
        delegate: LocalScopedTaskService::new(execution_host),
    });
    let containment = process_host.process_containment().cloned();
    let watcher =
        LocalWatcherHostService::new_with_task_service(process_host, task_service, 2, containment);
    let turn = RuntimeTurnId::new("fixture.watcher.panic.turn").expect("turn");
    let owning_turn = WatcherOwningTurn::new(turn.as_str()).expect("owning turn");
    let watcher_id = futures_executor::block_on(watcher.accept_start(
        turn.clone(),
        WatcherRequester::Model,
        operation,
    ))
    .expect("watcher starts")
    .watcher_id()
    .clone();

    let wakes = Arc::new(AtomicUsize::new(0));
    let waker = Waker::from(Arc::new(CountingWake(Arc::clone(&wakes))));
    let mut context = Context::from_waker(&waker);
    let mut wait = Box::pin(watcher.wait(owning_turn, watcher_id, WatcherWaitOptions::default()));
    assert!(matches!(wait.as_mut().poll(&mut context), Poll::Pending));
    let deadline = Instant::now() + Duration::from_secs(2);
    while wakes.load(Ordering::SeqCst) == 0 && Instant::now() < deadline {
        thread::sleep(Duration::from_millis(10));
    }
    assert!(
        wakes.load(Ordering::SeqCst) > 0,
        "panic completion wakes the wait"
    );
    let failure = futures_executor::block_on(wait).expect_err("panic is not clean joined truth");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.local_task.panicked"
    );
    drop(failure);
    let (_, cleanup) = futures_executor::block_on(
        watcher.stop_and_join_all(turn, swallowtail_core::WatcherCleanupCause::Cancelled),
    )
    .expect("cleanup returns its failed outcome");
    match cleanup {
        CleanupOutcome::Failed(diagnostic) | CleanupOutcome::Degraded(diagnostic) => {
            assert_eq!(diagnostic.code(), "swallowtail.local_task.panicked");
        }
        CleanupOutcome::Clean | CleanupOutcome::NotApplicable => {
            panic!("panicking monitor cannot cleanly join")
        }
    }
}
