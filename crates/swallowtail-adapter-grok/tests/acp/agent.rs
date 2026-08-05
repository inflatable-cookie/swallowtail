#[derive(Clone, Copy, Debug)]
enum Scenario {
    Success,
    Permission,
    Cancellation,
    Deadline,
    Disconnect,
    Malformed,
    RecoveryForeign,
    RecoveryCallback,
    RecoveryMalformed,
    RecoveryOversized,
    RecoveryLate,
    RecoveryDisconnect,
    RecoveryResponseMismatch,
}

#[derive(Default)]
struct AgentState {
    output: VecDeque<ProcessOutputChunk>,
    writes: Vec<Value>,
    prompt_id: Option<u64>,
    stopped: bool,
}

struct Agent {
    state: Mutex<AgentState>,
    changed: Condvar,
    scenario: Scenario,
    version: String,
}

impl Agent {
    fn enqueue(state: &mut AgentState, message: Value) {
        let mut bytes = serde_json::to_vec(&message).expect("fixture serializes");
        bytes.push(b'\n');
        state
            .output
            .push_back(ProcessOutputChunk::new(ProcessOutputStream::Stdout, bytes));
    }

    fn update(state: &mut AgentState, update: Value) {
        Self::enqueue(
            state,
            json!({
                "jsonrpc": "2.0",
                "method": "session/update",
                "params": {"sessionId": "grok-fixture-session", "update": update}
            }),
        );
    }

    fn enqueue_many(state: &mut AgentState, messages: &[Value]) {
        let mut bytes = Vec::new();
        for message in messages {
            bytes.extend_from_slice(&serde_json::to_vec(message).expect("fixture serializes"));
            bytes.push(b'\n');
        }
        state
            .output
            .push_back(ProcessOutputChunk::new(ProcessOutputStream::Stdout, bytes));
    }

    fn write(&self, chunk: ProcessInputChunk) -> Result<(), RuntimeFailure> {
        let message: Value =
            serde_json::from_slice(chunk.bytes()).map_err(|_| fixture_failure())?;
        let mut state = self.state.lock().expect("agent lock poisoned");
        state.writes.push(message.clone());
        let id = message.get("id").and_then(Value::as_u64);
        match message.get("method").and_then(Value::as_str) {
            Some("initialize") => Self::enqueue(
                &mut state,
                if matches!(self.scenario, Scenario::Malformed) {
                    json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {
                            "protocolVersion": 1,
                            "providerSecret": "private-fixture-secret"
                        }
                    })
                } else {
                    json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {
                            "protocolVersion": 1,
                            "agentCapabilities": {
                                "loadSession": true,
                                "promptCapabilities": {"embeddedContext": true}
                            },
                            "authMethods": [
                                {"id": "cached_token", "name": "cached_token"},
                                {"id": "grok.com", "name": "Grok"}
                            ],
                            "_meta": {
                                "defaultAuthMethodId": "cached_token",
                                "agentVersion": self.version,
                                "modelState": {
                                    "currentModelId": "grok-4.5",
                                    "availableModels": [
                                        {"modelId": "grok-4.5", "name": "Grok 4.5"}
                                    ]
                                }
                            }
                        }
                    })
                },
            ),
            Some("authenticate") => Self::enqueue(
                &mut state,
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {"providerPrivateAccountMetadata": "must-be-discarded"}
                }),
            ),
            Some("session/new") => Self::enqueue(
                &mut state,
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {"sessionId": "grok-fixture-session"}
                }),
            ),
            Some("session/load") => match self.scenario {
                Scenario::RecoveryForeign => Self::enqueue(
                    &mut state,
                    json!({
                        "jsonrpc": "2.0",
                        "method": "session/update",
                        "params": {
                            "sessionId": "foreign-grok-session",
                            "update": {"sessionUpdate": "agent_message_chunk", "content": {"type": "text", "text": "foreign"}}
                        }
                    }),
                ),
                Scenario::RecoveryCallback => Self::enqueue(
                    &mut state,
                    json!({
                        "jsonrpc": "2.0",
                        "id": 901,
                        "method": "fs/read_text_file",
                        "params": {"sessionId": "grok-fixture-session", "path": "private.txt"}
                    }),
                ),
                Scenario::RecoveryMalformed => Self::enqueue(
                    &mut state,
                    json!({
                        "jsonrpc": "2.0",
                        "method": "session/update",
                        "params": {"sessionId": "grok-fixture-session", "update": "invalid"}
                    }),
                ),
                Scenario::RecoveryOversized => Self::update(
                    &mut state,
                    json!({
                        "sessionUpdate": "agent_message_chunk",
                        "content": {"type": "text", "text": "x".repeat(9 * 1024 * 1024)}
                    }),
                ),
                Scenario::RecoveryLate => Self::enqueue_many(
                    &mut state,
                    &[
                        json!({
                            "jsonrpc": "2.0",
                            "id": id,
                            "result": {"sessionId": "grok-fixture-session"}
                        }),
                        json!({
                            "jsonrpc": "2.0",
                            "method": "session/update",
                            "params": {
                                "sessionId": "grok-fixture-session",
                                "update": {"sessionUpdate": "agent_message_chunk", "content": {"type": "text", "text": "late"}}
                            }
                        }),
                    ],
                ),
                Scenario::RecoveryDisconnect => state.stopped = true,
                Scenario::RecoveryResponseMismatch => Self::enqueue(
                    &mut state,
                    json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {"sessionId": "different-grok-session"}
                    }),
                ),
                _ => {
                    Self::update(
                        &mut state,
                        json!({
                            "sessionUpdate": "tool_call_update",
                            "toolCallId": "historical-tool",
                            "status": "completed",
                            "content": [{
                                "type": "content",
                                "content": {"type": "text", "text": "x".repeat(128 * 1024)}
                            }]
                        }),
                    );
                    Self::enqueue(
                        &mut state,
                        json!({
                            "jsonrpc": "2.0",
                            "id": id,
                            "result": {"sessionId": "grok-fixture-session"}
                        }),
                    );
                }
            },
            Some("session/prompt") => {
                state.prompt_id = id;
                match self.scenario {
                    Scenario::Success => self.success(&mut state, id),
                    Scenario::Permission => Self::enqueue(
                        &mut state,
                        json!({
                            "jsonrpc": "2.0",
                            "id": 900,
                            "method": "session/request_permission",
                            "params": {
                                "sessionId": "grok-fixture-session",
                                "toolCall": {"toolCallId": "fixture-tool"},
                                "options": [{
                                    "optionId": "allow-once",
                                    "name": "Allow once",
                                    "kind": "allow_once"
                                }]
                            }
                        }),
                    ),
                    Scenario::Cancellation | Scenario::Deadline => {}
                    Scenario::Disconnect => {
                        state.stopped = true;
                    }
                    Scenario::Malformed => unreachable!("malformed initialization stops first"),
                    Scenario::RecoveryForeign
                    | Scenario::RecoveryCallback
                    | Scenario::RecoveryMalformed
                    | Scenario::RecoveryOversized
                    | Scenario::RecoveryLate
                    | Scenario::RecoveryDisconnect
                    | Scenario::RecoveryResponseMismatch => {
                        unreachable!("recovery scenarios do not prompt")
                    }
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
            None if matches!(id, Some(900 | 901)) => {}
            _ => return Err(fixture_failure()),
        }
        self.changed.notify_all();
        Ok(())
    }

    fn success(&self, state: &mut AgentState, id: Option<u64>) {
        for update in [
            json!({
                "sessionUpdate": "agent_thought_chunk",
                "content": {"type": "text", "text": "Fixture reasoning summary."}
            }),
            json!({
                "sessionUpdate": "plan",
                "entries": [{
                    "content": "Inspect fixture",
                    "priority": "medium",
                    "status": "in_progress"
                }]
            }),
            json!({
                "sessionUpdate": "tool_call",
                "toolCallId": "fixture-tool",
                "title": "Inspect",
                "status": "in_progress",
                "content": []
            }),
            json!({
                "sessionUpdate": "tool_call_update",
                "toolCallId": "fixture-tool",
                "status": "completed",
                "content": [{
                    "type": "content",
                    "content": {"type": "text", "text": "Fixture tool display."}
                }]
            }),
            json!({
                "sessionUpdate": "agent_message_chunk",
                "content": {"type": "text", "text": "Fixture response."}
            }),
        ] {
            Self::update(state, update);
        }
        state.prompt_id = None;
        Self::enqueue(
            state,
            json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {"stopReason": "end_turn"}
            }),
        );
    }
}
