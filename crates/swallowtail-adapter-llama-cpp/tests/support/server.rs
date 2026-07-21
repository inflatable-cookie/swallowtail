use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

const READY: &str = include_str!("../fixtures/llama-cpp-b9910-openai-chat/health-ready.json");
const PROPERTIES: &str = include_str!("../fixtures/llama-cpp-b9910-openai-chat/properties.json");
const MODELS: &str = include_str!("../fixtures/llama-cpp-b9910-openai-chat/models.json");
const SUCCESS: &str = include_str!("../fixtures/llama-cpp-b9910-openai-chat/success.sse");
const MIDSTREAM_ERROR: &str =
    include_str!("../fixtures/llama-cpp-b9910-openai-chat/midstream-error.sse");

#[derive(Clone, Copy)]
pub enum PropertiesFixture {
    Expected,
    VersionMismatch,
    #[allow(dead_code)]
    RouteMismatch,
}

#[derive(Clone, Copy)]
pub enum StreamFixture {
    Success,
    MidstreamError,
    WaitForCancel,
}

pub struct FixtureServer {
    endpoint: String,
    targets: Arc<Mutex<Vec<String>>>,
    inference_attempts: Arc<AtomicUsize>,
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl FixtureServer {
    pub fn start() -> Self {
        Self::start_with(PropertiesFixture::Expected, StreamFixture::Success)
    }

    pub fn start_with(properties: PropertiesFixture, stream_fixture: StreamFixture) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("fixture listener binds");
        let endpoint = format!("http://{}", listener.local_addr().expect("address exists"));
        let targets = Arc::new(Mutex::new(Vec::new()));
        let inference_attempts = Arc::new(AtomicUsize::new(0));
        let stop = Arc::new(AtomicBool::new(false));
        let server_targets = Arc::clone(&targets);
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
                    server_targets
                        .lock()
                        .expect("target lock is available")
                        .push(request.target.clone());
                    respond(
                        &mut stream,
                        &request,
                        &server_attempts,
                        properties,
                        stream_fixture,
                    );
                }
            }
        });
        Self {
            endpoint,
            targets,
            inference_attempts,
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

    pub fn is_reachable(&self) -> bool {
        let Ok(mut stream) = TcpStream::connect(self.endpoint.trim_start_matches("http://")) else {
            return false;
        };
        if stream
            .write_all(b"GET /health HTTP/1.1\r\nHost: fixture\r\nConnection: close\r\n\r\n")
            .is_err()
        {
            return false;
        }
        let mut response = Vec::new();
        stream.read_to_end(&mut response).is_ok()
            && response.starts_with(b"HTTP/1.1 200")
            && response.ends_with(READY.as_bytes())
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
        return Some(FixtureRequest { method, target });
    }
}

fn respond(
    stream: &mut TcpStream,
    request: &FixtureRequest,
    attempts: &AtomicUsize,
    properties: PropertiesFixture,
    stream_fixture: StreamFixture,
) {
    match (request.method.as_str(), request.target.as_str()) {
        ("GET", "/health") => respond_with(stream, 200, "application/json", READY),
        ("GET", "/props") => {
            let body = match properties {
                PropertiesFixture::Expected => PROPERTIES.to_owned(),
                PropertiesFixture::VersionMismatch => {
                    PROPERTIES.replace("b9910-f5525f7e7", "b10069-178a6c449")
                }
                PropertiesFixture::RouteMismatch => PROPERTIES
                    .replace("b9910-f5525f7e7", "b10069-178a6c449")
                    .replace(
                        "swallowtail-fixture-stories260k",
                        "swallowtail-fixture-unexpected-route",
                    ),
            };
            respond_with(stream, 200, "application/json", &body);
        }
        ("GET", "/v1/models") => respond_with(stream, 200, "application/json", MODELS),
        ("POST", "/v1/chat/completions") => {
            attempts.fetch_add(1, Ordering::SeqCst);
            match stream_fixture {
                StreamFixture::Success => respond_with(stream, 200, "text/event-stream", SUCCESS),
                StreamFixture::MidstreamError => {
                    respond_with(stream, 200, "text/event-stream", MIDSTREAM_ERROR)
                }
                StreamFixture::WaitForCancel => respond_wait_for_cancel(stream),
            }
        }
        _ => respond_with(
            stream,
            404,
            "application/json",
            r#"{"error":{"code":404,"message":"fixture route missing","type":"not_found_error"}}"#,
        ),
    }
}

fn respond_wait_for_cancel(stream: &mut TcpStream) {
    write!(
        stream,
        "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nConnection: close\r\n\r\ndata: {{\"id\":\"cancel\",\"choices\":[{{\"index\":0,\"delta\":{{\"role\":\"assistant\",\"content\":null}},\"finish_reason\":null}}]}}\n\n"
    )
    .expect("waiting stream starts");
    for _ in 0..2_000 {
        thread::sleep(Duration::from_millis(1));
        if stream.write_all(b": keepalive\n\n").is_err() {
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
