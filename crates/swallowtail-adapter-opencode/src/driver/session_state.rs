struct ActiveTurn {
    task: Option<Box<dyn JoinedTask>>,
    cancellation: Arc<TurnCancellation>,
    terminal: Arc<AtomicBool>,
}

type ActiveSlot = Arc<Mutex<Option<ActiveTurn>>>;

struct SessionCancellation {
    active: ActiveSlot,
    requested: AtomicBool,
}

impl SessionCancellation {
    fn new(active: ActiveSlot) -> Self {
        Self {
            active,
            requested: AtomicBool::new(false),
        }
    }
}

impl CancellationControl for SessionCancellation {
    fn scope(&self) -> CancellationScope {
        CancellationScope::InteractiveSession
    }

    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        let already = self.requested.swap(true, Ordering::SeqCst);
        let active = self
            .active
            .lock()
            .expect("active turn lock poisoned")
            .as_ref()
            .map(|turn| Arc::clone(&turn.cancellation));
        Box::pin(async move {
            if already {
                return Ok(CancellationAcknowledgement::AlreadyRequested);
            }
            if let Some(active) = active {
                let _ = active.request().await?;
            }
            Ok(CancellationAcknowledgement::Requested)
        })
    }
}


