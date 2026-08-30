use futures_executor::block_on;
use futures_util::StreamExt;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use swallowtail_adapter_claude_agent::{
    CLAUDE_CODE_HEADLESS_AXIS, CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION,
    ClaudeCodeModelSelection, ClaudeCodePreparationInput, ClaudeCodePreparationProbe,
    ClaudeCodeRunProfileInput, prepare_claude_code_headless,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, ConfiguredInstanceId, CredentialMechanism,
    CredentialState, EndpointAudience, EndpointAuthorization, EntitlementMetering,
    EntitlementState, ExecutionHostId, InstanceRevision, InterfaceVersionAxis, ModelId,
    ModelRouteId, ModelRouteRevision, RuntimeReadiness, SupportAuthority, WatcherOwningTurn,
};
use swallowtail_host_local::{
    LocalExecutableLaunch, LocalHostServices, LocalProcessHost, LocalProcessLimits,
};
use swallowtail_runtime::{
    CleanupOutcome, DiscoveryCancellation, EnvironmentRef, ExecutableRef, OperationContent,
    PreparedAccessEvidence, ProcessRequest, RequestId, RuntimeEvent, ScopeId, TerminalStatus,
    WorkingResourceRef,
};

const WATCHER_VERSION: &str = "2.1.251";
const WATCHER_NATIVE_SHA256: &str =
    "625869b01e0050f260b2980fac248fd9cef9e462612bded4ec9d3d49ff8969a5";
const WATCHER_MODEL: &str = "claude-haiku-4-5";
const WATCHER_OPERATION: &str = "sleep-operation";
const LIVE_REQUEST: &str = "live-claude-code-watcher";

#[test]
#[ignore = "requires SWALLOWTAIL_LIVE_CLAUDE_CODE_WATCHER=1, exact Claude Code 2.1.251, and local Max/OAuth state"]
fn configured_claude_code_blocks_early_completion_then_joins_one_watcher() {
    assert_eq!(
        std::env::var("SWALLOWTAIL_LIVE_CLAUDE_CODE_WATCHER").as_deref(),
        Ok("1"),
        "authenticated watcher probe requires its explicit gate"
    );
    assert!(
        std::env::var_os("ANTHROPIC_API_KEY").is_none(),
        "watcher live proof must use local subscription auth"
    );
    assert_eq!(
        CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION,
        WATCHER_VERSION
    );
    let source_before = git_status();
    let binary = installed_claude();
    assert_eq!(
        sha256_hex(&binary),
        WATCHER_NATIVE_SHA256,
        "native digest drifted"
    );
    assert!(
        version_output(&binary).contains(WATCHER_VERSION),
        "installed Claude Code is not exact 2.1.251"
    );
    assert!(
        help_output(&binary).contains("--model"),
        "exact model selection is unavailable without a provider request"
    );

    let workspace = unique_workspace();
    let (local, target, environment, working_resource, execution_host_id) =
        live_host(binary, workspace.clone());
    let access_id = AccessProfileId::new("live.claude-code.local-subscription").expect("access id");
    let access = AccessProfile::new(
        access_id.clone(),
        CredentialMechanism::LocalUnauthenticated,
        EntitlementMetering::SubscriptionAllowance,
        EndpointAudience::new("anthropic-claude-code").expect("audience"),
        SupportAuthority::ProviderSupported,
    );
    let prepared = block_on(prepare_claude_code_headless(
        ClaudeCodePreparationInput::new(
            ConfiguredInstanceId::new("live.claude-code.watchers").expect("instance id"),
            InstanceRevision::new(WATCHER_VERSION).expect("revision"),
            execution_host_id,
            target,
            environment,
            access,
            PreparedAccessEvidence::caller_asserted(AccessStatus::new(
                access_id,
                CredentialState::NotRequired,
                EntitlementState::Available,
                EndpointAuthorization::Allowed,
                RuntimeReadiness::Ready,
                SupportAuthority::ProviderSupported,
            )),
        ),
        ClaudeCodePreparationProbe::new(
            RequestId::new("live-claude-code-watcher-prepare").expect("request id"),
            ScopeId::new("live-claude-code-watcher-prepare").expect("scope id"),
            local.deadline_after(Duration::from_secs(5)),
            DiscoveryCancellation::new(),
        ),
        local.services().clone(),
    ))
    .expect("installed Claude Code watcher route prepares");
    assert_eq!(
        prepared.observation().version().version().as_str(),
        WATCHER_VERSION
    );

    let run = prepared
        .prepare_run(
            ClaudeCodeRunProfileInput::new(
                RequestId::new(LIVE_REQUEST).expect("request id"),
                ClaudeCodeModelSelection::new(
                    ModelRouteId::new("live.claude-code.watchers").expect("route id"),
                    ModelRouteRevision::new(WATCHER_VERSION).expect("route revision"),
                    ModelId::new(WATCHER_MODEL).expect("model id"),
                ),
                OperationContent::new(
                    "Use the Swallowtail watcher MCP tools to start one watcher with operation_data sleep-operation. After it starts, immediately try to finish this turn and reply WATCHER_EARLY_DONE. If you are told watchers remain active or unjoined, wait or stop that watcher, then reply with exactly WATCHER_LIVE_OK and nothing else.",
                )
                .expect("prompt"),
                working_resource,
                local.deadline_after(Duration::from_secs(90)),
            )
            .with_watchers(),
        )
        .expect("watcher run prepares");

    let seen_active = Arc::new(AtomicBool::new(false));
    let watcher = local.services().watcher().expect("watcher service").clone();
    let owning = WatcherOwningTurn::new(format!("claude-code-headless:{LIVE_REQUEST}"))
        .expect("owning turn");
    let seen = Arc::clone(&seen_active);
    let poller = std::thread::spawn(move || {
        let started = std::time::Instant::now();
        while started.elapsed() < Duration::from_secs(80) {
            if let Ok(snapshots) = block_on(watcher.list(owning.clone()))
                && !snapshots.is_empty()
            {
                seen.store(true, Ordering::SeqCst);
                return;
            }
            std::thread::sleep(Duration::from_millis(50));
        }
    });

    let mut handle = block_on(run.start_run(local.services().clone()))
        .expect("authenticated watcher run starts");
    let mut events = handle.take_events().expect("event stream");
    let terminal = handle.take_terminal_outcome().expect("terminal outcome");
    let (observed_events, outcome) = block_on(async {
        let mut observed = Vec::new();
        while let Some(event) = events.next().await {
            observed.push(event.expect("live watcher event remains valid"));
        }
        (observed, terminal.await)
    });
    let _ = poller.join();
    secret_free_events(&observed_events);
    assert!(
        seen_active.load(Ordering::SeqCst),
        "live watcher never became visible on the host registry"
    );
    match outcome.status() {
        TerminalStatus::Completed => {}
        TerminalStatus::RuntimeFailed(diagnostic)
            if diagnostic.code()
                == "swallowtail.claude_code.headless.watcher_completion_blocked" =>
        {
            panic!(
                "live Stop continuation was missing; provider success was only locally rejected"
            );
        }
        _ => panic!("live watcher turn did not complete cleanly"),
    }
    assert_eq!(
        outcome
            .output()
            .map(OperationContent::as_str)
            .map(str::trim),
        Some("WATCHER_LIVE_OK")
    );
    assert_eq!(outcome.cleanup(), &CleanupOutcome::Clean);
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    assert_eq!(
        git_status(),
        source_before,
        "provider created a source artifact"
    );
    let _ = std::fs::remove_dir_all(workspace);
}

fn live_host(
    binary: PathBuf,
    workspace: PathBuf,
) -> (
    LocalHostServices,
    swallowtail_runtime::InstalledExecutableTarget,
    EnvironmentRef,
    WorkingResourceRef,
    ExecutionHostId,
) {
    let environment =
        EnvironmentRef::new("live.claude-code.local-subscription").expect("environment");
    let working_resource =
        WorkingResourceRef::new("live.claude-code.watcher-workspace").expect("resource");
    let execution_host_id = ExecutionHostId::new("live.claude-code.local-host").expect("host id");
    let executable = ExecutableRef::new("live.claude-code.installed").expect("executable ref");
    let watcher_executable =
        ExecutableRef::new("live.claude-code.watcher.sleep").expect("watcher executable");
    let operation =
        swallowtail_core::WatcherOperationData::new(WATCHER_OPERATION).expect("operation is valid");
    let (builder, target) = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_installed_executable_launch(
            executable,
            InterfaceVersionAxis::new(CLAUDE_CODE_HEADLESS_AXIS).expect("release axis"),
            LocalExecutableLaunch::new(binary),
        );
    let home = std::env::var_os("HOME").expect("local Claude auth requires HOME");
    let user = std::env::var_os("USER").expect("local Claude auth requires USER");
    let logname = std::env::var_os("LOGNAME").expect("local Claude auth requires LOGNAME");
    let local = builder
        .approve_executable(watcher_executable.clone(), "/bin/sleep")
        .approve_watcher_operation(
            operation,
            ProcessRequest::new(watcher_executable).with_arguments(["25".to_owned()]),
        )
        .approve_environment(
            environment.clone(),
            [
                (OsString::from("HOME"), home),
                (OsString::from("USER"), user),
                (OsString::from("LOGNAME"), logname),
            ],
        )
        .approve_working_resource(working_resource.clone(), workspace)
        .build_services(execution_host_id.clone());
    (
        local,
        target,
        environment,
        working_resource,
        execution_host_id,
    )
}

fn installed_claude() -> PathBuf {
    let selected = installed_path("claude").expect("Claude Code is installed on PATH");
    std::fs::canonicalize(selected).expect("Claude Code resolves exactly")
}

fn installed_path(command: &str) -> Option<PathBuf> {
    std::env::var_os("PATH")
        .into_iter()
        .flat_map(|path| std::env::split_paths(&path).collect::<Vec<_>>())
        .map(|directory| directory.join(command))
        .find(|candidate| candidate.is_file())
}

fn sha256_hex(path: &Path) -> String {
    let output = std::process::Command::new("shasum")
        .args(["-a", "256"])
        .arg(path)
        .output()
        .expect("native digest can be hashed without a provider request");
    assert!(
        output.status.success(),
        "native digest could not be hashed without a provider request"
    );
    String::from_utf8_lossy(&output.stdout)
        .split_whitespace()
        .next()
        .expect("digest")
        .to_owned()
}

fn version_output(binary: &Path) -> String {
    let output = std::process::Command::new(binary)
        .arg("--version")
        .output()
        .expect("Claude Code version can be read without a provider request");
    assert!(
        output.status.success(),
        "Claude Code version could not be read without a provider request"
    );
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

fn help_output(binary: &Path) -> String {
    let output = std::process::Command::new(binary)
        .arg("--help")
        .output()
        .expect("Claude Code help can be read without a provider request");
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

fn unique_workspace() -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "swallowtail-claude-watcher-live-{}",
        std::process::id()
    ));
    std::fs::create_dir_all(&path).expect("watcher workspace is created");
    path
}

fn secret_free_events(events: &[RuntimeEvent]) {
    for event in events {
        let debug = format!("{event:?}");
        assert!(!debug.contains("Bearer"), "event leaked a bearer");
        assert!(!debug.contains("127.0.0.1"), "event leaked an endpoint");
        assert!(
            !debug.contains("Authorization"),
            "event leaked authorization"
        );
        assert!(!debug.contains("/bin/sleep"), "event leaked a command");
    }
}

fn git_status() -> Vec<u8> {
    std::process::Command::new("git")
        .args(["status", "--porcelain=v1", "--untracked-files=all"])
        .output()
        .expect("git status runs")
        .stdout
}
