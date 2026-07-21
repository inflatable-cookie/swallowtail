#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Scenario {
    Success,
    Permission,
    Cancellation,
    Disconnect,
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
}

impl SharedAgent {
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
            Some("initialize") => Self::enqueue(
                &mut state,
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "protocolVersion": 1,
                        "agentCapabilities": {"loadSession": true},
                        "authMethods": [{"id": "gemini-api-key", "name": "Gemini API key"}],
                        "agentInfo": {"name": "gemini-cli", "version": "0.51.0"}
                    }
                }),
            ),
            Some("session/new") => Self::enqueue(
                &mut state,
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "sessionId": "fixture-session",
                        "modes": {"currentModeId": "plan"},
                        "models": {"currentModelId": "fixture-observed-model"}
                    }
                }),
            ),
            Some("session/prompt") => {
                state.prompt_id = id;
                match self.scenario {
                    Scenario::Success => {
                        Self::enqueue(
                            &mut state,
                            json!({
                                "jsonrpc": "2.0",
                                "method": "session/update",
                                "params": {
                                    "sessionId": "fixture-session",
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
                                "id": 701,
                                "method": "fs/read_text_file",
                                "params": {
                                    "sessionId": "fixture-session",
                                    "path": "/private/fixture/src/lib.rs",
                                    "line": 1,
                                    "limit": 32
                                }
                            }),
                        );
                    }
                    Scenario::Permission => Self::enqueue(
                        &mut state,
                        json!({
                            "jsonrpc": "2.0",
                            "id": 900,
                            "method": "session/request_permission",
                            "params": {
                                "sessionId": "fixture-session",
                                "toolCall": {"toolCallId": "fixture-tool"},
                                "options": [{
                                    "optionId": "allow-once",
                                    "name": "Allow once",
                                    "kind": "allow_once"
                                }]
                            }
                        }),
                    ),
                    Scenario::Cancellation => {}
                    Scenario::Disconnect => state.stopped = true,
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
            None if id == Some(701) => {
                if message
                    .get("result")
                    .and_then(|result| result.get("content"))
                    .is_none()
                {
                    return Err(fixture_failure());
                }
                Self::enqueue(
                    &mut state,
                    json!({
                        "jsonrpc": "2.0",
                        "method": "session/update",
                        "params": {
                            "sessionId": "fixture-session",
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
            None if id == Some(900) => {}
            _ => return Err(fixture_failure()),
        }
        self.changed.notify_all();
        Ok(())
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
