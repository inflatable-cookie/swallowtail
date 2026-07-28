mod http;
mod websocket;

use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;

pub(super) const TOKEN: &str = "fixture-kimi-local-bearer";
pub(super) const SESSION: &str = "interactive-session";
pub(super) const EPOCH: &str = "fixture-epoch";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InteractiveScenario {
    Complete,
    GlobalNoise,
    Approval,
    Question,
    UnexpectedApproval,
    Cancel,
    Resync,
    Disconnect,
}

pub struct InteractiveFixtureServer {
    endpoint: String,
    requests: Arc<Mutex<Vec<String>>>,
    stopped: Arc<AtomicBool>,
    listener: Option<JoinHandle<()>>,
    connections: Arc<Mutex<Vec<JoinHandle<()>>>>,
}

impl InteractiveFixtureServer {
    pub fn start(scenario: InteractiveScenario) -> Self {
        Self::start_with_version(scenario, "0.29.0")
    }

    pub fn start_with_version(scenario: InteractiveScenario, version: &str) -> Self {
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
        let connections = Arc::new(Mutex::new(Vec::new()));
        let callback_resolved = Arc::new(AtomicBool::new(false));
        let worker_stopped = Arc::clone(&stopped);
        let worker_requests = Arc::clone(&requests);
        let worker_connections = Arc::clone(&connections);
        let worker_callback = Arc::clone(&callback_resolved);
        let version = Arc::new(version.to_owned());
        let worker = std::thread::spawn(move || {
            while !worker_stopped.load(Ordering::SeqCst) {
                match listener.accept() {
                    Ok((stream, _)) => {
                        let requests = Arc::clone(&worker_requests);
                        let callback = Arc::clone(&worker_callback);
                        let version = Arc::clone(&version);
                        let connection = std::thread::spawn(move || {
                            serve(stream, scenario, requests, callback, &version);
                        });
                        worker_connections
                            .lock()
                            .expect("connection lock is not poisoned")
                            .push(connection);
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
            stopped,
            listener: Some(worker),
            connections,
        }
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    pub fn requests(&self) -> Vec<String> {
        self.requests
            .lock()
            .expect("request lock is not poisoned")
            .clone()
    }
}

impl Drop for InteractiveFixtureServer {
    fn drop(&mut self) {
        self.stopped.store(true, Ordering::SeqCst);
        if let Some(listener) = self.listener.take() {
            let _ = listener.join();
        }
        for connection in self
            .connections
            .lock()
            .expect("connection lock is not poisoned")
            .drain(..)
        {
            let _ = connection.join();
        }
    }
}

fn serve(
    mut stream: TcpStream,
    scenario: InteractiveScenario,
    requests: Arc<Mutex<Vec<String>>>,
    callback_resolved: Arc<AtomicBool>,
    version: &str,
) {
    stream
        .set_nonblocking(false)
        .expect("fixture stream becomes blocking");
    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .expect("fixture read timeout is set");
    let mut peek = [0_u8; 4096];
    let mut count = 0;
    for _ in 0..100 {
        count = stream.peek(&mut peek).unwrap_or_default();
        if peek[..count].windows(4).any(|window| window == b"\r\n\r\n") {
            break;
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    let head = String::from_utf8_lossy(&peek[..count]).to_ascii_lowercase();
    if head.contains("upgrade: websocket") {
        let authenticated = head.contains(&format!("authorization: bearer {TOKEN}"));
        requests
            .lock()
            .expect("request lock is not poisoned")
            .push(format!("WS /api/v1/ws auth={authenticated}"));
        if authenticated {
            websocket::serve(stream, scenario, callback_resolved, requests);
        }
    } else {
        http::serve(&mut stream, requests, callback_resolved, version);
    }
}
