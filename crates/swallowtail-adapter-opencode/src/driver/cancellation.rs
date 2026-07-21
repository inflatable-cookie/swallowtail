struct TurnCancellation {
    scope: ScopeId,
    session_id: String,
    directory: String,
    endpoint: String,
    services: HostServices,
    transport: CurlTransport,
    stream_cancelled: Arc<AtomicBool>,
    requested: AtomicBool,
}

impl CancellationControl for TurnCancellation {
    fn scope(&self) -> CancellationScope {
        CancellationScope::ActiveTurn
    }

    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        let already = self.requested.swap(true, Ordering::SeqCst);
        self.stream_cancelled.store(true, Ordering::SeqCst);
        Box::pin(async move {
            if already {
                return Ok(CancellationAcknowledgement::AlreadyRequested);
            }
            let response = self
                .transport
                .request(
                    self.scope.clone(),
                    self.endpoint.clone(),
                    abort(&self.session_id, &self.directory),
                    &self.services,
                    Arc::new(AtomicBool::new(false)),
                )
                .await?;
            require_abort_success(&response)?;
            Ok(CancellationAcknowledgement::Requested)
        })
    }
}


