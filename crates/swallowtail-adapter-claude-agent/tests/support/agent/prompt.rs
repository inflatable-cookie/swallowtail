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
            options.extend(reasoning::effort_options(
                self.scenario,
                state.effort.as_deref(),
            ));
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

}
