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
        if message.get("result").is_some() {
            let provider_id = message.get("id").and_then(serde_json::Value::as_str);
            if provider_id == Some("callback-900")
                && matches!(self.mode, AppServerMode::DynamicToolCall)
            {
                self.complete_turn("completed");
                return;
            }
            if provider_id == Some("input-900")
                && matches!(self.mode, AppServerMode::ExchangeUserInput)
            {
                self.complete_turn("completed");
                return;
            }
            if message.get("id").and_then(serde_json::Value::as_i64) == Some(900)
                && matches!(self.mode, AppServerMode::ExchangeUserInputNumericRequestId)
            {
                let thread_id = self
                    .state
                    .active_thread
                    .lock()
                    .expect("active thread lock is available")
                    .clone()
                    .expect("a turn is active");
                self.state.push(serde_json::json!({
                    "method": "serverRequest/resolved",
                    "params": {"threadId": thread_id, "requestId": 900}
                }));
                self.complete_turn("completed");
                return;
            }
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
                let turns = if message["params"]["excludeTurns"] == true {
                    serde_json::Value::Null
                } else {
                    serde_json::json!([
                        {"id":"turn-1","items":[
                            {"type":"userMessage","id":"item-1","clientId":null,"content":[
                                {"type":"text","text":"Earlier question."}
                            ]}
                        ]},
                        {"id":"turn-2","items":[
                            {"type":"agentMessage","id":"item-2","text":"Earlier answer."}
                        ]}
                    ])
                };
                let mut thread = serde_json::json!({"id": thread_id});
                if !turns.is_null() {
                    thread["turns"] = turns;
                }
                self.state.push(serde_json::json!({
                    "id": id,
                    "result": {"thread": thread}
                }));
            }
            (method @ ("thread/archive" | "thread/unarchive" | "thread/delete"), Some(id)) => {
                if matches!(self.mode, AppServerMode::LifecycleDisconnect) {
                    self.state.closed.store(true, Ordering::SeqCst);
                    return;
                }
                if matches!(self.mode, AppServerMode::LifecycleHold) {
                    return;
                }
                if matches!(self.mode, AppServerMode::LifecycleReject) {
                    self.state.push(serde_json::json!({
                        "id": id,
                        "error": {"code": -32602, "message": "unknown thread"}
                    }));
                    return;
                }
                let thread_id = message["params"]["threadId"]
                    .as_str()
                    .expect("lifecycle request carries a thread id");
                let result = if matches!(self.mode, AppServerMode::LifecycleMalformed) {
                    serde_json::json!({"unexpected": true})
                } else if method == "thread/unarchive" {
                    serde_json::json!({"thread": {"id": thread_id}})
                } else {
                    serde_json::json!({})
                };
                self.state
                    .push(serde_json::json!({"id": id, "result": result}));
                let notification = match method {
                    "thread/archive" => "thread/archived",
                    "thread/unarchive" => "thread/unarchived",
                    "thread/delete" => "thread/deleted",
                    _ => unreachable!(),
                };
                let notification_thread =
                    if matches!(self.mode, AppServerMode::LifecycleWrongNotification) {
                        "thread-provider-unrelated"
                    } else {
                        thread_id
                    };
                self.state.push(serde_json::json!({
                    "method": notification,
                    "params": {"threadId": notification_thread}
                }));
                if method == "thread/delete" {
                    self.state.push(serde_json::json!({
                        "method": notification,
                        "params": {"threadId": "thread-provider-descendant"}
                    }));
                }
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
                    | AppServerMode::SubstituteResume
                    | AppServerMode::LifecycleSuccess
                    | AppServerMode::LifecycleReject
                    | AppServerMode::LifecycleDisconnect
                    | AppServerMode::LifecycleHold
                    | AppServerMode::LifecycleMalformed
                    | AppServerMode::LifecycleCleanupFailure
                    | AppServerMode::LifecycleWrongNotification => {}
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
                    AppServerMode::ExchangeUserInput => self.state.push(serde_json::json!({
                        "id": "input-900",
                        "method": "item/tool/requestUserInput",
                        "params": {
                            "threadId": thread_id,
                            "turnId": "turn-provider-1",
                            "itemId": "input-1",
                            "autoResolutionMs": 60000,
                            "questions": [{
                                "id": "scope",
                                "header": "Scope",
                                "question": "Choose a scope",
                                "isOther": true,
                                "isSecret": false,
                                "options": [
                                    {"label": "Tests", "description": "Change tests"},
                                    {"label": "Docs", "description": "Change docs"}
                                ]
                            }]
                        }
                    })),
                    AppServerMode::ExchangeUserInputNumericRequestId => {
                        self.state.push(serde_json::json!({
                            "id": 900,
                            "method": "item/tool/requestUserInput",
                            "params": {
                                "threadId": thread_id,
                                "turnId": "turn-provider-1",
                                "itemId": "input-1",
                                "questions": [{
                                    "id": "scope",
                                    "header": "Scope",
                                    "question": "Choose a scope",
                                    "isOther": false,
                                    "isSecret": false,
                                    "options": [
                                        {"label": "Tests", "description": "Change tests"}
                                    ]
                                }]
                            }
                        }));
                    }
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
