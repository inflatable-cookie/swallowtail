struct TrackingCredential {
    host: LocalProcessHost,
    releases: Arc<AtomicUsize>,
}

impl CredentialService for TrackingCredential {
    fn acquire(&self, scope: ScopeId, reference: CredentialRef, audience: EndpointAudience) -> BoxFuture<'static, Result<CredentialLease, RuntimeFailure>> {
        self.host.acquire(scope, reference, audience)
    }

    fn release(&self, lease: CredentialLease) -> BoxFuture<'static, CleanupOutcome> {
        let future = self.host.release(lease);
        let releases = Arc::clone(&self.releases);
        Box::pin(async move {
            let result = future.await;
            releases.fetch_add(1, Ordering::SeqCst);
            result
        })
    }
}

#[derive(Clone)]
struct ThreadServices { origin: Instant }

impl ThreadServices {
    fn new() -> Self { Self { origin: Instant::now() } }
}

struct ThreadTask(JoinHandle<()>);

impl JoinedTask for ThreadTask {
    fn join(self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        Box::pin(async move { self.0.join().map_err(|_| failure("fixture.task_panicked", "Fixture task panicked")) })
    }
}

impl ScopedTaskService for ThreadServices {
    fn spawn(&self, _scope: ScopeId, task: BoxFuture<'static, ()>) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
        Ok(Box::new(ThreadTask(thread::spawn(move || block_on(task)))))
    }
}

impl BlockingWorkService for ThreadServices {
    fn run(&self, _scope: ScopeId, job: BlockingJob) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        let (sender, receiver) = oneshot::channel();
        thread::spawn(move || { let _ = sender.send(job()); });
        Box::pin(async move { receiver.await.map_err(|_| failure("fixture.blocking_panicked", "Fixture blocking work panicked"))? })
    }
}

impl TimeService for ThreadServices {
    fn now(&self) -> swallowtail_runtime::MonotonicInstant {
        swallowtail_runtime::MonotonicInstant::from_ticks(self.origin.elapsed().as_millis() as u64)
    }

    fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        Box::pin(async move { DeadlineObservation::new(deadline, deadline.instant()) })
    }
}
