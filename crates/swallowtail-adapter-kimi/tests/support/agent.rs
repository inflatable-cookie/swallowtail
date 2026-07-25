use super::fixture_failure;
use serde_json::{Value, json};
use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};
use swallowtail_runtime::{
    ProcessInputChunk, ProcessOutputChunk, ProcessOutputStream, RuntimeFailure,
};

#[derive(Clone, Copy)]
pub enum Scenario {
    Complete,
    HoldPrompt,
    DisconnectPrompt,
    ReasoningLegacySuccess,
    ReasoningEffortSuccess,
    ReasoningNewerSuccess,
    ReasoningMissing,
    ReasoningAmbiguous,
    ReasoningMalformed,
    ReasoningRejected,
    ReasoningConfirmationMissing,
    ReasoningDrift,
    ReasoningAlwaysThinking,
}

impl Scenario {
    fn version(self) -> &'static str {
        match self {
            Self::Complete
            | Self::HoldPrompt
            | Self::DisconnectPrompt
            | Self::ReasoningLegacySuccess => "0.28.1",
            Self::ReasoningNewerSuccess => "0.30.0",
            _ => "0.29.0",
        }
    }

    fn has_reasoning(self) -> bool {
        !matches!(
            self,
            Self::Complete | Self::HoldPrompt | Self::DisconnectPrompt
        )
    }
}

#[derive(Default)]
pub(super) struct AgentState {
    pub(super) output: VecDeque<ProcessOutputChunk>,
    pub(super) writes: Vec<Value>,
    prompt_id: Option<u64>,
    pub(super) stopped: bool,
}

pub(super) struct SharedAgent {
    pub(super) state: Mutex<AgentState>,
    pub(super) changed: Condvar,
    pub(super) scenario: Scenario,
}

impl SharedAgent {
    fn enqueue(state: &mut AgentState, message: Value) {
        let mut bytes = serde_json::to_vec(&message).expect("fixture message serializes");
        bytes.push(b'\n');
        state
            .output
            .push_back(ProcessOutputChunk::new(ProcessOutputStream::Stdout, bytes));
    }

    fn response(id: Option<u64>, result: Value) -> Value {
        json!({"jsonrpc": "2.0", "id": id, "result": result})
    }

    fn session_configuration(&self) -> Value {
        let mut options = vec![json!({"id": "model", "currentValue": "kimi-coder"})];
        match self.scenario {
            Scenario::ReasoningLegacySuccess | Scenario::ReasoningRejected => {
                options.push(reasoning_option(&["off", "on"], "off"));
            }
            Scenario::ReasoningEffortSuccess
            | Scenario::ReasoningNewerSuccess
            | Scenario::ReasoningConfirmationMissing
            | Scenario::ReasoningDrift => {
                options.push(reasoning_option(&["off", "low", "medium", "high"], "off"));
            }
            Scenario::ReasoningAmbiguous => {
                let option = reasoning_option(&["off", "low", "medium", "high"], "off");
                options.push(option.clone());
                options.push(option);
            }
            Scenario::ReasoningMalformed => {
                let mut option = reasoning_option(&["off", "low", "medium", "high"], "off");
                option["category"] = Value::String("unmapped_provider_category".to_owned());
                options.push(option);
            }
            Scenario::ReasoningAlwaysThinking => {
                options.push(reasoning_option(&["low", "medium", "high"], "medium"));
            }
            Scenario::Complete
            | Scenario::HoldPrompt
            | Scenario::DisconnectPrompt
            | Scenario::ReasoningMissing => {}
        }
        json!({"configOptions": options})
    }

    pub(super) fn handle_write(&self, chunk: ProcessInputChunk) -> Result<(), RuntimeFailure> {
        let message: Value =
            serde_json::from_slice(chunk.bytes()).map_err(|_| fixture_failure())?;
        let mut state = self.state.lock().expect("fixture agent lock poisoned");
        state.writes.push(message.clone());
        let id = message.get("id").and_then(Value::as_u64);
        match message.get("method").and_then(Value::as_str) {
            Some("initialize") => self.initialize(&mut state, id),
            Some("session/new") => {
                let mut result = self.session_configuration();
                result["sessionId"] = Value::String("kimi-session-bound".to_owned());
                Self::enqueue(&mut state, Self::response(id, result));
            }
            Some("session/load") => self.load(&mut state, id),
            Some("session/resume") => {
                Self::enqueue(&mut state, Self::response(id, self.session_configuration()));
                Self::enqueue(&mut state, passive_update());
            }
            Some("session/set_config_option") => self.set_reasoning(&mut state, id, &message)?,
            Some("session/prompt") => self.prompt(&mut state, id)?,
            Some("session/cancel") => finish_prompt(&mut state, "cancelled"),
            None if id == Some(701) && message.get("result") == Some(&Value::Null) => {
                finish_prompt(&mut state, "end_turn");
            }
            _ => return Err(fixture_failure()),
        }
        self.changed.notify_all();
        Ok(())
    }

    fn initialize(&self, state: &mut AgentState, id: Option<u64>) {
        Self::enqueue(
            state,
            Self::response(
                id,
                json!({
                    "protocolVersion": 1,
                    "agentCapabilities": {"loadSession": true, "sessionCapabilities": {"list": {}, "resume": {}}},
                    "authMethods": [{"id": "login", "type": "terminal"}],
                    "agentInfo": {"name": "Kimi Code CLI", "version": self.scenario.version()}
                }),
            ),
        );
    }

    fn load(&self, state: &mut AgentState, id: Option<u64>) {
        for (kind, text) in [
            ("user_message_chunk", "Previous question."),
            ("agent_message_chunk", "Previous answer."),
        ] {
            Self::enqueue(
                state,
                json!({"jsonrpc": "2.0", "method": "session/update", "params": {
                    "sessionId": "kimi-session-bound",
                    "update": {"sessionUpdate": kind, "content": {"type": "text", "text": text}}
                }}),
            );
        }
        Self::enqueue(state, Self::response(id, self.session_configuration()));
        Self::enqueue(state, passive_update());
    }

    fn prompt(&self, state: &mut AgentState, id: Option<u64>) -> Result<(), RuntimeFailure> {
        state.prompt_id = id;
        match self.scenario {
            Scenario::Complete
            | Scenario::ReasoningLegacySuccess
            | Scenario::ReasoningEffortSuccess
            | Scenario::ReasoningNewerSuccess => {
                Self::enqueue(
                    state,
                    json!({"jsonrpc": "2.0", "method": "session/update", "params": {
                        "sessionId": "kimi-session-bound",
                        "update": {"sessionUpdate": "agent_message_chunk", "content": {"type": "text", "text": "Kimi fixture response."}}
                    }}),
                );
                Self::enqueue(
                    state,
                    json!({"jsonrpc": "2.0", "id": 701, "method": "fs/write_text_file", "params": {
                        "sessionId": "kimi-session-bound", "path": "src/generated.rs", "content": "pub fn generated() {}\n"
                    }}),
                );
            }
            Scenario::HoldPrompt => {}
            Scenario::DisconnectPrompt => state.stopped = true,
            _ => return Err(fixture_failure()),
        }
        Ok(())
    }
}

fn passive_update() -> Value {
    json!({"jsonrpc": "2.0", "method": "session/update", "params": {
        "sessionId": "kimi-session-bound",
        "update": {"sessionUpdate": "available_commands_update", "availableCommands": []}
    }})
}

include!("agent/reasoning.rs");

fn finish_prompt(state: &mut AgentState, reason: &str) {
    if let Some(id) = state.prompt_id.take() {
        SharedAgent::enqueue(
            state,
            SharedAgent::response(Some(id), json!({"stopReason": reason})),
        );
    }
}
