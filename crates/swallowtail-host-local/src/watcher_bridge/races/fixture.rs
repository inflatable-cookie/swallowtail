use super::super::LocalWatcherBridgeHostService;
use crate::host::LocalProcessHost;
use crate::limits::LocalProcessLimits;
use crate::task::LocalScopedTaskService;
use crate::watcher::LocalWatcherHostService;
use futures_executor::block_on;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Poll, Waker};
use std::thread;
use swallowtail_core::{
    ExecutionHostId, SafeDiagnostic, WatcherCleanupCause, WatcherId, WatcherOperationData,
    WatcherOwningTurn, WatcherRequester,
};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, ProcessRequest, RuntimeFailure, RuntimeTurnId, WatcherHostService,
    WatcherSnapshot, WatcherStopAcknowledgement, WatcherWaitOptions, WatcherWaitRepresentation,
};

pub(super) struct Hold {
    ready: AtomicBool,
    waiters: Mutex<Vec<Waker>>,
}

impl Hold {
    pub(super) fn new() -> Self {
        Self {
            ready: AtomicBool::new(false),
            waiters: Mutex::new(Vec::new()),
        }
    }

    pub(super) fn is_ready(&self) -> bool {
        self.ready.load(Ordering::SeqCst)
    }

    pub(super) fn register(&self, waker: &Waker) {
        let mut waiters = self.waiters.lock().expect("hold waiters");
        if !waiters.iter().any(|waiter| waiter.will_wake(waker)) {
            waiters.push(waker.clone());
        }
    }

    pub(super) fn wait(&self) -> impl Future<Output = ()> + '_ {
        std::future::poll_fn(|context| {
            if self.is_ready() {
                return Poll::Ready(());
            }
            self.register(context.waker());
            if self.is_ready() {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
    }

    pub(super) fn release(&self) {
        self.ready.store(true, Ordering::SeqCst);
        for waiter in self.waiters.lock().expect("hold waiters").drain(..) {
            waiter.wake();
        }
    }
}

pub(super) struct ControllableWatcher {
    inner: Arc<LocalWatcherHostService>,
    pub(super) start_hold: Hold,
    pub(super) wait_hold: Hold,
    pub(super) start_entered: AtomicBool,
    pub(super) wait_entered: AtomicBool,
    pub(super) stop_all_error: Mutex<Option<RuntimeFailure>>,
    pub(super) wait_override: Mutex<Option<WatcherWaitRepresentation>>,
}

impl WatcherHostService for ControllableWatcher {
    fn accept_start(
        &self,
        turn: RuntimeTurnId,
        requester: WatcherRequester,
        operation_data: WatcherOperationData,
    ) -> BoxFuture<'_, Result<WatcherSnapshot, RuntimeFailure>> {
        Box::pin(async move {
            self.start_entered.store(true, Ordering::SeqCst);
            self.start_hold.wait().await;
            let inner = Arc::clone(&self.inner);
            thread::spawn(move || block_on(inner.accept_start(turn, requester, operation_data)))
                .join()
                .unwrap_or_else(|_| {
                    Err(RuntimeFailure::new(SafeDiagnostic::new(
                        "fixture.bridge.start_thread",
                        "Fixture start thread failed",
                    )))
                })
        })
    }

    fn inspect(
        &self,
        owning_turn: WatcherOwningTurn,
        watcher_id: WatcherId,
    ) -> BoxFuture<'_, Result<WatcherSnapshot, RuntimeFailure>> {
        self.inner.inspect(owning_turn, watcher_id)
    }

    fn list(
        &self,
        owning_turn: WatcherOwningTurn,
    ) -> BoxFuture<'_, Result<Vec<WatcherSnapshot>, RuntimeFailure>> {
        self.inner.list(owning_turn)
    }

    fn wait<'a>(
        &'a self,
        owning_turn: WatcherOwningTurn,
        watcher_id: WatcherId,
        mut options: WatcherWaitOptions<'a>,
    ) -> BoxFuture<'a, Result<WatcherWaitRepresentation, RuntimeFailure>> {
        Box::pin(async move {
            self.wait_entered.store(true, Ordering::SeqCst);
            let interrupted = std::future::poll_fn(|context| {
                if let Some(outcome) = options.poll(context) {
                    return Poll::Ready(Some(outcome));
                }
                if self.wait_hold.is_ready() {
                    return Poll::Ready(None);
                }
                self.wait_hold.register(context.waker());
                if self.wait_hold.is_ready() {
                    Poll::Ready(None)
                } else {
                    Poll::Pending
                }
            })
            .await;
            if let Some(outcome) = interrupted {
                return Ok(outcome);
            }
            if let Some(outcome) = self.wait_override.lock().expect("wait override").take() {
                return Ok(outcome);
            }
            self.inner.wait(owning_turn, watcher_id, options).await
        })
    }

    fn request_stop(
        &self,
        owning_turn: WatcherOwningTurn,
        watcher_id: WatcherId,
    ) -> BoxFuture<'_, Result<(WatcherStopAcknowledgement, WatcherSnapshot), RuntimeFailure>> {
        self.inner.request_stop(owning_turn, watcher_id)
    }

    fn stop_and_join_all(
        &self,
        turn: RuntimeTurnId,
        cause: WatcherCleanupCause,
    ) -> BoxFuture<'_, Result<(Vec<WatcherSnapshot>, CleanupOutcome), RuntimeFailure>> {
        if let Some(error) = self.stop_all_error.lock().expect("stop error").take() {
            return Box::pin(async move { Err(error) });
        }
        self.inner.stop_and_join_all(turn, cause)
    }

    fn finalize_turn(
        &self,
        turn: RuntimeTurnId,
    ) -> BoxFuture<'_, Result<CleanupOutcome, RuntimeFailure>> {
        self.inner.finalize_turn(turn)
    }
}

pub(super) fn fixture(label: &str) -> (LocalWatcherBridgeHostService, Arc<ControllableWatcher>) {
    let host_id = ExecutionHostId::new(format!("fixture.bridge.race.{label}")).expect("host");
    let executable =
        swallowtail_runtime::ExecutableRef::new("fixture.bridge.race").expect("executable");
    let operation = WatcherOperationData::new("sleep-operation").expect("operation");
    let request = ProcessRequest::new(executable.clone()).with_arguments(["30".to_owned()]);
    let mut builder = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_executable(executable, "/bin/sleep")
        .approve_watcher_operation(operation, request);
    builder.execution_host_id = Some(host_id.clone());
    let process = Arc::new(builder.build());
    let inner = Arc::new(LocalWatcherHostService::new(
        process.clone(),
        Arc::new(LocalScopedTaskService::new(host_id.clone())),
        4,
    ));
    let watcher = Arc::new(ControllableWatcher {
        inner,
        start_hold: Hold::new(),
        wait_hold: Hold::new(),
        start_entered: AtomicBool::new(false),
        wait_entered: AtomicBool::new(false),
        stop_all_error: Mutex::new(None),
        wait_override: Mutex::new(None),
    });
    (
        LocalWatcherBridgeHostService::new(host_id, watcher.clone(), process),
        watcher,
    )
}

pub(super) fn fixture_with_wait_bound(
    label: &str,
    wait_bound: std::time::Duration,
) -> (LocalWatcherBridgeHostService, Arc<ControllableWatcher>) {
    let (bridge, watcher) = fixture(label);
    (bridge.with_wait_bound(wait_bound), watcher)
}
