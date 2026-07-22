use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

const CREATED: &str =
    include_str!("../../fixtures/model-studio-2026-07-22/conversation-created.json");
const ITEMS: &str = include_str!("../../fixtures/model-studio-2026-07-22/items.json");
const ITEMS_INCOMPLETE: &str =
    include_str!("../../fixtures/model-studio-2026-07-22/items-incomplete.json");
const DELETED_CONVERSATION: &str =
    include_str!("../../fixtures/model-studio-2026-07-22/delete-conversation.json");
const SUCCESS: &str = include_str!("../../fixtures/model-studio-2026-07-22/success.sse");
const DISCONNECT: &str = include_str!("../../fixtures/model-studio-2026-07-22/disconnect.sse");
const PROVIDER_ERROR: &str =
    include_str!("../../fixtures/model-studio-2026-07-22/provider-error.json");

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServerScenario {
    Success,
    ProviderError,
    Disconnect,
    WaitForCancel,
    CleanupFailure,
}

#[derive(Clone, Debug)]
pub struct FixtureRequest {
    pub method: String,
    pub target: String,
    pub headers: BTreeMap<String, String>,
    pub body: Vec<u8>,
}

pub struct FixtureServer {
    endpoint: String,
    requests: Arc<Mutex<Vec<FixtureRequest>>>,
    response_attempts: Arc<AtomicUsize>,
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl FixtureServer {
    pub fn start(scenario: ServerScenario) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("fixture listener binds");
        let endpoint = format!("http://{}", listener.local_addr().expect("address exists"));
        let requests = Arc::new(Mutex::new(Vec::new()));
        let response_attempts = Arc::new(AtomicUsize::new(0));
        let stop = Arc::new(AtomicBool::new(false));
        let state = (
            Arc::clone(&requests),
            Arc::clone(&response_attempts),
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
            response_attempts,
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
    pub fn response_attempts(&self) -> usize {
        self.response_attempts.load(Ordering::SeqCst)
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
    scenario: ServerScenario,
) {
    if request.headers.get("authorization").map(String::as_str) != Some("Bearer fixture-secret") {
        return write_response(stream, 401, "application/json", PROVIDER_ERROR);
    }
    match (request.method.as_str(), request.target.as_str()) {
        ("POST", "/compatible-mode/v1/conversations") => {
            write_response(stream, 200, "application/json", CREATED)
        }
        ("POST", "/compatible-mode/v1/responses") => {
            attempts.fetch_add(1, Ordering::SeqCst);
            match scenario {
                ServerScenario::Success | ServerScenario::CleanupFailure => {
                    write_response(stream, 200, "text/event-stream", SUCCESS)
                }
                ServerScenario::ProviderError => {
                    write_response(stream, 429, "application/json", PROVIDER_ERROR)
                }
                ServerScenario::Disconnect => {
                    write_response(stream, 200, "text/event-stream", DISCONNECT)
                }
                ServerScenario::WaitForCancel => wait_for_cancel(stream),
            }
        }
        ("GET", "/compatible-mode/v1/conversations/conv_fixture_01/items?limit=100&order=asc") => {
            let body = if scenario == ServerScenario::CleanupFailure {
                ITEMS_INCOMPLETE
            } else {
                ITEMS
            };
            write_response(stream, 200, "application/json", body);
        }
        ("DELETE", target)
            if target.starts_with("/compatible-mode/v1/conversations/conv_fixture_01/items/") =>
        {
            let id = target.rsplit('/').next().expect("item id exists");
            let body =
                format!(r#"{{"id":"{id}","object":"conversation.item.deleted","deleted":true}}"#);
            write_response(stream, 200, "application/json", &body);
        }
        ("DELETE", "/compatible-mode/v1/conversations/conv_fixture_01") => {
            write_response(stream, 200, "application/json", DELETED_CONVERSATION)
        }
        _ => write_response(stream, 409, "application/json", PROVIDER_ERROR),
    }
}

fn write_response(stream: &mut TcpStream, status: u16, content_type: &str, body: &str) {
    let reason = if status == 200 {
        "OK"
    } else {
        "Fixture Failure"
    };
    write!(stream, "HTTP/1.1 {status} {reason}\r\nContent-Type: {content_type}\r\nX-Request-Id: req_fixture_01\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len()).expect("response writes");
}

fn wait_for_cancel(stream: &mut TcpStream) {
    write!(stream, "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nX-Request-Id: req_fixture_01\r\nConnection: close\r\n\r\nevent: response.created\ndata: {{\"type\":\"response.created\",\"sequence_number\":0,\"response\":{{\"id\":\"resp_fixture_01\",\"status\":\"queued\",\"model\":\"qwen3.7-plus-2026-05-26\"}}}}\n\n").expect("stream starts");
    for _ in 0..2_000 {
        thread::sleep(Duration::from_millis(1));
        if stream.write_all(b": keepalive\n\n").is_err() {
            break;
        }
        let _ = stream.flush();
    }
}
