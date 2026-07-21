#![allow(dead_code)]

use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

const CREATE: &str = include_str!("../fixtures/openai-responses-2026-07-21/create-request.json");
const INITIAL: &str = include_str!("../fixtures/openai-responses-2026-07-21/initial-stream.sse");
const REATTACHED: &str =
    include_str!("../fixtures/openai-responses-2026-07-21/reattached-stream.sse");
const IN_PROGRESS: &str =
    include_str!("../fixtures/openai-responses-2026-07-21/retrieve-in-progress.json");
const CANCELLED: &str = include_str!("../fixtures/openai-responses-2026-07-21/cancelled.json");
const COMPLETED: &str =
    include_str!("../fixtures/openai-responses-2026-07-21/retrieve-completed.json");

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[allow(dead_code)]
pub enum ServerMode {
    #[default]
    Success,
    HoldForCancel,
    CancelRace,
    CancelUnconfirmed,
    DisconnectBeforeIdentity,
    ProviderFailed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureRequest {
    pub method: String,
    pub target: String,
    pub headers: BTreeMap<String, String>,
    pub body: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureResponse {
    pub status: u16,
    pub headers: BTreeMap<String, String>,
    pub body: Vec<u8>,
}

pub struct FixtureServer {
    endpoint: String,
    requests: Arc<Mutex<Vec<FixtureRequest>>>,
    inference_attempts: Arc<AtomicUsize>,
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl FixtureServer {
    pub fn start() -> Self {
        Self::start_with(ServerMode::Success)
    }

    pub fn start_with(mode: ServerMode) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("fixture listener binds");
        let endpoint = format!("http://{}", listener.local_addr().expect("address exists"));
        let requests = Arc::new(Mutex::new(Vec::new()));
        let inference_attempts = Arc::new(AtomicUsize::new(0));
        let stop = Arc::new(AtomicBool::new(false));
        let server_requests = Arc::clone(&requests);
        let server_attempts = Arc::clone(&inference_attempts);
        let server_stop = Arc::clone(&stop);
        let thread = thread::spawn(move || {
            loop {
                let Ok((mut stream, _)) = listener.accept() else {
                    break;
                };
                if server_stop.load(Ordering::SeqCst) {
                    break;
                }
                if let Some(request) = read_request(&mut stream) {
                    server_requests
                        .lock()
                        .expect("request lock is available")
                        .push(request.clone());
                    respond(&mut stream, &request, &server_attempts, mode);
                }
            }
        });
        Self {
            endpoint,
            requests,
            inference_attempts,
            stop,
            thread: Some(thread),
        }
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    pub fn requests(&self) -> Vec<FixtureRequest> {
        self.requests
            .lock()
            .expect("request lock is available")
            .clone()
    }

    pub fn inference_attempts(&self) -> usize {
        self.inference_attempts.load(Ordering::SeqCst)
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

fn respond(
    stream: &mut TcpStream,
    request: &FixtureRequest,
    attempts: &AtomicUsize,
    mode: ServerMode,
) {
    if request.headers.get("authorization").map(String::as_str) != Some("Bearer fixture-secret") {
        write_response(
            stream,
            401,
            "application/json",
            r#"{"error":{"code":"invalid_api_key"}}"#,
        );
        return;
    }
    match (request.method.as_str(), request.target.as_str()) {
        ("POST", "/v1/responses") => {
            let attempt = attempts.fetch_add(1, Ordering::SeqCst);
            if attempt > 0 || !same_json(&request.body, CREATE.as_bytes()) {
                write_response(stream, 409, "application/json", "{}");
            } else if mode == ServerMode::DisconnectBeforeIdentity {
                write_response(stream, 200, "text/event-stream", "");
            } else if matches!(
                mode,
                ServerMode::HoldForCancel | ServerMode::CancelRace | ServerMode::CancelUnconfirmed
            ) {
                write_held_stream(stream);
            } else {
                write_response(stream, 200, "text/event-stream", INITIAL);
            }
        }
        ("GET", "/v1/responses/resp_fixture_123") => {
            write_response(
                stream,
                200,
                "application/json",
                if mode == ServerMode::CancelRace {
                    COMPLETED
                } else {
                    IN_PROGRESS
                },
            );
        }
        ("GET", "/v1/responses/resp_fixture_123?stream=true&starting_after=3") => {
            write_response(
                stream,
                200,
                "text/event-stream",
                if mode == ServerMode::ProviderFailed {
                    include_str!("../fixtures/openai-responses-2026-07-21/failed-stream.sse")
                } else {
                    REATTACHED
                },
            );
        }
        ("POST", "/v1/responses/resp_fixture_123/cancel") => match mode {
            ServerMode::CancelRace => {
                write_response(stream, 200, "application/json", COMPLETED);
            }
            ServerMode::CancelUnconfirmed => {
                write_response(stream, 500, "application/json", "{}");
            }
            _ => write_response(stream, 200, "application/json", CANCELLED),
        },
        _ => write_response(stream, 404, "application/json", "{}"),
    }
}

fn write_held_stream(stream: &mut TcpStream) {
    let created = concat!(
        "event: response.created\n",
        "data: {\"type\":\"response.created\",\"sequence_number\":0,",
        "\"response\":{\"id\":\"resp_fixture_123\",\"status\":\"queued\",",
        "\"output\":[],\"usage\":null}}\n\n"
    );
    let head = concat!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\n",
        "x-request-id: req_fixture_success\r\n",
        "x-ratelimit-limit-requests: 100\r\n",
        "x-ratelimit-remaining-requests: 99\r\n",
        "x-ratelimit-reset-requests: 1s\r\n",
        "x-ratelimit-limit-tokens: 100000\r\n",
        "x-ratelimit-remaining-tokens: 99000\r\n",
        "x-ratelimit-reset-tokens: 2s\r\n",
        "Connection: close\r\n\r\n"
    );
    if stream.write_all(head.as_bytes()).is_err() || stream.write_all(created.as_bytes()).is_err() {
        return;
    }
    for _ in 0..1_000 {
        std::thread::sleep(Duration::from_millis(2));
        if stream.write_all(b": keepalive\n\n").is_err() {
            break;
        }
    }
}

fn same_json(actual: &[u8], expected: &[u8]) -> bool {
    serde_json::from_slice::<serde_json::Value>(actual).ok()
        == serde_json::from_slice::<serde_json::Value>(expected).ok()
}

fn write_response(stream: &mut TcpStream, status: u16, content_type: &str, body: &str) {
    let reason = match status {
        200 => "OK",
        401 => "Unauthorized",
        404 => "Not Found",
        409 => "Conflict",
        _ => "Fixture",
    };
    let response = format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: {content_type}\r\nx-request-id: req_fixture_success\r\nx-ratelimit-limit-requests: 100\r\nx-ratelimit-remaining-requests: 99\r\nx-ratelimit-reset-requests: 1s\r\nx-ratelimit-limit-tokens: 100000\r\nx-ratelimit-remaining-tokens: 99000\r\nx-ratelimit-reset-tokens: 2s\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
    stream
        .write_all(response.as_bytes())
        .expect("response writes");
}

include!("server/http.rs");
