use super::Scenario;
use serde_json::{Value, json};

pub(super) fn effort_options(scenario: Scenario, selected: Option<&str>) -> Vec<Value> {
    let mut options = ["default", "low", "medium", "high", "xhigh", "max"]
        .into_iter()
        .map(|value| json!({"value": value, "name": value}))
        .collect::<Vec<_>>();
    let current = match (scenario, selected) {
        (Scenario::ReasoningMismatchAdvertised, Some("low")) => json!("high"),
        (Scenario::ReasoningMismatchUnadvertised, Some("low")) => {
            options.retain(|option| option["value"] != "high");
            json!("high")
        }
        (Scenario::ReasoningMismatchUnqualified, Some("low")) => {
            options.push(json!({"value": "ultra", "name": "ultra"}));
            json!("ultra")
        }
        (Scenario::ReasoningConfirmationMalformed, Some(_)) => json!(42),
        (Scenario::ReasoningConfirmationUnbounded, Some(_)) => {
            let value = "x".repeat(1024);
            options.push(json!({"value": value, "name": "unbounded"}));
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
        "options": options
    });
    match (scenario, selected) {
        (Scenario::ReasoningConfirmationMissing, Some(_)) => Vec::new(),
        (Scenario::ReasoningConfirmationDuplicate, Some(_)) => vec![effort.clone(), effort],
        _ => vec![effort],
    }
}
