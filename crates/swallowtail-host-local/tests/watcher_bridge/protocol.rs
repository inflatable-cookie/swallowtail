use super::http::post_json;
use super::support::{
    close_lease, default_host, handshake, open_lease, tool_call, tool_payload, tools_list_body,
    watcher_host,
};
use serde_json::json;
use swallowtail_runtime::{
    WATCHER_BRIDGE_INITIALIZED_NOTIFICATION, WATCHER_BRIDGE_JSONRPC_VERSION,
    WATCHER_BRIDGE_MCP_PROTOCOL_VERSION, WATCHER_BRIDGE_TOOL_COMPLETION_GATE,
    WATCHER_BRIDGE_TOOL_INSPECT, WATCHER_BRIDGE_TOOL_LIST, WATCHER_BRIDGE_TOOL_START,
    WATCHER_BRIDGE_TOOL_STOP, WATCHER_BRIDGE_TOOL_WAIT, WATCHER_BRIDGE_TOOLS_CALL_METHOD,
};

#[test]
fn malformed_ids_fail_before_watcher_work() {
    let local = watcher_host("malformed-id", "sleep");
    let lease = open_lease(&local, "turn-id");
    let endpoint = lease.endpoint().expose().to_owned();
    let bearer = lease.bearer().expose().to_owned();
    handshake(&endpoint, &bearer);

    for id in [
        json!(null),
        json!(true),
        json!([]),
        json!({"k": "v"}),
        json!(""),
    ] {
        let (status, body) = post_json(
            &endpoint,
            Some(&bearer),
            &tool_call(
                id,
                WATCHER_BRIDGE_TOOL_START,
                json!({"operation_data": "sleep-operation"}),
            ),
        );
        assert_eq!(status, 400, "{body}");
        assert!(body.contains("malformed") || body.contains("Bad Request"));
    }

    let missing_id = json!({
        "jsonrpc": WATCHER_BRIDGE_JSONRPC_VERSION,
        "method": WATCHER_BRIDGE_TOOLS_CALL_METHOD,
        "params": { "name": WATCHER_BRIDGE_TOOL_START, "arguments": { "operation_data": "sleep-operation" } }
    })
    .to_string();
    let (status, body) = post_json(&endpoint, Some(&bearer), &missing_id);
    assert_eq!(status, 400, "{body}");

    let initialized_with_id = json!({
        "jsonrpc": WATCHER_BRIDGE_JSONRPC_VERSION,
        "id": 9,
        "method": WATCHER_BRIDGE_INITIALIZED_NOTIFICATION
    })
    .to_string();
    let (status, body) = post_json(&endpoint, Some(&bearer), &initialized_with_id);
    assert_eq!(status, 400, "{body}");

    let listed = futures_executor::block_on(
        local
            .services()
            .watcher()
            .expect("watcher")
            .list(swallowtail_core::WatcherOwningTurn::new("turn-id").expect("owning turn")),
    );
    match listed {
        Ok(listed) => assert!(listed.is_empty()),
        Err(error) => assert_eq!(
            error.diagnostic().code(),
            "swallowtail.local_watcher.turn_not_found"
        ),
    }
    close_lease(&local, lease);
}

#[test]
fn tools_list_schemas_match_required_fields() {
    let local = default_host("schemas");
    let lease = open_lease(&local, "turn-schema");
    let endpoint = lease.endpoint().expose().to_owned();
    let bearer = lease.bearer().expose().to_owned();
    handshake(&endpoint, &bearer);
    let (status, body) = post_json(&endpoint, Some(&bearer), &tools_list_body(2));
    assert_eq!(status, 200);
    let tools = serde_json::from_str::<serde_json::Value>(&body).expect("json")["result"]["tools"]
        .as_array()
        .expect("tools")
        .clone();
    let required = |name: &str| {
        tools
            .iter()
            .find(|tool| tool["name"] == name)
            .and_then(|tool| tool["inputSchema"]["required"].as_array())
            .expect("schema")
            .iter()
            .map(|value| value.as_str().expect("field").to_owned())
            .collect::<Vec<_>>()
    };
    assert_eq!(required(WATCHER_BRIDGE_TOOL_START), ["operation_data"]);
    assert_eq!(required(WATCHER_BRIDGE_TOOL_INSPECT), ["watcher_id"]);
    assert_eq!(required(WATCHER_BRIDGE_TOOL_WAIT), ["watcher_id"]);
    assert_eq!(required(WATCHER_BRIDGE_TOOL_STOP), ["watcher_id"]);
    assert!(required(WATCHER_BRIDGE_TOOL_LIST).is_empty());
    assert!(required(WATCHER_BRIDGE_TOOL_COMPLETION_GATE).is_empty());
    let names = tools
        .iter()
        .map(|tool| tool["name"].as_str().expect("name").to_owned())
        .collect::<Vec<_>>();
    assert_eq!(
        names,
        [
            WATCHER_BRIDGE_TOOL_START,
            WATCHER_BRIDGE_TOOL_INSPECT,
            WATCHER_BRIDGE_TOOL_LIST,
            WATCHER_BRIDGE_TOOL_WAIT,
            WATCHER_BRIDGE_TOOL_STOP,
            WATCHER_BRIDGE_TOOL_COMPLETION_GATE,
        ]
    );
    for tool in &tools {
        assert_eq!(tool["inputSchema"]["additionalProperties"], false);
        assert_eq!(tool["inputSchema"]["type"], "object");
    }
    close_lease(&local, lease);
}

#[test]
fn wait_payload_includes_terminal_cause_and_snapshot() {
    let local = watcher_host("wait-payload", "exit-zero");
    let lease = open_lease(&local, "turn-wait");
    let endpoint = lease.endpoint().expose().to_owned();
    let bearer = lease.bearer().expose().to_owned();
    handshake(&endpoint, &bearer);
    let (status, body) = post_json(
        &endpoint,
        Some(&bearer),
        &tool_call(
            json!(2),
            WATCHER_BRIDGE_TOOL_START,
            json!({"operation_data": "exit-zero-operation"}),
        ),
    );
    assert_eq!(status, 200);
    let watcher_id = tool_payload(&body)["watcher_id"]
        .as_str()
        .expect("id")
        .to_owned();
    let (status, body) = post_json(
        &endpoint,
        Some(&bearer),
        &tool_call(
            json!(3),
            WATCHER_BRIDGE_TOOL_WAIT,
            json!({"watcher_id": watcher_id}),
        ),
    );
    assert_eq!(status, 200);
    let payload = tool_payload(&body);
    assert_eq!(payload["wait"], "satisfied");
    assert_eq!(payload["terminal_cause"], "completed");
    assert_eq!(payload["watcher"]["watcher_id"], watcher_id);
    assert_eq!(payload["watcher"]["phase"], "joined");
    close_lease(&local, lease);
}

#[test]
fn tools_before_initialize_fail_closed() {
    let local = default_host("handshake");
    let lease = open_lease(&local, "turn-handshake");
    let endpoint = lease.endpoint().expose().to_owned();
    let bearer = lease.bearer().expose().to_owned();
    let (status, body) = post_json(&endpoint, Some(&bearer), &tools_list_body(1));
    assert_eq!(status, 200);
    assert!(
        body.contains("initialization") || body.contains("handshake"),
        "{body}"
    );
    close_lease(&local, lease);
}

#[test]
fn error_responses_echo_valid_numeric_and_string_ids() {
    let local = default_host("echo-id");
    let lease = open_lease(&local, "turn-echo");
    let endpoint = lease.endpoint().expose().to_owned();
    let bearer = lease.bearer().expose().to_owned();
    let (status, body) = post_json(&endpoint, Some(&bearer), &tools_list_body(7));
    assert_eq!(status, 200, "{body}");
    let response: serde_json::Value = serde_json::from_str(&body).expect("json");
    assert_eq!(response["id"], json!(7));
    assert_ne!(response["id"], json!(null));

    handshake(&endpoint, &bearer);
    let (status, body) = post_json(
        &endpoint,
        Some(&bearer),
        &json!({
            "jsonrpc": WATCHER_BRIDGE_JSONRPC_VERSION,
            "id": "abc",
            "method": WATCHER_BRIDGE_TOOLS_CALL_METHOD,
            "params": { "name": "shell", "arguments": {} }
        })
        .to_string(),
    );
    assert_eq!(status, 200, "{body}");
    let response: serde_json::Value = serde_json::from_str(&body).expect("json");
    assert_eq!(response["id"], json!("abc"));
    close_lease(&local, lease);
}

#[test]
fn unknown_and_oversized_requests_fail_closed() {
    let local = default_host("malformed");
    let lease = open_lease(&local, "turn-malformed");
    let endpoint = lease.endpoint().expose().to_owned();
    let bearer = lease.bearer().expose().to_owned();
    let (status, _) = post_json(&endpoint, Some(&bearer), "{not-json");
    assert_eq!(status, 400);
    handshake(&endpoint, &bearer);
    let (status, _) = post_json(
        &endpoint,
        Some(&bearer),
        &json!({
            "jsonrpc": WATCHER_BRIDGE_JSONRPC_VERSION,
            "id": 2,
            "method": "tools/call",
            "params": { "name": "shell", "arguments": {} }
        })
        .to_string(),
    );
    assert_eq!(status, 200);
    let oversized = format!(
        "{{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"initialize\",\"params\":{{\"protocolVersion\":\"{}\",\"pad\":\"{}\"}}}}",
        WATCHER_BRIDGE_MCP_PROTOCOL_VERSION,
        "x".repeat(20_000)
    );
    let (status, _) = post_json(&endpoint, Some(&bearer), &oversized);
    assert_eq!(status, 413);
    close_lease(&local, lease);
}

fn tool_text(body: &str) -> String {
    serde_json::from_str::<serde_json::Value>(body).expect("json")["result"]["content"][0]["text"]
        .as_str()
        .expect("tool text")
        .to_owned()
}

#[test]
fn completion_gate_tool_text_blocks_stop_while_work_remains() {
    let local = watcher_host("stop-decision", "sleep");
    let lease = open_lease(&local, "turn-stop-decision");
    let endpoint = lease.endpoint().expose().to_owned();
    let bearer = lease.bearer().expose().to_owned();
    handshake(&endpoint, &bearer);
    let (status, body) = post_json(
        &endpoint,
        Some(&bearer),
        &tool_call(
            json!(2),
            WATCHER_BRIDGE_TOOL_START,
            json!({"operation_data": "sleep-operation"}),
        ),
    );
    assert_eq!(status, 200, "{body}");
    let watcher_id = tool_payload(&body)["watcher_id"]
        .as_str()
        .expect("id")
        .to_owned();
    let (status, body) = post_json(
        &endpoint,
        Some(&bearer),
        &tool_call(json!(3), WATCHER_BRIDGE_TOOL_COMPLETION_GATE, json!({})),
    );
    assert_eq!(status, 200, "{body}");
    let text = tool_text(&body);
    assert!(
        text.contains("\"decision\":\"block\""),
        "raw Stop tool text must block: {text}"
    );
    let payload = tool_payload(&body);
    assert_eq!(payload["decision"], "block");
    assert_eq!(payload["allows_successful_completion"], false);
    let reason = payload["reason"].as_str().expect("reason");
    assert!(reason.contains("active or unjoined"));
    assert!(!reason.contains(&bearer));
    assert!(!reason.contains(&endpoint));
    assert!(!text.contains(&bearer));
    let (status, body) = post_json(
        &endpoint,
        Some(&bearer),
        &tool_call(
            json!(4),
            WATCHER_BRIDGE_TOOL_STOP,
            json!({"watcher_id": watcher_id}),
        ),
    );
    assert_eq!(status, 200, "{body}");
    let (status, body) = post_json(
        &endpoint,
        Some(&bearer),
        &tool_call(
            json!(5),
            WATCHER_BRIDGE_TOOL_WAIT,
            json!({"watcher_id": watcher_id}),
        ),
    );
    assert_eq!(status, 200, "{body}");
    let (status, body) = post_json(
        &endpoint,
        Some(&bearer),
        &tool_call(json!(6), WATCHER_BRIDGE_TOOL_COMPLETION_GATE, json!({})),
    );
    assert_eq!(status, 200, "{body}");
    let idle = tool_text(&body);
    assert!(
        !idle.contains("\"decision\""),
        "idle Stop tool text must omit decision: {idle}"
    );
    let payload = tool_payload(&body);
    assert!(payload.get("decision").is_none());
    assert_eq!(payload["allows_successful_completion"], true);
    close_lease(&local, lease);
}
