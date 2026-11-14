//! ACP-specific source/fake proof; no kernel or original/native qualification.
use super::*;
use serde_json::{Value, json};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use swallowtail_adapter_claude_agent::ClaudeAgentPreparedSession;
use swallowtail_runtime::{
    CallbackPayload, CallbackResponse, CallbackResult, HostServices, InteractiveSessionHandle,
};

struct OwnedEndpoint {
    port: u16,
    stop: Arc<AtomicBool>,
    requests: Arc<AtomicUsize>,
    thread: Option<std::thread::JoinHandle<()>>,
}
impl OwnedEndpoint {
    fn new() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("owned endpoint bind");
        let port = listener
            .local_addr()
            .expect("owned endpoint address")
            .port();
        listener
            .set_nonblocking(true)
            .expect("nonblocking endpoint");
        let stop = Arc::new(AtomicBool::new(false));
        let requests = Arc::new(AtomicUsize::new(0));
        let worker_stop = stop.clone();
        let observed = requests.clone();
        let thread = std::thread::spawn(move || {
            while !worker_stop.load(Ordering::Acquire) {
                match listener.accept() {
                    Ok((mut connection, _)) => {
                        connection
                            .set_nonblocking(false)
                            .expect("blocking accepted stream");
                        connection
                            .set_read_timeout(Some(Duration::from_secs(2)))
                            .expect("owned read bound");
                        connection
                            .set_write_timeout(Some(Duration::from_secs(2)))
                            .expect("owned write bound");
                        let mut bytes = [0; 4096];
                        let count = connection.read(&mut bytes).expect("owned request");
                        assert!(bytes[..count].starts_with(b"POST /turn/"));
                        observed.fetch_add(1, Ordering::AcqRel);
                        let body: &[u8] = if bytes[..count].starts_with(b"POST /turn/denied ") {
                            br#"{"text":"The read was denied; continuing."}"#
                        } else {
                            br#"{"text":"Read allowed; continuing."}"#
                        };
                        write!(
                            connection,
                            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                            body.len()
                        )
                        .expect("owned response header");
                        connection.write_all(body).expect("owned response body");
                    }
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        std::thread::sleep(Duration::from_millis(2))
                    }
                    Err(error) => panic!("owned endpoint failed: {error}"),
                }
            }
        });
        Self {
            port,
            stop,
            requests,
            thread: Some(thread),
        }
    }
}
impl Drop for OwnedEndpoint {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Release);
        let joined = self.thread.take().expect("owned endpoint thread").join();
        if !std::thread::panicking() {
            joined.expect("owned endpoint joined");
        }
    }
}

fn graph(root: &Path) -> LocalCapturedGraph {
    let (node, native) = runtime_for_fixture(root);
    let sdk = include_str!("guard_full_route/sdk.mjs")
        .replace(
            "__NATIVE_PATH__",
            &serde_json::to_string(&node).expect("owned native path"),
        )
        .replace(
            "__NATIVE_PROGRAM__",
            &serde_json::to_string(include_str!("guard_full_route/native.cjs"))
                .expect("owned native source"),
        );
    let inputs = [
        ("entry.mjs", include_str!("guard_full_route/entry.mjs")),
        ("idle.mjs", crate::IDLE),
        ("sdk.mjs", sdk.as_str()),
    ];
    let mut files = serde_json::Map::new();
    for (name, source) in inputs {
        std::fs::write(root.join(name), source).expect("owned source");
        files.insert(
            name.into(),
            json!({"format":"module","sha256":format!("{:x}",Sha256::digest(source.as_bytes()))}),
        );
    }
    let native_record = json!({"sha256":format!("{:x}",Sha256::digest(std::fs::read(&node).expect("owned native runtime"))),"bytes":std::fs::metadata(&node).expect("runtime metadata").len(),"specifier":"@owned/native"});
    let manifest = json!({"vendor":"11".repeat(32),"entry":"entry.mjs","idle":"idle.mjs","files":files,
      "imports":[{"parent":"entry.mjs","specifier":"./idle.mjs","target":"idle.mjs"},{"parent":"entry.mjs","specifier":"./sdk.mjs","target":"sdk.mjs"}],
      "builtins":[{"parent":"entry.mjs","specifier":"node:fs","target":"node:fs"},{"parent":"sdk.mjs","specifier":"node:fs","target":"node:fs"},{"parent":"sdk.mjs","specifier":"node:child_process","target":"node:child_process"}],
      "require_parents":[],"native_parents":["sdk.mjs"],"native":{"darwin-arm64":native_record,"linux-x64":native_record},"denied_dynamic":[],"source_cautions":[],"platform_programs":[]});
    LocalCapturedGraph::capture_manifest(
        root,
        &serde_json::to_vec(&manifest).expect("owned manifest"),
        if cfg!(target_os = "macos") {
            "darwin-arm64"
        } else {
            "linux-x64"
        },
        &native,
        None,
    )
    .expect("owned graph admission")
}
fn trace(root: &Path) -> Vec<Value> {
    std::fs::read_to_string(root.join("trace.jsonl"))
        .unwrap_or_default()
        .lines()
        .map(|line| serde_json::from_str(line).expect("owned trace record"))
        .collect()
}
fn wait_phase(root: &Path, phase: &str) {
    let deadline = Instant::now() + Duration::from_secs(8);
    while !trace(root).iter().any(|row| row["phase"] == phase) {
        assert!(
            Instant::now() < deadline,
            "phase {phase} not observed: {:?}",
            trace(root)
        );
        std::thread::sleep(Duration::from_millis(5));
    }
}
fn prepare(
    root: &Path,
    endpoint: &OwnedEndpoint,
    settings: Value,
) -> (ClaudeAgentPreparedSession, HostServices, FixtureHost) {
    std::fs::write(
        root.join("settings.json"),
        serde_json::to_vec(&settings).expect("owned settings"),
    )
    .expect("owned settings file");
    std::fs::write(root.join("tool.txt"), "owned tool content").expect("owned tool fixture");
    let fake = fake_child_with_environment(
        root,
        graph(root),
        8 * 1024 * 1024,
        vec![
            ("SWALLOWTAIL_FAKE_ROOT".into(), root.as_os_str().to_owned()),
            (
                "SWALLOWTAIL_FAKE_ENDPOINT_PORT".into(),
                endpoint.port.to_string().into(),
            ),
        ],
    );
    let host = ExecutionHostId::new("owned.full-route").expect("host");
    let resources = FixtureHost::new(Scenario::Success, "0.106.0");
    let services = resources
        .services(host.clone())
        .with_process(Arc::new(fake.local));
    let prepared = block_on(prepare_claude_agent(
        preparation(host).with_checked_process(
            CheckedProcessRef::new("fixture.checked.binding").expect("binding"),
        ),
        ClaudeAgentPreparationProbe::new(
            RequestId::new("full-route-probe").expect("request"),
            ScopeId::new("full-route-probe").expect("scope"),
            Deadline::at(MonotonicInstant::from_ticks(100)),
            DiscoveryCancellation::new(),
        ),
        services.clone(),
    ))
    .expect("owned preparation");
    assert!(!prepared.observation().is_qualified());
    let profile = prepared
        .prepare_session(
            ClaudeAgentSessionProfileInput::new(
                RequestId::new("full-route-open").expect("request"),
                ClaudeAgentModelSelection::new(
                    ModelRouteId::new("owned.route").expect("route"),
                    ModelRouteRevision::new("1").expect("revision"),
                    ModelId::new("claude-sonnet-4-6").expect("model"),
                ),
                WorkingResourceRef::new("checked.workspace").expect("resource"),
                SessionOptions::default(),
            )
            .with_consumer_mediated_permissions(),
        )
        .expect("owned session profile");
    (profile, services, resources)
}
fn turn(
    session: &mut dyn InteractiveSessionHandle,
    services: HostServices,
    root: &Path,
    callback_expected: bool,
    cancel: bool,
) {
    let prior_reads = trace(root)
        .iter()
        .filter(|row| row["phase"] == "tool-read")
        .count();
    let mut turn = block_on(
        session.start_turn(
            TurnRequest::new(
                RuntimeTurnId::new("owned.turn").expect("turn"),
                OperationContent::new(if cancel { "hold" } else { "read owned fixture" })
                    .expect("prompt"),
            ),
            services,
        ),
    )
    .expect("owned turn");
    if cancel {
        wait_phase(root, "native-prompt");
        block_on(turn.cancellation().request()).expect("cancel request");
    } else if callback_expected {
        let mut callbacks = turn.take_callbacks().expect("callback exchange");
        let mut requests = callbacks.take_requests().expect("callback stream");
        let request = block_on(requests.next())
            .expect("real callback")
            .expect("valid callback");
        assert!(
            trace(root)
                .iter()
                .filter(|row| row["phase"] == "tool-read")
                .count()
                == prior_reads,
            "tool effect preceded permission"
        );
        block_on(
            callbacks.responder().respond(CallbackResponse::new(
                request.callback_id().clone(),
                request.turn_id().expect("callback turn").clone(),
                CallbackResult::Success(
                    CallbackPayload::new(br#"{"optionId":"allow-once"}"#, 256)
                        .expect("bounded response"),
                ),
            )),
        )
        .expect("permission response");
    }
    let mut events = turn.take_events().expect("event stream");
    let terminal = turn.take_terminal_outcome().expect("terminal");
    let (events, outcome) = block_on(async {
        let mut collected = vec![];
        while let Some(event) = events.next().await {
            collected.push(event.expect("runtime event"));
        }
        (collected, terminal.await)
    });
    assert_eq!(
        outcome.status(),
        if cancel {
            &TerminalStatus::Cancelled
        } else {
            &TerminalStatus::Completed
        }
    );
    if !cancel {
        assert_eq!(
            outcome.output().expect("continued output").as_str(),
            if callback_expected {
                "Read allowed; continuing."
            } else {
                "The read was denied; continuing."
            }
        );
        let at = |predicate: fn(&swallowtail_runtime::RuntimeEventKind) -> bool| {
            events
                .iter()
                .position(|event| predicate(event.kind()))
                .expect("ordered event")
        };
        assert!(
            at(|kind| matches!(kind, swallowtail_runtime::RuntimeEventKind::Activity(_)))
                < at(|kind| matches!(kind, swallowtail_runtime::RuntimeEventKind::OutputDelta))
        );
        assert_eq!(
            events
                .iter()
                .filter(|event| matches!(
                    event.kind(),
                    swallowtail_runtime::RuntimeEventKind::CallbackRequested(_)
                ))
                .count(),
            usize::from(callback_expected)
        );
    }
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
}
fn close_after_turn(
    mut session: Box<dyn InteractiveSessionHandle>,
    services: HostServices,
    resources: &FixtureHost,
    root: &Path,
    callback_expected: bool,
    cancel: bool,
) {
    let observed = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        turn(
            session.as_mut(),
            services.clone(),
            root,
            callback_expected,
            cancel,
        )
    }));
    let cleanup = block_on(session.close(resources.cleanup_request(), services));
    assert_eq!(cleanup, CleanupOutcome::Clean);
    if let Err(failure) = observed {
        std::panic::resume_unwind(failure);
    }
}

#[test]
fn actual_prepared_new_load_resume_use_sdk_native_endpoint_and_settings_fakes() {
    for (user_denies, managed, callback_expected) in [
        (false, None, true),
        (false, Some(false), true),
        (false, Some(true), false),
        (true, None, false),
    ] {
        let root = fixture();
        let endpoint = OwnedEndpoint::new();
        let settings = json!({"user":{"permissions":{"denyInsteadOfPrompting":user_denies}},"managed":managed.map(|deny|json!({"permissions":{"denyInsteadOfPrompting":deny}}))});
        let (profile, services, resources) = prepare(&root, &endpoint, settings.clone());
        let opened = block_on(profile.open_session(services.clone())).expect("actual new session");
        let binding = opened.resume_binding().expect("resume binding").clone();
        close_after_turn(
            opened,
            services.clone(),
            &resources,
            &root,
            callback_expected,
            false,
        );
        let loaded = block_on(
            profile
                .load_session(
                    RequestId::new("owned.load").expect("request"),
                    binding.clone(),
                    services.clone(),
                )
                .expect("load derives"),
        )
        .expect("actual loaded session");
        let (_, loaded) = loaded.into_parts();
        close_after_turn(
            loaded,
            services.clone(),
            &resources,
            &root,
            callback_expected,
            false,
        );
        let resumed = block_on(
            profile
                .resume_session(
                    RequestId::new("owned.resume").expect("request"),
                    binding,
                    services.clone(),
                )
                .expect("resume derives"),
        )
        .expect("actual resumed session");
        close_after_turn(
            resumed,
            services.clone(),
            &resources,
            &root,
            callback_expected,
            false,
        );
        wait_phase(&root, "sdk-joined");
        let rows = trace(&root);
        for method in ["session/new", "session/load", "session/resume"] {
            assert!(
                rows.iter()
                    .any(|row| row["phase"] == "acp-method" && row["method"] == method)
            );
        }
        assert_eq!(
            rows.iter()
                .filter(|row| row["phase"] == "sdk-can-use-tool")
                .count(),
            if callback_expected { 3 } else { 0 }
        );
        assert_eq!(
            rows.iter()
                .filter(|row| row["phase"] == "tool-read")
                .count(),
            if callback_expected { 3 } else { 0 }
        );
        assert_eq!(
            rows.iter()
                .filter(|row| row["phase"] == "sdk-native-wait" && row["code"] == 0)
                .count(),
            3
        );
        assert_eq!(endpoint.requests.load(Ordering::Acquire), 3);
        assert_eq!(
            serde_json::from_slice::<Value>(
                &std::fs::read(root.join("settings.json")).expect("settings retained")
            )
            .expect("settings JSON"),
            settings
        );
        assert_eq!(resources.resource_releases(), 3);
        drop(endpoint);
        std::fs::remove_dir_all(root).expect("owned fixture cleanup");
    }
}
#[test]
fn actual_prepared_cancel_stops_owned_native_and_joins_session_cleanup() {
    let root = fixture();
    let endpoint = OwnedEndpoint::new();
    let (profile, services, resources) = prepare(
        &root,
        &endpoint,
        json!({"managed":{"permissions":{"denyInsteadOfPrompting":true}}}),
    );
    let session = block_on(profile.open_session(services.clone())).expect("actual new session");
    close_after_turn(session, services, &resources, &root, false, true);
    wait_phase(&root, "sdk-joined");
    let rows = trace(&root);
    assert!(rows.iter().any(|row| row["phase"] == "native-cancel"));
    assert!(
        rows.iter()
            .any(|row| row["phase"] == "sdk-native-wait" && row["code"] == 0)
    );
    assert!(!rows.iter().any(|row| row["phase"] == "tool-read"));
    assert_eq!(endpoint.requests.load(Ordering::Acquire), 0);
    drop(endpoint);
    std::fs::remove_dir_all(root).expect("owned fixture cleanup");
}
