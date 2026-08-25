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
            Scenario::ReasoningDrift => confirmation(
                id,
                &["off", "low", "medium", "high", "xhigh", "max"],
                "low",
            ),
            Scenario::ReasoningLegacySuccess => {
                confirmation(id, &["off", "on"], requested_value(message)?)
            }
            Scenario::ReasoningEffortSuccess
            | Scenario::ReasoningEffort291Success
            | Scenario::ReasoningEffort292Success
            | Scenario::ReasoningEffort300Success
            | Scenario::ReasoningEffort310Success
            | Scenario::ReasoningEffort311Success
            | Scenario::ReasoningNewerSuccess
            | Scenario::PlanConfirmationMissing
            | Scenario::PlanDrift
            | Scenario::PlanRejected => {
                confirmation(
                    id,
                    &["off", "low", "medium", "high", "xhigh", "max"],
                    requested_effort(message)?,
                )
            }
            Scenario::PlanMissingAfterReasoning => confirmation_without_mode(
                id,
                &["off", "low", "medium", "high", "xhigh", "max"],
                requested_effort(message)?,
            ),
            Scenario::PlanMalformedAfterReasoning => confirmation_with_malformed_mode(
                id,
                &["off", "low", "medium", "high", "xhigh", "max"],
                requested_effort(message)?,
            ),
            Scenario::ReasoningEffortExtended => {
                let requested = requested_value(message)?;
                confirmation(
                    id,
                    &["off", "low", "medium", "high", "xhigh", "max", "ultra"],
                    requested,
                )
            }
            Scenario::ReasoningEffortNarrow => {
                let requested = requested_value(message)?;
                confirmation(id, &["off", "low", "medium", "high"], requested)
            }
            Scenario::ReasoningMissing
            | Scenario::ReasoningAmbiguous
            | Scenario::ReasoningMalformed
            | Scenario::ReasoningAlwaysThinking
            | Scenario::Complete
            | Scenario::HoldPrompt
            | Scenario::DisconnectPrompt
            | Scenario::CatalogueChanged
            | Scenario::CataloguePaginated
            | Scenario::CatalogueHold
            | Scenario::CatalogueDisconnect
            | Scenario::CatalogueUnsupported
            | Scenario::CleanupFailure
            | Scenario::PlanSuccess
            | Scenario::PlanLegacySuccess
            | Scenario::PlanCeilingSuccess
            | Scenario::PlanNewerSuccess
            | Scenario::PlanMissing
            | Scenario::PlanAmbiguous
            | Scenario::PlanMalformed
            | Scenario::PlanUnknownRow => return Err(fixture_failure()),
        };
        Self::enqueue(state, result);
        Ok(())
    }
}

fn confirmation(id: Option<u64>, values: &[&str], current: &str) -> Value {
    confirmation_with(id, values, current, Some(mode_option("default")))
}

fn confirmation_without_mode(id: Option<u64>, values: &[&str], current: &str) -> Value {
    confirmation_with(id, values, current, None)
}

fn confirmation_with_malformed_mode(id: Option<u64>, values: &[&str], current: &str) -> Value {
    confirmation_with(id, values, current, Some(malformed_mode_option()))
}

fn confirmation_with(
    id: Option<u64>,
    values: &[&str],
    current: &str,
    mode: Option<Value>,
) -> Value {
    let mut options = vec![
        json!({"id": "model", "currentValue": "kimi-coder"}),
        reasoning_option(values, current),
    ];
    if let Some(mode) = mode {
        options.push(mode);
    }
    SharedAgent::response(id, json!({"configOptions": options}))
}

fn requested_value(message: &Value) -> Result<&str, RuntimeFailure> {
    message["params"]["value"]
        .as_str()
        .ok_or_else(fixture_failure)
}

fn requested_effort(message: &Value) -> Result<&str, RuntimeFailure> {
    let requested = requested_value(message)?;
    Ok(if requested == "on" { "medium" } else { requested })
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
