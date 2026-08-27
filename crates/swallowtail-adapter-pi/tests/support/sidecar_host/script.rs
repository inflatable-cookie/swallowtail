use super::{FIXTURE_SESSION_REF, ProcessState, SidecarScenario, fixture_failure};

mod continuity;

enum ThinkingPhase {
    Bootstrap,
    State,
}

fn thinking_level_field(
    scenario: SidecarScenario,
    phase: ThinkingPhase,
    expected: &str,
) -> Option<String> {
    match (scenario, phase) {
        (SidecarScenario::ThinkingBootstrapMismatch, ThinkingPhase::Bootstrap) => {
            Some("low".to_owned())
        }
        (SidecarScenario::ThinkingStateMismatch, ThinkingPhase::State) => Some("low".to_owned()),
        (SidecarScenario::ThinkingStateMissing, ThinkingPhase::State) => None,
        _ => Some(expected.to_owned()),
    }
}
use serde_json::{Value, json};
use swallowtail_runtime::{ProcessOutputChunk, ProcessOutputStream, RuntimeFailure};

pub(super) fn respond(
    scenario: SidecarScenario,
    command: &Value,
    state: &mut ProcessState,
) -> Result<(), RuntimeFailure> {
    let kind = command
        .get("command")
        .and_then(Value::as_str)
        .ok_or_else(fixture_failure)?;
    let id = command.get("id").and_then(Value::as_str);
    let params = command.get("params").cloned().unwrap_or_else(|| json!({}));
    match kind {
        "bootstrap" => {
            if params.get("catalogueOnly").and_then(Value::as_bool) == Some(true) {
                output(
                    state,
                    json!({
                        "type": "response",
                        "id": id,
                        "command": "bootstrap",
                        "success": true,
                        "data": catalogue_identity()
                    }),
                );
                return Ok(());
            }
            let cwd = params
                .get("cwd")
                .and_then(Value::as_str)
                .ok_or_else(fixture_failure)?
                .to_owned();
            let provider = params
                .get("provider")
                .and_then(Value::as_str)
                .ok_or_else(fixture_failure)?
                .to_owned();
            let model = params
                .get("model")
                .and_then(Value::as_str)
                .ok_or_else(fixture_failure)?
                .to_owned();
            state.bootstrap = Some((cwd.clone(), provider.clone(), model.clone()));
            state.session_ref = Some(FIXTURE_SESSION_REF.to_owned());
            state.thinking_level = params
                .get("thinkingLevel")
                .and_then(Value::as_str)
                .map(str::to_owned);
            let (sdk_version, node_version) =
                if matches!(scenario, SidecarScenario::BootstrapVersionMismatch) {
                    ("0.84.1", "22.23.1")
                } else {
                    ("0.84.2", "22.23.2")
                };
            let effective_cwd = if matches!(scenario, SidecarScenario::BootstrapCwdMismatch) {
                "/fixture/other-workspace"
            } else {
                cwd.as_str()
            };
            let mut data = json!({
                "wire": "swallowtail-pi-sdk-jsonl-v1",
                "behavior": "pi.sdk-sidecar-v1",
                "sdkPackage": "@earendil-works/pi-coding-agent",
                "sdkVersion": sdk_version,
                "nodeVersion": node_version,
                "provider": provider,
                "model": model,
                "cwd": effective_cwd,
                "idle": true,
                "streaming": false,
                "messages": 0,
                "sessionId": "00000000-0000-0000-0000-000000000000",
                "sessionRef": FIXTURE_SESSION_REF,
                "tools": ["read", "grep", "find", "ls"]
            });
            if let Some(thinking_level) = state.thinking_level.as_ref()
                && let Some(reported) =
                    thinking_level_field(scenario, ThinkingPhase::Bootstrap, thinking_level)
            {
                data["thinkingLevel"] = json!(reported);
            }
            output(
                state,
                json!({
                    "type": "response",
                    "id": id,
                    "command": "bootstrap",
                    "success": true,
                    "data": data
                }),
            );
        }
        "state" => {
            let (cwd, provider, model) = state.bootstrap.clone().ok_or_else(fixture_failure)?;
            let session_ref = state.session_ref.clone().ok_or_else(fixture_failure)?;
            let provider = if matches!(scenario, SidecarScenario::StateMismatch) {
                "wrong-provider"
            } else {
                provider.as_str()
            };
            let mut data = json!({
                "cwd": cwd,
                "provider": provider,
                "model": model,
                "idle": true,
                "streaming": false,
                "messages": 0,
                "sessionId": "00000000-0000-0000-0000-000000000000",
                "sessionRef": session_ref,
                "tools": ["read", "grep", "find", "ls"]
            });
            if let Some(thinking_level) = state.thinking_level.as_ref()
                && let Some(reported) =
                    thinking_level_field(scenario, ThinkingPhase::State, thinking_level)
            {
                data["thinkingLevel"] = json!(reported);
            }
            output(
                state,
                json!({
                    "type": "response",
                    "id": id,
                    "command": "state",
                    "success": true,
                    "data": data
                }),
            );
        }
        "session_switch" => {
            continuity::session_switch(scenario, id, &params, state)?;
        }
        "session_replay" => {
            continuity::session_replay(scenario, id, state);
        }
        "prompt" => {
            let response_command = if matches!(scenario, SidecarScenario::ResponseMismatch) {
                "state"
            } else {
                "prompt"
            };
            output(
                state,
                json!({
                    "type": "response",
                    "id": id,
                    "command": response_command,
                    "success": true,
                    "data": {"accepted": true}
                }),
            );
            if matches!(scenario, SidecarScenario::ResponseMismatch) {
                return Ok(());
            }
            output(state, json!({"type": "event", "event": "agent_start"}));
            match scenario {
                SidecarScenario::Complete => {
                    output(
                        state,
                        json!({"type": "event", "event": "output_delta", "delta": "fixture answer"}),
                    );
                    settled(state, "stop");
                }
                SidecarScenario::Disconnect => state.stopped = true,
                SidecarScenario::Malformed => raw(state, b"{not-json}\n"),
                SidecarScenario::UnknownEvent => {
                    output(state, json!({"type": "event", "event": "mystery_event"}));
                }
                SidecarScenario::TerminalRecord => {
                    output(
                        state,
                        json!({
                            "type": "terminal",
                            "failure": {
                                "code": "internal_error",
                                "message": "sidecar terminated: internal_error"
                            }
                        }),
                    );
                    state.stopped = true;
                }
                SidecarScenario::ProviderFailure => {
                    output(
                        state,
                        json!({
                            "type": "event",
                            "event": "message_end",
                            "role": "assistant",
                            "stopReason": "error"
                        }),
                    );
                }
                SidecarScenario::Hold
                | SidecarScenario::ResponseMismatch
                | SidecarScenario::BootstrapCwdMismatch
                | SidecarScenario::BootstrapVersionMismatch
                | SidecarScenario::StateMismatch
                | SidecarScenario::SessionNotFound
                | SidecarScenario::SessionSubstituted
                | SidecarScenario::SwitchCwdMismatch
                | SidecarScenario::ReplayFailure
                | SidecarScenario::ReplaySequenceGap
                | SidecarScenario::ReplayCountMismatch
                | SidecarScenario::ReplayOverflow
                | SidecarScenario::ReplayAfterResponse
                | SidecarScenario::ReplayDuringResume
                | SidecarScenario::HoldReplay
                | SidecarScenario::ThinkingBootstrapMismatch
                | SidecarScenario::ThinkingStateMismatch
                | SidecarScenario::ThinkingStateMissing => {}
            }
        }
        "steer" => {
            output(
                state,
                json!({"type": "response", "id": id, "command": "steer", "success": true, "data": {}}),
            );
        }
        "follow_up" => {
            output(
                state,
                json!({"type": "response", "id": id, "command": "follow_up", "success": true, "data": {}}),
            );
            output(
                state,
                json!({"type": "event", "event": "output_delta", "delta": "fixture answer"}),
            );
            settled(state, "stop");
        }
        "abort" => {
            output(
                state,
                json!({"type": "response", "id": id, "command": "abort", "success": true, "data": {}}),
            );
            settled(state, "aborted");
        }
        "close" => {
            output(
                state,
                json!({"type": "response", "id": id, "command": "close", "success": true, "data": {}}),
            );
            state.stopped = true;
        }
        _ => return Err(fixture_failure()),
    }
    Ok(())
}

fn catalogue_identity() -> Value {
    json!({
        "wire": "swallowtail-pi-sdk-jsonl-v1",
        "behavior": "pi.sdk-sidecar-v1",
        "sdkPackage": "@earendil-works/pi-coding-agent",
        "sdkVersion": "0.84.2",
        "nodeVersion": "22.23.2",
        "models": [
            {"provider": "fixture-provider", "id": "fixture-model"},
            {"provider": "fixture-provider", "id": "fixture-text-model"}
        ]
    })
}

fn settled(state: &mut ProcessState, stop_reason: &str) {
    if stop_reason == "stop" {
        output(
            state,
            json!({
                "type": "event",
                "event": "message_end",
                "role": "assistant",
                "stopReason": "stop",
                "usage": {"input": 12, "output": 4, "cacheRead": 3, "cacheWrite": 2}
            }),
        );
    } else {
        output(
            state,
            json!({
                "type": "event",
                "event": "message_end",
                "role": "assistant",
                "stopReason": stop_reason
            }),
        );
    }
    output(state, json!({"type": "event", "event": "turn_end"}));
    output(state, json!({"type": "event", "event": "agent_end"}));
    output(state, json!({"type": "event", "event": "agent_settled"}));
}

fn output(state: &mut ProcessState, value: Value) {
    let mut bytes = serde_json::to_vec(&value).expect("sidecar fixture JSON serializes");
    bytes.push(b'\n');
    raw(state, &bytes);
}

fn raw(state: &mut ProcessState, bytes: &[u8]) {
    state.output.push_back(ProcessOutputChunk::new(
        ProcessOutputStream::Stdout,
        bytes.to_vec(),
    ));
}
