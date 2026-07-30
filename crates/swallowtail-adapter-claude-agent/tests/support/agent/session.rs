impl SharedAgent {
    pub(super) fn new(scenario: Scenario, version: &str) -> Arc<Self> {
        let mut state = AgentState::default();
        if scenario == Scenario::Version {
            state.output.push_back(ProcessOutputChunk::new(
                ProcessOutputStream::Stdout,
                format!("{version}\n").into_bytes(),
            ));
        }
        Arc::new(Self {
            state: Mutex::new(state),
            changed: Condvar::new(),
            scenario,
            version: version.to_owned(),
        })
    }

    fn enqueue(state: &mut AgentState, message: Value) {
        let mut bytes = serde_json::to_vec(&message).expect("fixture message serializes");
        bytes.push(b'\n');
        state
            .output
            .push_back(ProcessOutputChunk::new(ProcessOutputStream::Stdout, bytes));
    }

    fn handle_write(&self, chunk: ProcessInputChunk) -> Result<(), RuntimeFailure> {
        let message: Value =
            serde_json::from_slice(chunk.bytes()).map_err(|_| fixture_failure())?;
        let mut state = self.state.lock().expect("fixture agent lock poisoned");
        state.writes.push(message.clone());
        let id = message.get("id").and_then(Value::as_u64);
        match message.get("method").and_then(Value::as_str) {
            Some("initialize") => self.initialize(&mut state, id),
            Some("session/new") => self.new_session(&mut state, id, &message),
            Some("session/load") => self.attach_session(&mut state, id, &message, true),
            Some("session/resume") => self.attach_session(&mut state, id, &message, false),
            Some("session/set_mode") => self.set_mode(&mut state, id, &message),
            Some("session/set_config_option") => self.set_config(&mut state, id, &message),
            Some("session/prompt") => self.prompt(&mut state, id),
            Some("session/cancel") => self.cancel(&mut state),
            Some("session/close") => self.close_session(&mut state, id),
            Some("session/delete") => self.delete_session(&mut state, id),
            None if id == Some(701) => self.complete_read(&mut state, &message),
            None if id == Some(900) => self.permission_response(&mut state, &message),
            None if id == Some(901) => self.elicitation_response(&mut state, &message),
            _ => return Err(fixture_failure()),
        }?;
        self.changed.notify_all();
        Ok(())
    }

    fn initialize(&self, state: &mut AgentState, id: Option<u64>) -> Result<(), RuntimeFailure> {
        let version = semver::Version::parse(&self.version).map_err(|_| fixture_failure())?;
        let mut capabilities = json!({
            "_meta": {"claudeCode": {"promptQueueing": true}},
            "promptCapabilities": {"image": true, "embeddedContext": true},
            "loadSession": true,
            "sessionCapabilities": {"close": {}, "delete": {}, "resume": {}}
        });
        if self.scenario == Scenario::LifecycleDrift {
            capabilities["sessionCapabilities"] = json!({"close": {}, "resume": {}});
        }
        if version >= semver::Version::new(0, 60, 0) {
            capabilities["providers"] = json!({});
        }
        let steering = (version >= semver::Version::new(0, 61, 0))
            .then(|| json!({"steering": {"supported": true}}));
        let auth_methods = if self.scenario == Scenario::AuthDrift {
            json!([{"id": "claude-login", "name": "Claude login"}])
        } else {
            json!([])
        };
        Self::enqueue(
            state,
            json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "protocolVersion": 1,
                    "agentCapabilities": capabilities,
                    "agentInfo": {
                        "name": "@agentclientprotocol/claude-agent-acp",
                        "version": self.version
                    },
                    "authMethods": auth_methods,
                    "_meta": steering
                }
            }),
        );
        Ok(())
    }

    fn new_session(
        &self,
        state: &mut AgentState,
        id: Option<u64>,
        message: &Value,
    ) -> Result<(), RuntimeFailure> {
        let tools = &message["params"]["_meta"]["claudeCode"]["options"]["tools"];
        if tools != &json!(["Read", "Glob", "Grep"])
            && tools != &json!(["Read", "Glob", "Grep", "Edit", "Write"])
        {
            return Err(fixture_failure());
        }
        let requested_model =
            message["params"]["_meta"]["claudeCode"]["options"]["settings"]["model"]
                .as_str()
                .ok_or_else(fixture_failure)?
                .to_owned();
        state.requested_model = Some(requested_model.clone());
        let version = semver::Version::parse(&self.version).map_err(|_| fixture_failure())?;
        let current_model =
            if version >= semver::Version::new(0, 54, 0) || self.scenario == Scenario::ModelDrift {
                "default".to_owned()
            } else {
                requested_model
            };
        state.current_model = Some(current_model);
        let config_options = self.config_options(state)?;
        Self::enqueue(
            state,
            json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "sessionId": "claude-agent-session-fixture",
                    "modes": {
                        "currentModeId": "default",
                        "availableModes": [
                            {"id": "default", "name": "Default"},
                            {"id": "acceptEdits", "name": "Accept Edits"}
                        ]
                    },
                    "configOptions": config_options
                }
            }),
        );
        Self::enqueue(
            state,
            json!({"jsonrpc": "2.0", "method": "session/update", "params": {
                "sessionId": "claude-agent-session-fixture",
                "update": {"sessionUpdate": "available_commands_update", "availableCommands": []}
            }}),
        );
        Self::enqueue(
            state,
            json!({"jsonrpc": "2.0", "method": "session/update", "params": {
                "sessionId": "claude-agent-session-fixture",
                "update": {
                    "sessionUpdate": "config_option_update",
                    "configOptions": config_options
                }
            }}),
        );
        Self::enqueue(
            state,
            json!({"jsonrpc": "2.0", "method": "session/update", "params": {
                "sessionId": "claude-agent-session-fixture",
                "update": {
                    "sessionUpdate": "current_mode_update",
                    "currentModeId": "default"
                }
            }}),
        );
        Ok(())
    }

    fn attach_session(
        &self,
        state: &mut AgentState,
        id: Option<u64>,
        message: &Value,
        load: bool,
    ) -> Result<(), RuntimeFailure> {
        if message["params"]["sessionId"] != "claude-agent-session-fixture"
            || message["params"]["mcpServers"] != json!([])
        {
            return Err(fixture_failure());
        }
        state.requested_model = Some("claude-sonnet-4-6".to_owned());
        state.current_model = Some("claude-sonnet-4-6".to_owned());
        let config_options = self.config_options(state)?;
        if load {
            for update in [
                json!({"sessionUpdate":"user_message_chunk","content":{"type":"text","text":"Earlier question."}}),
                json!({"sessionUpdate":"agent_message_chunk","content":{"type":"text","text":"Earlier answer."}}),
            ] {
                Self::enqueue(
                    state,
                    json!({"jsonrpc":"2.0","method":"session/update","params":{
                        "sessionId":"claude-agent-session-fixture",
                        "update":update
                    }}),
                );
            }
        }
        Self::enqueue(
            state,
            json!({"jsonrpc":"2.0","id":id,"result":{
                "sessionId":"claude-agent-session-fixture",
                "configOptions":config_options
            }}),
        );
        Self::enqueue(
            state,
            json!({"jsonrpc":"2.0","method":"session/update","params":{
                "sessionId":"claude-agent-session-fixture",
                "update":{"sessionUpdate":"available_commands_update","availableCommands":[]}
            }}),
        );
        Ok(())
    }

    fn set_mode(
        &self,
        state: &mut AgentState,
        id: Option<u64>,
        message: &Value,
    ) -> Result<(), RuntimeFailure> {
        if message["params"]["sessionId"] != "claude-agent-session-fixture"
            || message["params"]["modeId"] != "acceptEdits"
        {
            return Err(fixture_failure());
        }
        Self::enqueue(
            state,
            json!({"jsonrpc": "2.0", "method": "session/update", "params": {
                "sessionId": "claude-agent-session-fixture",
                "update": {"sessionUpdate": "current_mode_update", "currentModeId": "acceptEdits"}
            }}),
        );
        Self::enqueue(state, json!({"jsonrpc": "2.0", "id": id, "result": {}}));
        Ok(())
    }

    fn set_config(
        &self,
        state: &mut AgentState,
        id: Option<u64>,
        message: &Value,
    ) -> Result<(), RuntimeFailure> {
        let version = semver::Version::parse(&self.version).map_err(|_| fixture_failure())?;
        if version < semver::Version::new(0, 54, 0)
            || message["params"]["sessionId"] != "claude-agent-session-fixture"
        {
            return Err(fixture_failure());
        }
        let value = message["params"]["value"]
            .as_str()
            .ok_or_else(fixture_failure)?
            .to_owned();
        match message["params"]["configId"].as_str() {
            Some("model") => {
                if Some(value.as_str()) != state.requested_model.as_deref() {
                    return Err(fixture_failure());
                }
                state.current_model = Some(if self.scenario == Scenario::ModelDrift {
                    "default".to_owned()
                } else {
                    value
                });
            }
            Some("effort")
                if matches!(
                    value.as_str(),
                    "default" | "low" | "medium" | "high" | "xhigh" | "max"
                ) =>
            {
                state.effort = Some(value);
            }
            Some("mode") if value == "plan" => {
                state.mode = Some(value);
            }
            _ => return Err(fixture_failure()),
        }
        let config_options = self.config_options(state)?;
        Self::enqueue(
            state,
            json!({"jsonrpc": "2.0", "method": "session/update", "params": {
                "sessionId": "claude-agent-session-fixture",
                "update": {
                    "sessionUpdate": "config_option_update",
                    "configOptions": config_options
                }
            }}),
        );
        Self::enqueue(
            state,
            json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {"configOptions": config_options}
            }),
        );
        Ok(())
    }

}
