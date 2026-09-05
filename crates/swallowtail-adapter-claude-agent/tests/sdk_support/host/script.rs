use super::super::host::{FIXTURE_CWD, FIXTURE_MODEL, ProcessState, SdkScenario};
use serde_json::{Value, json};
use swallowtail_runtime::{ProcessOutputChunk, ProcessOutputStream, RuntimeFailure};

pub(super) fn push(state: &mut ProcessState, record: Value) {
    let mut bytes = serde_json::to_vec(&record).expect("fixture record serializes");
    bytes.push(b'\n');
    state
        .output
        .push_back(ProcessOutputChunk::new(ProcessOutputStream::Stdout, bytes));
}

fn push_raw(state: &mut ProcessState, bytes: &[u8]) {
    state.output.push_back(ProcessOutputChunk::new(
        ProcessOutputStream::Stdout,
        bytes.to_vec(),
    ));
}

fn push_stderr(state: &mut ProcessState, bytes: &[u8]) {
    state.output.push_back(ProcessOutputChunk::new(
        ProcessOutputStream::Stderr,
        bytes.to_vec(),
    ));
}

fn complete_turn_record() -> Value {
    let mut result_field_presence = serde_json::Map::new();
    for field in super::super::capture::SDK_RESULT_FIELD_NAMES {
        result_field_presence.insert(
            (*field).to_owned(),
            json!(matches!(
                *field,
                "type" | "subtype" | "duration_ms" | "is_error" | "num_turns"
            )),
        );
    }
    json!({
        "type": "event",
        "event": "turn_ended",
        "subtype": "success",
        "stopReason": "success",
        "isError": false,
        "numTurns": 1,
        "durationMs": 7,
        "errorTextPresent": false,
        "errorTextType": "absent",
        "resultFieldPresence": result_field_presence,
    })
}

pub(super) fn respond(
    scenario: SdkScenario,
    command: &Value,
    state: &mut ProcessState,
) -> Result<(), RuntimeFailure> {
    let Some(kind) = command.get("type").and_then(Value::as_str) else {
        return Err(super::super::host::fixture_failure());
    };
    if kind == "callback_response" {
        return Ok(());
    }
    let id = command["id"].as_str().unwrap_or_default().to_owned();
    match command["command"].as_str().unwrap_or_default() {
        "open" => open(scenario, state, &id, &command["params"]),
        "query" => query(scenario, state, &id),
        "interrupt" => interrupt(scenario, state, &id),
        "set_permission_mode" => set_permission_mode(scenario, state, &id, &command["params"]),
        "close" => close(scenario, state, &id),
        _ => return Err(super::super::host::fixture_failure()),
    }
    Ok(())
}

fn open(scenario: SdkScenario, state: &mut ProcessState, id: &str, params: &Value) {
    state.opened = true;
    if matches!(scenario, SdkScenario::OpenHold) {
        // No response and no exit: only the host deadline can end this.
        return;
    }
    if matches!(scenario, SdkScenario::OpenRejected) {
        push(
            state,
            json!({"type": "response", "id": id, "command": "open", "success": false,
                   "failure": {"code": "construction_failed",
                               "message": "sidecar command failed: construction_failed"}}),
        );
        return;
    }
    let mut data = json!({
        "wire": "swallowtail-claude-agent-sdk-jsonl-v1",
        "behavior": "claude-agent.sdk-v1",
        "sdkPackage": "@anthropic-ai/claude-agent-sdk",
        "sdkVersion": "0.3.259",
        "nativeVersion": "2.1.259",
        "nodeVersion": "22.23.2",
        "cwd": FIXTURE_CWD,
        "requestedModel": FIXTURE_MODEL,
        "supportedModels": [FIXTURE_MODEL],
        "readiness": "requested-with-supported-list",
        "capabilities": [],
        "account": {
            "apiProvider": "firstParty",
            "subscriptionTypePresent": true,
            "tokenSourcePresent": false,
            "apiKeySourcePresent": false
        },
        // The fixture sidecar echoes exactly what the driver admitted, so a
        // widened or reordered echo is a deliberate scenario rather than an
        // accident of the fixture.
        "tools": params["tools"].clone(),
        "permissionMode": params["permissionMode"].clone()
    });
    match scenario {
        SdkScenario::AccountNotSubscription => {
            data["account"]
                .as_object_mut()
                .expect("account object")
                .insert("subscriptionTypePresent".to_owned(), json!(false));
        }
        SdkScenario::AccountNotFirstParty => data["account"]["apiProvider"] = json!("bedrock"),
        SdkScenario::AccountIdentityLeak => {
            data["account"]["email"] = json!("person@example.test");
        }
        SdkScenario::IdentityMismatch => data["sdkVersion"] = json!("0.3.258"),
        SdkScenario::CwdMismatch => data["cwd"] = json!("/fixture/elsewhere"),
        SdkScenario::CanonicalModel => {
            data["supportedModels"] = json!([FIXTURE_MODEL, "claude-sonnet-5-20250929"]);
        }
        SdkScenario::MissingModel | SdkScenario::UnsupportedModel => {}
        SdkScenario::EmptySupportedModels => data["supportedModels"] = json!([]),
        SdkScenario::NewerNode => data["nodeVersion"] = json!("26.7.0"),
        SdkScenario::ToolsWidened => data["tools"] = json!(["Read", "Glob", "Grep", "Bash"]),
        SdkScenario::PermissionModeDrift => data["permissionMode"] = json!("plan"),
        SdkScenario::UnadvertisedInterruptReceipt => data["capabilities"] = json!([]),
        _ => {}
    }
    push(
        state,
        json!({"type": "response", "id": id, "command": "open", "success": true, "data": data}),
    );
}

fn query(scenario: SdkScenario, state: &mut ProcessState, id: &str) {
    if matches!(scenario, SdkScenario::QueryHold) {
        // No response and no exit: only the caller's turn deadline ends this.
        return;
    }
    if matches!(scenario, SdkScenario::QueryRejected) {
        push(
            state,
            json!({"type": "response", "id": id, "command": "query", "success": false,
                   "failure": {"code": "turn_active",
                               "message": "sidecar command failed: turn_active"}}),
        );
        return;
    }
    if matches!(scenario, SdkScenario::InitMissing) {
        push(
            state,
            json!({"type": "response", "id": id, "command": "query", "success": false,
                   "failure": {"code": "init_missing",
                               "message": "sidecar command failed: init_missing"}}),
        );
        return;
    }
    if matches!(scenario, SdkScenario::InitializationFailed) {
        push(
            state,
            json!({"type": "response", "id": id, "command": "query", "success": false,
                   "failure": {"code": "initialization_failed",
                               "message": "sidecar command failed: initialization_failed"}}),
        );
        return;
    }
    if matches!(scenario, SdkScenario::MissingModel) {
        push(
            state,
            json!({"type": "response", "id": id, "command": "query", "success": false,
                   "failure": {"code": "model_missing",
                               "message": "sidecar command failed: model_missing"}}),
        );
        return;
    }
    if matches!(scenario, SdkScenario::UnsupportedModel) {
        push(
            state,
            json!({"type": "response", "id": id, "command": "query", "success": false,
                   "failure": {"code": "supported_model_rejected",
                               "message": "sidecar command failed: supported_model_rejected"}}),
        );
        return;
    }
    let effective_model = if scenario == SdkScenario::CanonicalModel {
        "claude-sonnet-5-20250929"
    } else {
        FIXTURE_MODEL
    };
    let capabilities = if scenario == SdkScenario::UnadvertisedInterruptReceipt {
        json!([])
    } else {
        json!(["interrupt_receipt_v1"])
    };
    push(
        state,
        json!({"type": "response", "id": id, "command": "query", "success": true,
               "data": {"accepted": true,
                        "readiness": "confirmed",
                        "cwd": FIXTURE_CWD,
                        "requestedModel": FIXTURE_MODEL,
                        "model": effective_model,
                        "capabilities": capabilities}}),
    );
    push(state, json!({"type": "event", "event": "turn_started"}));
    match scenario {
        SdkScenario::ToolAdmission => {
            push(
                state,
                json!({"type": "callback", "id": "cb-1", "callback": "can_use_tool",
                       "toolName": "Read"}),
            );
        }
        SdkScenario::BashAdmission => {
            push(
                state,
                json!({"type": "callback", "id": "cb-1", "callback": "can_use_tool",
                       "toolName": "Bash", "command": "git status --porcelain",
                       "commandByteLength": 22,
                       "description": "inspect the working tree", "truncated": false}),
            );
        }
        SdkScenario::UnadmittedToolAdmission => {
            push(
                state,
                json!({"type": "callback", "id": "cb-1", "callback": "can_use_tool",
                       "toolName": "Bash", "command": "git status --porcelain",
                       "commandByteLength": 22,
                       "description": "inspect the working tree", "truncated": false}),
            );
        }
        SdkScenario::ToolAdmissionOverflow => {
            for index in 1..=9 {
                push(
                    state,
                    json!({"type": "callback", "id": format!("cb-{index}"),
                           "callback": "can_use_tool", "toolName": "Read"}),
                );
            }
        }
        SdkScenario::UnknownEvent => {
            push(state, json!({"type": "event", "event": "usage_report"}));
        }
        SdkScenario::Malformed => push_raw(state, b"{\"type\":\"event\",\n"),
        SdkScenario::Disconnect => {
            push_raw(state, b"{\"type\":\"event\",\"event\":\"output_");
            state.stopped = true;
        }
        SdkScenario::TerminalRecord => {
            push(
                state,
                json!({"type": "terminal",
                       "failure": {"code": "unknown_message",
                                   "message": "sidecar terminated: unknown_message"}}),
            );
        }
        // The turn ends first, then an admission request the sidecar had
        // already written arrives. The wire order is what a real interrupt or
        // deadline race produces.
        SdkScenario::AdmissionAfterResult => {
            push(
                state,
                json!({"type": "event", "event": "turn_ended", "stopReason": "success",
                       "isError": false}),
            );
            push(
                state,
                json!({"type": "callback", "id": "cb-late", "callback": "can_use_tool",
                       "toolName": "Read"}),
            );
        }
        SdkScenario::ToolOrderingDrift => {
            push(
                state,
                json!({"type": "event", "event": "tool_ended", "toolCallId": "t-9",
                       "isError": false}),
            );
        }
        _ => {
            push(
                state,
                json!({"type": "event", "event": "output_delta", "delta": "answer"}),
            );
            push(
                state,
                json!({"type": "event", "event": "tool_started", "toolCallId": "t-1",
                       "toolName": "Read"}),
            );
            push(
                state,
                json!({"type": "event", "event": "tool_ended", "toolCallId": "t-1",
                       "isError": false}),
            );
            push_stderr(state, b"fixture native stderr tail\n");
            push(state, complete_turn_record());
        }
    }
}

fn set_permission_mode(scenario: SdkScenario, state: &mut ProcessState, id: &str, params: &Value) {
    match scenario {
        SdkScenario::PermissionModeRejected => push(
            state,
            json!({"type": "response", "id": id, "command": "set_permission_mode",
                   "success": false,
                   "failure": {"code": "permission_mode_failed",
                               "message": "sidecar command failed: permission_mode_failed"}}),
        ),
        // A confirmation that names a different mode is not a confirmation.
        SdkScenario::PermissionModeUnconfirmed => push(
            state,
            json!({"type": "response", "id": id, "command": "set_permission_mode",
                   "success": true, "data": {"permissionMode": "default"}}),
        ),
        _ => push(
            state,
            json!({"type": "response", "id": id, "command": "set_permission_mode",
                   "success": true, "data": {"permissionMode": params["mode"].clone()}}),
        ),
    }
}

fn interrupt(scenario: SdkScenario, state: &mut ProcessState, id: &str) {
    if matches!(scenario, SdkScenario::InterruptRejected) {
        push(
            state,
            json!({"type": "response", "id": id, "command": "interrupt", "success": false,
                   "failure": {"code": "interrupt_failed",
                               "message": "sidecar command failed: interrupt_failed"}}),
        );
        return;
    }
    let receipt = !matches!(scenario, SdkScenario::NativeChildSurvives);
    push(
        state,
        json!({"type": "response", "id": id, "command": "interrupt", "success": true,
               "data": {"interrupted": true, "receipt": receipt}}),
    );
}

fn close(scenario: SdkScenario, state: &mut ProcessState, id: &str) {
    if matches!(scenario, SdkScenario::CloseRejected) {
        push(
            state,
            json!({"type": "response", "id": id, "command": "close", "success": false,
                   "failure": {"code": "invalid_command",
                               "message": "sidecar command failed: invalid_command"}}),
        );
        state.stopped = true;
        return;
    }
    let (native_join, observed) = match scenario {
        // The retained handle still shows a live child: a positive survivor
        // observation, not an absence of news.
        SdkScenario::NativeChildSurvives => ("survivor", false),
        // A claimed exit with no observation behind it is inadmissible.
        SdkScenario::NativeJoinWithoutObservation => ("exited", false),
        _ => ("exited", true),
    };
    push(
        state,
        json!({"type": "response", "id": id, "command": "close", "success": true,
               "data": {"nativeJoin": native_join, "joinBoundMs": 2000,
                        "nativeExitObserved": observed,
                        "nativeExitEvent": "exit", "nativeExitCode": 0,
                        "nativeExitSignal": null,
                        "sdkTransportCloseRan": true,
                        "closeTimeline": ["close_requested", "session_input_closed",
                                           "sdk_transport_close_ran", "native_join_exited"]}}),
    );
    state.stopped = true;
}
