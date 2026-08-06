struct ManagedRunCancellation {
    requested: AtomicBool,
    active_connection: Mutex<Arc<AtomicBool>>,
    callbacks: ManagedCallbackHub,
    waiter: Mutex<Option<Waker>>,
}

impl ManagedRunCancellation {
    fn new(connection: Arc<AtomicBool>, callbacks: ManagedCallbackHub) -> Self {
        Self {
            requested: AtomicBool::new(false),
            active_connection: Mutex::new(connection),
            callbacks,
            waiter: Mutex::new(None),
        }
    }

    fn is_requested(&self) -> bool {
        self.requested.load(Ordering::SeqCst)
    }

    fn install(&self, connection: Arc<AtomicBool>) {
        if self.is_requested() {
            connection.store(true, Ordering::SeqCst);
        }
        *self
            .active_connection
            .lock()
            .expect("managed active connection lock poisoned") = connection;
    }

    fn stop_active(&self) {
        self.active_connection
            .lock()
            .expect("managed active connection lock poisoned")
            .store(true, Ordering::SeqCst);
    }

    fn poll_requested(&self, context: &mut Context<'_>) -> Poll<()> {
        if self.is_requested() {
            return Poll::Ready(());
        }
        let mut waiter = self
            .waiter
            .lock()
            .expect("managed cancellation waiter lock poisoned");
        if self.is_requested() {
            Poll::Ready(())
        } else {
            *waiter = Some(context.waker().clone());
            Poll::Pending
        }
    }

    fn wake_waiter(&self) {
        if let Some(waiter) = self
            .waiter
            .lock()
            .expect("managed cancellation waiter lock poisoned")
            .take()
        {
            waiter.wake();
        }
    }
}

impl CancellationControl for ManagedRunCancellation {
    fn scope(&self) -> CancellationScope {
        CancellationScope::StructuredRun
    }

    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        let prior = self.requested.swap(true, Ordering::SeqCst);
        self.stop_active();
        self.callbacks.abandon(CallbackAbandonment::TurnCancelled);
        self.wake_waiter();
        Box::pin(async move {
            Ok(if prior {
                CancellationAcknowledgement::AlreadyRequested
            } else {
                CancellationAcknowledgement::Requested
            })
        })
    }
}

struct ManagedRunHandle {
    request_id: RequestId,
    run_id: RuntimeRunId,
    events: Option<BoxEventStream>,
    callbacks: Option<CallbackExchange>,
    terminal: Option<BoxFuture<'static, TerminalOutcome>>,
    cancellation: Arc<ManagedRunCancellation>,
    task: Box<dyn JoinedTask>,
}

impl RunHandle for ManagedRunHandle {
    fn request_id(&self) -> &RequestId {
        &self.request_id
    }

    fn run_id(&self) -> &RuntimeRunId {
        &self.run_id
    }

    fn provider_run_ref(&self) -> Option<&RunRef> {
        None
    }

    fn take_events(&mut self) -> Option<BoxEventStream> {
        self.events.take()
    }

    fn take_callbacks(&mut self) -> Option<CallbackExchange> {
        self.callbacks.take()
    }

    fn cancellation(&self) -> &dyn CancellationControl {
        self.cancellation.as_ref()
    }

    fn take_terminal_outcome(&mut self) -> Option<BoxFuture<'static, TerminalOutcome>> {
        self.terminal.take()
    }

    fn close(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome> {
        Box::pin(async move {
            self.cancellation.requested.store(true, Ordering::SeqCst);
            self.cancellation.stop_active();
            self.cancellation.wake_waiter();
            self.cancellation
                .callbacks
                .abandon(CallbackAbandonment::Closed);
            match self.task.join().await {
                Ok(()) => CleanupOutcome::Clean,
                Err(_) => CleanupOutcome::Failed(SafeDiagnostic::new(
                    "swallowtail.anthropic.managed.task_join_failed",
                    "Anthropic Managed Agents operation task could not be joined",
                )),
            }
        })
    }
}

#[cfg(test)]
mod cancellation_tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;
    use std::task::Wake;

    #[derive(Default)]
    struct WakeCounter(AtomicUsize);

    impl Wake for WakeCounter {
        fn wake(self: Arc<Self>) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn accepted_cancellation_wakes_the_attachment_and_wins_a_ready_deadline() {
        let (callbacks, _exchange) = ManagedCallbackHub::new();
        let connection = Arc::new(AtomicBool::new(false));
        let cancellation = ManagedRunCancellation::new(Arc::clone(&connection), callbacks);
        let wakes = Arc::new(WakeCounter::default());
        let waker = Waker::from(Arc::clone(&wakes));
        let mut context = Context::from_waker(&waker);

        assert_eq!(cancellation.poll_requested(&mut context), Poll::Pending);
        assert_eq!(
            futures_executor::block_on(cancellation.request()).expect("cancellation accepted"),
            CancellationAcknowledgement::Requested
        );

        assert!(connection.load(Ordering::SeqCst));
        assert_eq!(wakes.0.load(Ordering::SeqCst), 1);
        assert_eq!(cancellation.poll_requested(&mut context), Poll::Ready(()));
        assert!(matches!(
            deadline_exit(&cancellation),
            AttachmentExit::Cancelled
        ));
    }
}
