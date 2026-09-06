#[derive(Clone)]
pub struct ThreadServices {
    origin: Instant,
    completed: Arc<AtomicUsize>,
    manual_deadline: Arc<Mutex<Option<Arc<ManualDeadlineState>>>>,
}
impl ThreadServices {
    pub fn new() -> Self {
        Self {
            origin: Instant::now(),
            completed: Arc::new(AtomicUsize::new(0)),
            manual_deadline: Arc::new(Mutex::new(None)),
        }
    }

    pub fn completed(&self) -> usize {
        self.completed.load(Ordering::SeqCst)
    }

    pub fn manual_deadline(&self) -> (Deadline, ManualDeadlineTrigger) {
        let state = Arc::new(ManualDeadlineState::default());
        *self
            .manual_deadline
            .lock()
            .expect("manual deadline lock") = Some(Arc::clone(&state));
        let deadline = Deadline::at(MonotonicInstant::from_ticks(
            self.now().ticks().saturating_add(3_600_000),
        ));
        (deadline, ManualDeadlineTrigger { state })
    }
}

#[derive(Clone)]
pub struct ManualDeadlineTrigger {
    state: Arc<ManualDeadlineState>,
}

impl ManualDeadlineTrigger {
    pub fn fire(&self) {
        if self.state.fired.swap(true, Ordering::Release) {
            return;
        }
        let wakers = self
            .state
            .wakers
            .lock()
            .expect("manual deadline waker lock")
            .drain(..)
            .collect::<Vec<_>>();
        for waker in wakers {
            waker.wake();
        }
    }
}

#[derive(Default)]
struct ManualDeadlineState {
    fired: std::sync::atomic::AtomicBool,
    wakers: Mutex<Vec<Waker>>,
}

struct ManualDeadlineWait {
    deadline: Deadline,
    state: Arc<ManualDeadlineState>,
}

impl Future for ManualDeadlineWait {
    type Output = DeadlineObservation;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        if self.state.fired.load(Ordering::Acquire) {
            return Poll::Ready(DeadlineObservation::new(
                self.deadline,
                self.deadline.instant(),
            ));
        }
        let mut wakers = self.state.wakers.lock().expect("manual deadline waker lock");
        if self.state.fired.load(Ordering::Acquire) {
            let pending = wakers.drain(..).collect::<Vec<_>>();
            drop(wakers);
            for waker in pending {
                waker.wake();
            }
        } else if !wakers
            .iter()
            .any(|waker| waker.will_wake(context.waker()))
        {
            wakers.push(context.waker().clone());
        }
        Poll::Pending
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
        if let Some(state) = self
            .manual_deadline
            .lock()
            .expect("manual deadline lock")
            .clone()
        {
            return Box::pin(ManualDeadlineWait { deadline, state });
        }
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
