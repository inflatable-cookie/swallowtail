use super::reaper::ReapPermit;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, mpsc};
use std::task::Waker;
use std::thread::{self, JoinHandle};
use swallowtail_core::{ExecutionHostId, SafeDiagnostic};
use swallowtail_runtime::{BoxFuture, JoinedTask, RuntimeFailure, ScopeId};

pub(super) struct LocalJoinedTask {
    execution_host_id: ExecutionHostId,
    scope: ScopeId,
    worker: Option<JoinHandle<()>>,
    signal: Arc<JoinSignal>,
    reaped: Arc<AtomicBool>,
    reap_permit: Option<ReapPermit>,
}

impl LocalJoinedTask {
    pub(super) fn spawn(
        execution_host_id: ExecutionHostId,
        scope: ScopeId,
        task: BoxFuture<'static, ()>,
    ) -> Result<Self, RuntimeFailure> {
        let signal = Arc::new(JoinSignal::default());
        let worker_signal = Arc::clone(&signal);
        let worker = thread::Builder::new()
            .name("swallowtail-local-task".to_owned())
            .spawn(move || {
                let _notification = NotifyOnDrop(worker_signal);
                futures_executor::block_on(task);
            })
            .map_err(|_| {
                task_failure(
                    "swallowtail.local_task.spawn_failed",
                    "Local task could not be started",
                )
            })?;
        Ok(Self {
            execution_host_id,
            scope,
            worker: Some(worker),
            signal,
            reaped: Arc::new(AtomicBool::new(false)),
            reap_permit: None,
        })
    }

    pub(super) fn spawn_reapable(
        execution_host_id: ExecutionHostId,
        scope: ScopeId,
        task: BoxFuture<'static, ()>,
        reap_permit: ReapPermit,
    ) -> Result<Self, RuntimeFailure> {
        let signal = Arc::new(JoinSignal::default());
        let worker_signal = Arc::clone(&signal);
        let (start_sender, start_receiver) = mpsc::sync_channel(0);
        let reap_completion = reap_permit.completion();
        let worker = thread::Builder::new()
            .name("swallowtail-local-task".to_owned())
            .spawn(move || {
                let _reap_completion = reap_completion;
                if start_receiver.recv().is_ok() {
                    let _notification = NotifyOnDrop(worker_signal);
                    futures_executor::block_on(task);
                }
            })
            .map_err(|_| {
                task_failure(
                    "swallowtail.local_task.spawn_failed",
                    "Local task could not be started",
                )
            })?;
        let joined = Self {
            execution_host_id,
            scope,
            worker: Some(worker),
            signal,
            reaped: Arc::new(AtomicBool::new(false)),
            reap_permit: Some(reap_permit),
        };
        start_sender.send(()).map_err(|_| {
            task_failure(
                "swallowtail.local_task.start_failed",
                "Reservation-backed local task could not be started",
            )
        })?;
        Ok(joined)
    }
}

impl JoinedTask for LocalJoinedTask {
    fn join(mut self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        let worker = self.worker.take();
        let reaped = Arc::clone(&self.reaped);
        let Some(reap_permit) = self.reap_permit.take() else {
            return Box::pin(async move { reap_worker(worker, &reaped) });
        };
        let Some(worker) = worker else {
            drop(reap_permit);
            return Box::pin(async move { reap_worker(None, &reaped) });
        };
        match reap_permit.accept_for_join(worker, Arc::clone(&reaped)) {
            Ok(join) => join.into_future(),
            Err((_handoff_error, worker, reap_permit)) => {
                // A completed task may settle its reservation just before join.
                // Join it before returning so even an unpolled future cannot
                // discard a live worker. A failed reserved lane uses the same
                // ownership-preserving fallback.
                let outcome = reap_worker(Some(worker), &reaped);
                drop(reap_permit);
                Box::pin(async move { outcome })
            }
        }
    }

    fn is_finished(&self) -> bool {
        self.worker
            .as_ref()
            .is_none_or(std::thread::JoinHandle::is_finished)
    }

    fn register_waker(&self, waker: &Waker) {
        self.signal.register(waker);
    }

    fn relinquish_to_host(
        &mut self,
        execution_host_id: &ExecutionHostId,
        scope: &ScopeId,
    ) -> Result<(), RuntimeFailure> {
        if &self.execution_host_id != execution_host_id {
            return Err(task_failure(
                "swallowtail.local_task.execution_host_mismatch",
                "Local task belongs to a different execution host",
            ));
        }
        if &self.scope != scope {
            return Err(task_failure(
                "swallowtail.local_task.scope_mismatch",
                "Local task belongs to a different operation scope",
            ));
        }
        let worker = self.worker.as_ref().ok_or_else(|| {
            task_failure(
                "swallowtail.local_task.already_relinquished",
                "Local task ownership was already transferred",
            )
        })?;
        if worker.is_finished() {
            return Err(task_failure(
                "swallowtail.local_task.already_finished",
                "Finished local tasks must use ordinary join",
            ));
        }

        let reap_permit = self.reap_permit.take().ok_or_else(|| {
            task_failure(
                "swallowtail.local_task.reap_reservation_required",
                "Local task was not started under a reap reservation",
            )
        })?;
        let worker = self.worker.take().ok_or_else(|| {
            task_failure(
                "swallowtail.local_task.already_relinquished",
                "Local task ownership was already transferred",
            )
        })?;
        if let Err((error, worker, reap_permit)) =
            reap_permit.accept(worker, Arc::clone(&self.reaped))
        {
            self.worker = Some(worker);
            self.reap_permit = Some(reap_permit);
            return Err(error);
        }
        Ok(())
    }
}

impl Drop for LocalJoinedTask {
    fn drop(&mut self) {
        if let Some(worker) = self.worker.take() {
            // Join-on-drop deliberately preserves ordinary task ownership.
            let _ = reap_worker(Some(worker), &self.reaped);
        }
        drop(self.reap_permit.take());
    }
}

struct NotifyOnDrop(Arc<JoinSignal>);

impl Drop for NotifyOnDrop {
    fn drop(&mut self) {
        self.0.notify();
    }
}

fn join_worker(worker: Option<JoinHandle<()>>) -> Result<(), RuntimeFailure> {
    let worker = worker.ok_or_else(|| {
        task_failure(
            "swallowtail.local_task.already_joined",
            "Local task was already joined",
        )
    })?;
    worker.join().map_err(|_| {
        task_failure(
            "swallowtail.local_task.panicked",
            "Local task failed while executing",
        )
    })
}

pub(super) fn reap_worker(
    worker: Option<JoinHandle<()>>,
    reaped: &AtomicBool,
) -> Result<(), RuntimeFailure> {
    let outcome = join_worker(worker);
    reaped.store(true, Ordering::Release);
    outcome
}

pub(super) fn task_failure(code: &'static str, message: &'static str) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, message))
}

#[derive(Default)]
struct JoinSignal {
    finished: AtomicBool,
    wakers: Mutex<Vec<Waker>>,
}

impl JoinSignal {
    fn notify(&self) {
        self.finished.store(true, Ordering::Release);
        let wakers = self
            .wakers
            .lock()
            .expect("local task waker lock poisoned")
            .drain(..)
            .collect::<Vec<_>>();
        for waker in wakers {
            waker.wake();
        }
    }

    fn register(&self, waker: &Waker) {
        if self.finished.load(Ordering::Acquire) {
            waker.wake_by_ref();
            return;
        }
        let mut registered = self.wakers.lock().expect("local task waker lock poisoned");
        if !registered.iter().any(|current| current.will_wake(waker)) {
            registered.push(waker.clone());
        }
        if self.finished.load(Ordering::Acquire) {
            let wakers = registered.drain(..).collect::<Vec<_>>();
            drop(registered);
            for waker in wakers {
                waker.wake();
            }
        }
    }
}
