use super::*;

impl AcpConnection {
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
        match self.phase.lock().expect("ACP phase lock poisoned").take() {
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
        match self.phase.lock().expect("ACP phase lock poisoned").take() {
            Some(AttachPhase::Resuming {
                response_seen: true,
                ..
            }) => Ok(response),
            _ => Err(protocol_failure()),
        }
    }
}
