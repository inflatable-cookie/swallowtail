use super::*;

impl SharedAgent {
    pub(super) fn close_session(
        &self,
        state: &mut AgentState,
        id: Option<u64>,
    ) -> Result<(), RuntimeFailure> {
        self.cancel(state)?;
        Self::enqueue(state, json!({"jsonrpc": "2.0", "id": id, "result": {}}));
        Ok(())
    }

    pub(super) fn delete_session(
        &self,
        state: &mut AgentState,
        id: Option<u64>,
    ) -> Result<(), RuntimeFailure> {
        match self.scenario {
            Scenario::DeleteMissing => Self::enqueue(
                state,
                json!({"jsonrpc": "2.0", "id": id, "error": {
                    "code": -32603, "message": "private missing target"
                }}),
            ),
            Scenario::DeleteProviderFailure => Self::enqueue(
                state,
                json!({"jsonrpc": "2.0", "id": id, "error": {
                    "code": -32603, "message": "private provider failure"
                }}),
            ),
            Scenario::DeleteDisconnect | Scenario::RunDeleteDisconnect => state.stopped = true,
            Scenario::DeleteMalformed => Self::enqueue(
                state,
                json!({"jsonrpc": "2.0", "id": id, "result": {"unexpected": true}}),
            ),
            Scenario::DeletePending => {}
            _ => Self::enqueue(state, json!({"jsonrpc": "2.0", "id": id, "result": {}})),
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub(in crate::support) fn wait_for_method(&self, method: &str) {
        let state = self.state.lock().expect("fixture agent lock poisoned");
        let (state, timeout) =
            self.changed
                .wait_timeout_while(state, std::time::Duration::from_secs(2), |state| {
                    !state.writes.iter().any(|message| {
                        message.get("method").and_then(Value::as_str) == Some(method)
                    })
                })
                .expect("fixture agent wait lock poisoned");
        assert!(!timeout.timed_out(), "fixture did not observe {method}");
        drop(state);
    }
}
