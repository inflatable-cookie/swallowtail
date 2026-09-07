use serde_json::Value;
use swallowtail_testkit::{
    ClientMcpVerdict, ECHO_MCP_SERVER_NAME, ECHO_MCP_TOOL, grok_acp_client_mcp_fixture_probe,
    grok_acp_client_mcp_verdict_from_frames, grok_acp_echo_mcp_reply,
    grok_acp_echo_mcp_stdio_frame,
};

#[test]
fn fake_acp_fixture_proves_all_four_verdicts_for_each_exact_segment() {
    for version in ["1.0.4", "1.0.5"] {
        for expected in [
            ClientMcpVerdict::AcceptsClientMcp,
            ClientMcpVerdict::IgnoresClientMcp,
            ClientMcpVerdict::RejectsClientMcp,
            ClientMcpVerdict::Inconclusive,
        ] {
            let capsule = grok_acp_client_mcp_fixture_probe(version, expected)
                .expect("fixture probe completes");
            assert_eq!(capsule.version(), version);
            assert_eq!(capsule.verdict(), expected);
            assert_eq!(
                grok_acp_client_mcp_verdict_from_frames(capsule.frames()),
                expected
            );
            assert!(capsule.cleanup().joined());
            assert!(capsule.stale_callback_rejected());
            assert_eq!(
                capsule.to_json()["verdict"].as_str(),
                Some(expected.as_str())
            );
            if expected != ClientMcpVerdict::Inconclusive {
                assert!(
                    capsule.frames().iter().any(|frame| frame.is_outbound()
                        && frame.message()["method"] == "session/new"
                        && frame.message()["params"]["mcpServers"]
                            .as_array()
                            .is_some_and(|servers| !servers.is_empty())),
                    "non-inconclusive verdicts require the session/new frame"
                );
            }
            assert_no_credentials_or_paths(&capsule.to_json());
            assert!(!capsule.truncated());
        }
    }
}

#[test]
fn review_oracle_rejects_a_verdict_without_session_new() {
    let capsule = grok_acp_client_mcp_fixture_probe("1.0.5", ClientMcpVerdict::AcceptsClientMcp)
        .expect("accepts fixture");
    let without_session_new: Vec<_> = capsule
        .frames()
        .iter()
        .filter(|frame| frame.message()["method"] != "session/new")
        .cloned()
        .collect();
    assert_eq!(
        grok_acp_client_mcp_verdict_from_frames(&without_session_new),
        ClientMcpVerdict::Inconclusive
    );
}

#[test]
fn disposable_echo_server_has_one_tool_and_no_resource_authority() {
    let initialize = grok_acp_echo_mcp_reply(&serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {"protocolVersion": "2024-11-05", "capabilities": {}, "clientInfo": {"name": "fixture"}}
    }))
    .expect("initialize");
    assert_eq!(
        initialize["result"]["serverInfo"]["name"],
        ECHO_MCP_SERVER_NAME
    );
    let tools = grok_acp_echo_mcp_reply(&serde_json::json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/list"
    }))
    .expect("tools/list");
    let listed = tools["result"]["tools"].as_array().expect("tools array");
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0]["name"], ECHO_MCP_TOOL);
    for method in ["resources/list", "resources/read", "roots/list"] {
        let reply = grok_acp_echo_mcp_reply(&serde_json::json!({
            "jsonrpc": "2.0",
            "id": 3,
            "method": method
        }))
        .expect("method error");
        assert_eq!(reply["error"]["code"], serde_json::json!(-32601));
    }
    let framed = grok_acp_echo_mcp_stdio_frame(&initialize).expect("stdio frame");
    let text = std::str::from_utf8(&framed).expect("utf8");
    assert!(text.ends_with('\n'));
    assert_eq!(text.bytes().filter(|byte| *byte == b'\n').count(), 1);
    assert!(!text.to_ascii_lowercase().contains("content-length"));
}

fn assert_no_credentials_or_paths(value: &Value) {
    match value {
        Value::String(text) => {
            assert!(!text.starts_with('/'), "capsule leaked path {text}");
            assert!(!text.contains("Bearer "), "capsule leaked credential");
            assert!(!text.contains("secret-value"), "capsule leaked secret");
        }
        Value::Array(items) => items.iter().for_each(assert_no_credentials_or_paths),
        Value::Object(map) => {
            for (key, child) in map {
                let lower = key.to_ascii_lowercase();
                if lower.contains("token") || lower.contains("secret") || key == "path" {
                    assert_eq!(child, &Value::String("<redacted>".to_owned()));
                } else {
                    assert_no_credentials_or_paths(child);
                }
            }
        }
        _ => {}
    }
}
