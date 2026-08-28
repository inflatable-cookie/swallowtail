use super::super::{FIXTURE_SESSION_REF, ProcessState, SidecarScenario, fixture_failure};
use super::output;
use serde_json::{Value, json};
use swallowtail_runtime::RuntimeFailure;

pub(super) fn session_switch(
    scenario: SidecarScenario,
    id: Option<&str>,
    params: &Value,
    state: &mut ProcessState,
) -> Result<(), RuntimeFailure> {
    let session_ref = params
        .get("sessionRef")
        .and_then(Value::as_str)
        .ok_or_else(fixture_failure)?;
    let expected_cwd = params
        .get("expectedCwd")
        .and_then(Value::as_str)
        .ok_or_else(fixture_failure)?;
    if matches!(scenario, SidecarScenario::SessionNotFound) || session_ref != FIXTURE_SESSION_REF {
        output(
            state,
            json!({
                "type": "response",
                "id": id,
                "command": "session_switch",
                "success": false,
                "failure": {"code": "session_not_found", "message": "sidecar command failed: session_not_found"}
            }),
        );
        return Ok(());
    }
    if matches!(scenario, SidecarScenario::ReplayDuringResume) {
        // Emit unexpected replay before the switch response so resume fails
        // closed on the pending command instead of racing a completed switch
        // against a later force-stop wait.
        replay_item(state, 0);
        output(
            state,
            json!({
                "type": "response",
                "id": id,
                "command": "session_switch",
                "success": true,
                "data": {"effectiveCwd": expected_cwd, "sessionRef": session_ref, "sessionId": "00000000-0000-0000-0000-000000000000", "messages": 3}
            }),
        );
        return Ok(());
    }
    let (effective_cwd, reported_ref) = match scenario {
        SidecarScenario::SwitchCwdMismatch => ("/fixture/other-workspace", session_ref),
        SidecarScenario::SessionSubstituted => {
            (expected_cwd, "11111111-1111-1111-1111-111111111111")
        }
        _ => (expected_cwd, session_ref),
    };
    state.session_ref = Some(session_ref.to_owned());
    output(
        state,
        json!({
            "type": "response",
            "id": id,
            "command": "session_switch",
            "success": true,
            "data": {"effectiveCwd": effective_cwd, "sessionRef": reported_ref, "sessionId": "00000000-0000-0000-0000-000000000000", "messages": 3}
        }),
    );
    Ok(())
}

pub(super) fn session_replay(
    scenario: SidecarScenario,
    id: Option<&str>,
    state: &mut ProcessState,
) {
    match scenario {
        SidecarScenario::ReplayFailure => {
            output(
                state,
                json!({
                    "type": "response",
                    "id": id,
                    "command": "session_replay",
                    "success": false,
                    "failure": {"code": "replay_overflow", "message": "sidecar command failed: replay_overflow"}
                }),
            );
        }
        SidecarScenario::ReplaySequenceGap => {
            replay_item(state, 0);
            replay_item(state, 2);
            replay_response(state, id, 2, true);
        }
        SidecarScenario::ReplayCountMismatch => {
            for sequence in 0..3 {
                replay_item(state, sequence);
            }
            replay_response(state, id, 4, true);
        }
        SidecarScenario::ReplayOverflow => {
            for sequence in 0..1025u64 {
                replay_item(state, sequence);
            }
            replay_response(state, id, 1025, true);
        }
        SidecarScenario::ReplayAfterResponse => {
            replay_item(state, 0);
            replay_response(state, id, 1, true);
            replay_item(state, 1);
        }
        SidecarScenario::HoldReplay => {
            replay_item(state, 0);
        }
        _ => {
            for sequence in 0..3 {
                replay_item(state, sequence);
            }
            replay_response(state, id, 3, true);
        }
    }
}

fn replay_item(state: &mut ProcessState, sequence: u64) {
    let item = match sequence % 3 {
        0 => json!({"kind": "user", "text": "fixture question", "images": 0}),
        1 => json!({
            "kind": "assistant",
            "parts": [
                {"type": "thinking", "thinking": "fixture reasoning"},
                {"type": "text", "text": "fixture answer"},
                {"type": "tool_call", "name": "read", "arguments": {}}
            ],
            "stopReason": "stop",
            "usage": {"input": 12, "output": 4, "cacheRead": 3, "cacheWrite": 2}
        }),
        _ => {
            json!({"kind": "tool_result", "toolName": "read", "isError": false, "text": "fixture file body"})
        }
    };
    output(
        state,
        json!({"type": "event", "event": "replay_item", "sequence": sequence, "item": item}),
    );
}

fn replay_response(state: &mut ProcessState, id: Option<&str>, items: u64, complete: bool) {
    output(
        state,
        json!({
            "type": "response",
            "id": id,
            "command": "session_replay",
            "success": true,
            "data": {"items": items, "complete": complete}
        }),
    );
}
