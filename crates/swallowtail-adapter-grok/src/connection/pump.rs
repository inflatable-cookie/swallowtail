impl AcpConnection {
    pub(crate) async fn pump(self: Arc<Self>) {
        let mut decoder = NdjsonDecoder::new(RECEIVE_FRAMING_LIMITS);
        let mut transport_failure = None;
        loop {
            match self.process.read_output().await {
                Ok(Some(chunk)) if chunk.stream() == ProcessOutputStream::Stdout => {
                    match decoder.push(chunk.bytes()) {
                        Ok(messages) => {
                            for message in messages {
                                if let Err(error) = self.dispatch(message).await {
                                    self.emit_protocol_debug(&error, "acp.pump.dispatch");
                                    transport_failure = Some(error);
                                    break;
                                }
                            }
                            if transport_failure.is_none() {
                                self.complete_attachment_recovery_batch();
                            }
                        }
                        Err(_) => {
                            let error = protocol_failure();
                            self.emit_protocol_debug(&error, "acp.pump.decode");
                            transport_failure = Some(error);
                        }
                    }
                    if transport_failure.is_some() {
                        break;
                    }
                }
                Ok(Some(_)) => {}
                Ok(None) => break,
                Err(error) => {
                    self.emit_protocol_debug(&error, "acp.pump.read");
                    transport_failure = Some(error);
                    break;
                }
            }
        }
        if transport_failure.is_none() && decoder.finish().is_err() {
            let error = protocol_failure();
            self.emit_protocol_debug(&error, "acp.pump.finish");
            transport_failure = Some(error);
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
                "swallowtail.grok.acp.process_cleanup_failed",
                "Grok Build process cleanup failed",
            ))
        };
        *self.cleanup.lock().expect("ACP cleanup lock poisoned") = Some(cleanup);
        let error = transport_failure.unwrap_or_else(|| {
            failure(
                "swallowtail.grok.acp.connection_ended",
                "Grok Build ACP connection ended",
            )
        });
        self.fail_attachment_recovery(error.clone());
        // The pump is a second terminal publisher: on EOF or protocol failure
        // it fails the active turn directly, before `fail_pending` wakes the
        // prompt task that would otherwise settle. Settle here first, so no
        // registered call can be admitted after the consumer sees terminal.
        if let Some(registered) = self.registered_session() {
            let _ = registered
                .settle(
                    &self.services,
                    swallowtail_runtime::RegisteredToolCleanupCause::TransportFailure,
                )
                .await;
        }
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
