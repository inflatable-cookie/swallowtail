#![allow(dead_code)]

#[path = "acp_support/mod.rs"]
mod support;

#[path = "http_mcp_live/mod.rs"]
mod http_mcp_live;

use futures_executor::block_on;
use futures_util::StreamExt;
use http_mcp_live::{
    DisposableHttpMcpServer, HTTP_MCP_LIVE_TOOL, HTTP_MCP_LIVE_TOOL_RESULT, HttpMcpLiveRecord,
};
use support::{FixtureHost, Scenario, selection};
use swallowtail_adapter_opencode::{
    OPENCODE_ACP_MCP_SERVER_NAME, OpenCodeAcpDriver, OpenCodeAcpRemoteMcpPlacement,
};
use swallowtail_core::{ExecutionHostId, ResourceAccess};
use swallowtail_runtime::{
    CleanupOutcome, InteractiveSessionDriver, OpenSessionRequest, OperationContent, RequestId,
    RuntimeTurnId, SessionPlanAgreement, TerminalStatus, TurnRequest,
};

#[test]
fn disposable_mcp_server_rejects_missing_bearer_and_serves_one_tool() {
    let server = DisposableHttpMcpServer::start();
    let denied = support::connect_and_list(
        server.endpoint(),
        &[("Authorization".to_owned(), "Bearer wrong-token".to_owned())],
    );
    assert!(denied.is_err(), "bearer mismatch must fail");
    let listed = support::connect_and_list(
        server.endpoint(),
        &[("Authorization".to_owned(), server.authorization_header())],
    );
    assert!(listed.is_ok(), "{listed:?}");
    assert!(server.transcript().connected());
    assert!(server.transcript().tools_listed());
    let called = support::call_ping(
        server.endpoint(),
        &[("Authorization".to_owned(), server.authorization_header())],
    );
    assert_eq!(called, Ok(HTTP_MCP_LIVE_TOOL_RESULT.to_owned()));
    assert!(server.transcript().tool_called());
    assert_eq!(
        server.transcript().tool_result(),
        Some(HTTP_MCP_LIVE_TOOL_RESULT)
    );
}

#[test]
fn harness_proves_declaration_connect_list_call_result_and_cleanup() {
    let server = DisposableHttpMcpServer::start();
    let host_id = ExecutionHostId::new("fixture.host.http-mcp-live").expect("valid host id");
    let selected = selection(host_id.clone());
    let host = FixtureHost::new(Scenario::HttpMcpHonour);
    let services = host.services(host_id);
    let placement = OpenCodeAcpRemoteMcpPlacement::new(
        OPENCODE_ACP_MCP_SERVER_NAME,
        server.endpoint(),
        vec![("Authorization".to_owned(), server.authorization_header())],
    );
    let driver = OpenCodeAcpDriver::new(
        swallowtail_runtime::EnvironmentRef::new("opencode.acp.fixture.isolated")
            .expect("valid environment"),
    )
    .with_http_mcp_placement(placement)
    .expect("route-owned http placement is admitted");
    let mut session = block_on(driver.open_session(
        selected.plan,
        OpenSessionRequest::new(
            RequestId::new("opencode-http-mcp-live").expect("valid request"),
            selected.resource,
            None,
            SessionPlanAgreement::explicit(
                swallowtail_core::SessionAccessPolicy::ambient_harness(ResourceAccess::Read),
                Some(swallowtail_core::SessionProviderStatePolicy::Prohibited),
                Some(swallowtail_core::HarnessConfigurationPosture::Ambient),
            ),
        ),
        services.clone(),
    ))
    .expect("session opens");
    let declaration_sent = host.writes().iter().any(|message| {
        message["method"] == "session/new"
            && message["params"]["mcpServers"][0]["type"] == "http"
            && message["params"]["mcpServers"][0]["name"] == OPENCODE_ACP_MCP_SERVER_NAME
            && message["params"]["mcpServers"][0]["url"] == server.endpoint()
            && message["params"]["mcpServers"][0]["headers"][0]["name"] == "Authorization"
    });
    assert!(declaration_sent);
    let debug_server = format!("{server:?}");
    assert!(
        !debug_server.contains(server.bearer()) && !debug_server.contains(server.endpoint()),
        "disposable MCP server Debug must redact the URL and bearer"
    );
    let mut turn = block_on(
        session.start_turn(
            TurnRequest::new(
                RuntimeTurnId::new("opencode-http-mcp-live-turn").expect("valid turn"),
                OperationContent::new(format!(
                    "Call the tool named {HTTP_MCP_LIVE_TOOL}. Do not finish until it returns."
                ))
                .expect("valid prompt"),
            ),
            services.clone(),
        ),
    )
    .expect("turn starts");
    let mut events = turn.take_events().expect("events are available");
    let terminal = block_on(async {
        while let Some(event) = events.next().await {
            event.expect("harness event remains valid");
        }
        turn.take_terminal_outcome()
            .expect("terminal outcome is available")
            .await
    });
    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert_eq!(
        terminal.output().map(OperationContent::as_str),
        Some(HTTP_MCP_LIVE_TOOL_RESULT)
    );
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    let cleanup = block_on(session.close(host.cleanup_request(), services));
    let record = HttpMcpLiveRecord::from_attempt(
        declaration_sent,
        &server.transcript(),
        terminal.status(),
        cleanup,
        Some("fixture".to_owned()),
    );
    assert!(record.accepted(), "{record:?}");
    assert!(record.declaration_sent());
    assert!(record.connected());
    assert!(record.tools_listed());
    assert!(record.tool_called());
    assert!(record.tool_result());
    assert!(record.terminal_completed());
    assert!(record.cleanup_clean());
    assert!(record.stop().is_none());
    let debug_record = format!("{record:?}");
    assert!(
        !debug_record.contains(server.bearer()) && !debug_record.contains(server.endpoint()),
        "live record Debug must not carry the URL or bearer"
    );
}
