struct SessionCancellation {
    connection: Arc<AcpConnection>,
    requested: AtomicBool,
}

impl SessionCancellation {
    fn new(connection: Arc<AcpConnection>) -> Self {
        Self {
            connection,
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
        Box::pin(async move {
            if already {
                Ok(CancellationAcknowledgement::AlreadyRequested)
            } else {
                self.connection.cancel_session().await?;
                Ok(CancellationAcknowledgement::Requested)
            }
        })
    }
}

struct TurnCancellation {
    connection: Arc<AcpConnection>,
    session_id: String,
    turn: Arc<ActiveTurn>,
    requested: AtomicBool,
}

impl CancellationControl for TurnCancellation {
    fn scope(&self) -> CancellationScope {
        CancellationScope::ActiveTurn
    }

    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        let already = self.requested.swap(true, Ordering::SeqCst);
        Box::pin(async move {
            if already {
                return Ok(CancellationAcknowledgement::AlreadyRequested);
            }
            self.turn.mark_cancelled();
            self.connection
                .notify("session/cancel", json!({"sessionId": self.session_id}))
                .await?;
            Ok(CancellationAcknowledgement::Requested)
        })
    }
}
