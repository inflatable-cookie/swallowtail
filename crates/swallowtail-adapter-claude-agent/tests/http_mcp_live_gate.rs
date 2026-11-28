#[path = "http_mcp_live/mod.rs"]
mod http_mcp_live;

use futures_executor::block_on;
use futures_util::StreamExt;
use http_mcp_live::{
    DisposableHttpMcpServer, HTTP_MCP_LIVE_TOOL, HTTP_MCP_LIVE_TOOL_RESULT, HttpMcpLiveRecord,
    HttpMcpLiveStop, persist_and_print_record,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::ffi::{OsStr, OsString};
use std::io;
use std::path::{Path, PathBuf};
use std::time::Duration;
use swallowtail_adapter_claude_agent::{
    CLAUDE_AGENT_ACP_AXIS, CLAUDE_AGENT_ACP_MCP_SERVER_NAME, ClaudeAgentAcpRemoteMcpPlacement,
    ClaudeAgentModelSelection, ClaudeAgentPreparationInput, ClaudeAgentPreparationProbe,
    ClaudeAgentSessionProfileInput, claude_agent_acp_guarded_v1_frozen,
    claude_agent_acp_subscription_access_profile, prepare_claude_agent,
};
use swallowtail_core::{
    AccessProfileId, AccessStatus, ConfiguredInstanceId, CredentialState, EndpointAuthorization,
    EntitlementState, ExecutionHostId, InstanceRevision, InterfaceVersionAxis, ModelId,
    ModelRouteId, ModelRouteRevision, RuntimeReadiness, SupportAuthority,
};
use swallowtail_host_local::{
    LocalCheckedNodeLaunch, LocalExecutableLaunch, LocalHostServices, LocalProcessHost,
    LocalProcessLimits, LocalRuntimeTrust, LocalTrustedRuntime, capture_frozen_guarded_recipe,
};
use swallowtail_runtime::{
    CallbackPayload, CallbackRequestKind, CallbackResponse, CallbackResult, CheckedProcessRef,
    CleanupOutcome, DiscoveryCancellation, EnvironmentRef, ExecutableRef,
    InstalledExecutableTarget, OperationContent, PreparedAccessEvidence, RequestId, RuntimeTurnId,
    ScopeId, SessionOptions, TerminalStatus, TurnRequest, WorkingResourceRef,
};

const LIVE_GATE: &str = "SWALLOWTAIL_LIVE_CLAUDE_AGENT_ACP_HTTP_MCP";
const LIVE_MODEL: &str = "claude-sonnet-4-6";
/// Research 361 accepted honouring on exact `0.79.0`, Research 364 accepted
/// exact `0.81.2`, and Research 492 accepted exact guarded `0.106.0`. This gate
/// tests the separately authorized guarded point, exact `0.113.0`.
const HTTP_MCP_HONOURING_VERSION: &str = "0.113.0";

const PACKAGE_PREFIXES: [(&str, &str); 5] = [
    ("acp", "@agentclientprotocol/claude-agent-acp"),
    ("acpsdk", "@agentclientprotocol/sdk"),
    ("agentsdk", "@anthropic-ai/claude-agent-sdk"),
    ("diff", "diff"),
    ("zod", "zod"),
];

#[test]
#[ignore = "requires SWALLOWTAIL_LIVE_CLAUDE_AGENT_ACP_HTTP_MCP=1, repo-local claude-agent-acp 0.113.0, and local Claude subscription auth"]
fn one_authorized_claude_agent_acp_http_mcp_live_attempt() {
    assert_eq!(
        std::env::var(LIVE_GATE).as_deref(),
        Ok("1"),
        "Claude Agent ACP HTTP MCP live gate requires its explicit env gate"
    );
    let record = run_one_attempt();
    let record_path = live_record_path();
    persist_and_print_record(&record, &record_path, &mut std::io::stdout().lock())
        .expect("live gate must persist and print its typed record");
    assert!(record_path.is_file(), "live gate record file must exist");
    assert!(
        record.accepted() || record.stop().is_some(),
        "one attempt must accept or name a typed stop: {record:?}"
    );
}

fn live_record_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("target/claude-agent-acp-http-mcp-live")
        .join(format!("attempt-{}.json", std::process::id()))
}

struct TempDirGuard(PathBuf);

impl Drop for TempDirGuard {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

struct LiveGuardedHost {
    local: LocalHostServices,
    target: InstalledExecutableTarget,
    environment: EnvironmentRef,
    working_resource: WorkingResourceRef,
    execution_host_id: ExecutionHostId,
    checked_process: CheckedProcessRef,
    _stage_guard: TempDirGuard,
}

fn run_one_attempt() -> HttpMcpLiveRecord {
    let Some((node_modules, node)) = installed_sidecar_root() else {
        eprintln!("http_mcp_live_gate: repo-local node_modules or node executable missing");
        return HttpMcpLiveRecord::pre_attempt_stop(HttpMcpLiveStop::HostVersion, None);
    };
    let host = match live_guarded_host(&node_modules, &node) {
        Ok(host) => host,
        Err(error) => {
            eprintln!("http_mcp_live_gate: guarded host setup failed: {error}");
            return HttpMcpLiveRecord::pre_attempt_stop(HttpMcpLiveStop::HostVersion, None);
        }
    };
    let LiveGuardedHost {
        local,
        target,
        environment,
        working_resource,
        execution_host_id,
        checked_process,
        _stage_guard,
    } = host;
    let access_id = AccessProfileId::new("live.claude-agent.acp.subscription").expect("access id");
    let prepared = match block_on(prepare_claude_agent(
        ClaudeAgentPreparationInput::new(
            ConfiguredInstanceId::new("live.claude-agent.acp.instance").expect("instance id"),
            InstanceRevision::new(HTTP_MCP_HONOURING_VERSION).expect("instance revision"),
            execution_host_id,
            target,
            environment,
            claude_agent_acp_subscription_access_profile(access_id.clone()),
            PreparedAccessEvidence::caller_asserted(AccessStatus::new(
                access_id,
                CredentialState::NotRequired,
                EntitlementState::Available,
                EndpointAuthorization::Allowed,
                RuntimeReadiness::Ready,
                SupportAuthority::IntegrationMaintainerSupported,
            )),
        )
        .with_checked_process(checked_process),
        ClaudeAgentPreparationProbe::new(
            RequestId::new("live.claude-agent.acp.http-mcp.prepare").expect("request id"),
            ScopeId::new("live.claude-agent.acp.http-mcp.prepare").expect("scope id"),
            local.deadline_after(Duration::from_secs(30)),
            DiscoveryCancellation::new(),
        ),
        local.services().clone(),
    )) {
        Ok(prepared) => prepared,
        Err(error) => {
            eprintln!(
                "http_mcp_live_gate: prepare_claude_agent failed: {}",
                error.diagnostic().safe().code()
            );
            let _ = local.shutdown_task_reapers();
            return HttpMcpLiveRecord::pre_attempt_stop(HttpMcpLiveStop::HostVersion, None);
        }
    };
    if prepared.observation().version().version().as_str() != HTTP_MCP_HONOURING_VERSION {
        let observed = prepared
            .observation()
            .version()
            .version()
            .as_str()
            .to_owned();
        let _ = local.shutdown_task_reapers();
        return HttpMcpLiveRecord::pre_attempt_stop(HttpMcpLiveStop::HostVersion, Some(observed));
    }

    let server = DisposableHttpMcpServer::start();
    let placement = ClaudeAgentAcpRemoteMcpPlacement::new(
        CLAUDE_AGENT_ACP_MCP_SERVER_NAME,
        server.endpoint(),
        vec![("Authorization".to_owned(), server.authorization_header())],
    );
    let session = match prepared.prepare_session(
        ClaudeAgentSessionProfileInput::new(
            RequestId::new("live.claude-agent.acp.http-mcp.session").expect("request id"),
            ClaudeAgentModelSelection::new(
                ModelRouteId::new("live.claude-agent.acp.http-mcp.route").expect("route id"),
                ModelRouteRevision::new("1").expect("route revision"),
                ModelId::new(LIVE_MODEL).expect("model id"),
            ),
            working_resource,
            SessionOptions::default(),
        )
        .with_consumer_mediated_permissions()
        .with_http_mcp_placement(placement),
    ) {
        Ok(session) => session,
        Err(_) => {
            let _ = local.shutdown_task_reapers();
            return HttpMcpLiveRecord::pre_attempt_stop(
                HttpMcpLiveStop::NoUsableModel,
                Some(LIVE_MODEL.to_owned()),
            );
        }
    };

    let mut handle = match block_on(session.open_session(local.services().clone())) {
        Ok(handle) => handle,
        Err(error) => {
            let stop = if error.diagnostic().code()
                == "swallowtail.claude_agent.acp.terminal_auth_rejected"
            {
                HttpMcpLiveStop::HostAuthRequired
            } else if server.transcript().connected() {
                HttpMcpLiveStop::ToolNotCalled
            } else {
                HttpMcpLiveStop::McpNotConnected
            };
            // No session handle exists, so no cleanup ran and none is reported.
            let mut record = HttpMcpLiveRecord::from_attempt(
                true,
                &server.transcript(),
                &TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                CleanupOutcome::NotApplicable,
                Some(LIVE_MODEL.to_owned()),
            );
            record.force_stop(stop);
            let _ = local.shutdown_task_reapers();
            return record;
        }
    };

    let mut turn = match block_on(handle.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("live.claude-agent.acp.http-mcp.turn").expect("turn id"),
            OperationContent::new(format!(
                "Call the MCP tool named {HTTP_MCP_LIVE_TOOL}. Do not finish the turn until that tool call has returned. Then reply with exactly {HTTP_MCP_LIVE_TOOL_RESULT}."
            ))
            .expect("prompt"),
        ),
        local.services().clone(),
    )) {
        Ok(turn) => turn,
        Err(error) => {
            // Keep the cleanup diagnostic the close actually produced: a
            // deadline or process stage observed here must not be replaced by
            // the turn error's diagnostic.
            let cleanup = block_on(handle.close(
                swallowtail_runtime::SessionCleanupRequest::new(
                    local.deadline_after(Duration::from_secs(30)),
                ),
                local.services().clone(),
            ));
            let _ = local.shutdown_task_reapers();
            return HttpMcpLiveRecord::from_attempt(
                true,
                &server.transcript(),
                &TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                cleanup,
                Some(LIVE_MODEL.to_owned()),
            );
        }
    };

    if let Some(mut callbacks) = turn.take_callbacks()
        && let Some(mut requests) = callbacks.take_requests()
    {
        let responder = callbacks.responder();
        std::thread::spawn(move || {
            block_on(async move {
                while let Some(request) = requests.next().await {
                    let Ok(callback) = request else {
                        continue;
                    };
                    let Some(turn_id) = callback.turn_id() else {
                        continue;
                    };
                    let option_id = select_allow_once_option_id(callback.kind())
                        .unwrap_or_else(|| "allow-once".to_owned());
                    let payload = serde_json::to_vec(&json!({ "optionId": option_id }))
                        .expect("permission response JSON");
                    let _ = responder
                        .respond(CallbackResponse::new(
                            callback.callback_id().clone(),
                            turn_id.clone(),
                            CallbackResult::Success(
                                CallbackPayload::new(payload, 256).expect("allow-once is bounded"),
                            ),
                        ))
                        .await;
                }
            });
        });
    }

    let mut events = turn.take_events().expect("event stream");
    let terminal = turn.take_terminal_outcome().expect("terminal outcome");
    let outcome = block_on(async {
        while let Some(event) = events.next().await {
            let _ = event;
        }
        terminal.await
    });
    let _ = block_on(turn.close());
    let cleanup = block_on(handle.close(
        swallowtail_runtime::SessionCleanupRequest::new(
            local.deadline_after(Duration::from_secs(30)),
        ),
        local.services().clone(),
    ));
    let _ = local.shutdown_task_reapers();
    HttpMcpLiveRecord::from_attempt(
        true,
        &server.transcript(),
        outcome.status(),
        cleanup,
        Some(LIVE_MODEL.to_owned()),
    )
}

fn select_allow_once_option_id(kind: &CallbackRequestKind) -> Option<String> {
    let CallbackRequestKind::Extension(extension) = kind else {
        return None;
    };
    let offered: Value = serde_json::from_slice(extension.payload()).ok()?;
    offered["options"]
        .as_array()?
        .iter()
        .find(|option| option["kind"] == "allow_once")
        .and_then(|option| option["optionId"].as_str())
        .map(str::to_owned)
}

fn current_guarded_platform() -> Option<&'static str> {
    if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
        Some("darwin-arm64")
    } else if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
        Some("linux-x64")
    } else {
        None
    }
}

fn resolve_manifest_source(node_modules: &Path, relative: &str) -> io::Result<PathBuf> {
    let (prefix, tail) = relative.split_once('/').ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("manifest path missing package prefix: {relative}"),
        )
    })?;
    let package = PACKAGE_PREFIXES
        .iter()
        .find_map(|(name, directory)| (*name == prefix).then_some(*directory))
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("unknown guarded manifest prefix: {prefix}"),
            )
        })?;
    Ok(node_modules.join(package).join(tail))
}

fn manifest_entries(manifest: &[u8], platform: &str) -> io::Result<(Vec<String>, String)> {
    let value: Value = serde_json::from_slice(manifest)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    let files = value["files"]
        .as_object()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "manifest missing files"))?
        .keys()
        .cloned()
        .collect();
    let native_specifier = value["native"][platform]["specifier"]
        .as_str()
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "manifest missing platform native specifier",
            )
        })?
        .to_owned();
    Ok((files, native_specifier))
}

#[cfg(unix)]
fn live_guarded_host(node_modules: &Path, node_path: &Path) -> io::Result<LiveGuardedHost> {
    use std::os::unix::fs::PermissionsExt;

    let platform = current_guarded_platform().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::Unsupported,
            "guarded Claude Agent ACP is qualified only on darwin-arm64 and linux-x64",
        )
    })?;
    let frozen = claude_agent_acp_guarded_v1_frozen(HTTP_MCP_HONOURING_VERSION, platform)
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                "frozen guarded manifest unavailable for version/platform",
            )
        })?;
    let (files, native_specifier) = manifest_entries(frozen.manifest(), platform)?;

    let stage_root = create_stage_root()?;
    let stage_guard = TempDirGuard(stage_root.clone());
    let sources_root = stage_root.join("sources");
    let workspace = stage_root.join("workspace");
    let native_dir = stage_root.join("native");
    for directory in [&sources_root, &workspace, &native_dir] {
        std::fs::create_dir(directory)?;
        std::fs::set_permissions(directory, std::fs::Permissions::from_mode(0o700))?;
    }

    for relative in &files {
        let source = resolve_manifest_source(node_modules, relative)?;
        let destination = sources_root.join(relative);
        if let Some(parent) = destination.parent() {
            create_private_dirs(&sources_root, parent)?;
        }
        std::fs::copy(&source, &destination)?;
        std::fs::set_permissions(&destination, std::fs::Permissions::from_mode(0o600))?;
    }

    let native_source = node_modules.join(&native_specifier);
    let native_staged = native_dir.join("claude");
    std::fs::copy(&native_source, &native_staged)?;
    std::fs::set_permissions(&native_staged, std::fs::Permissions::from_mode(0o700))?;
    let native_staged = native_staged.canonicalize()?;

    let node_runtime = capture_trusted_runtime(node_path)?;
    let native_runtime = capture_trusted_runtime(&native_staged)?;
    let ps_path = if platform == "darwin-arm64" {
        Path::new("/bin/ps")
    } else {
        Path::new("/usr/bin/ps")
    };
    let ps_runtime = capture_trusted_runtime(ps_path)?;

    let recipe = capture_frozen_guarded_recipe(
        &sources_root.canonicalize()?,
        frozen,
        &node_runtime,
        &native_runtime,
        Some(&ps_runtime),
    )?;
    let fingerprint = *recipe.fingerprint().digest();

    let environment = EnvironmentRef::new("live.claude-agent.acp.isolated").expect("environment");
    let working_resource =
        WorkingResourceRef::new("live.claude-agent.acp.workspace").expect("resource");
    let execution_host_id =
        ExecutionHostId::new("live.claude-agent.acp.local-host").expect("host id");
    let executable =
        ExecutableRef::new("live.claude-agent.acp.installed").expect("executable ref is valid");
    let checked_process =
        CheckedProcessRef::new("live.claude-agent.acp.checked").expect("checked ref is valid");
    let target = InstalledExecutableTarget::new(
        executable.clone(),
        InterfaceVersionAxis::new(CLAUDE_AGENT_ACP_AXIS).expect("axis"),
    );

    let home = std::env::var_os("HOME").ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, "local Claude auth requires HOME")
    })?;
    let user = std::env::var_os("USER").unwrap_or_else(|| OsString::from("swallowtail"));
    let logname = std::env::var_os("LOGNAME").unwrap_or_else(|| user.clone());
    let path = std::env::var_os("PATH").unwrap_or_default();

    let checked_launch = LocalCheckedNodeLaunch::new(
        executable.clone(),
        node_runtime,
        recipe.clone(),
        fingerprint,
        vec![vec![], vec!["--version".to_owned()]],
    )?;
    let local = LocalProcessHost::builder(LocalProcessLimits::new(
        256,
        1024 * 1024,
        8 * 1024 * 1024,
        8 * 1024 * 1024,
        1024 * 1024,
    ))
    .approve_executable_launch(
        executable,
        LocalExecutableLaunch::new(node_path)
            .with_prefix_arguments(recipe.prefix_arguments().map(OsStr::to_os_string))
            .with_node_startup_environment_filter(),
    )
    .approve_environment(
        environment.clone(),
        [
            (OsString::from("HOME"), home),
            (OsString::from("USER"), user),
            (OsString::from("LOGNAME"), logname),
            (OsString::from("PATH"), path),
        ],
    )
    .approve_working_resource(working_resource.clone(), &workspace)
    .approve_checked_node(checked_process.clone(), checked_launch)?
    .build_services(execution_host_id.clone());

    Ok(LiveGuardedHost {
        local,
        target,
        environment,
        working_resource,
        execution_host_id,
        checked_process,
        _stage_guard: stage_guard,
    })
}

#[cfg(not(unix))]
fn live_guarded_host(_node_modules: &Path, _node_path: &Path) -> io::Result<LiveGuardedHost> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "guarded Claude Agent ACP live gate requires Unix",
    ))
}

#[cfg(unix)]
fn create_stage_root() -> io::Result<PathBuf> {
    use std::os::unix::fs::PermissionsExt;

    let base = Path::new("/tmp").canonicalize()?;
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos())
        .unwrap_or(0);
    let root = base.join(format!(
        "swallowtail-claude-acp-http-mcp-{}-{nonce}",
        std::process::id()
    ));
    std::fs::create_dir(&root)?;
    std::fs::set_permissions(&root, std::fs::Permissions::from_mode(0o700))?;
    root.canonicalize()
}

#[cfg(unix)]
fn create_private_dirs(root: &Path, target: &Path) -> io::Result<()> {
    use std::os::unix::fs::PermissionsExt;

    let relative = target.strip_prefix(root).map_err(|_| {
        io::Error::new(io::ErrorKind::InvalidInput, "staged directory escapes root")
    })?;
    let mut current = root.to_path_buf();
    for component in relative.components() {
        current.push(component);
        if !current.exists() {
            std::fs::create_dir(&current)?;
            std::fs::set_permissions(&current, std::fs::Permissions::from_mode(0o700))?;
        }
    }
    Ok(())
}

#[cfg(unix)]
fn capture_trusted_runtime(file: &Path) -> io::Result<LocalTrustedRuntime> {
    use std::os::unix::fs::MetadataExt;

    let canonical = file.canonicalize()?;
    let digest: [u8; 32] = Sha256::digest(std::fs::read(&canonical)?).into();
    let users = vec![0, nix::unistd::geteuid().as_raw()];
    let groups = canonical
        .ancestors()
        .filter_map(|ancestor| ancestor.metadata().ok().map(|meta| meta.gid()))
        .collect();
    LocalTrustedRuntime::capture_with_trust(
        &canonical,
        digest,
        &[],
        LocalRuntimeTrust::new(users, groups)?,
    )
}

fn installed_sidecar_root() -> Option<(PathBuf, PathBuf)> {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let node_modules = manifest.join("../..").join("node_modules");
    let sidecar = node_modules.join(".bin/claude-agent-acp");
    if !sidecar.is_file() {
        return None;
    }
    let node = installed_path("node")?;
    Some((node_modules, node))
}

fn installed_path(command: &str) -> Option<PathBuf> {
    std::env::var_os("PATH")
        .into_iter()
        .flat_map(|path| std::env::split_paths(&path).collect::<Vec<_>>())
        .map(|directory| directory.join(command))
        .find(|candidate| candidate.is_file())
        .and_then(|path| std::fs::canonicalize(path).ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    use swallowtail_core::{ExtensionNamespace, ProviderExtension};

    #[test]
    fn guarded_live_gate_manifest_maps_every_frozen_0_113_0_entry() {
        for platform in ["darwin-arm64", "linux-x64"] {
            let frozen = claude_agent_acp_guarded_v1_frozen(HTTP_MCP_HONOURING_VERSION, platform)
                .expect("0.113.0 frozen guarded manifest exists");
            let (files, native_specifier) =
                manifest_entries(frozen.manifest(), platform).expect("manifest parses");
            assert!(!files.is_empty(), "manifest contains captured source files");
            assert!(
                native_specifier.ends_with("/claude"),
                "native specifier names claude binary: {native_specifier}"
            );
            let fake_root = Path::new("/nonexistent/node_modules");
            for relative in &files {
                let resolved = resolve_manifest_source(fake_root, relative)
                    .expect("every manifest prefix maps into node_modules");
                assert!(resolved.starts_with(fake_root));
            }
        }
    }

    #[test]
    fn allow_once_selector_reads_offered_option_id() {
        let extension = ProviderExtension::new(
            ExtensionNamespace::new("acp/session/request-permission").expect("namespace"),
            br#"{"options":[{"optionId":"reject","kind":"reject_once"},{"optionId":"allow","kind":"allow_once"}]}"#
                .to_vec(),
        );
        let kind = CallbackRequestKind::Extension(extension);
        assert_eq!(select_allow_once_option_id(&kind).as_deref(), Some("allow"));
    }
}
