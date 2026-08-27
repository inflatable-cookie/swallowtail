use super::{AgentState, Scenario, SharedAgent, enqueue_session_metadata};
use serde_json::{Value, json};
use swallowtail_runtime::RuntimeFailure;

impl SharedAgent {
    pub(super) fn session_new(
        &self,
        state: &mut AgentState,
        id: Option<u64>,
    ) -> Result<(), RuntimeFailure> {
        match self.scenario {
            Scenario::AuthRequired => {
                Self::enqueue(
                    state,
                    json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "error": {"code": -32000, "message": "auth_required"}
                    }),
                );
                Ok(())
            }
            _ => {
                Self::enqueue(
                    state,
                    json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": session_new_result(self.scenario)
                    }),
                );
                enqueue_session_metadata(state);
                Ok(())
            }
        }
    }

    pub(super) fn set_config_option(
        &self,
        state: &mut AgentState,
        id: Option<u64>,
        message: &Value,
    ) -> Result<(), RuntimeFailure> {
        let Some(response) = plan_set_response(self.scenario) else {
            return Err(super::super::fixture_failure());
        };
        if message.pointer("/params/configId").and_then(Value::as_str) != Some("mode")
            || message.pointer("/params/value").and_then(Value::as_str) != Some("plan")
        {
            return Err(super::super::fixture_failure());
        }
        match response {
            PlanSetResponse::Disconnect => {
                state.stopped = true;
                Ok(())
            }
            PlanSetResponse::Rejected => {
                Self::enqueue(
                    state,
                    json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "error": {"code": -32602, "message": "fixture provider rejection"}
                    }),
                );
                Ok(())
            }
            PlanSetResponse::ConfirmationMissing => {
                Self::enqueue(
                    state,
                    json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {
                            "configOptions": [{
                                "id": "model",
                                "type": "select",
                                "category": "model",
                                "currentValue": "fixture-model",
                                "options": [{"value": "fixture-model", "name": "Fixture"}]
                            }]
                        }
                    }),
                );
                Ok(())
            }
            PlanSetResponse::ConfirmationAmbiguous => {
                Self::enqueue(
                    state,
                    json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {"configOptions": [mode_option("plan"), mode_option("plan")]}
                    }),
                );
                Ok(())
            }
            PlanSetResponse::ConfirmationMalformed => {
                let mut option = mode_option("plan");
                option["category"] = Value::String("unmapped_provider_category".to_owned());
                Self::enqueue(
                    state,
                    json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {"configOptions": [option]}
                    }),
                );
                Ok(())
            }
            PlanSetResponse::Drift => {
                Self::enqueue(
                    state,
                    json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {"configOptions": [mode_option("act")]}
                    }),
                );
                Ok(())
            }
            PlanSetResponse::Confirm => {
                Self::enqueue(
                    state,
                    json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {"configOptions": [mode_option("plan")]}
                    }),
                );
                Ok(())
            }
        }
    }
}

#[derive(Clone, Copy)]
enum PlanSetResponse {
    Confirm,
    Rejected,
    ConfirmationMissing,
    ConfirmationAmbiguous,
    ConfirmationMalformed,
    Drift,
    Disconnect,
}

fn plan_set_response(scenario: Scenario) -> Option<PlanSetResponse> {
    match scenario {
        Scenario::PlanRejected => Some(PlanSetResponse::Rejected),
        Scenario::PlanConfirmationMissing => Some(PlanSetResponse::ConfirmationMissing),
        Scenario::PlanConfirmationAmbiguous => Some(PlanSetResponse::ConfirmationAmbiguous),
        Scenario::PlanConfirmationMalformed => Some(PlanSetResponse::ConfirmationMalformed),
        Scenario::PlanDrift => Some(PlanSetResponse::Drift),
        Scenario::PlanDisconnect => Some(PlanSetResponse::Disconnect),
        Scenario::Success
        | Scenario::UnexpectedWrite
        | Scenario::Permission
        | Scenario::Cancellation
        | Scenario::Disconnect
        | Scenario::Oversized => Some(PlanSetResponse::Confirm),
        _ => None,
    }
}

fn session_new_result(scenario: Scenario) -> Value {
    let mut result = json!({
        "sessionId": "opaque-fixture-session",
        "modes": {
            "availableModes": [
                {"id": "plan", "name": "Plan"},
                {"id": "act", "name": "Act"}
            ],
            "currentModeId": "act"
        },
        "configOptions": [mode_option("act")]
    });
    match scenario {
        Scenario::PlanBlankSessionId => {
            result["sessionId"] = Value::String(String::new());
            result
        }
        Scenario::PlanMissingModes => {
            result
                .as_object_mut()
                .expect("session/new result is an object")
                .remove("modes");
            result
        }
        Scenario::PlanMissingConfig => {
            result
                .as_object_mut()
                .expect("session/new result is an object")
                .remove("configOptions");
            result
        }
        Scenario::PlanAmbiguousModes => {
            result["modes"]["availableModes"] = json!([
                {"id": "plan", "name": "Plan"},
                {"id": "plan", "name": "Plan Duplicate"},
                {"id": "act", "name": "Act"}
            ]);
            result
        }
        Scenario::PlanAmbiguousConfig => {
            result["configOptions"] = json!([mode_option("act"), mode_option("act")]);
            result
        }
        Scenario::PlanMalformedModes => {
            result["modes"]["availableModes"] = json!([
                {"id": "plan", "name": "Plan"},
                {"id": "yolo", "name": "Yolo"}
            ]);
            result
        }
        Scenario::PlanMalformedConfig => {
            let mut option = mode_option("act");
            option["category"] = Value::String("unmapped_provider_category".to_owned());
            result["configOptions"] = json!([option]);
            result
        }
        Scenario::PlanCurrentContradiction => {
            result["configOptions"] = json!([mode_option("plan")]);
            result
        }
        _ => result,
    }
}

fn mode_option(current: &str) -> Value {
    json!({
        "id": "mode",
        "name": "Mode",
        "category": "mode",
        "type": "select",
        "currentValue": current,
        "options": [
            {"value": "plan", "name": "Plan"},
            {"value": "act", "name": "Act"}
        ]
    })
}
