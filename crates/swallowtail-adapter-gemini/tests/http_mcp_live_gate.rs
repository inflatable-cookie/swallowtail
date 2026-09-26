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
use std::process::Command;
use std::time::Duration;
use swallowtail_adapter_gemini::{
    GEMINI_ACP_MCP_SERVER_NAME, GEMINI_CLI_ACP_AXIS, GEMINI_CLI_ACP_LATEST_QUALIFIED_VERSION,
    GeminiAcpHttpMcpPlacement, GeminiPreparationInput, GeminiPreparationProbe,
    GeminiSessionProfileInput, prepare_gemini_acp,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, ConfiguredInstanceId, CredentialMechanism,
    CredentialRef, CredentialState, EndpointAudience, EndpointAuthorization, EntitlementMetering,
    EntitlementState, ExecutionHostId, InstanceRevision, InterfaceVersionAxis, RuntimeReadiness,
    SafeDiagnostic, SupportAuthority,
};
use swallowtail_host_local::{LocalExecutableLaunch, LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    CleanupOutcome, DiscoveryCancellation, EnvironmentRef, ExecutableRef, OperationContent,
    PreparedAccessEvidence, RequestId, RuntimeTurnId, ScopeId, SessionOptions, TerminalStatus,
    TurnRequest, WorkingResourceRef,
};

const LIVE_GATE: &str = "SWALLOWTAIL_LIVE_GEMINI_ACP_HTTP_MCP";
const EXACT_VERSION: &str = GEMINI_CLI_ACP_LATEST_QUALIFIED_VERSION;
const CREDENTIAL: &str = "live.gemini.acp.credential";
const AUDIENCE: &str = "gemini-developer-api";

#[test]
#[ignore = "requires SWALLOWTAIL_LIVE_GEMINI_ACP_HTTP_MCP=1, host gemini-cli 0.61.0, and existing host Gemini API-key auth"]
fn one_authorized_gemini_cli_acp_http_mcp_live_attempt() {
    assert_eq!(
        std::env::var(LIVE_GATE).as_deref(),
        Ok("1"),
        "Gemini CLI ACP HTTP MCP live gate requires its explicit env gate"
    );
    let record = run_one_attempt();
    eprintln!(
        "gemini-cli.acp http mcp live record accepted={} stop={:?} model={:?} terminal_diagnostic={:?} cleanup_diagnostic={:?}",
        record.accepted(),
        record.stop().map(HttpMcpLiveStop::as_str),
        record.model(),
        record
            .terminal_diagnostic()
            .map(|diagnostic| (diagnostic.code(), diagnostic.class())),
        record
            .cleanup_diagnostic()
            .map(|diagnostic| (diagnostic.code(), diagnostic.class())),
    );
    assert!(
        record.accepted() || record.stop().is_some(),
        "one attempt must accept or name a typed stop: {record:?}"
    );
}

fn run_one_attempt() -> HttpMcpLiveRecord {
    if !host_auth_present() {
        return HttpMcpLiveRecord::pre_attempt_stop(
            HttpMcpLiveStop::HostAuthRequired,
            gate_diagnostic(
                "swallowtail.gemini.acp.http_mcp.host_auth_missing",
                "Existing host Gemini auth is not present",
            ),
            None,
        );
    }
    let Some((gemini, node)) = host_gemini() else {
        return HttpMcpLiveRecord::pre_attempt_stop(
            HttpMcpLiveStop::HostVersion,
            host_version_diagnostic(),
            None,
        );
    };
    let workspace = std::env::temp_dir().join(format!(
        "swallowtail-gemini-acp-http-mcp-live-{}",
        std::process::id()
    ));
    if std::fs::create_dir_all(&workspace).is_err() {
        return HttpMcpLiveRecord::pre_attempt_stop(
            HttpMcpLiveStop::HostVersion,
            host_version_diagnostic(),
            None,
        );
    }
    let credential = CredentialRef::new(CREDENTIAL).expect("credential");
    let (local, target, environment, working_resource, execution_host_id) =
        live_host(&gemini, &node, &workspace, credential.clone());
    let access_id = AccessProfileId::new("live.gemini.acp.api-key").expect("access id");
    let prepared = match block_on(prepare_gemini_acp(
        GeminiPreparationInput::new(
            ConfiguredInstanceId::new("live.gemini.acp.instance").expect("instance id"),
            InstanceRevision::new(EXACT_VERSION).expect("instance revision"),
            execution_host_id,
            target,
            environment,
            AccessProfile::new(
                access_id.clone(),
                CredentialMechanism::ApiKey,
                EntitlementMetering::PayAsYouGo,
                EndpointAudience::new(AUDIENCE).expect("audience"),
                SupportAuthority::ProviderSupported,
            )
            .with_credential_reference(credential),
            PreparedAccessEvidence::caller_asserted(AccessStatus::new(
                access_id,
                CredentialState::Ready,
                EntitlementState::Available,
                EndpointAuthorization::Allowed,
                RuntimeReadiness::Ready,
                SupportAuthority::ProviderSupported,
            )),
        ),
        GeminiPreparationProbe::new(
            RequestId::new("live.gemini.acp.http-mcp.prepare").expect("request id"),
            ScopeId::new("live.gemini.acp.http-mcp.prepare").expect("scope id"),
            local.deadline_after(Duration::from_secs(10)),
            DiscoveryCancellation::new(),
        ),
        local.services().clone(),
    )) {
        Ok(prepared) => prepared,
        Err(error) => {
            return HttpMcpLiveRecord::pre_attempt_stop(
                HttpMcpLiveStop::HostVersion,
                error.diagnostic().safe().clone(),
                None,
            );
        }
    };
    if prepared.observation().version().version().as_str() != EXACT_VERSION {
        return HttpMcpLiveRecord::pre_attempt_stop(
            HttpMcpLiveStop::HostVersion,
            host_version_diagnostic(),
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
    let placement = GeminiAcpHttpMcpPlacement::new(
        GEMINI_ACP_MCP_SERVER_NAME,
        server.endpoint(),
        vec![("Authorization".to_owned(), server.authorization_header())],
    );
    let session = match prepared.prepare_session(
        GeminiSessionProfileInput::new(
            RequestId::new("live.gemini.acp.http-mcp.session").expect("request id"),
            working_resource,
            SessionOptions::default(),
        )
        .with_http_mcp_placement(placement),
    ) {
        Ok(session) => session,
        Err(error) => {
            return HttpMcpLiveRecord::pre_attempt_stop(
                HttpMcpLiveStop::NoUsableModel,
                error.diagnostic().safe().clone(),
                None,
            );
        }
    };

    let mut handle = match block_on(session.open_session(local.services().clone())) {
        Ok(handle) => handle,
        Err(error) => {
            let stop = if error.diagnostic().code() == "swallowtail.gemini.acp.auth_required" {
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
                None,
            );
            record.force_stop(stop);
            return record;
        }
    };
    let model = handle
        .negotiated_model_options()
        .map(|options| options.current_value().to_owned());

    let mut turn = match block_on(handle.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("live.gemini.acp.http-mcp.turn").expect("turn id"),
            OperationContent::new(format!(
                "Call the MCP tool named {HTTP_MCP_LIVE_TOOL}. Do not finish the turn until that tool call has returned. Then reply with exactly {HTTP_MCP_LIVE_TOOL_RESULT}."
            ))
            .expect("prompt"),
        ),
        local.services().clone(),
    )) {
        Ok(turn) => turn,
        Err(error) => {
            let cleanup = block_on(handle.close(
                swallowtail_runtime::SessionCleanupRequest::new(
                    local.deadline_after(Duration::from_secs(30)),
                ),
                local.services().clone(),
            ));
            return HttpMcpLiveRecord::from_attempt(
                true,
                &server.transcript(),
                &TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                cleanup,
                model,
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
    HttpMcpLiveRecord::from_attempt(true, &server.transcript(), outcome.status(), cleanup, model)
}

fn live_host(
    gemini: &Path,
    node: &Path,
    workspace: &Path,
    credential: CredentialRef,
) -> (
    swallowtail_host_local::LocalHostServices,
    swallowtail_runtime::InstalledExecutableTarget,
    EnvironmentRef,
    WorkingResourceRef,
    ExecutionHostId,
) {
    let environment = EnvironmentRef::new("live.gemini.acp.isolated").expect("environment");
    let working_resource = WorkingResourceRef::new("live.gemini.acp.workspace").expect("resource");
    let execution_host_id = ExecutionHostId::new("live.gemini.acp.local-host").expect("host id");
    let executable = ExecutableRef::new("live.gemini.acp.installed").expect("executable ref");
    let (builder, target) = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_installed_executable_launch(
            executable,
            InterfaceVersionAxis::new(GEMINI_CLI_ACP_AXIS).expect("axis"),
            LocalExecutableLaunch::interpreted_script(node, gemini),
        );
    let home = std::env::var_os("HOME").expect("existing Gemini auth requires HOME");
    let path = std::env::var_os("PATH").unwrap_or_default();
    let user = std::env::var_os("USER").unwrap_or_else(|| OsString::from("swallowtail"));
    let logname = std::env::var_os("LOGNAME").unwrap_or_else(|| user.clone());
    let audience = EndpointAudience::new(AUDIENCE).expect("audience");
    let local = builder
        .approve_delegated_credential(credential, audience)
        .approve_environment(
            environment.clone(),
            [
                (OsString::from("HOME"), home),
                (OsString::from("PATH"), path),
                (OsString::from("USER"), user),
                (OsString::from("LOGNAME"), logname),
                (
                    OsString::from("GEMINI_CLI_TRUST_WORKSPACE"),
                    OsString::from("true"),
                ),
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

fn host_gemini() -> Option<(PathBuf, PathBuf)> {
    let gemini = installed_path("gemini")?;
    let output = Command::new(&gemini).arg("--version").output().ok()?;
    if !output.status.success() {
        return None;
    }
    let version = String::from_utf8(output.stdout).ok()?;
    if !version_is_exact(version.trim()) {
        return None;
    }
    let node = installed_path("node")?;
    Some((gemini, node))
}

fn version_is_exact(reported: &str) -> bool {
    reported == EXACT_VERSION
        || reported
            .split_whitespace()
            .any(|token| token == EXACT_VERSION)
}

fn installed_path(command: &str) -> Option<PathBuf> {
    std::env::var_os("PATH")
        .into_iter()
        .flat_map(|path| std::env::split_paths(&path).collect::<Vec<_>>())
        .map(|directory| directory.join(command))
        .find(|candidate| candidate.is_file())
        .and_then(|path| std::fs::canonicalize(path).ok())
}

fn host_version_diagnostic() -> SafeDiagnostic {
    gate_diagnostic(
        "swallowtail.gemini.acp.http_mcp.host_version",
        "Host Gemini CLI 0.61.0 was not available",
    )
}

fn gate_diagnostic(code: &'static str, message: &'static str) -> SafeDiagnostic {
    SafeDiagnostic::new(code, message)
}

fn host_auth_present() -> bool {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .is_some_and(|home| home.join(".gemini").is_dir())
}
