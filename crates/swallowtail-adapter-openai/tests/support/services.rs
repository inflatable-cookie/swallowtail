use futures_channel::oneshot;
use futures_executor::block_on;
use std::thread::{self, JoinHandle};
use std::time::Instant;
use swallowtail_runtime::{
    BlockingJob, BlockingWorkService, BoxFuture, Deadline, DeadlineObservation, JoinedTask,
    MonotonicInstant, RuntimeFailure, ScopeId, ScopedTaskService, TimeService,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TimeMode {
    #[default]
    Pending,
    Delayed,
}

#[derive(Clone)]
pub struct ThreadServices {
    origin: Instant,
    time_mode: TimeMode,
}

impl ThreadServices {
    pub fn new(time_mode: TimeMode) -> Self {
        Self {
            origin: Instant::now(),
            time_mode,
        }
    }
}

struct ThreadTask(JoinHandle<()>);

impl JoinedTask for ThreadTask {
    fn join(self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        Box::pin(async move {
            self.0.join().map_err(|_| {
                RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                    "fixture.task_panicked",
                    "Fixture task panicked",
                ))
            })
        })
    }
}

impl ScopedTaskService for ThreadServices {
    fn spawn(
        &self,
        _scope: ScopeId,
        task: BoxFuture<'static, ()>,
    ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
        Ok(Box::new(ThreadTask(thread::spawn(move || block_on(task)))))
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
        match self.time_mode {
            TimeMode::Pending => Box::pin(std::future::pending()),
            TimeMode::Delayed => {
                let (sender, receiver) = oneshot::channel();
                thread::spawn(move || {
                    thread::sleep(std::time::Duration::from_millis(20));
                    let _ = sender.send(DeadlineObservation::new(deadline, deadline.instant()));
                });
                Box::pin(async move { receiver.await.expect("deadline sender remains available") })
            }
        }
    }
}
