const HTTP_CANARY_URL: &str = "http://127.0.0.1:9/mcp/g06-033-redaction-canary";
const HTTP_CANARY_HEADER: &str = "Bearer g06-033-redaction-canary";

fn http_session_input(request: &str) -> ClaudeAgentSessionProfileInput {
    ClaudeAgentSessionProfileInput::new(
        RequestId::new(request).expect("valid request"),
        ClaudeAgentModelSelection::new(
            ModelRouteId::new("claude-agent.prepared.route").expect("valid route"),
            ModelRouteRevision::new("1").expect("valid route revision"),
            ModelId::new("claude-sonnet-4-6").expect("valid model"),
        ),
        WorkingResourceRef::new("claude-agent.prepared.workspace").expect("valid resource"),
        SessionOptions::default(),
    )
}

fn http_placement() -> swallowtail_adapter_claude_agent::ClaudeAgentAcpRemoteMcpPlacement {
    swallowtail_adapter_claude_agent::ClaudeAgentAcpRemoteMcpPlacement::new(
        swallowtail_adapter_claude_agent::CLAUDE_AGENT_ACP_MCP_SERVER_NAME,
        HTTP_CANARY_URL,
        vec![("Authorization".to_owned(), HTTP_CANARY_HEADER.to_owned())],
    )
}

fn assert_http_secrets_redacted(value: &impl std::fmt::Debug) {
    let rendered = format!("{value:?}");
    assert!(
        !rendered.contains(HTTP_CANARY_URL),
        "debug leaked the URL: {rendered}"
    );
    assert!(
        !rendered.contains(HTTP_CANARY_HEADER),
        "debug leaked a header value: {rendered}"
    );
}

#[test]
fn prepared_session_omission_keeps_empty_mcp_servers() {
    let host_id = ExecutionHostId::new("fixture.prepared.http-mcp-omit").expect("valid host");
    let preparation_host = FixtureHost::new(Scenario::Version, "0.79.0");
    let prepared = block_on(prepare_claude_agent(
        preparation_input(host_id.clone()),
        probe(),
        preparation_host.services(host_id.clone()),
    ))
    .expect("Claude Agent prepares");
    let profile = prepared
        .prepare_session(http_session_input("http-mcp-omit"))
        .expect("session prepares");
    let operation_host = FixtureHost::new(Scenario::Success, "0.79.0");
    let session = block_on(profile.open_session(operation_host.services(host_id.clone())))
        .expect("prepared session opens");
    assert!(operation_host.writes().iter().any(|message| {
        message["method"] == "session/new" && message["params"]["mcpServers"] == serde_json::json!([])
    }));
    assert_eq!(
        block_on(session.close(
            operation_host.cleanup_request(),
            operation_host.services(host_id),
        )),
        CleanupOutcome::Clean
    );
}

#[test]
fn prepared_session_can_bind_the_admitted_http_mcp_placement() {
    let host_id = ExecutionHostId::new("fixture.prepared.http-mcp").expect("valid host");
    let preparation_host = FixtureHost::new(Scenario::Version, "0.79.0");
    let prepared = block_on(prepare_claude_agent(
        preparation_input(host_id.clone()),
        probe(),
        preparation_host.services(host_id.clone()),
    ))
    .expect("Claude Agent prepares");
    let profile = prepared
        .prepare_session(http_session_input("http-mcp").with_http_mcp_placement(http_placement()))
        .expect("session prepares");
    assert_http_secrets_redacted(&profile);
    assert_http_secrets_redacted(profile.plan());
    assert_http_secrets_redacted(profile.evidence());
    let contribution = profile
        .consumer_route_projection_contribution(
            swallowtail_runtime::ConsumerRouteProjectionSourceId::new(
                "claude-agent.acp.prepared-session",
            )
            .expect("source"),
        )
        .expect("projects");
    assert_http_secrets_redacted(&contribution);
    let placement = contribution
        .selection_rows()
        .find(|row| {
            row.identity()
                .namespaced_extension()
                .is_some_and(|extension| extension.semantic_id() == "mcp.placement")
        })
        .expect("HTTP placement is named");
    let values = match placement
        .control_value()
        .expect("placement names values")
        .domain()
    {
        swallowtail_runtime::ConsumerRouteValueDomain::Enumerated(values) => values,
        other => panic!("placement must be enumerated, got {other:?}"),
    };
    let texts: Vec<&str> = values.values().map(|value| value.as_str()).collect();
    assert!(texts.contains(&swallowtail_adapter_claude_agent::CLAUDE_AGENT_ACP_HTTP_MCP_PLACEMENT));
    assert!(texts.contains(&swallowtail_adapter_claude_agent::CLAUDE_AGENT_ACP_MCP_SERVER_NAME));
    assert!(
        !texts
            .iter()
            .any(|value| value.contains("g06-033-redaction-canary"))
    );
    let operation_host = FixtureHost::new(Scenario::Success, "0.79.0");
    let session = block_on(profile.open_session(operation_host.services(host_id.clone())))
        .expect("prepared session opens");
    assert!(operation_host.writes().iter().any(|message| {
        message["method"] == "session/new"
            && message["params"]["mcpServers"][0]["type"] == "http"
            && message["params"]["mcpServers"][0]["name"]
                == swallowtail_adapter_claude_agent::CLAUDE_AGENT_ACP_MCP_SERVER_NAME
            && message["params"]["mcpServers"][0]["url"] == HTTP_CANARY_URL
            && message["params"]["mcpServers"][0]["headers"][0]["value"] == HTTP_CANARY_HEADER
    }));
    assert_eq!(
        block_on(session.close(
            operation_host.cleanup_request(),
            operation_host.services(host_id),
        )),
        CleanupOutcome::Clean
    );
}

#[test]
fn prepared_session_refuses_sse_and_invalid_http_before_session_new() {
    let host_id = ExecutionHostId::new("fixture.prepared.http-mcp-refuse").expect("valid host");
    let preparation_host = FixtureHost::new(Scenario::Version, "0.79.0");
    let prepared = block_on(prepare_claude_agent(
        preparation_input(host_id.clone()),
        probe(),
        preparation_host.services(host_id.clone()),
    ))
    .expect("Claude Agent prepares");

    let sse = prepared
        .prepare_session(http_session_input("http-mcp-sse").with_http_mcp_placement(
            swallowtail_adapter_claude_agent::ClaudeAgentAcpRemoteMcpPlacement::sse(
                swallowtail_adapter_claude_agent::CLAUDE_AGENT_ACP_MCP_SERVER_NAME,
                HTTP_CANARY_URL,
                vec![("Authorization".to_owned(), HTTP_CANARY_HEADER.to_owned())],
            ),
        ))
        .expect("input can name sse before open");
    let operation_host = FixtureHost::new(Scenario::Success, "0.79.0");
    let error = match block_on(sse.open_session(operation_host.services(host_id.clone()))) {
        Err(error) => error,
        Ok(_) => panic!("sse must refuse"),
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.claude_agent.acp.mcp_sse_not_emitted"
    );
    assert_http_secrets_redacted(&error);
    assert!(
        !operation_host
            .writes()
            .iter()
            .any(|message| message["method"] == "session/new")
    );

    let invalid = prepared
        .prepare_session(http_session_input("http-mcp-invalid").with_http_mcp_placement(
            swallowtail_adapter_claude_agent::ClaudeAgentAcpRemoteMcpPlacement::new(
                swallowtail_adapter_claude_agent::CLAUDE_AGENT_ACP_MCP_SERVER_NAME,
                "not-a-url",
                Vec::<(String, String)>::new(),
            ),
        ))
        .expect("input can name an invalid URL before open");
    let error = match block_on(invalid.open_session(operation_host.services(host_id))) {
        Err(error) => error,
        Ok(_) => panic!("invalid HTTP must refuse"),
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.claude_agent.acp.mcp_http_invalid"
    );
    assert!(!error.diagnostic().message().contains("not-a-url"));
}
