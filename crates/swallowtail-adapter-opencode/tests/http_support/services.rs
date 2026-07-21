#[derive(Clone)]
pub struct ThreadServices {
    origin: Instant,
}

impl ThreadServices {
    pub fn new() -> Self {
        Self {
            origin: Instant::now(),
        }
    }

    pub fn deadline_after(&self, duration: Duration) -> Deadline {
        Deadline::at(MonotonicInstant::from_ticks(
            self.origin.elapsed().as_millis() as u64 + duration.as_millis() as u64,
        ))
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
        let now = self.now().ticks();
        let wait = deadline.instant().ticks().saturating_sub(now);
        Box::pin(async move {
            if wait != 0 {
                thread::sleep(Duration::from_millis(wait));
            }
            DeadlineObservation::new(deadline, deadline.instant())
        })
    }
}

