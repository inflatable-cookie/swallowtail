use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;

const TOKEN: &str = "fixture-kimi-local-bearer";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureRequest {
    pub method: String,
    pub path: String,
    pub authenticated: bool,
}

pub struct FixtureServer {
    endpoint: String,
    requests: Arc<Mutex<Vec<FixtureRequest>>>,
    lifecycle_delay_ms: Arc<AtomicU64>,
    stopped: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl FixtureServer {
    pub fn start() -> Self {
        Self::start_with_version("0.29.0")
    }

    pub fn start_with_version(version: &str) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("fixture server binds");
        listener
            .set_nonblocking(true)
            .expect("fixture listener becomes nonblocking");
        let endpoint = format!(
            "http://{}",
            listener.local_addr().expect("fixture address is visible")
        );
        let requests = Arc::new(Mutex::new(Vec::new()));
        let stopped = Arc::new(AtomicBool::new(false));
        let lifecycle_delay_ms = Arc::new(AtomicU64::new(0));
        let version = Arc::new(version.to_owned());
        let worker_requests = Arc::clone(&requests);
        let worker_stopped = Arc::clone(&stopped);
        let worker_delay = Arc::clone(&lifecycle_delay_ms);
        let worker_version = Arc::clone(&version);
        let thread = std::thread::spawn(move || {
            while !worker_stopped.load(Ordering::SeqCst) {
                match listener.accept() {
                    Ok((stream, _)) => {
                        serve(stream, &worker_requests, &worker_delay, &worker_version);
                    }
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        std::thread::sleep(Duration::from_millis(1));
                    }
                    Err(_) => break,
                }
            }
        });
        Self {
            endpoint,
            requests,
            lifecycle_delay_ms,
            stopped,
            thread: Some(thread),
        }
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    pub fn token() -> &'static [u8] {
        TOKEN.as_bytes()
    }

    pub fn requests(&self) -> Vec<FixtureRequest> {
        self.requests
            .lock()
            .expect("fixture request lock is not poisoned")
            .clone()
    }

    pub fn delay_lifecycle_response(&self, milliseconds: u64) {
        self.lifecycle_delay_ms
            .store(milliseconds, Ordering::SeqCst);
    }

    pub fn wait_until_seen(&self, path: &str) {
        self.wait_until_seen_count(path, 1);
    }

    pub fn wait_until_seen_count(&self, path: &str, expected: usize) {
        for _ in 0..1_000 {
            if self
                .requests()
                .iter()
                .filter(|request| request.path == path)
                .count()
                >= expected
            {
                return;
            }
            std::thread::sleep(Duration::from_millis(1));
        }
        panic!("fixture server did not observe expected route");
    }
}

impl Drop for FixtureServer {
    fn drop(&mut self) {
        self.stopped.store(true, Ordering::SeqCst);
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

fn serve(
    mut stream: TcpStream,
    requests: &Arc<Mutex<Vec<FixtureRequest>>>,
    lifecycle_delay_ms: &AtomicU64,
    version: &str,
) {
    stream
        .set_nonblocking(false)
        .expect("fixture stream becomes blocking");
    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .expect("fixture read timeout is set");
    let mut bytes = Vec::new();
    let mut chunk = [0_u8; 1024];
    while !bytes.windows(4).any(|window| window == b"\r\n\r\n") {
        let read = stream
            .read(&mut chunk)
            .expect("fixture request is readable");
        if read == 0 {
            return;
        }
        bytes.extend_from_slice(&chunk[..read]);
        assert!(bytes.len() <= 16 * 1024, "fixture request remains bounded");
    }
    let request = String::from_utf8(bytes).expect("fixture request is UTF-8");
    let mut lines = request.lines();
    let start = lines.next().expect("fixture request has a start line");
    let mut start = start.split_ascii_whitespace();
    let method = start.next().expect("fixture method is present").to_owned();
    let path = start.next().expect("fixture path is present").to_owned();
    let authenticated =
        lines.any(|line| line.eq_ignore_ascii_case(&format!("Authorization: Bearer {TOKEN}")));
    requests
        .lock()
        .expect("fixture request lock is not poisoned")
        .push(FixtureRequest {
            method: method.clone(),
            path: path.clone(),
            authenticated,
        });
    if path.starts_with("/api/v1/sessions/") {
        std::thread::sleep(Duration::from_millis(
            lifecycle_delay_ms.load(Ordering::SeqCst),
        ));
    }
    if path == "/api/v1/sessions/disconnect-session" {
        return;
    }

    let (status, body) = response(&method, &path, authenticated, version);
    let response = format!(
        "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
    let _ = stream.write_all(response.as_bytes());
}

fn response(
    method: &str,
    path: &str,
    authenticated: bool,
    version: &str,
) -> (&'static str, String) {
    if method == "GET" && path == "/api/v1/healthz" {
        return (
            "200 OK",
            r#"{"code":0,"msg":"success","data":{"ok":true},"request_id":"fixture-health"}"#
                .to_owned(),
        );
    }
    if !authenticated {
        return (
            "401 Unauthorized",
            r#"{"code":40101,"msg":"unauthorized","data":null,"request_id":"fixture-unauthorized"}"#
                .to_owned(),
        );
    }
    match (method, path) {
        ("GET", "/api/v1/meta") => (
            "200 OK",
            format!(
                r#"{{"code":0,"msg":"success","data":{{"server_version":"{version}","capabilities":{{"websocket":true}},"dangerous_bypass_auth":false,"backend":"v2"}},"request_id":"fixture-meta"}}"#
            ),
        ),
        ("GET", "/api/v1/sessions/session-1") => (
            "200 OK",
            r#"{"code":0,"msg":"success","data":{"id":"session-1","archived":false},"request_id":"fixture-lookup"}"#.to_owned(),
        ),
        ("GET", "/api/v1/sessions/archived-session") => (
            "200 OK",
            r#"{"code":0,"msg":"success","data":{"id":"archived-session","archived":true},"request_id":"fixture-lookup"}"#.to_owned(),
        ),
        ("POST", "/api/v1/sessions/session-1:archive") => (
            "200 OK",
            r#"{"code":0,"msg":"success","data":{"archived":true},"request_id":"fixture-archive"}"#.to_owned(),
        ),
        ("POST", "/api/v1/sessions/session-1:restore") => (
            "200 OK",
            r#"{"code":0,"msg":"success","data":{"id":"session-1","archived":false},"request_id":"fixture-restore"}"#.to_owned(),
        ),
        _ => (
            "404 Not Found",
            r#"{"code":40401,"msg":"missing","data":null,"request_id":"fixture-missing"}"#.to_owned(),
        ),
    }
}
