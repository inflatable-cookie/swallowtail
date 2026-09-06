//! Provider-free proofs for consumer-declared stdio MCP servers.

use crate::host_id;
use crate::sdk_support::{
    SdkFixtureHost, SdkScenario, cleanup_request, prepared_session, prepared_session_with_mcp,
    turn_request,
};
use futures_executor::block_on;
use futures_util::StreamExt;
use swallowtail_adapter_claude_agent::sdk::{
    ClaudeAgentSdkMcpServer, ClaudeAgentSdkMcpServerStatusKind, ClaudeAgentSdkSessionProfile,
};
use swallowtail_runtime::{
    CallbackPayload, CallbackResponse, CallbackResult, InteractiveSessionHandle,
};

fn fixture_server(optional: bool) -> ClaudeAgentSdkMcpServer {
    let server = ClaudeAgentSdkMcpServer::stdio(
        "fixture",
        "/usr/bin/node",
        ["server.mjs"],
        ["PATH", "HOME"],
        ["search"],
    )
    .expect("fixture MCP server is admissible");
    if optional { server.optional() } else { server }
}

fn fixture_binding(
    optional: bool,
) -> swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkMcpBinding {
    ClaudeAgentSdkSessionProfile::read_only()
        .with_mcp_servers([fixture_server(optional)])
        .expect("fixture MCP binding is admissible")
}

fn close_session(
    session: swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkSessionHandle,
    services: swallowtail_runtime::HostServices,
) {
    let _ = block_on(Box::new(session).close(cleanup_request(), services));
}

#[test]
fn the_default_open_omits_mcp_servers_and_reports_empty_status() {
    let host = host_id("claude-agent-sdk.fixture.mcp-default");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let prepared = prepared_session(host.clone());
    assert!(prepared.mcp_servers().is_empty());
    let services = fixture.services(host);
    let cleanup_services = services.clone();
    let session = block_on(prepared.open_route_session(services)).expect("default session opens");
    assert!(session.mcp_server_status().is_empty());
    let open = fixture
        .inputs()
        .into_iter()
        .find(|input| input["command"] == "open")
        .expect("default open is on the wire");
    assert!(
        open["params"].get("mcpServers").is_none(),
        "the default profile must omit mcpServers: {open}"
    );
    close_session(session, cleanup_services);
}

#[test]
fn a_connected_stdio_server_appears_in_open_evidence() {
    let host = host_id("claude-agent-sdk.fixture.mcp-connected");
    let fixture = SdkFixtureHost::new(SdkScenario::McpConnected);
    let prepared = prepared_session_with_mcp(host.clone(), fixture_binding(false));
    let services = fixture.services(host);
    let cleanup_services = services.clone();
    let session = block_on(prepared.open_route_session(services)).expect("MCP session opens");
    let status = session.mcp_server_status();
    assert_eq!(status.len(), 1);
    assert_eq!(status[0].name(), "fixture");
    assert_eq!(
        status[0].kind(),
        ClaudeAgentSdkMcpServerStatusKind::Connected
    );
    assert_eq!(status[0].failure_code(), None);
    let open = fixture
        .inputs()
        .into_iter()
        .find(|input| input["command"] == "open")
        .expect("MCP open is on the wire");
    assert_eq!(
        open["params"]["tools"],
        serde_json::json!(["Read", "Glob", "Grep", "mcp__fixture__search"])
    );
    close_session(session, cleanup_services);
}

#[test]
fn a_failing_required_server_fails_open_typed() {
    let host = host_id("claude-agent-sdk.fixture.mcp-required-fail");
    let fixture = SdkFixtureHost::new(SdkScenario::McpRequiredFail);
    let prepared = prepared_session_with_mcp(host.clone(), fixture_binding(false));
    let services = fixture.services(host);
    let Err(error) = block_on(prepared.open_session(services)) else {
        panic!("required MCP failure must fail open");
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.claude-agent.sdk.open_rejected"
    );
    assert!(
        error
            .diagnostic()
            .message()
            .ends_with(": mcp_server_failed")
    );
}

#[test]
fn an_optional_server_failure_is_recorded_without_failing_open() {
    let host = host_id("claude-agent-sdk.fixture.mcp-optional-fail");
    let fixture = SdkFixtureHost::new(SdkScenario::McpOptionalFail);
    let prepared = prepared_session_with_mcp(host.clone(), fixture_binding(true));
    let services = fixture.services(host);
    let cleanup_services = services.clone();
    let session = block_on(prepared.open_route_session(services)).expect("optional MCP opens");
    let status = session.mcp_server_status();
    assert_eq!(status.len(), 1);
    assert_eq!(status[0].kind(), ClaudeAgentSdkMcpServerStatusKind::Failed);
    assert_eq!(
        status[0].failure_code(),
        Some("swallowtail.claude-agent.sdk.mcp_server_failed")
    );
    close_session(session, cleanup_services);
}

#[test]
fn an_admitted_mcp_tool_is_mediated_through_can_use_tool() {
    let host = host_id("claude-agent-sdk.fixture.mcp-admission");
    let fixture = SdkFixtureHost::new(SdkScenario::McpAdmission);
    let prepared = prepared_session_with_mcp(host.clone(), fixture_binding(false));
    let services = fixture.services(host);
    let services_for_cleanup = services.clone();
    let mut session = block_on(prepared.open_session(services.clone())).expect("MCP session opens");
    let mut turn = block_on(session.start_turn(turn_request("turn-1", "search it"), services))
        .expect("MCP turn starts");
    let mut callbacks = turn.take_callbacks().expect("turn exposes tool admission");
    let mut requests = callbacks
        .take_requests()
        .expect("admission requests are available once");
    let request = block_on(requests.next())
        .expect("one MCP admission request arrives")
        .expect("admission request is healthy");
    let swallowtail_runtime::CallbackRequestKind::Extension(extension) = request.kind() else {
        panic!("MCP admission is a route-local extension callback");
    };
    assert_eq!(
        extension.namespace().as_str(),
        "claude-agent-sdk/can-use-tool"
    );
    let payload: serde_json::Value =
        serde_json::from_slice(extension.payload()).expect("payload is JSON");
    assert_eq!(
        payload,
        serde_json::json!({"toolName": "mcp__fixture__search"})
    );
    assert_eq!(payload.as_object().expect("object").len(), 1);
    block_on(
        callbacks.responder().respond(CallbackResponse::new(
            request.callback_id().clone(),
            swallowtail_runtime::RuntimeTurnId::new("turn-1").expect("valid turn"),
            CallbackResult::Success(
                CallbackPayload::new(br#"{"decision":"allow"}"#.to_vec(), 4096)
                    .expect("payload fits the bound"),
            ),
        )),
    )
    .expect("MCP admission response reaches the sidecar");
    let decision = fixture
        .inputs()
        .into_iter()
        .find(|value| value["type"] == "callback_response")
        .expect("a callback response crosses the wire");
    assert_eq!(decision["id"], "cb-1");
    assert_eq!(decision["decision"], "allow");
    let _ = block_on(session.close(cleanup_request(), services_for_cleanup));
}
