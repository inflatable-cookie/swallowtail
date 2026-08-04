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
const COMPACTION: &str = include_str!("../fixtures/opencode-v1.14.48-v1.18.10/compaction.sse");

#[derive(Clone, Copy, Eq, PartialEq)]
#[allow(dead_code)]
pub enum StreamFixture {
    Success,
    ProviderError,
    Unknown,
    Disconnect,
    DuplicateUsage,
    MissingUsage,
    Compaction,
    InputCallbacks,
    WaitForAbort,
    DeleteMissing,
    DeleteUnauthorized,
    DeleteServerError,
    DeleteMalformedSuccess,
    DeleteDisconnect,
    DeleteDelayed,
    DeleteHealthDrift,
    ImportTitleDrift,
    ImportDelayed,
    ReconciliationActive,
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

include!("handlers.rs");
include!("responses.rs");

include!("services.rs");
