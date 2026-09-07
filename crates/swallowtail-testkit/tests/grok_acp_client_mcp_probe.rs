use serde_json::Value;
use swallowtail_testkit::{
    ClientMcpOracleShape, ClientMcpVerdict, ECHO_MCP_SERVER_NAME, ECHO_MCP_TOOL, ECHO_PROMPT,
    EchoMcpTranscript, InconclusiveCause, echo_mcp_helper_is_live,
    grok_acp_client_mcp_fixture_probe, grok_acp_client_mcp_oracle_fixture_probe,
    grok_acp_client_mcp_verdict_from_frames, grok_acp_client_mcp_verdict_with_helper,
    grok_acp_echo_mcp_reply, grok_acp_echo_mcp_stdio_frame,
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
            assert_eq!(capsule.prompt(), ECHO_PROMPT);
            assert_eq!(capsule.to_json()["prompt"].as_str(), Some(ECHO_PROMPT));
            if expected == ClientMcpVerdict::AcceptsClientMcp {
                assert!(
                    capsule
                        .echo_mcp_methods()
                        .iter()
                        .any(|method| method == "tools/call"),
                    "accepts_client_mcp requires the echo MCP tools/call transcript"
                );
                assert!(capsule.client_mcp_admitted());
                assert!(capsule.client_mcp_tools_listed());
                assert!(capsule.prompt_turn_completed());
                assert_eq!(capsule.stop_reason(), Some("end_turn"));
                assert_eq!(
                    grok_acp_client_mcp_verdict_from_frames(capsule.frames()),
                    ClientMcpVerdict::Inconclusive,
                    "ACP frames alone cannot attribute an echo tool_call"
                );
            } else if expected == ClientMcpVerdict::IgnoresClientMcp {
                assert!(
                    !capsule.client_mcp_admitted(),
                    "ignores_client_mcp forbids echo MCP initialize"
                );
                assert!(!capsule.client_mcp_tools_listed());
                assert!(
                    capsule.echo_helper_live(),
                    "ignores_client_mcp requires proven echo helper liveness"
                );
                assert!(
                    !capsule
                        .echo_mcp_methods()
                        .iter()
                        .any(|method| method == "tools/call"),
                    "ignores_client_mcp forbids echo MCP tools/call"
                );
                assert!(capsule.prompt_turn_completed());
                assert_eq!(capsule.stop_reason(), Some("end_turn"));
                assert_eq!(
                    grok_acp_client_mcp_verdict_from_frames(capsule.frames()),
                    ClientMcpVerdict::Inconclusive,
                    "frames alone cannot prove helper liveness"
                );
                assert_eq!(
                    grok_acp_client_mcp_verdict_with_helper(
                        capsule.frames(),
                        &EchoMcpTranscript::default(),
                        true
                    ),
                    ClientMcpVerdict::IgnoresClientMcp
                );
            } else if expected == ClientMcpVerdict::Inconclusive {
                assert!(capsule.inconclusive_cause().is_some());
                assert_eq!(
                    grok_acp_client_mcp_verdict_from_frames(capsule.frames()),
                    expected
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
fn review_oracle_rejects_ignores_when_echo_initialize_is_present() {
    let admitted = grok_acp_client_mcp_oracle_fixture_probe(
        "1.0.5",
        ClientMcpOracleShape::AdmittedListedNotCalledCompleted,
    )
    .expect("admitted listed fixture");
    assert!(
        admitted
            .echo_mcp_methods()
            .iter()
            .any(|method| method == "initialize")
    );
    assert_ne!(admitted.verdict(), ClientMcpVerdict::IgnoresClientMcp);
    assert_eq!(admitted.verdict(), ClientMcpVerdict::Inconclusive);
    assert_eq!(
        admitted.inconclusive_cause(),
        Some(InconclusiveCause::TurnCompletedWithoutToolCall)
    );
}

#[test]
fn fake_acp_oracle_shapes_cover_admission_and_invocation() {
    for version in ["1.0.4", "1.0.5"] {
        let called = grok_acp_client_mcp_oracle_fixture_probe(
            version,
            ClientMcpOracleShape::AdmittedAndCalled,
        )
        .expect("admitted and called");
        assert_eq!(called.verdict(), ClientMcpVerdict::AcceptsClientMcp);
        assert!(called.client_mcp_admitted());
        assert!(called.client_mcp_tools_listed());

        let listed = grok_acp_client_mcp_oracle_fixture_probe(
            version,
            ClientMcpOracleShape::AdmittedListedNotCalledCompleted,
        )
        .expect("admitted listed not called");
        assert_eq!(listed.verdict(), ClientMcpVerdict::Inconclusive);
        assert_eq!(
            listed.inconclusive_cause(),
            Some(InconclusiveCause::TurnCompletedWithoutToolCall)
        );
        assert!(listed.client_mcp_admitted());
        assert!(listed.client_mcp_tools_listed());
        assert!(listed.prompt_turn_completed());
        assert_eq!(listed.stop_reason(), Some("end_turn"));

        let admitted = grok_acp_client_mcp_oracle_fixture_probe(
            version,
            ClientMcpOracleShape::AdmittedNotListed,
        )
        .expect("admitted not listed");
        assert_eq!(admitted.verdict(), ClientMcpVerdict::Inconclusive);
        assert!(admitted.client_mcp_admitted());
        assert!(!admitted.client_mcp_tools_listed());
        assert_ne!(admitted.verdict(), ClientMcpVerdict::IgnoresClientMcp);

        let ignored =
            grok_acp_client_mcp_oracle_fixture_probe(version, ClientMcpOracleShape::NoAdmission)
                .expect("no admission");
        assert_eq!(ignored.verdict(), ClientMcpVerdict::IgnoresClientMcp);
        assert!(ignored.echo_helper_live());
        assert!(!ignored.client_mcp_admitted());
        assert!(!ignored.client_mcp_tools_listed());
        assert!(ignored.prompt_turn_completed());

        let dead = grok_acp_client_mcp_oracle_fixture_probe(
            version,
            ClientMcpOracleShape::HelperUnspawnable,
        )
        .expect("helper unspawnable");
        assert_eq!(dead.verdict(), ClientMcpVerdict::Inconclusive);
        assert_eq!(
            dead.inconclusive_cause(),
            Some(InconclusiveCause::EchoLivenessUnproven)
        );
        assert!(!dead.echo_helper_live());
        assert!(!dead.client_mcp_admitted());
        assert!(dead.prompt_turn_completed());
        assert_ne!(dead.verdict(), ClientMcpVerdict::IgnoresClientMcp);
    }
}

#[test]
fn session_new_client_answer_gate_completes_the_session() {
    for version in ["1.0.4", "1.0.5"] {
        let capsule = grok_acp_client_mcp_oracle_fixture_probe(
            version,
            ClientMcpOracleShape::SessionNewAnswerRequired,
        )
        .expect("answer-gate fixture completes");
        assert_eq!(capsule.verdict(), ClientMcpVerdict::AcceptsClientMcp);
        assert!(capsule.prompt_turn_completed());
        assert_eq!(capsule.stop_reason(), Some("end_turn"));
        assert!(capsule.client_mcp_admitted());
        assert!(capsule.client_mcp_tools_listed());
        let request = capsule
            .frames()
            .iter()
            .find(|frame| {
                !frame.is_outbound() && frame.message()["method"] == "session/request_permission"
            })
            .expect("agent asked inside session/new");
        let request_id = request.message()["id"].clone();
        let answered = capsule.frames().iter().any(|frame| {
            frame.is_outbound()
                && frame.message()["id"] == request_id
                && frame.message().get("result").is_some()
        });
        assert!(
            answered,
            "the probe's answer must be recorded during session/new"
        );
        assert_no_credentials_or_paths(&capsule.to_json());
    }
}

#[test]
fn session_new_bound_exceeded_names_the_silence_after_recorded_answers() {
    let capsule = grok_acp_client_mcp_oracle_fixture_probe(
        "1.0.5",
        ClientMcpOracleShape::SessionNewSilentAfterAnswer,
    )
    .expect("silent-after-answer fixture");
    assert_eq!(capsule.verdict(), ClientMcpVerdict::Inconclusive);
    assert_eq!(
        capsule.inconclusive_cause(),
        Some(InconclusiveCause::SessionNewBoundExceeded)
    );
    let request = capsule
        .frames()
        .iter()
        .find(|frame| {
            !frame.is_outbound() && frame.message()["method"] == "session/request_permission"
        })
        .expect("agent asked inside session/new");
    let request_id = request.message()["id"].clone();
    assert!(
        capsule.frames().iter().any(|frame| {
            frame.is_outbound()
                && frame.message()["id"] == request_id
                && frame.message().get("result").is_some()
        }),
        "the recorded answer must be on the capsule"
    );
    assert_eq!(
        capsule.to_json()["inconclusive_cause"],
        serde_json::json!("session_new_bound_exceeded")
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
    use swallowtail_testkit::{
        ECHO_MCP_TRANSCRIPT_FLAG, create_echo_mcp_transcript, read_echo_mcp_transcript,
    };

    let home = std::env::temp_dir().join(format!(
        "swallowtail-echo-mcp-spawn-home-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&home);
    std::fs::create_dir_all(&home).expect("spawn home");
    let path = create_echo_mcp_transcript(&home).expect("exclusive transcript");
    let exe = grok_acp_echo_mcp_bin();
    assert!(
        exe.is_file(),
        "grok-acp-echo-mcp example was not built at {}",
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
    let _ = std::fs::remove_dir_all(&home);
    assert!(transcript.initialize());
    assert!(transcript.tools_call());
}

#[test]
fn echo_helper_liveness_self_check_distinguishes_healthy_from_dead() {
    let exe = grok_acp_echo_mcp_bin();
    assert!(
        exe.is_file(),
        "grok-acp-echo-mcp example was not built at {}",
        exe.display()
    );
    assert!(
        echo_mcp_helper_is_live(&exe),
        "healthy echo example must prove liveness"
    );
    assert!(!echo_mcp_helper_is_live(std::path::Path::new("/bin/false")));
    assert!(!echo_mcp_helper_is_live(std::path::Path::new("/bin/true")));
    assert!(!echo_mcp_helper_is_live(std::path::Path::new(
        "/no/such/swallowtail-echo-helper"
    )));
}

fn grok_acp_echo_mcp_bin() -> std::path::PathBuf {
    let mut path = std::env::current_exe().expect("test executable");
    path.pop();
    if path.ends_with("deps") {
        path.pop();
    }
    path.push("examples");
    path.push("grok-acp-echo-mcp");
    if !path.is_file() {
        let status = std::process::Command::new(env!("CARGO"))
            .args([
                "build",
                "--offline",
                "--locked",
                "-p",
                "swallowtail-testkit",
                "--example",
                "grok-acp-echo-mcp",
            ])
            .status()
            .expect("build grok-acp-echo-mcp example");
        assert!(
            status.success(),
            "cargo build --example grok-acp-echo-mcp failed"
        );
    }
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
