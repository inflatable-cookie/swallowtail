#![allow(dead_code)]

mod mode;
mod process;

use serde_json::{Value, json};
use std::{
    collections::VecDeque,
    sync::{Condvar, Mutex},
};
use swallowtail_runtime::{
    ProcessInputChunk, ProcessOutputChunk, ProcessOutputStream, RuntimeFailure,
};

pub use process::FixtureProcessHandle;

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
    PlanMissingModes,
    PlanMissingConfig,
    PlanAmbiguousModes,
    PlanAmbiguousConfig,
    PlanMalformedModes,
    PlanMalformedConfig,
    PlanCurrentContradiction,
    PlanBlankSessionId,
    PlanRejected,
    PlanConfirmationMissing,
    PlanConfirmationAmbiguous,
    PlanConfirmationMalformed,
    PlanDrift,
    PlanDisconnect,
    ModelExact,
    ModelMalformed,
    ModelDuplicate,
    ModelUnadvertised,
    ModelUnbounded,
    ModelExactPlanDrift,
}

#[derive(Clone, Debug)]
pub struct ObservedProcess {
    pub arguments: Vec<String>,
    pub environment_count: usize,
    pub working_resource: Option<swallowtail_runtime::WorkingResourceRef>,
}

#[derive(Default)]
pub(super) struct AgentState {
    pub(super) output: VecDeque<ProcessOutputChunk>,
    pub(super) writes: Vec<Value>,
    pub(super) prompt_id: Option<u64>,
    pub(super) stopped: bool,
}

pub(crate) struct SharedAgent {
    pub(super) state: Mutex<AgentState>,
    pub(super) changed: Condvar,
    pub(super) scenario: Scenario,
    pub(super) version: String,
}

impl SharedAgent {
    pub(super) fn new(scenario: Scenario, version: String) -> Self {
        Self {
            state: Mutex::new(AgentState::default()),
            changed: Condvar::new(),
            scenario,
            version,
        }
    }

    pub(super) fn enqueue(state: &mut AgentState, message: Value) {
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

    pub(super) fn handle_write(&self, chunk: ProcessInputChunk) -> Result<(), RuntimeFailure> {
        let message: Value =
            serde_json::from_slice(chunk.bytes()).map_err(|_| super::fixture_failure())?;
        let mut state = self.state.lock().expect("fixture agent lock poisoned");
        state.writes.push(message.clone());
        let id = message.get("id").and_then(Value::as_u64);
        match message.get("method").and_then(Value::as_str) {
            Some("initialize") => self.initialize(&mut state, id),
            Some("session/new") => self.session_new(&mut state, id)?,
            Some("session/set_config_option") => {
                self.set_config_option(&mut state, id, &message)?
            }
            Some("session/prompt") => self.prompt(&mut state, id),
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
            Some("session/load") | Some("authenticate") => return Err(super::fixture_failure()),
            None if id == Some(900) || id == Some(702) => {}
            _ => return Err(super::fixture_failure()),
        }
        self.changed.notify_all();
        Ok(())
    }

    fn initialize(&self, state: &mut AgentState, id: Option<u64>) {
        match self.scenario {
            Scenario::Malformed => Self::enqueue_raw(state, b"{\n".to_vec()),
            Scenario::ProtocolMismatch => Self::enqueue(
                state,
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "protocolVersion": 2,
                        "agentInfo": {"name": "cline", "version": self.version}
                    }
                }),
            ),
            _ => Self::enqueue(
                state,
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "protocolVersion": 1,
                        "agentCapabilities": {
                            "loadSession": true,
                            "promptCapabilities": {
                                "image": true,
                                "audio": false,
                                "embeddedContext": false
                            }
                        },
                        "authMethods": [
                            {"id": "cline", "name": "Sign in with Cline"},
                            {"id": "cline-pass", "name": "Sign in with ClinePass"},
                            {"id": "openai-codex", "name": "Sign in with ChatGPT Subscription"}
                        ],
                        "agentInfo": {"name": "cline", "version": self.version}
                    }
                }),
            ),
        }
    }

    fn prompt(&self, state: &mut AgentState, id: Option<u64>) {
        state.prompt_id = id;
        match self.scenario {
            Scenario::Success => {
                enqueue_session_metadata(state);
                Self::enqueue(
                    state,
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
                    state,
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
                        state,
                        json!({
                            "jsonrpc": "2.0",
                            "id": prompt_id,
                            "result": {"stopReason": "end_turn"}
                        }),
                    );
                }
            }
            Scenario::UnexpectedWrite => Self::enqueue(
                state,
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
                state,
                json!({
                    "jsonrpc": "2.0",
                    "id": 900,
                    "method": "session/request_permission",
                    "params": {
                        "sessionId": "opaque-fixture-session",
                        "toolCall": {"toolCallId": "tool-cline", "status": "pending"},
                        "options": [
                            {"optionId": "allow_once", "name": "Allow once", "kind": "allow_once"},
                            {"optionId": "allow_always", "name": "Allow always", "kind": "allow_always"},
                            {"optionId": "reject_once", "name": "Reject", "kind": "reject_once"}
                        ]
                    }
                }),
            ),
            Scenario::Oversized => {
                let mut bytes = b"{\"jsonrpc\":\"2.0\",\"method\":\"session/update\",\"params\":{\"sessionId\":\"opaque-fixture-session\",\"update\":{\"sessionUpdate\":\"agent_message_chunk\",\"content\":{\"type\":\"text\",\"text\":\"".to_vec();
                bytes.extend(std::iter::repeat_n(b'x', 64 * 1024));
                bytes.extend(br#"\"}}}"#);
                bytes.push(b'\n');
                Self::enqueue_raw(state, bytes);
            }
            Scenario::Cancellation => {}
            Scenario::Disconnect => state.stopped = true,
            _ => {}
        }
    }
}

pub(super) fn enqueue_session_metadata(state: &mut AgentState) {
    for update in [
        json!({"sessionUpdate": "config_option_update", "configOptions": []}),
        json!({"sessionUpdate": "current_mode_update", "currentModeId": "act"}),
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
