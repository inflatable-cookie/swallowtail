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
        "negotiate_protocol" => output(
            state,
            json!({"id": id, "type": "response", "command": kind, "success": true, "data": {"protocolVersion": 2}}),
        ),
        "set_model" => output(
            state,
            json!({"id": id, "type": "response", "command": kind, "success": true, "data": {"provider": "fixture-provider", "id": "fixture-model"}}),
        ),
        "get_available_models" => {
            if matches!(scenario, Scenario::Hold) {
                return Ok(());
            }
            if matches!(scenario, Scenario::Disconnect) {
                state.stopped = true;
                return Ok(());
            }
            let response_command = if matches!(scenario, Scenario::ResponseMismatch) {
                "get_state"
            } else {
                "get_available_models"
            };
            let (success, data) = match scenario {
                Scenario::ProviderFailure => (
                    false,
                    json!({"error": "fixture private provider catalogue error"}),
                ),
                Scenario::Malformed => (true, json!({"models": "invalid"})),
                _ => (
                    true,
                    json!({
                        "models": [
                            {
                                "id": "fixture-model",
                                "name": "Fixture Model",
                                "api": "anthropic-messages",
                                "provider": "fixture-provider",
                                "baseUrl": "https://fixture-private.invalid",
                                "reasoning": true,
                                "input": ["text", "image"],
                                "contextWindow": 200000,
                                "maxTokens": 8192,
                                "cost": {"input": 1.0, "output": 2.0}
                            },
                            {
                                "id": "fixture-text-model",
                                "name": "Fixture Text Model",
                                "api": "openai-completions",
                                "provider": "fixture-provider",
                                "baseUrl": "https://fixture-private.invalid",
                                "reasoning": false,
                                "input": ["text"],
                                "contextWindow": 128000,
                                "maxTokens": 4096,
                                "cost": {"input": 0.0, "output": 0.0}
                            }
                        ]
                    }),
                ),
            };
            output(
                state,
                json!({
                    "id": id,
                    "type": "response",
                    "command": response_command,
                    "success": success,
                    "data": data
                }),
            );
        }
        "set_thinking_level" => {
            output(
                state,
                json!({"type": "thinking_level_changed", "thinkingLevel": "low"}),
            );
            output(
                state,
                json!({"id": id, "type": "response", "command": kind, "success": true}),
            );
        }
        "set_auto_retry"
        | "set_auto_compaction"
        | "set_steering_mode"
        | "set_follow_up_mode"
        | "set_interrupt_mode"
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
                json!({"id": id, "type": "response", "command": "get_state", "success": true, "data": {"model": {"id": "fixture-model", "provider": provider}, "thinkingLevel": "low", "isStreaming": false, "isCompacting": false, "steeringMode": "one-at-a-time", "followUpMode": "one-at-a-time", "interruptMode": "wait", "autoCompactionEnabled": false, "queuedMessageCount": 0}}),
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
            output(
                state,
                json!({"type": "extension_ui_request", "id": "ui-widget-clear", "method": "setWidget"}),
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
                Scenario::PromptUi => {
                    output(
                        state,
                        json!({"type": "extension_ui_request", "id": "ui-dialog-run", "method": "select", "title": "fixture title", "options": ["Allow", "Block"], "timeout": 10000}),
                    );
                    output(
                        state,
                        json!({"type": "extension_ui_request", "id": "ui-display-run", "method": "notify", "message": "fixture notice"}),
                    );
                }
                Scenario::RetryDrift => output(
                    state,
                    json!({"type": "auto_retry_start", "attempt": 1, "delayMs": 1}),
                ),
                Scenario::SummarizationRetryDrift => output(
                    state,
                    json!({"type": "summarization_retry_attempt_start", "source": "compaction", "reason": "threshold"}),
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
        json!({
            "type": "message_end",
            "message": {
                "role": "assistant",
                "content": [],
                "api": "anthropic-messages",
                "provider": "fixture-provider",
                "model": "fixture-model",
                "usage": {
                    "input": 12,
                    "output": 4,
                    "cacheRead": 3,
                    "cacheWrite": 2,
                    "cost": {
                        "input": 0.001,
                        "output": 0.002,
                        "cacheRead": 0.0001,
                        "cacheWrite": 0.0002,
                        "total": 0.0033
                    }
                },
                "stopReason": "stop",
                "timestamp": 1
            }
        }),
    );
    output(
        state,
        json!({
            "type": "message_end",
            "message": {
                "role": "assistant",
                "content": [],
                "api": "anthropic-messages",
                "provider": "fixture-provider",
                "model": "fixture-model",
                "usage": {
                    "input": 8,
                    "output": 6,
                    "cacheRead": 1,
                    "cacheWrite": 0,
                    "cost": {
                        "input": 0.001,
                        "output": 0.003,
                        "cacheRead": 0.0001,
                        "cacheWrite": 0.0,
                        "total": 0.0041
                    }
                },
                "stopReason": "stop",
                "timestamp": 2
            }
        }),
    );
    output(
        state,
        json!({"type": "agent_end", "messages": [], "isTerminal": true}),
    );
}

fn output(state: &mut ProcessState, value: Value) {
    let mut bytes = serde_json::to_vec(&value).expect("OhMyPi fixture JSON serializes");
    bytes.push(b'\n');
    state
        .output
        .push_back(ProcessOutputChunk::new(ProcessOutputStream::Stdout, bytes));
}
