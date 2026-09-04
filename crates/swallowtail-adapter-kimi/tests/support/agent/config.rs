impl SharedAgent {
    fn session_configuration(&self) -> Value {
        let mut options = vec![json!({
            "type": "select",
            "id": "model",
            "name": "Model",
            "category": "model",
            "currentValue": "kimi-coder",
            "options": [
                {"value": "kimi-coder", "name": "Kimi Coder"},
                {"value": "kimi-alternate", "name": "Kimi Alternate"}
            ]
        })];
        match self.scenario {
            Scenario::ReasoningLegacySuccess | Scenario::ReasoningRejected => {
                options.push(reasoning_option(&["off", "on"], "off"));
            }
            Scenario::ReasoningEffortSuccess
            | Scenario::ReasoningEffort291Success
            | Scenario::ReasoningEffort292Success
            | Scenario::ReasoningEffort300Success
            | Scenario::ReasoningEffort310Success
            | Scenario::ReasoningEffort311Success
            | Scenario::ReasoningConfirmationMissing
            | Scenario::ReasoningDrift => {
                options.push(reasoning_option(
                    &["off", "low", "medium", "high", "xhigh", "max"],
                    "off",
                ));
            }
            Scenario::ReasoningEffortExtended => {
                options.push(reasoning_option(
                    &["off", "low", "medium", "high", "xhigh", "max", "ultra"],
                    "off",
                ));
            }
            Scenario::ReasoningForeign => {
                options.push(reasoning_option(
                    &["off", "low", "medium", "high", "xhigh", "max", "ultra"],
                    "off",
                ));
            }
            Scenario::ReasoningUnbounded => {
                options.push(reasoning_option(
                    &["off", "low", "medium", "high", "xhigh", "max", long_value()],
                    "off",
                ));
            }
            Scenario::ReasoningEffortNarrow => {
                options.push(reasoning_option(&["off", "low", "medium", "high"], "off"));
            }
            Scenario::ReasoningAmbiguous => {
                let option =
                    reasoning_option(&["off", "low", "medium", "high", "xhigh", "max"], "off");
                options.push(option.clone());
                options.push(option);
            }
            Scenario::ReasoningMalformed => {
                let mut option =
                    reasoning_option(&["off", "low", "medium", "high", "xhigh", "max"], "off");
                option["category"] = Value::String("unmapped_provider_category".to_owned());
                options.push(option);
            }
            Scenario::ReasoningAlwaysThinking => {
                options.push(reasoning_option(
                    &["low", "medium", "high", "xhigh", "max"],
                    "medium",
                ));
            }
            Scenario::Complete
            | Scenario::HoldPrompt
            | Scenario::DisconnectPrompt
            | Scenario::CatalogueChanged
            | Scenario::CataloguePaginated
            | Scenario::CatalogueHold
            | Scenario::CatalogueDisconnect
            | Scenario::CatalogueUnsupported
            | Scenario::CleanupFailure
            | Scenario::ReasoningMissing
            | Scenario::PlanSuccess
            | Scenario::PlanLegacySuccess
            | Scenario::PlanCeilingSuccess
            | Scenario::PlanMissing
            | Scenario::PlanAmbiguous
            | Scenario::PlanMalformed
            | Scenario::PlanUnknownRow => {}
            Scenario::PlanConfirmationMissing
            | Scenario::PlanDrift
            | Scenario::PlanRejected
            | Scenario::PlanMissingAfterReasoning
            | Scenario::PlanMalformedAfterReasoning => {
                options.push(reasoning_option(
                    &["off", "low", "medium", "high", "xhigh", "max"],
                    "off",
                ));
            }
        }
        match self.scenario {
            Scenario::PlanMissing => {}
            Scenario::PlanAmbiguous => {
                let option = mode_option("default");
                options.push(option.clone());
                options.push(option);
            }
            Scenario::PlanMalformed => {
                let mut option = mode_option("default");
                option["category"] = Value::String("unmapped_provider_category".to_owned());
                options.push(option);
            }
            Scenario::PlanUnknownRow => {
                let mut option = mode_option("default");
                option["options"]
                    .as_array_mut()
                    .expect("mode option rows")
                    .push(json!({"value": "custom", "name": "Custom"}));
                options.push(option);
            }
            _ => options.push(mode_option("default")),
        }
        json!({"configOptions": options})
    }
}

fn long_value() -> &'static str {
    "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
}
