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
    CatalogueChanged,
    CataloguePaginated,
    CatalogueHold,
    CatalogueDisconnect,
    CatalogueUnsupported,
    CleanupFailure,
    ReasoningLegacySuccess,
    ReasoningEffortSuccess,
    ReasoningEffort291Success,
    ReasoningEffort292Success,
    ReasoningEffort300Success,
    ReasoningEffort310Success,
    ReasoningEffort311Success,
    ReasoningNewerSuccess,
    ReasoningEffortExtended,
    ReasoningEffortNarrow,
    ReasoningMissing,
    ReasoningAmbiguous,
    ReasoningMalformed,
    ReasoningRejected,
    ReasoningConfirmationMissing,
    ReasoningDrift,
    ReasoningAlwaysThinking,
    PlanSuccess,
    PlanLegacySuccess,
    PlanCeilingSuccess,
    PlanNewerSuccess,
    PlanMissing,
    PlanAmbiguous,
    PlanMalformed,
    PlanConfirmationMissing,
    PlanDrift,
    PlanRejected,
    PlanUnknownRow,
}

impl Scenario {
    fn version(self) -> &'static str {
        match self {
            Self::Complete
            | Self::HoldPrompt
            | Self::DisconnectPrompt
            | Self::CatalogueChanged
            | Self::CataloguePaginated
            | Self::CatalogueHold
            | Self::CatalogueDisconnect
            | Self::CatalogueUnsupported
            | Self::CleanupFailure
            | Self::ReasoningLegacySuccess => "0.28.1",
            Self::ReasoningEffort291Success => "0.29.1",
            Self::ReasoningEffort292Success => "0.29.2",
            Self::ReasoningEffort300Success => "0.30.0",
            Self::ReasoningEffort310Success => "0.31.0",
            Self::ReasoningEffort311Success => "0.31.1",
            Self::ReasoningNewerSuccess | Self::PlanNewerSuccess => "0.38.1",
            Self::PlanLegacySuccess => "0.28.1",
            Self::PlanCeilingSuccess => "0.38.0",
            _ => "0.29.0",
        }
    }

    fn has_reasoning(self) -> bool {
        !matches!(
            self,
            Self::Complete
                | Self::HoldPrompt
                | Self::DisconnectPrompt
                | Self::CatalogueChanged
                | Self::CataloguePaginated
                | Self::CatalogueHold
                | Self::CatalogueDisconnect
                | Self::CatalogueUnsupported
                | Self::CleanupFailure
                | Self::PlanSuccess
                | Self::PlanLegacySuccess
                | Self::PlanCeilingSuccess
                | Self::PlanNewerSuccess
                | Self::PlanMissing
                | Self::PlanAmbiguous
                | Self::PlanMalformed
                | Self::PlanUnknownRow
        )
    }

    fn has_plan(self) -> bool {
        matches!(
            self,
            Self::PlanSuccess
                | Self::PlanLegacySuccess
                | Self::PlanCeilingSuccess
                | Self::PlanNewerSuccess
                | Self::PlanConfirmationMissing
                | Self::PlanDrift
                | Self::PlanRejected
                | Self::ReasoningLegacySuccess
                | Self::ReasoningEffortSuccess
                | Self::ReasoningEffort291Success
                | Self::ReasoningEffort292Success
                | Self::ReasoningEffort300Success
                | Self::ReasoningEffort310Success
                | Self::ReasoningEffort311Success
                | Self::ReasoningNewerSuccess
                | Self::ReasoningEffortExtended
        )
    }

    pub(super) fn cleanup_fails(self) -> bool {
        matches!(self, Self::CleanupFailure)
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
                enqueue_session_metadata(&mut state);
            }
            Some("session/load") => self.load(&mut state, id),
            Some("session/list") => {
                if matches!(self.scenario, Scenario::CatalogueHold) {
                    self.changed.notify_all();
                    return Ok(());
                }
                if matches!(self.scenario, Scenario::CatalogueDisconnect) {
                    state.stopped = true;
                    self.changed.notify_all();
                    return Ok(());
                }
                let cwd = message["params"]["cwd"]
                    .as_str()
                    .ok_or_else(fixture_failure)?;
                let second_page = message["params"]["cursor"].as_str() == Some("page-2");
                let title = if matches!(self.scenario, Scenario::CatalogueChanged) {
                    "Changed Kimi fixture session"
                } else if second_page {
                    "Second Kimi fixture session"
                } else {
                    "Kimi fixture session"
                };
                let session_id = if second_page {
                    "kimi-session-second"
                } else {
                    "kimi-session-bound"
                };
                let next_cursor =
                    if matches!(self.scenario, Scenario::CataloguePaginated) && !second_page {
                        Some("page-2")
                    } else {
                        None
                    };
                Self::enqueue(
                    &mut state,
                    Self::response(
                        id,
                        json!({
                            "sessions": [{
                                "sessionId": session_id,
                                "cwd": cwd,
                                "title": title,
                                "updatedAt": "2026-08-01T12:34:56.789Z",
                                "_meta": {"fixturePrivate": "not-public"}
                            }],
                            "nextCursor": next_cursor,
                            "_meta": {"fixturePrivate": "not-public"}
                        }),
                    ),
                );
            }
            Some("session/resume") => {
                Self::enqueue(&mut state, Self::response(id, self.session_configuration()));
                enqueue_session_metadata(&mut state);
            }
            Some("session/set_config_option") => {
                self.set_config_option(&mut state, id, &message)?
            }
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
                    "agentCapabilities": if matches!(self.scenario, Scenario::CatalogueUnsupported) {
                        json!({"loadSession": true, "sessionCapabilities": {"resume": {}}})
                    } else {
                        json!({"loadSession": true, "sessionCapabilities": {"list": {}, "resume": {}}})
                    },
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
        enqueue_session_metadata(state);
    }

    fn prompt(&self, state: &mut AgentState, id: Option<u64>) -> Result<(), RuntimeFailure> {
        state.prompt_id = id;
        match self.scenario {
            Scenario::Complete
            | Scenario::ReasoningLegacySuccess
            | Scenario::ReasoningEffortSuccess
            | Scenario::ReasoningEffort291Success
            | Scenario::ReasoningEffort292Success
            | Scenario::ReasoningEffort300Success
            | Scenario::ReasoningEffort310Success
            | Scenario::ReasoningEffort311Success
            | Scenario::ReasoningNewerSuccess => {
                enqueue_session_metadata(state);
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
            Scenario::CatalogueChanged => return Err(fixture_failure()),
            Scenario::CataloguePaginated
            | Scenario::CatalogueHold
            | Scenario::CatalogueDisconnect
            | Scenario::CatalogueUnsupported
            | Scenario::CleanupFailure => return Err(fixture_failure()),
            _ => return Err(fixture_failure()),
        }
        Ok(())
    }
}

fn enqueue_session_metadata(state: &mut AgentState) {
    for update in [
        json!({"sessionUpdate": "available_commands_update", "availableCommands": []}),
        json!({"sessionUpdate": "config_option_update", "configOptions": []}),
        json!({"sessionUpdate": "current_mode_update", "currentModeId": "default"}),
    ] {
        SharedAgent::enqueue(
            state,
            json!({
                "jsonrpc": "2.0",
                "method": "session/update",
                "params": {"sessionId": "kimi-session-bound", "update": update}
            }),
        );
    }
}

include!("agent/reasoning.rs");
include!("agent/mode.rs");
include!("agent/config.rs");

fn finish_prompt(state: &mut AgentState, reason: &str) {
    if let Some(id) = state.prompt_id.take() {
        SharedAgent::enqueue(
            state,
            SharedAgent::response(Some(id), json!({"stopReason": reason})),
        );
    }
}
