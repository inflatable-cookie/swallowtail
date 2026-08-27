use super::{
    FORMAT_DRIFT, PROVIDER_ERROR, RealtimeScenario, SESSION_EVENTS, read_turn_frames, record, send,
    send_success, wait_for_close,
};
use serde_json::{Value, json};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

pub(super) fn run(
    scenario: RealtimeScenario,
    socket: &mut tungstenite::WebSocket<TcpStream>,
    frames: &Arc<Mutex<Vec<String>>>,
) {
    let mut session = SESSION_EVENTS.lines();
    send(socket, session.next().expect("session.created exists"));
    let update = record(socket, frames);
    if matches!(scenario, RealtimeScenario::FormatDrift) {
        send(socket, FORMAT_DRIFT);
        return;
    }
    let updated = match scenario {
        RealtimeScenario::ReasoningAckMissing => session_updated_without_reasoning(),
        RealtimeScenario::ReasoningAckMismatch => session_updated_with_effort("high"),
        RealtimeScenario::ReasoningAckMalformed => session_updated_malformed_reasoning(),
        _ => session_updated_echoing(&update, session.next().expect("session.updated exists")),
    };
    send(socket, &updated);
    match scenario {
        RealtimeScenario::TwoTurns => {
            for turn in 1..=2 {
                read_turn_frames(socket, frames);
                send_success(socket, turn);
            }
            wait_for_close(socket, frames);
        }
        RealtimeScenario::Cancel | RealtimeScenario::CancelDisconnect => {
            read_turn_frames(socket, frames);
            send_response_started(socket);
            wait_for_cancel(socket, frames);
            if matches!(scenario, RealtimeScenario::Cancel) {
                send(
                    socket,
                    &json!({
                        "type":"response.done",
                        "response":{
                            "id":"resp_private",
                            "status":"cancelled",
                            "usage":{"input_tokens":2,"output_tokens":0}
                        }
                    })
                    .to_string(),
                );
                wait_for_close(socket, frames);
            }
        }
        RealtimeScenario::Disconnect => {
            read_turn_frames(socket, frames);
            send_response_started(socket);
        }
        RealtimeScenario::ProviderFailed => {
            read_turn_frames(socket, frames);
            send_response_started(socket);
            send(socket, PROVIDER_ERROR);
        }
        RealtimeScenario::Unknown => {
            read_turn_frames(socket, frames);
            send_response_started(socket);
            send(
                socket,
                r#"{"type":"future.semantic.event","private":"hidden"}"#,
            );
        }
        RealtimeScenario::FormatDrift
        | RealtimeScenario::ReasoningAckMissing
        | RealtimeScenario::ReasoningAckMismatch
        | RealtimeScenario::ReasoningAckMalformed => {}
    }
}

fn session_updated_echoing(update_frame: &str, baseline: &str) -> String {
    let update: Value = serde_json::from_str(update_frame).expect("session update is JSON");
    let mut updated: Value = serde_json::from_str(baseline).expect("session updated is JSON");
    match update
        .pointer("/session/reasoning/effort")
        .and_then(Value::as_str)
    {
        Some(effort) => {
            updated["session"]["reasoning"] = json!({"effort": effort});
        }
        None => {
            if let Some(session) = updated.get_mut("session").and_then(Value::as_object_mut) {
                session.remove("reasoning");
            }
        }
    }
    updated.to_string()
}

fn session_updated_without_reasoning() -> String {
    json!({
        "event_id": "server-session-2",
        "type": "session.updated",
        "session": {
            "id": "sess_fixture",
            "model": "gpt-realtime-2.1",
            "audio": {
                "input": {"format": {"type": "audio/pcm", "rate": 24000}},
                "output": {"format": {"type": "audio/pcm", "rate": 24000}, "voice": "marin"}
            }
        }
    })
    .to_string()
}

fn session_updated_with_effort(effort: &str) -> String {
    json!({
        "event_id": "server-session-2",
        "type": "session.updated",
        "session": {
            "id": "sess_fixture",
            "model": "gpt-realtime-2.1",
            "reasoning": {"effort": effort},
            "audio": {
                "input": {"format": {"type": "audio/pcm", "rate": 24000}},
                "output": {"format": {"type": "audio/pcm", "rate": 24000}, "voice": "marin"}
            }
        }
    })
    .to_string()
}

fn session_updated_malformed_reasoning() -> String {
    json!({
        "event_id": "server-session-2",
        "type": "session.updated",
        "session": {
            "id": "sess_fixture",
            "model": "gpt-realtime-2.1",
            "reasoning": {"effort": 1},
            "audio": {
                "input": {"format": {"type": "audio/pcm", "rate": 24000}},
                "output": {"format": {"type": "audio/pcm", "rate": 24000}, "voice": "marin"}
            }
        }
    })
    .to_string()
}

fn send_response_started(socket: &mut tungstenite::WebSocket<TcpStream>) {
    send(
        socket,
        &json!({
            "type":"response.created",
            "response":{"id":"resp_private","status":"in_progress"}
        })
        .to_string(),
    );
}

fn wait_for_cancel(
    socket: &mut tungstenite::WebSocket<TcpStream>,
    frames: &Arc<Mutex<Vec<String>>>,
) {
    loop {
        let frame = record(socket, frames);
        if serde_json::from_str::<Value>(&frame)
            .is_ok_and(|value| value["type"] == "response.cancel")
        {
            return;
        }
    }
}
