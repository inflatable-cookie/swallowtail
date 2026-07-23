use super::{ProcessState, Scenario, fixture_failure};
use serde_json::{Value, json};
use swallowtail_runtime::{ProcessOutputChunk, ProcessOutputStream, RuntimeFailure};

pub(super) fn respond(
    scenario: Scenario,
    command: &Value,
    state: &mut ProcessState,
) -> Result<(), RuntimeFailure> {
    let kind = command
        .get("type")
        .and_then(Value::as_str)
        .ok_or_else(fixture_failure)?;
    let id = command.get("id").and_then(Value::as_str);
    match kind {
        "set_auto_retry"
        | "set_auto_compaction"
        | "set_steering_mode"
        | "set_follow_up_mode"
        | "steer"
        | "follow_up" => {
            output(
                state,
                json!({"id": id, "type": "response", "command": kind, "success": true}),
            );
            if kind == "follow_up" {
                output(
                    state,
                    json!({"type": "message_update", "message": {}, "assistantMessageEvent": {"type": "text_delta", "contentIndex": 0, "delta": "fixture answer", "partial": {}}}),
                );
                output(
                    state,
                    json!({"type": "extension_ui_request", "id": "ui-dialog-1", "method": "select", "title": "fixture title", "options": ["Allow", "Block"], "timeout": 10000}),
                );
                output(
                    state,
                    json!({"type": "extension_ui_request", "id": "ui-display-1", "method": "notify", "message": "fixture notice"}),
                );
            }
        }
        "get_state" => {
            let provider = if matches!(scenario, Scenario::StateMismatch) {
                "wrong-provider"
            } else {
                "fixture-provider"
            };
            output(
                state,
                json!({"id": id, "type": "response", "command": "get_state", "success": true, "data": {"model": {"id": "fixture-model", "provider": provider}, "isStreaming": false, "isCompacting": false, "steeringMode": "one-at-a-time", "followUpMode": "one-at-a-time", "autoCompactionEnabled": false, "pendingMessageCount": 0}}),
            );
        }
        "prompt" => {
            let response_command = if matches!(scenario, Scenario::ResponseMismatch) {
                "follow_up"
            } else {
                "prompt"
            };
            output(
                state,
                json!({"id": id, "type": "response", "command": response_command, "success": true}),
            );
            output(state, json!({"type": "agent_start"}));
            match scenario {
                Scenario::Complete => {
                    output(
                        state,
                        json!({"type": "message_update", "message": {}, "assistantMessageEvent": {"type": "text_delta", "contentIndex": 0, "delta": "fixture answer", "partial": {}}}),
                    );
                    settled(state);
                }
                Scenario::Disconnect => state.stopped = true,
                Scenario::Malformed => state.output.push_back(ProcessOutputChunk::new(
                    ProcessOutputStream::Stdout,
                    b"{not-json}\n".to_vec(),
                )),
                Scenario::ProviderFailure => output(
                    state,
                    json!({"type": "extension_error", "message": "fixture provider secret"}),
                ),
                Scenario::RetryDrift => output(
                    state,
                    json!({"type": "auto_retry_start", "attempt": 1, "delayMs": 1}),
                ),
                Scenario::Hold | Scenario::ResponseMismatch | Scenario::StateMismatch => {}
            }
        }
        "extension_ui_response" => {
            if command.get("cancelled").and_then(Value::as_bool) != Some(true) {
                settled(state);
            }
        }
        "abort" => {
            output(
                state,
                json!({"id": id, "type": "response", "command": "abort", "success": true}),
            );
            settled(state);
        }
        _ => return Err(fixture_failure()),
    }
    Ok(())
}

fn settled(state: &mut ProcessState) {
    output(
        state,
        json!({"type": "agent_end", "messages": [], "willRetry": false}),
    );
    output(state, json!({"type": "agent_settled"}));
}

fn output(state: &mut ProcessState, value: Value) {
    let mut bytes = serde_json::to_vec(&value).expect("Pi fixture JSON serializes");
    bytes.push(b'\n');
    state
        .output
        .push_back(ProcessOutputChunk::new(ProcessOutputStream::Stdout, bytes));
}
