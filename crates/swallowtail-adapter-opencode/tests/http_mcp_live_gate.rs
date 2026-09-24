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
use swallowtail_adapter_opencode::{
    OPENCODE_ACP_AXIS, OPENCODE_ACP_BASELINE_VERSION, OPENCODE_ACP_EXECUTABLE_NAME,
    OPENCODE_ACP_MCP_SERVER_NAME, OpenCodeAcpPreparationInput, OpenCodeAcpPreparationProbe,
    OpenCodeAcpRemoteMcpPlacement, OpenCodeAcpSessionProfileInput,
    opencode_acp_host_account_access_profile, prepare_opencode_acp,
};
use swallowtail_core::{
    AccessProfileId, AccessStatus, ConfiguredInstanceId, CredentialState, EndpointAuthorization,
    EntitlementState, ExecutionHostId, InstanceRevision, InterfaceVersionAxis, RuntimeReadiness,
    SupportAuthority,
};
use swallowtail_host_local::{LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    CleanupOutcome, DiscoveryCancellation, EnvironmentRef, ExecutableRef, OperationContent,
    PreparedAccessEvidence, RequestId, RuntimeTurnId, ScopeId, TerminalStatus, TurnRequest,
    WorkingResourceRef,
};

const LIVE_GATE: &str = "SWALLOWTAIL_LIVE_OPENCODE_ACP_HTTP_MCP";
const CONFIGURED_MODEL: &str = "kimi-for-coding/k3";

#[test]
#[ignore = "requires SWALLOWTAIL_LIVE_OPENCODE_ACP_HTTP_MCP=1, host opencode 1.18.18, and an already-configured model"]
fn one_authorized_opencode_acp_http_mcp_live_attempt() {
    assert_eq!(
        std::env::var(LIVE_GATE).as_deref(),
        Ok("1"),
        "OpenCode ACP HTTP MCP live gate requires its explicit env gate"
    );
    let record = run_one_attempt();
    eprintln!(
        "g06.028 live record accepted={} stop={:?} model={:?}",
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
    let Some(opencode) = installed_opencode() else {
        return HttpMcpLiveRecord::pre_attempt_stop(HttpMcpLiveStop::HostVersion, None);
    };
    if !configured_model_is_present() {
        return HttpMcpLiveRecord::pre_attempt_stop(HttpMcpLiveStop::NoUsableModel, None);
    }
    let workspace = std::env::temp_dir().join(format!(
        "swallowtail-opencode-acp-http-mcp-live-{}",
        std::process::id()
    ));
    std::fs::create_dir_all(&workspace).expect("live workspace creates");
    let (local, target, environment, working_resource, execution_host_id) =
        live_host(&opencode, &workspace);
    let access_id = AccessProfileId::new("live.opencode.acp.host-account").expect("access id");
    let prepared = match block_on(prepare_opencode_acp(
        OpenCodeAcpPreparationInput::new(
            ConfiguredInstanceId::new("live.opencode.acp.instance").expect("instance id"),
            InstanceRevision::new(OPENCODE_ACP_BASELINE_VERSION).expect("instance revision"),
            execution_host_id,
            target,
            environment,
            opencode_acp_host_account_access_profile(access_id.clone()),
            PreparedAccessEvidence::caller_asserted(AccessStatus::new(
                access_id,
                CredentialState::NotRequired,
                EntitlementState::Available,
                EndpointAuthorization::Allowed,
                RuntimeReadiness::Ready,
                SupportAuthority::ProviderSupported,
            )),
        ),
        OpenCodeAcpPreparationProbe::new(
            RequestId::new("live.opencode.acp.http-mcp.prepare").expect("request id"),
            ScopeId::new("live.opencode.acp.http-mcp.prepare").expect("scope id"),
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
    if prepared.observation().version().version().as_str() != OPENCODE_ACP_BASELINE_VERSION {
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
    let placement = OpenCodeAcpRemoteMcpPlacement::new(
        OPENCODE_ACP_MCP_SERVER_NAME,
        server.endpoint(),
        vec![("Authorization".to_owned(), server.authorization_header())],
    );
    let session = match prepared.prepare_session(
        OpenCodeAcpSessionProfileInput::new(
            RequestId::new("live.opencode.acp.http-mcp.session").expect("request id"),
            working_resource,
        )
        .with_http_mcp_placement(placement),
    ) {
        Ok(session) => session,
        Err(_) => {
            return HttpMcpLiveRecord::pre_attempt_stop(
                HttpMcpLiveStop::NoUsableModel,
                Some(CONFIGURED_MODEL.to_owned()),
            );
        }
    };

    let mut handle = match block_on(session.open_session(local.services().clone())) {
        Ok(handle) => handle,
        Err(error) => {
            let stop = if error.diagnostic().code() == "swallowtail.opencode.acp.host_auth_required"
            {
                HttpMcpLiveStop::HostAuthRequired
            } else if server.transcript().connected() {
                HttpMcpLiveStop::ToolNotCalled
            } else {
                HttpMcpLiveStop::McpNotConnected
            };
            return {
                let mut record = HttpMcpLiveRecord::from_attempt(
                    true,
                    &server.transcript(),
                    &TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                    CleanupOutcome::Failed(error.diagnostic().clone()),
                    Some(CONFIGURED_MODEL.to_owned()),
                );
                record.force_stop(stop);
                record
            };
        }
    };

    let mut turn = match block_on(handle.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("live.opencode.acp.http-mcp.turn").expect("turn id"),
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
                Some(CONFIGURED_MODEL.to_owned()),
            );
        }
    };

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
        Some(CONFIGURED_MODEL.to_owned()),
    )
}

fn live_host(
    opencode: &Path,
    workspace: &Path,
) -> (
    swallowtail_host_local::LocalHostServices,
    swallowtail_runtime::InstalledExecutableTarget,
    EnvironmentRef,
    WorkingResourceRef,
    ExecutionHostId,
) {
    let environment = EnvironmentRef::new("live.opencode.acp.isolated").expect("environment");
    let working_resource =
        WorkingResourceRef::new("live.opencode.acp.workspace").expect("resource");
    let execution_host_id = ExecutionHostId::new("live.opencode.acp.local-host").expect("host id");
    let executable =
        ExecutableRef::new("live.opencode.acp.installed").expect("executable ref is valid");
    let (builder, target) = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_installed_executable(
            executable,
            InterfaceVersionAxis::new(OPENCODE_ACP_AXIS).expect("axis"),
            opencode,
        );
    let home = std::env::var_os("HOME").expect("host-owned OpenCode login requires HOME");
    let path = std::env::var_os("PATH").unwrap_or_default();
    let local = builder
        .approve_environment(
            environment.clone(),
            [
                (OsString::from("HOME"), home),
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

fn installed_opencode() -> Option<PathBuf> {
    let mut candidates = Vec::new();
    if let Some(home) = std::env::var_os("HOME") {
        candidates.push(
            PathBuf::from(home)
                .join(".opencode/bin")
                .join(OPENCODE_ACP_EXECUTABLE_NAME),
        );
    }
    if let Some(path) = std::env::var_os("PATH") {
        candidates.extend(
            std::env::split_paths(&path)
                .map(|directory| directory.join(OPENCODE_ACP_EXECUTABLE_NAME)),
        );
    }
    candidates.into_iter().find(|candidate| candidate.is_file())
}

fn configured_model_is_present() -> bool {
    let Some(home) = std::env::var_os("HOME") else {
        return false;
    };
    let config = PathBuf::from(home).join(".config/opencode/opencode.json");
    let Ok(text) = std::fs::read_to_string(config) else {
        return false;
    };
    let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) else {
        return false;
    };
    value
        .pointer("/provider/kimi-for-coding/models/k3")
        .is_some()
}
