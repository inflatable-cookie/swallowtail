use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

const VERSION: &str = include_str!("../fixtures/ollama-native-v0.14.0-v0.32.1/version-0.30.0.json");
const TAGS: &str = include_str!("../fixtures/ollama-native-v0.14.0-v0.32.1/tags.json");
const RUNNING: &str = include_str!("../fixtures/ollama-native-v0.14.0-v0.32.1/ps.json");
const SHOW: &str = include_str!("../fixtures/ollama-native-v0.14.0-v0.32.1/show.json");
const SUCCESS: &str = include_str!("../fixtures/ollama-native-v0.14.0-v0.32.1/chat-success.ndjson");
const MIDSTREAM_ERROR: &str =
    include_str!("../fixtures/ollama-native-v0.14.0-v0.32.1/chat-error.ndjson");
const MALFORMED: &str =
    include_str!("../fixtures/ollama-native-v0.14.0-v0.32.1/chat-malformed.ndjson");
const DISCONNECT: &str =
    include_str!("../fixtures/ollama-native-v0.14.0-v0.32.1/chat-disconnect.ndjson");
const INTERACTIVE_TURN_1: &str =
    include_str!("../fixtures/ollama-native-v0.14.0-v0.32.1/interactive-turn-1-success.ndjson");
const INTERACTIVE_TURN_2: &str =
    include_str!("../fixtures/ollama-native-v0.14.0-v0.32.1/interactive-turn-2-success.ndjson");
const INTERACTIVE_ERROR: &str =
    include_str!("../fixtures/ollama-native-v0.14.0-v0.32.1/interactive-turn-2-error.ndjson");

#[derive(Clone, Copy)]
pub enum VersionFixture {
    Expected,
    Drift,
    DriftAfterPreparation,
    Excluded,
    Newer,
}

#[derive(Clone, Copy)]
pub enum StreamFixture {
    Success,
    MidstreamError,
    Malformed,
    Disconnect,
    WaitForCancel,
    InteractiveSequence,
    InteractiveFailureThenSuccess,
}

pub struct FixtureServer {
    endpoint: String,
    targets: Arc<Mutex<Vec<String>>>,
    bodies: Arc<Mutex<Vec<Vec<u8>>>>,
    inference_attempts: Arc<AtomicUsize>,
    version_requests: Arc<AtomicUsize>,
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl FixtureServer {
    pub fn start() -> Self {
        Self::start_with(VersionFixture::Expected, StreamFixture::Success)
    }

    pub fn start_with(version: VersionFixture, stream_fixture: StreamFixture) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("fixture listener binds");
        let endpoint = format!("http://{}", listener.local_addr().expect("address exists"));
        let targets = Arc::new(Mutex::new(Vec::new()));
        let bodies = Arc::new(Mutex::new(Vec::new()));
        let inference_attempts = Arc::new(AtomicUsize::new(0));
        let version_requests = Arc::new(AtomicUsize::new(0));
        let stop = Arc::new(AtomicBool::new(false));
        let server_targets = Arc::clone(&targets);
        let server_bodies = Arc::clone(&bodies);
        let server_attempts = Arc::clone(&inference_attempts);
        let server_versions = Arc::clone(&version_requests);
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
                    server_targets
                        .lock()
                        .expect("target lock is available")
                        .push(request.target.clone());
                    if request.method == "POST" && request.target == "/api/chat" {
                        server_bodies
                            .lock()
                            .expect("body lock is available")
                            .push(request.body.clone());
                    }
                    respond(
                        &mut stream,
                        &request,
                        &server_attempts,
                        &server_versions,
                        version,
                        stream_fixture,
                    );
                }
            }
        });
        Self {
            endpoint,
            targets,
            bodies,
            inference_attempts,
            version_requests,
            stop,
            thread: Some(thread),
        }
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    pub fn targets(&self) -> Vec<String> {
        self.targets
            .lock()
            .expect("target lock is available")
            .clone()
    }

    pub fn inference_attempts(&self) -> usize {
        self.inference_attempts.load(Ordering::SeqCst)
    }

    pub fn inference_bodies(&self) -> Vec<Vec<u8>> {
        self.bodies.lock().expect("body lock is available").clone()
    }

    pub fn version_requests(&self) -> usize {
        self.version_requests.load(Ordering::SeqCst)
    }

    pub fn is_reachable(&self) -> bool {
        let Ok(mut stream) = TcpStream::connect(self.endpoint.trim_start_matches("http://")) else {
            return false;
        };
        if stream
            .write_all(b"GET /api/version HTTP/1.1\r\nHost: fixture\r\nConnection: close\r\n\r\n")
            .is_err()
        {
            return false;
        }
        let mut response = Vec::new();
        stream.read_to_end(&mut response).is_ok() && response.starts_with(b"HTTP/1.1 200")
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

struct FixtureRequest {
    method: String,
    target: String,
    body: Vec<u8>,
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
        let end = bytes.windows(4).position(|window| window == b"\r\n\r\n")?;
        let head = std::str::from_utf8(&bytes[..end]).ok()?;
        let mut start = head.lines().next()?.split_whitespace();
        let method = start.next()?.to_owned();
        let target = start.next()?.to_owned();
        let length = head
            .lines()
            .filter_map(|line| line.split_once(':'))
            .find(|(name, _)| name.eq_ignore_ascii_case("content-length"))
            .and_then(|(_, value)| value.trim().parse::<usize>().ok())
            .unwrap_or(0);
        if bytes.len() < end + 4 + length {
            continue;
        }
        return Some(FixtureRequest {
            method,
            target,
            body: bytes[end + 4..end + 4 + length].to_vec(),
        });
    }
}

fn respond(
    stream: &mut TcpStream,
    request: &FixtureRequest,
    attempts: &AtomicUsize,
    version_requests: &AtomicUsize,
    version: VersionFixture,
    stream_fixture: StreamFixture,
) {
    match (request.method.as_str(), request.target.as_str()) {
        ("GET", "/api/version") => {
            let request_index = version_requests.fetch_add(1, Ordering::SeqCst);
            let body = match version {
                VersionFixture::Expected => VERSION.to_owned(),
                VersionFixture::Drift => VERSION.replace("0.30.0", "0.32.1"),
                VersionFixture::DriftAfterPreparation if request_index == 0 => VERSION.to_owned(),
                VersionFixture::DriftAfterPreparation => VERSION.replace("0.30.0", "0.32.1"),
                VersionFixture::Excluded => VERSION.replace("0.30.0", "0.32.2"),
                VersionFixture::Newer => VERSION.replace("0.30.0", "0.33.0"),
            };
            respond_with(stream, 200, "application/json", &body);
        }
        ("GET", "/api/tags") => respond_with(stream, 200, "application/json", TAGS),
        ("GET", "/api/ps") => respond_with(stream, 200, "application/json", RUNNING),
        ("POST", "/api/show") => respond_with(stream, 200, "application/json", SHOW),
        ("POST", "/api/chat") => {
            let attempt = attempts.fetch_add(1, Ordering::SeqCst);
            match stream_fixture {
                StreamFixture::Success => {
                    respond_with(stream, 200, "application/x-ndjson", SUCCESS)
                }
                StreamFixture::MidstreamError => {
                    respond_with(stream, 200, "application/x-ndjson", MIDSTREAM_ERROR)
                }
                StreamFixture::Malformed => {
                    respond_with(stream, 200, "application/x-ndjson", MALFORMED)
                }
                StreamFixture::Disconnect => {
                    respond_with(stream, 200, "application/x-ndjson", DISCONNECT)
                }
                StreamFixture::WaitForCancel => respond_wait_for_cancel(stream),
                StreamFixture::InteractiveSequence => respond_with(
                    stream,
                    200,
                    "application/x-ndjson",
                    if attempt == 0 {
                        INTERACTIVE_TURN_1
                    } else {
                        INTERACTIVE_TURN_2
                    },
                ),
                StreamFixture::InteractiveFailureThenSuccess => respond_with(
                    stream,
                    200,
                    "application/x-ndjson",
                    match attempt {
                        0 => INTERACTIVE_TURN_1,
                        1 => INTERACTIVE_ERROR,
                        _ => INTERACTIVE_TURN_2,
                    },
                ),
            }
        }
        _ => respond_with(
            stream,
            404,
            "application/json",
            r#"{"error":"fixture route missing"}"#,
        ),
    }
}

fn respond_wait_for_cancel(stream: &mut TcpStream) {
    write!(
        stream,
        "HTTP/1.1 200 OK\r\nContent-Type: application/x-ndjson\r\nConnection: close\r\n\r\n{{\"model\":\"fixture-model:8b\",\"created_at\":\"2026-07-23T10:00:01Z\",\"message\":{{\"role\":\"assistant\",\"content\":\"Waiting\"}},\"done\":false}}\n"
    )
    .expect("waiting stream starts");
    for _ in 0..2_000 {
        thread::sleep(Duration::from_millis(1));
        if stream.write_all(b" ").is_err() {
            break;
        }
        let _ = stream.flush();
    }
}

fn respond_with(stream: &mut TcpStream, status: u16, content_type: &str, body: &str) {
    let reason = if status == 200 {
        "OK"
    } else {
        "Fixture Failure"
    };
    write!(
        stream,
        "HTTP/1.1 {status} {reason}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )
    .expect("fixture response writes");
}
