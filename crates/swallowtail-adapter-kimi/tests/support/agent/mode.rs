impl SharedAgent {
    fn set_config_option(
        &self,
        state: &mut AgentState,
        id: Option<u64>,
        message: &Value,
    ) -> Result<(), RuntimeFailure> {
        match message["params"]["configId"].as_str() {
            Some("thinking") => self.set_reasoning(state, id, message),
            Some("mode") => self.set_mode(state, id, message),
            _ => Err(fixture_failure()),
        }
    }

    fn set_mode(
        &self,
        state: &mut AgentState,
        id: Option<u64>,
        message: &Value,
    ) -> Result<(), RuntimeFailure> {
        if !self.scenario.has_plan() {
            return Err(fixture_failure());
        }
        let result = match self.scenario {
            Scenario::PlanRejected => json!({"jsonrpc": "2.0", "id": id, "error": {
                "code": -32602, "message": "fixture provider rejection"
            }}),
            Scenario::PlanConfirmationMissing => Self::response(
                id,
                json!({"configOptions": [{"id": "model", "currentValue": "kimi-coder"}]}),
            ),
            Scenario::PlanDrift => mode_confirmation(id, "default"),
            Scenario::PlanSuccess
            | Scenario::PlanLegacySuccess
            | Scenario::PlanCeilingSuccess
            | Scenario::ReasoningLegacySuccess
            | Scenario::ReasoningEffortSuccess
            | Scenario::ReasoningEffort291Success
            | Scenario::ReasoningEffort292Success
            | Scenario::ReasoningEffort300Success
            | Scenario::ReasoningEffort310Success
            | Scenario::ReasoningEffort311Success
            | Scenario::ReasoningEffortExtended => {
                mode_confirmation(id, requested_value(message)?)
            }
            _ => return Err(fixture_failure()),
        };
        Self::enqueue(state, result);
        Ok(())
    }
}

fn mode_confirmation(id: Option<u64>, current: &str) -> Value {
    SharedAgent::response(
        id,
        json!({"configOptions": [
            {"id": "model", "currentValue": "kimi-coder"},
            mode_option(current)
        ]}),
    )
}

fn malformed_mode_option() -> Value {
    let mut option = mode_option("default");
    option["category"] = Value::String("unmapped_provider_category".to_owned());
    option
}

fn mode_option(current: &str) -> Value {
    json!({
        "id": "mode",
        "name": "Mode",
        "category": "mode",
        "type": "select",
        "currentValue": current,
        "options": [
            {"value": "default", "name": "Default"},
            {"value": "plan", "name": "Plan"},
            {"value": "auto", "name": "Auto"},
            {"value": "yolo", "name": "YOLO"}
        ]
    })
}
