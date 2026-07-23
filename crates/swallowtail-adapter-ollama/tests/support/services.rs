use futures_channel::oneshot;
use futures_executor::block_on;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use swallowtail_core::CatalogTimestamp;
use swallowtail_runtime::{
    BlockingJob, BlockingWorkService, BoxFuture, Deadline, DeadlineObservation, JoinedTask,
    MonotonicInstant, RuntimeFailure, ScopeId, ScopedTaskService, TimeService,
};

#[derive(Clone)]
pub struct ThreadServices {
    origin: Instant,
    fail_join: bool,
}

impl ThreadServices {
    pub fn new() -> Self {
        Self {
            origin: Instant::now(),
            fail_join: false,
        }
    }

    pub fn with_join_failure(mut self) -> Self {
        self.fail_join = true;
        self
    }

    pub fn deadline_after(&self, duration: Duration) -> Deadline {
        Deadline::at(MonotonicInstant::from_ticks(
            self.now().ticks() + duration.as_millis() as u64,
        ))
    }
}

struct ThreadTask {
    handle: JoinHandle<()>,
    fail_join: bool,
}

impl JoinedTask for ThreadTask {
    fn join(self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        Box::pin(async move {
            self.handle.join().map_err(|_| {
                RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                    "fixture.task_panicked",
                    "Fixture task panicked",
                ))
            })?;
            if self.fail_join {
                Err(RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                    "fixture.task_join_failed",
                    "Fixture task join failed",
                )))
            } else {
                Ok(())
            }
        })
    }
}

impl ScopedTaskService for ThreadServices {
    fn spawn(
        &self,
        _scope: ScopeId,
        task: BoxFuture<'static, ()>,
    ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
        Ok(Box::new(ThreadTask {
            handle: thread::spawn(move || block_on(task)),
            fail_join: self.fail_join,
        }))
    }
}

impl BlockingWorkService for ThreadServices {
    fn run(
        &self,
        _scope: ScopeId,
        job: BlockingJob,
    ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        let (sender, receiver) = oneshot::channel();
        thread::spawn(move || {
            let _ = sender.send(job());
        });
        Box::pin(async move {
            receiver.await.map_err(|_| {
                RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                    "fixture.blocking_panicked",
                    "Fixture blocking work panicked",
                ))
            })?
        })
    }
}

impl TimeService for ThreadServices {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(self.origin.elapsed().as_millis() as u64)
    }

    fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        let wait = deadline
            .instant()
            .ticks()
            .saturating_sub(self.now().ticks());
        let (sender, receiver) = oneshot::channel();
        thread::spawn(move || {
            if wait != 0 {
                thread::sleep(Duration::from_millis(wait));
            }
            let _ = sender.send(DeadlineObservation::new(deadline, deadline.instant()));
        });
        Box::pin(async move {
            receiver
                .await
                .expect("fixture deadline thread returns an observation")
        })
    }

    fn catalog_now(&self) -> Result<CatalogTimestamp, RuntimeFailure> {
        Ok(CatalogTimestamp::new(1_700_000_000, 42).expect("fixture timestamp is valid"))
    }
}
