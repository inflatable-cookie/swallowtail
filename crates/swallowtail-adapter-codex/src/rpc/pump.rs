impl RpcConnection {
    pub(crate) async fn pump(self: Arc<Self>) {
        let mut pending_bytes = Vec::new();
        let mut protocol_failure = None;
        loop {
            match self.process.read_output().await {
                Ok(Some(chunk)) if chunk.stream() == ProcessOutputStream::Stdout => {
                    pending_bytes.extend_from_slice(chunk.bytes());
                    while let Some(newline) = pending_bytes.iter().position(|byte| *byte == b'\n') {
                        let line: Vec<_> = pending_bytes.drain(..=newline).collect();
                        let line = trim_newline(&line);
                        if let Err(error) = self.dispatch(line).await {
                            let error = with_inbound_context(error, line);
                            self.emit_malformed_inbound_debug(&error, line);
                            protocol_failure = Some(error);
                            break;
                        }
                    }
                    if protocol_failure.is_some() {
                        break;
                    }
                }
                Ok(Some(chunk)) => self.record_stderr(chunk.bytes()),
                Ok(None) => break,
                Err(error) => {
                    protocol_failure = Some(error);
                    break;
                }
            }
        }
        if protocol_failure.is_none()
            && !pending_bytes.is_empty()
            && let Err(error) = self.dispatch(&pending_bytes).await
        {
            let error = with_inbound_context(error, &pending_bytes);
            self.emit_malformed_inbound_debug(&error, &pending_bytes);
            protocol_failure = Some(error);
        }
        if protocol_failure.is_some() {
            let _ = self.process.force_stop().await;
        }
        let exit = self.process.wait().await;
        self.closed.store(true, Ordering::SeqCst);
        let cleanup = if exit.is_ok() {
            CleanupOutcome::Clean
        } else {
            CleanupOutcome::Failed(SafeDiagnostic::new(
                "swallowtail.codex.app_server.process_cleanup_failed",
                "Codex app-server process cleanup failed",
            ))
        };
        *self.cleanup.lock().expect("RPC cleanup lock poisoned") = Some(cleanup.clone());

        let terminal = if self.session_cancelled.load(Ordering::SeqCst) {
            TerminalStatus::Cancelled
        } else if let Some(error) = protocol_failure {
            TerminalStatus::RuntimeFailed(self.protocol_terminal(&error))
        } else if !self.closing.load(Ordering::SeqCst) {
            TerminalStatus::HostFailed(SafeDiagnostic::new(
                "swallowtail.codex.app_server.connection_ended",
                "Codex app-server connection ended unexpectedly",
            ))
        } else {
            TerminalStatus::Cancelled
        };
        if let Some(turn) = self
            .active_turn
            .lock()
            .expect("active-turn lock poisoned")
            .take()
        {
            turn.finish(terminal, cleanup);
        }
        self.fail_pending(failure(
            "swallowtail.codex.app_server.connection_ended",
            "Codex app-server connection ended",
        ));
    }

    async fn dispatch(&self, line: &[u8]) -> Result<(), RuntimeFailure> {
        if line.iter().all(u8::is_ascii_whitespace) {
            return Ok(());
        }
        let message: Value = serde_json::from_slice(line).map_err(|_| malformed_inbound())?;
        if let Some(id_value) = message.get("id") {
            if message.get("method").is_some() {
                return self.dispatch_server_request(&message).await;
            }
            let id = id_value.as_u64().ok_or_else(malformed_inbound)?;
            if self
                .ignored_responses
                .lock()
                .expect("ignored-response lock poisoned")
                .remove(&id)
            {
                return Ok(());
            }
            let sender = self
                .pending
                .lock()
                .expect("RPC pending-response lock poisoned")
                .remove(&id)
                .ok_or_else(|| {
                    failure(
                        "swallowtail.codex.app_server.unknown_response",
                        "Codex app-server returned an unknown response id",
                    )
                })?;
            let response = if message.get("error").is_some() {
                Err(failure(
                    "swallowtail.codex.app_server.request_failed",
                    "Codex app-server rejected a request",
                ))
            } else {
                message.get("result").cloned().ok_or_else(malformed_inbound)
            };
            sender.complete(response);
            return Ok(());
        }
        let method = message
            .get("method")
            .and_then(Value::as_str)
            .ok_or_else(malformed_inbound)?;
        let params = message.get("params").cloned().unwrap_or(Value::Null);
        let active_turn = {
            self.active_turn
                .lock()
                .expect("active-turn lock poisoned")
                .clone()
        };
        if let Some(turn) = active_turn {
            turn.handle_notification(method, &params)?;
            if turn.is_finished() {
                self.reject_abandoned_callbacks(turn.take_abandoned_provider_requests())
                    .await?;
                self.clear_active_turn(&turn);
            }
        } else if let Some(notification) = LifecycleNotification::from_message(method, &params) {
            self.lifecycle_notifications
                .lock()
                .expect("lifecycle-notification lock poisoned")
                .push(notification);
        }
        Ok(())
    }

    async fn dispatch_server_request(&self, message: &Value) -> Result<(), RuntimeFailure> {
        let id = message.get("id").ok_or_else(malformed_inbound)?;
        let method = message
            .get("method")
            .and_then(Value::as_str)
            .ok_or_else(malformed_inbound)?;
        let turn = self
            .active_turn
            .lock()
            .expect("active-turn lock poisoned")
            .clone()
            .ok_or_else(|| {
                failure(
                    "swallowtail.codex.app_server.callback_without_turn",
                    "Codex app-server requested a callback without an active turn",
                )
            });
        let turn = match turn {
            Ok(turn) => turn,
            Err(error) => {
                self.reject_server_request(id, -32602, "Dynamic tool callback rejected")
                    .await?;
                return Err(error);
            }
        };
        let params = match message.get("params") {
            Some(params) => params,
            None => {
                self.reject_server_request(id, -32602, "Dynamic tool callback rejected")
                    .await?;
                return Err(malformed_inbound());
            }
        };
        let callback_id = self.allocate_callback_id(turn.runtime_id());
        if method == "item/tool/call" {
            if let Err(error) = turn.handle_tool_call(id.clone(), params, callback_id) {
                self.reject_server_request(id, -32602, "Dynamic tool callback rejected")
                    .await?;
                if turn.is_stopping() {
                    return Ok(());
                }
                return Err(error);
            }
            return Ok(());
        }

        let disposition = match turn.handle_provider_request(id, method, params, callback_id) {
            Ok(disposition) => disposition,
            Err(error) => {
                self.reject_server_request(id, -32601, "Client callback unsupported")
                    .await?;
                return Err(error);
            }
        };
        let crate::turn_state::ProviderRequestDisposition::Observed(observation) = disposition
        else {
            return Ok(());
        };
        self.reject_server_request(id, -32001, "Provider request observed; turn stopped")
            .await?;
        self.request_without_waiting(
            "turn/interrupt",
            serde_json::json!({
                "threadId": params["threadId"].clone(),
                "turnId": params["turnId"].clone()
            }),
        )
        .await?;
        turn.finish(
            TerminalStatus::ProviderRequestObserved(observation),
            CleanupOutcome::NotApplicable,
        );
        self.clear_active_turn(&turn);
        Ok(())
    }

    async fn reject_server_request(
        &self,
        id: &Value,
        code: i64,
        message: &str,
    ) -> Result<(), RuntimeFailure> {
        self.write_message(&serde_json::json!({
            "id": id,
            "error": {
                "code": code,
                "message": message
            }
        }))
        .await
    }

    fn fail_pending(&self, error: RuntimeFailure) {
        let pending = std::mem::take(
            &mut *self
                .pending
                .lock()
                .expect("RPC pending-response lock poisoned"),
        );
        for (_, sender) in pending {
            sender.complete(Err(error.clone()));
        }
    }

    fn record_stderr(&self, bytes: &[u8]) {
        let mut tail = self
            .stderr_tail
            .lock()
            .expect("RPC stderr-tail lock poisoned");
        tail.extend_from_slice(bytes);
        if tail.len() > MAX_STDERR_TAIL_BYTES {
            let overflow = tail.len() - MAX_STDERR_TAIL_BYTES;
            tail.drain(..overflow);
        }
    }

    fn protocol_terminal(&self, error: &RuntimeFailure) -> SafeDiagnostic {
        let diagnostic = error.diagnostic();
        let tail = std::mem::take(
            &mut *self
                .stderr_tail
                .lock()
                .expect("RPC stderr-tail lock poisoned"),
        );
        let Some(excerpt) = sanitize_stderr(&tail, false) else {
            return diagnostic.clone();
        };
        self.services.emit_debug_observation(
            &DebugObservation::new(
                DebugObservationKind::StderrRing,
                format!("stderr={excerpt}"),
            )
            .with_route("codex.app_server")
            .with_stage("rpc.pump.protocol_terminal")
            .with_correlated_code(diagnostic.code()),
        );
        SafeDiagnostic::new(
            diagnostic.code(),
            format!("{}; stderr: {excerpt}", diagnostic.message()),
        )
        .with_failure_classification(diagnostic.failure_classification())
    }

    fn emit_malformed_inbound_debug(&self, error: &RuntimeFailure, line: &[u8]) {
        let diagnostic = error.diagnostic();
        if !matches!(
            diagnostic.code(),
            "swallowtail.codex.app_server.malformed_notification"
                | "swallowtail.codex.app_server.malformed_message"
        ) {
            return;
        }
        let method = match serde_json::from_slice::<Value>(line) {
            Ok(message) => message
                .get("method")
                .and_then(Value::as_str)
                .map_or_else(|| "<absent>".to_owned(), bounded_method),
            Err(_) => "<unparseable>".to_owned(),
        };
        let excerpt = sanitize_stderr(line, false).unwrap_or_else(|| "<empty>".to_owned());
        self.services.emit_debug_observation(
            &DebugObservation::new(
                DebugObservationKind::WireInbound,
                format!("method={method}"),
            )
            .with_route("codex.app_server")
            .with_stage("rpc.pump.inbound")
            .with_correlated_code(diagnostic.code()),
        );
        self.services.emit_debug_observation(
            &DebugObservation::new(
                DebugObservationKind::ProtocolParse,
                format!("method={method}; excerpt={excerpt}"),
            )
            .with_route("codex.app_server")
            .with_stage("rpc.pump.inbound")
            .with_correlated_code(diagnostic.code()),
        );
    }
}

const MAX_STDERR_TAIL_BYTES: usize = 2048;
const MAX_METHOD_CHARS: usize = 64;

fn with_inbound_context(error: RuntimeFailure, line: &[u8]) -> RuntimeFailure {
    let diagnostic = error.diagnostic();
    if !matches!(
        diagnostic.code(),
        "swallowtail.codex.app_server.malformed_notification"
            | "swallowtail.codex.app_server.malformed_message"
    ) {
        return error;
    }
    let method = match serde_json::from_slice::<Value>(line) {
        Ok(message) => message
            .get("method")
            .and_then(Value::as_str)
            .map_or_else(|| "<absent>".to_owned(), bounded_method),
        Err(_) => "<unparseable>".to_owned(),
    };
    let excerpt = sanitize_stderr(line, false).unwrap_or_else(|| "<empty>".to_owned());
    RuntimeFailure::new(
        SafeDiagnostic::new(
            diagnostic.code(),
            format!(
                "{} (method `{method}`, excerpt `{excerpt}`)",
                diagnostic.message()
            ),
        )
        .with_failure_classification(diagnostic.failure_classification()),
    )
}

fn bounded_method(method: &str) -> String {
    let normalized = normalized_ascii(method.as_bytes());
    let mut bounded: String = normalized.chars().take(MAX_METHOD_CHARS).collect();
    if normalized.chars().count() > MAX_METHOD_CHARS {
        bounded.push_str("...");
    }
    bounded
}
