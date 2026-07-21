mod support;

use serde_json::json;
use support::{
    Direction, MAX_FRAME_BYTES, MAX_TRANSCRIPT_BYTES, MAX_TRANSCRIPT_FRAMES, ParseError, methods,
    parse_json, parse_transcript,
};
use swallowtail_protocol_acp::ACP_PROTOCOL_VERSION;

const ROOT: &str = "fixtures/acp-v1-gemini-cli-0.51.0";
const PROTOCOL: &str = include_str!("fixtures/acp-v1-gemini-cli-0.51.0/protocol.json");
const INITIALIZE: &str = include_str!("fixtures/acp-v1-gemini-cli-0.51.0/initialize.ndjson");
const VERSION_MISMATCH: &str =
    include_str!("fixtures/acp-v1-gemini-cli-0.51.0/version-mismatch.ndjson");
const NEW_SESSION: &str = include_str!("fixtures/acp-v1-gemini-cli-0.51.0/new-session.ndjson");
const PROMPT: &str = include_str!("fixtures/acp-v1-gemini-cli-0.51.0/prompt-success.ndjson");
const PERMISSION_CANCEL: &str =
    include_str!("fixtures/acp-v1-gemini-cli-0.51.0/permission-cancel.ndjson");
const FILESYSTEM: &str = include_str!("fixtures/acp-v1-gemini-cli-0.51.0/filesystem.ndjson");
const EXTENSIONS: &str = include_str!("fixtures/acp-v1-gemini-cli-0.51.0/extensions.ndjson");
const AUTH_REQUIRED: &str = include_str!("fixtures/acp-v1-gemini-cli-0.51.0/auth-required.ndjson");
const DISCONNECT: &str = include_str!("fixtures/acp-v1-gemini-cli-0.51.0/disconnect.ndjson");

#[test]
fn manifest_pins_wire_schema_gemini_and_access_authority() {
    let fixture = parse_json(PROTOCOL);
    assert_eq!(fixture["fixture_schema"], 1);
    assert_eq!(fixture["protocol"]["wire_version"], ACP_PROTOCOL_VERSION);
    assert_eq!(fixture["protocol"]["schema_artifact"], "schema-v1.19.0");
    assert_eq!(fixture["protocol"]["stable_schema_only"], true);
    assert_eq!(fixture["protocol"]["transport"], "stdio_ndjson");
    assert_eq!(fixture["gemini"]["version"], "0.51.0");
    assert_eq!(fixture["gemini"]["acp_sdk"], "0.16.1");
    assert_eq!(
        fixture["gemini"]["command"],
        json!(["gemini", "--acp", "--approval-mode", "plan"])
    );
    assert_eq!(fixture["access"]["authenticate_method_in_subset"], false);
    assert_eq!(fixture["access"]["consumer_membership_in_subset"], false);
    assert_eq!(fixture["access"]["ambient_config_allowed"], false);
    assert_eq!(fixture["access"]["implicit_auth_fallback_allowed"], false);
}

#[test]
fn initialization_negotiates_version_and_treats_omission_as_unsupported() {
    let frames = parse_transcript(INITIALIZE).expect("initialization transcript parses");
    assert_eq!(frames.len(), 2);
    assert_eq!(frames[0].direction(), Direction::ClientToAgent);
    assert_eq!(frames[0].method(), Some("initialize"));
    assert_eq!(frames[0].message()["params"]["protocolVersion"], 1);
    assert_eq!(
        frames[0].message()["params"]["clientCapabilities"]["fs"]["readTextFile"],
        true
    );
    assert_eq!(
        frames[0].message()["params"]["clientCapabilities"]["fs"]["writeTextFile"],
        false
    );
    assert!(
        frames[0].message()["params"]["clientCapabilities"]
            .get("terminal")
            .is_none()
    );
    assert_eq!(frames[1].message()["result"]["protocolVersion"], 1);
    assert_eq!(
        frames[1].message()["result"]["agentCapabilities"]["loadSession"],
        true
    );

    let mismatch = parse_transcript(VERSION_MISMATCH).expect("mismatch transcript parses");
    assert_ne!(
        mismatch[1].message()["result"]["protocolVersion"],
        ACP_PROTOCOL_VERSION
    );
    assert_eq!(
        parse_json(PROTOCOL)["negotiation"]["version_mismatch"],
        "close_and_fail"
    );
}

#[test]
fn first_session_subset_is_new_only_and_keeps_modes_and_models_observational() {
    let frames = parse_transcript(NEW_SESSION).expect("new session transcript parses");
    assert_eq!(methods(&frames), ["session/new"]);
    assert_eq!(frames[0].message()["params"]["cwd"], "/fixture/workspace");
    assert_eq!(frames[0].message()["params"]["mcpServers"], json!([]));
    assert_eq!(
        frames[1].message()["result"]["sessionId"],
        "gemini-session-fixture"
    );
    assert_eq!(
        frames[1].message()["result"]["modes"]["currentModeId"],
        "plan"
    );

    let protocol = parse_json(PROTOCOL);
    assert_eq!(
        protocol["gemini_subset"]["load_session"],
        "observed_not_claimed"
    );
    assert_eq!(protocol["gemini_subset"]["set_mode"], "not_called");
    assert_eq!(
        protocol["gemini_subset"]["set_model"],
        "unstable_not_called"
    );
    assert_eq!(
        protocol["gemini_subset"]["session_close"],
        "not_advertised_process_close_only"
    );
}

#[test]
fn prompt_updates_are_correlated_and_finish_with_one_stop_reason() {
    let frames = parse_transcript(PROMPT).expect("prompt transcript parses");
    assert_eq!(
        methods(&frames),
        ["session/prompt", "session/update", "session/update"]
    );
    for frame in frames
        .iter()
        .filter(|frame| frame.method() == Some("session/update"))
    {
        assert_eq!(
            frame.message()["params"]["sessionId"],
            "gemini-session-fixture"
        );
    }
    let output = frames
        .iter()
        .filter(|frame| frame.method() == Some("session/update"))
        .filter_map(|frame| frame.message()["params"]["update"]["content"]["text"].as_str())
        .collect::<String>();
    assert_eq!(output, "Read-only fixture response.");
    assert_eq!(
        frames.last().expect("response exists").message()["result"]["stopReason"],
        "end_turn"
    );
}

#[test]
fn permission_wait_is_cancelled_without_granting_consumer_authority() {
    let frames = parse_transcript(PERMISSION_CANCEL).expect("permission transcript parses");
    assert_eq!(
        methods(&frames),
        [
            "session/prompt",
            "session/update",
            "session/request_permission",
            "session/cancel"
        ]
    );
    let permission = frames
        .iter()
        .find(|frame| frame.method() == Some("session/request_permission"))
        .expect("permission request exists");
    assert_eq!(permission.direction(), Direction::AgentToClient);
    let response = frames
        .iter()
        .find(|frame| frame.id() == permission.id() && frame.method().is_none())
        .expect("permission response is correlated");
    assert_eq!(
        response.message()["result"]["outcome"]["outcome"],
        "cancelled"
    );
    assert_eq!(
        frames.last().expect("prompt response exists").message()["result"]["stopReason"],
        "cancelled"
    );
    assert_eq!(
        parse_json(PROTOCOL)["permission_policy"]["allow_option_selected"],
        false
    );
}

#[test]
fn filesystem_read_is_capability_and_workspace_bound_while_write_and_terminal_stay_absent() {
    let frames = parse_transcript(FILESYSTEM).expect("filesystem transcript parses");
    assert_eq!(methods(&frames), ["fs/read_text_file"]);
    assert_eq!(frames[0].direction(), Direction::AgentToClient);
    assert_eq!(
        frames[0].message()["params"]["path"],
        "/fixture/workspace/src/lib.rs"
    );
    assert_eq!(frames[0].message()["params"]["limit"], 32);
    assert_eq!(
        frames[1].message()["result"]["content"],
        "pub fn fixture() {}\n"
    );

    let callback = &parse_json(PROTOCOL)["client_callbacks"];
    assert_eq!(callback["fs_read"], "host_bounded_working_resource_io");
    assert_eq!(callback["fs_write"], "not_advertised");
    assert_eq!(callback["terminal"], "not_advertised");
    assert_eq!(callback["path_escape"], "reject_before_io");
}

#[test]
fn extensions_and_auth_failure_remain_explicit_and_redacted() {
    let frames = parse_transcript(EXTENSIONS).expect("extension transcript parses");
    assert_eq!(methods(&frames), ["_gemini/private", "_gemini/notice"]);
    let response = frames
        .iter()
        .find(|frame| frame.method().is_none())
        .expect("unknown request receives a response");
    assert_eq!(response.message()["error"]["code"], -32601);
    assert_eq!(
        parse_json(PROTOCOL)["extensions"]["unknown_notification"],
        "ignore"
    );

    let auth = parse_transcript(AUTH_REQUIRED).expect("auth error parses");
    assert_eq!(auth[1].message()["error"]["code"], -32000);
    assert!(!AUTH_REQUIRED.contains("api-key-value"));
    assert!(!AUTH_REQUIRED.contains("oauth-token"));
}

#[test]
fn framing_is_bounded_and_disconnects_fail_before_partial_payload_use() {
    assert_eq!(
        parse_transcript(DISCONNECT),
        Err(ParseError::IncompleteFrame)
    );
    let oversized_frame = format!(
        "{{\"direction\":\"client_to_agent\",\"message\":{{\"jsonrpc\":\"2.0\",\"method\":\"{}\"}}}}\n",
        "x".repeat(MAX_FRAME_BYTES)
    );
    assert_eq!(
        parse_transcript(&oversized_frame),
        Err(ParseError::FrameTooLarge)
    );
    assert_eq!(
        parse_transcript(&"x".repeat(MAX_TRANSCRIPT_BYTES + 1)),
        Err(ParseError::TranscriptTooLarge)
    );

    let one = "{\"direction\":\"client_to_agent\",\"message\":{\"jsonrpc\":\"2.0\",\"method\":\"session/cancel\"}}\n";
    assert_eq!(
        parse_transcript(&one.repeat(MAX_TRANSCRIPT_FRAMES + 1)),
        Err(ParseError::TooManyFrames)
    );
    assert!(ROOT.contains("gemini-cli-0.51.0"));
}
