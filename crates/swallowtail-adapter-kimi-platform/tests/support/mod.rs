use futures_channel::oneshot;
use futures_executor::block_on;
use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use swallowtail_runtime::{
    BlockingJob, BlockingWorkService, BoxFuture, Deadline, DeadlineObservation, JoinedTask,
    MonotonicInstant, RuntimeFailure, ScopeId, ScopedTaskService, TimeService,
};

const MODELS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../swallowtail-protocol-openai-chat/tests/fixtures/kimi-platform-k3-2026-07-21/models.json"
));
const SUCCESS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../swallowtail-protocol-openai-chat/tests/fixtures/kimi-platform-k3-2026-07-21/success.sse"
));
const UNKNOWN: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../swallowtail-protocol-openai-chat/tests/fixtures/kimi-platform-k3-2026-07-21/unknown.sse"
));
const MISMATCH: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../swallowtail-protocol-openai-chat/tests/fixtures/kimi-platform-k3-2026-07-21/model-mismatch.sse"
));
const DISCONNECT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../swallowtail-protocol-openai-chat/tests/fixtures/kimi-platform-k3-2026-07-21/disconnect.sse"
));

#[derive(Clone, Copy)]
pub enum StreamFixture {
    Success,
    Unknown,
    Mismatch,
    Disconnect,
    ProviderError,
    WaitForCancel,
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
    pub fn start(fixture: StreamFixture) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("fixture listener binds");
        let endpoint = format!("http://{}", listener.local_addr().expect("address exists"));
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
                    respond(&mut stream, &request, &state.1, fixture);
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
            thread.join().expect("server joins");
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
    fixture: StreamFixture,
) {
    if request.headers.get("authorization").map(String::as_str) != Some("Bearer fixture-secret") {
        return write_response(
            stream,
            401,
            "application/json",
            r#"{"error":{"type":"invalid_authentication_error","message":"secret detail","code":"invalid_authentication_error"}}"#,
        );
    }
    match (request.method.as_str(), request.target.as_str()) {
        ("GET", "/v1/models") => write_response(stream, 200, "application/json", MODELS),
        ("POST", "/v1/chat/completions") if attempts.fetch_add(1, Ordering::SeqCst) == 0 => {
            match fixture {
                StreamFixture::Success => write_response(stream, 200, "text/event-stream", SUCCESS),
                StreamFixture::Unknown => write_response(stream, 200, "text/event-stream", UNKNOWN),
                StreamFixture::Mismatch => {
                    write_response(stream, 200, "text/event-stream", MISMATCH)
                }
                StreamFixture::Disconnect => {
                    write_response(stream, 200, "text/event-stream", DISCONNECT)
                }
                StreamFixture::ProviderError => write_response(
                    stream,
                    429,
                    "application/json",
                    r#"{"error":{"type":"rate_limit_reached_error","message":"raw private detail","code":"rate_limit_reached_error"}}"#,
                ),
                StreamFixture::WaitForCancel => wait_for_cancel(stream),
            }
        }
        _ => write_response(
            stream,
            409,
            "application/json",
            r#"{"error":{"type":"server_unavailable","message":"one attempt only","code":"server_unavailable"}}"#,
        ),
    }
}

fn write_response(stream: &mut TcpStream, status: u16, content_type: &str, body: &str) {
    let reason = if status == 200 {
        "OK"
    } else {
        "Fixture Failure"
    };
    write!(stream, "HTTP/1.1 {status} {reason}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len()).expect("response writes");
}

fn wait_for_cancel(stream: &mut TcpStream) {
    write!(stream, "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nConnection: close\r\n\r\ndata: {{\"id\":\"fixture\",\"object\":\"chat.completion.chunk\",\"created\":1,\"model\":\"kimi-k3\",\"choices\":[{{\"index\":0,\"delta\":{{\"role\":\"assistant\",\"content\":null}},\"finish_reason\":null}}]}}\n\n").expect("stream starts");
    for _ in 0..2_000 {
        thread::sleep(Duration::from_millis(1));
        if stream.write_all(b": keepalive\n\n").is_err() {
            break;
        }
        let _ = stream.flush();
    }
}

include!("services.rs");
