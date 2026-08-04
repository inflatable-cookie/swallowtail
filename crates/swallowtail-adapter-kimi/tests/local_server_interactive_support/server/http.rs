use super::{InteractiveScenario, TOKEN};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

pub(super) fn serve(
    stream: &mut TcpStream,
    requests: Arc<Mutex<Vec<String>>>,
    callback_resolved: Arc<AtomicBool>,
    version: &str,
    scenario: InteractiveScenario,
) {
    let Some((method, path, authenticated, body)) = read_request(stream) else {
        return;
    };
    requests
        .lock()
        .expect("request lock is not poisoned")
        .push(format!("{method} {path} auth={authenticated} body={body}"));
    let (status, response) = response(
        &method,
        &path,
        authenticated,
        &body,
        &callback_resolved,
        version,
        scenario,
    );
    let reply = format!(
        "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{response}",
        response.len()
    );
    let _ = stream.write_all(reply.as_bytes());
}

fn read_request(stream: &mut TcpStream) -> Option<(String, String, bool, String)> {
    let mut bytes = Vec::new();
    let mut chunk = [0_u8; 1024];
    while !bytes.windows(4).any(|window| window == b"\r\n\r\n") {
        let read = stream.read(&mut chunk).ok()?;
        if read == 0 || bytes.len().saturating_add(read) > 64 * 1024 {
            return None;
        }
        bytes.extend_from_slice(&chunk[..read]);
    }
    let header_end = bytes.windows(4).position(|window| window == b"\r\n\r\n")? + 4;
    let headers = String::from_utf8_lossy(&bytes[..header_end]);
    let mut lines = headers.lines();
    let mut start = lines.next()?.split_ascii_whitespace();
    let method = start.next()?.to_owned();
    let path = start.next()?.to_owned();
    let mut authenticated = false;
    let mut content_length = 0usize;
    for line in lines {
        let lower = line.to_ascii_lowercase();
        authenticated |= lower == format!("authorization: bearer {TOKEN}");
        if let Some(value) = lower.strip_prefix("content-length:") {
            content_length = value.trim().parse().ok()?;
        }
    }
    while bytes.len() < header_end.saturating_add(content_length) {
        let read = stream.read(&mut chunk).ok()?;
        if read == 0 || bytes.len().saturating_add(read) > 64 * 1024 {
            return None;
        }
        bytes.extend_from_slice(&chunk[..read]);
    }
    let body = String::from_utf8(bytes[header_end..header_end + content_length].to_vec()).ok()?;
    Some((method, path, authenticated, body))
}

fn response(
    method: &str,
    path: &str,
    authenticated: bool,
    body: &str,
    callback_resolved: &AtomicBool,
    version: &str,
    scenario: InteractiveScenario,
) -> (&'static str, String) {
    if method == "GET" && path == "/api/v1/healthz" {
        return ("200 OK", envelope(r#"{"ok":true}"#, 0, "success"));
    }
    if !authenticated {
        return ("401 Unauthorized", envelope("null", 40_101, "unauthorized"));
    }
    match (method, path) {
        ("GET", "/api/v1/meta") => (
            "200 OK",
            envelope(
                &format!(
                    r#"{{"server_version":"{version}","capabilities":{{"websocket":true}},"dangerous_bypass_auth":false,"backend":"v2"}}"#
                ),
                0,
                "success",
            ),
        ),
        ("POST", "/api/v1/sessions") => {
            assert!(body.contains(r#""cwd":"fixture.kimi.workspace""#));
            ("200 OK", session_envelope(scenario))
        }
        ("GET", "/api/v1/sessions/interactive-session") => ("200 OK", session_envelope(scenario)),
        ("POST", "/api/v1/sessions/interactive-session:archive") => {
            ("200 OK", envelope(r#"{"archived":true}"#, 0, "success"))
        }
        ("POST", "/api/v1/sessions/interactive-session/prompts") => (
            "200 OK",
            envelope(
                r#"{"prompt_id":"fixture-prompt","status":"running"}"#,
                0,
                "success",
            ),
        ),
        ("GET", "/api/v1/sessions/interactive-session/approvals?status=pending") => (
            "200 OK",
            envelope(
                r#"{"items":[{"approval_id":"approval-1","session_id":"interactive-session","turn_id":7,"tool_call_id":"tool-1","tool_name":"shell","action":"run","tool_input_display":{"command":"echo fixture"},"created_at":"now","expires_at":"later"}]}"#,
                0,
                "success",
            ),
        ),
        ("GET", "/api/v1/sessions/interactive-session/questions?status=pending") => (
            "200 OK",
            envelope(
                r#"{"items":[{"question_id":"question-1","session_id":"interactive-session","turn_id":7,"created_at":"now","questions":[{"id":"q1","question":"Continue?","options":[{"id":"yes","label":"Yes"},{"id":"no","label":"No"}]}]}]}"#,
                0,
                "success",
            ),
        ),
        ("POST", "/api/v1/sessions/interactive-session/approvals/approval-1")
        | ("POST", "/api/v1/sessions/interactive-session/questions/question-1") => {
            callback_resolved.store(true, Ordering::SeqCst);
            (
                "200 OK",
                envelope(r#"{"resolved":true,"resolved_at":"now"}"#, 0, "success"),
            )
        }
        ("POST", "/api/v1/sessions/interactive-session/questions/question-1:dismiss") => {
            callback_resolved.store(true, Ordering::SeqCst);
            (
                "200 OK",
                envelope(r#"{"dismissed":true,"dismissed_at":"now"}"#, 0, "success"),
            )
        }
        _ => ("404 Not Found", envelope("null", 40_401, "missing")),
    }
}

fn session_envelope(scenario: InteractiveScenario) -> String {
    let (last_seq, busy) = match scenario {
        InteractiveScenario::ReconcileComplete => (2, false),
        InteractiveScenario::ReconcileActive => (1, true),
        _ => (0, false),
    };
    envelope(
        &format!(
            r#"{{"id":"interactive-session","archived":false,"busy":{busy},"last_seq":{last_seq},"metadata":{{"cwd":"fixture.kimi.workspace"}}}}"#
        ),
        0,
        "success",
    )
}

fn envelope(data: &str, code: i64, message: &str) -> String {
    format!(r#"{{"code":{code},"msg":"{message}","data":{data},"request_id":"fixture-request"}}"#)
}
