#[derive(Clone)]
pub struct ThreadServices {
    origin: Instant,
    completed: Arc<AtomicUsize>,
}
impl ThreadServices {
    pub fn new() -> Self {
        Self {
            origin: Instant::now(),
            completed: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn completed(&self) -> usize {
        self.completed.load(Ordering::SeqCst)
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
        let completed = Arc::clone(&self.completed);
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
        let remaining = deadline
            .instant()
            .ticks()
            .saturating_sub(self.now().ticks());
        let (sender, receiver) = oneshot::channel();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(remaining));
            let _ = sender.send(DeadlineObservation::new(deadline, deadline.instant()));
        });
        Box::pin(async move { receiver.await.expect("deadline observation") })
    }
}
