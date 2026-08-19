#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Scenario {
    Success,
    UnexpectedWrite,
    Permission,
    Cancellation,
    Disconnect,
    AuthRequired,
    Malformed,
    ProtocolMismatch,
    Oversized,
}

#[derive(Clone, Debug)]
pub struct ObservedProcess {
    pub arguments: Vec<String>,
    pub environment_count: usize,
    pub working_resource: Option<WorkingResourceRef>,
}

#[derive(Default)]
struct AgentState {
    output: VecDeque<ProcessOutputChunk>,
    writes: Vec<Value>,
    prompt_id: Option<u64>,
    stopped: bool,
}

struct SharedAgent {
    state: Mutex<AgentState>,
    changed: Condvar,
    scenario: Scenario,
    version: String,
}

impl SharedAgent {
    fn enqueue(state: &mut AgentState, message: Value) {
        let mut bytes = serde_json::to_vec(&message).expect("fixture message serializes");
        bytes.push(b'\n');
        state
            .output
            .push_back(ProcessOutputChunk::new(ProcessOutputStream::Stdout, bytes));
    }

    fn enqueue_raw(state: &mut AgentState, bytes: Vec<u8>) {
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
            Some("initialize") => match self.scenario {
                Scenario::Malformed => {
                    Self::enqueue_raw(&mut state, b"{\n".to_vec());
                }
                Scenario::ProtocolMismatch => Self::enqueue(
                    &mut state,
                    json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {
                            "protocolVersion": 2,
                            "agentInfo": {"name": "kiro-cli", "version": self.version}
                        }
                    }),
                ),
                _ => Self::enqueue(
                    &mut state,
                    json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {
                            "protocolVersion": 1,
                            "agentCapabilities": {
                                "loadSession": true,
                                "promptCapabilities": { "image": true }
                            },
                            "agentInfo": {"name": "kiro-cli", "version": self.version}
                        }
                    }),
                ),
            },
            Some("session/new") => match self.scenario {
                Scenario::AuthRequired => Self::enqueue(
                    &mut state,
                    json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "error": {
                            "code": -32603,
                            "message": "not authenticated; sign in or set API key"
                        }
                    }),
                ),
                _ => {
                    Self::enqueue(
                        &mut state,
                        json!({
                            "jsonrpc": "2.0",
                            "id": id,
                            "result": {
                                "sessionId": "opaque-fixture-session"
                            }
                        }),
                    );
                    enqueue_session_metadata(&mut state);
                }
            },
            Some("session/prompt") => {
                state.prompt_id = id;
                match self.scenario {
                    Scenario::Success => {
                        enqueue_session_metadata(&mut state);
                        Self::enqueue(
                            &mut state,
                            json!({
                                "jsonrpc": "2.0",
                                "method": "session/update",
                                "params": {
                                    "sessionId": "opaque-fixture-session",
                                    "update": {
                                        "sessionUpdate": "agent_message_chunk",
                                        "content": {"type": "text", "text": "fixture "}
                                    }
                                }
                            }),
                        );
                        Self::enqueue(
                            &mut state,
                            json!({
                                "jsonrpc": "2.0",
                                "method": "session/update",
                                "params": {
                                    "sessionId": "opaque-fixture-session",
                                    "update": {
                                        "sessionUpdate": "agent_message_chunk",
                                        "content": {"type": "text", "text": "response."}
                                    }
                                }
                            }),
                        );
                        if let Some(prompt_id) = state.prompt_id.take() {
                            Self::enqueue(
                                &mut state,
                                json!({
                                    "jsonrpc": "2.0",
                                    "id": prompt_id,
                                    "result": {"stopReason": "end_turn"}
                                }),
                            );
                        }
                    }
                    Scenario::UnexpectedWrite => Self::enqueue(
                        &mut state,
                        json!({
                            "jsonrpc": "2.0",
                            "id": 702,
                            "method": "fs/write_text_file",
                            "params": {
                                "sessionId": "opaque-fixture-session",
                                "path": "/private/fixture/src/lib.rs",
                                "content": "fixture replacement"
                            }
                        }),
                    ),
                    Scenario::Permission => Self::enqueue(
                        &mut state,
                        json!({
                            "jsonrpc": "2.0",
                            "id": 900,
                            "method": "session/request_permission",
                            "params": {
                                "sessionId": "opaque-fixture-session",
                                "toolCall": {"toolCallId": "tool-kiro", "status": "pending"},
                                "options": [
                                    {"optionId": "allow_always", "name": "allow_always", "kind": "allow_always"},
                                    {"optionId": "allow_once", "name": "allow_once", "kind": "allow_once"},
                                    {"optionId": "reject_once", "name": "reject_once", "kind": "reject_once"},
                                    {"optionId": "reject_always", "name": "reject_always", "kind": "reject_always"}
                                ]
                            }
                        }),
                    ),
                    Scenario::Oversized => {
                        let mut bytes = b"{\"jsonrpc\":\"2.0\",\"method\":\"session/update\",\"params\":{\"sessionId\":\"opaque-fixture-session\",\"update\":{\"sessionUpdate\":\"agent_message_chunk\",\"content\":{\"type\":\"text\",\"text\":\"".to_vec();
                        bytes.extend(std::iter::repeat_n(b'x', 64 * 1024));
                        bytes.extend(br#"\"}}}"#);
                        bytes.push(b'\n');
                        Self::enqueue_raw(&mut state, bytes);
                    }
                    Scenario::Cancellation => {}
                    Scenario::Disconnect => state.stopped = true,
                    Scenario::AuthRequired | Scenario::Malformed | Scenario::ProtocolMismatch => {}
                }
            }
            Some("session/cancel") => {
                if let Some(prompt_id) = state.prompt_id.take() {
                    Self::enqueue(
                        &mut state,
                        json!({
                            "jsonrpc": "2.0",
                            "id": prompt_id,
                            "result": {"stopReason": "cancelled"}
                        }),
                    );
                }
            }
            Some("session/load") | Some("authenticate") => return Err(fixture_failure()),
            None if id == Some(900) => {}
            None if id == Some(702) => {}
            _ => return Err(fixture_failure()),
        }
        self.changed.notify_all();
        Ok(())
    }
}

fn enqueue_session_metadata(state: &mut AgentState) {
    for update in [
        json!({"sessionUpdate": "config_option_update", "configOptions": []}),
        json!({"sessionUpdate": "current_mode_update", "currentModeId": "approve"}),
    ] {
        SharedAgent::enqueue(
            state,
            json!({
                "jsonrpc": "2.0",
                "method": "session/update",
                "params": {"sessionId": "opaque-fixture-session", "update": update}
            }),
        );
    }
}

struct FixtureProcessHandle(Arc<SharedAgent>);

impl ProcessHandle for FixtureProcessHandle {
    fn write_stdin(&self, chunk: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        let result = self.0.handle_write(chunk);
        Box::pin(async move { result })
    }

    fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.stop()
    }

    fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
        Box::pin(async move {
            let mut state = self.0.state.lock().expect("fixture agent lock poisoned");
            while state.output.is_empty() && !state.stopped {
                state = self
                    .0
                    .changed
                    .wait(state)
                    .expect("fixture agent wait lock poisoned");
            }
            Ok(state.output.pop_front())
        })
    }

    fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.stop()
    }

    fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.stop()
    }

    fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
        Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
    }
}

impl FixtureProcessHandle {
    fn stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        let mut state = self.0.state.lock().expect("fixture agent lock poisoned");
        state.stopped = true;
        self.0.changed.notify_all();
        Box::pin(async { Ok(()) })
    }
}
