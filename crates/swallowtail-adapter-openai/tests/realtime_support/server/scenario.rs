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
    record(socket, frames);
    if matches!(scenario, RealtimeScenario::FormatDrift) {
        send(socket, FORMAT_DRIFT);
        return;
    }
    send(socket, session.next().expect("session.updated exists"));
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
        RealtimeScenario::FormatDrift => unreachable!("format drift exits during handshake"),
    }
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
