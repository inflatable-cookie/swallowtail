use super::*;

#[path = "agent/lifecycle.rs"]
mod lifecycle;
mod process;

pub(super) use process::FixtureProcessHandle;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(dead_code)]
pub enum Scenario {
    Success,
    DeleteMissing,
    DeleteProviderFailure,
    DeleteDisconnect,
    DeleteMalformed,
    DeletePending,
    Permission,
    Cancellation,
    Disconnect,
    ModelDrift,
    AuthDrift,
    LifecycleDrift,
    Version,
}

#[derive(Clone, Debug)]
pub struct ObservedProcess {
    pub executable: String,
    pub arguments: Vec<String>,
    pub environment_count: usize,
    pub working_resource: Option<WorkingResourceRef>,
}

#[derive(Default)]
pub(super) struct AgentState {
    pub(super) output: VecDeque<ProcessOutputChunk>,
    pub(super) writes: Vec<Value>,
    prompt_id: Option<u64>,
    stopped: bool,
}

pub(super) struct SharedAgent {
    pub(super) state: Mutex<AgentState>,
    changed: Condvar,
    scenario: Scenario,
    version: String,
}

impl SharedAgent {
    pub(super) fn new(scenario: Scenario, version: &str) -> Arc<Self> {
        let mut state = AgentState::default();
        if scenario == Scenario::Version {
            state.output.push_back(ProcessOutputChunk::new(
                ProcessOutputStream::Stdout,
                format!("{version}\n").into_bytes(),
            ));
        }
        Arc::new(Self {
            state: Mutex::new(state),
            changed: Condvar::new(),
            scenario,
            version: version.to_owned(),
        })
    }

    fn enqueue(state: &mut AgentState, message: Value) {
        let mut bytes = serde_json::to_vec(&message).expect("fixture message serializes");
        bytes.push(b'\n');
        state
            .output
            .push_back(ProcessOutputChunk::new(ProcessOutputStream::Stdout, bytes));
    }

    fn handle_write(&self, chunk: ProcessInputChunk) -> Result<(), RuntimeFailure> {
        let message: Value =
            serde_json::from_slice(chunk.bytes()).map_err(|_| fixture_failure())?;
        let mut state = self.state.lock().expect("fixture agent lock poisoned");
        state.writes.push(message.clone());
        let id = message.get("id").and_then(Value::as_u64);
        match message.get("method").and_then(Value::as_str) {
            Some("initialize") => self.initialize(&mut state, id),
            Some("session/new") => self.new_session(&mut state, id, &message),
            Some("session/prompt") => self.prompt(&mut state, id),
            Some("session/cancel") => self.cancel(&mut state),
            Some("session/close") => self.close_session(&mut state, id),
            Some("session/delete") => self.delete_session(&mut state, id),
            None if id == Some(701) => self.complete_read(&mut state, &message),
            None if id == Some(900) => self.permission_response(&message),
            _ => return Err(fixture_failure()),
        }?;
        self.changed.notify_all();
        Ok(())
    }

    fn initialize(&self, state: &mut AgentState, id: Option<u64>) -> Result<(), RuntimeFailure> {
        let version = semver::Version::parse(&self.version).map_err(|_| fixture_failure())?;
        let mut capabilities = json!({
            "_meta": {"claudeCode": {"promptQueueing": true}},
            "promptCapabilities": {"image": true, "embeddedContext": true},
            "sessionCapabilities": {"close": {}, "delete": {}}
        });
        if self.scenario == Scenario::LifecycleDrift {
            capabilities["sessionCapabilities"] = json!({"close": {}});
        }
        if version >= semver::Version::new(0, 60, 0) {
            capabilities["providers"] = json!({});
        }
        let steering = (version >= semver::Version::new(0, 61, 0))
            .then(|| json!({"steering": {"supported": true}}));
        let auth_methods = if self.scenario == Scenario::AuthDrift {
            json!([{"id": "claude-login", "name": "Claude login"}])
        } else {
            json!([])
        };
        Self::enqueue(
            state,
            json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "protocolVersion": 1,
                    "agentCapabilities": capabilities,
                    "agentInfo": {
                        "name": "@agentclientprotocol/claude-agent-acp",
                        "version": self.version
                    },
                    "authMethods": auth_methods,
                    "_meta": steering
                }
            }),
        );
        Ok(())
    }

    fn new_session(
        &self,
        state: &mut AgentState,
        id: Option<u64>,
        message: &Value,
    ) -> Result<(), RuntimeFailure> {
        if message["params"]["_meta"]["claudeCode"]["options"]["tools"]
            != json!(["Read", "Glob", "Grep"])
        {
            return Err(fixture_failure());
        }
        let model = if self.scenario == Scenario::ModelDrift {
            "default"
        } else {
            "claude-sonnet-4-6"
        };
        Self::enqueue(
            state,
            json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "sessionId": "claude-agent-session-fixture",
                    "configOptions": [{
                        "type": "select",
                        "id": "model",
                        "currentValue": model,
                        "options": [{"value": model, "name": model}]
                    }]
                }
            }),
        );
        Ok(())
    }

    fn prompt(&self, state: &mut AgentState, id: Option<u64>) -> Result<(), RuntimeFailure> {
        state.prompt_id = id;
        match self.scenario {
            Scenario::Success => {
                for update in [
                    json!({"sessionUpdate": "agent_thought_chunk", "content": {"type": "text", "text": "Inspecting."}}),
                    json!({"sessionUpdate": "tool_call", "toolCallId": "read-1", "kind": "read"}),
                    json!({"sessionUpdate": "usage_update", "used": 42, "size": 200000}),
                    json!({"sessionUpdate": "agent_message_chunk", "content": {"type": "text", "text": "fixture response."}}),
                ] {
                    Self::enqueue(
                        state,
                        json!({"jsonrpc": "2.0", "method": "session/update", "params": {
                            "sessionId": "claude-agent-session-fixture", "update": update
                        }}),
                    );
                }
                Self::enqueue(
                    state,
                    json!({"jsonrpc": "2.0", "id": 701, "method": "fs/read_text_file", "params": {
                        "sessionId": "claude-agent-session-fixture",
                        "path": "/private/fixture/src/lib.rs"
                    }}),
                );
            }
            Scenario::Permission => {
                Self::enqueue(
                    state,
                    json!({"jsonrpc": "2.0", "method": "session/update", "params": {
                        "sessionId": "claude-agent-session-fixture",
                        "update": {"sessionUpdate": "tool_call", "toolCallId": "shell-1", "kind": "execute"}
                    }}),
                );
                Self::enqueue(
                    state,
                    json!({"jsonrpc": "2.0", "id": 900, "method": "session/request_permission", "params": {
                        "sessionId": "claude-agent-session-fixture",
                        "toolCall": {"toolCallId": "shell-1"},
                        "options": [
                            {"optionId": "allow-once", "kind": "allow_once"},
                            {"optionId": "reject-once", "kind": "reject_once"}
                        ]
                    }}),
                );
            }
            Scenario::Cancellation => {}
            Scenario::Disconnect => state.stopped = true,
            Scenario::DeleteMissing
            | Scenario::DeleteProviderFailure
            | Scenario::DeleteDisconnect
            | Scenario::DeleteMalformed
            | Scenario::DeletePending
            | Scenario::ModelDrift
            | Scenario::AuthDrift
            | Scenario::LifecycleDrift
            | Scenario::Version => {
                return Err(fixture_failure());
            }
        }
        Ok(())
    }

    fn complete_read(&self, state: &mut AgentState, message: &Value) -> Result<(), RuntimeFailure> {
        if message["result"]["content"] != "fixture file" {
            return Err(fixture_failure());
        }
        if let Some(id) = state.prompt_id.take() {
            Self::enqueue(
                state,
                json!({"jsonrpc": "2.0", "id": id, "result": {"stopReason": "end_turn"}}),
            );
        }
        Ok(())
    }

    fn permission_response(&self, message: &Value) -> Result<(), RuntimeFailure> {
        if message["result"]["outcome"]["outcome"] == "selected"
            && message["result"]["outcome"]["optionId"] == "reject-once"
        {
            Ok(())
        } else {
            Err(fixture_failure())
        }
    }

    fn cancel(&self, state: &mut AgentState) -> Result<(), RuntimeFailure> {
        if let Some(id) = state.prompt_id.take() {
            Self::enqueue(
                state,
                json!({"jsonrpc": "2.0", "id": id, "result": {"stopReason": "cancelled"}}),
            );
        }
        Ok(())
    }
}
