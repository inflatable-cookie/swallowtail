impl AcpConnection {
    pub(crate) async fn initialize(&self) -> Result<Value, RuntimeFailure> {
        self.initialize_with_write_capability(true).await
    }

    pub(crate) async fn initialize_catalogue(&self) -> Result<Value, RuntimeFailure> {
        self.initialize_with_write_capability(false).await
    }

    async fn initialize_with_write_capability(
        &self,
        write_text_file: bool,
    ) -> Result<Value, RuntimeFailure> {
        let response = self
            .request_with_id(
                0,
                "initialize",
                json!({
                    "protocolVersion": ACP_PROTOCOL_VERSION,
                    "clientCapabilities": {
                        "fs": {"readTextFile": false, "writeTextFile": write_text_file},
                        "terminal": false,
                        "auth": {"terminal": false}
                    },
                    "clientInfo": {
                        "name": "swallowtail",
                        "title": "Swallowtail",
                        "version": env!("CARGO_PKG_VERSION")
                    }
                }),
            )
            .await?;
        if response.get("protocolVersion").and_then(Value::as_u64) != Some(ACP_PROTOCOL_VERSION) {
            return Err(failure(
                "swallowtail.kimi.acp.version_mismatch",
                "Kimi Code negotiated an incompatible ACP version",
            ));
        }
        Ok(response)
    }

    pub(crate) async fn new_session(&self, cwd: String) -> Result<Value, RuntimeFailure> {
        self.request("session/new", json!({"cwd": cwd, "mcpServers": []}))
            .await
    }

    pub(crate) async fn set_config_option(
        &self,
        session_id: &str,
        value: &str,
    ) -> Result<Value, RuntimeFailure> {
        self.request(
            "session/set_config_option",
            json!({"sessionId": session_id, "configId": "thinking", "value": value}),
        )
        .await
    }

    pub(crate) async fn load_session(
        &self,
        session: SessionRef,
        cwd: String,
    ) -> Result<(Value, Vec<SessionReplayItem>), RuntimeFailure> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        self.set_session_id(session.as_provider_value().to_owned())?;
        *self.phase.lock().expect("ACP phase lock poisoned") = Some(AttachPhase::Loading {
            response_id: id,
            session: session.clone(),
            response_seen: false,
            bytes: 0,
            replay: Vec::new(),
        });
        let response = self
            .begin_request_with_id(
                id,
                "session/load",
                json!({"sessionId": session.as_provider_value(), "cwd": cwd, "mcpServers": []}),
            )
            .await?
            .await?;
        let phase = self.phase.lock().expect("ACP phase lock poisoned").take();
        match phase {
            Some(AttachPhase::Loading {
                response_seen: true,
                replay,
                ..
            }) => Ok((response, replay)),
            _ => Err(protocol_failure()),
        }
    }

    pub(crate) async fn resume_session(
        &self,
        session: SessionRef,
        cwd: String,
    ) -> Result<Value, RuntimeFailure> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        self.set_session_id(session.as_provider_value().to_owned())?;
        *self.phase.lock().expect("ACP phase lock poisoned") = Some(AttachPhase::Resuming {
            response_id: id,
            session: session.clone(),
            response_seen: false,
        });
        let response = self
            .begin_request_with_id(
                id,
                "session/resume",
                json!({"sessionId": session.as_provider_value(), "cwd": cwd, "mcpServers": []}),
            )
            .await?
            .await?;
        let phase = self.phase.lock().expect("ACP phase lock poisoned").take();
        match phase {
            Some(AttachPhase::Resuming {
                response_seen: true,
                ..
            }) => Ok(response),
            _ => Err(protocol_failure()),
        }
    }

    pub(crate) async fn list_sessions(
        &self,
        capabilities: swallowtail_protocol_acp::AcpSessionListCapabilities,
        cwd: String,
        cursor: Option<String>,
        limits: swallowtail_protocol_acp::AcpSessionListLimits,
    ) -> Result<swallowtail_protocol_acp::AcpSessionListPage, RuntimeFailure> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let request = swallowtail_protocol_acp::AcpSessionListRequest::new(
            json!(id),
            capabilities,
            Some(cwd),
            cursor,
            limits,
        )
        .map_err(|_| {
            failure(
                "swallowtail.kimi.acp.session_list_request_invalid",
                "Kimi Code ACP session-list request is invalid",
            )
        })?;
        let result = self
            .begin_request_with_id(
                id,
                swallowtail_protocol_acp::ACP_SESSION_LIST_METHOD,
                json!({"cwd": request.cwd(), "cursor": request.cursor()}),
            )
            .await?
            .await?;
        request.decode_result(&result).map_err(|_| {
            failure(
                "swallowtail.kimi.acp.session_list_response_invalid",
                "Kimi Code returned invalid ACP session-list evidence",
            )
        })
    }
}
