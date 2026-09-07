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
            if expected == ClientMcpVerdict::AcceptsClientMcp {
                assert!(
                    capsule
                        .echo_mcp_methods()
                        .iter()
                        .any(|method| method == "tools/call"),
                    "accepts_client_mcp requires the echo MCP tools/call transcript"
                );
                assert_eq!(
                    grok_acp_client_mcp_verdict_from_frames(capsule.frames()),
                    ClientMcpVerdict::Inconclusive,
                    "ACP frames alone cannot attribute an echo tool_call"
                );
            } else if expected == ClientMcpVerdict::IgnoresClientMcp {
                assert!(
                    capsule
                        .echo_mcp_methods()
                        .iter()
                        .any(|method| method == "initialize"),
                    "ignores_client_mcp requires echo MCP initialize as liveness"
                );
                assert!(
                    !capsule
                        .echo_mcp_methods()
                        .iter()
                        .any(|method| method == "tools/call"),
                    "ignores_client_mcp forbids echo MCP tools/call"
                );
                assert_eq!(
                    grok_acp_client_mcp_verdict_from_frames(capsule.frames()),
                    ClientMcpVerdict::Inconclusive,
                    "ACP frames alone cannot prove the echo server was reached"
                );
            } else {
                assert_eq!(
                    grok_acp_client_mcp_verdict_from_frames(capsule.frames()),
                    expected
                );
            }
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

#[test]
fn spawned_echo_server_records_transcript_from_args() {
    use std::io::{BufRead, BufReader, Write};
    use std::process::{Command, Stdio};
    use swallowtail_testkit::{ECHO_MCP_TRANSCRIPT_FLAG, read_echo_mcp_transcript};

    let path = std::env::temp_dir().join(format!(
        "swallowtail-echo-mcp-spawn-{}-{}.ndjson",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time")
            .as_nanos()
    ));
    let _ = std::fs::remove_file(&path);
    let exe = grok_acp_echo_mcp_bin();
    assert!(
        exe.is_file(),
        "grok-acp-echo-mcp was not built next to the test executable: {}",
        exe.display()
    );
    let mut child = Command::new(&exe)
        .args([ECHO_MCP_TRANSCRIPT_FLAG, path.to_str().expect("utf8 path")])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn grok-acp-echo-mcp");
    let mut stdin = child.stdin.take().expect("stdin");
    let mut reader = BufReader::new(child.stdout.take().expect("stdout"));
    for (id, method, params) in [
        (
            1,
            "initialize",
            serde_json::json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": {"name": "spawn-test"}
            }),
        ),
        (2, "tools/list", serde_json::json!({})),
        (
            3,
            "tools/call",
            serde_json::json!({"name": "echo", "arguments": {"text": "ping"}}),
        ),
    ] {
        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params
        });
        let frame = grok_acp_echo_mcp_stdio_frame(&request).expect("frame");
        stdin.write_all(&frame).expect("write");
        stdin.flush().expect("flush");
        let mut line = String::new();
        reader.read_line(&mut line).expect("reply");
        assert!(!line.is_empty(), "echo server closed before {method}");
    }
    drop(stdin);
    let status = child.wait().expect("wait");
    assert!(status.success());
    let transcript = read_echo_mcp_transcript(&path);
    let _ = std::fs::remove_file(&path);
    assert!(transcript.initialize());
    assert!(transcript.tools_call());
}

fn grok_acp_echo_mcp_bin() -> std::path::PathBuf {
    let mut path = std::env::current_exe().expect("test executable");
    path.pop();
    if path.ends_with("deps") {
        path.pop();
    }
    path.push("grok-acp-echo-mcp");
    path
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
