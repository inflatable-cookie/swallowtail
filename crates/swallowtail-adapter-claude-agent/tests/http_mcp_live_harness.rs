#![allow(dead_code)]

#[path = "support/mod.rs"]
mod support;

#[path = "http_mcp_live/mod.rs"]
mod http_mcp_live;

use futures_executor::block_on;
use futures_util::StreamExt;
use http_mcp_live::{
    DisposableHttpMcpServer, HTTP_MCP_LIVE_TOOL, HTTP_MCP_LIVE_TOOL_RESULT, HttpMcpCleanupClass,
    HttpMcpLiveRecord, HttpMcpLiveStop, persist_and_print_record,
};
use serde_json::{Value, json};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;
use support::selection::{open_request, selection};
use support::{DeadlineWait, FixtureHost, Scenario, connect_and_list};
use swallowtail_adapter_claude_agent::{
    CLAUDE_AGENT_ACP_MCP_SERVER_NAME, ClaudeAgentAcpDriver, ClaudeAgentAcpRemoteMcpPlacement,
};
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{
    CleanupOutcome, InteractiveSessionDriver, OperationContent, RuntimeTurnId, TerminalStatus,
    TurnRequest,
};

#[test]
fn disposable_mcp_server_rejects_missing_bearer_and_serves_one_tool() {
    let server = DisposableHttpMcpServer::start();
    let denied = connect_and_list(
        server.endpoint(),
        &[("Authorization".to_owned(), "Bearer wrong-token".to_owned())],
    );
    assert!(denied.is_err(), "bearer mismatch must fail");
    let listed = connect_and_list(
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

/// Reads one HTTP response header plus any declared body. TCP may split the
/// listener's single write, so a one-shot read is not a response.
fn read_response(stream: &mut TcpStream) -> String {
    let mut bytes = Vec::new();
    let mut chunk = [0_u8; 2048];
    loop {
        let count = stream.read(&mut chunk).expect("response reads");
        assert!(count > 0, "connection closed before a complete response");
        bytes.extend_from_slice(&chunk[..count]);
        let Some(header_end) = bytes.windows(4).position(|window| window == b"\r\n\r\n") else {
            continue;
        };
        let headers = String::from_utf8_lossy(&bytes[..header_end]).into_owned();
        let length = headers
            .lines()
            .find_map(|line| {
                line.to_ascii_lowercase()
                    .strip_prefix("content-length:")
                    .and_then(|value| value.trim().parse::<usize>().ok())
            })
            .unwrap_or(0);
        if bytes.len() >= header_end + 4 + length {
            return String::from_utf8_lossy(&bytes).into_owned();
        }
    }
}

/// Opens one HTTP/1.1 GET SSE stream with no `Connection` header, so the
/// listener sees a persistent client, and returns the open client.
fn open_idle_sse_stream(server: &DisposableHttpMcpServer) -> (TcpStream, String, String) {
    let endpoint = url::Url::parse(server.endpoint()).expect("loopback endpoint parses");
    let address = format!(
        "{}:{}",
        endpoint.host_str().expect("endpoint has a host"),
        endpoint
            .port_or_known_default()
            .expect("endpoint has a port")
    );
    let mut stream = TcpStream::connect(&address).expect("client connects");
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .expect("client read timeout");
    // No `Connection` header: HTTP/1.1 is persistent by default.
    let open = format!(
        "GET /mcp HTTP/1.1\r\nHost: {address}\r\nAccept: text/event-stream\r\nAuthorization: {}\r\n\r\n",
        server.authorization_header()
    );
    stream.write_all(open.as_bytes()).expect("sse open writes");
    let response = read_response(&mut stream);
    (stream, address, response)
}

#[test]
fn disposable_mcp_server_keeps_an_idle_sse_stream_and_defaults_to_keep_alive() {
    let server = DisposableHttpMcpServer::start();
    let (mut stream, address, response) = open_idle_sse_stream(&server);
    assert!(response.contains("200 OK"), "{response}");
    assert!(response.contains("text/event-stream"), "{response}");
    assert!(response.contains("Connection: keep-alive"), "{response}");
    // The stream is silent now. Wait past several idle poll intervals: a
    // listener that treated an idle read as a close would drop the MCP
    // session's server-to-client stream here.
    std::thread::sleep(Duration::from_millis(400));
    let body = json!({"jsonrpc": "2.0", "id": 9, "method": "ping"}).to_string();
    let ping = format!(
        "POST /mcp HTTP/1.1\r\nHost: {address}\r\nContent-Type: application/json\r\nAuthorization: {}\r\nContent-Length: {}\r\n\r\n{body}",
        server.authorization_header(),
        body.len()
    );
    stream.write_all(ping.as_bytes()).expect("ping writes");
    let response = read_response(&mut stream);
    assert!(response.contains("200 OK"), "{response}");
    assert!(response.contains("\"result\""), "{response}");
}

#[test]
fn disposable_mcp_server_drops_with_an_idle_sse_stream_open() {
    let server = DisposableHttpMcpServer::start();
    // Keep the client socket alive across the drop: the handler sits on a
    // silent, persistent stream.
    let (_stream, _address, response) = open_idle_sse_stream(&server);
    assert!(response.contains("text/event-stream"), "{response}");
    let started = std::time::Instant::now();
    drop(server);
    assert!(
        started.elapsed() < Duration::from_secs(5),
        "dropping the listener must join a handler waiting on an idle SSE stream"
    );
}

#[test]
fn harness_proves_declaration_connect_list_call_result_and_cleanup() {
    let server = DisposableHttpMcpServer::start();
    let host_id = ExecutionHostId::new("fixture.host.http-mcp-live").expect("valid host id");
    let selected = selection(host_id.clone(), "0.79.0");
    let host = FixtureHost::new(Scenario::HttpMcpHonour, "0.79.0");
    let services = host.services(host_id);
    let placement = ClaudeAgentAcpRemoteMcpPlacement::new(
        CLAUDE_AGENT_ACP_MCP_SERVER_NAME,
        server.endpoint(),
        vec![("Authorization".to_owned(), server.authorization_header())],
    );
    let driver = ClaudeAgentAcpDriver::new(
        swallowtail_runtime::EnvironmentRef::new("claude-agent.acp.fixture.isolated")
            .expect("valid environment"),
        selected.credential,
    )
    .with_http_mcp_placement(placement)
    .expect("route-owned http placement is admitted");
    let mut session = block_on(driver.open_session(
        selected.plan,
        open_request("claude-agent-http-mcp-live", selected.resource),
        services.clone(),
    ))
    .expect("session opens");
    let declaration_sent = host.writes().iter().any(|message| {
        message["method"] == "session/new"
            && message["params"]["mcpServers"][0]["type"] == "http"
            && message["params"]["mcpServers"][0]["name"] == CLAUDE_AGENT_ACP_MCP_SERVER_NAME
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
                RuntimeTurnId::new("claude-agent-http-mcp-live-turn").expect("valid turn"),
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
    assert_eq!(record.cleanup_class(), HttpMcpCleanupClass::Clean);
    assert_eq!(record.cleanup_diagnostic_code(), None);
    let debug_record = format!("{record:?}");
    assert!(
        !debug_record.contains(server.bearer()) && !debug_record.contains(server.endpoint()),
        "live record Debug must not carry the URL or bearer"
    );

    let record_dir = unique_record_test_dir();
    let record_path = record_dir.join("attempt.json");
    let mut printed = Vec::new();
    persist_and_print_record(&record, &record_path, &mut printed)
        .expect("fake-agent record persists and prints");
    let persisted = std::fs::read_to_string(&record_path).expect("persisted record is readable");
    let rendered = record.to_json_line();
    assert_eq!(persisted, format!("{rendered}\n"));
    assert_eq!(
        String::from_utf8(printed).expect("record output is UTF-8"),
        format!("CLAUDE_AGENT_ACP_HTTP_MCP_RECORD={rendered}\n")
    );
    let value: Value = serde_json::from_str(&persisted).expect("record is valid JSON");
    assert_eq!(value["accepted"], true);
    assert_eq!(value["stop"], Value::Null);
    assert_eq!(value["model"], "fixture");
    assert_eq!(value["terminal"]["status"], "completed");
    assert_eq!(value["terminal"]["code"], Value::Null);
    assert_eq!(value["cleanup"]["class"], "clean");
    assert_eq!(value["cleanup"]["code"], Value::Null);
    assert_eq!(value["cleanup"]["stage"], Value::Null);
    assert!(!persisted.contains(server.bearer()));
    assert!(!persisted.contains(server.endpoint()));
    std::fs::remove_dir_all(record_dir).expect("test record directory removes");
}

#[test]
fn gate_record_writer_fails_when_it_cannot_persist_the_record() {
    let blocker = unique_record_test_dir();
    std::fs::write(&blocker, b"not a directory").expect("file blocks record directory");
    let record_path = blocker.join("attempt.json");
    let record = HttpMcpLiveRecord::pre_attempt_stop(HttpMcpLiveStop::HostVersion, None);
    let mut printed = Vec::new();
    let result = persist_and_print_record(&record, &record_path, &mut printed);
    assert!(result.is_err(), "gate must fail when persistence fails");
    assert!(
        printed.is_empty(),
        "no record is printed before persistence"
    );
    assert!(!record_path.exists());
    std::fs::remove_file(blocker).expect("blocking file removes");
}

fn unique_record_test_dir() -> std::path::PathBuf {
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system clock follows UNIX epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "swallowtail-http-mcp-record-test-{}-{nonce}",
        std::process::id()
    ))
}

/// Runs the fake HTTP MCP honouring tuple to `Completed`, then closes the
/// session and returns the exact cleanup outcome the close produced.
fn honour_and_close(
    host: &FixtureHost,
    host_id: &ExecutionHostId,
    id: &str,
) -> (DisposableHttpMcpServer, TerminalStatus, CleanupOutcome) {
    let server = DisposableHttpMcpServer::start();
    let selected = selection(host_id.clone(), "0.79.0");
    let services = host.services(host_id.clone());
    let placement = ClaudeAgentAcpRemoteMcpPlacement::new(
        CLAUDE_AGENT_ACP_MCP_SERVER_NAME,
        server.endpoint(),
        vec![("Authorization".to_owned(), server.authorization_header())],
    );
    let driver = ClaudeAgentAcpDriver::new(
        swallowtail_runtime::EnvironmentRef::new("claude-agent.acp.fixture.isolated")
            .expect("valid environment"),
        selected.credential,
    )
    .with_http_mcp_placement(placement)
    .expect("route-owned http placement is admitted");
    let mut session = block_on(driver.open_session(
        selected.plan,
        open_request(id, selected.resource),
        services.clone(),
    ))
    .expect("session opens");
    let mut turn = block_on(
        session.start_turn(
            TurnRequest::new(
                RuntimeTurnId::new(format!("{id}-turn")).expect("valid turn"),
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
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    let cleanup = block_on(session.close(host.cleanup_request(), services));
    (server, terminal.status().clone(), cleanup)
}

#[test]
fn harness_keeps_the_cleanup_diagnostic_when_close_crosses_its_deadline() {
    let host_id =
        ExecutionHostId::new("fixture.host.http-mcp-live-deadline").expect("valid host id");
    let host = FixtureHost::new(Scenario::HttpMcpHonour, "0.79.0")
        .with_held_session_close_response()
        .with_deadline_waits([DeadlineWait::Expires]);
    let (server, terminal, cleanup) =
        honour_and_close(&host, &host_id, "claude-agent-http-mcp-deadline");
    assert_eq!(
        cleanup
            .diagnostic()
            .map(swallowtail_core::SafeDiagnostic::code),
        Some("swallowtail.session_cleanup.deadline_expired")
    );
    let record = HttpMcpLiveRecord::from_attempt(
        true,
        &server.transcript(),
        &terminal,
        cleanup,
        Some("fixture".to_owned()),
    );
    assert!(record.connected());
    assert!(record.tools_listed());
    assert!(record.tool_called());
    assert!(record.tool_result());
    assert!(record.terminal_completed());
    assert!(!record.accepted(), "{record:?}");
    assert_eq!(record.stop(), Some(HttpMcpLiveStop::CleanupFailed));
    assert_eq!(record.cleanup_class(), HttpMcpCleanupClass::Failed);
    assert_eq!(
        record.cleanup_diagnostic_code(),
        Some("swallowtail.session_cleanup.deadline_expired"),
        "the record must keep the typed cleanup diagnostic, not only the stop name"
    );
    let value: Value = serde_json::from_str(&record.to_json_line()).expect("record JSON parses");
    assert_eq!(value["accepted"], false);
    assert_eq!(value["stop"], "cleanup_failed");
    assert_eq!(value["terminal"]["status"], "completed");
    assert_eq!(value["cleanup"]["class"], "failed");
    assert_eq!(
        value["cleanup"]["code"],
        "swallowtail.session_cleanup.deadline_expired"
    );
    assert_eq!(value["cleanup"]["stage"], Value::Null);
}

#[test]
fn harness_keeps_the_cleanup_diagnostic_when_the_provider_rejects_close() {
    let host_id =
        ExecutionHostId::new("fixture.host.http-mcp-live-rejected").expect("valid host id");
    let host = FixtureHost::new(Scenario::HttpMcpHonour, "0.79.0").with_rejected_close_response();
    let (server, terminal, cleanup) =
        honour_and_close(&host, &host_id, "claude-agent-http-mcp-rejected");
    assert_eq!(
        cleanup
            .diagnostic()
            .map(swallowtail_core::SafeDiagnostic::code),
        Some("swallowtail.claude_agent.acp.request_rejected")
    );
    let record = HttpMcpLiveRecord::from_attempt(
        true,
        &server.transcript(),
        &terminal,
        cleanup,
        Some("fixture".to_owned()),
    );
    assert!(!record.accepted(), "{record:?}");
    assert_eq!(record.stop(), Some(HttpMcpLiveStop::CleanupFailed));
    assert_eq!(record.cleanup_class(), HttpMcpCleanupClass::Degraded);
    assert_eq!(
        record.cleanup_diagnostic_code(),
        Some("swallowtail.claude_agent.acp.request_rejected")
    );
    assert_eq!(record.cleanup_diagnostic_stage(), None);
    let debug_record = format!("{record:?}");
    assert!(
        !debug_record.contains("private provider close failure"),
        "the record must keep the code, never the provider message body"
    );
}
