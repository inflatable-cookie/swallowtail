use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;

const TOKEN: &str = "fixture-kimi-local-bearer";

/// Large named hang guard for fixture waits that must resolve through
/// explicit test ordering. Expiry is a broken ordering contract, so it fails
/// loudly instead of hanging the run; no passing test relies on this bound.
const HANG_GUARD: Duration = Duration::from_secs(120);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureRequest {
    pub method: String,
    pub path: String,
    pub authenticated: bool,
}

pub struct FixtureServer {
    endpoint: String,
    requests: Arc<Mutex<Vec<FixtureRequest>>>,
    request_changed: Arc<Condvar>,
    lifecycle_response_gate: Arc<(Mutex<LifecycleResponseGate>, Condvar)>,
    stopped: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

#[derive(Default)]
struct LifecycleResponseGate {
    held: bool,
    active: bool,
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
        let request_changed = Arc::new(Condvar::new());
        let stopped = Arc::new(AtomicBool::new(false));
        let lifecycle_response_gate =
            Arc::new((Mutex::new(LifecycleResponseGate::default()), Condvar::new()));
        let version = Arc::new(version.to_owned());
        let worker_requests = Arc::clone(&requests);
        let worker_request_changed = Arc::clone(&request_changed);
        let worker_stopped = Arc::clone(&stopped);
        let worker_lifecycle_response_gate = Arc::clone(&lifecycle_response_gate);
        let worker_version = Arc::clone(&version);
        let thread = std::thread::spawn(move || {
            while !worker_stopped.load(Ordering::SeqCst) {
                match listener.accept() {
                    Ok((stream, _)) => {
                        serve(
                            stream,
                            &worker_requests,
                            &worker_request_changed,
                            &worker_lifecycle_response_gate,
                            &worker_version,
                        );
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
            request_changed,
            lifecycle_response_gate,
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

    pub fn hold_lifecycle_responses(&self) {
        let (gate, changed) = &*self.lifecycle_response_gate;
        let gate = gate
            .lock()
            .expect("fixture lifecycle response lock is not poisoned");
        let (mut gate, wait) = changed
            .wait_timeout_while(gate, HANG_GUARD, |gate| gate.active)
            .expect("fixture lifecycle response lock is not poisoned");
        assert!(
            !wait.timed_out(),
            "fixture hang guard: lifecycle response never settled before hold within {HANG_GUARD:?}"
        );
        gate.held = true;
    }

    pub fn release_lifecycle_responses(&self) {
        let (gate, changed) = &*self.lifecycle_response_gate;
        let mut gate = gate
            .lock()
            .expect("fixture lifecycle response lock is not poisoned");
        gate.held = false;
        changed.notify_all();
        let (_gate, wait) = changed
            .wait_timeout_while(gate, HANG_GUARD, |gate| gate.active)
            .expect("fixture lifecycle response lock is not poisoned");
        assert!(
            !wait.timed_out(),
            "fixture hang guard: release never drained the lifecycle response within {HANG_GUARD:?}"
        );
    }

    pub fn wait_until_seen(&self, path: &str) {
        self.wait_until_seen_count(path, 1);
    }

    pub fn wait_until_seen_count(&self, path: &str, expected: usize) {
        let requests = self
            .requests
            .lock()
            .expect("fixture request lock is not poisoned");
        let (_requests, wait) = self
            .request_changed
            .wait_timeout_while(requests, HANG_GUARD, |requests| {
                requests
                    .iter()
                    .filter(|request| request.path == path)
                    .count()
                    < expected
            })
            .expect("fixture request lock is not poisoned");
        assert!(
            !wait.timed_out(),
            "fixture hang guard: {path} never reached {expected} observations within {HANG_GUARD:?}"
        );
        drop(_requests);
        let (gate, changed) = &*self.lifecycle_response_gate;
        let gate = gate
            .lock()
            .expect("fixture lifecycle response lock is not poisoned");
        let (_gate, wait) = changed
            .wait_timeout_while(gate, HANG_GUARD, |gate| gate.held && !gate.active)
            .expect("fixture lifecycle response lock is not poisoned");
        assert!(
            !wait.timed_out(),
            "fixture hang guard: gated lifecycle response never drained within {HANG_GUARD:?}"
        );
    }
}

impl Drop for FixtureServer {
    fn drop(&mut self) {
        self.stopped.store(true, Ordering::SeqCst);
        self.release_lifecycle_responses();
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

fn serve(
    mut stream: TcpStream,
    requests: &Arc<Mutex<Vec<FixtureRequest>>>,
    request_changed: &Condvar,
    lifecycle_response_gate: &(Mutex<LifecycleResponseGate>, Condvar),
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
    request_changed.notify_all();
    if path.starts_with("/api/v1/sessions/") {
        let (gate, changed) = lifecycle_response_gate;
        let mut gate = gate
            .lock()
            .expect("fixture lifecycle response lock is not poisoned");
        if gate.held {
            gate.active = true;
            changed.notify_all();
        }
        let (mut gate, wait) = changed
            .wait_timeout_while(gate, HANG_GUARD, |gate| gate.held)
            .expect("fixture lifecycle response lock is not poisoned");
        assert!(
            !wait.timed_out(),
            "fixture hang guard: held lifecycle response was never released within {HANG_GUARD:?}"
        );
        gate.active = false;
        changed.notify_all();
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
        ("GET", "/api/v1/models") => (
            "200 OK",
            include_str!(
                "../fixtures/kimi-local-server-0.28.1-0.29.0/model-catalogue.json"
            )
            .to_owned(),
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
