use super::{EPOCH, InteractiveScenario, SESSION};
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tungstenite::{Message, WebSocket, accept};

pub(super) fn serve(
    stream: TcpStream,
    scenario: InteractiveScenario,
    callback_resolved: Arc<AtomicBool>,
    requests: Arc<Mutex<Vec<String>>>,
) {
    let mut socket = match accept(stream) {
        Ok(socket) => socket,
        Err(error) => {
            requests
                .lock()
                .expect("request lock is not poisoned")
                .push(format!("WS accept failed: {error}"));
            return;
        }
    };
    send_json(
        &mut socket,
        r#"{"type":"server_hello","payload":{"protocol_version":2,"ws_connection_id":"fixture-ws","max_event_buffer_size":128}}"#,
    );
    requests
        .lock()
        .expect("request lock is not poisoned")
        .push("WS hello sent".to_owned());
    let message = match socket.read() {
        Ok(message) => message,
        Err(error) => {
            requests
                .lock()
                .expect("request lock is not poisoned")
                .push(format!("WS subscribe read failed: {error}"));
            return;
        }
    };
    let Message::Text(subscribe) = message else {
        return;
    };
    requests
        .lock()
        .expect("request lock is not poisoned")
        .push(format!(
            "WS subscribe seq1={} epoch={}",
            subscribe.contains(r#""seq":1"#),
            subscribe.contains(r#""epoch":"fixture-epoch""#)
        ));
    if !subscribe.contains("swallowtail-subscribe") || !subscribe.contains(SESSION) {
        return;
    }
    let acknowledgement = match scenario {
        InteractiveScenario::ReconcileComplete => {
            r#"{"type":"ack","id":"swallowtail-subscribe","code":0,"msg":"success","payload":{"accepted":["interactive-session"],"resync_required":[],"cursors":{"interactive-session":{"seq":2,"epoch":"fixture-epoch"}}}}"#
        }
        InteractiveScenario::ReconcileActive => {
            r#"{"type":"ack","id":"swallowtail-subscribe","code":0,"msg":"success","payload":{"accepted":["interactive-session"],"resync_required":[],"cursors":{"interactive-session":{"seq":1,"epoch":"fixture-epoch"}}}}"#
        }
        InteractiveScenario::Reattach if subscribe.contains(r#""seq":1"#) => {
            r#"{"type":"ack","id":"swallowtail-subscribe","code":0,"msg":"success","payload":{"accepted":["interactive-session"],"resync_required":[],"cursors":{"interactive-session":{"seq":1,"epoch":"fixture-epoch"}}}}"#
        }
        _ => {
            r#"{"type":"ack","id":"swallowtail-subscribe","code":0,"msg":"success","payload":{"accepted":["interactive-session"],"resync_required":[],"cursors":{"interactive-session":{"seq":0}}}}"#
        }
    };
    send_json(&mut socket, acknowledgement);
    match scenario {
        InteractiveScenario::Disconnect => {}
        InteractiveScenario::Detach => {
            event(&mut socket, 1, None, "turn.started", r#"{"turnId":7}"#);
            match socket.read() {
                Ok(Message::Close(_)) => requests
                    .lock()
                    .expect("request lock is not poisoned")
                    .push("WS observer closed".to_owned()),
                Ok(Message::Text(_)) => requests
                    .lock()
                    .expect("request lock is not poisoned")
                    .push("WS unexpected control text".to_owned()),
                _ => {}
            }
        }
        InteractiveScenario::ReconcileActive => {
            let _ = socket.read();
        }
        InteractiveScenario::ReconcileComplete => {
            if subscribe.contains(r#""seq":1"#) && subscribe.contains(r#""epoch":"fixture-epoch""#)
            {
                event(
                    &mut socket,
                    2,
                    None,
                    "turn.ended",
                    r#"{"turnId":7,"reason":"completed"}"#,
                );
            }
            let _ = socket.read();
        }
        InteractiveScenario::Reattach => {
            let connection_count = requests
                .lock()
                .expect("request lock is not poisoned")
                .iter()
                .filter(|request| request.starts_with("WS /api/v1/ws"))
                .count();
            if connection_count == 1 {
                event(&mut socket, 1, None, "turn.started", r#"{"turnId":7}"#);
            } else if subscribe.contains(r#""seq":1"#)
                && subscribe.contains(r#""epoch":"fixture-epoch""#)
            {
                complete_flow(&mut socket, 2);
            }
        }
        InteractiveScenario::Resync => send_json(
            &mut socket,
            r#"{"type":"resync_required","payload":{"session_id":"interactive-session","reason":"buffer_overflow","current_seq":9}}"#,
        ),
        InteractiveScenario::Cancel => cancel_flow(&mut socket),
        InteractiveScenario::Complete => {
            event(&mut socket, 1, None, "turn.started", r#"{"turnId":7}"#);
            complete_flow(&mut socket, 2);
        }
        InteractiveScenario::Retry => {
            event(&mut socket, 1, None, "turn.started", r#"{"turnId":7}"#);
            event(
                &mut socket,
                2,
                None,
                "turn.step.retrying",
                r#"{"turnId":7,"step":1,"failedAttempt":1,"nextAttempt":2,"maxAttempts":3,"delayMs":100,"errorName":"FixtureError","errorMessage":"private fixture error","statusCode":429}"#,
            );
            complete_flow(&mut socket, 3);
        }
        InteractiveScenario::GlobalNoise => {
            foreign_global_event(&mut socket);
            event(&mut socket, 1, None, "turn.started", r#"{"turnId":7}"#);
            complete_flow(&mut socket, 2);
        }
        InteractiveScenario::Approval => {
            waiting_flow(&mut socket, "awaiting_approval");
            await_callback(&callback_resolved);
            complete_flow(&mut socket, 3);
        }
        InteractiveScenario::Question => {
            waiting_flow(&mut socket, "awaiting_question");
            await_callback(&callback_resolved);
            complete_flow(&mut socket, 3);
        }
        InteractiveScenario::UnexpectedApproval => {
            waiting_flow(&mut socket, "awaiting_approval");
            cancel_response(&mut socket, 3);
        }
    }
}

fn foreign_global_event(socket: &mut WebSocket<TcpStream>) {
    send_json(
        socket,
        r#"{"type":"event.session.created","seq":1,"timestamp":"now","session_id":"foreign-session","epoch":"foreign-epoch","payload":{"session":{"id":"foreign-session"}}}"#,
    );
}

fn complete_flow(socket: &mut WebSocket<TcpStream>, first_seq: u64) {
    volatile_event(
        socket,
        first_seq - 1,
        Some(0),
        "assistant.delta",
        r#"{"turnId":7,"delta":"fixture result"}"#,
    );
    event(
        socket,
        first_seq,
        None,
        "turn.ended",
        r#"{"turnId":7,"reason":"completed"}"#,
    );
    let _ = socket.read();
}

fn volatile_event(
    socket: &mut WebSocket<TcpStream>,
    seq: u64,
    offset: Option<u64>,
    event_type: &str,
    payload: &str,
) {
    let offset = offset.map_or(String::new(), |offset| format!(r#","offset":{offset}"#));
    send_json(
        socket,
        &format!(
            r#"{{"type":"{event_type}","seq":{seq},"timestamp":"now","session_id":"{SESSION}","volatile":true{offset},"payload":{payload}}}"#
        ),
    );
}

fn waiting_flow(socket: &mut WebSocket<TcpStream>, status: &str) {
    event(socket, 1, None, "turn.started", r#"{"turnId":7}"#);
    event(
        socket,
        2,
        None,
        "event.session.status_changed",
        &format!(r#"{{"previous_status":"running","status":"{status}"}}"#),
    );
}

fn cancel_flow(socket: &mut WebSocket<TcpStream>) {
    event(socket, 1, None, "turn.started", r#"{"turnId":7}"#);
    cancel_response(socket, 2);
}

fn cancel_response(socket: &mut WebSocket<TcpStream>, terminal_seq: u64) {
    let Ok(Message::Text(frame)) = socket.read() else {
        return;
    };
    let value: serde_json::Value = serde_json::from_str(&frame).expect("abort is JSON");
    let id = value["id"].as_str().expect("abort id");
    send_json(
        socket,
        &format!(
            r#"{{"type":"ack","id":"{id}","code":0,"msg":"success","payload":{{"accepted":[],"resync_required":[]}}}}"#
        ),
    );
    event(
        socket,
        terminal_seq,
        None,
        "turn.ended",
        r#"{"turnId":7,"reason":"cancelled"}"#,
    );
    let _ = socket.read();
}

fn await_callback(callback_resolved: &AtomicBool) {
    for _ in 0..2_000 {
        if callback_resolved.load(Ordering::SeqCst) {
            return;
        }
        std::thread::sleep(Duration::from_millis(1));
    }
}

fn event(
    socket: &mut WebSocket<TcpStream>,
    seq: u64,
    offset: Option<u64>,
    event_type: &str,
    payload: &str,
) {
    let offset = offset.map_or(String::new(), |offset| format!(r#","offset":{offset}"#));
    send_json(
        socket,
        &format!(
            r#"{{"type":"{event_type}","seq":{seq},"timestamp":"now","session_id":"{SESSION}","epoch":"{EPOCH}"{offset},"payload":{payload}}}"#
        ),
    );
}

fn send_json(socket: &mut WebSocket<TcpStream>, value: &str) {
    let _ = socket.send(Message::Text(value.to_owned().into()));
}
