#[path = "http_mcp_live/mod.rs"]
mod http_mcp_live;

use futures_executor::block_on;
use futures_util::StreamExt;
use http_mcp_live::{
    DisposableHttpMcpServer, HTTP_MCP_LIVE_TOOL, HTTP_MCP_LIVE_TOOL_RESULT, HttpMcpLiveRecord,
    HttpMcpLiveStop,
};
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::time::Duration;
use swallowtail_adapter_claude_agent::{
    CLAUDE_AGENT_ACP_AXIS, CLAUDE_AGENT_ACP_LATEST_QUALIFIED_VERSION,
    CLAUDE_AGENT_ACP_MCP_SERVER_NAME, ClaudeAgentAcpRemoteMcpPlacement, ClaudeAgentModelSelection,
    ClaudeAgentPreparationInput, ClaudeAgentPreparationProbe, ClaudeAgentSessionProfileInput,
    claude_agent_acp_subscription_access_profile, prepare_claude_agent,
};
use swallowtail_core::{
    AccessProfileId, AccessStatus, ConfiguredInstanceId, CredentialState, EndpointAuthorization,
    EntitlementState, ExecutionHostId, InstanceRevision, InterfaceVersionAxis, ModelId,
    ModelRouteId, ModelRouteRevision, RuntimeReadiness, SupportAuthority,
};
use swallowtail_host_local::{LocalExecutableLaunch, LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    CallbackPayload, CallbackResponse, CallbackResult, CleanupOutcome, DiscoveryCancellation,
    EnvironmentRef, ExecutableRef, OperationContent, PreparedAccessEvidence, RequestId,
    RuntimeTurnId, ScopeId, SessionOptions, TerminalStatus, TurnRequest, WorkingResourceRef,
};

const LIVE_GATE: &str = "SWALLOWTAIL_LIVE_CLAUDE_AGENT_ACP_HTTP_MCP";
const LIVE_MODEL: &str = "claude-sonnet-4-6";

#[test]
#[ignore = "requires SWALLOWTAIL_LIVE_CLAUDE_AGENT_ACP_HTTP_MCP=1, repo-local claude-agent-acp 0.79.0, and local Claude subscription auth"]
fn one_authorized_claude_agent_acp_http_mcp_live_attempt() {
    assert_eq!(
        std::env::var(LIVE_GATE).as_deref(),
        Ok("1"),
        "Claude Agent ACP HTTP MCP live gate requires its explicit env gate"
    );
    let record = run_one_attempt();
    eprintln!(
        "claude-agent.acp http mcp live record accepted={} stop={:?} model={:?}",
        record.accepted(),
        record.stop().map(HttpMcpLiveStop::as_str),
        record.model()
    );
    assert!(
        record.accepted() || record.stop().is_some(),
        "one attempt must accept or name a typed stop: {record:?}"
    );
}

fn run_one_attempt() -> HttpMcpLiveRecord {
    let Some((sidecar, node)) = installed_sidecar() else {
        return HttpMcpLiveRecord::pre_attempt_stop(HttpMcpLiveStop::HostVersion, None);
    };
    let workspace = std::env::temp_dir().join(format!(
        "swallowtail-claude-agent-acp-http-mcp-live-{}",
        std::process::id()
    ));
    if std::fs::create_dir_all(&workspace).is_err() {
        return HttpMcpLiveRecord::pre_attempt_stop(HttpMcpLiveStop::HostVersion, None);
    }
    let (local, target, environment, working_resource, execution_host_id) =
        live_host(&sidecar, &node, &workspace);
    let access_id = AccessProfileId::new("live.claude-agent.acp.subscription").expect("access id");
    let prepared = match block_on(prepare_claude_agent(
        ClaudeAgentPreparationInput::new(
            ConfiguredInstanceId::new("live.claude-agent.acp.instance").expect("instance id"),
            InstanceRevision::new(CLAUDE_AGENT_ACP_LATEST_QUALIFIED_VERSION)
                .expect("instance revision"),
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
        ),
        ClaudeAgentPreparationProbe::new(
            RequestId::new("live.claude-agent.acp.http-mcp.prepare").expect("request id"),
            ScopeId::new("live.claude-agent.acp.http-mcp.prepare").expect("scope id"),
            local.deadline_after(Duration::from_secs(10)),
            DiscoveryCancellation::new(),
        ),
        local.services().clone(),
    )) {
        Ok(prepared) => prepared,
        Err(_) => {
            return HttpMcpLiveRecord::pre_attempt_stop(HttpMcpLiveStop::HostVersion, None);
        }
    };
    if prepared.observation().version().version().as_str()
        != CLAUDE_AGENT_ACP_LATEST_QUALIFIED_VERSION
    {
        return HttpMcpLiveRecord::pre_attempt_stop(
            HttpMcpLiveStop::HostVersion,
            Some(
                prepared
                    .observation()
                    .version()
                    .version()
                    .as_str()
                    .to_owned(),
            ),
        );
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
            let mut record = HttpMcpLiveRecord::from_attempt(
                true,
                &server.transcript(),
                &TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                CleanupOutcome::Failed(error.diagnostic().clone()),
                Some(LIVE_MODEL.to_owned()),
            );
            record.force_stop(stop);
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
            let _ = block_on(handle.close(
                swallowtail_runtime::SessionCleanupRequest::new(
                    local.deadline_after(Duration::from_secs(30)),
                ),
                local.services().clone(),
            ));
            return HttpMcpLiveRecord::from_attempt(
                true,
                &server.transcript(),
                &TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                CleanupOutcome::Failed(error.diagnostic().clone()),
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
                    let _ = responder
                        .respond(CallbackResponse::new(
                            callback.callback_id().clone(),
                            turn_id.clone(),
                            CallbackResult::Success(
                                CallbackPayload::new(br#"{"optionId":"allow-once"}"#, 256)
                                    .expect("allow-once is bounded"),
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
    HttpMcpLiveRecord::from_attempt(
        true,
        &server.transcript(),
        outcome.status(),
        cleanup,
        Some(LIVE_MODEL.to_owned()),
    )
}

fn live_host(
    sidecar: &Path,
    node: &Path,
    workspace: &Path,
) -> (
    swallowtail_host_local::LocalHostServices,
    swallowtail_runtime::InstalledExecutableTarget,
    EnvironmentRef,
    WorkingResourceRef,
    ExecutionHostId,
) {
    let environment = EnvironmentRef::new("live.claude-agent.acp.isolated").expect("environment");
    let working_resource =
        WorkingResourceRef::new("live.claude-agent.acp.workspace").expect("resource");
    let execution_host_id =
        ExecutionHostId::new("live.claude-agent.acp.local-host").expect("host id");
    let executable =
        ExecutableRef::new("live.claude-agent.acp.installed").expect("executable ref is valid");
    let (builder, target) = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_installed_executable_launch(
            executable,
            InterfaceVersionAxis::new(CLAUDE_AGENT_ACP_AXIS).expect("axis"),
            LocalExecutableLaunch::interpreted_script(node, sidecar),
        );
    let home = std::env::var_os("HOME").expect("local Claude auth requires HOME");
    let user = std::env::var_os("USER").unwrap_or_else(|| OsString::from("swallowtail"));
    let logname = std::env::var_os("LOGNAME").unwrap_or_else(|| user.clone());
    let path = std::env::var_os("PATH").unwrap_or_default();
    let local = builder
        .approve_environment(
            environment.clone(),
            [
                (OsString::from("HOME"), home),
                (OsString::from("USER"), user),
                (OsString::from("LOGNAME"), logname),
                (OsString::from("PATH"), path),
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

fn installed_sidecar() -> Option<(PathBuf, PathBuf)> {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let sidecar = manifest
        .join("../..")
        .join("node_modules/.bin/claude-agent-acp");
    if !sidecar.is_file() {
        return None;
    }
    let sidecar = std::fs::canonicalize(sidecar).ok()?;
    let node = installed_path("node")?;
    Some((sidecar, node))
}

fn installed_path(command: &str) -> Option<PathBuf> {
    std::env::var_os("PATH")
        .into_iter()
        .flat_map(|path| std::env::split_paths(&path).collect::<Vec<_>>())
        .map(|directory| directory.join(command))
        .find(|candidate| candidate.is_file())
        .and_then(|path| std::fs::canonicalize(path).ok())
}
