const MANAGED_ROOT: &str = "../../fixtures/managed-agents-2026-04-01";
const MANAGED_AGENT: &str =
    include_str!(concat!("../../fixtures/managed-agents-2026-04-01/agent.json"));
const MANAGED_ENVIRONMENT_CREATE: &str = include_str!(concat!(
    "../../fixtures/managed-agents-2026-04-01/environment-create.json"
));
const MANAGED_ENVIRONMENT: &str = include_str!(concat!(
    "../../fixtures/managed-agents-2026-04-01/environment.json"
));
const MANAGED_SESSION_CREATE: &str = include_str!(concat!(
    "../../fixtures/managed-agents-2026-04-01/session-create.json"
));
const MANAGED_SESSION: &str = include_str!(concat!(
    "../../fixtures/managed-agents-2026-04-01/session.json"
));
const MANAGED_MESSAGE: &str = include_str!(concat!(
    "../../fixtures/managed-agents-2026-04-01/user-message.json"
));
const MANAGED_TOOL_RESULT: &str = include_str!(concat!(
    "../../fixtures/managed-agents-2026-04-01/custom-tool-result.json"
));
const MANAGED_INTERRUPT: &str = include_str!(concat!(
    "../../fixtures/managed-agents-2026-04-01/interrupt.json"
));
const MANAGED_SUCCESS: &str =
    include_str!(concat!("../../fixtures/managed-agents-2026-04-01/success.sse"));
const MANAGED_REQUIRES_ACTION: &str = include_str!(concat!(
    "../../fixtures/managed-agents-2026-04-01/requires-action.sse"
));
const MANAGED_DISCONNECT: &str = include_str!(concat!(
    "../../fixtures/managed-agents-2026-04-01/disconnect.sse"
));
const MANAGED_RESCHEDULING: &str = include_str!(concat!(
    "../../fixtures/managed-agents-2026-04-01/rescheduling.sse"
));
const MANAGED_PROVIDER_FAILURE: &str = include_str!(concat!(
    "../../fixtures/managed-agents-2026-04-01/provider-failure.sse"
));
const MANAGED_HISTORY: &str = include_str!(concat!(
    "../../fixtures/managed-agents-2026-04-01/history.json"
));
const MANAGED_DELETE_SESSION: &str = include_str!(concat!(
    "../../fixtures/managed-agents-2026-04-01/delete-session.json"
));
const MANAGED_DELETE_ENVIRONMENT: &str = include_str!(concat!(
    "../../fixtures/managed-agents-2026-04-01/delete-environment.json"
));

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ManagedFixtureState {
    pub environment_created: bool,
    pub session_creations: usize,
    pub stream_attachments: usize,
    pub session_deleted: bool,
    pub environment_deleted: bool,
    pub tool_results: usize,
    pub interrupts: usize,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ManagedStreamFixture {
    #[default]
    Success,
    RequiresActionThenSuccess,
    DisconnectThenSuccess,
    Rescheduling,
    ProviderFailure,
    WaitForInterrupt,
    SessionDeleteFailure,
}

pub struct ManagedFixtureServer {
    endpoint: String,
    requests: Arc<Mutex<Vec<FixtureRequest>>>,
    state: Arc<Mutex<ManagedFixtureState>>,
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl ManagedFixtureServer {
    pub fn start() -> Self {
        Self::start_with(ManagedStreamFixture::Success)
    }

    pub fn start_with(fixture: ManagedStreamFixture) -> Self {
        debug_assert!(MANAGED_ROOT.starts_with("../../fixtures/"));
        let listener = TcpListener::bind("127.0.0.1:0").expect("managed fixture listener binds");
        let endpoint = format!("http://{}", listener.local_addr().expect("address exists"));
        let requests = Arc::new(Mutex::new(Vec::new()));
        let state = Arc::new(Mutex::new(ManagedFixtureState::default()));
        let stop = Arc::new(AtomicBool::new(false));
        let server_requests = Arc::clone(&requests);
        let server_state = Arc::clone(&state);
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
                    respond_managed(&mut stream, &request, &server_state, fixture);
                }
            }
        });
        Self {
            endpoint,
            requests,
            state,
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

    pub fn state(&self) -> ManagedFixtureState {
        *self.state.lock().expect("managed state lock is available")
    }
}

impl Drop for ManagedFixtureServer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
        let _ = TcpStream::connect(self.endpoint.trim_start_matches("http://"));
        if let Some(thread) = self.thread.take() {
            thread.join().expect("managed fixture server joins");
        }
    }
}

fn respond_managed(
    stream: &mut TcpStream,
    request: &FixtureRequest,
    state: &Mutex<ManagedFixtureState>,
    fixture: ManagedStreamFixture,
) {
    if !managed_authorized(request) {
        return respond_json(
            stream,
            400,
            r#"{"type":"error","error":{"type":"invalid_request_error","message":"managed fixture rejected headers"}}"#,
        );
    }
    let mut state = state.lock().expect("managed state lock is available");
    match (request.method.as_str(), request.target.as_str()) {
        ("GET", "/v1/agents/agent_fixture?version=7") => respond_json(stream, 200, MANAGED_AGENT),
        ("POST", "/v1/environments") if body_matches(request, MANAGED_ENVIRONMENT_CREATE) => {
            state.environment_created = true;
            respond_json(stream, 200, MANAGED_ENVIRONMENT);
        }
        ("POST", "/v1/sessions")
            if state.environment_created && managed_session_matches(request) =>
        {
            state.session_creations += 1;
            if state.session_creations == 1 {
                if session_request_has_tools(request) {
                    respond_json(stream, 200, MANAGED_SESSION);
                } else {
                    let mut response: serde_json::Value =
                        serde_json::from_str(MANAGED_SESSION).expect("session fixture is JSON");
                    response["agent"]["tools"] = serde_json::json!([]);
                    respond_json(stream, 200, &response.to_string());
                }
            } else {
                respond_json(
                    stream,
                    409,
                    r#"{"type":"error","error":{"type":"conflict_error","message":"fixture permits one session"}}"#,
                );
            }
        }
        ("GET", "/v1/sessions/session_fixture") if state.session_creations == 1 => {
            respond_json(stream, 200, MANAGED_SESSION)
        }
        ("GET", "/v1/sessions/session_fixture/events/stream") if state.session_creations == 1 => {
            state.stream_attachments += 1;
            let attachment = state.stream_attachments;
            match fixture {
                ManagedStreamFixture::Success => respond_sse(stream, MANAGED_SUCCESS),
                ManagedStreamFixture::RequiresActionThenSuccess if attachment == 1 => {
                    respond_sse(stream, MANAGED_REQUIRES_ACTION)
                }
                ManagedStreamFixture::RequiresActionThenSuccess => {
                    respond_sse(stream, MANAGED_SUCCESS)
                }
                ManagedStreamFixture::DisconnectThenSuccess if attachment == 1 => {
                    respond_sse(stream, MANAGED_DISCONNECT)
                }
                ManagedStreamFixture::DisconnectThenSuccess => respond_sse(stream, MANAGED_SUCCESS),
                ManagedStreamFixture::Rescheduling => respond_sse(stream, MANAGED_RESCHEDULING),
                ManagedStreamFixture::ProviderFailure => {
                    respond_sse(stream, MANAGED_PROVIDER_FAILURE)
                }
                ManagedStreamFixture::WaitForInterrupt => respond_managed_wait(stream),
                ManagedStreamFixture::SessionDeleteFailure => respond_sse(stream, MANAGED_SUCCESS),
            }
        }
        ("GET", "/v1/sessions/session_fixture/events?limit=1000&order=asc")
            if state.session_creations == 1 =>
        {
            respond_json(stream, 200, MANAGED_HISTORY)
        }
        ("POST", "/v1/sessions/session_fixture/events")
            if state.session_creations == 1
                && [MANAGED_MESSAGE, MANAGED_TOOL_RESULT, MANAGED_INTERRUPT]
                    .iter()
                    .any(|fixture| body_matches(request, fixture)) =>
        {
            if body_matches(request, MANAGED_TOOL_RESULT) {
                state.tool_results += 1;
            }
            if body_matches(request, MANAGED_INTERRUPT) {
                state.interrupts += 1;
            }
            respond_json(stream, 200, r#"{"data":[]}"#)
        }
        ("DELETE", "/v1/environments/env_fixture") if !state.session_deleted => respond_json(
            stream,
            409,
            include_str!(concat!(
                "../../fixtures/managed-agents-2026-04-01/deletion-failure.json"
            )),
        ),
        ("DELETE", "/v1/sessions/session_fixture")
            if state.session_creations == 1
                && fixture == ManagedStreamFixture::SessionDeleteFailure =>
        {
            respond_json(
                stream,
                500,
                include_str!(concat!(
                    "../../fixtures/managed-agents-2026-04-01/deletion-failure.json"
                )),
            );
        }
        ("DELETE", "/v1/sessions/session_fixture") if state.session_creations == 1 => {
            state.session_deleted = true;
            respond_json(stream, 200, MANAGED_DELETE_SESSION);
        }
        ("DELETE", "/v1/environments/env_fixture")
            if state.environment_created && state.session_deleted =>
        {
            state.environment_deleted = true;
            respond_json(stream, 200, MANAGED_DELETE_ENVIRONMENT);
        }
        _ => respond_json(
            stream,
            404,
            r#"{"type":"error","error":{"type":"not_found_error","message":"managed fixture route not found"}}"#,
        ),
    }
}

include!("managed/support.rs");
