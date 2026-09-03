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
        "open" => open(scenario, state, &id),
        "query" => query(scenario, state, &id),
        "interrupt" => interrupt(scenario, state, &id),
        "close" => close(scenario, state, &id),
        _ => return Err(super::super::host::fixture_failure()),
    }
    Ok(())
}

fn open(scenario: SdkScenario, state: &mut ProcessState, id: &str) {
    state.opened = true;
    if matches!(scenario, SdkScenario::OpenHold) {
        // No response and no exit: only the host deadline can end this.
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
        "model": FIXTURE_MODEL,
        "capabilities": ["interrupt_receipt_v1"],
        "account": {
            "apiProvider": "firstParty",
            "apiKeySource": "oauth",
            "subscriptionPresent": true
        },
        "tools": ["Read", "Glob", "Grep"]
    });
    match scenario {
        SdkScenario::AccountApiKeySource => data["account"]["apiKeySource"] = json!("apiKeyHelper"),
        SdkScenario::AccountNotFirstParty => data["account"]["apiProvider"] = json!("bedrock"),
        SdkScenario::AccountIdentityLeak => {
            data["account"]["email"] = json!("person@example.test");
        }
        SdkScenario::IdentityMismatch => data["sdkVersion"] = json!("0.3.258"),
        SdkScenario::CwdMismatch => data["cwd"] = json!("/fixture/elsewhere"),
        SdkScenario::ModelMismatch => data["model"] = json!("claude-opus-5"),
        SdkScenario::ToolsWidened => data["tools"] = json!(["Read", "Glob", "Grep", "Bash"]),
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
    push(
        state,
        json!({"type": "response", "id": id, "command": "query", "success": true,
               "data": {"accepted": true}}),
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
        SdkScenario::UnadmittedToolAdmission => {
            push(
                state,
                json!({"type": "callback", "id": "cb-1", "callback": "can_use_tool",
                       "toolName": "Bash"}),
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
            push(
                state,
                json!({"type": "event", "event": "turn_ended", "stopReason": "success",
                       "isError": false}),
            );
        }
    }
}

fn interrupt(scenario: SdkScenario, state: &mut ProcessState, id: &str) {
    let receipt = !matches!(scenario, SdkScenario::NativeChildSurvives);
    push(
        state,
        json!({"type": "response", "id": id, "command": "interrupt", "success": true,
               "data": {"interrupted": true, "receipt": receipt}}),
    );
}

fn close(scenario: SdkScenario, state: &mut ProcessState, id: &str) {
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
                        "nativeExitObserved": observed}}),
    );
    state.stopped = true;
}
