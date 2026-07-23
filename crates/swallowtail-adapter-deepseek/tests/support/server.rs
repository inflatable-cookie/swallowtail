use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

const MODELS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/deepseek-openai-chat-2026-07-22/models.json"
));
const TOOL: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/deepseek-openai-chat-2026-07-22/attempt-1-tool-response.json"
));
const FINAL_TWO: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/deepseek-openai-chat-2026-07-22/attempt-2-final.sse"
));
const FINAL_THREE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/deepseek-openai-chat-2026-07-22/attempt-3-final.sse"
));
const DISCONNECT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/deepseek-openai-chat-2026-07-22/disconnect.sse"
));

#[derive(Clone, Copy)]
pub enum ServerScenario {
    Success,
    ProviderError,
    DisconnectAfterTool,
    WaitAfterTool,
}

#[derive(Clone)]
pub struct FixtureRequest {
    pub method: String,
    pub target: String,
    pub headers: BTreeMap<String, String>,
    pub body: Vec<u8>,
}

pub struct FixtureServer {
    endpoint: String,
    requests: Arc<Mutex<Vec<FixtureRequest>>>,
    attempts: Arc<AtomicUsize>,
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl FixtureServer {
    pub fn start(scenario: ServerScenario) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("fixture listener binds");
        let endpoint = format!("http://{}", listener.local_addr().expect("fixture address"));
        let requests = Arc::new(Mutex::new(Vec::new()));
        let attempts = Arc::new(AtomicUsize::new(0));
        let stop = Arc::new(AtomicBool::new(false));
        let state = (
            Arc::clone(&requests),
            Arc::clone(&attempts),
            Arc::clone(&stop),
        );
        let thread = thread::spawn(move || {
            loop {
                let Ok((mut stream, _)) = listener.accept() else {
                    break;
                };
                if state.2.load(Ordering::SeqCst) {
                    break;
                }
                if let Some(request) = read_request(&mut stream) {
                    state.0.lock().expect("request lock").push(request.clone());
                    respond(&mut stream, &request, &state.1, scenario);
                }
            }
        });
        Self {
            endpoint,
            requests,
            attempts,
            stop,
            thread: Some(thread),
        }
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    pub fn requests(&self) -> Vec<FixtureRequest> {
        self.requests.lock().expect("request lock").clone()
    }

    pub fn attempts(&self) -> usize {
        self.attempts.load(Ordering::SeqCst)
    }
}

impl Drop for FixtureServer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
        let _ = TcpStream::connect(self.endpoint.trim_start_matches("http://"));
        if let Some(thread) = self.thread.take() {
            thread.join().expect("fixture server joins");
        }
    }
}

fn read_request(stream: &mut TcpStream) -> Option<FixtureRequest> {
    stream.set_read_timeout(Some(Duration::from_secs(2))).ok()?;
    let mut bytes = Vec::new();
    let mut chunk = [0_u8; 4096];
    loop {
        let count = stream.read(&mut chunk).ok()?;
        if count == 0 && bytes.is_empty() {
            return None;
        }
        bytes.extend_from_slice(&chunk[..count]);
        let Some(end) = bytes.windows(4).position(|value| value == b"\r\n\r\n") else {
            continue;
        };
        let head = std::str::from_utf8(&bytes[..end]).ok()?;
        let mut lines = head.lines();
        let mut start = lines.next()?.split_whitespace();
        let method = start.next()?.to_owned();
        let target = start.next()?.to_owned();
        let headers: BTreeMap<_, _> = lines
            .filter_map(|line| line.split_once(':'))
            .map(|(name, value)| (name.to_ascii_lowercase(), value.trim().to_owned()))
            .collect();
        let length = headers
            .get("content-length")
            .and_then(|value| value.parse().ok())
            .unwrap_or(0);
        if bytes.len() < end + 4 + length {
            continue;
        }
        return Some(FixtureRequest {
            method,
            target,
            headers,
            body: bytes[end + 4..end + 4 + length].to_vec(),
        });
    }
}

fn respond(
    stream: &mut TcpStream,
    request: &FixtureRequest,
    attempts: &AtomicUsize,
    scenario: ServerScenario,
) {
    if request.headers.get("authorization").map(String::as_str) != Some("Bearer fixture-secret") {
        return write_response(stream, 401, "application/json", "{}", None);
    }
    match (request.method.as_str(), request.target.as_str()) {
        ("GET", "/models") => write_response(
            stream,
            200,
            "application/json",
            MODELS,
            Some("catalogue-request"),
        ),
        ("POST", "/chat/completions") => {
            let attempt = attempts.fetch_add(1, Ordering::SeqCst) + 1;
            match (scenario, attempt) {
                (ServerScenario::ProviderError, 1) => write_response(
                    stream,
                    429,
                    "application/json",
                    r#"{"error":{"type":"rate_limit_error","message":"raw secret detail"}}"#,
                    Some("provider-error-request"),
                ),
                (_, 1) => write_response(stream, 200, "application/json", TOOL, Some("request-1")),
                (ServerScenario::Success, 2) => write_response(
                    stream,
                    200,
                    "text/event-stream",
                    FINAL_TWO,
                    Some("request-2"),
                ),
                (ServerScenario::Success, 3) => write_response(
                    stream,
                    200,
                    "text/event-stream",
                    FINAL_THREE,
                    Some("request-3"),
                ),
                (ServerScenario::DisconnectAfterTool, 2) => write_response(
                    stream,
                    200,
                    "text/event-stream",
                    DISCONNECT,
                    Some("disconnect-request"),
                ),
                (ServerScenario::WaitAfterTool, 2) => wait_for_stop(stream),
                _ => write_response(stream, 409, "application/json", "{}", None),
            }
        }
        _ => write_response(stream, 404, "application/json", "{}", None),
    }
}

fn write_response(
    stream: &mut TcpStream,
    status: u16,
    content_type: &str,
    body: &str,
    request_id: Option<&str>,
) {
    let request_header =
        request_id.map_or_else(String::new, |id| format!("x-request-id: {id}\r\n"));
    write!(
        stream,
        "HTTP/1.1 {status} Fixture\r\nContent-Type: {content_type}\r\n{request_header}Content-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )
    .expect("fixture response writes");
}

fn wait_for_stop(stream: &mut TcpStream) {
    write!(stream, "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nx-request-id: waiting-request\r\nConnection: close\r\n\r\n").expect("stream starts");
    for _ in 0..2_000 {
        thread::sleep(Duration::from_millis(1));
        if stream.write_all(b": keepalive\n\n").is_err() {
            break;
        }
        let _ = stream.flush();
    }
}
