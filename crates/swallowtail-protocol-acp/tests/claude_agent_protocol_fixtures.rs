#[path = "claude_agent_releases/mod.rs"]
mod claude_agent_releases;

use crate::{claude_agent_support, support};

use claude_agent_support::current_model;
use serde_json::Value;
use support::{Direction, methods, parse_json, parse_transcript};
use swallowtail_protocol_acp::ACP_PROTOCOL_VERSION;

const ROOT: &str = "fixtures/acp-v1-claude-agent-acp-0.53.0-0.61.0";
const PROTOCOL: &str = include_str!("fixtures/acp-v1-claude-agent-acp-0.53.0-0.61.0/protocol.json");
const RELEASES: &str =
    include_str!("fixtures/acp-v1-claude-agent-acp-0.53.0-0.61.0/release-corpus.json");
const INITIALIZE_BASELINE: &str =
    include_str!("fixtures/acp-v1-claude-agent-acp-0.53.0-0.61.0/initialize-0.53.0.ndjson");
const INITIALIZE_PROVIDERS: &str =
    include_str!("fixtures/acp-v1-claude-agent-acp-0.53.0-0.61.0/initialize-0.60.0.ndjson");
const INITIALIZE_STEERING: &str =
    include_str!("fixtures/acp-v1-claude-agent-acp-0.53.0-0.61.0/initialize-0.61.0.ndjson");
const NEW_SESSION: &str =
    include_str!("fixtures/acp-v1-claude-agent-acp-0.53.0-0.61.0/new-session.ndjson");
const PROMPT_SUCCESS: &str =
    include_str!("fixtures/acp-v1-claude-agent-acp-0.53.0-0.61.0/prompt-success.ndjson");
const PERMISSION_CANCEL: &str =
    include_str!("fixtures/acp-v1-claude-agent-acp-0.53.0-0.61.0/permission-cancel.ndjson");
const FORM_ELICITATION: &str =
    include_str!("fixtures/acp-v1-claude-agent-acp-0.53.0-0.61.0/form-elicitation.ndjson");
const MODEL_DRIFT: &str =
    include_str!("fixtures/acp-v1-claude-agent-acp-0.53.0-0.61.0/model-drift.ndjson");
const ACCESS_FAILURE: &str =
    include_str!("fixtures/acp-v1-claude-agent-acp-0.53.0-0.61.0/access-failure.ndjson");
const DISCONNECT: &str =
    include_str!("fixtures/acp-v1-claude-agent-acp-0.53.0-0.61.0/disconnect.ndjson");

#[test]
fn initialization_keeps_additive_capability_milestones_private() {
    for (version, transcript, providers, steering) in [
        ("0.53.0", INITIALIZE_BASELINE, false, false),
        ("0.60.0", INITIALIZE_PROVIDERS, true, false),
        ("0.61.0", INITIALIZE_STEERING, true, true),
    ] {
        let frames = parse_transcript(transcript).expect("initialize transcript parses");
        assert_eq!(methods(&frames), ["initialize"]);
        assert_eq!(frames[0].direction(), Direction::ClientToAgent);
        assert_eq!(frames[0].id(), frames[1].id());
        assert_eq!(
            frames[0].message()["params"]["protocolVersion"],
            ACP_PROTOCOL_VERSION
        );
        assert!(
            frames[0].message()["params"]["clientCapabilities"]
                .get("auth")
                .is_none()
        );
        assert_eq!(
            frames[0].message()["params"]["clientCapabilities"]["elicitation"]["form"],
            serde_json::json!({})
        );
        assert_eq!(
            frames[1].message()["result"]["agentInfo"]["version"],
            version
        );
        assert_eq!(
            frames[1].message()["result"]["authMethods"],
            serde_json::json!([])
        );
        assert_eq!(
            frames[1].message()["result"]["agentCapabilities"]
                .get("providers")
                .is_some(),
            providers
        );
        assert_eq!(
            frames[1].message()["result"]["_meta"]
                .get("steering")
                .is_some(),
            steering
        );
    }
}

#[test]
fn form_elicitation_preserves_typed_choice_and_answer_field_identity() {
    let frames = parse_transcript(FORM_ELICITATION).expect("elicitation transcript parses");
    assert_eq!(methods(&frames), ["elicitation/create"]);
    assert_eq!(frames[0].direction(), Direction::AgentToClient);
    assert_eq!(frames[1].direction(), Direction::ClientToAgent);
    assert_eq!(frames[0].id(), frames[1].id());
    assert_eq!(
        frames[0].message()["params"]["requestedSchema"]["properties"]["question_0"]["title"],
        "Component"
    );
    assert_eq!(
        frames[1].message()["result"],
        serde_json::json!({"action": "accept", "content": {"question_0": "Panel"}})
    );
}

#[test]
fn new_session_binds_exact_model_and_provider_native_read_tools() {
    let frames = parse_transcript(NEW_SESSION).expect("new-session transcript parses");
    assert_eq!(methods(&frames), ["session/new"]);
    assert_eq!(frames[0].id(), frames[1].id());
    assert_eq!(
        frames[0].message()["params"]["_meta"]["claudeCode"]["options"]["tools"],
        serde_json::json!(["Read", "Glob", "Grep"])
    );
    assert_eq!(
        frames[0].message()["params"]["_meta"]["claudeCode"]["options"]["settings"]["model"],
        "claude-sonnet-4-6"
    );
    assert_eq!(
        current_model(&frames[1].message()["result"]["configOptions"]),
        Some("claude-sonnet-4-6")
    );

    let boundary = parse_json(PROTOCOL);
    assert_eq!(boundary["configuration"]["posture"], "ambient");
    assert_eq!(
        boundary["configuration"]["process_isolation"],
        "ambient_host"
    );
    assert_eq!(
        boundary["configuration"]["tool_policy_is_containment"],
        false
    );
    assert_eq!(boundary["configuration"]["sandbox_required"], false);
}

#[test]
fn prompt_corpus_preserves_reasoning_tool_usage_output_and_terminal_order() {
    let frames = parse_transcript(PROMPT_SUCCESS).expect("prompt transcript parses");
    assert_eq!(
        frames.first().and_then(|frame| frame.method()),
        Some("session/prompt")
    );
    assert_eq!(
        frames.last().and_then(|frame| frame.id()),
        Some(&Value::from(2))
    );

    let updates: Vec<_> = frames
        .iter()
        .filter(|frame| frame.method() == Some("session/update"))
        .map(|frame| {
            frame.message()["params"]["update"]["sessionUpdate"]
                .as_str()
                .expect("update kind")
        })
        .collect();
    assert_eq!(
        updates,
        [
            "agent_thought_chunk",
            "tool_call",
            "tool_call_update",
            "usage_update",
            "agent_message_chunk"
        ]
    );
    assert_eq!(
        frames.last().expect("terminal response").message()["result"]["stopReason"],
        "end_turn"
    );
}

#[test]
fn permission_model_access_and_disconnect_fail_closed() {
    let permission = parse_transcript(PERMISSION_CANCEL).expect("permission transcript parses");
    let tool_index = permission
        .iter()
        .position(|frame| {
            frame.method() == Some("session/update")
                && frame.message()["params"]["update"]["sessionUpdate"] == "tool_call"
        })
        .expect("tool call");
    let permission_index = permission
        .iter()
        .position(|frame| frame.method() == Some("session/request_permission"))
        .expect("permission request");
    assert!(tool_index < permission_index);
    assert_eq!(
        permission[3].message()["result"]["outcome"]["optionId"],
        "reject-once"
    );
    assert_eq!(permission[4].method(), Some("session/cancel"));
    assert_eq!(
        permission.last().expect("cancel response").message()["result"]["stopReason"],
        "cancelled"
    );

    let drift = parse_transcript(MODEL_DRIFT).expect("model drift transcript parses");
    assert_eq!(
        current_model(&drift[1].message()["result"]["configOptions"]),
        Some("default")
    );
    assert_ne!(
        current_model(&drift[1].message()["result"]["configOptions"]),
        Some("claude-sonnet-4-6")
    );

    let access = parse_transcript(ACCESS_FAILURE).expect("access failure parses");
    assert_eq!(access[1].message()["error"]["code"], -32603);
    assert!(access[1].message()["error"]["data"].is_null());

    let disconnect = parse_transcript(DISCONNECT).expect("disconnect transcript parses");
    assert_eq!(disconnect[0].method(), Some("session/prompt"));
    assert!(
        disconnect
            .iter()
            .all(|frame| frame.id() != Some(&Value::from(6)) || frame.method().is_some())
    );
}

#[test]
fn access_and_capability_exclusions_are_explicit_and_redacted() {
    let boundary = parse_json(PROTOCOL);
    assert_eq!(boundary["access"]["mechanism"], "api_key");
    assert_eq!(
        boundary["access"]["endpoint_audience"],
        "anthropic_public_api"
    );
    assert_eq!(boundary["access"]["terminal_auth_advertised"], false);
    assert_eq!(boundary["access"]["authenticate_called"], false);
    assert_eq!(boundary["access"]["claude_subscription_supported"], false);
    assert_eq!(boundary["access"]["implicit_fallback"], false);
    assert_eq!(
        boundary["session_subset"]["load"],
        "corpus_qualified_not_realized"
    );
    assert_eq!(
        boundary["session_subset"]["resume"],
        "corpus_qualified_not_realized"
    );
    assert_eq!(boundary["session_subset"]["close"], "realized");
    assert_eq!(boundary["session_subset"]["delete"], "realized");
    assert_eq!(boundary["session_subset"]["list"], "upstream_not_selected");
    assert_eq!(boundary["session_subset"]["fork"], "upstream_not_selected");
    assert_eq!(
        boundary["session_subset"]["writes_shell_web_subagents_terminals"],
        false
    );
    assert_eq!(boundary["session_subset"]["client_mcp_elicitation"], false);
    assert_eq!(
        boundary["session_subset"]["form_elicitation"],
        "typed_choice_and_other_subset"
    );

    for fixture in [
        PROTOCOL,
        RELEASES,
        INITIALIZE_BASELINE,
        INITIALIZE_PROVIDERS,
        INITIALIZE_STEERING,
        NEW_SESSION,
        PROMPT_SUCCESS,
        PERMISSION_CANCEL,
        FORM_ELICITATION,
        MODEL_DRIFT,
        ACCESS_FAILURE,
        DISCONNECT,
    ] {
        for forbidden in [
            "/Users/",
            "Toms-MacBook-Pro",
            "ANTHROPIC_API_KEY=",
            "sk-ant-",
            "Bearer ",
            "OAuth",
            "keychain",
        ] {
            assert!(!fixture.contains(forbidden), "{ROOT} leaked {forbidden}");
        }
    }
}
