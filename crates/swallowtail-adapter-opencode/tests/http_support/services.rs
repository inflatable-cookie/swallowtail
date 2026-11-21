#[derive(Clone)]
pub struct ThreadServices {
    origin: Instant,
    refuse_spawns: Arc<AtomicBool>,
    completed_blocking_jobs: Arc<AtomicUsize>,
}

impl ThreadServices {
    pub fn new() -> Self {
        Self {
            origin: Instant::now(),
            refuse_spawns: Arc::new(AtomicBool::new(false)),
            completed_blocking_jobs: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Makes every later scoped-task spawn fail, as a host that refuses new tasks would.
    pub fn refuse_spawns(&self) {
        self.refuse_spawns.store(true, Ordering::SeqCst);
    }

    /// Counts blocking jobs that have run to completion, so a test can tell
    /// whether a worker was joined rather than detached.
    pub fn completed_blocking_jobs(&self) -> usize {
        self.completed_blocking_jobs.load(Ordering::SeqCst)
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
        if self.refuse_spawns.load(Ordering::SeqCst) {
            return Err(RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                "fixture.spawn_refused",
                "Fixture refused the task spawn",
            )));
        }
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
        let completed = Arc::clone(&self.completed_blocking_jobs);
        thread::spawn(move || {
            let result = job();
            completed.fetch_add(1, Ordering::SeqCst);
            let _ = sender.send(result);
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

