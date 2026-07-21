impl AcpConnection {
    pub(crate) async fn pump(self: Arc<Self>) {
        let mut decoder = NdjsonDecoder::default();
        let mut transport_failure = None;
        loop {
            match self.process.read_output().await {
                Ok(Some(chunk)) if chunk.stream() == ProcessOutputStream::Stdout => {
                    match decoder.push(chunk.bytes()) {
                        Ok(messages) => {
                            for message in messages {
                                if let Err(error) = self.dispatch(message).await {
                                    transport_failure = Some(error);
                                    break;
                                }
                            }
                        }
                        Err(_) => transport_failure = Some(protocol_failure()),
                    }
                    if transport_failure.is_some() {
                        break;
                    }
                }
                Ok(Some(_)) => {}
                Ok(None) => break,
                Err(error) => {
                    transport_failure = Some(error);
                    break;
                }
            }
        }
        if transport_failure.is_none() && decoder.finish().is_err() {
            transport_failure = Some(protocol_failure());
        }
        if transport_failure.is_some() {
            let _ = self.process.force_stop().await;
        }
        let waited = self.process.wait().await;
        self.closed.store(true, Ordering::SeqCst);
        let cleanup = if waited.is_ok() {
            CleanupOutcome::Clean
        } else {
            CleanupOutcome::Failed(SafeDiagnostic::new(
                "swallowtail.gemini.acp.process_cleanup_failed",
                "Gemini CLI process cleanup failed",
            ))
        };
        *self.cleanup.lock().expect("ACP cleanup lock poisoned") = Some(cleanup.clone());
        let error = transport_failure.unwrap_or_else(|| {
            failure(
                "swallowtail.gemini.acp.connection_ended",
                "Gemini CLI ACP connection ended",
            )
        });
        if let Some(turn) = self
            .active_turn
            .lock()
            .expect("ACP active lock poisoned")
            .take()
            && !turn.is_finished()
        {
            turn.fail(&error);
        }
        self.fail_pending(error);
    }
}
