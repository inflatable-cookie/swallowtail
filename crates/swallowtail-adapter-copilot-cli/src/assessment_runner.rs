use super::*;
use std::{
    collections::BTreeMap,
    ffi::OsString,
    os::unix::fs::{OpenOptionsExt, PermissionsExt},
    time::{Duration, Instant},
};
use swallowtail_host_local::{
    LocalExecutableLaunch, LocalHostServices, LocalProcessHost, LocalProcessLimits,
};
use swallowtail_runtime::{
    ActivityKind, ActivityLifecyclePhase, ActivityStatus, CleanupOutcome, ProviderObservation,
    RuntimeEventKind, SessionCleanupRequest,
};

const TASK_ID: &str = "7823a168-9b8f-4604-8fd7-201c2998722e";
const FROZEN_ENABLE_COMMAND: &str =
    "effigy observe:copilot-acp-private-assessment <binding-payload.json>";
const ORIGINAL_ENTRY_PAYLOAD_SCHEMA: &str = "copilot-acp-private-original-binding.v1";
const ORIGINAL_ENTRY_PAYLOAD_MAX_BYTES: u64 = 8 * 1024;
const ORIGINAL_TOTAL_BUDGET: Duration = Duration::from_secs(60);
const ORIGINAL_CLEANUP_BUDGET: Duration = Duration::from_secs(3);
const SENTINEL_AFTER_TEXT: &str = "SWALLOWTAIL_PERMISSION_SENTINEL_AFTER_V1\n";
const FAKE_PROTOCOL_SCRIPT: &str = r#"import atexit, json, os, signal, sys

if "--version" in sys.argv:
    print("copilot 1.0.95", flush=True)
    raise SystemExit(0)

mode = os.environ["SWALLOWTAIL_FAKE_MODE"]
trace_path = os.environ["SWALLOWTAIL_FAKE_TRACE"]
records_root = os.environ.get("SWALLOWTAIL_FAKE_RECORDS")
task_dir = None
prompt_id = None

def trace(value):
    with open(trace_path, "a", encoding="utf-8") as stream:
        stream.write(value + "\n")
        stream.flush()
        os.fsync(stream.fileno())

def stopped(_signum, _frame):
    trace("process-stopped")
    raise SystemExit(0)

signal.signal(signal.SIGTERM, stopped)
atexit.register(lambda: trace("process-exited"))

def send(value):
    sys.stdout.write(json.dumps(value, separators=(",", ":")) + "\n")
    sys.stdout.flush()

for raw in sys.stdin:
    request = json.loads(raw)
    method = request.get("method")
    request_id = request.get("id")
    if method == "initialize":
        send({"jsonrpc":"2.0","id":request_id,"result":{"protocolVersion":1,"agentInfo":{"name":"copilot","version":"1.0.95"}}})
    elif method == "session/new":
        task_dir = request["params"]["cwd"]
        if mode == "session-new-failure":
            send({"jsonrpc":"2.0","id":request_id,"error":{"code":-32603,"message":"fixture session/new failed"}})
        elif mode != "session-new-hang":
            send({"jsonrpc":"2.0","id":request_id,"result":{"sessionId":"task-owned-fake-session"}})
    elif method == "session/prompt":
        prompt_id = request_id
        if records_root is not None:
            prompt = os.path.join(records_root,"1.0.95-prompt-3.json")
            visible = os.path.isfile(prompt)
            if visible:
                with open(prompt, encoding="utf-8") as stream:
                    record = json.load(stream)
                visible = (
                    record.get("phase") == "prompt-consumed-before-prepared-turn"
                    and record.get("prompt_consumed") is True
                    and record.get("prompt_send_started_at_consumption") is False
                    and os.stat(prompt, follow_symlinks=False).st_mode & 0o777 == 0o600
                )
            trace("prompt-record-visible-before-send:" + str(visible))
        if mode == "execution-failure":
            trace("prompt-execution-failure")
            send({"jsonrpc":"2.0","id":prompt_id,"error":{"code":-32603,"message":"fixture prompt failed"}})
        elif mode != "hang":
            action_id = "different-action" if mode == "mismatched-action" else "sentinel-edit"
            action_path = "permission-sentinel-other.txt" if mode == "wrong-action-path" else "permission-sentinel.txt"
            if mode != "missing-action":
                send({"jsonrpc":"2.0","method":"session/update","params":{"sessionId":"task-owned-fake-session","update":{"sessionUpdate":"tool_call","toolCallId":action_id,"title":"Overwrite task sentinel","kind":"execute","status":"pending","content":[{"type":"diff","path":action_path,"oldText":"SWALLOWTAIL_PERMISSION_SENTINEL_BEFORE_V1\n","newText":"SWALLOWTAIL_PERMISSION_SENTINEL_AFTER_V1\n"}],"locations":[{"path":action_path}]}}})
            trace("permission-request-tool-call-id:sentinel-edit")
            send({"jsonrpc":"2.0","id":900,"method":"session/request_permission","params":{"sessionId":"task-owned-fake-session","toolCall":{"toolCallId":"sentinel-edit","status":"pending"},"options":[{"optionId":"reject_once","name":"reject_once","kind":"reject_once"}]}})
    elif method == "session/cancel":
        trace("session-cancel")
        if prompt_id is not None:
            trace("prompt-result-cancelled")
            send({"jsonrpc":"2.0","id":prompt_id,"result":{"stopReason":"cancelled"}})
    elif request_id == 900:
        tool_call_id = request.get("params",{}).get("toolCall",{}).get("toolCallId")
        trace("permission-action-id:" + str(tool_call_id))
        outcome = request.get("result",{}).get("outcome",{}).get("outcome")
        trace("permission-" + str(outcome))
        if mode == "provider-effect":
            with open(os.path.join(task_dir,"permission-sentinel.txt"),"wb") as stream:
                stream.write(b"SWALLOWTAIL_PERMISSION_SENTINEL_AFTER_V1\n")
                stream.flush()
                os.fsync(stream.fileno())
    else:
        send({"jsonrpc":"2.0","id":request_id,"error":{"code":-32601,"message":"fixture method unsupported"}})
"#;

struct RealScratch {
    task: PathBuf,
    executable: PathBuf,
    protected_executable: ProtectedExecutable,
    records: PathBuf,
    trace: PathBuf,
}

impl RealScratch {
    fn new() -> Self {
        let task = fresh_temp_dir("copilot-acp-real-task");
        let executable_root = fresh_temp_dir("copilot-acp-real-executable");
        let executable = executable_root.join("copilot.py");
        let sentinel = task.join(SENTINEL_PATH);
        let trace = executable_root.join("fake-trace.log");
        fs::write(&executable, FAKE_PROTOCOL_SCRIPT).expect("write test-owned fake ACP executable");
        let protected_executable =
            ProtectedExecutable::copy_from(&executable, &sha256_file(&executable))
                .expect("freeze exact fake executable bytes in private scratch");
        let records = fresh_home_temp_dir("copilot-acp-real-records");
        fs::write(&sentinel, SENTINEL_BEFORE).expect("create test-owned sentinel bytes");
        Self {
            task,
            executable,
            protected_executable,
            records,
            trace,
        }
    }
}

struct ProtectedExecutable {
    root: PathBuf,
    path: PathBuf,
    sha256: String,
}

impl ProtectedExecutable {
    fn copy_from(source: &Path, expected_sha256: &str) -> std::io::Result<Self> {
        let metadata = fs::symlink_metadata(source)?;
        if !metadata.is_file() || metadata.file_type().is_symlink() {
            return Err(std::io::Error::other(
                "approved executable is not a regular file",
            ));
        }
        let root = fresh_temp_dir("copilot-acp-frozen-executable");
        fs::set_permissions(&root, fs::Permissions::from_mode(0o700))?;
        let path = root.join("copilot-executable.bin");
        let mut output = OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .custom_flags(libc::O_NOFOLLOW)
            .open(&path)?;
        std::io::copy(&mut File::open(source)?, &mut output)?;
        output.sync_all()?;
        fs::set_permissions(&path, fs::Permissions::from_mode(0o400))?;
        File::open(&root)?.sync_all()?;
        let sha256 = sha256_file(&path);
        if sha256 != expected_sha256 {
            fs::remove_dir_all(&root)?;
            return Err(std::io::Error::other(
                "approved executable identity changed while freezing",
            ));
        }
        Ok(Self { root, path, sha256 })
    }
}

impl Drop for ProtectedExecutable {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}

impl ProtectedExecutable {
    fn cleanup(&self) -> bool {
        match fs::symlink_metadata(&self.root) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => true,
            Err(_) => false,
            Ok(metadata) if metadata.is_dir() && !metadata.file_type().is_symlink() => {
                fs::remove_dir_all(&self.root).is_ok()
            }
            Ok(_) => false,
        }
    }
}

impl Drop for RealScratch {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.task).expect("remove only the fresh task-owned fixture directory");
        fs::remove_dir_all(
            self.executable
                .parent()
                .expect("fake executable has a parent"),
        )
        .expect("remove only the fresh fake executable directory");
        fs::remove_dir_all(&self.records).expect("remove only the fresh fake record directory");
    }
}

#[derive(Debug)]
struct RunEvidence {
    status: &'static str,
    terminal: &'static str,
    cleanup: &'static str,
    task_joined: bool,
    resource_released: bool,
    process_stopped: bool,
    process_exited: bool,
    attempt_visible_before_version: bool,
    prompt_visible_before_send: bool,
    elapsed: Duration,
    timed_out: bool,
    cancel_request_sent: bool,
    action_id: Option<String>,
    action_count: usize,
    action_content_matches: bool,
    permission_action_metadata_verified: bool,
    terminal_accepted: bool,
    action_cancelled: bool,
    permission_request: Option<String>,
    permission_request_count: usize,
    permission_action_id: Option<String>,
    permission_cancelled: bool,
    session_cancel_sent: bool,
    prompt_cancelled: bool,
    original_started: bool,
    before_tree_sha256: String,
    after_tree_sha256: String,
    attempt_record: serde_json::Value,
    prompt_record: Option<serde_json::Value>,
    result_record: serde_json::Value,
    record_files_private: bool,
    protected_executable_removed: bool,
}

fn run_with_local_host(mode: &str, total: Duration, cleanup: Duration) -> RunEvidence {
    assert!(total > cleanup);
    let scratch = RealScratch::new();
    let python = find_program("python3").expect("Effigy-provided python3 is on PATH");
    let host = ExecutionHostId::new("assessment.real-local-host").expect("host id");
    let environment = EnvironmentRef::new(APPROVED_ENVIRONMENT).expect("environment ref");
    let working = WorkingResourceRef::new("assessment.real-task-scratch").expect("resource ref");
    let executable_ref = ExecutableRef::new(
        scratch
            .executable
            .to_str()
            .expect("fake executable path is UTF-8"),
    )
    .expect("executable ref");
    let (builder, _target) = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_installed_executable_launch(
            executable_ref.clone(),
            InterfaceVersionAxis::new(COPILOT_CLI_PACKAGE_AXIS).expect("version axis"),
            LocalExecutableLaunch::interpreted_script(python, scratch.executable.clone()),
        );
    let local = builder
        .approve_environment(
            environment,
            [
                (
                    OsString::from("SWALLOWTAIL_FAKE_MODE"),
                    OsString::from(mode),
                ),
                (
                    OsString::from("SWALLOWTAIL_FAKE_TRACE"),
                    scratch.trace.as_os_str().to_owned(),
                ),
                (
                    OsString::from("SWALLOWTAIL_FAKE_RECORDS"),
                    scratch.records.as_os_str().to_owned(),
                ),
            ],
        )
        .approve_working_resource(working.clone(), scratch.task.clone())
        .build_services(host.clone());
    let services = local.services().clone();
    let started = Instant::now();
    let action_window = total - cleanup;
    let action_deadline = local.deadline_after(action_window.saturating_sub(started.elapsed()));
    let before_tree_sha256 = tree_sha256(&scratch.task);
    let plan_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/copilot-cli-acp-currentness-1.0.95/private-assessment-plan.json");
    let plan: serde_json::Value =
        serde_json::from_slice(&fs::read(&plan_path).expect("exact disabled plan exists"))
            .expect("plan JSON parses");
    let plan_sha256 = sha256_reader(
        serde_json::to_vec(&plan)
            .expect("plan serializes")
            .as_slice(),
    )
    .expect("plan hash computes");
    let runner_sha256 = sha256_file(
        &Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../scripts/copilot-acp-private-assessment.py"),
    );
    let rust_runner_sha256 =
        sha256_file(&Path::new(env!("CARGO_MANIFEST_DIR")).join("src/assessment_runner.rs"));
    let (authority_binding_sha256, reviewed_head, enable_command, source_manifest_sha256) =
        plan_binding_metadata(&plan);
    let executable_bytes = fs::read(&scratch.executable).expect("approved fake bytes read");
    let executable_sha256 = sha256_reader(executable_bytes.as_slice()).expect("fake hashes");
    let binding = crate::assessment::capture_host_binding(
        &host,
        &executable_ref,
        &EnvironmentRef::new(APPROVED_ENVIRONMENT).expect("environment ref"),
        fs::File::open(&scratch.executable).expect("approved fake executable opens"),
        &executable_sha256,
    )
    .expect("safe approved inputs and executable digest captured before discovery");
    let attempt_path = scratch.records.join("1.0.95-attempt-4.json");
    let attempt = serde_json::json!({
        "schema": "copilot-cli-private-assessment-consumed.v1",
        "phase": "invocation-consumed-before-prepared-open",
        "task_id": TASK_ID,
        "version": PRIVATE_ASSESSMENT_VERSION,
        "invocation_number": 4,
        "plan_sha256": plan_sha256,
        "runner_sha256": runner_sha256,
        "rust_runner_sha256": rust_runner_sha256,
        "authority_binding_sha256": authority_binding_sha256,
        "reviewed_head": reviewed_head,
        "enable_command": enable_command,
        "source_manifest_sha256": source_manifest_sha256,
        "execution_host_id": binding.execution_host_id,
        "execution_host_id_sha256": binding.execution_host_id_sha256,
        "executable_ref": binding.executable_ref,
        "executable_ref_sha256": binding.executable_ref_sha256,
        "environment_ref": binding.environment_ref,
        "environment_ref_sha256": binding.environment_ref_sha256,
        "executable_bytes_sha256": binding.executable_bytes_sha256,
        "wrapper_archive_sha256": crate::assessment::WRAPPER_ARCHIVE_SHA256,
        "native_archive_sha256": crate::assessment::NATIVE_ARCHIVE_SHA256,
        "argv": ["--acp", "--stdio"],
        "mode": "real-local-host-fake-child",
        "original_started": false
    });
    write_exclusive_fsynced(&attempt_path, &attempt)
        .expect("invocation consumption record is exclusive and fsynced before discovery");

    let input = preparation_input_with_executable(host.clone(), executable_ref.as_host_value());
    let probe = CopilotCliPreparationProbe::new(
        RequestId::new("copilot-cli.assessment.real-probe").expect("request"),
        ScopeId::new("copilot-cli.assessment.real-probe").expect("scope"),
        action_deadline,
        DiscoveryCancellation::new(),
    );
    let prepared = block_on(prepare_copilot_cli_acp_for_assessment_with_reader(
        input,
        probe,
        services.clone(),
        Cursor::new(executable_bytes.as_slice()),
        &executable_sha256,
    ))
    .expect("fake version discovery uses the same prepared admission");
    let session = prepared
        .prepare_session(CopilotCliSessionProfileInput::new(
            RequestId::new("copilot-cli.assessment.real-session").expect("request"),
            working,
        ))
        .expect("prepared fake session preflights");

    drive_prepared_session(
        session,
        &local,
        RunScratch {
            task: &scratch.task,
            records: &scratch.records,
            trace: Some(&scratch.trace),
            protected_executable: Some(&scratch.protected_executable),
        },
        services,
        started,
        action_window,
        action_deadline,
        total,
        plan_sha256,
        runner_sha256,
        rust_runner_sha256,
        authority_binding_sha256,
        reviewed_head,
        enable_command,
        source_manifest_sha256,
        binding,
        consumed_record_visible_before_effect(
            &attempt_path,
            "invocation-consumed-before-prepared-open",
        ),
        attempt_path,
        before_tree_sha256,
        false,
        "real-local-host-fake-child",
    )
}

fn run_fake_original_branch(mode: &str, total: Duration, cleanup: Duration) -> RunEvidence {
    let scratch = RealScratch::new();
    run_fake_original_branch_with_scratch(&scratch, mode, total, cleanup)
        .expect("test-only planner binding runs only the fake child")
}

fn run_fake_original_branch_with_scratch(
    scratch: &RealScratch,
    mode: &str,
    total: Duration,
    cleanup: Duration,
) -> Result<RunEvidence, OriginalRunDenied> {
    let python = find_program("python3").expect("Effigy-provided python3 is on PATH");
    let host = ExecutionHostId::new("assessment.real-local-host").expect("host id");
    let environment = EnvironmentRef::new(APPROVED_ENVIRONMENT).expect("environment ref");
    let working = WorkingResourceRef::new("assessment.real-task-scratch").expect("resource ref");
    let executable = ExecutableRef::new(
        scratch
            .executable
            .to_str()
            .expect("fake executable path is UTF-8"),
    )
    .expect("executable ref");
    let (builder, _target) = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_installed_executable_launch(
            executable,
            InterfaceVersionAxis::new(COPILOT_CLI_PACKAGE_AXIS).expect("version axis"),
            LocalExecutableLaunch::interpreted_script(python, scratch.executable.clone()),
        );
    let local = builder
        .approve_environment(
            environment,
            [
                (
                    OsString::from("SWALLOWTAIL_FAKE_MODE"),
                    OsString::from(mode),
                ),
                (
                    OsString::from("SWALLOWTAIL_FAKE_TRACE"),
                    scratch.trace.as_os_str().to_owned(),
                ),
                (
                    OsString::from("SWALLOWTAIL_FAKE_RECORDS"),
                    scratch.records.as_os_str().to_owned(),
                ),
            ],
        )
        .approve_working_resource(working.clone(), scratch.task.clone())
        .build_services(host.clone());
    let plan = enabled_fake_plan(&scratch);
    let plan_sha256 = sha256_reader(
        serde_json::to_vec(&plan)
            .expect("fake-bound plan serializes")
            .as_slice(),
    )
    .expect("fake-bound plan hash computes");
    let continuation = PlannerBoundContinuation {
        reviewed_head: plan["original_enable_gate"]["reviewed_head"]
            .as_str()
            .expect("fake reviewed head")
            .to_owned(),
        plan_sha256,
        binding_sha256: plan["original_enable_gate"]["binding_sha256"]
            .as_str()
            .expect("fake host binding digest")
            .to_owned(),
        enable_command: plan["original_enable_gate"]["enable_command"]
            .as_str()
            .expect("fake enable command")
            .to_owned(),
    };
    let executable_sha256 = sha256_file(&scratch.executable);
    run_original_task_once(
        &plan,
        Some(&continuation),
        preparation_input_with_executable(
            host,
            scratch
                .executable
                .to_str()
                .expect("fake executable path is UTF-8"),
        ),
        RequestId::new("copilot-cli.assessment.fake-original.probe").expect("probe request"),
        RequestId::new("copilot-cli.assessment.fake-original.session").expect("session request"),
        working,
        &local,
        &scratch.task,
        &scratch.records,
        Some(&scratch.trace),
        OriginalRunPolicy {
            total,
            cleanup,
            expected_executable_sha256: executable_sha256,
            original_started: false,
            execution_mode: "fake-local-process-only",
        },
    )
}

fn enabled_fake_plan(scratch: &RealScratch) -> serde_json::Value {
    let plan_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/copilot-cli-acp-currentness-1.0.95/private-assessment-plan.json");
    let mut plan: serde_json::Value =
        serde_json::from_slice(&fs::read(plan_path).expect("disabled exact plan is readable"))
            .expect("disabled exact plan parses");
    let home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .expect("HOME exists");
    let relative_records = scratch
        .records
        .strip_prefix(&home)
        .expect("fresh fake record directory is under HOME");
    plan["records"]["root"] =
        serde_json::Value::String(format!("$HOME/{}", relative_records.display()));
    plan["records"]["original_execution_enabled"] = serde_json::Value::Bool(true);
    plan["original_execution_enabled"] = serde_json::Value::Bool(true);
    plan["original_enable_gate"]["state"] =
        serde_json::Value::String("enabled for planner-bound continuation".to_owned());
    plan["original_enable_gate"]["reviewed_head"] =
        serde_json::Value::String("0123456789abcdef0123456789abcdef01234567".to_owned());
    plan["original_enable_gate"]["enable_command"] =
        serde_json::Value::String(FROZEN_ENABLE_COMMAND.to_owned());
    plan["original_enable_gate"]["binding_sha256"] = serde_json::Value::String("0".repeat(64));
    plan
}

struct RunScratch<'a> {
    task: &'a Path,
    records: &'a Path,
    trace: Option<&'a Path>,
    protected_executable: Option<&'a ProtectedExecutable>,
}

fn drive_prepared_session(
    session: crate::prepared::CopilotCliPreparedSession,
    local: &LocalHostServices,
    scratch: RunScratch<'_>,
    services: swallowtail_runtime::HostServices,
    started: Instant,
    action_window: Duration,
    action_deadline: Deadline,
    total: Duration,
    plan_sha256: String,
    runner_sha256: String,
    rust_runner_sha256: String,
    authority_binding_sha256: String,
    reviewed_head: Option<String>,
    enable_command: Option<String>,
    source_manifest_sha256: String,
    binding: crate::assessment::CopilotCliAssessmentHostBinding,
    attempt_record_visible_before_version: bool,
    attempt_path: PathBuf,
    before_tree_sha256: String,
    original_started: bool,
    execution_mode: &'static str,
) -> RunEvidence {
    let mut status: &str;
    let mut terminal_label = "not-started";
    let mut cleanup_label: &str;
    let mut timed_out = false;
    let mut cancel_request_sent = false;
    let mut cancellation_request_failed = false;
    let mut prompt_record_failed = false;
    let mut action_id = None;
    let mut action_count = 0;
    let mut action_content_matches = false;
    let mut action_cancelled = false;
    let mut permission_request = None;
    let mut permission_request_count = 0;
    let mut permission_action_id = None;
    let prompt_path = scratch.records.join("1.0.95-prompt-3.json");
    let result_path = scratch.records.join("1.0.95-execution.json");
    let mut prompt_record_path = None;
    let remaining_action = action_window.saturating_sub(started.elapsed());
    let remaining_total = total.saturating_sub(started.elapsed());
    let open_action_deadline = local.deadline_after(remaining_action);
    let open_total_deadline = local.deadline_after(remaining_total);
    let opened = block_on(session.open_assessment_session_with_deadlines(
        services.clone(),
        open_action_deadline,
        open_total_deadline,
    ));
    match opened {
        Err(error) => {
            status = if error.diagnostic().code().ends_with("session_open_deadline") {
                "session-open-timeout"
            } else {
                "session-open-failed"
            };
            cleanup_label = if error
                .diagnostic()
                .code()
                .ends_with("session_open_cleanup_failed")
            {
                "failed"
            } else {
                "driver-joined"
            };
        }
        Ok(mut handle) => {
            if started.elapsed() >= action_window {
                status = "action-cutoff-before-prompt";
                let remaining = total.saturating_sub(started.elapsed());
                let deadline = local.deadline_after(remaining);
                let cleanup_outcome =
                    block_on(handle.close(SessionCleanupRequest::new(deadline), services.clone()));
                cleanup_label = cleanup_name(&cleanup_outcome);
            } else {
                let prompt_record = serde_json::json!({
                    "schema": "copilot-cli-private-assessment-consumed.v1",
                    "phase": "prompt-consumed-before-prepared-turn",
                    "task_id": TASK_ID,
                    "version": PRIVATE_ASSESSMENT_VERSION,
                    "prompt_number": 3,
                    "plan_sha256": plan_sha256,
                    "runner_sha256": runner_sha256,
                    "rust_runner_sha256": rust_runner_sha256,
                    "authority_binding_sha256": authority_binding_sha256,
                    "reviewed_head": reviewed_head,
                    "enable_command": enable_command,
                    "source_manifest_sha256": source_manifest_sha256,
                    "invocation_record_sha256": sha256_file(&attempt_path),
                    "execution_host_id": binding.execution_host_id,
                    "execution_host_id_sha256": binding.execution_host_id_sha256,
                    "executable_ref": binding.executable_ref,
                    "executable_ref_sha256": binding.executable_ref_sha256,
                    "environment_ref": binding.environment_ref,
                    "environment_ref_sha256": binding.environment_ref_sha256,
                    "executable_bytes_sha256": binding.executable_bytes_sha256,
                    "argv": ["--acp", "--stdio"],
                    "mode": execution_mode,
                    "prompt_consumed": true,
                    "prompt_send_started_at_consumption": false
                });
                match write_exclusive_fsynced(&prompt_path, &prompt_record) {
                    Ok(()) => {
                        prompt_record_path = Some(prompt_path.clone());
                        let start_turn = handle.start_turn(
                            TurnRequest::new(
                                RuntimeTurnId::new("assessment.real-local-host.turn")
                                    .expect("turn"),
                                OperationContent::new(SENTINEL_PROMPT).expect("prompt"),
                            ),
                            services.clone(),
                        );
                        let started_turn = if let Some(time) = services.time() {
                            match block_on(select(
                                Box::pin(start_turn),
                                time.wait_until(action_deadline),
                            )) {
                                Either::Left((result, _)) => Some(result),
                                Either::Right((_deadline, _pending_start)) => {
                                    timed_out = true;
                                    terminal_label = "turn-start-deadline";
                                    None
                                }
                            }
                        } else {
                            terminal_label = "monotonic-clock-missing";
                            None
                        };
                        if let Some(started_turn) = started_turn {
                            match started_turn {
                                Ok(mut turn) => {
                                    if let Some(mut events) = turn.take_events() {
                                        if let Some(time) = services.time() {
                                            let mut wait =
                                                Box::pin(time.wait_until(action_deadline));
                                            loop {
                                                match block_on(select(
                                                    Box::pin(events.next()),
                                                    wait,
                                                )) {
                                                    Either::Left((
                                                        Some(Ok(event)),
                                                        remaining_wait,
                                                    )) => {
                                                        match event.kind() {
                                                        RuntimeEventKind::Activity(activity)
                                                            if activity.kind() == &ActivityKind::ProviderOwnedTool
                                                                && activity.phase() == ActivityLifecyclePhase::Started
                                                                && activity.status() == ActivityStatus::Pending =>
                                                        {
                                                            action_count += 1;
                                                            action_id = activity
                                                                .provider_activity_ref()
                                                                .map(|reference| reference.as_provider_value().to_owned());
                                                            action_content_matches = activity
                                                                .content()
                                                                .is_some_and(|update| {
                                                                    update.content().as_str()
                                                                        == SENTINEL_AFTER_TEXT
                                                                });
                                                        }
                                                        RuntimeEventKind::Activity(activity)
                                                            if activity.kind() == &ActivityKind::ProviderOwnedTool
                                                                && activity.phase() == ActivityLifecyclePhase::Completed
                                                                && activity.status() == ActivityStatus::Cancelled =>
                                                        {
                                                            action_cancelled = activity
                                                                .provider_activity_ref()
                                                                .map(|reference| reference.as_provider_value().to_owned())
                                                                == action_id;
                                                        }
                                                        RuntimeEventKind::ProviderObservation(
                                                            ProviderObservation::RequestCorrelation(reference),
                                                        ) => {
                                                            permission_request_count += 1;
                                                            permission_request =
                                                                Some(reference.as_provider_value().to_owned());
                                                        }
                                                        _ => {}
                                                    }
                                                        wait = remaining_wait;
                                                    }
                                                    Either::Left((Some(Err(_)), _)) => {
                                                        break;
                                                    }
                                                    Either::Left((None, _)) => break,
                                                    Either::Right((
                                                        _observation,
                                                        _pending_events,
                                                    )) => {
                                                        timed_out = true;
                                                        let remaining =
                                                            total.saturating_sub(started.elapsed());
                                                        let cancel_deadline =
                                                            local.deadline_after(remaining);
                                                        match block_on(select(
                                                            turn.cancellation().request(),
                                                            time.wait_until(cancel_deadline),
                                                        )) {
                                                            Either::Left((Ok(_), _)) => {
                                                                cancel_request_sent = true;
                                                            }
                                                            _ => {
                                                                cancellation_request_failed = true;
                                                            }
                                                        }
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    if let Some(terminal) = turn.take_terminal_outcome() {
                                        let remaining = total.saturating_sub(started.elapsed());
                                        let terminal_deadline = local.deadline_after(remaining);
                                        if let Some(time) = services.time() {
                                            match block_on(select(
                                                terminal,
                                                time.wait_until(terminal_deadline),
                                            )) {
                                                Either::Left((outcome, _)) => {
                                                    terminal_label = match outcome.status() {
                                                        TerminalStatus::Cancelled => "cancelled",
                                                        TerminalStatus::ProviderRequestObserved(
                                                            _,
                                                        ) => "permission-observed",
                                                        TerminalStatus::RuntimeFailed(_) => {
                                                            "runtime-failed"
                                                        }
                                                        TerminalStatus::ProviderFailed(_) => {
                                                            "provider-failed"
                                                        }
                                                        _ => "other-terminal",
                                                    };
                                                }
                                                Either::Right((
                                                    _observation,
                                                    _pending_terminal,
                                                )) => {
                                                    terminal_label = "terminal-deadline-expired";
                                                }
                                            }
                                        } else {
                                            terminal_label = "monotonic-clock-missing";
                                        }
                                    } else {
                                        terminal_label = "terminal-outcome-missing";
                                    }
                                    drop(turn);
                                }
                                Err(_) => {
                                    terminal_label = "runtime-failed";
                                }
                            }
                        }
                    }
                    Err(_) => prompt_record_failed = true,
                }
                let remaining = total.saturating_sub(started.elapsed());
                let cleanup_deadline = local.deadline_after(remaining);
                let cleanup_outcome = block_on(handle.close(
                    SessionCleanupRequest::new(cleanup_deadline),
                    services.clone(),
                ));
                cleanup_label = cleanup_name(&cleanup_outcome);
                let protocol_trace = scratch
                    .trace
                    .and_then(|path| fs::read_to_string(path).ok())
                    .unwrap_or_default();
                let permission_cancelled = protocol_trace.contains("permission-cancelled");
                let session_cancel_sent = protocol_trace.contains("session-cancel");
                let prompt_cancelled = protocol_trace.contains("prompt-result-cancelled");
                permission_action_id = protocol_trace
                    .lines()
                    .find_map(|line| line.strip_prefix("permission-request-tool-call-id:"))
                    .map(str::to_owned);
                let action_complete = action_count == 1
                    && action_content_matches
                    && action_id.as_deref() == Some("sentinel-edit")
                    && action_cancelled
                    && permission_request_count == 1
                    && permission_request.as_deref() == Some("acp:900")
                    && (scratch.trace.is_none()
                        || permission_action_id.as_deref() == action_id.as_deref());
                if prompt_record_failed {
                    status = "prompt-record-failed";
                } else if cancellation_request_failed {
                    status = "cancellation-request-failed";
                } else if terminal_label == "permission-observed" && !action_complete {
                    status = "incomplete-action-correlation";
                    terminal_label = "unaccepted-permission-terminal";
                } else if terminal_label == "permission-observed"
                    && !(permission_cancelled && session_cancel_sent && prompt_cancelled)
                {
                    status = "incomplete-cancellation-protocol";
                    terminal_label = "unaccepted-permission-terminal";
                } else if timed_out && terminal_label == "cancelled" {
                    status = "action-timeout-cancelled";
                } else if terminal_label == "permission-observed" && action_complete {
                    status = "permission-cancelled";
                } else if terminal_label == "runtime-failed" {
                    status = if protocol_trace.contains("permission-request-tool-call-id:") {
                        "incomplete-action-correlation"
                    } else {
                        "execution-failed"
                    };
                } else if terminal_label == "turn-start-deadline" {
                    status = "prompt-start-timeout";
                    terminal_label = "unaccepted-prompt-start-timeout";
                } else {
                    status = "incomplete";
                }
            }
        }
    }

    let permission_action_metadata_verified = terminal_label == "permission-observed"
        && action_count == 1
        && action_content_matches
        && action_id.as_deref() == Some("sentinel-edit");
    let after_tree_sha256 = tree_sha256(scratch.task);
    let effect_count = usize::from(before_tree_sha256 != after_tree_sha256);
    if effect_count != 0 {
        status = "effect-detected";
    }
    let process_trace = scratch
        .trace
        .and_then(|path| fs::read_to_string(path).ok())
        .unwrap_or_default();
    let executable_path = Path::new(&binding.executable_ref);
    let executable_digest_after_cleanup = fs::symlink_metadata(executable_path)
        .ok()
        .filter(|metadata| metadata.is_file() && !metadata.file_type().is_symlink())
        .map(|_| sha256_file(executable_path));
    let identity_drift_after_cleanup = executable_digest_after_cleanup.as_deref()
        != Some(binding.executable_bytes_sha256.as_str());
    if identity_drift_after_cleanup {
        status = "identity-drift-after-cleanup";
    }
    let protected_executable_sha256_after_cleanup =
        scratch.protected_executable.and_then(|protected| {
            fs::symlink_metadata(&protected.path)
                .ok()
                .filter(|metadata| metadata.is_file() && !metadata.file_type().is_symlink())
                .map(|_| sha256_file(&protected.path))
        });
    let protected_executable_drift = protected_executable_sha256_after_cleanup.as_deref()
        != scratch
            .protected_executable
            .map(|protected| protected.sha256.as_str());
    if protected_executable_drift {
        status = "protected-executable-drift";
    }
    let protected_executable_removed = scratch
        .protected_executable
        .is_none_or(ProtectedExecutable::cleanup);
    if !protected_executable_removed {
        status = "protected-executable-cleanup-failed";
        cleanup_label = "failed";
    }
    let permission_cancelled = process_trace.contains("permission-cancelled");
    let session_cancel_sent = process_trace.contains("session-cancel");
    let prompt_cancelled = process_trace.contains("prompt-result-cancelled");
    if cleanup_label == "not-run" {
        cleanup_label = if process_trace.contains("process-exited") {
            "driver-joined"
        } else {
            "unknown"
        };
    }
    let task_joined = matches!(cleanup_label, "clean" | "driver-joined");
    let resource_released = task_joined;
    let host_confirmed_process_cleanup = matches!(cleanup_label, "clean" | "driver-joined");
    let process_stopped =
        process_trace.contains("process-stopped") || host_confirmed_process_cleanup;
    let process_exited = process_trace.contains("process-exited") || host_confirmed_process_cleanup;
    let attempt_visible_before_version = attempt_record_visible_before_version;
    let prompt_visible_before_send =
        process_trace.contains("prompt-record-visible-before-send:True");
    let mut terminal_accepted = effect_count == 0
        && cleanup_label == "clean"
        && !identity_drift_after_cleanup
        && !protected_executable_drift
        && protected_executable_removed
        && match terminal_label {
            "permission-observed" => {
                action_count == 1
                    && action_content_matches
                    && action_id.as_deref() == Some("sentinel-edit")
                    && permission_action_metadata_verified
                    && action_cancelled
                    && permission_request_count == 1
                    && permission_request.as_deref() == Some("acp:900")
                    && (scratch.trace.is_none()
                        || permission_action_id.as_deref() == action_id.as_deref())
                    && permission_cancelled
                    && session_cancel_sent
                    && prompt_cancelled
            }
            "cancelled" => timed_out && cancel_request_sent,
            _ => false,
        };
    let elapsed_before_result_fsync = started.elapsed();
    if elapsed_before_result_fsync >= total {
        status = "inclusive-deadline-exceeded";
        cleanup_label = "failed-deadline";
        terminal_accepted = false;
    }
    let mut result = serde_json::json!({
        "schema": "copilot-cli-private-assessment-result.v1",
        "task_id": TASK_ID,
        "version": PRIVATE_ASSESSMENT_VERSION,
        "status": status,
        "terminal": terminal_label,
        "cleanup": cleanup_label,
        "owned_tasks_joined": task_joined,
        "working_resource_released": resource_released,
        "process_stopped": process_stopped,
        "process_exited": process_exited,
        "attempt_record_visible_before_version": attempt_visible_before_version,
        "prompt_record_visible_before_send": prompt_visible_before_send,
        "plan_sha256": plan_sha256,
        "runner_sha256": runner_sha256,
                    "rust_runner_sha256": rust_runner_sha256,
                    "authority_binding_sha256": authority_binding_sha256,
                    "reviewed_head": reviewed_head,
                    "enable_command": enable_command,
                    "source_manifest_sha256": source_manifest_sha256,
        "execution_host_id_sha256": binding.execution_host_id_sha256,
        "executable_ref_sha256": binding.executable_ref_sha256,
        "environment_ref_sha256": binding.environment_ref_sha256,
        "executable_bytes_sha256": binding.executable_bytes_sha256,
        "executable_digest_after_cleanup": executable_digest_after_cleanup,
        "identity_drift_after_cleanup": identity_drift_after_cleanup,
        "protected_executable_sha256": scratch.protected_executable.map(|protected| &protected.sha256),
        "protected_executable_sha256_after_cleanup": protected_executable_sha256_after_cleanup,
        "protected_executable_drift": protected_executable_drift,
        "protected_executable_removed": protected_executable_removed,
        "wrapper_archive_sha256": binding.wrapper_archive_sha256,
        "native_archive_sha256": binding.native_archive_sha256,
        "argv": ["--acp", "--stdio"],
        "task_directory_sha256_before": before_tree_sha256,
        "task_directory_sha256_after": after_tree_sha256,
        "effect_count": effect_count,
        "action_id": action_id,
        "action_count": action_count,
        "action_content_matches_requested_new_bytes": action_content_matches,
        "permission_action_metadata_verified": permission_action_metadata_verified,
        "permission_request": permission_request,
        "permission_request_count": permission_request_count,
        "permission_action_id": permission_action_id,
        "action_cancelled": action_cancelled,
        "permission_cancelled": permission_cancelled,
        "session_cancel_sent": session_cancel_sent,
        "prompt_cancelled": prompt_cancelled,
        "terminal_accepted": terminal_accepted,
        "timed_out": timed_out,
        "cancel_request_sent": cancel_request_sent,
        "elapsed_milliseconds_before_result_fsync": elapsed_before_result_fsync.as_millis(),
        "original_started": original_started
    });
    let result_write = write_exclusive_fsynced(&result_path, &result);
    if result_write.is_err() {
        status = "result-record-write-failed";
        terminal_accepted = false;
        result["status"] = serde_json::Value::String(status.to_owned());
        result["terminal_accepted"] = serde_json::Value::Bool(false);
    }
    let attempt_record = serde_json::from_slice(
        &fs::read(&attempt_path).expect("consumed invocation record remains readable"),
    )
    .expect("invocation record round-trips");
    let prompt_record = prompt_record_path.as_ref().map(|path| {
        serde_json::from_slice(&fs::read(path).expect("consumed prompt record remains readable"))
            .expect("prompt record round-trips")
    });
    let result_record = fs::read(&result_path)
        .ok()
        .and_then(|bytes| serde_json::from_slice(&bytes).ok())
        .unwrap_or(result);
    let elapsed = started.elapsed();
    let record_files_private = [
        Some(attempt_path.as_path()),
        prompt_record_path.as_deref(),
        Some(result_path.as_path()),
    ]
    .into_iter()
    .flatten()
    .all(|path| {
        fs::symlink_metadata(path)
            .is_ok_and(|metadata| metadata.is_file() && metadata.permissions().mode() & 0o077 == 0)
    });
    RunEvidence {
        status,
        terminal: terminal_label,
        cleanup: cleanup_label,
        task_joined,
        resource_released,
        process_stopped,
        process_exited,
        attempt_visible_before_version,
        prompt_visible_before_send,
        elapsed,
        timed_out,
        cancel_request_sent,
        action_id,
        action_count,
        action_content_matches,
        permission_action_metadata_verified,
        terminal_accepted,
        action_cancelled,
        permission_request,
        permission_request_count,
        permission_action_id,
        permission_cancelled,
        session_cancel_sent,
        prompt_cancelled,
        original_started,
        before_tree_sha256,
        after_tree_sha256,
        attempt_record,
        prompt_record,
        result_record,
        record_files_private,
        protected_executable_removed,
    }
}

#[derive(Clone, Debug)]
struct PlannerBoundContinuation {
    reviewed_head: String,
    plan_sha256: String,
    binding_sha256: String,
    enable_command: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OriginalRunDenied {
    Disabled,
    ContinuationMismatch,
    HostInputMismatch,
    ExecutableIdentityMismatch,
    ScratchMismatch,
    RecordRootUnsafe,
}

fn authorize_original_run(
    plan: &serde_json::Value,
    continuation: Option<&PlannerBoundContinuation>,
) -> Result<(String, String), OriginalRunDenied> {
    if plan
        .get("original_execution_enabled")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        return Err(OriginalRunDenied::Disabled);
    }
    if plan
        .get("preparation_originals_run")
        .and_then(serde_json::Value::as_u64)
        != Some(0)
        || plan
            .get("task")
            .and_then(|value| value.get("number"))
            .and_then(serde_json::Value::as_u64)
            != Some(119)
        || plan
            .get("task")
            .and_then(|value| value.get("id"))
            .and_then(serde_json::Value::as_str)
            != Some(TASK_ID)
        || plan
            .get("task")
            .and_then(|value| value.get("run_id"))
            .and_then(serde_json::Value::as_str)
            != Some("9adcb021-e36b-471e-bf53-ad2b741a1b71")
        || plan
            .get("qualification")
            .and_then(|value| value.get("public_qualified_point"))
            .and_then(serde_json::Value::as_str)
            != Some("1.0.80")
        || plan
            .get("qualification")
            .and_then(|value| value.get("assessment_point"))
            .and_then(serde_json::Value::as_str)
            != Some(PRIVATE_ASSESSMENT_VERSION)
    {
        return Err(OriginalRunDenied::ContinuationMismatch);
    }
    let artifacts = plan
        .get("artifacts")
        .ok_or(OriginalRunDenied::ContinuationMismatch)?;
    for (key, expected) in [
        (
            "wrapper_archive_sha256",
            crate::assessment::WRAPPER_ARCHIVE_SHA256,
        ),
        (
            "native_archive_sha256",
            crate::assessment::NATIVE_ARCHIVE_SHA256,
        ),
        (
            "native_executable_sha256",
            crate::assessment::NATIVE_EXECUTABLE_SHA256,
        ),
    ] {
        if artifacts.get(key).and_then(serde_json::Value::as_str) != Some(expected) {
            return Err(OriginalRunDenied::ContinuationMismatch);
        }
    }
    if plan
        .get("production_route")
        .and_then(|value| value.get("argv"))
        != Some(&serde_json::json!(["--acp", "--stdio"]))
        || plan
            .get("production_route")
            .and_then(|value| value.get("executable_ref"))
            .and_then(|value| value.get("sha256"))
            .and_then(serde_json::Value::as_str)
            != Some(crate::assessment::NATIVE_EXECUTABLE_SHA256)
        || plan
            .get("assessment_admission")
            .and_then(|value| value.get("visibility"))
            .and_then(serde_json::Value::as_str)
            != Some("crate-internal test build only; cfg(test); absent from ordinary consumers")
    {
        return Err(OriginalRunDenied::ContinuationMismatch);
    }
    validate_plan_source_manifest(plan)?;
    let attempt = plan
        .get("attempt")
        .and_then(serde_json::Value::as_object)
        .ok_or(OriginalRunDenied::ContinuationMismatch)?;
    let records_enabled = plan
        .get("records")
        .and_then(|value| value.get("original_execution_enabled"))
        .and_then(serde_json::Value::as_bool);
    if attempt
        .get("invocations_consumed_before")
        .and_then(serde_json::Value::as_u64)
        != Some(3)
        || attempt
            .get("prompts_consumed_before")
            .and_then(serde_json::Value::as_u64)
            != Some(2)
        || attempt
            .get("shared_prompt_slots_remaining_before")
            .and_then(serde_json::Value::as_u64)
            != Some(1)
        || attempt
            .get("maximum_invocations")
            .and_then(serde_json::Value::as_u64)
            != Some(1)
        || attempt
            .get("maximum_prompts")
            .and_then(serde_json::Value::as_u64)
            != Some(1)
        || attempt
            .get("maximum_seconds_including_cleanup")
            .and_then(serde_json::Value::as_u64)
            != Some(60)
        || attempt
            .get("cleanup_seconds")
            .and_then(serde_json::Value::as_u64)
            != Some(3)
        || attempt.get("retries").and_then(serde_json::Value::as_u64) != Some(0)
        || attempt.get("resends").and_then(serde_json::Value::as_u64) != Some(0)
        || records_enabled != Some(true)
    {
        return Err(OriginalRunDenied::ContinuationMismatch);
    }
    let continuation = continuation.ok_or(OriginalRunDenied::ContinuationMismatch)?;
    if continuation.reviewed_head.len() != 40
        || !continuation
            .reviewed_head
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit())
    {
        return Err(OriginalRunDenied::ContinuationMismatch);
    }
    let plan_sha256 = sha256_reader(
        serde_json::to_vec(plan)
            .expect("validated assessment plan serializes")
            .as_slice(),
    )
    .expect("serialized plan hashes");
    let gate = plan
        .get("original_enable_gate")
        .and_then(serde_json::Value::as_object)
        .ok_or(OriginalRunDenied::ContinuationMismatch)?;
    if gate.get("state").and_then(serde_json::Value::as_str)
        != Some("enabled for planner-bound continuation")
        || gate
            .get("reviewed_head")
            .and_then(serde_json::Value::as_str)
            != Some(continuation.reviewed_head.as_str())
        || gate
            .get("enable_command")
            .and_then(serde_json::Value::as_str)
            != Some(continuation.enable_command.as_str())
        || !is_lower_hex_digest(&continuation.binding_sha256)
        || gate
            .get("binding_sha256")
            .and_then(serde_json::Value::as_str)
            != Some(continuation.binding_sha256.as_str())
        || continuation.plan_sha256 != plan_sha256
    {
        return Err(OriginalRunDenied::ContinuationMismatch);
    }
    let runner = plan
        .get("runner")
        .and_then(serde_json::Value::as_object)
        .ok_or(OriginalRunDenied::ContinuationMismatch)?;
    let runner_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../").join(
        runner
            .get("path")
            .and_then(serde_json::Value::as_str)
            .ok_or(OriginalRunDenied::ContinuationMismatch)?,
    );
    let runner_sha256 = sha256_file(&runner_path);
    if runner.get("sha256").and_then(serde_json::Value::as_str) != Some(runner_sha256.as_str()) {
        return Err(OriginalRunDenied::ContinuationMismatch);
    }
    let rust_runner_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/assessment_runner.rs");
    let rust_runner_sha256 = sha256_file(&rust_runner_path);
    if runner
        .get("implementation_path")
        .and_then(serde_json::Value::as_str)
        != Some("crates/swallowtail-adapter-copilot-cli/src/assessment_runner.rs")
        || runner
            .get("implementation_sha256")
            .and_then(serde_json::Value::as_str)
            != Some(rust_runner_sha256.as_str())
    {
        return Err(OriginalRunDenied::ContinuationMismatch);
    }
    let source_file = plan
        .get("prepared_path_sources")
        .and_then(|value| value.get("files"))
        .and_then(serde_json::Value::as_array)
        .and_then(|files| {
            files.iter().find(|file| {
                file.get("path").and_then(serde_json::Value::as_str)
                    == Some("crates/swallowtail-adapter-copilot-cli/src/assessment_runner.rs")
            })
        })
        .ok_or(OriginalRunDenied::ContinuationMismatch)?;
    if source_file
        .get("sha256")
        .and_then(serde_json::Value::as_str)
        != Some(rust_runner_sha256.as_str())
    {
        return Err(OriginalRunDenied::ContinuationMismatch);
    }
    Ok((plan_sha256, runner_sha256))
}

fn plan_binding_metadata(
    plan: &serde_json::Value,
) -> (String, Option<String>, Option<String>, String) {
    let gate = plan
        .get("original_enable_gate")
        .cloned()
        .unwrap_or(serde_json::Value::Null);
    let authority_sha256 = serde_json::to_vec(&gate)
        .map(|bytes| sha256_reader(bytes.as_slice()).expect("authority binding hashes"))
        .expect("authority binding serializes");
    let reviewed_head = gate
        .get("reviewed_head")
        .and_then(serde_json::Value::as_str)
        .map(str::to_owned);
    let enable_command = gate
        .get("enable_command")
        .and_then(serde_json::Value::as_str)
        .map(str::to_owned);
    let source_manifest_sha256 = plan
        .get("prepared_path_sources")
        .and_then(|manifest| manifest.get("sha256"))
        .and_then(serde_json::Value::as_str)
        .expect("prepared-path source manifest digest exists")
        .to_owned();
    (
        authority_sha256,
        reviewed_head,
        enable_command,
        source_manifest_sha256,
    )
}

fn validate_plan_source_manifest(plan: &serde_json::Value) -> Result<(), OriginalRunDenied> {
    let manifest = plan
        .get("prepared_path_sources")
        .and_then(serde_json::Value::as_object)
        .ok_or(OriginalRunDenied::ContinuationMismatch)?;
    let files = manifest
        .get("files")
        .and_then(serde_json::Value::as_array)
        .ok_or(OriginalRunDenied::ContinuationMismatch)?;
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../");
    for entry in files {
        let path = entry
            .get("path")
            .and_then(serde_json::Value::as_str)
            .ok_or(OriginalRunDenied::ContinuationMismatch)?;
        let relative = Path::new(path);
        if relative.is_absolute()
            || relative
                .components()
                .any(|component| !matches!(component, std::path::Component::Normal(_)))
        {
            return Err(OriginalRunDenied::ContinuationMismatch);
        }
        let expected = entry
            .get("sha256")
            .and_then(serde_json::Value::as_str)
            .ok_or(OriginalRunDenied::ContinuationMismatch)?;
        let source = root.join(relative);
        let metadata =
            fs::symlink_metadata(&source).map_err(|_| OriginalRunDenied::ContinuationMismatch)?;
        if !metadata.is_file() || metadata.file_type().is_symlink() {
            return Err(OriginalRunDenied::ContinuationMismatch);
        }
        if sha256_file(&source) != expected {
            return Err(OriginalRunDenied::ContinuationMismatch);
        }
    }
    let encoded_files =
        serde_json::to_vec(files).map_err(|_| OriginalRunDenied::ContinuationMismatch)?;
    let actual_manifest_sha256 = sha256_reader(encoded_files.as_slice())
        .map_err(|_| OriginalRunDenied::ContinuationMismatch)?;
    if manifest.get("sha256").and_then(serde_json::Value::as_str)
        != Some(actual_manifest_sha256.as_str())
    {
        return Err(OriginalRunDenied::ContinuationMismatch);
    }
    Ok(())
}

/// Executes only after the plan contains the exact reviewed-head binding.
/// Test-owned policies exercise the same entry with a local fake child while
/// the committed production plan remains disabled.
#[allow(clippy::too_many_arguments)]
fn run_original_task_once(
    plan: &serde_json::Value,
    continuation: Option<&PlannerBoundContinuation>,
    input: CopilotCliPreparationInput,
    probe_request: RequestId,
    session_request: RequestId,
    working_resource: WorkingResourceRef,
    local: &LocalHostServices,
    task_directory: &Path,
    records_directory: &Path,
    trace: Option<&Path>,
    policy: OriginalRunPolicy,
) -> Result<RunEvidence, OriginalRunDenied> {
    let OriginalRunPolicy {
        total,
        cleanup,
        expected_executable_sha256,
        original_started,
        execution_mode,
    } = policy;
    run_original_task_with_budget(
        plan,
        continuation,
        input,
        probe_request,
        session_request,
        working_resource,
        local,
        task_directory,
        records_directory,
        trace,
        total,
        cleanup,
        &expected_executable_sha256,
        original_started,
        execution_mode,
    )
}

struct OriginalRunPolicy {
    total: Duration,
    cleanup: Duration,
    expected_executable_sha256: String,
    original_started: bool,
    execution_mode: &'static str,
}

impl OriginalRunPolicy {
    fn production() -> Self {
        Self {
            total: ORIGINAL_TOTAL_BUDGET,
            cleanup: ORIGINAL_CLEANUP_BUDGET,
            expected_executable_sha256: crate::assessment::NATIVE_EXECUTABLE_SHA256.to_owned(),
            original_started: true,
            execution_mode: "approved-local-process-host-original",
        }
    }
}

struct OriginalEntryBinding {
    reviewed_head: String,
    plan_sha256: String,
    enable_command: String,
    execution_host_id: String,
    executable_ref: String,
    environment_ref: String,
    working_resource_ref: String,
    home_directory: String,
}

struct OriginalEntryHost {
    local: LocalHostServices,
    input: CopilotCliPreparationInput,
    working_resource: WorkingResourceRef,
    task_directory: PathBuf,
    trace: Option<PathBuf>,
}

fn production_original_entry_arguments() -> Result<Vec<OsString>, OriginalRunDenied> {
    let payload = std::env::var_os("SWALLOWTAIL_COPILOT_ACP_BINDING_PAYLOAD")
        .ok_or(OriginalRunDenied::ContinuationMismatch)?;
    Ok(vec![OsString::from("--payload"), payload])
}

fn invoke_reviewed_original_from_environment() -> Result<RunEvidence, OriginalRunDenied> {
    let arguments = production_original_entry_arguments()?;
    let plan_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/copilot-cli-acp-currentness-1.0.95/private-assessment-plan.json");
    let plan = read_json_object(&plan_path).map_err(|_| OriginalRunDenied::ContinuationMismatch)?;
    dispatch_original_entry(&arguments, &plan, build_production_original_host)
}

fn dispatch_original_entry(
    arguments: &[OsString],
    plan: &serde_json::Value,
    resolve_host: impl FnOnce(&OriginalEntryBinding) -> Result<OriginalEntryHost, OriginalRunDenied>,
) -> Result<RunEvidence, OriginalRunDenied> {
    let payload_path = parse_original_entry_arguments(arguments)?;
    let payload = read_bounded_original_binding(&payload_path)?;
    let binding = parse_original_entry_binding(&payload)?;
    let continuation = PlannerBoundContinuation {
        reviewed_head: binding.reviewed_head.clone(),
        plan_sha256: binding.plan_sha256.clone(),
        binding_sha256: original_entry_binding_sha256(&binding)?,
        enable_command: binding.enable_command.clone(),
    };

    // The committed disabled plan is checked before resolving host IDs, paths,
    // environment bindings, scratch, or persistent records.
    authorize_original_run(plan, Some(&continuation))?;
    let host = resolve_host(&binding)?;
    let records_directory = records_directory(plan, &binding.home_directory)?;
    run_original_task_once(
        plan,
        Some(&continuation),
        host.input,
        RequestId::new("copilot-cli.assessment.original.probe")
            .map_err(|_| OriginalRunDenied::ContinuationMismatch)?,
        RequestId::new("copilot-cli.assessment.original.session")
            .map_err(|_| OriginalRunDenied::ContinuationMismatch)?,
        host.working_resource,
        &host.local,
        &host.task_directory,
        &records_directory,
        host.trace.as_deref(),
        OriginalRunPolicy::production(),
    )
}

fn parse_original_entry_arguments(arguments: &[OsString]) -> Result<PathBuf, OriginalRunDenied> {
    if arguments.len() != 2 || arguments[0] != OsString::from("--payload") {
        return Err(OriginalRunDenied::ContinuationMismatch);
    }
    let path = PathBuf::from(&arguments[1]);
    if !path.is_absolute()
        || path.as_os_str().len() > 1024
        || path.components().any(|component| {
            matches!(
                component,
                std::path::Component::ParentDir | std::path::Component::CurDir
            )
        })
    {
        return Err(OriginalRunDenied::ContinuationMismatch);
    }
    Ok(path)
}

fn read_bounded_original_binding(path: &Path) -> Result<Vec<u8>, OriginalRunDenied> {
    use std::os::unix::fs::{MetadataExt, OpenOptionsExt};

    let metadata =
        fs::symlink_metadata(path).map_err(|_| OriginalRunDenied::ContinuationMismatch)?;
    if !metadata.is_file()
        || metadata.file_type().is_symlink()
        || metadata.len() > ORIGINAL_ENTRY_PAYLOAD_MAX_BYTES
        || metadata.mode() & 0o7777 != 0o600
    {
        return Err(OriginalRunDenied::ContinuationMismatch);
    }
    let mut file = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW)
        .open(path)
        .map_err(|_| OriginalRunDenied::ContinuationMismatch)?;
    let opened = file
        .metadata()
        .map_err(|_| OriginalRunDenied::ContinuationMismatch)?;
    if opened.dev() != metadata.dev()
        || opened.ino() != metadata.ino()
        || opened.len() > ORIGINAL_ENTRY_PAYLOAD_MAX_BYTES
        || opened.mode() & 0o7777 != 0o600
    {
        return Err(OriginalRunDenied::ContinuationMismatch);
    }
    let mut bytes = Vec::with_capacity(opened.len() as usize);
    use std::io::Read;
    (&mut file)
        .take(ORIGINAL_ENTRY_PAYLOAD_MAX_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| OriginalRunDenied::ContinuationMismatch)?;
    if bytes.len() as u64 > ORIGINAL_ENTRY_PAYLOAD_MAX_BYTES {
        return Err(OriginalRunDenied::ContinuationMismatch);
    }
    Ok(bytes)
}

fn parse_original_entry_binding(bytes: &[u8]) -> Result<OriginalEntryBinding, OriginalRunDenied> {
    let value: serde_json::Value =
        serde_json::from_slice(bytes).map_err(|_| OriginalRunDenied::ContinuationMismatch)?;
    let object = value
        .as_object()
        .ok_or(OriginalRunDenied::ContinuationMismatch)?;
    const KEYS: [&str; 9] = [
        "schema",
        "reviewed_head",
        "plan_sha256",
        "enable_command",
        "execution_host_id",
        "executable_ref",
        "environment_ref",
        "working_resource_ref",
        "home_directory",
    ];
    if object.len() != KEYS.len() || KEYS.iter().any(|key| !object.contains_key(*key)) {
        return Err(OriginalRunDenied::ContinuationMismatch);
    }
    let get = |key: &str| {
        object
            .get(key)
            .and_then(serde_json::Value::as_str)
            .map(str::to_owned)
            .ok_or(OriginalRunDenied::ContinuationMismatch)
    };
    let binding = OriginalEntryBinding {
        reviewed_head: get("reviewed_head")?,
        plan_sha256: get("plan_sha256")?,
        enable_command: get("enable_command")?,
        execution_host_id: get("execution_host_id")?,
        executable_ref: get("executable_ref")?,
        environment_ref: get("environment_ref")?,
        working_resource_ref: get("working_resource_ref")?,
        home_directory: get("home_directory")?,
    };
    if get("schema")? != ORIGINAL_ENTRY_PAYLOAD_SCHEMA
        || binding.reviewed_head.len() != 40
        || !binding
            .reviewed_head
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit())
        || !is_lower_hex_digest(&binding.plan_sha256)
        || binding.enable_command != FROZEN_ENABLE_COMMAND
        || !is_safe_host_id(&binding.execution_host_id)
        || !is_safe_host_id(&binding.environment_ref)
        || !is_safe_host_id(&binding.working_resource_ref)
        || !is_absolute_plain_path(&binding.executable_ref)
        || !is_absolute_plain_path(&binding.home_directory)
    {
        return Err(OriginalRunDenied::ContinuationMismatch);
    }
    Ok(binding)
}

fn original_entry_binding_sha256(
    binding: &OriginalEntryBinding,
) -> Result<String, OriginalRunDenied> {
    let safe_binding = BTreeMap::from([
        ("execution_host_id", binding.execution_host_id.as_str()),
        ("executable_ref", binding.executable_ref.as_str()),
        ("environment_ref", binding.environment_ref.as_str()),
        (
            "working_resource_ref",
            binding.working_resource_ref.as_str(),
        ),
        ("home_directory", binding.home_directory.as_str()),
    ]);
    let bytes =
        serde_json::to_vec(&safe_binding).map_err(|_| OriginalRunDenied::ContinuationMismatch)?;
    sha256_reader(bytes.as_slice()).map_err(|_| OriginalRunDenied::ContinuationMismatch)
}

fn is_lower_hex_digest(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn is_safe_host_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
}

fn is_absolute_plain_path(value: &str) -> bool {
    let path = Path::new(value);
    path.is_absolute()
        && value.len() <= 1024
        && path.components().all(|component| {
            matches!(
                component,
                std::path::Component::RootDir | std::path::Component::Normal(_)
            )
        })
}

fn read_json_object(path: &Path) -> std::io::Result<serde_json::Value> {
    let bytes = fs::read(path)?;
    let value: serde_json::Value = serde_json::from_slice(&bytes)
        .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error))?;
    if !value.is_object() {
        return Err(std::io::Error::other("assessment plan is not an object"));
    }
    Ok(value)
}

fn records_directory(
    plan: &serde_json::Value,
    home_directory: &str,
) -> Result<PathBuf, OriginalRunDenied> {
    let root = plan
        .get("records")
        .and_then(|value| value.get("root"))
        .and_then(serde_json::Value::as_str)
        .and_then(|root| root.strip_prefix("$HOME/"))
        .ok_or(OriginalRunDenied::RecordRootUnsafe)?;
    if !is_absolute_plain_path(home_directory) {
        return Err(OriginalRunDenied::RecordRootUnsafe);
    }
    Ok(Path::new(home_directory).join(root))
}

fn build_production_original_host(
    binding: &OriginalEntryBinding,
) -> Result<OriginalEntryHost, OriginalRunDenied> {
    let home = PathBuf::from(&binding.home_directory);
    if std::env::var_os("HOME").as_deref() != Some(home.as_os_str()) {
        return Err(OriginalRunDenied::HostInputMismatch);
    }
    let home_metadata =
        fs::symlink_metadata(&home).map_err(|_| OriginalRunDenied::HostInputMismatch)?;
    if !home_metadata.is_dir() || home_metadata.file_type().is_symlink() {
        return Err(OriginalRunDenied::HostInputMismatch);
    }
    let execution_host_id = ExecutionHostId::new(binding.execution_host_id.clone())
        .map_err(|_| OriginalRunDenied::HostInputMismatch)?;
    let executable_ref = ExecutableRef::new(binding.executable_ref.clone())
        .map_err(|_| OriginalRunDenied::HostInputMismatch)?;
    let environment_ref = EnvironmentRef::new(binding.environment_ref.clone())
        .map_err(|_| OriginalRunDenied::HostInputMismatch)?;
    let working_resource = WorkingResourceRef::new(binding.working_resource_ref.clone())
        .map_err(|_| OriginalRunDenied::HostInputMismatch)?;
    let executable_path = PathBuf::from(executable_ref.as_host_value());
    let task_directory = fresh_original_task_directory()?;
    create_sentinel(&task_directory.join(SENTINEL_PATH))?;
    let local = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_executable(executable_ref.clone(), executable_path)
        // HOME is the sole environment value passed to the child so its
        // existing host-owned login/config and configured default remain in use.
        .approve_environment(
            environment_ref.clone(),
            [(OsString::from("HOME"), home.as_os_str().to_owned())],
        )
        .approve_working_resource(working_resource.clone(), task_directory.clone())
        .build_services(execution_host_id.clone());
    let input = original_preparation_input(execution_host_id, executable_ref, environment_ref);
    Ok(OriginalEntryHost {
        local,
        input,
        working_resource,
        task_directory,
        trace: None,
    })
}

fn original_preparation_input(
    host: ExecutionHostId,
    executable: ExecutableRef,
    environment: EnvironmentRef,
) -> CopilotCliPreparationInput {
    let profile = AccessProfileId::new("copilot-cli.assessment.host-account")
        .expect("static profile id is valid");
    CopilotCliPreparationInput::new(
        ConfiguredInstanceId::new("copilot-cli.assessment.instance")
            .expect("static instance id is valid"),
        InstanceRevision::new("1").expect("static revision is valid"),
        host,
        InstalledExecutableTarget::new(
            executable,
            InterfaceVersionAxis::new(COPILOT_CLI_PACKAGE_AXIS)
                .expect("static version axis is valid"),
        ),
        environment,
        copilot_cli_host_account_access_profile(profile.clone()),
        PreparedAccessEvidence::caller_asserted(AccessStatus::new(
            profile,
            CredentialState::NotRequired,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::ExperimentalObserved,
        )),
    )
}

fn fresh_original_task_directory() -> Result<PathBuf, OriginalRunDenied> {
    let output = Command::new("mktemp")
        .args(["-d", "-t", "copilot-acp-original-task"])
        .output()
        .map_err(|_| OriginalRunDenied::ScratchMismatch)?;
    if !output.status.success() {
        return Err(OriginalRunDenied::ScratchMismatch);
    }
    let path = PathBuf::from(
        String::from_utf8(output.stdout)
            .map_err(|_| OriginalRunDenied::ScratchMismatch)?
            .trim(),
    );
    if !path.is_absolute() {
        return Err(OriginalRunDenied::ScratchMismatch);
    }
    Ok(path)
}

fn create_sentinel(path: &Path) -> Result<(), OriginalRunDenied> {
    use std::os::unix::fs::OpenOptionsExt;
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(path)
        .map_err(|_| OriginalRunDenied::ScratchMismatch)?;
    file.write_all(SENTINEL_BEFORE)
        .and_then(|()| file.sync_all())
        .map_err(|_| OriginalRunDenied::ScratchMismatch)?;
    File::open(path.parent().ok_or(OriginalRunDenied::ScratchMismatch)?)
        .and_then(|directory| directory.sync_all())
        .map_err(|_| OriginalRunDenied::ScratchMismatch)
}

#[allow(clippy::too_many_arguments)]
fn run_original_task_with_budget(
    plan: &serde_json::Value,
    continuation: Option<&PlannerBoundContinuation>,
    input: CopilotCliPreparationInput,
    probe_request: RequestId,
    session_request: RequestId,
    working_resource: WorkingResourceRef,
    local: &LocalHostServices,
    task_directory: &Path,
    records_directory: &Path,
    trace: Option<&Path>,
    total: Duration,
    cleanup: Duration,
    expected_executable_sha256: &str,
    original_started: bool,
    execution_mode: &'static str,
) -> Result<RunEvidence, OriginalRunDenied> {
    let started = Instant::now();
    if total <= cleanup {
        return Err(OriginalRunDenied::ContinuationMismatch);
    }
    let (plan_sha256, runner_sha256) = authorize_original_run(plan, continuation)?;
    let services = local.services().clone();
    if input.execution_host_id() != services.execution_host_id() {
        return Err(OriginalRunDenied::HostInputMismatch);
    }
    if !task_scratch_is_sentinel_only(task_directory) {
        return Err(OriginalRunDenied::ScratchMismatch);
    }
    let executable_ref = input.target().executable().clone();
    let executable_path = Path::new(executable_ref.as_host_value());
    let host_binding = crate::assessment::capture_host_binding(
        input.execution_host_id(),
        &executable_ref,
        input.environment(),
        File::open(executable_path).map_err(|_| OriginalRunDenied::ExecutableIdentityMismatch)?,
        expected_executable_sha256,
    )
    .map_err(|_| OriginalRunDenied::ExecutableIdentityMismatch)?;
    let protected_executable =
        ProtectedExecutable::copy_from(executable_path, expected_executable_sha256)
            .map_err(|_| OriginalRunDenied::ExecutableIdentityMismatch)?;
    let planned_records_root = plan
        .get("records")
        .and_then(|value| value.get("root"))
        .and_then(serde_json::Value::as_str)
        .and_then(|root| root.strip_prefix("$HOME/"))
        .and_then(|root| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(root)))
        .ok_or(OriginalRunDenied::RecordRootUnsafe)?;
    if records_directory != planned_records_root.as_path() {
        return Err(OriginalRunDenied::RecordRootUnsafe);
    }
    if ensure_private_record_directory(records_directory).is_err() {
        return Err(OriginalRunDenied::RecordRootUnsafe);
    }

    let action_window = total - cleanup;
    let action_deadline = local.deadline_after(action_window.saturating_sub(started.elapsed()));
    let before_tree_sha256 = tree_sha256(task_directory);
    let (authority_binding_sha256, reviewed_head, enable_command, source_manifest_sha256) =
        plan_binding_metadata(plan);
    let rust_runner_sha256 =
        sha256_file(&Path::new(env!("CARGO_MANIFEST_DIR")).join("src/assessment_runner.rs"));
    let attempt_path = records_directory.join("1.0.95-attempt-4.json");
    let attempt = serde_json::json!({
        "schema": "copilot-cli-private-assessment-consumed.v1",
        "phase": "invocation-consumed-before-prepared-open",
        "task_id": TASK_ID,
        "version": PRIVATE_ASSESSMENT_VERSION,
        "invocation_number": 4,
        "plan_sha256": plan_sha256,
        "runner_sha256": runner_sha256,
                    "rust_runner_sha256": rust_runner_sha256,
                    "authority_binding_sha256": authority_binding_sha256,
                    "reviewed_head": reviewed_head,
                    "enable_command": enable_command,
                    "source_manifest_sha256": source_manifest_sha256,
        "execution_host_id": host_binding.execution_host_id,
        "execution_host_id_sha256": host_binding.execution_host_id_sha256,
        "executable_ref": host_binding.executable_ref,
        "executable_ref_sha256": host_binding.executable_ref_sha256,
        "environment_ref": host_binding.environment_ref,
        "environment_ref_sha256": host_binding.environment_ref_sha256,
        "executable_bytes_sha256": host_binding.executable_bytes_sha256,
        "wrapper_archive_sha256": host_binding.wrapper_archive_sha256,
        "native_archive_sha256": host_binding.native_archive_sha256,
        "argv": ["--acp", "--stdio"],
        "mode": execution_mode,
        "original_started": original_started
    });
    write_exclusive_fsynced(&attempt_path, &attempt)
        .map_err(|_| OriginalRunDenied::RecordRootUnsafe)?;
    let probe = CopilotCliPreparationProbe::new(
        probe_request,
        ScopeId::new(format!("copilot-cli.assessment:{TASK_ID}:probe"))
            .expect("static task scope is valid"),
        action_deadline,
        DiscoveryCancellation::new(),
    );
    let prepared = match block_on(prepare_copilot_cli_acp_for_assessment_with_reader(
        input,
        probe,
        services.clone(),
        File::open(executable_path).map_err(|_| OriginalRunDenied::ExecutableIdentityMismatch)?,
        expected_executable_sha256,
    )) {
        Ok(prepared) => prepared,
        Err(_) => {
            let result = serde_json::json!({
                "schema": "copilot-cli-private-assessment-result.v1",
                "task_id": TASK_ID,
                "version": PRIVATE_ASSESSMENT_VERSION,
                "status": "preparation-failed",
                "cleanup": "not-started",
                "plan_sha256": plan_sha256,
                "runner_sha256": runner_sha256,
                    "rust_runner_sha256": rust_runner_sha256,
                    "authority_binding_sha256": authority_binding_sha256,
                    "reviewed_head": reviewed_head,
                    "enable_command": enable_command,
                    "source_manifest_sha256": source_manifest_sha256,
                "executable_bytes_sha256": host_binding.executable_bytes_sha256,
                "original_started": original_started
            });
            let result_path = records_directory.join("1.0.95-execution.json");
            let _ = write_exclusive_fsynced(&result_path, &result);
            return Err(OriginalRunDenied::ExecutableIdentityMismatch);
        }
    };
    let session = match prepared.prepare_session(CopilotCliSessionProfileInput::new(
        session_request,
        working_resource,
    )) {
        Ok(session) => session,
        Err(_) => {
            let result = serde_json::json!({
                    "schema": "copilot-cli-private-assessment-result.v1",
                    "task_id": TASK_ID,
                    "version": PRIVATE_ASSESSMENT_VERSION,
                    "status": "session-preparation-failed",
                    "cleanup": "not-started",
                    "plan_sha256": plan_sha256,
                    "runner_sha256": runner_sha256,
            "rust_runner_sha256": rust_runner_sha256,
            "authority_binding_sha256": authority_binding_sha256,
            "reviewed_head": reviewed_head,
            "enable_command": enable_command,
            "source_manifest_sha256": source_manifest_sha256,
                    "executable_bytes_sha256": host_binding.executable_bytes_sha256,
                    "original_started": original_started
                });
            let result_path = records_directory.join("1.0.95-execution.json");
            let _ = write_exclusive_fsynced(&result_path, &result);
            return Err(OriginalRunDenied::HostInputMismatch);
        }
    };
    Ok(drive_prepared_session(
        session,
        local,
        RunScratch {
            task: task_directory,
            records: records_directory,
            trace,
            protected_executable: Some(&protected_executable),
        },
        services,
        started,
        action_window,
        action_deadline,
        total,
        plan_sha256,
        runner_sha256,
        rust_runner_sha256,
        authority_binding_sha256,
        reviewed_head,
        enable_command,
        source_manifest_sha256,
        host_binding,
        consumed_record_visible_before_effect(
            &attempt_path,
            "invocation-consumed-before-prepared-open",
        ),
        attempt_path,
        before_tree_sha256,
        original_started,
        execution_mode,
    ))
}

fn ensure_private_record_directory(path: &Path) -> std::io::Result<()> {
    use std::os::unix::fs::{MetadataExt, PermissionsExt};
    fs::create_dir_all(path)?;
    let metadata = fs::symlink_metadata(path)?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(std::io::Error::other(
            "assessment record root is not a real directory",
        ));
    }
    fs::set_permissions(path, fs::Permissions::from_mode(0o700))?;
    File::open(
        path.parent()
            .ok_or_else(|| std::io::Error::other("record root has no parent"))?,
    )?
    .sync_all()?;
    let updated = fs::symlink_metadata(path)?;
    if updated.dev() != metadata.dev()
        || updated.ino() != metadata.ino()
        || updated.mode() & 0o077 != 0
    {
        return Err(std::io::Error::other(
            "assessment record root changed while validating",
        ));
    }
    Ok(())
}

fn task_scratch_is_sentinel_only(path: &Path) -> bool {
    if !fs::symlink_metadata(path)
        .is_ok_and(|metadata| metadata.is_dir() && !metadata.file_type().is_symlink())
    {
        return false;
    }
    let Ok(entries) = fs::read_dir(path) else {
        return false;
    };
    let entries = entries.filter_map(Result::ok).collect::<Vec<_>>();
    entries.len() == 1
        && entries[0].file_name() == SENTINEL_PATH
        && fs::symlink_metadata(entries[0].path())
            .is_ok_and(|metadata| metadata.is_file() && !metadata.file_type().is_symlink())
        && fs::read(entries[0].path()).is_ok_and(|value| value.as_slice() == SENTINEL_BEFORE)
}

fn consumed_record_visible_before_effect(path: &Path, phase: &str) -> bool {
    use std::os::unix::fs::PermissionsExt;

    fs::symlink_metadata(path).is_ok_and(|metadata| {
        metadata.is_file()
            && !metadata.file_type().is_symlink()
            && metadata.permissions().mode() & 0o777 == 0o600
            && fs::read(path).is_ok_and(|bytes| {
                serde_json::from_slice::<serde_json::Value>(&bytes).is_ok_and(|record| {
                    record.get("phase").and_then(|value| value.as_str()) == Some(phase)
                })
            })
    })
}

#[test]
fn original_ready_path_is_disabled_before_host_or_ledger_effects() {
    let scratch = RealScratch::new();
    let plan_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/copilot-cli-acp-currentness-1.0.95/private-assessment-plan.json");
    let plan: serde_json::Value =
        serde_json::from_slice(&fs::read(plan_path).expect("disabled plan fixture is readable"))
            .expect("disabled plan parses");
    let host = ExecutionHostId::new("assessment.disabled-original-host").expect("host id");
    let builder = LocalProcessHost::builder(LocalProcessLimits::default());
    let local = builder.build_services(host.clone());
    let input = preparation_input_with_executable(
        host,
        scratch.executable.to_str().expect("fixture path is UTF-8"),
    );
    let result = run_original_task_once(
        &plan,
        None,
        input,
        RequestId::new("assessment.disabled.probe").expect("probe request"),
        RequestId::new("assessment.disabled.session").expect("session request"),
        WorkingResourceRef::new("assessment.disabled.scratch").expect("resource ref"),
        &local,
        &scratch.task,
        &scratch.records,
        None,
        OriginalRunPolicy::production(),
    );
    assert!(matches!(result, Err(OriginalRunDenied::Disabled)));
    assert_eq!(
        fs::read_dir(&scratch.records)
            .expect("record directory exists")
            .count(),
        0
    );
    assert!(fs::read_to_string(&scratch.trace).is_err());
    let untrusted = PlannerBoundContinuation {
        reviewed_head: "untrusted-head".to_owned(),
        plan_sha256: "untrusted-plan".to_owned(),
        binding_sha256: "untrusted-binding".to_owned(),
        enable_command: "untrusted-command".to_owned(),
    };
    let denied = run_original_task_once(
        &plan,
        Some(&untrusted),
        preparation_input_with_executable(
            ExecutionHostId::new("assessment.disabled-original-host").expect("host id"),
            scratch.executable.to_str().expect("fixture path is UTF-8"),
        ),
        RequestId::new("assessment.disabled.probe").expect("probe request"),
        RequestId::new("assessment.disabled.session").expect("session request"),
        WorkingResourceRef::new("assessment.disabled.scratch").expect("resource ref"),
        &local,
        &scratch.task,
        &scratch.records,
        None,
        OriginalRunPolicy::production(),
    );
    assert!(matches!(denied, Err(OriginalRunDenied::Disabled)));
    assert_eq!(
        fs::read_dir(&scratch.records)
            .expect("record directory exists")
            .count(),
        0
    );
}

#[test]
fn real_local_prepared_path_timeout_cancels_joins_and_records_cleanup() {
    let total = Duration::from_secs(6);
    let cleanup = Duration::from_secs(2);
    assert_eq!(
        Duration::from_secs(60) - Duration::from_secs(3),
        Duration::from_secs(57)
    );
    let evidence = run_fake_original_branch("hang", total, cleanup);
    assert_eq!(evidence.status, "action-timeout-cancelled");
    assert_eq!(evidence.terminal, "cancelled");
    assert_eq!(evidence.cleanup, "clean");
    assert!(evidence.timed_out);
    assert!(evidence.cancel_request_sent);
    assert!(
        evidence.process_stopped,
        "host cleanup stops the hanging child"
    );
    assert!(
        evidence.process_exited,
        "the hanging child exits before the run returns"
    );
    assert!(
        evidence.task_joined,
        "owned task handles are joined before the run returns"
    );
    assert!(
        evidence.resource_released,
        "the approved working resource is released"
    );
    assert!(
        evidence.attempt_visible_before_version,
        "attempt record is durable before discovery launch"
    );
    assert!(
        evidence.prompt_visible_before_send,
        "prompt record is durable before prompt launch"
    );
    assert!(!evidence.original_started);
    assert!(evidence.terminal_accepted);
    assert!(
        evidence.elapsed < total,
        "real monotonic inclusive budget holds"
    );
    assert_eq!(evidence.before_tree_sha256, evidence.after_tree_sha256);
    assert_eq!(evidence.result_record["effect_count"], 0);
    assert_eq!(evidence.result_record["cleanup"], "clean");
    assert_eq!(
        evidence.attempt_record["phase"],
        "invocation-consumed-before-prepared-open"
    );
    assert_eq!(
        evidence.attempt_record["execution_host_id"],
        "assessment.real-local-host"
    );
    assert_eq!(
        evidence.attempt_record["environment_ref"],
        APPROVED_ENVIRONMENT
    );
    assert_eq!(
        evidence.attempt_record["original_started"],
        serde_json::Value::Bool(false)
    );
    assert_eq!(evidence.attempt_record["mode"], "fake-local-process-only");
    assert_eq!(
        evidence
            .prompt_record
            .as_ref()
            .expect("prompt record exists")["phase"],
        "prompt-consumed-before-prepared-turn"
    );
    assert_eq!(
        evidence.attempt_record["plan_sha256"],
        evidence
            .prompt_record
            .as_ref()
            .expect("prompt record exists")["plan_sha256"]
    );
    assert_eq!(
        evidence.attempt_record["execution_host_id"],
        evidence
            .prompt_record
            .as_ref()
            .expect("prompt record exists")["execution_host_id"]
    );
    assert_eq!(
        evidence.attempt_record["executable_ref"],
        evidence
            .prompt_record
            .as_ref()
            .expect("prompt record exists")["executable_ref"]
    );
    assert_eq!(
        evidence.attempt_record["environment_ref"],
        evidence
            .prompt_record
            .as_ref()
            .expect("prompt record exists")["environment_ref"]
    );
    let executable_ref = evidence.attempt_record["executable_ref"]
        .as_str()
        .expect("exact approved fake executable reference is retained");
    assert!(executable_ref.ends_with("copilot.py"));
    assert_eq!(
        evidence.attempt_record["executable_ref_sha256"],
        sha256_reader(executable_ref.as_bytes()).expect("executable ref hashes")
    );
    assert!(evidence.record_files_private);
    assert!(evidence.protected_executable_removed);
}

#[test]
fn prepared_path_fake_permission_and_action_correlation_are_preserved() {
    let plan_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/copilot-cli-acp-currentness-1.0.95/private-assessment-plan.json");
    let plan: serde_json::Value =
        serde_json::from_slice(&fs::read(plan_path).expect("plan fixture is readable"))
            .expect("plan parses");
    assert_eq!(
        plan["original_enable_gate"]["enable_command"],
        FROZEN_ENABLE_COMMAND
    );

    let prepared = run_with_local_host(
        "permission",
        Duration::from_secs(3),
        Duration::from_millis(500),
    );
    assert!(prepared.terminal_accepted);
    let normal = run_fake_original_branch(
        "permission",
        Duration::from_secs(3),
        Duration::from_millis(500),
    );
    assert_eq!(normal.status, "permission-cancelled");
    assert_eq!(normal.terminal, "permission-observed");
    assert_eq!(normal.cleanup, "clean");
    assert_eq!(normal.action_id.as_deref(), Some("sentinel-edit"));
    assert_eq!(normal.action_count, 1);
    assert!(normal.action_content_matches);
    assert!(normal.action_cancelled);
    assert_eq!(normal.permission_request.as_deref(), Some("acp:900"));
    assert_eq!(normal.permission_request_count, 1);
    assert_eq!(
        normal.permission_action_id.as_deref(),
        Some("sentinel-edit")
    );
    assert!(normal.permission_cancelled);
    assert!(normal.session_cancel_sent);
    assert!(normal.prompt_cancelled);
    assert!(normal.terminal_accepted);
    assert!(normal.permission_action_metadata_verified);
    assert_eq!(normal.before_tree_sha256, normal.after_tree_sha256);
    assert_eq!(normal.result_record["effect_count"], 0);
    assert!(normal.process_stopped);
    assert!(normal.process_exited);
    assert!(normal.task_joined);
    assert!(normal.resource_released);
    assert!(normal.attempt_visible_before_version);
    assert!(normal.prompt_visible_before_send);
    assert_eq!(normal.before_tree_sha256, normal.after_tree_sha256);
    assert_eq!(normal.attempt_record["mode"], "fake-local-process-only");
    assert_eq!(
        normal.attempt_record["enable_command"],
        FROZEN_ENABLE_COMMAND
    );
    assert_eq!(normal.attempt_record["original_started"], false);
    assert!(normal.record_files_private);
    assert!(normal.protected_executable_removed);

    for mode in ["missing-action", "mismatched-action", "wrong-action-path"] {
        let invalid =
            run_fake_original_branch(mode, Duration::from_secs(3), Duration::from_millis(500));
        assert_eq!(invalid.status, "incomplete-action-correlation", "{mode}");
        assert_ne!(invalid.terminal, "permission-observed", "{mode}");
        assert_eq!(invalid.cleanup, "clean", "{mode}");
        assert!(!invalid.terminal_accepted, "{mode}");
        assert!(!invalid.permission_action_metadata_verified, "{mode}");
        assert!(invalid.process_stopped, "{mode}");
        assert!(invalid.process_exited, "{mode}");
        assert!(invalid.task_joined, "{mode}");
        assert!(invalid.resource_released, "{mode}");
        assert!(invalid.attempt_visible_before_version, "{mode}");
        assert!(invalid.prompt_visible_before_send, "{mode}");
        assert_eq!(
            invalid.before_tree_sha256, invalid.after_tree_sha256,
            "{mode}"
        );
        assert_eq!(invalid.result_record["effect_count"], 0, "{mode}");
        assert!(invalid.record_files_private, "{mode}");
        assert!(invalid.protected_executable_removed, "{mode}");
    }
}

#[test]
fn frozen_enable_entry_dispatches_payload_through_the_one_shot_runner() {
    let scratch = RealScratch::new();
    let mut plan = enabled_fake_plan(&scratch);
    let home = std::env::var("HOME").expect("test process has an approved home directory");
    let mut payload = serde_json::json!({
        "schema": ORIGINAL_ENTRY_PAYLOAD_SCHEMA,
        "reviewed_head": "0123456789abcdef0123456789abcdef01234567",
        "plan_sha256": "0".repeat(64),
        "enable_command": FROZEN_ENABLE_COMMAND,
        "execution_host_id": "assessment.dispatch.fake-host",
        "executable_ref": scratch.executable.to_string_lossy(),
        "environment_ref": APPROVED_ENVIRONMENT,
        "working_resource_ref": "assessment.dispatch.fake-working-resource",
        "home_directory": home,
    });
    let binding = parse_original_entry_binding(
        serde_json::to_vec(&payload)
            .expect("fake binding serializes")
            .as_slice(),
    )
    .expect("fake binding validates");
    plan["original_enable_gate"]["binding_sha256"] = serde_json::Value::String(
        original_entry_binding_sha256(&binding).expect("safe binding hashes"),
    );
    payload["plan_sha256"] = serde_json::Value::String(
        sha256_reader(
            serde_json::to_vec(&plan)
                .expect("fake enabled plan serializes")
                .as_slice(),
        )
        .expect("fake enabled plan hashes"),
    );
    let payload_path = scratch.executable.parent().unwrap().join("binding.json");
    write_private_payload(&payload_path, &payload);
    let arguments = vec![
        OsString::from("--payload"),
        payload_path.clone().into_os_string(),
    ];

    let resolved = std::cell::Cell::new(false);
    let result = dispatch_original_entry(&arguments, &plan, |binding| {
        resolved.set(true);
        let host = ExecutionHostId::new(binding.execution_host_id.clone())
            .map_err(|_| OriginalRunDenied::HostInputMismatch)?;
        let executable = ExecutableRef::new(binding.executable_ref.clone())
            .map_err(|_| OriginalRunDenied::HostInputMismatch)?;
        let environment = EnvironmentRef::new(binding.environment_ref.clone())
            .map_err(|_| OriginalRunDenied::HostInputMismatch)?;
        let working_resource = WorkingResourceRef::new(binding.working_resource_ref.clone())
            .map_err(|_| OriginalRunDenied::HostInputMismatch)?;
        let python = find_program("python3").expect("Effigy provides python3");
        let (builder, _target) = LocalProcessHost::builder(LocalProcessLimits::default())
            .approve_installed_executable_launch(
                executable.clone(),
                InterfaceVersionAxis::new(COPILOT_CLI_PACKAGE_AXIS).expect("static package axis"),
                LocalExecutableLaunch::interpreted_script(python, scratch.executable.clone()),
            );
        let local = builder
            .approve_environment(
                environment.clone(),
                [
                    (
                        OsString::from("HOME"),
                        OsString::from(&binding.home_directory),
                    ),
                    (
                        OsString::from("SWALLOWTAIL_FAKE_MODE"),
                        OsString::from("permission"),
                    ),
                    (
                        OsString::from("SWALLOWTAIL_FAKE_TRACE"),
                        scratch.trace.as_os_str().to_owned(),
                    ),
                    (
                        OsString::from("SWALLOWTAIL_FAKE_RECORDS"),
                        scratch.records.as_os_str().to_owned(),
                    ),
                ],
            )
            .approve_working_resource(working_resource.clone(), scratch.task.clone())
            .build_services(host.clone());
        Ok(OriginalEntryHost {
            local,
            input: original_preparation_input(host, executable, environment),
            working_resource,
            task_directory: scratch.task.clone(),
            trace: Some(scratch.trace.clone()),
        })
    });
    assert!(resolved.get());
    assert!(matches!(
        result,
        Err(OriginalRunDenied::ExecutableIdentityMismatch)
    ));
    assert_eq!(fs::read_dir(&scratch.records).unwrap().count(), 0);
    assert!(fs::read_to_string(&scratch.trace).is_err());
    let production = OriginalRunPolicy::production();
    assert_eq!(production.total, Duration::from_secs(60));
    assert_eq!(production.cleanup, Duration::from_secs(3));
    assert_eq!(
        production.expected_executable_sha256,
        crate::assessment::NATIVE_EXECUTABLE_SHA256
    );
    assert!(production.original_started);
    assert_eq!(
        production.execution_mode,
        "approved-local-process-host-original"
    );

    let mut drifted = payload;
    drifted["environment_ref"] =
        serde_json::Value::String("assessment.other-environment".to_owned());
    write_private_payload(&payload_path, &drifted);
    let resolved = std::cell::Cell::new(false);
    let result = dispatch_original_entry(&arguments, &plan, |_| {
        resolved.set(true);
        Err(OriginalRunDenied::HostInputMismatch)
    });
    assert!(matches!(
        result,
        Err(OriginalRunDenied::ContinuationMismatch)
    ));
    assert!(!resolved.get());
    assert_eq!(fs::read_dir(&scratch.records).unwrap().count(), 0);
}

#[test]
fn disabled_original_entry_rejects_before_host_resolution_or_record_effects() {
    let scratch = RealScratch::new();
    let plan_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/copilot-cli-acp-currentness-1.0.95/private-assessment-plan.json");
    let plan = read_json_object(&plan_path).expect("disabled plan is readable");
    let plan_sha256 = sha256_reader(
        serde_json::to_vec(&plan)
            .expect("disabled plan serializes")
            .as_slice(),
    )
    .expect("disabled plan hashes");
    let payload = serde_json::json!({
        "schema": ORIGINAL_ENTRY_PAYLOAD_SCHEMA,
        "reviewed_head": "0123456789abcdef0123456789abcdef01234567",
        "plan_sha256": plan_sha256,
        "enable_command": FROZEN_ENABLE_COMMAND,
        "execution_host_id": "assessment.disabled.fake-host",
        "executable_ref": scratch.executable.to_string_lossy(),
        "environment_ref": APPROVED_ENVIRONMENT,
        "working_resource_ref": "assessment.disabled.fake-working-resource",
        "home_directory": std::env::var("HOME").expect("HOME is set"),
    });
    let payload_path = scratch
        .executable
        .parent()
        .unwrap()
        .join("disabled-binding.json");
    write_private_payload(&payload_path, &payload);
    let arguments = vec![OsString::from("--payload"), payload_path.into_os_string()];
    let resolved = std::cell::Cell::new(false);
    let result = dispatch_original_entry(&arguments, &plan, |_| {
        resolved.set(true);
        Err(OriginalRunDenied::HostInputMismatch)
    });
    assert!(matches!(result, Err(OriginalRunDenied::Disabled)));
    assert!(!resolved.get());
    assert_eq!(fs::read_dir(&scratch.records).unwrap().count(), 0);
    assert!(fs::read_to_string(&scratch.trace).is_err());
}

#[test]
#[ignore = "only the planner-bound observe selector may invoke the original entry"]
fn invoke_reviewed_original_entry() {
    let evidence = invoke_reviewed_original_from_environment()
        .unwrap_or_else(|_| panic!("reviewed Copilot ACP original entry failed closed"));
    assert_eq!(evidence.cleanup, "clean");
    assert_eq!(evidence.status, "permission-cancelled");
    assert_eq!(evidence.result_record["effect_count"], 0);
}

fn write_private_payload(path: &Path, value: &serde_json::Value) {
    use std::os::unix::fs::OpenOptionsExt;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o600)
        .open(path)
        .expect("create mode-0600 test binding payload");
    file.write_all(&serde_json::to_vec(value).expect("payload serializes"))
        .expect("write test binding payload");
    file.sync_all().expect("sync test binding payload");
}

#[test]
fn real_local_prepared_path_failures_and_effect_are_measured() {
    let failed = run_fake_original_branch(
        "execution-failure",
        Duration::from_secs(3),
        Duration::from_millis(500),
    );
    assert_eq!(failed.status, "execution-failed");
    assert_eq!(failed.terminal, "runtime-failed");
    assert_eq!(failed.cleanup, "clean");
    assert_eq!(failed.before_tree_sha256, failed.after_tree_sha256);
    assert_eq!(failed.result_record["effect_count"], 0);
    assert!(failed.process_stopped);
    assert!(failed.process_exited);
    assert!(failed.task_joined);
    assert!(failed.resource_released);
    assert!(failed.attempt_visible_before_version);
    assert!(failed.prompt_visible_before_send);
    assert!(failed.protected_executable_removed);

    let effect = run_fake_original_branch(
        "provider-effect",
        Duration::from_secs(3),
        Duration::from_millis(500),
    );
    assert_eq!(effect.status, "effect-detected");
    assert_ne!(effect.before_tree_sha256, effect.after_tree_sha256);
    assert!(!effect.terminal_accepted);
    assert!(effect.process_stopped);
    assert!(effect.process_exited);
    assert!(effect.task_joined);
    assert!(effect.resource_released);
    assert!(effect.attempt_visible_before_version);
    assert!(effect.prompt_visible_before_send);
    assert!(effect.protected_executable_removed);
}

#[test]
fn real_local_prepared_open_failure_stops_and_joins_child_before_return() {
    let failed = run_fake_original_branch(
        "session-new-failure",
        Duration::from_secs(2),
        Duration::from_millis(500),
    );
    assert_eq!(failed.status, "session-open-failed");
    assert_eq!(failed.cleanup, "driver-joined");
    assert!(failed.elapsed < Duration::from_secs(2));
    assert!(failed.prompt_record.is_none());
    assert_eq!(failed.before_tree_sha256, failed.after_tree_sha256);
    assert_eq!(failed.result_record["effect_count"], 0);
    assert!(failed.process_stopped);
    assert!(failed.process_exited);
    assert!(failed.task_joined);
    assert!(failed.resource_released);
    assert!(failed.attempt_visible_before_version);
    assert!(!failed.prompt_visible_before_send);
    assert!(failed.protected_executable_removed);

    let timed_out = run_fake_original_branch(
        "session-new-hang",
        Duration::from_secs(6),
        Duration::from_secs(2),
    );
    assert_eq!(timed_out.status, "session-open-timeout");
    assert_eq!(timed_out.cleanup, "driver-joined");
    assert!(timed_out.elapsed < Duration::from_secs(6));
    assert!(timed_out.prompt_record.is_none());
    assert_eq!(timed_out.before_tree_sha256, timed_out.after_tree_sha256);
    assert_eq!(timed_out.result_record["effect_count"], 0);
    assert!(timed_out.process_stopped);
    assert!(timed_out.process_exited);
    assert!(timed_out.task_joined);
    assert!(timed_out.resource_released);
    assert!(timed_out.attempt_visible_before_version);
    assert!(!timed_out.prompt_visible_before_send);
    assert!(timed_out.protected_executable_removed);
}

#[test]
fn existing_invocation_record_fails_closed_before_any_prepared_process() {
    let scratch = RealScratch::new();
    let attempt_path = scratch.records.join("1.0.95-attempt-4.json");
    let existing = serde_json::json!({
        "schema": "copilot-cli-private-assessment-consumed.v1",
        "phase": "historical-consumed-attempt",
        "invocation_number": 4,
    });
    write_exclusive_fsynced(&attempt_path, &existing).expect("create prior consumed guard");
    let before_tree = tree_sha256(&scratch.task);

    let result = run_fake_original_branch_with_scratch(
        &scratch,
        "permission",
        Duration::from_secs(3),
        Duration::from_millis(500),
    );

    assert!(matches!(result, Err(OriginalRunDenied::RecordRootUnsafe)));
    assert_eq!(tree_sha256(&scratch.task), before_tree);
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(
            &fs::read(&attempt_path).expect("consumed record remains readable"),
        )
        .expect("consumed record parses"),
        existing,
        "a prior one-shot record is immutable"
    );
    assert_eq!(
        fs::read_dir(&scratch.records)
            .expect("record directory exists")
            .count(),
        1
    );
    assert!(fs::read_to_string(&scratch.trace).is_err());
}

#[test]
fn enabled_runner_requires_the_exact_planner_head_plan_and_command_binding() {
    let scratch = RealScratch::new();
    let plan = enabled_fake_plan(&scratch);
    let continuation = PlannerBoundContinuation {
        reviewed_head: plan["original_enable_gate"]["reviewed_head"]
            .as_str()
            .expect("fake reviewed head exists")
            .to_owned(),
        plan_sha256: sha256_reader(
            serde_json::to_vec(&plan)
                .expect("fake plan serializes")
                .as_slice(),
        )
        .expect("fake plan hashes"),
        binding_sha256: plan["original_enable_gate"]["binding_sha256"]
            .as_str()
            .expect("fake host binding digest exists")
            .to_owned(),
        enable_command: plan["original_enable_gate"]["enable_command"]
            .as_str()
            .expect("fake enable command exists")
            .to_owned(),
    };
    assert!(authorize_original_run(&plan, Some(&continuation)).is_ok());

    let mut wrong_head = continuation.clone();
    wrong_head.reviewed_head = "abcdefabcdefabcdefabcdefabcdefabcdefabcd".to_owned();
    assert!(matches!(
        authorize_original_run(&plan, Some(&wrong_head)),
        Err(OriginalRunDenied::ContinuationMismatch)
    ));

    let mut wrong_plan = continuation.clone();
    wrong_plan.plan_sha256 = "0".repeat(64);
    assert!(matches!(
        authorize_original_run(&plan, Some(&wrong_plan)),
        Err(OriginalRunDenied::ContinuationMismatch)
    ));

    let mut wrong_command = continuation;
    wrong_command.enable_command.push_str(" --extra");
    assert!(matches!(
        authorize_original_run(&plan, Some(&wrong_command)),
        Err(OriginalRunDenied::ContinuationMismatch)
    ));

    let mut records_disabled = plan.clone();
    records_disabled["records"]["original_execution_enabled"] = serde_json::Value::Bool(false);
    let records_disabled_continuation = PlannerBoundContinuation {
        reviewed_head: plan["original_enable_gate"]["reviewed_head"]
            .as_str()
            .expect("fake reviewed head exists")
            .to_owned(),
        plan_sha256: sha256_reader(
            serde_json::to_vec(&records_disabled)
                .expect("record-gate plan serializes")
                .as_slice(),
        )
        .expect("record-gate plan hashes"),
        binding_sha256: plan["original_enable_gate"]["binding_sha256"]
            .as_str()
            .expect("fake host binding digest exists")
            .to_owned(),
        enable_command: plan["original_enable_gate"]["enable_command"]
            .as_str()
            .expect("fake enable command exists")
            .to_owned(),
    };
    assert!(matches!(
        authorize_original_run(&records_disabled, Some(&records_disabled_continuation)),
        Err(OriginalRunDenied::ContinuationMismatch)
    ));
    assert_eq!(
        fs::read_dir(&scratch.records)
            .expect("record directory exists")
            .count(),
        0
    );
    assert!(fs::read_to_string(&scratch.trace).is_err());
}

fn cleanup_name(outcome: &CleanupOutcome) -> &'static str {
    match outcome {
        CleanupOutcome::Clean => "clean",
        CleanupOutcome::Degraded(_) => "degraded",
        CleanupOutcome::NotApplicable => "not-applicable",
        CleanupOutcome::Failed(_) => "failed",
    }
}

fn find_program(name: &str) -> Option<PathBuf> {
    std::env::var_os("PATH")?
        .to_string_lossy()
        .split(':')
        .map(|directory| Path::new(directory).join(name))
        .find(|candidate| candidate.is_file())
}

fn tree_sha256(root: &Path) -> String {
    let mut entries = BTreeMap::new();
    collect_tree(root, root, &mut entries);
    let bytes = serde_json::to_vec(&entries).expect("task tree inventory serializes");
    sha256_reader(bytes.as_slice()).expect("task tree inventory hashes")
}

fn collect_tree(root: &Path, current: &Path, entries: &mut BTreeMap<String, String>) {
    for entry in fs::read_dir(current).expect("task-owned directory remains readable") {
        let entry = entry.expect("task-owned directory entry is readable");
        let path = entry.path();
        let metadata = fs::symlink_metadata(&path).expect("task-owned entry metadata is readable");
        assert!(
            !metadata.file_type().is_symlink(),
            "task scratch contains no links"
        );
        if metadata.is_dir() {
            collect_tree(root, &path, entries);
        } else {
            let relative = path
                .strip_prefix(root)
                .expect("entry is within task scratch");
            let name = relative.to_string_lossy().replace('\\', "/");
            entries.insert(name, sha256_file(&path));
        }
    }
}

fn write_exclusive_fsynced(path: &Path, value: &serde_json::Value) -> std::io::Result<()> {
    use std::io::Write;
    let bytes = serde_json::to_vec(value).expect("secret-free record serializes");
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .custom_flags(libc::O_NOFOLLOW)
        .open(path)?;
    file.write_all(&bytes)?;
    file.sync_all()?;
    drop(file);
    let parent = path.parent().expect("record path has a parent");
    std::fs::OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_DIRECTORY | libc::O_NOFOLLOW)
        .open(parent)?
        .sync_all()
}
