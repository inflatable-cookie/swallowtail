//! Provider-free route-binding proofs for registered tools on `claude-agent.sdk`.
//!
//! The fake sidecar observes the reserved courier declaration. The real Card
//! 116 proxy and courier carry one mediated call into
//! `RegisteredToolDispatcher::dispatch`. No live Claude, Node SDK, or
//! credential is used.

use crate::host_id;
use crate::sdk_support::{SdkFixtureHost, SdkScenario, cleanup_request, prepared_session};
use futures_executor::block_on;
use std::future::ready;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::task::{Poll, Waker};
use std::time::{Duration, Instant};
use swallowtail_adapter_claude_agent::sdk::open_receipt::{
    ClaudeAgentSdkOpenStage, ClaudeAgentSdkOpenSubcode,
};
use swallowtail_adapter_claude_agent::sdk::registered_tool::CLAUDE_AGENT_SDK_REGISTERED_TOOL_SERVER;
use swallowtail_adapter_claude_agent::sdk::{
    ClaudeAgentSdkSessionPreparation, ClaudeAgentSdkSessionProfile,
    prepare_claude_agent_sdk_session,
};
use swallowtail_core::{ConfiguredInstanceId, ExecutionHostId};
use swallowtail_host_local::{LocalHostServices, LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    AdmissionPhase, BoxFuture, CleanupOutcome, EnvironmentRef, ExecutableRef, HostServices,
    InteractiveSessionHandle, ProcessHandle, ProcessInputChunk, ProcessOutputStream,
    ProcessTreeCompletion, REGISTERED_TOOL_CONFORMANCE_PROTOCOL_VERSION,
    REGISTERED_TOOL_PROXY_WIRE_TAG, RegisteredServerId, RegisteredServerRevision,
    RegisteredToolAttachment, RegisteredToolBounds, RegisteredToolCall,
    RegisteredToolDispatchContext, RegisteredToolDispatcher, RegisteredToolEffectPosture,
    RegisteredToolExecutionKind, RegisteredToolId, RegisteredToolLimits, RegisteredToolLocalName,
    RegisteredToolNamespace, RegisteredToolOutcome, RegisteredToolPayload,
    RegisteredToolPreparation, RegisteredToolProtocolVersion, RegisteredToolProxyRecipe,
    RegisteredToolResult, RegisteredToolRetryPosture, RegisteredToolSchema,
    RegisteredToolSchemaDialect, RegisteredToolSchemaDigest, RegisteredToolSchemaDocument,
    RegisteredToolSchemaMediaType, RegisteredToolSchemaNamespace, RegisteredToolSelection,
    RegisteredToolSnapshot, RegisteredToolSnapshotInput, RegisteredToolSource,
    RegisteredToolSourceId, RegisteredToolTransport, RegisteredToolTransportSupport,
    RuntimeFailure,
};
use swallowtail_testkit::{ScriptedAdmissionPort, fixture_admission};

const NAMESPACE: &str = "desktop";
const RECONCILE: &str = "reconcile";
const OPEN_DEADLINE_TICKS: u64 = 10_000_000_000;

struct CountingDispatcher {
    calls: Arc<AtomicUsize>,
}

impl RegisteredToolDispatcher for CountingDispatcher {
    fn dispatch(
        &self,
        call: RegisteredToolCall,
        _context: RegisteredToolDispatchContext,
    ) -> BoxFuture<'_, Result<RegisteredToolOutcome, RuntimeFailure>> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        let payload = RegisteredToolPayload::new(
            RegisteredToolSchemaMediaType::new("application/json").expect("media type"),
            br#"{"answer":"from-dispatcher"}"#.to_vec(),
            call.binding().effective_bounds().max_result_bytes(),
        )
        .expect("bounded dispatcher result");
        let result = RegisteredToolResult::new(
            payload,
            RegisteredToolSchemaDigest::new("sha256:output").expect("output digest"),
        );
        Box::pin(ready(Ok(RegisteredToolOutcome::completed(&call, result))))
    }
}

struct BlockingDispatcher {
    entered: Arc<AtomicBool>,
    release: Arc<AtomicBool>,
    waker: Arc<Mutex<Option<Waker>>>,
}

impl RegisteredToolDispatcher for BlockingDispatcher {
    fn dispatch(
        &self,
        _call: RegisteredToolCall,
        context: RegisteredToolDispatchContext,
    ) -> BoxFuture<'_, Result<RegisteredToolOutcome, RuntimeFailure>> {
        let entered = Arc::clone(&self.entered);
        let release = Arc::clone(&self.release);
        let waker = Arc::clone(&self.waker);
        let cancellation = context.cancellation().clone();
        Box::pin(std::future::poll_fn(move |cx| {
            entered.store(true, Ordering::SeqCst);
            if release.load(Ordering::SeqCst) || cancellation.is_cancelled() {
                *waker.lock().expect("blocking dispatcher waker lock") = None;
                return Poll::Ready(Err(RuntimeFailure::new(
                    swallowtail_core::SafeDiagnostic::new(
                        "fixture.blocking_dispatcher.released",
                        "fixture blocking dispatcher released",
                    ),
                )));
            }
            let mut slot = waker.lock().expect("blocking dispatcher waker lock");
            let start_ticker = slot.is_none();
            *slot = Some(cx.waker().clone());
            drop(slot);
            if start_ticker {
                let waker = Arc::clone(&waker);
                let release = Arc::clone(&release);
                let cancellation = cancellation.clone();
                std::thread::spawn(move || {
                    while !release.load(Ordering::SeqCst) && !cancellation.is_cancelled() {
                        std::thread::sleep(std::time::Duration::from_millis(5));
                        if let Some(waker) = waker
                            .lock()
                            .expect("blocking dispatcher waker lock")
                            .clone()
                        {
                            waker.wake();
                        }
                    }
                    if let Some(waker) =
                        waker.lock().expect("blocking dispatcher waker lock").take()
                    {
                        waker.wake();
                    }
                });
            }
            Poll::Pending
        }))
    }
}

impl BlockingDispatcher {
    fn release(&self) {
        self.release.store(true, Ordering::SeqCst);
        if let Some(waker) = self
            .waker
            .lock()
            .expect("blocking dispatcher waker lock")
            .take()
        {
            waker.wake();
        }
    }
}

struct CourierClient {
    process: Arc<dyn ProcessHandle>,
}

impl CourierClient {
    fn request(&self, request: &[u8]) -> String {
        let mut bytes = request.to_vec();
        bytes.push(b'\n');
        block_on(self.process.write_stdin(ProcessInputChunk::new(bytes)))
            .expect("SDK writes one JSONL request");
        self.read_stdout_line()
            .expect("SDK reads one JSONL response")
    }

    fn notify(&self, request: &[u8]) {
        let mut bytes = request.to_vec();
        bytes.push(b'\n');
        block_on(self.process.write_stdin(ProcessInputChunk::new(bytes)))
            .expect("SDK writes one notification");
    }

    fn call_tool(&self, name: &str, arguments: &str) -> String {
        let request = format!(
            "{{\"jsonrpc\":\"2.0\",\"id\":3,\"method\":\"tools/call\",\"params\":{{\"name\":{name:?},\"arguments\":{arguments}}}}}"
        );
        self.request(request.as_bytes())
    }

    fn try_call_tool(&self, name: &str, arguments: &str) -> Option<String> {
        let request = format!(
            "{{\"jsonrpc\":\"2.0\",\"id\":3,\"method\":\"tools/call\",\"params\":{{\"name\":{name:?},\"arguments\":{arguments}}}}}"
        );
        let mut bytes = request.into_bytes();
        bytes.push(b'\n');
        block_on(self.process.write_stdin(ProcessInputChunk::new(bytes))).ok()?;
        self.read_stdout_line()
    }

    fn handshake(&self) {
        let initialized = self.request(
            br#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-11-25"}}"#,
        );
        assert!(
            initialized.contains("2025-11-25"),
            "courier handshake: {initialized}"
        );
        self.notify(br#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#);
    }

    fn read_stdout_line(&self) -> Option<String> {
        let mut output = Vec::new();
        loop {
            let chunk = block_on(self.process.read_output()).ok()??;
            if chunk.stream() == ProcessOutputStream::Stdout {
                output.extend_from_slice(chunk.bytes());
                if output.contains(&b'\n') {
                    return Some(String::from_utf8(output).expect("JSON response is UTF-8"));
                }
            }
        }
    }
}

fn courier_binary() -> &'static Path {
    static PATH: OnceLock<PathBuf> = OnceLock::new();
    PATH.get_or_init(|| {
        let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("crate dir")
            .parent()
            .expect("workspace")
            .to_path_buf();
        // Nested cargo must not share the outer `cargo test` target lock.
        let nested_target = workspace.join("target").join("card125-courier");
        let binary = nested_target
            .join("debug")
            .join("swallowtail-registered-tool-courier");
        let bytes = acquire_built_courier(&workspace, &nested_target, &binary);
        let published = publish_write_once(&bytes, &nested_target);
        assert!(
            is_executable(&published),
            "courier binary at {published:?} is not executable, so every later spawn would fail"
        );
        published
    })
}

/// Builds the courier and reads the completed artifact, retrying the pair
/// until the read succeeds.
///
/// Card 139: Cargo's uplift is not atomic and its build lock does not cover
/// the window after the build returns. A sibling test process rebuilding into
/// the same nested target removes and recreates
/// `debug/swallowtail-registered-tool-courier`, so both a read and a spawn of
/// that path can transiently fail with `ENOENT` — measured at 42 such
/// observations across eight rebuilds. That transient absence is the spawn
/// failure the fixture used to report as a bare code. Reading is retried
/// because the artifact is transiently absent, never permanently wrong.
fn acquire_built_courier(workspace: &Path, nested_target: &Path, binary: &Path) -> Vec<u8> {
    const ATTEMPTS: usize = 5;
    let mut absences = Vec::new();
    for attempt in 1..=ATTEMPTS {
        let built = run_bounded(
            std::process::Command::new("cargo")
                .args([
                    "build",
                    "-p",
                    "swallowtail-host-local",
                    "--features",
                    "mediated-stdio-proxy",
                    "--bin",
                    "swallowtail-registered-tool-courier",
                ])
                .env("CARGO_TARGET_DIR", nested_target)
                .current_dir(workspace),
        );
        // Card 139: only the measured transient failure is retried. A build
        // that fails is a real defect and fails here with its own output;
        // retrying it would let an intermittent compiler error or a killed
        // rustc pass as clean validation on a later attempt.
        assert!(
            built.status.success(),
            "courier binary failed to build on attempt {attempt}: {}\nstdout: {}\nstderr: {}",
            built.status,
            built.stdout,
            built.stderr
        );
        match std::fs::read(binary) {
            Ok(bytes) if !bytes.is_empty() => return bytes,
            Ok(_) => absences.push(format!("attempt {attempt}: built courier read as empty")),
            // Only the measured transient absence is recoverable. A denied or
            // otherwise failing read is a real defect, and retrying it would
            // let it pass as clean validation on a later attempt.
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => absences.push(format!(
                "attempt {attempt}: {binary:?} was absent: {error} ({})",
                error.raw_os_error().map_or_else(
                    || "no os code".to_owned(),
                    |code| format!("os error {code}")
                )
            )),
            Err(error) => panic!(
                "reading the built courier at {binary:?} failed on attempt {attempt}: {error} ({}, kind {:?})",
                error.raw_os_error().map_or_else(
                    || "no os code".to_owned(),
                    |code| format!("os error {code}")
                ),
                error.kind()
            ),
        }
    }
    panic!(
        "courier binary was absent on all {ATTEMPTS} acquisition attempts:\n{}",
        absences.join("\n")
    );
}

/// One command's exit plus its output, with each stream bounded as it is read.
struct BoundedOutput {
    status: std::process::ExitStatus,
    stdout: String,
    stderr: String,
}

/// Bound on collecting build output after the build itself has exited.
///
/// Card 139: a surviving descendant can hold an inherited pipe open after the
/// build exits. Joining the readers outright would withhold an exit status
/// already observed, for as long as that descendant lives.
const BUILD_DRAIN_BOUND: Duration = Duration::from_secs(5);

/// Runs `command`, capping each captured stream instead of buffering all of
/// it. Both streams are drained so the child cannot block on a full pipe, and
/// collection after exit is bounded so an observed failure is always reported.
fn run_bounded(command: &mut std::process::Command) -> BoundedOutput {
    let mut child = command
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("courier build starts");
    let stdout = child.stdout.take().expect("build stdout pipe");
    let stderr = child.stderr.take().expect("build stderr pipe");
    let out_capture = Arc::new(Mutex::new(StreamCapture::default()));
    let err_capture = Arc::new(Mutex::new(StreamCapture::default()));
    let (send_out, recv_out) = std::sync::mpsc::channel();
    let (send_err, recv_err) = std::sync::mpsc::channel();
    let out_writer = Arc::clone(&out_capture);
    let err_writer = Arc::clone(&err_capture);
    std::thread::spawn(move || {
        read_bounded(stdout, &out_writer);
        send_out.send(())
    });
    std::thread::spawn(move || {
        read_bounded(stderr, &err_writer);
        send_err.send(())
    });
    let status = child.wait().expect("courier build completes");
    // The exit is already known; collecting output may not withhold it. One
    // deadline covers both streams: a stream left with no remaining wait still
    // reports everything it read, so per-stream bounds would only double the
    // worst-case delay without preserving one extra byte.
    let deadline = Instant::now() + BUILD_DRAIN_BOUND;
    BoundedOutput {
        status,
        stdout: collect_bounded(&recv_out, &out_capture, "stdout", deadline),
        stderr: collect_bounded(&recv_err, &err_capture, "stderr", deadline),
    }
}

/// Takes one reader's capture, whether or not the reader finished.
///
/// The reader thread is left running rather than joined: it ends when its
/// pipe closes, and nothing here may wait on a descendant to do that. The
/// capture is shared rather than sent, so a bound that expires still reports
/// every byte read so far instead of discarding the diagnostic it was meant
/// to preserve.
fn collect_bounded(
    finished: &std::sync::mpsc::Receiver<()>,
    capture: &Arc<Mutex<StreamCapture>>,
    stream: &str,
    deadline: Instant,
) -> String {
    let ending = match finished.recv_timeout(deadline.saturating_duration_since(Instant::now())) {
        Ok(()) => None,
        Err(std::sync::mpsc::RecvTimeoutError::Timeout) => Some(format!(
            "build {stream} still open {BUILD_DRAIN_BOUND:?} after exit"
        )),
        // A dropped sender is a reader that ended without reporting, which is
        // not the same as a stream that is still producing.
        Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
            Some(format!("build {stream} reader ended without reporting"))
        }
    };
    let capture = capture.lock().expect("build capture lock");
    let rendered = capture.describe();
    let fault = capture.fault.clone();
    drop(capture);
    match (ending, fault) {
        (None, None) => rendered,
        (ending, fault) => {
            let reasons = [ending, fault]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>()
                .join("; ");
            format!("{rendered} (capture incomplete: {reasons})")
        }
    }
}

/// One bounded stream capture, readable while its reader is still running.
#[derive(Default)]
struct StreamCapture {
    retained: std::collections::VecDeque<u8>,
    dropped: usize,
    fault: Option<String>,
}

impl StreamCapture {
    fn describe(&self) -> String {
        let bytes = self.retained.iter().copied().collect::<Vec<_>>();
        let text = String::from_utf8_lossy(&bytes).into_owned();
        if self.dropped == 0 {
            text
        } else {
            format!("(earlier {} bytes dropped)… {text}", self.dropped)
        }
    }
}

/// Reads a stream to its end into `capture`, retaining only its last
/// `OUTPUT_CAP` bytes.
///
/// A build's diagnosis is at the end of its output, behind however much
/// progress noise the run produced, so the tail is the part worth keeping.
/// Bytes land in the shared capture as they are read, so a caller whose bound
/// expires still sees everything read up to that point.
fn read_bounded(mut stream: impl std::io::Read, capture: &Arc<Mutex<StreamCapture>>) {
    const OUTPUT_CAP: usize = 4_096;
    let mut buffer = [0_u8; 1_024];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => return,
            Ok(read) => {
                let mut capture = capture.lock().expect("build capture lock");
                capture.retained.extend(&buffer[..read]);
                while capture.retained.len() > OUTPUT_CAP {
                    capture.retained.pop_front();
                    capture.dropped += 1;
                }
            }
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => {}
            // A failed read is not an end of stream, and reporting it as one
            // would present a truncated build log as the whole story.
            Err(error) => {
                capture.lock().expect("build capture lock").fault =
                    Some(format!("read failed: {error}"));
                return;
            }
        }
    }
}

fn publish_write_once(bytes: &[u8], nested_target: &Path) -> PathBuf {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    std::hash::Hasher::write(&mut hasher, bytes);
    let digest = std::hash::Hasher::finish(&hasher);
    let directory = nested_target.join("write-once");
    std::fs::create_dir_all(&directory).expect("write-once courier directory");
    let published = directory.join(format!("swallowtail-registered-tool-courier-{digest:016x}"));
    if published.is_file() {
        return published;
    }
    let staged = directory.join(format!("staged-{}-{digest:016x}", std::process::id()));
    std::fs::write(&staged, bytes).expect("staged courier is written");
    set_executable(&staged);
    // Hard link, not rename: rename replaces an existing destination, so a
    // concurrent publisher could swap the file another process is about to
    // spawn. Linking fails when the name already exists, which makes the
    // published path literally write-once.
    match std::fs::hard_link(&staged, &published) {
        Ok(()) => {}
        Err(error) if published.is_file() => {
            // A concurrent publisher of identical content won the race.
            let _ = error;
        }
        Err(error) => panic!("published courier could not be installed at {published:?}: {error}"),
    }
    let _ = std::fs::remove_file(&staged);
    published
}

#[cfg(unix)]
fn set_executable(path: &Path) {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o755))
        .expect("published courier is executable");
}

#[cfg(not(unix))]
fn set_executable(_path: &Path) {}

#[cfg(unix)]
fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    std::fs::metadata(path).is_ok_and(|data| data.permissions().mode() & 0o111 != 0)
}

#[cfg(not(unix))]
fn is_executable(path: &Path) -> bool {
    path.is_file()
}

fn tool_id() -> RegisteredToolId {
    RegisteredToolId::new(
        RegisteredToolNamespace::new(NAMESPACE).expect("namespace"),
        RegisteredToolLocalName::new(RECONCILE).expect("local name"),
    )
}

fn schema(digest: &str) -> RegisteredToolSchema {
    RegisteredToolSchema::new(
        RegisteredToolSchemaNamespace::new("desktop.registered-tools.schema").expect("namespace"),
        RegisteredToolSchemaMediaType::new("application/json").expect("media type"),
        RegisteredToolSchemaDialect::new("json-schema-2020-12").expect("dialect"),
        RegisteredServerRevision::new("1").expect("revision"),
        RegisteredToolSchemaDigest::new(digest).expect("digest"),
        RegisteredToolSchemaDocument::new("{\"type\":\"object\"}").expect("document"),
    )
}

fn snapshot(
    host: &ExecutionHostId,
    executable: ExecutableRef,
    environment: EnvironmentRef,
) -> RegisteredToolSnapshot {
    RegisteredToolSnapshot::new(RegisteredToolSnapshotInput {
        server_id: RegisteredServerId::new("desktop.registered-tools").expect("server"),
        revision: RegisteredServerRevision::new("2026-09-07.1").expect("revision"),
        execution_host_id: host.clone(),
        declarations: vec![
            swallowtail_runtime::RegisteredToolDeclaration::new(
                tool_id(),
                RegisteredToolExecutionKind::Mcp,
                schema("sha256:input"),
                schema("sha256:output"),
                RegisteredToolEffectPosture::Mutating,
                RegisteredToolRetryPosture::ConsumerRetryable,
                RegisteredToolBounds::ceiling(),
            )
            .expect("declaration"),
        ],
        transports: vec![
            RegisteredToolTransportSupport::new(
                RegisteredToolTransport::PrivateLoopbackHttp,
                [
                    RegisteredToolProtocolVersion::new(
                        REGISTERED_TOOL_CONFORMANCE_PROTOCOL_VERSION,
                    )
                    .expect("protocol"),
                ],
            )
            .expect("transport"),
        ],
        required_services: [].into_iter().collect(),
        credential_references: Vec::new(),
        executable_recipes: vec![executable],
        environment_recipes: vec![environment],
        bounds: RegisteredToolBounds::ceiling(),
        source: RegisteredToolSource::new(
            RegisteredToolSourceId::new("desktop.registration.1").expect("source"),
            swallowtail_runtime::MonotonicInstant::from_ticks(1),
        ),
    })
    .expect("snapshot")
}

fn preparation_for(
    host: ExecutionHostId,
    admission: swallowtail_runtime::ConsumerAdmissionBinding,
    executable: ExecutableRef,
    environment: EnvironmentRef,
) -> RegisteredToolPreparation {
    let snapshot = Arc::new(snapshot(&host, executable.clone(), environment.clone()));
    let recipe =
        RegisteredToolProxyRecipe::new(executable, environment, REGISTERED_TOOL_PROXY_WIRE_TAG)
            .expect("recipe");
    let selection = RegisteredToolSelection::new(
        Arc::clone(&snapshot),
        [tool_id()],
        RegisteredToolTransport::PrivateLoopbackHttp,
        RegisteredToolProtocolVersion::new(REGISTERED_TOOL_CONFORMANCE_PROTOCOL_VERSION)
            .expect("protocol"),
    )
    .expect("selection")
    .with_attachment(RegisteredToolAttachment::MediatedStdioProxy)
    .with_proxy_recipe(recipe);
    RegisteredToolPreparation::new(
        snapshot,
        selection,
        admission,
        RegisteredToolLimits::ceiling(),
    )
}

struct OpenedRoute {
    session: swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkSessionHandle,
    fixture: SdkFixtureHost,
    local: LocalHostServices,
    services: HostServices,
    courier: CourierClient,
}

fn open_route(
    host: ExecutionHostId,
    dispatcher: Arc<dyn RegisteredToolDispatcher>,
    admission: Arc<ScriptedAdmissionPort>,
) -> OpenedRoute {
    open_route_with_scenario(host, dispatcher, admission, SdkScenario::McpConnected)
}

/// Card 144: opens the registered route against an arbitrary sidecar
/// scenario, so the exact Card 132 registered-open request can be reproduced
/// against the frozen fake sidecar.
fn open_route_with_scenario(
    host: ExecutionHostId,
    dispatcher: Arc<dyn RegisteredToolDispatcher>,
    admission: Arc<ScriptedAdmissionPort>,
    scenario: SdkScenario,
) -> OpenedRoute {
    let fixture = SdkFixtureHost::new(scenario);
    let executable = ExecutableRef::new("fixture.registered-tool.courier").expect("executable");
    let environment = EnvironmentRef::new("fixture.registered-tool.environment").expect("env");
    let local = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_executable(executable.clone(), courier_binary())
        .approve_environment(environment.clone(), [("PATH".into(), "/usr/bin".into())])
        .with_registered_tool_dispatcher(dispatcher)
        .with_registered_tool_clock(Arc::new(fixture.clone()))
        .build_services(host.clone());
    let services = local
        .services()
        .clone()
        .with_process(Arc::new(fixture.clone()))
        .with_credential(Arc::new(fixture.clone()))
        .with_working_resource(Arc::new(fixture.clone()))
        .with_time(Arc::new(fixture.clone()));
    let preparation = preparation_for(
        host.clone(),
        fixture_admission(Arc::clone(&admission)),
        executable,
        environment,
    );
    let prepared = prepare_claude_agent_sdk_session(
        ClaudeAgentSdkSessionPreparation::new(
            ConfiguredInstanceId::new("claude-agent-sdk.fixture").expect("instance"),
            swallowtail_core::InstanceRevision::new("fixture-revision").expect("revision"),
            host,
            swallowtail_core::InstanceTargetRef::new("claude-agent-sdk.fixture.launch-recipe")
                .expect("target"),
            swallowtail_runtime::EnvironmentRef::new("claude-agent-sdk.fixture.environment")
                .expect("environment"),
            swallowtail_core::CredentialRef::new("claude-agent-sdk.fixture.delegated-subscription")
                .expect("credential"),
            swallowtail_core::AccessProfileId::new("claude-agent-sdk.fixture.subscription")
                .expect("access"),
            swallowtail_core::ModelRouteId::new("claude-agent-sdk.fixture.route").expect("route"),
            swallowtail_core::ModelRouteRevision::new("fixture-route-revision").expect("revision"),
            swallowtail_core::ModelId::new("claude-sonnet-5").expect("model"),
            swallowtail_runtime::WorkingResourceRef::new("claude-agent-sdk.fixture.workspace")
                .expect("workspace"),
            swallowtail_runtime::RequestId::new("request-1").expect("request"),
            swallowtail_runtime::Deadline::at(swallowtail_runtime::MonotonicInstant::from_ticks(
                OPEN_DEADLINE_TICKS,
            )),
        )
        .with_registered_tools(preparation, local.clone())
        .expect("openable preparation binds host-resolved courier path and env"),
        swallowtail_runtime::SessionOptions::default(),
    )
    .expect("registered-tool preparation succeeds");
    // Card 139: an open failure reports the courier's own bounded output and
    // exit evidence, so the next occurrence is readable from the failing run.
    let session = block_on(prepared.open_route_session(services.clone())).unwrap_or_else(|error| {
        panic!(
            "registered session did not open: {} ({})\n{}",
            error.diagnostic().code(),
            error.diagnostic().message(),
            fixture.courier_evidence()
        )
    });
    let courier = CourierClient {
        process: fixture
            .spawned_registered_courier()
            .expect("fake SDK spawned the declared courier child"),
    };
    OpenedRoute {
        session,
        fixture,
        local,
        services,
        courier,
    }
}

fn close_route(opened: OpenedRoute) {
    let _ = block_on(Box::new(opened.session).close(cleanup_request(), opened.services));
}

#[test]
fn the_default_open_without_registered_tools_omits_mcp_servers() {
    let host = host_id("claude-agent-sdk.fixture.registered-omit");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let prepared = prepared_session(host.clone());
    assert!(prepared.registered_tools().is_none());
    let services = fixture.services(host);
    let cleanup_services = services.clone();
    let session = block_on(prepared.open_route_session(services)).expect("default session opens");
    let open = fixture
        .inputs()
        .into_iter()
        .find(|input| input["command"] == "open")
        .expect("default open is on the wire");
    assert!(
        open["params"].get("mcpServers").is_none(),
        "omission stays byte-identical: {open}"
    );
    let _ = block_on(Box::new(session).close(cleanup_request(), cleanup_services));
}

#[test]
fn open_without_a_host_composition_fails_typed() {
    let host = host_id("claude-agent-sdk.fixture.registered-host-missing");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let executable = ExecutableRef::new("fixture.registered-tool.courier").expect("executable");
    let environment = EnvironmentRef::new("fixture.registered-tool.environment").expect("env");
    let preparation = preparation_for(
        host.clone(),
        fixture_admission(Arc::new(ScriptedAdmissionPort::current())),
        executable,
        environment,
    );
    let binding = ClaudeAgentSdkSessionProfile::qualify(preparation)
        .expect("qualify does not require the host composition");
    let prepared = prepare_claude_agent_sdk_session(
        crate::sdk_support::preparation(host.clone()).with_registered_tool_binding(binding),
        swallowtail_runtime::SessionOptions::default(),
    )
    .expect("prepared without host");
    let Err(error) = block_on(prepared.open_session(fixture.services(host))) else {
        panic!("open requires the local host composition");
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.claude-agent.sdk.registered_tool.host_missing"
    );
}

#[test]
fn open_declares_the_reserved_courier_and_round_trips_one_mediated_call() {
    let host = host_id("claude-agent-sdk.fixture.registered-round-trip");
    let calls = Arc::new(AtomicUsize::new(0));
    let admission = Arc::new(ScriptedAdmissionPort::current());
    let opened = open_route(
        host,
        Arc::new(CountingDispatcher {
            calls: Arc::clone(&calls),
        }),
        Arc::clone(&admission),
    );
    let open = opened
        .fixture
        .inputs()
        .into_iter()
        .find(|input| input["command"] == "open")
        .expect("registered open is on the wire");
    let servers = open["params"]["mcpServers"]
        .as_array()
        .expect("courier is declared");
    assert_eq!(servers.len(), 1);
    assert_eq!(servers[0]["name"], CLAUDE_AGENT_SDK_REGISTERED_TOOL_SERVER);
    assert_eq!(
        servers[0]["command"].as_str().expect("declared command"),
        courier_binary().to_str().expect("courier path is UTF-8"),
        "SDK command is the resolved filesystem path, not an ExecutableRef host value"
    );
    assert_eq!(servers[0]["args"][0], REGISTERED_TOOL_PROXY_WIRE_TAG);
    let rendezvous = PathBuf::from(servers[0]["args"][1].as_str().expect("rendezvous path"));
    assert!(
        !rendezvous.exists(),
        "ready expires the one-shot rendezvous after the courier authenticates: {rendezvous:?}"
    );
    assert_eq!(servers[0]["env"]["PATH"], "/usr/bin");
    assert_eq!(
        servers[0]["env"].as_object().expect("declared env").len(),
        1,
        "env is the card 084 allowlisted recipe, not a hardcoded PATH dump: {}",
        servers[0]["env"]
    );
    assert!(
        open["params"]["tools"]
            .as_array()
            .expect("tools")
            .iter()
            .any(|tool| tool.as_str()
                == Some("mcp__swallowtail-registered-tools__desktop_reconcile")),
        "admitted tools include the carrier spelling: {}",
        open["params"]["tools"]
    );
    assert_eq!(
        opened.session.mcp_server_status()[0].name(),
        CLAUDE_AGENT_SDK_REGISTERED_TOOL_SERVER
    );
    opened.courier.handshake();
    let called = opened
        .courier
        .call_tool(&tool_id().to_string(), r#"{"path":"workspace/file"}"#);
    assert!(
        called.contains("from-dispatcher"),
        "result correlates back: {called}"
    );
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert!(
        admission
            .observed_phases()
            .contains(&AdmissionPhase::BeforeDispatch),
        "dispatch is kernel-admitted: {:?}",
        admission.observed_phases()
    );
    close_route(opened);
}

#[test]
fn a_revoked_before_dispatch_never_reaches_the_dispatcher() {
    let host = host_id("claude-agent-sdk.fixture.registered-deny");
    let calls = Arc::new(AtomicUsize::new(0));
    let admission = Arc::new(ScriptedAdmissionPort::current());
    admission.revoke_from(AdmissionPhase::BeforeDispatch);
    let opened = open_route(
        host,
        Arc::new(CountingDispatcher {
            calls: Arc::clone(&calls),
        }),
        admission,
    );
    opened.courier.handshake();
    let denied = opened
        .courier
        .call_tool(&tool_id().to_string(), r#"{"path":"workspace/file"}"#);
    assert!(
        !denied.contains("from-dispatcher"),
        "deny does not deliver a result: {denied}"
    );
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    close_route(opened);
}

#[test]
fn cancellation_during_a_registered_call_joins_the_lease() {
    let host = host_id("claude-agent-sdk.fixture.registered-cancel");
    let blocking = Arc::new(BlockingDispatcher {
        entered: Arc::new(AtomicBool::new(false)),
        release: Arc::new(AtomicBool::new(false)),
        waker: Arc::new(Mutex::new(None)),
    });
    let opened = open_route(
        host,
        Arc::clone(&blocking) as Arc<dyn RegisteredToolDispatcher>,
        Arc::new(ScriptedAdmissionPort::current()),
    );
    opened.courier.handshake();
    let courier = Arc::clone(&opened.courier.process);
    let client = CourierClient {
        process: Arc::clone(&courier),
    };
    let call = std::thread::spawn(move || {
        client.try_call_tool(&tool_id().to_string(), r#"{"path":"workspace/file"}"#)
    });
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
    while !blocking.entered.load(Ordering::SeqCst) {
        assert!(
            std::time::Instant::now() < deadline,
            "dispatcher never entered"
        );
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    let local = opened.local.clone();
    close_route(opened);
    blocking.release();
    let _ = call.join();
    assert_eq!(local.registered_tool_lease_count(), 0);
    assert_eq!(local.operation_bridge_listener_count(), 0);
}

#[test]
fn close_joins_the_registered_listener() {
    let host = host_id("claude-agent-sdk.fixture.registered-close");
    let opened = open_route(
        host,
        Arc::new(CountingDispatcher {
            calls: Arc::new(AtomicUsize::new(0)),
        }),
        Arc::new(ScriptedAdmissionPort::current()),
    );
    assert_eq!(opened.local.registered_tool_lease_count(), 1);
    let local = opened.local.clone();
    close_route(opened);
    assert_eq!(local.registered_tool_lease_count(), 0);
    assert_eq!(local.operation_bridge_listener_count(), 0);
}

#[test]
fn ready_follows_kernel_authenticated_connect() {
    let host = host_id("claude-agent-sdk.fixture.registered-ready");
    let opened = open_route(
        host,
        Arc::new(CountingDispatcher {
            calls: Arc::new(AtomicUsize::new(0)),
        }),
        Arc::new(ScriptedAdmissionPort::current()),
    );
    assert_eq!(opened.local.registered_tool_lease_count(), 1);
    assert_eq!(opened.local.operation_bridge_listener_count(), 1);
    assert!(
        opened.fixture.spawned_registered_courier().is_some(),
        "ready is observed after the fake SDK spawned the declared child"
    );
    // The fixture answered `open` only after the courier claimed its one-shot
    // rendezvous, so startup is an observed event and the child is live here.
    let evidence = opened.fixture.courier_evidence();
    assert!(
        evidence.contains("still running"),
        "startup evidence names a live courier: {evidence}"
    );
    close_route(opened);
}

#[test]
fn a_second_courier_cannot_reread_an_expired_rendezvous() {
    let host = host_id("claude-agent-sdk.fixture.registered-second-read");
    let opened = open_route(
        host,
        Arc::new(CountingDispatcher {
            calls: Arc::new(AtomicUsize::new(0)),
        }),
        Arc::new(ScriptedAdmissionPort::current()),
    );
    let open = opened
        .fixture
        .inputs()
        .into_iter()
        .find(|input| input["command"] == "open")
        .expect("registered open is on the wire");
    let rendezvous = open["params"]["mcpServers"][0]["args"][1]
        .as_str()
        .expect("declared rendezvous path");
    assert!(
        !Path::new(rendezvous).exists(),
        "ready unlinks the one-shot rendezvous: {rendezvous}"
    );
    let status = std::process::Command::new(courier_binary())
        .args([REGISTERED_TOOL_PROXY_WIRE_TAG, rendezvous])
        .stderr(std::process::Stdio::null())
        .status()
        .expect("second courier spawn starts");
    assert!(
        !status.success(),
        "a second read of the expired rendezvous must fail"
    );
    close_route(opened);
}

#[test]
fn an_unspawnable_command_fails_typed() {
    let host = host_id("claude-agent-sdk.fixture.registered-unspawnable");
    let fixture = SdkFixtureHost::new(SdkScenario::McpConnected);
    let executable = ExecutableRef::new("fixture.registered-tool.courier").expect("executable");
    let environment = EnvironmentRef::new("fixture.registered-tool.environment").expect("env");
    let missing = std::env::temp_dir().join("swallowtail-card125-missing-courier");
    let local = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_executable(executable.clone(), &missing)
        .approve_environment(environment.clone(), [("PATH".into(), "/usr/bin".into())])
        .with_registered_tool_dispatcher(Arc::new(CountingDispatcher {
            calls: Arc::new(AtomicUsize::new(0)),
        }))
        .with_registered_tool_clock(Arc::new(fixture.clone()))
        .build_services(host.clone());
    let services = local
        .services()
        .clone()
        .with_process(Arc::new(fixture.clone()))
        .with_credential(Arc::new(fixture.clone()))
        .with_working_resource(Arc::new(fixture.clone()))
        .with_time(Arc::new(fixture.clone()));
    let preparation = preparation_for(
        host.clone(),
        fixture_admission(Arc::new(ScriptedAdmissionPort::current())),
        executable,
        environment,
    );
    let prepared = prepare_claude_agent_sdk_session(
        crate::sdk_support::preparation(host)
            .with_registered_tools(preparation, local)
            .expect("unspawnable command still qualifies"),
        swallowtail_runtime::SessionOptions::default(),
    )
    .expect("unspawnable command still prepares");
    let Err(error) = block_on(prepared.open_session(services)) else {
        panic!("open requires a spawnable courier filesystem path");
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.claude-agent.sdk.registered_tool.command_unspawnable"
    );
}

/// A courier that exists but cannot be executed is the smallest setup failure
/// the fixture used to report as a bare `fixture.claude_agent_sdk.failed`.
#[cfg(unix)]
#[test]
fn a_courier_that_cannot_exec_names_its_own_cause() {
    use std::os::unix::fs::PermissionsExt;

    let host = host_id("claude-agent-sdk.fixture.registered-unexecutable");
    let fixture = SdkFixtureHost::new(SdkScenario::McpConnected);
    let executable = ExecutableRef::new("fixture.registered-tool.courier").expect("executable");
    let environment = EnvironmentRef::new("fixture.registered-tool.environment").expect("env");
    // Card 139: one directory per invocation. A machine-wide fixture filename
    // is the shared mutable state this card exists to remove, not to add.
    let scratch = scratch_directory("unexecutable");
    let unexecutable = scratch.join("courier");
    std::fs::write(&unexecutable, b"#!/bin/sh\nexit 0\n").expect("courier stand-in is written");
    std::fs::set_permissions(&unexecutable, std::fs::Permissions::from_mode(0o600))
        .expect("courier stand-in is not executable");
    let local = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_executable(executable.clone(), &unexecutable)
        .approve_environment(environment.clone(), [("PATH".into(), "/usr/bin".into())])
        .with_registered_tool_dispatcher(Arc::new(CountingDispatcher {
            calls: Arc::new(AtomicUsize::new(0)),
        }))
        .with_registered_tool_clock(Arc::new(fixture.clone()))
        .build_services(host.clone());
    let services = local
        .services()
        .clone()
        .with_process(Arc::new(fixture.clone()))
        .with_credential(Arc::new(fixture.clone()))
        .with_working_resource(Arc::new(fixture.clone()))
        .with_time(Arc::new(fixture.clone()));
    let preparation = preparation_for(
        host.clone(),
        fixture_admission(Arc::new(ScriptedAdmissionPort::current())),
        executable,
        environment,
    );
    let prepared = prepare_claude_agent_sdk_session(
        crate::sdk_support::preparation(host)
            .with_registered_tools(preparation, local)
            .expect("an unexecutable courier still qualifies"),
        swallowtail_runtime::SessionOptions::default(),
    )
    .expect("an unexecutable courier still prepares");
    let outcome = block_on(prepared.open_session(services));
    let _ = std::fs::remove_dir_all(&scratch);
    let Err(error) = outcome else {
        panic!("an unexecutable courier cannot open a registered session");
    };
    assert_eq!(
        error.diagnostic().code(),
        "fixture.claude_agent_sdk.registered_courier_startup_failed"
    );
    let message = error.diagnostic().message();
    assert!(
        message.contains(unexecutable.to_str().expect("path is UTF-8")),
        "the failure names the command it tried to start: {message}"
    );
    assert!(
        message.contains("os error 13"),
        "the failure carries the operating system cause: {message}"
    );
    assert!(
        fixture.courier_evidence().contains("os error 13"),
        "the fixture retains the same evidence for the report: {}",
        fixture.courier_evidence()
    );
}

/// One private directory per invocation, so no two fixture runs can remove or
/// overwrite each other's scratch executable.
#[cfg(unix)]
fn scratch_directory(label: &str) -> PathBuf {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("clock is after the epoch")
        .as_nanos();
    let directory = std::env::temp_dir().join(format!(
        "swallowtail-card139-{label}-{}-{unique}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::create_dir_all(&directory).expect("scratch directory is created");
    directory
}

/// A courier that writes to stderr and exits immediately must not be reported
/// as `stderr empty`: observing the exit does not mean the pipe was drained.
#[cfg(unix)]
#[test]
fn a_courier_that_dies_at_startup_reports_its_own_stderr() {
    use std::os::unix::fs::PermissionsExt;

    let host = host_id("claude-agent-sdk.fixture.registered-startup-stderr");
    let fixture = SdkFixtureHost::new(SdkScenario::McpConnected);
    let executable = ExecutableRef::new("fixture.registered-tool.courier").expect("executable");
    let environment = EnvironmentRef::new("fixture.registered-tool.environment").expect("env");
    let scratch = scratch_directory("startup-stderr");
    let dying = scratch.join("courier");
    std::fs::write(
        &dying,
        b"#!/bin/sh\necho 'courier could not reach its rendezvous' >&2\nexit 3\n",
    )
    .expect("dying courier is written");
    std::fs::set_permissions(&dying, std::fs::Permissions::from_mode(0o755))
        .expect("dying courier is executable");
    let local = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_executable(executable.clone(), &dying)
        .approve_environment(environment.clone(), [("PATH".into(), "/usr/bin".into())])
        .with_registered_tool_dispatcher(Arc::new(CountingDispatcher {
            calls: Arc::new(AtomicUsize::new(0)),
        }))
        .with_registered_tool_clock(Arc::new(fixture.clone()))
        .build_services(host.clone());
    let services = local
        .services()
        .clone()
        .with_process(Arc::new(fixture.clone()))
        .with_credential(Arc::new(fixture.clone()))
        .with_working_resource(Arc::new(fixture.clone()))
        .with_time(Arc::new(fixture.clone()));
    let preparation = preparation_for(
        host.clone(),
        fixture_admission(Arc::new(ScriptedAdmissionPort::current())),
        executable,
        environment,
    );
    let prepared = prepare_claude_agent_sdk_session(
        crate::sdk_support::preparation(host)
            .with_registered_tools(preparation, local)
            .expect("a dying courier still qualifies"),
        swallowtail_runtime::SessionOptions::default(),
    )
    .expect("a dying courier still prepares");
    let outcome = block_on(prepared.open_session(services));
    let _ = std::fs::remove_dir_all(&scratch);
    let Err(error) = outcome else {
        panic!("a courier that exits at startup cannot open a registered session");
    };
    assert_eq!(
        error.diagnostic().code(),
        "fixture.claude_agent_sdk.registered_courier_startup_failed"
    );
    let message = error.diagnostic().message();
    assert!(
        message.contains("courier could not reach its rendezvous"),
        "the drained child stderr reaches the failure: {message}"
    );
    assert!(
        message.contains("exit status: 3"),
        "the observed exit reaches the failure: {message}"
    );
}
/// Card 144: the exact Card 132 registered-open request, reproduced
/// provider-free against the frozen fake `0.3.259` sidecar. Research 296
/// froze the live capsule (SHA-256 `e0460a54776a5644ff2c54bc412a52d8
/// 1b0d84f04ee34f56971b78181e501433`) and its tuple — SDK `0.3.259`, native
/// `2.1.259`, Node `22.23.2`, sidecar source tag `0.4.4`, carrier
/// `swallowtail-claude-agent-sdk-registered-tool-mcp-v1`,
/// `private-loopback-http` plus `mediated-stdio-proxy`, MCP `2025-11-25`,
/// model `claude-sonnet-5`, default permission, persistence false, strict
/// MCP configuration, empty setting sources, omitted `allowedTools` — but
/// the capsule could not distinguish the bounded rejection class. The
/// producer receipt can.
#[test]
fn the_card_132_registered_open_request_yields_a_typed_failed_open_receipt() {
    let host = host_id("claude-agent-sdk.fixture.registered-card132");
    let calls = Arc::new(AtomicUsize::new(0));
    let admission = Arc::new(ScriptedAdmissionPort::current());
    let fixture = SdkFixtureHost::new(SdkScenario::OpenRejected);
    let executable = ExecutableRef::new("fixture.registered-tool.courier").expect("executable");
    let environment = EnvironmentRef::new("fixture.registered-tool.environment").expect("env");
    let local = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_executable(executable.clone(), courier_binary())
        .approve_environment(environment.clone(), [("PATH".into(), "/usr/bin".into())])
        .with_registered_tool_dispatcher(Arc::new(CountingDispatcher { calls }))
        .with_registered_tool_clock(Arc::new(fixture.clone()))
        .build_services(host.clone());
    let services = local
        .services()
        .clone()
        .with_process(Arc::new(fixture.clone()))
        .with_credential(Arc::new(fixture.clone()))
        .with_working_resource(Arc::new(fixture.clone()))
        .with_time(Arc::new(fixture.clone()));
    let preparation = preparation_for(
        host.clone(),
        fixture_admission(Arc::clone(&admission)),
        executable,
        environment,
    );
    let prepared = prepare_claude_agent_sdk_session(
        crate::sdk_support::preparation(host)
            .with_registered_tools(preparation, local)
            .expect("registered preparation binds the host"),
        swallowtail_runtime::SessionOptions::default(),
    )
    .expect("registered preparation succeeds");
    let Err(rejection) = block_on(prepared.open_route_session_with_receipt(services)) else {
        panic!("the card 132 reproduction must fail the open");
    };

    // The ordinary failure is unchanged, bounded subcode still in the safe
    // message, and no forbidden material anywhere in it.
    assert_eq!(
        rejection.failure().diagnostic().code(),
        "swallowtail.claude-agent.sdk.open_rejected"
    );
    let message = rejection.failure().diagnostic().message();
    assert!(
        message.ends_with(": construction_failed"),
        "bounded subcode stays in the message: {message}"
    );
    for forbidden in ["/fixture/", "@example", "token", "organization"] {
        assert!(
            !message.contains(forbidden),
            "{forbidden} leaked: {message}"
        );
    }

    // The typed receipt distinguishes what the immutable capsule could not:
    // stage, exact bounded subcode, readiness truth, and cleanup truth.
    let receipt = rejection.receipt();
    assert_eq!(receipt.stage(), ClaudeAgentSdkOpenStage::SidecarRejected);
    assert_eq!(
        receipt.sidecar_code(),
        Some(ClaudeAgentSdkOpenSubcode::ConstructionFailed)
    );
    assert!(!receipt.provider_readiness_reached());
    let cleanup = receipt.cleanup();
    assert!(cleanup.confirmed(), "cleanup joins: {cleanup:?}");
    assert_eq!(cleanup.resource(), &CleanupOutcome::NotApplicable);
    assert_eq!(cleanup.credential(), &CleanupOutcome::Clean);
    assert_eq!(cleanup.registered_lease(), Some(&CleanupOutcome::Clean));
    assert_eq!(
        cleanup.survivor_posture(),
        Some(ProcessTreeCompletion::RootOnly)
    );

    // The wire request carries the Card 132 tuple: one reserved required
    // courier entry, default permission, persistence false, no allowedTools.
    let open = fixture
        .inputs()
        .into_iter()
        .find(|input| input["command"] == "open")
        .expect("registered open is on the wire");
    assert_eq!(open["params"]["model"], "claude-sonnet-5");
    assert_eq!(open["params"]["permissionMode"], "default");
    assert!(
        open["params"]
            .get("persistSession")
            .is_none_or(|value| value.as_bool() == Some(false)),
        "card 132 ran persistence false: {}",
        open["params"]
    );
    assert!(
        open["params"].get("allowedTools").is_none(),
        "allowedTools is never set: {}",
        open["params"]
    );
    let servers = open["params"]["mcpServers"]
        .as_array()
        .expect("courier is declared");
    assert_eq!(servers.len(), 1);
    assert_eq!(servers[0]["name"], CLAUDE_AGENT_SDK_REGISTERED_TOOL_SERVER);
    assert_eq!(servers[0]["optional"], false);
    assert!(
        open["params"]["tools"]
            .as_array()
            .expect("tools")
            .iter()
            .any(|tool| tool.as_str()
                == Some("mcp__swallowtail-registered-tools__desktop_reconcile")),
        "admitted tools include the carrier spelling: {}",
        open["params"]["tools"]
    );
}

/// Deterministic proofs for the capture lifecycle itself.
///
/// Card 139: every defect three review rounds found lived here, in code that
/// only runs when something else has already gone wrong, and none of it was
/// reachable from a failing route test. These drive the fault paths directly.
mod capture_lifecycle {
    use super::{StreamCapture, collect_bounded, read_bounded};
    use std::io::{Error, ErrorKind, Read};
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    /// A reader that replays a fixed script of read outcomes.
    struct ScriptedReader(Vec<Result<Vec<u8>, ErrorKind>>);

    impl Read for ScriptedReader {
        fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
            match self.0.pop() {
                None => Ok(0),
                Some(Ok(bytes)) => {
                    buffer[..bytes.len()].copy_from_slice(&bytes);
                    Ok(bytes.len())
                }
                Some(Err(kind)) => Err(Error::new(kind, "scripted read outcome")),
            }
        }
    }

    fn scripted(mut outcomes: Vec<Result<Vec<u8>, ErrorKind>>) -> ScriptedReader {
        outcomes.reverse();
        ScriptedReader(outcomes)
    }

    #[test]
    fn an_interrupted_read_is_retried_rather_than_ending_the_stream() {
        let capture = Arc::new(Mutex::new(StreamCapture::default()));
        read_bounded(
            scripted(vec![
                Ok(b"progress".to_vec()),
                Err(ErrorKind::Interrupted),
                Ok(b" then the diagnostic".to_vec()),
            ]),
            &capture,
        );
        let capture = capture.lock().expect("capture");
        assert_eq!(capture.describe(), "progress then the diagnostic");
        assert!(
            capture.fault.is_none(),
            "an interruption is not a capture fault"
        );
    }

    #[test]
    fn a_failed_read_is_not_an_end_of_stream() {
        let capture = Arc::new(Mutex::new(StreamCapture::default()));
        read_bounded(
            scripted(vec![
                Ok(b"what was read".to_vec()),
                Err(ErrorKind::BrokenPipe),
            ]),
            &capture,
        );
        let capture = capture.lock().expect("capture");
        assert_eq!(capture.describe(), "what was read");
        let fault = capture.fault.as_ref().expect("a failed read is a fault");
        assert!(
            fault.contains("read failed"),
            "the fault names itself: {fault}"
        );
    }

    #[test]
    fn an_expired_collection_still_reports_what_was_already_read() {
        let capture = Arc::new(Mutex::new(StreamCapture::default()));
        read_bounded(
            scripted(vec![Ok(b"ROOT CAUSE: it failed here".to_vec())]),
            &capture,
        );
        // The reader never reports: a descendant still holds the pipe.
        let (_send, recv) = std::sync::mpsc::channel();
        let collected = collect_bounded(
            &recv,
            &capture,
            "stderr",
            std::time::Instant::now() + Duration::from_millis(50),
        );
        assert!(
            collected.contains("ROOT CAUSE: it failed here"),
            "an expired bound keeps the diagnostic it already had: {collected}"
        );
        assert!(
            collected.contains("still open"),
            "and says the capture is incomplete: {collected}"
        );
    }

    /// A reader still blocked on its next chunk must not hide what it has
    /// already read. This is the case that would catch a return to buffering
    /// privately until end of stream, which the expired-collection test above
    /// cannot: that one finishes reading before collection begins.
    ///
    /// The reader announces that it has entered its second read before the
    /// collection starts, so this proves the property rather than the
    /// scheduler. A card about removing timing dependence has no business
    /// introducing one.
    #[test]
    fn a_blocked_reader_does_not_withhold_what_it_already_read() {
        struct GatedReader {
            delivered: bool,
            entered: std::sync::mpsc::Sender<()>,
            release: Arc<(Mutex<bool>, std::sync::Condvar)>,
        }

        impl Read for GatedReader {
            fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
                if !self.delivered {
                    self.delivered = true;
                    let bytes = b"ROOT CAUSE: read before the block";
                    buffer[..bytes.len()].copy_from_slice(bytes);
                    return Ok(bytes.len());
                }
                let (lock, changed) = &*self.release;
                let mut released = lock.lock().expect("gate");
                // Announced under the gate lock, so the waiter cannot observe
                // entry before this read can be released.
                let _ = self.entered.send(());
                while !*released {
                    released = changed.wait(released).expect("gate wait");
                }
                Ok(0)
            }
        }

        let capture = Arc::new(Mutex::new(StreamCapture::default()));
        let release = Arc::new((Mutex::new(false), std::sync::Condvar::new()));
        let (send_entered, entered) = std::sync::mpsc::channel();
        let (send_done, done) = std::sync::mpsc::channel();
        let reader_capture = Arc::clone(&capture);
        let reader_release = Arc::clone(&release);
        let reader = std::thread::spawn(move || {
            read_bounded(
                GatedReader {
                    delivered: false,
                    entered: send_entered,
                    release: reader_release,
                },
                &reader_capture,
            );
            send_done.send(())
        });
        entered
            .recv_timeout(Duration::from_secs(30))
            .expect("the reader delivers its chunk and enters its second read");
        // Collect while the reader is known to be blocked on that second read.
        let collected = collect_bounded(
            &done,
            &capture,
            "stderr",
            std::time::Instant::now() + Duration::from_millis(50),
        );
        // Release and join before asserting: a failing assertion must not
        // strand the reader on its gate.
        let (lock, changed) = &*release;
        *lock.lock().expect("gate") = true;
        changed.notify_all();
        reader
            .join()
            .expect("reader thread ends")
            .expect("reader reports");
        assert!(
            collected.contains("ROOT CAUSE: read before the block"),
            "a blocked reader's earlier output is still reported: {collected}"
        );
        assert!(
            collected.contains("still open"),
            "and the capture is marked incomplete: {collected}"
        );
    }

    #[test]
    fn a_vanished_reader_is_not_reported_as_an_open_pipe() {
        let capture = Arc::new(Mutex::new(StreamCapture::default()));
        read_bounded(scripted(vec![Ok(b"partial".to_vec())]), &capture);
        let (send, recv) = std::sync::mpsc::channel::<()>();
        drop(send);
        let collected = collect_bounded(
            &recv,
            &capture,
            "stdout",
            std::time::Instant::now() + Duration::from_secs(30),
        );
        assert!(
            collected.contains("ended without reporting"),
            "a dropped reader is named as such, not as a live stream: {collected}"
        );
        assert!(
            !collected.contains("still open"),
            "and is never reported as an open pipe: {collected}"
        );
    }
}
