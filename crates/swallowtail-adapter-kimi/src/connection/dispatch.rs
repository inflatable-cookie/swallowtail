impl AcpConnection {
    async fn dispatch(&self, message: Message) -> Result<(), RuntimeFailure> {
        match message {
            Message::Response { id, result } => self.dispatch_response(id, result),
            Message::Notification { method, params } => {
                self.dispatch_notification(&method, &params)
            }
            Message::Request { id, method, params } => {
                self.dispatch_request(id, &method, &params).await
            }
        }
    }

    fn dispatch_response(
        &self,
        id: Value,
        result: Result<Value, swallowtail_protocol_acp::RpcError>,
    ) -> Result<(), RuntimeFailure> {
        let id = id.as_u64().ok_or_else(malformed)?;
        if let Some(phase) = self.phase.lock().expect("ACP phase lock poisoned").as_mut() {
            match phase {
                AttachPhase::Loading {
                    response_id,
                    response_seen,
                    ..
                }
                | AttachPhase::Resuming {
                    response_id,
                    response_seen,
                    ..
                } if *response_id == id => *response_seen = true,
                _ => {}
            }
        }
        let sender = self
            .pending
            .lock()
            .expect("ACP pending lock poisoned")
            .remove(&id)
            .ok_or_else(|| {
                failure(
                    "swallowtail.kimi.acp.unknown_response",
                    "Kimi Code returned an unknown ACP response",
                )
            })?;
        sender.complete(result.map_err(|_| {
            failure(
                "swallowtail.kimi.acp.request_rejected",
                "Kimi Code rejected an ACP request",
            )
        }));
        Ok(())
    }

    fn dispatch_notification(&self, method: &str, params: &Value) -> Result<(), RuntimeFailure> {
        match method {
            "session/update" => self.dispatch_session_update(params),
            method if method.starts_with('_') => Ok(()),
            _ => Err(failure(
                "swallowtail.kimi.acp.notification_unsupported",
                "Kimi Code sent an unsupported ACP notification",
            )),
        }
    }

    fn dispatch_session_update(&self, params: &Value) -> Result<(), RuntimeFailure> {
        let session_id = params
            .get("sessionId")
            .and_then(Value::as_str)
            .ok_or_else(malformed)?;
        let update = params.get("update").ok_or_else(malformed)?;
        let kind = update
            .get("sessionUpdate")
            .and_then(Value::as_str)
            .ok_or_else(malformed)?;
        let mut phase = self.phase.lock().expect("ACP phase lock poisoned");
        if let Some(phase) = phase.as_mut() {
            return match phase {
                AttachPhase::Loading {
                    session,
                    response_seen,
                    bytes,
                    replay,
                    ..
                } => {
                    if session.as_provider_value() != session_id {
                        return Err(session_mismatch());
                    }
                    if *response_seen {
                        return passive_update(kind);
                    }
                    let item = replay_item(session.clone(), replay.len() as u64, kind, update)?;
                    let item_bytes = item.content().map_or(0, OperationContent::byte_len);
                    if replay.len() >= crate::MAXIMUM_REPLAY_ITEMS
                        || bytes.saturating_add(item_bytes) > crate::MAXIMUM_REPLAY_BYTES
                    {
                        return Err(failure(
                            "swallowtail.kimi.acp.replay_limit_exceeded",
                            "Kimi Code session replay exceeded the adapter limit",
                        ));
                    }
                    *bytes += item_bytes;
                    replay.push(item);
                    Ok(())
                }
                AttachPhase::Resuming {
                    session,
                    response_seen,
                    ..
                } => {
                    if session.as_provider_value() != session_id {
                        return Err(session_mismatch());
                    }
                    if *response_seen {
                        passive_update(kind)
                    } else {
                        Err(failure(
                            "swallowtail.kimi.acp.resume_replay_rejected",
                            "Kimi Code emitted historical replay while resuming",
                        ))
                    }
                }
            };
        }
        drop(phase);
        if let Some(turn) = self
            .active_turn
            .lock()
            .expect("ACP active lock poisoned")
            .clone()
        {
            turn.handle_update(params)
        } else {
            passive_update(kind)
        }
    }

    async fn dispatch_request(
        &self,
        id: Value,
        method: &str,
        params: &Value,
    ) -> Result<(), RuntimeFailure> {
        match method {
            "fs/write_text_file" => self.write_text(id, params).await,
            method if method.starts_with('_') => {
                self.write(
                    encode_error(id, -32601, "Method not found").map_err(|_| protocol_failure())?,
                )
                .await
            }
            _ => {
                self.write(
                    encode_error(id, -32601, "Method not found").map_err(|_| protocol_failure())?,
                )
                .await?;
                Err(failure(
                    "swallowtail.kimi.acp.callback_unsupported",
                    "Kimi Code requested an unsupported ACP client callback",
                ))
            }
        }
    }

    async fn write_text(&self, id: Value, params: &Value) -> Result<(), RuntimeFailure> {
        if self.verify_session(params).is_err() {
            return self.callback_rejected(id).await;
        }
        let request: Result<WorkingResourceWriteRequest, RuntimeFailure> = (|| {
            let locator = params
                .get("path")
                .and_then(Value::as_str)
                .ok_or_else(malformed)
                .and_then(|value| WorkingResourceLocator::new(value).map_err(|_| malformed()))?;
            let content = params
                .get("content")
                .and_then(Value::as_str)
                .ok_or_else(malformed)?;
            let content = WorkingResourceText::new(
                content.to_owned(),
                NonZeroUsize::new(crate::MAXIMUM_WRITE_BYTES).expect("static limit is non-zero"),
            )
            .map_err(|_| malformed())?;
            Ok(WorkingResourceWriteRequest::new(locator, content))
        })();
        let Ok(request) = request else {
            return self.callback_rejected(id).await;
        };
        if self.resource_io.write_text(&self.resource, request).await.is_err() {
            return self.callback_rejected(id).await;
        }
        self.write(encode_result(id, Value::Null).map_err(|_| protocol_failure())?)
            .await
    }

    async fn callback_rejected(&self, id: Value) -> Result<(), RuntimeFailure> {
        self.write(encode_error(id, -32602, "Invalid params").map_err(|_| protocol_failure())?)
            .await
    }

    fn verify_session(&self, params: &Value) -> Result<(), RuntimeFailure> {
        let current = self.session_id.lock().expect("ACP session lock poisoned");
        if params.get("sessionId").and_then(Value::as_str) == current.as_deref() {
            Ok(())
        } else {
            Err(session_mismatch())
        }
    }
}

include!("dispatch/replay.rs");
