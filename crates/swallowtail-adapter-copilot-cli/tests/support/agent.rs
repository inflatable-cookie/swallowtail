#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Scenario {
    Success,
    UnexpectedWrite,
    Permission,
    AssessmentPermission,
    AssessmentProviderEffect,
    AssessmentExecutionFailure,
    AssessmentSessionNewFailure,
    AssessmentHangingUntilCancel,
    AssessmentPermissionMissingAction,
    AssessmentPermissionMismatchedAction,
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
    pub executable: String,
    pub environments: Vec<String>,
}

#[derive(Default)]
struct AgentState {
    output: VecDeque<ProcessOutputChunk>,
    writes: Vec<Value>,
    prompt_id: Option<u64>,
    stopped: bool,
    assessment_trace: AssessmentTrace,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AssessmentTrace {
    pub initialized_version: Option<String>,
    pub session_new_seen: bool,
    pub announced_tool_call_id: Option<String>,
    pub announced_kind: Option<String>,
    pub announced_path: Option<String>,
    pub announced_old_text: Option<String>,
    pub announced_new_text: Option<String>,
    pub permission_request_id: Option<u64>,
    pub permission_tool_call_id: Option<String>,
    pub permission_reply: Option<String>,
    pub session_cancel_seen: bool,
    pub prompt_result: Option<String>,
    pub sentinel_before: Vec<u8>,
    pub sentinel_after: Vec<u8>,
    pub task_directory_before: BTreeMap<String, Vec<u8>>,
    pub task_directory_after: BTreeMap<String, Vec<u8>>,
    pub effect_count: usize,
}

struct SharedAgent {
    state: Mutex<AgentState>,
    changed: Condvar,
    scenario: Scenario,
    version: String,
    task_directory: Option<std::path::PathBuf>,
    progress_ticks: Option<Arc<AtomicUsize>>,
    joined_tasks: AtomicUsize,
}

impl Scenario {
    fn is_assessment_point(self) -> bool {
        matches!(
            self,
            Self::AssessmentPermission
                | Self::AssessmentProviderEffect
                | Self::AssessmentExecutionFailure
                | Self::AssessmentSessionNewFailure
                | Self::AssessmentHangingUntilCancel
                | Self::AssessmentPermissionMissingAction
                | Self::AssessmentPermissionMismatchedAction
        )
    }

    fn emits_assessment_permission(self) -> bool {
        matches!(
            self,
            Self::AssessmentPermission
                | Self::AssessmentProviderEffect
                | Self::AssessmentPermissionMissingAction
                | Self::AssessmentPermissionMismatchedAction
        )
    }
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
        self.advance_progress(250);
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
                            "agentInfo": {"name": "copilot", "version": self.version}
                        }
                    }),
                ),
                scenario if scenario.is_assessment_point() => {
                    state.assessment_trace.initialized_version = Some(self.version.clone());
                    Self::enqueue(
                        &mut state,
                        json!({
                            "jsonrpc": "2.0",
                            "id": id,
                            "result": {
                                "protocolVersion": 1,
                                "agentInfo": {"name": "copilot", "version": self.version}
                            }
                        }),
                    );
                }
                _ => Self::enqueue(
                    &mut state,
                    json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {
                            "protocolVersion": 1
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
                            "message": "Unauthorized"
                        }
                    }),
                ),
                Scenario::AssessmentSessionNewFailure => Self::enqueue(
                    &mut state,
                    json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "error": {
                            "code": -32603,
                            "message": "Fixture session/new failed"
                        }
                    }),
                ),
                _ => {
                    if self.scenario.is_assessment_point() {
                        state.assessment_trace.session_new_seen = true;
                    }
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
                                "toolCall": {"toolCallId": "tool-copilot", "status": "pending"},
                                "options": [
                                    {"optionId": "allow_always", "name": "allow_always", "kind": "allow_always"},
                                    {"optionId": "allow_once", "name": "allow_once", "kind": "allow_once"},
                                    {"optionId": "reject_once", "name": "reject_once", "kind": "reject_once"},
                                    {"optionId": "reject_always", "name": "reject_always", "kind": "reject_always"}
                                ]
                            }
                        }),
                    ),
                    Scenario::AssessmentPermission
                    | Scenario::AssessmentPermissionMissingAction
                    | Scenario::AssessmentPermissionMismatchedAction => {
                        if self.scenario != Scenario::AssessmentPermissionMissingAction {
                            let tool_call_id = if self.scenario
                                == Scenario::AssessmentPermissionMismatchedAction
                            {
                                "different-tool"
                            } else {
                                "sentinel-edit"
                            };
                            state.assessment_trace.announced_tool_call_id =
                                Some(tool_call_id.to_owned());
                            state.assessment_trace.announced_kind = Some("execute".to_owned());
                            state.assessment_trace.announced_path =
                                Some("permission-sentinel.txt".to_owned());
                            state.assessment_trace.announced_old_text = Some(
                                "SWALLOWTAIL_PERMISSION_SENTINEL_BEFORE_V1\n".to_owned(),
                            );
                            state.assessment_trace.announced_new_text = Some(
                                "SWALLOWTAIL_PERMISSION_SENTINEL_AFTER_V1\n".to_owned(),
                            );
                            Self::enqueue(
                                &mut state,
                                json!({
                                    "jsonrpc": "2.0",
                                    "method": "session/update",
                                    "params": {
                                        "sessionId": "opaque-fixture-session",
                                        "update": {
                                            "sessionUpdate": "tool_call",
                                            "toolCallId": tool_call_id,
                                            "title": "Overwrite the task-owned permission sentinel",
                                            "kind": "execute",
                                            "status": "pending",
                                            "content": [{
                                                "type": "diff",
                                                "path": "permission-sentinel.txt",
                                                "oldText": "SWALLOWTAIL_PERMISSION_SENTINEL_BEFORE_V1\n",
                                                "newText": "SWALLOWTAIL_PERMISSION_SENTINEL_AFTER_V1\n"
                                            }],
                                            "locations": [{"path": "permission-sentinel.txt"}]
                                        }
                                    }
                                }),
                            );
                        }
                        let permission_tool_call_id = "sentinel-edit";
                        state.assessment_trace.permission_request_id = Some(900);
                        state.assessment_trace.permission_tool_call_id =
                            Some(permission_tool_call_id.to_owned());
                        Self::enqueue(
                            &mut state,
                            json!({
                                "jsonrpc": "2.0",
                                "id": 900,
                                "method": "session/request_permission",
                                "params": {
                                    "sessionId": "opaque-fixture-session",
                                    "toolCall": {"toolCallId": permission_tool_call_id, "status": "pending"},
                                    "options": [
                                        {"optionId": "allow_always", "name": "allow_always", "kind": "allow_always"},
                                        {"optionId": "allow_once", "name": "allow_once", "kind": "allow_once"},
                                        {"optionId": "reject_once", "name": "reject_once", "kind": "reject_once"},
                                        {"optionId": "reject_always", "name": "reject_always", "kind": "reject_always"}
                                    ]
                                }
                            }),
                        );
                    }
                    Scenario::AssessmentProviderEffect => {
                        state.assessment_trace.announced_tool_call_id =
                            Some("sentinel-edit".to_owned());
                        state.assessment_trace.announced_kind = Some("execute".to_owned());
                        state.assessment_trace.announced_path =
                            Some("permission-sentinel.txt".to_owned());
                        state.assessment_trace.announced_old_text =
                            Some("SWALLOWTAIL_PERMISSION_SENTINEL_BEFORE_V1\n".to_owned());
                        state.assessment_trace.announced_new_text =
                            Some("SWALLOWTAIL_PERMISSION_SENTINEL_AFTER_V1\n".to_owned());
                        enqueue_assessment_permission(&mut state, "sentinel-edit");
                    }
                    Scenario::AssessmentExecutionFailure => Self::enqueue(
                        &mut state,
                        json!({
                            "jsonrpc": "2.0",
                            "id": id,
                            "error": {
                                "code": -32603,
                                "message": "Fixture execution failed"
                            }
                        }),
                    ),
                    Scenario::AssessmentHangingUntilCancel => {}
                    Scenario::Oversized => {
                        let mut bytes = b"{\"jsonrpc\":\"2.0\",\"method\":\"session/update\",\"params\":{\"sessionId\":\"opaque-fixture-session\",\"update\":{\"sessionUpdate\":\"agent_message_chunk\",\"content\":{\"type\":\"text\",\"text\":\"".to_vec();
                        bytes.extend(std::iter::repeat_n(b'x', 64 * 1024));
                        bytes.extend(br#"\"}}}"#);
                        bytes.push(b'\n');
                        Self::enqueue_raw(&mut state, bytes);
                    }
                    Scenario::Cancellation => {}
                    Scenario::Disconnect => state.stopped = true,
                    Scenario::AuthRequired
                    | Scenario::Malformed
                    | Scenario::ProtocolMismatch
                    | Scenario::AssessmentSessionNewFailure => {}
                }
            }
            Some("session/cancel") => {
                if self.scenario.is_assessment_point() {
                    state.assessment_trace.session_cancel_seen = true;
                }
                if let Some(prompt_id) = state.prompt_id.take() {
                    if self.scenario.is_assessment_point() {
                        state.assessment_trace.prompt_result = Some("cancelled".to_owned());
                    }
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
            None if id == Some(900) => {
                if self.scenario.emits_assessment_permission() {
                    state.assessment_trace.permission_reply = message
                        .get("result")
                        .and_then(|result| result.get("outcome"))
                        .and_then(|outcome| outcome.get("outcome"))
                        .and_then(Value::as_str)
                        .map(str::to_owned);
                }
                if self.scenario == Scenario::AssessmentProviderEffect {
                    let path = self
                        .task_directory
                        .as_ref()
                        .expect("provider-effect scenario binds task-owned scratch")
                        .join("permission-sentinel.txt");
                    std::fs::write(&path, b"SWALLOWTAIL_PERMISSION_SENTINEL_AFTER_V1\n")
                        .map_err(|_| fixture_failure())?;
                }
            }
            None if id == Some(702) => {}
            _ => return Err(fixture_failure()),
        }
        self.changed.notify_all();
        Ok(())
    }
}

fn enqueue_assessment_permission(state: &mut AgentState, tool_call_id: &str) {
    state.assessment_trace.permission_request_id = Some(900);
    state.assessment_trace.permission_tool_call_id = Some(tool_call_id.to_owned());
    SharedAgent::enqueue(
        state,
        json!({
            "jsonrpc": "2.0",
            "method": "session/update",
            "params": {
                "sessionId": "opaque-fixture-session",
                "update": {
                    "sessionUpdate": "tool_call",
                    "toolCallId": tool_call_id,
                    "title": "Overwrite the task-owned permission sentinel",
                    "kind": "execute",
                    "status": "pending",
                    "content": [{
                        "type": "diff",
                        "path": "permission-sentinel.txt",
                        "oldText": "SWALLOWTAIL_PERMISSION_SENTINEL_BEFORE_V1\n",
                        "newText": "SWALLOWTAIL_PERMISSION_SENTINEL_AFTER_V1\n"
                    }],
                    "locations": [{"path": "permission-sentinel.txt"}]
                }
            }
        }),
    );
    SharedAgent::enqueue(
        state,
        json!({
            "jsonrpc": "2.0",
            "id": 900,
            "method": "session/request_permission",
            "params": {
                "sessionId": "opaque-fixture-session",
                "toolCall": {"toolCallId": tool_call_id, "status": "pending"},
                "options": [
                    {"optionId": "allow_always", "name": "allow_always", "kind": "allow_always"},
                    {"optionId": "allow_once", "name": "allow_once", "kind": "allow_once"},
                    {"optionId": "reject_once", "name": "reject_once", "kind": "reject_once"},
                    {"optionId": "reject_always", "name": "reject_always", "kind": "reject_always"}
                ]
            }
        }),
    );
}

fn enqueue_session_metadata(state: &mut AgentState) {
    SharedAgent::enqueue(
        state,
        json!({
            "jsonrpc": "2.0",
            "method": "session/update",
            "params": {
                "sessionId": "opaque-fixture-session",
                "update": {
                    "sessionUpdate": "available_commands_update",
                    "availableCommands": [{"name": "context", "description": "fixture"}]
                }
            }
        }),
    );
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
        let newly_stopped = !state.stopped;
        state.stopped = true;
        self.0.changed.notify_all();
        drop(state);
        if newly_stopped {
            self.0.advance_progress(500);
        }
        Box::pin(async { Ok(()) })
    }
}

impl SharedAgent {
    fn advance_progress(&self, ticks: usize) {
        if let Some(progress) = &self.progress_ticks {
            progress.fetch_add(ticks, Ordering::SeqCst);
        }
    }
}
