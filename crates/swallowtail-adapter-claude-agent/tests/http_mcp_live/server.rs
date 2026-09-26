use super::HttpMcpTranscript;
use serde_json::{Value, json};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, PoisonError};
use std::thread::{self, JoinHandle};
use std::time::Duration;

/// Deterministic MCP tool the live gate and harness proof both use.
pub const HTTP_MCP_LIVE_TOOL: &str = "ping";
/// Deterministic tool result. Not a secret.
pub const HTTP_MCP_LIVE_TOOL_RESULT: &str = "pong";

/// Per-connection read timeout, used only to pace the listener's stop check.
///
/// A streamable-HTTP MCP client holds a GET SSE stream open with no traffic, so
/// an idle read must never end the connection. The listener treats a timed-out
/// read as "nothing yet" and keeps the buffered request bytes.
const IDLE_POLL: Duration = Duration::from_millis(100);

/// Disposable loopback streamable-HTTP MCP server with a per-run bearer.
pub struct DisposableHttpMcpServer {
    endpoint: String,
    bearer: String,
    transcript: Arc<Mutex<HttpMcpTranscript>>,
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl DisposableHttpMcpServer {
    /// Binds `127.0.0.1:0` and serves one MCP endpoint until dropped.
    #[must_use]
    pub fn start() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("MCP live listener binds");
        listener
            .set_nonblocking(true)
            .expect("MCP live listener is nonblocking");
        let address = listener
            .local_addr()
            .expect("MCP live address is available");
        let endpoint = format!("http://{address}/mcp");
        let bearer = per_run_bearer();
        let transcript = Arc::new(Mutex::new(HttpMcpTranscript::default()));
        let stop = Arc::new(AtomicBool::new(false));
        let server_transcript = Arc::clone(&transcript);
        let server_stop = Arc::clone(&stop);
        let expected_bearer = bearer.clone();
        let thread = thread::spawn(move || {
            let mut handlers = Vec::new();
            while !server_stop.load(Ordering::SeqCst) {
                match listener.accept() {
                    Ok((stream, _)) => {
                        let transcript = Arc::clone(&server_transcript);
                        let stop = Arc::clone(&server_stop);
                        let expected_bearer = expected_bearer.clone();
                        handlers.push(thread::spawn(move || {
                            handle_connection(stream, &expected_bearer, &transcript, &stop);
                        }));
                    }
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(1));
                    }
                    Err(_) => break,
                }
            }
            for handler in handlers {
                let _ = handler.join();
            }
        });
        Self {
            endpoint,
            bearer,
            transcript,
            stop,
            thread: Some(thread),
        }
    }

    /// Loopback URL declared on production `session/new`.
    #[must_use]
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// Per-run bearer. Callers that log this value leak a secret.
    #[must_use]
    pub fn bearer(&self) -> &str {
        &self.bearer
    }

    /// Authorization header value for the production encoder.
    #[must_use]
    pub fn authorization_header(&self) -> String {
        format!("Bearer {}", self.bearer)
    }

    /// Server-side transcript for this run.
    #[must_use]
    pub fn transcript(&self) -> HttpMcpTranscript {
        self.transcript
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }
}

impl Drop for DisposableHttpMcpServer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

impl std::fmt::Debug for DisposableHttpMcpServer {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("DisposableHttpMcpServer")
            .field("endpoint", &"<redacted>")
            .field("bearer", &"<redacted>")
            .finish_non_exhaustive()
    }
}

fn per_run_bearer() -> String {
    format!(
        "g06-064-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|elapsed| elapsed.as_nanos())
            .unwrap_or(0)
    )
}

fn handle_connection(
    mut stream: TcpStream,
    expected_bearer: &str,
    transcript: &Mutex<HttpMcpTranscript>,
    stop: &AtomicBool,
) {
    stream.set_nonblocking(false).ok();
    stream.set_read_timeout(Some(IDLE_POLL)).ok();
    stream.set_write_timeout(Some(Duration::from_secs(5))).ok();
    let mut pending = Vec::new();
    while !stop.load(Ordering::SeqCst) {
        // An idle read returns here so the stop flag is observed every poll:
        // the connection stays open, the handler thread stays joinable.
        let request = match read_request(&mut stream, &mut pending) {
            ReadOutcome::Request(request) => request,
            ReadOutcome::Idle => continue,
            ReadOutcome::Closed => return,
        };
        let ReadRequest {
            keep_alive,
            target,
            headers,
            body,
        } = request;
        if target.starts_with("GET ") {
            write_sse_open(&mut stream, keep_alive);
            if !keep_alive {
                return;
            }
            continue;
        }
        if !target.starts_with("POST ") {
            write_status(&mut stream, 405, keep_alive, b"");
            if !keep_alive {
                return;
            }
            continue;
        }
        if !authorization_matches(&headers, expected_bearer) {
            write_status(
                &mut stream,
                401,
                keep_alive,
                b"{\"error\":\"unauthorized\"}",
            );
            if !keep_alive {
                return;
            }
            continue;
        }
        transcript
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .mark_authenticated();
        if body.trim().is_empty() {
            write_status(&mut stream, 202, keep_alive, b"");
            if !keep_alive {
                return;
            }
            continue;
        }
        let Ok(request) = serde_json::from_str::<Value>(&body) else {
            write_status(
                &mut stream,
                400,
                keep_alive,
                b"{\"error\":\"invalid json\"}",
            );
            if !keep_alive {
                return;
            }
            continue;
        };
        if let Some(method) = request.get("method").and_then(Value::as_str) {
            transcript
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .record_method(method);
        }
        match mcp_reply(&request) {
            McpReply::Accepted => write_status(&mut stream, 202, keep_alive, b""),
            McpReply::Json(value) => {
                if request.get("method").and_then(Value::as_str) == Some("tools/call")
                    && let Some(text) = value
                        .pointer("/result/content/0/text")
                        .and_then(Value::as_str)
                {
                    transcript
                        .lock()
                        .unwrap_or_else(PoisonError::into_inner)
                        .record_result(text.to_owned());
                }
                let payload = value.to_string();
                if prefers_sse(&headers) {
                    write_sse_message(&mut stream, keep_alive, &payload);
                } else {
                    write_json(&mut stream, keep_alive, payload.as_bytes());
                }
            }
        }
        if !keep_alive {
            return;
        }
    }
}

enum McpReply {
    Accepted,
    Json(Value),
}

fn mcp_reply(request: &Value) -> McpReply {
    let method = request.get("method").and_then(Value::as_str).unwrap_or("");
    let id = request.get("id");
    if id.is_none() {
        return McpReply::Accepted;
    }
    let id = id.cloned().unwrap_or(Value::Null);
    let protocol = request
        .pointer("/params/protocolVersion")
        .and_then(Value::as_str)
        .unwrap_or("2024-11-05");
    McpReply::Json(match method {
        "initialize" => json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "protocolVersion": protocol,
                "capabilities": {"tools": {"listChanged": false}},
                "serverInfo": {"name": "swallowtail-claude-agent-acp-http-mcp-live", "version": "0.0.0"}
            }
        }),
        "tools/list" => json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "tools": [{
                    "name": HTTP_MCP_LIVE_TOOL,
                    "description": "Return a deterministic pong. Non-mutating.",
                    "inputSchema": {
                        "type": "object",
                        "properties": {},
                        "additionalProperties": false
                    }
                }]
            }
        }),
        "tools/call" => {
            let name = request
                .pointer("/params/name")
                .and_then(Value::as_str)
                .unwrap_or_default();
            if name != HTTP_MCP_LIVE_TOOL {
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "error": {"code": -32601, "message": "Method not found"}
                })
            } else {
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "content": [{"type": "text", "text": HTTP_MCP_LIVE_TOOL_RESULT}],
                        "isError": false
                    }
                })
            }
        }
        "ping" => json!({"jsonrpc": "2.0", "id": id, "result": {}}),
        _ => json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": {"code": -32601, "message": "Method not found"}
        }),
    })
}

fn authorization_matches(headers: &str, expected_bearer: &str) -> bool {
    headers.lines().any(|line| {
        let Some((name, value)) = line.split_once(':') else {
            return false;
        };
        name.eq_ignore_ascii_case("authorization")
            && value.trim() == format!("Bearer {expected_bearer}")
    })
}

fn prefers_sse(headers: &str) -> bool {
    headers.lines().any(|line| {
        let Some((name, value)) = line.split_once(':') else {
            return false;
        };
        name.eq_ignore_ascii_case("accept")
            && value.to_ascii_lowercase().contains("text/event-stream")
            && !value.to_ascii_lowercase().contains("application/json")
    })
}

fn read_request(stream: &mut TcpStream, pending: &mut Vec<u8>) -> ReadOutcome {
    let mut chunk = [0_u8; 4096];
    loop {
        match take_request(pending) {
            Ok(Some(request)) => return ReadOutcome::Request(request),
            Ok(None) => {}
            Err(()) => return ReadOutcome::Closed,
        }
        match stream.read(&mut chunk) {
            Ok(0) => return ReadOutcome::Closed,
            Ok(count) => pending.extend_from_slice(&chunk[..count]),
            // An idle read is not a close: a GET SSE stream stays silent while
            // the MCP client keeps it open. Return to the connection loop so it
            // can observe the listener's stop flag instead of blocking a join.
            Err(error)
                if error.kind() == std::io::ErrorKind::WouldBlock
                    || error.kind() == std::io::ErrorKind::TimedOut =>
            {
                return ReadOutcome::Idle;
            }
            Err(_) => return ReadOutcome::Closed,
        }
    }
}

enum ReadOutcome {
    Request(ReadRequest),
    Idle,
    Closed,
}

struct ReadRequest {
    keep_alive: bool,
    target: String,
    headers: String,
    body: String,
}

/// Consumes one complete request from the connection buffer, keeping partial
/// bytes for the next read. `Err` means the connection is not usable.
fn take_request(pending: &mut Vec<u8>) -> Result<Option<ReadRequest>, ()> {
    let Some(header_end) = pending.windows(4).position(|window| window == b"\r\n\r\n") else {
        return Ok(None);
    };
    let headers = std::str::from_utf8(&pending[..header_end])
        .map_err(|_| ())?
        .to_owned();
    let length = content_length(&headers);
    let body_start = header_end + 4;
    if pending.len() < body_start + length {
        return Ok(None);
    }
    let target = headers.lines().next().ok_or(())?.to_owned();
    let keep_alive = keep_alive(&headers);
    let body = String::from_utf8_lossy(&pending[body_start..body_start + length]).into_owned();
    pending.drain(..body_start + length);
    Ok(Some(ReadRequest {
        keep_alive,
        target,
        headers,
        body,
    }))
}

fn content_length(headers: &str) -> usize {
    headers
        .lines()
        .find_map(|line| {
            line.to_ascii_lowercase()
                .strip_prefix("content-length:")
                .and_then(|value| value.trim().parse::<usize>().ok())
        })
        .unwrap_or(0)
}

/// HTTP/1.1 connections are persistent unless the client says otherwise; an
/// explicit `Connection: close` still ends the connection after one response.
fn keep_alive(headers: &str) -> bool {
    let connection = headers.lines().find_map(|line| {
        let (name, value) = line.split_once(':')?;
        name.eq_ignore_ascii_case("connection")
            .then(|| value.trim().to_ascii_lowercase())
    });
    match connection.as_deref() {
        Some(value) if value.contains("close") => false,
        Some(value) if value.contains("keep-alive") => true,
        Some(_) => false,
        None => headers
            .lines()
            .next()
            .is_some_and(|line| line.ends_with("HTTP/1.1")),
    }
}

fn write_json(stream: &mut TcpStream, keep_alive: bool, body: &[u8]) {
    let connection = if keep_alive { "keep-alive" } else { "close" };
    let _ = write!(
        stream,
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nmcp-session-id: live-gate\r\nContent-Length: {}\r\nConnection: {connection}\r\n\r\n",
        body.len()
    );
    let _ = stream.write_all(body);
    let _ = stream.flush();
}

fn write_status(stream: &mut TcpStream, status: u16, keep_alive: bool, body: &[u8]) {
    let reason = match status {
        202 => "Accepted",
        400 => "Bad Request",
        401 => "Unauthorized",
        405 => "Method Not Allowed",
        _ => "OK",
    };
    let connection = if keep_alive { "keep-alive" } else { "close" };
    let _ = write!(
        stream,
        "HTTP/1.1 {status} {reason}\r\nContent-Length: {}\r\nConnection: {connection}\r\n\r\n",
        body.len()
    );
    let _ = stream.write_all(body);
    let _ = stream.flush();
}

fn write_sse_open(stream: &mut TcpStream, keep_alive: bool) {
    let connection = if keep_alive { "keep-alive" } else { "close" };
    let _ = write!(
        stream,
        "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nCache-Control: no-cache\r\nmcp-session-id: live-gate\r\nConnection: {connection}\r\n\r\n: connected\n\n"
    );
    let _ = stream.flush();
}

fn write_sse_message(stream: &mut TcpStream, keep_alive: bool, payload: &str) {
    let connection = if keep_alive { "keep-alive" } else { "close" };
    let body = format!("event: message\ndata: {payload}\n\n");
    let _ = write!(
        stream,
        "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nCache-Control: no-cache\r\nmcp-session-id: live-gate\r\nContent-Length: {}\r\nConnection: {connection}\r\n\r\n{body}",
        body.len()
    );
    let _ = stream.flush();
}
