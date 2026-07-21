use futures_executor::block_on;
use serde_json::{Value, json};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::JoinHandle;
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, AdapterId, Capability,
    CapabilityConstraint, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    ConfiguredInstanceId, CredentialMechanism, CredentialRef, CredentialState, DriverRole,
    EndpointAudience, EndpointAuthorization, EntitlementMetering, EntitlementState,
    ExecutionHostId, ExecutionLayer, HostServiceKind, InstanceOwnership, InstancePolicyId,
    InstanceRevision, InstanceTargetRef, ModelId, ModelRoute, ModelRouteId, ModelRouteRevision,
    OperationRequirements, OperationShape, PreflightContext, PreflightPlan, ProtocolFacadeId,
    ResourceAccess, ResourceRepresentation, RuntimeReadiness, SessionAccessPolicy,
    SupportAuthority, preflight,
};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, CredentialLease, CredentialService, DelegatedCredential,
    HostServices, JoinedTask, ProcessExit, ProcessHandle, ProcessInputChunk, ProcessOutputChunk,
    ProcessOutputStream, ProcessRequest, ProcessService, ResourceLease, RuntimeFailure, ScopeId,
    ScopedTaskService, WorkingResourceIoService, WorkingResourceReadRequest, WorkingResourceRef,
    WorkingResourceService, WorkingResourceText, WorkingResourceWriteRequest,
};

mod selection;
pub use selection::selection;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CleanupEvent {
    ProcessWait,
    ResourceRelease,
    CredentialRelease,
}

#[derive(Clone, Copy)]
pub enum Scenario {
    Complete,
    HoldPrompt,
    DisconnectPrompt,
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

    fn response(id: Option<u64>, result: Value) -> Value {
        json!({"jsonrpc": "2.0", "id": id, "result": result})
    }

    fn session_configuration() -> Value {
        json!({"configOptions": [{"id": "model", "currentValue": "kimi-coder"}]})
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
                Self::response(
                    id,
                    json!({
                        "protocolVersion": 1,
                        "agentCapabilities": {"loadSession": true, "sessionCapabilities": {"list": {}, "resume": {}}},
                        "authMethods": [{"id": "login", "type": "terminal"}],
                        "agentInfo": {"name": "Kimi Code CLI", "version": "0.28.1"}
                    }),
                ),
            ),
            Some("session/new") => {
                let mut result = Self::session_configuration();
                result["sessionId"] = Value::String("kimi-session-bound".to_owned());
                Self::enqueue(&mut state, Self::response(id, result));
            }
            Some("session/load") => {
                for (kind, text) in [
                    ("user_message_chunk", "Previous question."),
                    ("agent_message_chunk", "Previous answer."),
                ] {
                    Self::enqueue(
                        &mut state,
                        json!({"jsonrpc": "2.0", "method": "session/update", "params": {
                            "sessionId": "kimi-session-bound",
                            "update": {"sessionUpdate": kind, "content": {"type": "text", "text": text}}
                        }}),
                    );
                }
                Self::enqueue(
                    &mut state,
                    Self::response(id, Self::session_configuration()),
                );
                Self::enqueue(&mut state, passive_update());
            }
            Some("session/resume") => {
                Self::enqueue(
                    &mut state,
                    Self::response(id, Self::session_configuration()),
                );
                Self::enqueue(&mut state, passive_update());
            }
            Some("session/prompt") => {
                state.prompt_id = id;
                match self.scenario {
                    Scenario::Complete => {
                        Self::enqueue(
                            &mut state,
                            json!({"jsonrpc": "2.0", "method": "session/update", "params": {
                                "sessionId": "kimi-session-bound",
                                "update": {"sessionUpdate": "agent_message_chunk", "content": {"type": "text", "text": "Kimi fixture response."}}
                            }}),
                        );
                        Self::enqueue(
                            &mut state,
                            json!({"jsonrpc": "2.0", "id": 701, "method": "fs/write_text_file", "params": {
                                "sessionId": "kimi-session-bound", "path": "src/generated.rs", "content": "pub fn generated() {}\n"
                            }}),
                        );
                    }
                    Scenario::HoldPrompt => {}
                    Scenario::DisconnectPrompt => {
                        state.stopped = true;
                    }
                }
            }
            Some("session/cancel") => finish_prompt(&mut state, "cancelled"),
            None if id == Some(701) && message.get("result") == Some(&Value::Null) => {
                finish_prompt(&mut state, "end_turn");
            }
            _ => return Err(fixture_failure()),
        }
        self.changed.notify_all();
        Ok(())
    }
}

fn passive_update() -> Value {
    json!({"jsonrpc": "2.0", "method": "session/update", "params": {
        "sessionId": "kimi-session-bound",
        "update": {"sessionUpdate": "available_commands_update", "availableCommands": []}
    }})
}

fn finish_prompt(state: &mut AgentState, reason: &str) {
    if let Some(id) = state.prompt_id.take() {
        SharedAgent::enqueue(
            state,
            SharedAgent::response(Some(id), json!({"stopReason": reason})),
        );
    }
}

include!("host.rs");
