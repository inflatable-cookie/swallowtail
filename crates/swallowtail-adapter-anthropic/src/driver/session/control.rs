enum SubmitState {
    AwaitingCall,
    Waiting(BTreeSet<DirectToolCallId>),
    Submitted,
    Abandoned,
}

struct ResultSubmitter {
    state: Mutex<SubmitState>,
    sender: Mutex<Option<oneshot::Sender<Vec<DirectToolResult>>>>,
}

impl ResultSubmitter {
    fn new() -> (Self, oneshot::Receiver<Vec<DirectToolResult>>) {
        let (sender, receiver) = oneshot::channel();
        (
            Self {
                state: Mutex::new(SubmitState::AwaitingCall),
                sender: Mutex::new(Some(sender)),
            },
            receiver,
        )
    }

    fn open(&self, call_id: DirectToolCallId) -> Result<(), RuntimeFailure> {
        let mut state = self.state.lock().expect("tool result state lock poisoned");
        if !matches!(*state, SubmitState::AwaitingCall) {
            return Err(exchange_failure());
        }
        *state = SubmitState::Waiting(BTreeSet::from([call_id]));
        Ok(())
    }

    fn abandon(&self) {
        *self.state.lock().expect("tool result state lock poisoned") = SubmitState::Abandoned;
        self.sender
            .lock()
            .expect("tool result sender lock poisoned")
            .take();
    }
}

impl DirectToolResultSubmitter for ResultSubmitter {
    fn submit(&self, results: Vec<DirectToolResult>) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        let result = {
            let mut state = self.state.lock().expect("tool result state lock poisoned");
            let SubmitState::Waiting(expected) = &*state else {
                return Box::pin(async { Err(exchange_failure()) });
            };
            let supplied: BTreeSet<_> = results
                .iter()
                .map(|result| result.call_id().clone())
                .collect();
            if supplied != *expected || supplied.len() != results.len() {
                return Box::pin(async { Err(exchange_failure()) });
            }
            let sender = self
                .sender
                .lock()
                .expect("tool result sender lock poisoned")
                .take();
            *state = SubmitState::Submitted;
            sender
        };
        Box::pin(async move {
            result
                .ok_or_else(exchange_failure)?
                .send(results)
                .map_err(|_| exchange_failure())
        })
    }
}

struct TurnCancellation {
    cancelled: Arc<AtomicBool>,
    session_usable: Arc<AtomicBool>,
    reason: std::sync::atomic::AtomicU8,
    signal: Mutex<Option<oneshot::Sender<()>>>,
}

impl TurnCancellation {
    fn new(
        cancelled: Arc<AtomicBool>,
        session_usable: Arc<AtomicBool>,
    ) -> (Self, oneshot::Receiver<()>) {
        let (sender, receiver) = oneshot::channel();
        (
            Self {
                cancelled,
                session_usable,
                reason: std::sync::atomic::AtomicU8::new(0),
                signal: Mutex::new(Some(sender)),
            },
            receiver,
        )
    }

    fn timeout(&self) {
        if self
            .reason
            .compare_exchange(0, 2, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
        {
            self.stop();
        }
    }

    fn stop(&self) {
        self.session_usable.store(false, Ordering::SeqCst);
        self.cancelled.store(true, Ordering::SeqCst);
        if let Some(signal) = self.signal.lock().expect("cancel lock poisoned").take() {
            let _ = signal.send(());
        }
    }

    fn is_requested(&self) -> bool {
        self.reason.load(Ordering::SeqCst) != 0
    }

    fn stop_reason(&self) -> StopSignal {
        if self.reason.load(Ordering::SeqCst) == 2 {
            StopSignal::TimedOut
        } else {
            StopSignal::Cancelled
        }
    }
}

impl CancellationControl for TurnCancellation {
    fn scope(&self) -> CancellationScope {
        CancellationScope::ActiveTurn
    }

    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        let requested = self
            .reason
            .compare_exchange(0, 1, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok();
        if requested {
            self.stop();
        }
        Box::pin(async move {
            Ok(if requested {
                CancellationAcknowledgement::Requested
            } else {
                CancellationAcknowledgement::AlreadyRequested
            })
        })
    }
}

struct SessionCancellation {
    active: ActiveSlot,
    usable: Arc<AtomicBool>,
    requested: AtomicBool,
}

impl CancellationControl for SessionCancellation {
    fn scope(&self) -> CancellationScope {
        CancellationScope::InteractiveSession
    }

    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        let requested = !self.requested.swap(true, Ordering::SeqCst);
        self.usable.store(false, Ordering::SeqCst);
        let active = self
            .active
            .lock()
            .expect("active turn lock poisoned")
            .as_ref()
            .map(|turn| Arc::clone(&turn.cancellation));
        Box::pin(async move {
            if let Some(active) = active {
                let _ = active.request().await?;
            }
            Ok(if requested {
                CancellationAcknowledgement::Requested
            } else {
                CancellationAcknowledgement::AlreadyRequested
            })
        })
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum StopSignal {
    Cancelled,
    TimedOut,
}

enum StreamSignal {
    Item(Result<StreamItem, RuntimeFailure>),
    Closed,
    Stopped(StopSignal),
}

async fn next_signal(
    subscription: &mut Subscription,
    cancel: &mut oneshot::Receiver<()>,
    deadline: &mut BoxFuture<'static, swallowtail_runtime::DeadlineObservation>,
    cancellation: &TurnCancellation,
) -> StreamSignal {
    poll_fn(|context| {
        if let Poll::Ready(item) = subscription.poll_next(context) {
            return Poll::Ready(item.map_or(StreamSignal::Closed, StreamSignal::Item));
        }
        if Pin::new(&mut *cancel).poll(context).is_ready() {
            return Poll::Ready(StreamSignal::Stopped(cancellation.stop_reason()));
        }
        if deadline.as_mut().poll(context).is_ready() {
            cancellation.timeout();
            return Poll::Ready(StreamSignal::Stopped(StopSignal::TimedOut));
        }
        Poll::Pending
    })
    .await
}

async fn wait_results(
    receiver: &mut oneshot::Receiver<Vec<DirectToolResult>>,
    cancel: &mut oneshot::Receiver<()>,
    deadline: &mut BoxFuture<'static, swallowtail_runtime::DeadlineObservation>,
    cancellation: &TurnCancellation,
) -> Result<Vec<DirectToolResult>, StopSignal> {
    poll_fn(|context| {
        if let Poll::Ready(result) = Pin::new(&mut *receiver).poll(context) {
            return Poll::Ready(result.map_err(|_| cancellation.stop_reason()));
        }
        if Pin::new(&mut *cancel).poll(context).is_ready() {
            return Poll::Ready(Err(cancellation.stop_reason()));
        }
        if deadline.as_mut().poll(context).is_ready() {
            cancellation.timeout();
            return Poll::Ready(Err(StopSignal::TimedOut));
        }
        Poll::Pending
    })
    .await
}

