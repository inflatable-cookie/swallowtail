use super::{
    AUTH_REQUIRED, AUTHORITY, CAPABILITY_DRIFT, DISCONNECT, FILESYSTEM_WRITE,
    FILESYSTEM_WRITE_REJECTED, INITIALIZE, NEW_SESSION, PROTOCOL, ROOT, SESSION_ID, VERSION_DRIFT,
};
use crate::support::{Direction, ParseError, methods, parse_json, parse_transcript};
use serde_json::{Value, json};
use swallowtail_protocol_acp::ACP_PROTOCOL_VERSION;

#[test]
fn manifest_pins_successor_wire_schema_sdk_agent_and_access_independently() {
    let fixture = parse_json(PROTOCOL);
    assert_eq!(fixture["fixture_schema"], 1);
    assert_eq!(fixture["protocol"]["wire_version"], ACP_PROTOCOL_VERSION);
    assert_eq!(fixture["protocol"]["schema_artifact"], "schema-v1.19.1");
    assert_eq!(
        fixture["protocol"]["schema_source_commit"],
        "d0549a115750a0a25c1c5631dd72e0d248859aa4"
    );
    assert_eq!(fixture["kimi"]["source_repository"], "MoonshotAI/kimi-code");
    assert_eq!(fixture["kimi"]["version"], "0.28.1");
    assert_eq!(
        fixture["kimi"]["source_commit"],
        "efacf0452d46f5dbd67499eabc053869495d5213"
    );
    assert_eq!(fixture["kimi"]["acp_adapter_package"], "0.3.4");
    assert_eq!(fixture["kimi"]["acp_sdk"], "0.23.0");
    assert_eq!(fixture["kimi"]["arguments"], json!(["acp"]));
    assert_eq!(
        fixture["access"]["credential_mechanism"],
        "pre_existing_delegated_harness_auth"
    );
    assert_eq!(fixture["access"]["advertised_login_executed"], false);
    assert_eq!(
        fixture["access"]["configured_non_oauth_credentials_in_subset"],
        false
    );
}

#[test]
fn executable_state_upgrade_gate_and_ambient_posture_are_exact() {
    let fixture = parse_json(PROTOCOL);
    let executable = &fixture["executable"];
    assert_eq!(executable["platform"], "darwin-arm64");
    assert_eq!(
        executable["archive_sha256"],
        "fa93e9daa30449c5cb32d8adb2a75651ec6c60dcd72fd4bf65c530edb8c144f9"
    );
    assert_eq!(executable["ambient_path_allowed"], false);
    assert_eq!(executable["upstream_signature"]["app_sandbox"], false);
    assert!(executable.get("deployment_helper").is_none());
    assert_eq!(fixture["isolation"]["posture"], "ambient_host");
    assert_eq!(fixture["isolation"]["filesystem_boundary_claimed"], false);

    assert_eq!(
        fixture["provider_state"]["root_environment"],
        "KIMI_CODE_HOME"
    );
    assert_eq!(
        fixture["provider_state"]["auto_update_environment"]["KIMI_CODE_NO_AUTO_UPDATE"],
        "1"
    );
    assert_eq!(fixture["upgrade_gate"]["floating_latest_allowed"], false);
    assert_eq!(fixture["upgrade_gate"]["self_upgrade_allowed"], false);
    assert_eq!(
        fixture["upgrade_gate"]["mismatch"],
        "reject_before_session_work"
    );
}

#[test]
fn advertised_successor_features_do_not_enter_the_selected_subset() {
    let fixture = parse_json(PROTOCOL);
    let excluded = fixture["excluded_provider_features"]
        .as_array()
        .expect("excluded provider features");
    for feature in [
        "image_prompt",
        "embedded_context_prompt",
        "mcp_http",
        "session_list",
        "unstable_model_selection",
        "local_shell_execution",
        "plugins",
        "background_work",
        "subagents",
    ] {
        assert!(excluded.iter().any(|candidate| candidate == feature));
    }
    assert_eq!(
        fixture["access"]["separate_configured_instances"]
            .as_array()
            .expect("separate access instances")
            .len(),
        4
    );
}

#[test]
fn initialization_observes_exact_capabilities_without_accepting_login_authority() {
    let frames = parse_transcript(INITIALIZE).expect("initialization transcript parses");
    assert_eq!(frames.len(), 2);
    assert_eq!(frames[0].direction(), Direction::ClientToAgent);
    assert_eq!(frames[0].method(), Some("initialize"));
    assert_eq!(frames[0].message()["params"]["protocolVersion"], 1);
    assert_eq!(
        frames[0].message()["params"]["clientCapabilities"]["auth"]["terminal"],
        false
    );
    assert_eq!(frames[1].message()["result"]["protocolVersion"], 1);
    let capabilities = &frames[1].message()["result"]["agentCapabilities"];
    assert_eq!(capabilities["loadSession"], true);
    assert_eq!(capabilities["sessionCapabilities"]["list"], json!({}));
    assert_eq!(capabilities["sessionCapabilities"]["resume"], json!({}));
    assert!(capabilities["sessionCapabilities"].get("close").is_none());
    assert_eq!(
        frames[1].message()["result"]["authMethods"][0]["id"],
        "login"
    );
    assert_eq!(
        frames[1].message()["result"]["authMethods"][0]["type"],
        "terminal"
    );
    assert_eq!(
        parse_json(PROTOCOL)["access"]["advertised_login_is_evidence_only"],
        true
    );
}

#[test]
fn new_session_mints_the_only_provider_reference_eligible_for_attachment() {
    let frames = parse_transcript(NEW_SESSION).expect("new-session transcript parses");
    assert_eq!(methods(&frames), ["session/new"]);
    assert_eq!(frames[0].message()["params"]["cwd"], "/fixture/workspace");
    assert_eq!(frames[0].message()["params"]["mcpServers"], json!([]));
    assert_eq!(frames[1].message()["result"]["sessionId"], SESSION_ID);
    assert_eq!(
        frames[1].message()["result"]["configOptions"][0]["currentValue"],
        "kimi-coder"
    );

    let authority = parse_json(AUTHORITY);
    assert!(
        authority["load_or_resume"]
            .as_array()
            .expect("authority cases")
            .iter()
            .all(|case| case["expected"] == "reject_before_process_or_wire")
    );
}

#[test]
fn write_callback_is_correlated_bounded_and_not_tool_approval_or_containment() {
    let frames = parse_transcript(FILESYSTEM_WRITE).expect("write transcript parses");
    assert_eq!(methods(&frames), ["fs/write_text_file"]);
    assert_eq!(frames[0].direction(), Direction::AgentToClient);
    assert_eq!(frames[0].message()["params"]["sessionId"], SESSION_ID);
    assert_eq!(
        frames[0].message()["params"]["path"],
        "/fixture/workspace/src/generated.rs"
    );
    assert_eq!(
        frames[0].message()["params"]["content"],
        "pub fn generated() {}\n"
    );
    assert_eq!(frames[1].id(), frames[0].id());
    assert_eq!(frames[1].message()["result"], Value::Null);

    let callback = &parse_json(PROTOCOL)["client_callbacks"];
    assert_eq!(callback["required_resource_access"], "read_write");
    assert_eq!(callback["tool_approval"], "separate_not_granted");
    assert_eq!(callback["process_containment"], "ambient_host_not_claimed");
}

#[test]
fn write_authority_mismatches_return_errors_without_mutation() {
    let frames =
        parse_transcript(FILESYSTEM_WRITE_REJECTED).expect("rejected writes transcript parses");
    assert_eq!(methods(&frames), ["fs/write_text_file"; 3]);
    let requests = frames.iter().filter(|frame| frame.method().is_some());
    for request in requests {
        let response = frames
            .iter()
            .find(|frame| frame.method().is_none() && frame.id() == request.id())
            .expect("rejection response is correlated");
        assert_eq!(response.message()["error"]["code"], -32602);
        assert!(response.message().get("result").is_none());
    }
    assert!(
        parse_json(AUTHORITY)["write_callback"]
            .as_array()
            .expect("write authority cases")
            .iter()
            .all(|case| case["mutation"] == false)
    );
}

#[test]
fn version_capability_auth_and_disconnect_drift_fail_closed() {
    let version = parse_transcript(VERSION_DRIFT).expect("version drift parses");
    assert_ne!(
        version[1].message()["result"]["protocolVersion"],
        ACP_PROTOCOL_VERSION
    );

    let capability = parse_transcript(CAPABILITY_DRIFT).expect("capability drift parses");
    assert_eq!(
        capability[1].message()["result"]["agentCapabilities"]["sessionCapabilities"]["close"],
        json!({})
    );
    assert_eq!(
        parse_json(PROTOCOL)["negotiation"]["additive_capability"],
        "observed_not_claimed"
    );

    let auth = parse_transcript(AUTH_REQUIRED).expect("auth failure parses");
    assert_eq!(auth[1].message()["error"]["code"], -32000);
    assert!(!AUTH_REQUIRED.contains("token"));
    assert!(!AUTH_REQUIRED.contains("api-key"));

    assert_eq!(
        parse_transcript(DISCONNECT),
        Err(ParseError::IncompleteFrame)
    );
    assert_eq!(
        parse_json(PROTOCOL)["cleanup"]["session_close"],
        "close_stdin_stop_owned_process_join"
    );
    assert!(ROOT.contains("kimi-code-0.28.1"));
}
