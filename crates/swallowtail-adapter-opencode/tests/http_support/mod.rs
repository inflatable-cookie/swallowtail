use futures_channel::oneshot;
use futures_executor::block_on;
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

const HTTP_SUCCESS: &str = include_str!("../fixtures/opencode-1.14.48/http-success.json");
const SUCCESS: &str = include_str!("../fixtures/opencode-1.14.48/success.sse");
const PROVIDER_ERROR: &str = include_str!("../fixtures/opencode-1.14.48/provider-error.sse");
const UNKNOWN: &str = include_str!("../fixtures/opencode-1.14.48/unknown-event.sse");
const DISCONNECT: &str = include_str!("../fixtures/opencode-1.14.48/disconnect.sse");
const ABORTED: &str = include_str!("../fixtures/opencode-1.14.48/aborted.sse");
const DUPLICATE_USAGE: &str = include_str!("../fixtures/opencode-1.14.48/duplicate-usage.sse");
const MISSING_USAGE: &str = include_str!("../fixtures/opencode-1.14.48/missing-usage.sse");

#[derive(Clone, Copy, Eq, PartialEq)]
#[allow(dead_code)]
pub enum StreamFixture {
    Success,
    ProviderError,
    Unknown,
    Disconnect,
    DuplicateUsage,
    MissingUsage,
    InputCallbacks,
    WaitForAbort,
    DeleteMissing,
    DeleteUnauthorized,
    DeleteServerError,
    DeleteMalformedSuccess,
    DeleteDisconnect,
    DeleteDelayed,
    DeleteHealthDrift,
}

pub struct FixtureServer {
    endpoint: String,
    requests: Arc<Mutex<Vec<String>>>,
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

struct HandleState {
    requests: Arc<Mutex<Vec<String>>>,
    aborted: Arc<AtomicBool>,
    callback_replies: Arc<AtomicUsize>,
    health_requests: Arc<AtomicUsize>,
    stop: Arc<AtomicBool>,
}

impl FixtureServer {
    pub fn start(fixture: StreamFixture) -> Self {
        Self::start_with_version(fixture, "1.14.48")
    }

    pub fn start_with_version(fixture: StreamFixture, server_version: &str) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("fixture listener binds");
        listener
            .set_nonblocking(true)
            .expect("fixture listener is nonblocking");
        let address = listener.local_addr().expect("fixture address is available");
        let endpoint = format!("http://{address}");
        let requests = Arc::new(Mutex::new(Vec::new()));
        let stop = Arc::new(AtomicBool::new(false));
        let server_requests = Arc::clone(&requests);
        let server_stop = Arc::clone(&stop);
        let server_version = Arc::new(server_version.to_owned());
        let thread = thread::spawn(move || {
            let aborted = Arc::new(AtomicBool::new(false));
            let callback_replies = Arc::new(AtomicUsize::new(0));
            let health_requests = Arc::new(AtomicUsize::new(0));
            let mut handlers = Vec::new();
            while !server_stop.load(Ordering::SeqCst) {
                match listener.accept() {
                    Ok((stream, _)) => {
                        if matches!(
                            fixture,
                            StreamFixture::WaitForAbort | StreamFixture::InputCallbacks
                        ) {
                            let requests = Arc::clone(&server_requests);
                            let stop = Arc::clone(&server_stop);
                            let aborted = Arc::clone(&aborted);
                            let health_requests = Arc::clone(&health_requests);
                            let callback_replies = Arc::clone(&callback_replies);
                            let server_version = Arc::clone(&server_version);
                            handlers.push(thread::spawn(move || {
                                handle(
                                    stream,
                                    fixture,
                                    HandleState {
                                        requests,
                                        aborted,
                                        callback_replies,
                                        health_requests,
                                        stop,
                                    },
                                    &server_version,
                                );
                            }));
                        } else {
                            handle(
                                stream,
                                fixture,
                                HandleState {
                                    requests: Arc::clone(&server_requests),
                                    aborted: Arc::clone(&aborted),
                                    callback_replies: Arc::clone(&callback_replies),
                                    health_requests: Arc::clone(&health_requests),
                                    stop: Arc::clone(&server_stop),
                                },
                                &server_version,
                            );
                        }
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
            requests,
            stop,
            thread: Some(thread),
        }
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    pub fn requests(&self) -> Vec<String> {
        self.requests
            .lock()
            .expect("fixture request lock poisoned")
            .clone()
    }

    #[allow(dead_code)]
    pub fn request_log(&self) -> Arc<Mutex<Vec<String>>> {
        Arc::clone(&self.requests)
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

fn handle(mut stream: TcpStream, fixture: StreamFixture, state: HandleState, server_version: &str) {
    let HandleState {
        requests,
        aborted,
        callback_replies,
        health_requests,
        stop,
    } = state;
    stream
        .set_nonblocking(false)
        .expect("accepted fixture stream is blocking");
    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .expect("read timeout sets");
    let Some((target, body)) = read_request(&mut stream) else {
        return;
    };
    let recorded = if body.is_empty() {
        target.clone()
    } else {
        format!("{target}\n{body}")
    };
    requests
        .lock()
        .expect("fixture request lock poisoned")
        .push(recorded);
    let path = target
        .split_once(' ')
        .map_or(target.as_str(), |(_, target)| target);
    if path.starts_with("/global/health") {
        let health_request = health_requests.fetch_add(1, Ordering::SeqCst);
        let observed_version =
            if matches!(fixture, StreamFixture::DeleteHealthDrift) && health_request >= 2 {
                "1.18.5"
            } else {
                server_version
            };
        respond_json(
            &mut stream,
            200,
            &serde_json::json!({"healthy": true, "version": observed_version}).to_string(),
        );
    } else if path.starts_with("/provider") {
        let fixture: serde_json::Value =
            serde_json::from_str(HTTP_SUCCESS).expect("fixture parses");
        respond_json(
            &mut stream,
            200,
            &fixture[1]["response"]["body"].to_string(),
        );
    } else if path.starts_with("/session?") {
        let fixture: serde_json::Value =
            serde_json::from_str(HTTP_SUCCESS).expect("fixture parses");
        let mut body = fixture[2]["response"]["body"].clone();
        body["version"] = serde_json::Value::String(server_version.to_owned());
        respond_json(&mut stream, 200, &body.to_string());
    } else if target.starts_with("DELETE ") && path.starts_with("/session/") {
        match fixture {
            StreamFixture::DeleteMissing => respond_json(
                &mut stream,
                404,
                r#"{"error":"private missing-target detail"}"#,
            ),
            StreamFixture::DeleteUnauthorized => respond_json(
                &mut stream,
                401,
                r#"{"error":"private authorization detail"}"#,
            ),
            StreamFixture::DeleteServerError => {
                respond_json(&mut stream, 500, r#"{"error":"private server detail"}"#)
            }
            StreamFixture::DeleteMalformedSuccess => respond_json(&mut stream, 200, "false"),
            StreamFixture::DeleteDisconnect => {}
            StreamFixture::DeleteDelayed => {
                thread::sleep(Duration::from_millis(100));
                respond_json(&mut stream, 200, "true");
            }
            StreamFixture::DeleteHealthDrift => respond_json(&mut stream, 200, "true"),
            _ => respond_json(&mut stream, 200, "true"),
        }
    } else if path.contains("/prompt_async?") {
        respond_empty(&mut stream, 204);
    } else if path.starts_with("/permission/per_fixture/reply?")
        || path.starts_with("/question/que_fixture/reply?")
        || path.starts_with("/question/que_fixture/reject?")
    {
        callback_replies.fetch_add(1, Ordering::SeqCst);
        respond_json(&mut stream, 200, "true");
    } else if path.contains("/abort?") {
        aborted.store(true, Ordering::SeqCst);
        respond_json(&mut stream, 200, "true");
    } else if path.starts_with("/event?") {
        respond_sse(&mut stream, fixture, &aborted, &callback_replies, &stop);
    } else {
        respond_json(&mut stream, 404, r#"{"error":"private fixture payload"}"#);
    }
}

fn read_request(stream: &mut TcpStream) -> Option<(String, String)> {
    let mut bytes = Vec::new();
    let mut chunk = [0_u8; 4096];
    loop {
        let count = stream.read(&mut chunk).ok()?;
        if count == 0 {
            return None;
        }
        bytes.extend_from_slice(&chunk[..count]);
        let Some(header_end) = bytes.windows(4).position(|window| window == b"\r\n\r\n") else {
            continue;
        };
        let headers = std::str::from_utf8(&bytes[..header_end]).ok()?;
        let length = headers
            .lines()
            .find_map(|line| {
                line.to_ascii_lowercase()
                    .strip_prefix("content-length:")
                    .and_then(|value| value.trim().parse::<usize>().ok())
            })
            .unwrap_or(0);
        if bytes.len() < header_end + 4 + length {
            continue;
        }
        let target = headers.lines().next()?.to_owned();
        let body_start = header_end + 4;
        let body = String::from_utf8_lossy(&bytes[body_start..body_start + length]).into_owned();
        return Some((target, body));
    }
}

fn respond_json(stream: &mut TcpStream, status: u16, body: &str) {
    let reason = if status == 200 { "OK" } else { "Not Found" };
    write!(
        stream,
        "HTTP/1.1 {status} {reason}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )
    .expect("fixture response writes");
}

fn respond_empty(stream: &mut TcpStream, status: u16) {
    write!(
        stream,
        "HTTP/1.1 {status} No Content\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
    )
    .expect("fixture response writes");
}

fn respond_sse(
    stream: &mut TcpStream,
    fixture: StreamFixture,
    aborted: &AtomicBool,
    callback_replies: &AtomicUsize,
    stop: &AtomicBool,
) {
    write!(
        stream,
        "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nConnection: close\r\n\r\n"
    )
    .expect("SSE headers write");
    match fixture {
        StreamFixture::Success
        | StreamFixture::DeleteMissing
        | StreamFixture::DeleteUnauthorized
        | StreamFixture::DeleteServerError
        | StreamFixture::DeleteMalformedSuccess
        | StreamFixture::DeleteDisconnect
        | StreamFixture::DeleteDelayed
        | StreamFixture::DeleteHealthDrift => {
            stream.write_all(SUCCESS.as_bytes()).expect("SSE writes")
        }
        StreamFixture::ProviderError => stream
            .write_all(PROVIDER_ERROR.as_bytes())
            .expect("SSE writes"),
        StreamFixture::Unknown => stream.write_all(UNKNOWN.as_bytes()).expect("SSE writes"),
        StreamFixture::Disconnect => stream.write_all(DISCONNECT.as_bytes()).expect("SSE writes"),
        StreamFixture::DuplicateUsage => stream
            .write_all(DUPLICATE_USAGE.as_bytes())
            .expect("SSE writes"),
        StreamFixture::MissingUsage => stream
            .write_all(MISSING_USAGE.as_bytes())
            .expect("SSE writes"),
        StreamFixture::InputCallbacks => {
            for (event, expected_replies) in [
                (
                    r#"{"id":"evt_permission_1","type":"permission.asked","properties":{"id":"per_fixture","sessionID":"ses_fixture","permission":"edit","patterns":["src/approved.rs"],"metadata":{},"always":["src/**"]}}"#,
                    1,
                ),
                (
                    r#"{"id":"evt_question_1","type":"question.asked","properties":{"id":"que_fixture","sessionID":"ses_fixture","questions":[{"question":"Choose a bounded mode.","header":"Mode","options":[{"label":"Safe","description":"Keep the operation read-only."},{"label":"Stop","description":"Reject the request."}],"multiple":false}]}}"#,
                    2,
                ),
            ] {
                if aborted.load(Ordering::SeqCst) || stop.load(Ordering::SeqCst) {
                    break;
                }
                writeln!(stream, "data: {event}\n").expect("callback SSE writes");
                stream.flush().expect("callback SSE flushes");
                while callback_replies.load(Ordering::SeqCst) < expected_replies
                    && !aborted.load(Ordering::SeqCst)
                    && !stop.load(Ordering::SeqCst)
                {
                    thread::sleep(Duration::from_millis(1));
                }
            }
            if !aborted.load(Ordering::SeqCst) && !stop.load(Ordering::SeqCst) {
                stream.write_all(SUCCESS.as_bytes()).expect("SSE writes");
            }
        }
        StreamFixture::WaitForAbort => {
            stream
                .write_all(
                    b"data: {\"id\":\"evt_1\",\"type\":\"server.connected\",\"properties\":{}}\n\n",
                )
                .expect("connected event writes");
            stream.flush().expect("connected event flushes");
            while !aborted.load(Ordering::SeqCst) && !stop.load(Ordering::SeqCst) {
                thread::sleep(Duration::from_millis(1));
            }
            if aborted.load(Ordering::SeqCst) {
                stream
                    .write_all(ABORTED.as_bytes())
                    .expect("abort SSE writes");
            }
        }
    }
}

include!("services.rs");
