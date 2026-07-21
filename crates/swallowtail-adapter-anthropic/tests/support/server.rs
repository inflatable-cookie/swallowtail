use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

const PAGE_1: &str = include_str!("../fixtures/anthropic-2023-06-01/models-page-1.json");
const PAGE_2: &str = include_str!("../fixtures/anthropic-2023-06-01/models-page-2.json");
const SUCCESS: &str = include_str!("../fixtures/anthropic-2023-06-01/success.sse");
const MIDSTREAM_ERROR: &str = include_str!("../fixtures/anthropic-2023-06-01/midstream-error.sse");
const UNKNOWN: &str = include_str!("../fixtures/anthropic-2023-06-01/unknown-event.sse");
const DISCONNECT: &str = include_str!("../fixtures/anthropic-2023-06-01/disconnect.sse");

#[derive(Clone, Copy)]
pub enum StreamFixture {
    Success,
    MidstreamError,
    Unknown,
    Disconnect,
    WaitForCancel,
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
        Self::start_with(StreamFixture::Success)
    }

    pub fn start_with(fixture: StreamFixture) -> Self {
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
                    respond(&mut stream, &request, &server_attempts, fixture);
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

pub fn exchange(endpoint: &str, request: &[u8]) -> FixtureResponse {
    let mut stream = TcpStream::connect(endpoint.trim_start_matches("http://"))
        .expect("fixture endpoint connects");
    stream.write_all(request).expect("request writes");
    stream
        .shutdown(Shutdown::Write)
        .expect("request write side closes");
    let mut bytes = Vec::new();
    stream.read_to_end(&mut bytes).expect("response reads");
    parse_response(&bytes)
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
        let Some(end) = bytes.windows(4).position(|window| window == b"\r\n\r\n") else {
            continue;
        };
        let head = std::str::from_utf8(&bytes[..end]).ok()?;
        let mut lines = head.lines();
        let mut start = lines.next()?.split_whitespace();
        let method = start.next()?.to_owned();
        let target = start.next()?.to_owned();
        let headers = lines
            .filter_map(|line| line.split_once(':'))
            .map(|(name, value)| (name.to_ascii_lowercase(), value.trim().to_owned()))
            .collect::<BTreeMap<_, _>>();
        let length = headers
            .get("content-length")
            .and_then(|value| value.parse::<usize>().ok())
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

include!("server/responses.rs");
