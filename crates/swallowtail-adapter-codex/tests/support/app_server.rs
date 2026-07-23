use super::ObservedProcessRequest;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use swallowtail_runtime::{
    BoxFuture, ProcessExit, ProcessHandle, ProcessInputChunk, ProcessOutputChunk,
    ProcessOutputStream, ProcessRequest, ProcessService, RuntimeFailure, ScopeId,
};

#[derive(Clone, Copy)]
pub enum AppServerMode {
    CompleteTurn,
    HoldCatalog,
    HoldTurn,
    RequestCallback,
    ObserveApproval,
    ObserveUserInput,
    DynamicToolCall,
    HoldDynamicToolCall,
    DisconnectTurn,
    MismatchedTurnSession,
    SubstituteResume,
}

#[derive(Default)]
pub struct AppServerState {
    request: Mutex<Option<ObservedProcessRequest>>,
    messages: Mutex<Vec<serde_json::Value>>,
    output: Mutex<VecDeque<ProcessOutputChunk>>,
    input: Mutex<Vec<u8>>,
    active_thread: Mutex<Option<String>>,
    experimental_api: AtomicBool,
    closed: AtomicBool,
    forced: AtomicBool,
    waited: AtomicBool,
}

impl AppServerState {
    pub fn started(&self) -> bool {
        self.request
            .lock()
            .expect("request lock is available")
            .is_some()
    }

    pub fn request(&self) -> ObservedProcessRequest {
        self.request
            .lock()
            .expect("request lock is available")
            .clone()
            .expect("process request was captured")
    }

    pub fn methods(&self) -> Vec<String> {
        self.messages
            .lock()
            .expect("messages lock is available")
            .iter()
            .filter_map(|message| message.get("method")?.as_str().map(str::to_owned))
            .collect()
    }

    pub fn messages(&self) -> Vec<serde_json::Value> {
        self.messages
            .lock()
            .expect("messages lock is available")
            .clone()
    }

    pub fn forced(&self) -> bool {
        self.forced.load(Ordering::SeqCst)
    }

    pub fn waited(&self) -> bool {
        self.waited.load(Ordering::SeqCst)
    }

    fn push(&self, message: serde_json::Value) {
        let mut bytes = serde_json::to_vec(&message).expect("fixture JSON is valid");
        bytes.push(b'\n');
        self.output
            .lock()
            .expect("output lock is available")
            .push_back(ProcessOutputChunk::new(ProcessOutputStream::Stdout, bytes));
    }
}

pub struct ScriptedAppServer {
    state: Arc<AppServerState>,
    mode: AppServerMode,
    enforce_experimental_gate: bool,
    started: AtomicBool,
}

impl ScriptedAppServer {
    pub fn new(mode: AppServerMode) -> (Arc<Self>, Arc<AppServerState>) {
        Self::with_experimental_gate(mode, false)
    }

    pub fn gate_enforcing(mode: AppServerMode) -> (Arc<Self>, Arc<AppServerState>) {
        Self::with_experimental_gate(mode, true)
    }

    fn with_experimental_gate(
        mode: AppServerMode,
        enforce_experimental_gate: bool,
    ) -> (Arc<Self>, Arc<AppServerState>) {
        let state = Arc::new(AppServerState::default());
        (
            Arc::new(Self {
                state: Arc::clone(&state),
                mode,
                enforce_experimental_gate,
                started: AtomicBool::new(false),
            }),
            state,
        )
    }
}

impl ProcessService for ScriptedAppServer {
    fn start(
        &self,
        _scope: ScopeId,
        request: ProcessRequest,
    ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
        assert!(!self.started.swap(true, Ordering::SeqCst));
        *self
            .state
            .request
            .lock()
            .expect("request lock is available") = Some(ObservedProcessRequest {
            executable: request.executable().as_host_value().to_owned(),
            arguments: request.arguments().map(str::to_owned).collect(),
            environments: request
                .environment()
                .map(|value| value.as_host_value().to_owned())
                .collect(),
            working_resource: request
                .working_resource()
                .map(|value| value.as_host_value().to_owned()),
        });
        let handle = ScriptedAppServerHandle {
            state: Arc::clone(&self.state),
            mode: self.mode,
            enforce_experimental_gate: self.enforce_experimental_gate,
        };
        Box::pin(async move { Ok(Box::new(handle) as Box<dyn ProcessHandle>) })
    }
}

struct ScriptedAppServerHandle {
    state: Arc<AppServerState>,
    mode: AppServerMode,
    enforce_experimental_gate: bool,
}

impl ScriptedAppServerHandle {
    fn accept_input(&self, bytes: &[u8]) {
        let mut input = self.state.input.lock().expect("input lock is available");
        input.extend_from_slice(bytes);
        let mut lines = Vec::new();
        while let Some(newline) = input.iter().position(|byte| *byte == b'\n') {
            lines.push(input.drain(..=newline).collect::<Vec<_>>());
        }
        drop(input);
        for line in lines {
            let message: serde_json::Value =
                serde_json::from_slice(&line).expect("driver sends valid JSONL");
            self.state
                .messages
                .lock()
                .expect("messages lock is available")
                .push(message.clone());
            self.respond(&message);
        }
    }

    fn respond(&self, message: &serde_json::Value) {
        if message.get("id").and_then(serde_json::Value::as_str) == Some("callback-900")
            && message.get("result").is_some()
        {
            if matches!(self.mode, AppServerMode::DynamicToolCall) {
                self.complete_turn("completed");
            }
            return;
        }
        let Some(method) = message.get("method").and_then(serde_json::Value::as_str) else {
            return;
        };
        if method == "initialize" {
            let enabled = message
                .pointer("/params/capabilities/experimentalApi")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false);
            self.state.experimental_api.store(enabled, Ordering::SeqCst);
        } else if self.enforce_experimental_gate
            && message_requires_experimental_api(message)
            && !self.state.experimental_api.load(Ordering::SeqCst)
        {
            if let Some(id) = message.get("id") {
                self.state.push(serde_json::json!({
                    "id": id,
                    "error": {
                        "code": -32602,
                        "message": "experimentalApi capability required"
                    }
                }));
            }
            return;
        }
        let id = message.get("id").and_then(serde_json::Value::as_u64);
        match (method, id) {
            ("initialize", Some(id)) => self.state.push(serde_json::json!({
                "id": id,
                "result": {
                    "codexHome": "/private/codex-home",
                    "platformFamily": "unix",
                    "platformOs": "macos",
                    "userAgent": "fixture"
                }
            })),
            ("model/list", Some(id)) => {
                if matches!(self.mode, AppServerMode::HoldCatalog) {
                    return;
                }
                let cursor = message
                    .get("params")
                    .and_then(|params| params.get("cursor"))
                    .and_then(serde_json::Value::as_str);
                if cursor.is_none() {
                    self.state.push(serde_json::json!({
                        "id": id,
                        "result": {
                            "data": [{
                                "model": "gpt-5.4-mini",
                                "displayName": "GPT-5.4 Mini",
                                "description": "Fast structured work",
                                "isDefault": true,
                                "supportedReasoningEfforts": [
                                    {"reasoningEffort": "low", "description": "Fast"},
                                    {"reasoningEffort": "medium", "description": "Balanced"}
                                ],
                                "defaultReasoningEffort": "medium"
                            }],
                            "nextCursor": "page-2"
                        }
                    }));
                } else {
                    self.state.push(serde_json::json!({
                        "id": id,
                        "result": {
                            "data": [{
                                "model": "gpt-5.4",
                                "displayName": "GPT-5.4",
                                "description": "Deep structured work",
                                "isDefault": false,
                                "supportedReasoningEfforts": [
                                    {"reasoningEffort": "low", "description": "Fast"},
                                    {"reasoningEffort": "high", "description": "Deep"}
                                ],
                                "defaultReasoningEffort": "high"
                            }],
                            "nextCursor": null
                        }
                    }));
                }
            }
            ("thread/start", Some(id)) => self.state.push(serde_json::json!({
                "id": id,
                "result": {"thread": {"id": "thread-provider-new"}}
            })),
            ("thread/resume", Some(id)) => {
                let thread_id = if matches!(self.mode, AppServerMode::SubstituteResume) {
                    serde_json::Value::String("thread-provider-substituted".to_owned())
                } else {
                    message["params"]["threadId"].clone()
                };
                self.state.push(serde_json::json!({
                    "id": id,
                    "result": {"thread": {"id": thread_id}}
                }));
            }
            ("turn/start", Some(id)) => {
                let thread_id = message["params"]["threadId"]
                    .as_str()
                    .expect("turn/start carries a thread id")
                    .to_owned();
                *self
                    .state
                    .active_thread
                    .lock()
                    .expect("active thread lock is available") = Some(thread_id.clone());
                let notification_thread =
                    if matches!(self.mode, AppServerMode::MismatchedTurnSession) {
                        "thread-provider-unrelated".to_owned()
                    } else {
                        thread_id.clone()
                    };
                self.state.push(serde_json::json!({
                    "id": id,
                    "result": {"turn": {"id": "turn-provider-1"}}
                }));
                self.state.push(serde_json::json!({
                    "method": "turn/started",
                    "params": {
                        "threadId": notification_thread,
                        "turn": {"id": "turn-provider-1", "items": [], "status": "inProgress"}
                    }
                }));
                match self.mode {
                    AppServerMode::CompleteTurn => self.complete_turn("completed"),
                    AppServerMode::HoldCatalog
                    | AppServerMode::HoldTurn
                    | AppServerMode::MismatchedTurnSession
                    | AppServerMode::SubstituteResume => {}
                    AppServerMode::RequestCallback => self.state.push(serde_json::json!({
                        "id": "callback-900",
                        "method": "item/commandExecution/requestApproval",
                        "params": {}
                    })),
                    AppServerMode::ObserveApproval => self.state.push(serde_json::json!({
                        "id": "approval-900",
                        "method": "item/commandExecution/requestApproval",
                        "params": {
                            "threadId": thread_id,
                            "turnId": "turn-provider-1",
                            "itemId": "command-1",
                            "reason": "private approval body"
                        }
                    })),
                    AppServerMode::ObserveUserInput => self.state.push(serde_json::json!({
                        "id": "input-900",
                        "method": "item/tool/requestUserInput",
                        "params": {
                            "threadId": thread_id,
                            "turnId": "turn-provider-1",
                            "itemId": "input-1",
                            "questions": [{"id": "choice", "question": "private question"}]
                        }
                    })),
                    AppServerMode::DynamicToolCall | AppServerMode::HoldDynamicToolCall => {
                        self.state.push(serde_json::json!({
                            "id": "callback-900",
                            "method": "item/tool/call",
                            "params": {
                                "threadId": thread_id,
                                "turnId": "turn-provider-1",
                                "callId": "provider-call-1",
                                "tool": "task_ledger",
                                "arguments": {"operation": "list"}
                            }
                        }));
                    }
                    AppServerMode::DisconnectTurn => {
                        self.state.closed.store(true, Ordering::SeqCst);
                    }
                }
            }
            ("turn/interrupt", Some(id)) => {
                self.state.push(serde_json::json!({"id": id, "result": {}}));
                self.complete_turn("interrupted");
            }
            _ => {}
        }
    }

    fn complete_turn(&self, status: &str) {
        let thread_id = self
            .state
            .active_thread
            .lock()
            .expect("active thread lock is available")
            .clone()
            .expect("a turn is active");
        if status == "completed" {
            self.state.push(serde_json::json!({
                "method": "item/completed",
                "params": {
                    "threadId": thread_id,
                    "turnId": "turn-provider-1",
                    "completedAtMs": 1,
                    "item": {"id": "item-empty", "type": "agentMessage", "text": ""}
                }
            }));
            self.state.push(serde_json::json!({
                "method": "item/agentMessage/delta",
                "params": {
                    "threadId": thread_id,
                    "turnId": "turn-provider-1",
                    "itemId": "item-1",
                    "delta": "final "
                }
            }));
            self.state.push(serde_json::json!({
                "method": "item/agentMessage/delta",
                "params": {
                    "threadId": thread_id,
                    "turnId": "turn-provider-1",
                    "itemId": "item-1",
                    "delta": " "
                }
            }));
            self.state.push(serde_json::json!({
                "method": "item/completed",
                "params": {
                    "threadId": thread_id,
                    "turnId": "turn-provider-1",
                    "completedAtMs": 1,
                    "item": {"id": "item-1", "type": "agentMessage", "text": "final answer"}
                }
            }));
        }
        self.state.push(serde_json::json!({
            "method": "turn/completed",
            "params": {
                "threadId": thread_id,
                "turn": {"id": "turn-provider-1", "items": [], "status": status}
            }
        }));
    }
}

fn message_requires_experimental_api(message: &serde_json::Value) -> bool {
    const EXPERIMENTAL_FIELDS: &[&str] = &[
        "allowProviderModelFallback",
        "collaborationMode",
        "dynamicTools",
        "runtimeWorkspaceRoots",
    ];
    message
        .get("params")
        .and_then(serde_json::Value::as_object)
        .is_some_and(|params| {
            EXPERIMENTAL_FIELDS
                .iter()
                .any(|field| params.contains_key(*field))
        })
}

impl ProcessHandle for ScriptedAppServerHandle {
    fn write_stdin(&self, chunk: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.accept_input(chunk.bytes());
        Box::pin(async { Ok(()) })
    }

    fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.state.closed.store(true, Ordering::SeqCst);
        Box::pin(async { Ok(()) })
    }

    fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
        Box::pin(async move {
            loop {
                if let Some(chunk) = self
                    .state
                    .output
                    .lock()
                    .expect("output lock is available")
                    .pop_front()
                {
                    return Ok(Some(chunk));
                }
                if self.state.closed.load(Ordering::SeqCst)
                    || self.state.forced.load(Ordering::SeqCst)
                {
                    return Ok(None);
                }
                thread::yield_now();
            }
        })
    }

    fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.state.closed.store(true, Ordering::SeqCst);
        Box::pin(async { Ok(()) })
    }

    fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.state.forced.store(true, Ordering::SeqCst);
        Box::pin(async { Ok(()) })
    }

    fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
        self.state.waited.store(true, Ordering::SeqCst);
        Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
    }
}
