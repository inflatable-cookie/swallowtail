impl SharedAgent {
    fn set_reasoning(
        &self,
        state: &mut AgentState,
        id: Option<u64>,
        message: &Value,
    ) -> Result<(), RuntimeFailure> {
        if !self.scenario.has_reasoning() {
            return Err(fixture_failure());
        }
        let result = match self.scenario {
            Scenario::ReasoningRejected => {
                json!({"jsonrpc": "2.0", "id": id, "error": {
                    "code": -32602, "message": "fixture provider rejection"
                }})
            }
            Scenario::ReasoningConfirmationMissing => Self::response(
                id,
                json!({"configOptions": [{"id": "model", "currentValue": "kimi-coder"}]}),
            ),
            Scenario::ReasoningDrift => confirmation(id, &["off", "low", "medium", "high"], "low"),
            Scenario::ReasoningLegacySuccess => {
                confirmation(id, &["off", "on"], requested_value(message)?)
            }
            Scenario::ReasoningEffortSuccess
            | Scenario::ReasoningEffort291Success
            | Scenario::ReasoningEffort292Success
            | Scenario::ReasoningEffort300Success
            | Scenario::ReasoningEffort310Success
            | Scenario::ReasoningEffort311Success
            | Scenario::ReasoningNewerSuccess => {
                let requested = requested_value(message)?;
                let effective = if requested == "on" {
                    "medium"
                } else {
                    requested
                };
                confirmation(id, &["off", "low", "medium", "high"], effective)
            }
            Scenario::ReasoningMissing
            | Scenario::ReasoningAmbiguous
            | Scenario::ReasoningMalformed
            | Scenario::ReasoningAlwaysThinking
            | Scenario::Complete
            | Scenario::HoldPrompt
            | Scenario::DisconnectPrompt => return Err(fixture_failure()),
        };
        Self::enqueue(state, result);
        Ok(())
    }
}

fn confirmation(id: Option<u64>, values: &[&str], current: &str) -> Value {
    SharedAgent::response(
        id,
        json!({"configOptions": [
            {"id": "model", "currentValue": "kimi-coder"},
            reasoning_option(values, current)
        ]}),
    )
}

fn requested_value(message: &Value) -> Result<&str, RuntimeFailure> {
    message["params"]["value"]
        .as_str()
        .ok_or_else(fixture_failure)
}

fn reasoning_option(values: &[&str], current: &str) -> Value {
    json!({
        "id": "thinking",
        "name": "Thinking",
        "category": "thought_level",
        "type": "select",
        "currentValue": current,
        "options": values
            .iter()
            .map(|value| json!({"value": value, "name": value}))
            .collect::<Vec<_>>()
    })
}
