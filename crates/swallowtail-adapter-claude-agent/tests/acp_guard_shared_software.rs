//! Actual LocalProcessHost siblings bound to exact reviewed software entry helpers.
use super::*;
#[path = "acp_guard_scope.rs"]
mod scope;
use scope::FixtureScope;
use std::io::{BufRead, BufReader};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{Receiver, channel};

const SUPERVISOR: &str = include_str!("guard_shared_software/host.py");
const SHARED_LOADER: &str = include_str!("guard_shared_software/reviewed/loader.mjs");
const SHARED_ENTRY: &str = include_str!("guard_shared_software/reviewed/entry.py");

struct Supervisor {
    child: Child,
    input: ChildStdin,
    responses: Receiver<Value>,
    reader: Option<std::thread::JoinHandle<()>>,
    errors: Option<std::thread::JoinHandle<Vec<u8>>>,
}
impl Supervisor {
    fn start(python: &Path, root: &Path, config: &Value) -> Self {
        std::fs::write(
            root.join("supervisor.json"),
            serde_json::to_vec(config).expect("host config"),
        )
        .expect("host config file");
        let script = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/guard_shared_software/host.py")
            .canonicalize()
            .expect("host adapter path");
        assert_eq!(
            std::fs::read(&script).expect("host adapter bytes"),
            SUPERVISOR.as_bytes()
        );
        let mut child = Command::new(python)
            .args(["-I", "-B", "-S"])
            .arg(script)
            .arg(root.join("supervisor.json"))
            .env_clear()
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("owned software host");
        let input = child.stdin.take().expect("host control pipe");
        let output = child.stdout.take().expect("host receipts");
        let stderr = child.stderr.take().expect("host diagnostics");
        let (send, responses) = channel();
        let reader = std::thread::spawn(move || {
            let mut stream = BufReader::new(output);
            loop {
                let mut line = String::new();
                let count = stream.read_line(&mut line).expect("software receipt IO");
                if count == 0 {
                    break;
                }
                assert!(line.len() <= 1024 * 1024, "software receipt limit");
                let value = serde_json::from_str(&line).expect("software receipt JSON");
                if send.send(value).is_err() {
                    break;
                }
            }
        });
        let errors = std::thread::spawn(move || {
            let mut bytes = vec![];
            stderr
                .take(1024 * 1024)
                .read_to_end(&mut bytes)
                .expect("bounded software diagnostics");
            bytes
        });
        let result = Self {
            child,
            input,
            responses,
            reader: Some(reader),
            errors: Some(errors),
        };
        assert_eq!(
            result
                .responses
                .recv_timeout(Duration::from_secs(4))
                .expect("software host startup")["kind"],
            "ready"
        );
        result
    }
    fn control(&mut self, value: Value) -> Value {
        writeln!(self.input, "{value}").expect("private host control");
        self.input.flush().expect("host control flush");
        let result = self
            .responses
            .recv_timeout(Duration::from_secs(4))
            .expect("software host response");
        assert_eq!(result["reply"], value["op"]);
        result
    }
    fn arm(&mut self, context: &str, hold: bool) {
        self.control(json!({"op":"arm","context":context,"hold":hold}));
    }
    fn settle(&mut self, root: &Path) -> Value {
        let waits: Vec<_> = trace(root)
            .into_iter()
            .filter(|row| row["phase"] == "sdk-native-wait")
            .collect();
        self.control(json!({"op":"settle","waits":waits}))
    }
}
impl Drop for Supervisor {
    fn drop(&mut self) {
        let _ = writeln!(self.input, "{{\"op\":\"stop\"}}");
        let _ = self.input.flush();
        let deadline = Instant::now() + Duration::from_secs(4);
        loop {
            match self.child.try_wait() {
                Ok(Some(exit)) => {
                    if !std::thread::panicking() {
                        assert!(exit.success(), "software host failed: {exit:?}");
                    }
                    break;
                }
                Ok(None) if Instant::now() < deadline => {
                    std::thread::sleep(Duration::from_millis(10))
                }
                _ => {
                    let _ = self.child.kill();
                    let _ = self.child.wait();
                    break;
                }
            }
        }
        if let Some(reader) = self.reader.take() {
            let _ = reader.join();
        }
        if let Some(errors) = self.errors.take() {
            let data = errors.join().expect("software diagnostics joined");
            if !data.is_empty() {
                eprintln!("{}", String::from_utf8_lossy(&data));
            }
        }
    }
}

fn strict_loader(root: &Path, args: &[String]) -> String {
    // Reuse the exact reviewed facade body, preserving ACP's captured loading
    // and idle mutation. This host-owned recipe has its own compound identity.
    let start = SHARED_LOADER
        .find("function childFacade(binding, caller) {")
        .expect("reviewed facade anchor");
    let facade = &SHARED_LOADER[start..];
    let constants =
        json!({"argv":args,"cwd":root,"env":{},"callers":["file:///swallowtail-capture/sdk.mjs"]});
    let call = format!(
        "childFacade({{...native,uid:String(native.uid),bytes:String(native.bytes),ancestors:native.ancestors.map(p=>({{...p,uid:String(p.uid),mode:String(p.mode)}})),...{constants}}}, childFacades.get(url))"
    );
    let source = CLAUDE_AGENT_ACP_GUARD_LOADER_SOURCE;
    assert_eq!(
        source
            .matches("childFacade(native, platformPrograms.get(childFacades.get(url)))")
            .count(),
        1
    );
    assert_eq!(source.matches("function childFacade(").count(), 1);
    format!(
        "{}\n{facade}",
        source
            .replace("function childFacade(", "function originalAcpChildFacade(")
            .replace(
                "childFacade(native, platformPrograms.get(childFacades.get(url)))",
                &call
            )
    )
}

fn setup(
    root: &Path,
    endpoint: &OwnedEndpoint,
    settings: Value,
    wrong_argv: bool,
) -> (
    ClaudeAgentPreparedSession,
    HostServices,
    FixtureHost,
    CheckedChild,
    Supervisor,
    FixtureScope,
) {
    let record: Value = serde_json::from_slice(
        &std::fs::read(
            std::env::var_os("SWALLOWTAIL_ACP_SHARED_RUNTIME_RECORD")
                .expect("explicit software host record"),
        )
        .expect("host record"),
    )
    .expect("host JSON");
    let node = PathBuf::from(record["node"].as_str().expect("Node path"));
    let python = PathBuf::from(record["python"]["path"].as_str().expect("Python path"));
    for (name, bytes) in [
        ("entry.py", SHARED_ENTRY.as_bytes()),
        ("captured-loader.mjs", SHARED_LOADER.as_bytes()),
        (
            "bootstrap-native.mjs",
            include_bytes!("guard_shared_software/reviewed/bootstrap-native.mjs").as_slice(),
        ),
        (
            "native.cjs",
            include_bytes!("guard_full_route/native.cjs").as_slice(),
        ),
    ] {
        std::fs::write(root.join(name), bytes).expect("owned captured source");
    }
    let native_file = root.join("native.cjs");
    let source = std::fs::read_to_string(&native_file).expect("owned native source");
    let native_graph = json!({"files":[{"url":format!("file://{}",native_file.display()),"source":source,"format":"commonjs"}],"edges":[{"parent":null,"specifier":format!("file://{}",native_file.display()),"target":format!("file://{}",native_file.display())},{"parent":format!("file://{}",native_file.display()),"specifier":"node:fs","target":"node:fs"},{"parent":format!("file://{}",native_file.display()),"specifier":"node:http","target":"node:http"}],"native":null,"requireParents":[],"platform":format!("{}-{}",std::env::consts::OS,std::env::consts::ARCH).replace("macos-aarch64","darwin-arm64")});
    std::fs::write(
        root.join("native-graph.json"),
        serde_json::to_vec(&native_graph).expect("native graph"),
    )
    .expect("native graph file");
    std::fs::write(
        root.join("settings.json"),
        serde_json::to_vec(&settings).expect("settings"),
    )
    .expect("settings file");
    std::fs::write(root.join("tool.txt"), "owned tool").expect("owned tool");
    let entry_args = vec![
        "-I".to_owned(),
        "-B".into(),
        "-S".into(),
        root.join("entry.py").to_str().expect("entry path").into(),
        root.join("native-entry.json")
            .to_str()
            .expect("config path")
            .into(),
        "native".into(),
    ];
    let supplied = if wrong_argv {
        let mut changed = entry_args.clone();
        changed.push("unapproved".into());
        changed
    } else {
        entry_args.clone()
    };
    let sdk = include_str!("guard_shared_software/sdk.mjs")
        .replace(
            "__NATIVE_PATH__",
            &serde_json::to_string(&python).expect("Python literal"),
        )
        .replace(
            "__ENTRY_ARGUMENTS__",
            &serde_json::to_string(&supplied).expect("entry argv"),
        );
    let ids = |name: &str| {
        record[name]
            .as_array()
            .expect("host principals")
            .iter()
            .map(|v| u32::try_from(v.as_u64().expect("principal")).expect("principal width"))
            .collect()
    };
    let python_bytes = std::fs::read(&python).expect("Python source");
    let native = LocalTrustedRuntime::capture_with_trust(
        &python,
        Sha256::digest(&python_bytes).into(),
        &[],
        LocalRuntimeTrust::new(ids("trustedUsers"), ids("trustedGroups")).expect("host trust"),
    )
    .expect("held owned Python entry runtime");
    let graph = capture_route_graph(
        root,
        include_str!("guard_full_route/entry.mjs"),
        &sdk,
        &native,
        python_bytes.len() as u64,
    );
    let loader = strict_loader(root, &entry_args);
    let fake = fake_child_with_loader(
        root,
        graph,
        8 * 1024 * 1024,
        vec![
            ("SWALLOWTAIL_FAKE_ROOT".into(), root.as_os_str().to_owned()),
            (
                "SWALLOWTAIL_FAKE_ENDPOINT_PORT".into(),
                endpoint.port.to_string().into(),
            ),
        ],
        &loader,
    );
    let native_command = vec![
        node.to_str().expect("Node path").to_owned(),
        "--no-addons".into(),
        "--openssl-config=/dev/null".into(),
        "--disallow-code-generation-from-strings".into(),
        "--import".into(),
        root.join("bootstrap-native.mjs")
            .to_str()
            .expect("bootstrap")
            .into(),
        native_file.to_str().expect("native file").into(),
        root.to_str().expect("root").into(),
        endpoint.port.to_string(),
    ];
    let entry_command: Vec<_> = std::iter::once(python.to_str().expect("Python path").to_owned())
        .chain(entry_args)
        .collect();
    let mut sources = vec![];
    for name in [
        "entry.py",
        "captured-loader.mjs",
        "bootstrap-native.mjs",
        "native.cjs",
        "native-graph.json",
        "settings.json",
        "entry.mjs",
        "sdk.mjs",
        "idle.mjs",
    ] {
        let file = root.join(name);
        sources.push(json!({"path":file,"sha256":format!("{:x}",Sha256::digest(std::fs::read(&file).expect("host source")))}));
    }
    sources.push(json!({"path":node,"sha256":record["sha256"]}));
    sources.push(json!({"path":python,"sha256":record["python"]["sha256"]}));
    for row in record["dependencies"].as_array().expect("runtime closure") {
        sources.push(row.clone());
    }
    let config = json!({"schema":"swallowtail.acp227.shared-software.v1","originalActivation":false,"preparedOwnerPid":std::process::id(),"preparedOwnerCommand":std::env::args().collect::<Vec<_>>(),"root":root,"driverCommand":fake.command,"entryCommand":entry_command,"entrySourceSha256":format!("{:x}",Sha256::digest(SHARED_ENTRY.as_bytes())),"pythonSha256":record["python"]["sha256"],"nodeSha256":record["sha256"],"nativeCommand":native_command,"nativePath":native_file,"nativeSha256":format!("{:x}",Sha256::digest(std::fs::read(&native_file).expect("native bytes"))),"nativeEnv":{},"token":format!("owned-{}-{}",std::process::id(),root.display()),"sources":sources});
    let supervisor = Supervisor::start(&python, root, &config);
    // Keep independent host capabilities available for revocation tests.
    let cloned = CheckedChild {
        local: fake.local.clone(),
        runtime: fake.runtime.clone(),
        approval: fake.approval.clone(),
        command: fake.command.clone(),
    };
    let (scope, process) = FixtureScope::new(fake.local);
    let (profile, services, resources) = prepare_with_process(process);
    (profile, services, resources, cloned, supervisor, scope)
}

fn validate_actors(value: &Value, root: &Path, expected: usize) {
    let registered: Vec<_> = value["events"]
        .as_array()
        .expect("host events")
        .iter()
        .filter(|row| row["kind"] == "native_registered")
        .collect();
    assert_eq!(registered.len(), expected);
    for row in registered {
        let actors = &row["actors"];
        assert_eq!(actors["driver"]["parent"], std::process::id());
        assert_eq!(actors["group-owner"]["parent"], std::process::id());
        assert_eq!(actors["driver"]["group"], actors["group-owner"]["pid"]);
        assert_ne!(actors["driver"]["pid"], actors["group-owner"]["pid"]);
        assert_eq!(actors["native-entry"]["parent"], actors["driver"]["pid"]);
        assert_eq!(
            actors["native-entry"]["group"],
            actors["group-owner"]["pid"]
        );
        let pid = actors["native-entry"]["pid"].as_u64().expect("native PID");
        assert!(
            trace(root)
                .iter()
                .any(|v| v["phase"] == "sdk-native-wait" && v["child"] == pid)
        );
    }
    assert!(
        value["events"]
            .as_array()
            .expect("events")
            .iter()
            .filter(|v| v["kind"] == "scope_settled")
            .all(|v| v["fullKernelTreeSettlement"] == false)
    );
}

#[test]
#[ignore = "requires explicit installed-runtime shared software host record"]
fn actual_local_host_siblings_use_reviewed_entry_permissions_and_waits() {
    for callback in [false, true] {
        let root = fixture();
        let endpoint = OwnedEndpoint::new();
        let settings = json!({"user":{"permissions":{"denyInsteadOfPrompting":false}},"managed":{"permissions":{"denyInsteadOfPrompting":!callback}}});
        let (profile, services, resources, _host, mut supervisor, mut scope) =
            setup(&root, &endpoint, settings, false);
        supervisor.arm("new", false);
        let opened = block_on(profile.open_session(services.clone())).expect("actual new");
        let binding = opened.resume_binding().expect("binding").clone();
        close_after_turn(opened, services.clone(), &resources, &root, callback, false);
        supervisor.settle(&root);
        supervisor.arm("load", false);
        let loaded = block_on(
            profile
                .load_session(
                    RequestId::new("shared.load").expect("request"),
                    binding.clone(),
                    services.clone(),
                )
                .expect("load derives"),
        )
        .expect("actual load");
        let (_, loaded) = loaded.into_parts();
        close_after_turn(loaded, services.clone(), &resources, &root, callback, false);
        supervisor.settle(&root);
        supervisor.arm("resume", false);
        let resumed = block_on(
            profile
                .resume_session(
                    RequestId::new("shared.resume").expect("request"),
                    binding,
                    services.clone(),
                )
                .expect("resume derives"),
        )
        .expect("actual resume");
        close_after_turn(resumed, services, &resources, &root, callback, false);
        let value = supervisor.settle(&root);
        validate_actors(&value, &root, 3);
        assert_native_waits(&trace(&root), 3);
        assert_eq!(endpoint.requests.load(Ordering::Acquire), 3);
        println!(
            "SHARED_ACP_SOFTWARE {}",
            json!({"callback":callback,"events":value,"trace":trace(&root)})
        );
        let cleanup = scope.finish();
        assert_eq!(cleanup["timedOut"], false);
        assert_eq!(cleanup["ownDescriptorEof"], true);
        drop(supervisor);
        drop(endpoint);
        std::fs::remove_dir_all(root).expect("owned fixture cleanup");
    }
}

#[test]
#[ignore = "requires explicit installed-runtime shared software host record"]
fn actual_shared_cancel_and_checked_revocation_preserve_scope_cleanup() {
    cleanup_counterexample(false);
    cleanup_counterexample(true);
    dropped_start_counterexample();
    let root = fixture();
    let endpoint = OwnedEndpoint::new();
    let (profile, services, resources, host, mut supervisor, mut scope) = setup(
        &root,
        &endpoint,
        json!({"managed":{"permissions":{"denyInsteadOfPrompting":true}}}),
        false,
    );
    supervisor.arm("cancel", false);
    let session = block_on(profile.open_session(services.clone())).expect("session");
    close_after_turn(session, services.clone(), &resources, &root, false, true);
    let value = supervisor.settle(&root);
    validate_actors(&value, &root, 1);
    assert_native_waits(&trace(&root), 1);
    supervisor.arm("checked-revoked", false);
    host.approval.revoke();
    assert!(block_on(profile.open_session(services)).is_err());
    let value = supervisor.settle(&root);
    validate_actors(&value, &root, 1);
    println!(
        "SHARED_ACP_SOFTWARE {}",
        json!({"events":value,"trace":trace(&root)})
    );
    let cleanup = scope.finish();
    assert_eq!(cleanup["timedOut"], false);
    assert_eq!(cleanup["ownDescriptorEof"], true);
    drop(supervisor);
    drop(endpoint);
    std::fs::remove_dir_all(root).expect("owned fixture cleanup");
}

#[test]
#[ignore = "requires explicit installed-runtime shared software host record"]
fn actual_shared_preentry_revoke_and_argv_substitution_refuse_payload() {
    for substitute in [false, true] {
        let root = fixture();
        let endpoint = OwnedEndpoint::new();
        let (profile, services, resources, _host, mut supervisor, mut scope) =
            setup(&root, &endpoint, json!({}), substitute);
        supervisor.arm("refusal", !substitute);
        if substitute {
            assert!(block_on(profile.open_session(services.clone())).is_err());
        } else {
            let session =
                block_on(profile.open_session(services.clone())).expect("held entry session");
            wait_phase_file(&root, "native_registered");
            supervisor.control(json!({"op":"revoke"}));
            wait_phase_file(&root, "entry_refused");
            assert_eq!(
                block_on(session.close(resources.cleanup_request(), services.clone())),
                CleanupOutcome::Clean
            );
        }
        let value = supervisor.settle(&root);
        assert!(
            !trace(&root)
                .iter()
                .any(|row| row["phase"] == "native-start")
        );
        assert_eq!(endpoint.requests.load(Ordering::Acquire), 0);
        assert!(
            !value["events"]
                .as_array()
                .expect("events")
                .iter()
                .any(|v| v["kind"] == "native_released")
        );
        println!("SHARED_ACP_REFUSAL {}", value);
        let cleanup = scope.finish();
        assert_eq!(cleanup["timedOut"], false);
        assert_eq!(cleanup["ownDescriptorEof"], true);
        drop(supervisor);
        drop(endpoint);
        std::fs::remove_dir_all(root).expect("owned fixture cleanup");
    }
}
fn wait_phase_file(root: &Path, kind: &str) {
    let end = Instant::now() + Duration::from_secs(4);
    loop {
        let data =
            std::fs::read_to_string(root.join("supervisor-events.jsonl")).unwrap_or_default();
        if data
            .lines()
            .any(|line| serde_json::from_str::<Value>(line).is_ok_and(|v| v["kind"] == kind))
        {
            return;
        }
        assert!(Instant::now() < end, "missing host phase {kind}: {data}");
        std::thread::sleep(Duration::from_millis(5));
    }
}

// Both paths use the actual checked ACP root and its native child. The panic
// unwinds into the fixture owner; the controlled deadline stops it before the
// Nextest backstop can kill the Rust parent and orphan its private helper.
fn cleanup_counterexample(timeout: bool) {
    let root = fixture();
    let endpoint = OwnedEndpoint::new();
    let (profile, services, resources, _host, mut supervisor, scope) =
        setup(&root, &endpoint, json!({}), false);
    let receipt = scope.receipt();
    let case_root = root.clone();
    let observed = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        let mut scope = scope;
        supervisor.arm("owned-cleanup-fault", false);
        let mut session = block_on(profile.open_session(services.clone())).expect("fault session");
        let mut turn = block_on(session.start_turn(
            TurnRequest::new(
                RuntimeTurnId::new("fault.turn").expect("turn"),
                OperationContent::new("hold").expect("held prompt"),
            ),
            services.clone(),
        ))
        .expect("fault turn");
        wait_phase(&case_root, "native-prompt");
        scope.prove_live_inheritance();
        if !timeout {
            panic!("deliberate owned fixture failure");
        }
        scope.expire_soon();
        let terminal = block_on(turn.take_terminal_outcome().expect("pending terminal"));
        assert_ne!(terminal.status(), &TerminalStatus::Completed);
        let cleanup = scope.finish();
        assert_eq!(cleanup["timedOut"], true);
        assert_eq!(cleanup["ownDescriptorEof"], true);
        // Provider/session close remains separate from the host's process wait.
        assert_eq!(
            block_on(session.close(resources.cleanup_request(), services)),
            CleanupOutcome::Clean
        );
    }));
    assert_eq!(observed.is_ok(), timeout);
    let cleanup = receipt
        .lock()
        .expect("fixture receipt")
        .clone()
        .expect("cleanup before failure escapes");
    assert_eq!(cleanup["liveInheritanceObserved"], true);
    assert_eq!(cleanup["ownDescriptorEof"], true);
    let waits = cleanup["waits"].as_array().expect("host waits");
    assert_eq!(
        waits.len(),
        2,
        "version probe and ACP session both remain owned"
    );
    assert!(
        waits
            .iter()
            .all(|v| v["wait"] == true && v["stop"] == true && v["tree"] == "RootOnly")
    );
    assert_native_waits(&trace(&root), 1);
    println!(
        "ACP_CONTROLLED_FAULT {}",
        json!({"timeout":timeout,"cleanup":cleanup,"trace":trace(&root)})
    );
    drop(endpoint);
    std::fs::remove_dir_all(root).expect("owned fault fixture cleanup");
}

fn dropped_start_counterexample() {
    let root = fixture();
    let endpoint = OwnedEndpoint::new();
    let (_profile, services, _resources, _host, supervisor, mut scope) =
        setup(&root, &endpoint, json!({}), false);
    let request = ProcessRequest::new(
        ExecutableRef::new("claude-agent.prepared.executable").expect("target"),
    )
    .with_arguments(["--version".to_owned()])
    .with_environment([
        EnvironmentRef::new("claude-agent.prepared.environment").expect("environment")
    ])
    .with_working_resource(WorkingResourceRef::new("checked.workspace").expect("workspace"));
    let pending = services
        .process()
        .expect("tracked process host")
        .start_checked(
            ScopeId::new("dropped.fixture.start").expect("scope"),
            CheckedProcessRef::new("fixture.checked.binding").expect("checked binding"),
            request,
        );
    drop(pending);
    let cleanup = scope.finish();
    assert_eq!(cleanup["ownDescriptorEof"], true);
    let waits = cleanup["waits"].as_array().expect("wait receipts");
    assert_eq!(waits.len(), 2);
    assert!(
        waits
            .iter()
            .all(|v| v["stop"] == true && v["wait"] == true && v["tree"] == "RootOnly")
    );
    assert!(
        waits.iter().any(|v| v["late"] == true),
        "abandoned startup must finish under the stopped fixture owner"
    );
    assert!(!trace(&root).iter().any(|v| v["phase"] == "native-start"));
    println!("ACP_DROPPED_START {cleanup}");
    drop(supervisor);
    drop(endpoint);
    std::fs::remove_dir_all(root).expect("owned dropped-start fixture cleanup");
}
