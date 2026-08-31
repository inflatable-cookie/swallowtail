impl SharedAgent {
    fn config_options(&self, state: &AgentState) -> Result<Value, RuntimeFailure> {
        let requested = state
            .requested_model
            .as_deref()
            .ok_or_else(fixture_failure)?;
        let current = state.current_model.as_deref().ok_or_else(fixture_failure)?;
        let mut options = vec![json!({
            "type": "select",
            "id": "model",
            "name": "Model",
            "currentValue": current,
            "options": [
                {"value": "default", "name": "Default"},
                {"value": requested, "name": requested}
            ]
        })];
        let version = semver::Version::parse(&self.version).map_err(|_| fixture_failure())?;
        if version >= semver::Version::new(0, 54, 0) {
            options.push(json!({
                "type": "select",
                "id": "mode",
                "name": "Mode",
                "category": "mode",
                "currentValue": state.mode.as_deref().unwrap_or("default"),
                "options": [
                    {"value": "default", "name": "Default"},
                    {"value": "plan", "name": "Plan"}
                ]
            }));
            let mut effort_options = ["default", "low", "medium", "high", "xhigh", "max"]
                .into_iter()
                .map(|value| json!({"value": value, "name": value}))
                .collect::<Vec<_>>();
            let selected = state.effort.as_deref();
            let current = match (self.scenario, selected) {
                (Scenario::ReasoningMismatchAdvertised, Some("low")) => json!("high"),
                (Scenario::ReasoningMismatchUnadvertised, Some("low")) => {
                    effort_options.retain(|option| option["value"] != "high");
                    json!("high")
                }
                (Scenario::ReasoningMismatchUnqualified, Some("low")) => {
                    effort_options.push(json!({"value": "ultra", "name": "ultra"}));
                    json!("ultra")
                }
                (Scenario::ReasoningConfirmationMalformed, Some(_)) => json!(42),
                (Scenario::ReasoningConfirmationUnbounded, Some(_)) => {
                    let value = "x".repeat(1024);
                    effort_options.push(json!({"value": value, "name": "unbounded"}));
                    json!(value)
                }
                _ => json!(selected.unwrap_or("default")),
            };
            let effort = json!({
                "type": "select",
                "id": "effort",
                "name": "Effort",
                "category": "thought_level",
                "currentValue": current,
                "options": effort_options
            });
            if !(self.scenario == Scenario::ReasoningConfirmationMissing && selected.is_some()) {
                options.push(effort.clone());
            }
            if self.scenario == Scenario::ReasoningConfirmationDuplicate && selected.is_some() {
                options.push(effort);
            }
        }
        Ok(Value::Array(options))
    }

    fn prompt(&self, state: &mut AgentState, id: Option<u64>) -> Result<(), RuntimeFailure> {
        state.prompt_id = id;
        match self.scenario {
            Scenario::Success
            | Scenario::LargeToolUpdate
            | Scenario::MalformedUsage
            | Scenario::RunDeleteDisconnect => {
                let config_options = self.config_options(state)?;
                for update in [
                    json!({"sessionUpdate": "available_commands_update", "availableCommands": []}),
                    json!({"sessionUpdate": "config_option_update", "configOptions": config_options}),
                    json!({"sessionUpdate": "current_mode_update", "currentModeId": "acceptEdits"}),
                    json!({"sessionUpdate": "agent_thought_chunk", "content": {"type": "text", "text": "Inspecting."}}),
                    json!({"sessionUpdate": "tool_call", "toolCallId": "read-1", "title": "Read fixture", "kind": "read"}),
                    json!({"sessionUpdate": "usage_update", "used": 42, "size": 200000}),
                    json!({"sessionUpdate": "agent_message_chunk", "content": {"type": "text", "text": "fixture response."}}),
                ] {
                    Self::enqueue(
                        state,
                        json!({"jsonrpc": "2.0", "method": "session/update", "params": {
                            "sessionId": "claude-agent-session-fixture", "update": update
                        }}),
                    );
                }
                if self.scenario == Scenario::LargeToolUpdate {
                    Self::enqueue(
                        state,
                        json!({"jsonrpc": "2.0", "method": "session/update", "params": {
                            "sessionId": "claude-agent-session-fixture",
                            "update": {
                                "sessionUpdate": "tool_call_update",
                                "toolCallId": "read-1",
                                "status": "completed",
                                "rawOutput": "x".repeat(136 * 1024)
                            }
                        }}),
                    );
                }
                Self::enqueue(
                    state,
                    json!({"jsonrpc": "2.0", "id": 701, "method": "fs/read_text_file", "params": {
                        "sessionId": "claude-agent-session-fixture",
                        "path": "/private/fixture/src/lib.rs"
                    }}),
                );
            }
            Scenario::Permission => {
                Self::enqueue(
                    state,
                    json!({"jsonrpc": "2.0", "method": "session/update", "params": {
                        "sessionId": "claude-agent-session-fixture",
                        "update": {"sessionUpdate": "tool_call", "toolCallId": "shell-1", "title": "Run command", "kind": "execute"}
                    }}),
                );
                Self::enqueue(
                    state,
                    json!({"jsonrpc": "2.0", "id": 900, "method": "session/request_permission", "params": {
                        "sessionId": "claude-agent-session-fixture",
                        "toolCall": {"toolCallId": "shell-1"},
                        "options": [
                            {"optionId": "allow-once", "kind": "allow_once"},
                            {"optionId": "allow-always", "kind": "allow_always"},
                            {"optionId": "reject-once", "kind": "reject_once"}
                        ]
                    }}),
                );
            }
            Scenario::Elicitation => {
                Self::enqueue(
                    state,
                    json!({"jsonrpc": "2.0", "id": 901, "method": "elicitation/create", "params": {
                        "mode": "form",
                        "sessionId": "claude-agent-session-fixture",
                        "toolCallId": "ask-1",
                        "message": "Which component should be used?",
                        "requestedSchema": {
                            "type": "object",
                            "properties": {
                                "question_0": {
                                    "type": "string",
                                    "title": "Component",
                                    "oneOf": [
                                        {
                                            "const": "Card",
                                            "title": "Card",
                                            "description": "Use the card."
                                        },
                                        {
                                            "const": "Panel",
                                            "title": "Panel",
                                            "description": "Use the panel."
                                        }
                                    ]
                                },
                                "question_0_custom": {
                                    "type": "string",
                                    "title": "Other",
                                    "description": "Type your own answer instead of choosing an option above (optional).",
                                    "_meta": {
                                        "_askUserQuestionCustomAnswer": {
                                            "questionId": "question_0",
                                            "isCustomAnswer": true
                                        }
                                    }
                                }
                            }
                        }
                    }}),
                );
            }
            Scenario::Cancellation | Scenario::ClosePending => {}
            Scenario::Disconnect => state.stopped = true,
            Scenario::DeleteMissing
            | Scenario::DeleteProviderFailure
            | Scenario::DeleteDisconnect
            | Scenario::DeleteMalformed
            | Scenario::DeletePending
            | Scenario::ModelDrift
            | Scenario::AuthDrift
            | Scenario::LifecycleDrift
            | Scenario::ReasoningMismatchAdvertised
            | Scenario::ReasoningMismatchUnadvertised
            | Scenario::ReasoningMismatchUnqualified
            | Scenario::ReasoningConfirmationMissing
            | Scenario::ReasoningConfirmationMalformed
            | Scenario::ReasoningConfirmationDuplicate
            | Scenario::ReasoningConfirmationUnbounded
            | Scenario::Version => {
                return Err(fixture_failure());
            }
        }
        Ok(())
    }

    fn complete_read(&self, state: &mut AgentState, message: &Value) -> Result<(), RuntimeFailure> {
        if message["result"]["content"] != "fixture file" {
            return Err(fixture_failure());
        }
        if let Some(id) = state.prompt_id.take() {
            let total_tokens = if self.scenario == Scenario::MalformedUsage {
                22
            } else {
                21
            };
            Self::enqueue(
                state,
                json!({"jsonrpc": "2.0", "id": id, "result": {
                    "stopReason": "end_turn",
                    "usage": {
                        "inputTokens": 12,
                        "outputTokens": 4,
                        "cachedReadTokens": 3,
                        "cachedWriteTokens": 2,
                        "totalTokens": total_tokens
                    }
                }}),
            );
        }
        Ok(())
    }

    fn permission_response(
        &self,
        state: &mut AgentState,
        message: &Value,
    ) -> Result<(), RuntimeFailure> {
        if message["result"]["outcome"]["outcome"] == "selected"
            && message["result"]["outcome"]["optionId"] == "reject-once"
        {
            Ok(())
        } else if message["result"]["outcome"]["outcome"] == "selected"
            && message["result"]["outcome"]["optionId"] == "allow-once"
        {
            let id = state.prompt_id.take().ok_or_else(fixture_failure)?;
            Self::enqueue(
                state,
                json!({"jsonrpc": "2.0", "id": id, "result": {
                    "stopReason": "end_turn",
                    "usage": {
                        "inputTokens": 12,
                        "outputTokens": 4,
                        "cachedReadTokens": 3,
                        "cachedWriteTokens": 2,
                        "totalTokens": 21
                    }
                }}),
            );
            Ok(())
        } else {
            Err(fixture_failure())
        }
    }

    fn elicitation_response(
        &self,
        state: &mut AgentState,
        message: &Value,
    ) -> Result<(), RuntimeFailure> {
        if message["result"] != json!({"action": "accept", "content": {"question_0": "Panel"}}) {
            return Err(fixture_failure());
        }
        let id = state.prompt_id.take().ok_or_else(fixture_failure)?;
        Self::enqueue(
            state,
            json!({"jsonrpc": "2.0", "id": id, "result": {
                "stopReason": "end_turn",
                "usage": {
                    "inputTokens": 12,
                    "outputTokens": 4,
                    "cachedReadTokens": 3,
                    "cachedWriteTokens": 2,
                    "totalTokens": 21
                }
            }}),
        );
        Ok(())
    }

    fn cancel(&self, state: &mut AgentState) -> Result<(), RuntimeFailure> {
        if let Some(id) = state.prompt_id.take() {
            Self::enqueue(
                state,
                json!({"jsonrpc": "2.0", "id": id, "result": {
                    "stopReason": "cancelled",
                    "usage": {
                        "inputTokens": 0,
                        "outputTokens": 0,
                        "cachedReadTokens": 0,
                        "cachedWriteTokens": 0,
                        "totalTokens": 0
                    }
                }}),
            );
        }
        Ok(())
    }
}
