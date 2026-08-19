use serde_json::Value;
use swallowtail_adapter_openhands::{
    OPENHANDS_PACKAGE_AXIS, OPENHANDS_PACKAGE_VERSION, openhands_agent_server_claim,
    openhands_package_binding,
};
use swallowtail_core::InterfaceVersion;

const IDENTITY: &str = include_str!("fixtures/openhands-agent-server-1.42.1/identity.json");
const PROTOCOL: &str = include_str!("fixtures/openhands-agent-server-1.42.1/protocol.json");
const COMMAND: &str = include_str!("fixtures/openhands-agent-server-1.42.1/command.json");
const ACTIVITY: &str = include_str!("fixtures/openhands-agent-server-1.42.1/activity.jsonl");
const ABORT: &str = include_str!("fixtures/openhands-agent-server-1.42.1/abort.jsonl");
const LIMIT: &str = include_str!("fixtures/openhands-agent-server-1.42.1/limit.jsonl");
const ERROR: &str = include_str!("fixtures/openhands-agent-server-1.42.1/error.jsonl");
const NEGATIVE: &str = include_str!("fixtures/openhands-agent-server-1.42.1/negative-cases.json");
const CORPUS_PLAN: &str = include_str!("fixtures/openhands-agent-server-1.42.1/corpus-plan.json");
const HEALTH: &str = include_str!("fixtures/openhands-agent-server-1.42.1/success/health.json");
const ALIVE: &str = include_str!("fixtures/openhands-agent-server-1.42.1/success/alive.json");
const READY: &str = include_str!("fixtures/openhands-agent-server-1.42.1/success/ready.json");
const SERVER_INFO: &str =
    include_str!("fixtures/openhands-agent-server-1.42.1/success/server_info.json");
const START_CONVERSATION: &str =
    include_str!("fixtures/openhands-agent-server-1.42.1/success/start-conversation.json");
const MESSAGE_EVENT: &str =
    include_str!("fixtures/openhands-agent-server-1.42.1/success/message-event.json");
const STATE_UPDATE: &str =
    include_str!("fixtures/openhands-agent-server-1.42.1/success/conversation-state-update.json");

#[test]
fn frozen_identity_keeps_loopback_http_ws_separate_from_v0_acp_sdk_and_never_confirm() {
    let identity: Value = serde_json::from_str(IDENTITY).expect("identity fixture");
    assert_eq!(identity["axis"], OPENHANDS_PACKAGE_AXIS);
    assert_eq!(identity["route"], "openhands.agent-server");
    assert_eq!(identity["official"]["version"], OPENHANDS_PACKAGE_VERSION);
    assert_eq!(
        identity["official"]["wheel_sha256"],
        "772a73b19684acab5f9f61b1c244f156052625ade51a5e48a424b3c13039f7a7"
    );
    assert_eq!(
        identity["identity_decision"]["flatten_onto_v0_socketio"],
        false
    );
    assert_eq!(
        identity["identity_decision"]["flatten_onto_contract_035_remote_acp"],
        false
    );
    assert_eq!(
        identity["identity_decision"]["flatten_onto_python_sdk"],
        false
    );
    assert_eq!(
        identity["identity_decision"]["flatten_onto_switch_acp_model"],
        false
    );
    assert_eq!(
        identity["identity_decision"]["flatten_onto_docker_sandbox"],
        false
    );
    assert_eq!(
        identity["identity_decision"]["flatten_onto_hosted_api_sandbox"],
        false
    );
    assert_eq!(
        identity["identity_decision"]["require_explicit_loopback_host"],
        true
    );
    assert_eq!(
        identity["identity_decision"]["swallowtail_never_binds_wildcard"],
        true
    );
    assert_eq!(
        identity["identity_decision"]["inherit_ambient_session_api_keys"],
        false
    );
    assert_eq!(
        identity["identity_decision"]["inherit_ambient_oh_secret_key"],
        false
    );
    assert_eq!(
        identity["identity_decision"]["inherit_ambient_llm_api_key"],
        false
    );
    assert_eq!(
        identity["identity_decision"]["confirmation_policy_swallowtail"],
        "AlwaysConfirm"
    );
    assert_eq!(
        identity["identity_decision"]["mint_session_api_keys"],
        false
    );
    assert_eq!(
        identity["identity_decision"]["run_subscription_login"],
        false
    );
    assert_eq!(identity["identity_decision"]["provider_prompt_sent"], false);
    assert_eq!(identity["identity_decision"]["live_server_started"], false);
    assert_eq!(identity["identity_decision"]["claim_change_in_card"], false);

    let protocol: Value = serde_json::from_str(PROTOCOL).expect("protocol fixture");
    assert_eq!(protocol["authority"]["loopback_only"], true);
    assert_eq!(
        protocol["authority"]["swallowtail_passes_wildcard_host"],
        false
    );
    assert_eq!(
        protocol["authority"]["confirmation_policy_swallowtail"],
        "AlwaysConfirm"
    );
    assert_eq!(
        protocol["authority"]["confirmation_policy_server_default"],
        "NeverConfirm"
    );
    assert_eq!(
        protocol["authority"]["swallowtail_omits_confirmation_policy"],
        false
    );
    assert_eq!(protocol["websocket"]["not_v0_socketio"], true);
    assert_eq!(
        protocol["websocket"]["path"],
        "/sockets/events/{conversation_id}"
    );
    assert_eq!(
        protocol["websocket"]["not_the_docs_readme_path"],
        "/conversations/{conversation_id}/events/socket"
    );

    assert!(openhands_package_binding(OPENHANDS_PACKAGE_VERSION).is_some());
    let claim = openhands_agent_server_claim();
    let version = InterfaceVersion::new(OPENHANDS_PACKAGE_VERSION).expect("qualified version");
    assert!(claim.assess(&version).is_permitted());
    assert!(
        !claim
            .assess(&InterfaceVersion::new("1.42.2").expect("newer"))
            .is_permitted()
    );
}

#[test]
fn named_corpus_files_stay_wired_to_the_first_driver() {
    for (name, body) in [
        ("command.json", COMMAND),
        ("negative-cases.json", NEGATIVE),
        ("corpus-plan.json", CORPUS_PLAN),
        ("health.json", HEALTH),
        ("alive.json", ALIVE),
        ("ready.json", READY),
        ("server_info.json", SERVER_INFO),
        ("start-conversation.json", START_CONVERSATION),
        ("message-event.json", MESSAGE_EVENT),
        ("conversation-state-update.json", STATE_UPDATE),
    ] {
        let value: Value = serde_json::from_str(body).unwrap_or_else(|_| panic!("{name}"));
        assert!(value.is_object(), "{name} must be an object");
    }

    let command: Value = serde_json::from_str(COMMAND).expect("command");
    let argv = command["example_argv"]
        .as_array()
        .expect("example argv")
        .iter()
        .map(|value| value.as_str().expect("argv text"))
        .collect::<Vec<_>>();
    assert_eq!(argv[0], "python");
    assert!(argv.contains(&"-m"));
    assert!(argv.contains(&"openhands.agent_server"));
    assert!(argv.contains(&"--host"));
    assert!(argv.contains(&"127.0.0.1"));
    assert!(argv.contains(&"--port"));
    for forbidden in command["forbidden_argv"]
        .as_array()
        .expect("forbidden argv")
        .iter()
        .map(|value| value.as_str().expect("flag"))
    {
        assert!(
            !argv.contains(&forbidden),
            "{forbidden} must not appear on the Agent Server example argv"
        );
    }
    assert!(
        !argv
            .iter()
            .any(|argument| argument.contains("0.0.0.0") || argument.contains("NeverConfirm"))
    );

    let plan: Value = serde_json::from_str(CORPUS_PLAN).expect("corpus plan");
    assert_eq!(plan["route"], "openhands.agent-server");
    assert_eq!(plan["create_package_in_card"], 288);
    assert_eq!(plan["no_production_claim_in_card"], 287);

    let negatives: Value = serde_json::from_str(NEGATIVE).expect("negatives");
    let ids: Vec<_> = negatives["cases"]
        .as_array()
        .expect("cases")
        .iter()
        .map(|case| case["id"].as_str().expect("id"))
        .collect();
    for required in [
        "malformed-json",
        "unknown-kind-fail-closed",
        "acp-jsonrpc-is-not-this-route",
        "v0-socketio-init-session-unmapped",
        "wildcard-host-not-swallowtail-authority",
        "omit-host-when-session-key-set-binds-wildcard",
        "never-confirm-not-swallowtail-authority",
        "omit-confirmation-policy-inherits-never-confirm",
        "omit-max-iterations-inherits-500",
        "python-sdk-conversation-not-this-route",
        "inherit-ambient-session-api-key",
        "inherit-ambient-oh-secret-key",
        "inherit-ambient-llm-api-key",
        "host-python-3-9-cannot-run",
    ] {
        assert!(ids.contains(&required), "missing negative case {required}");
    }
}

#[test]
fn success_limit_and_abort_streams_are_kind_events_not_acp() {
    let kinds = jsonl_kinds(ACTIVITY);
    assert!(kinds.contains(&"StreamingDeltaEvent".to_owned()));
    assert!(kinds.contains(&"MessageEvent".to_owned()));
    assert!(kinds.contains(&"FinishAction".to_owned()));
    assert!(kinds.contains(&"ConversationStateUpdateEvent".to_owned()));
    for line in ACTIVITY.lines().filter(|line| !line.is_empty()) {
        let value: Value = serde_json::from_str(line).expect("activity line");
        assert_ne!(value.get("jsonrpc"), Some(&Value::from("2.0")));
        assert!(value.get("kind").is_some());
    }

    let abort_kinds = jsonl_kinds(ABORT);
    assert!(abort_kinds.contains(&"InterruptEvent".to_owned()));
    let limit: Value =
        serde_json::from_str(LIMIT.lines().next().expect("limit")).expect("limit event");
    assert_eq!(limit["kind"], "ConversationStateUpdateEvent");
    assert_eq!(limit["value"], "stuck");

    let error: Value =
        serde_json::from_str(ERROR.lines().next().expect("error")).expect("error event");
    assert!(
        error["kind"]
            .as_str()
            .is_some_and(|kind| kind.ends_with("ErrorEvent"))
    );
}

fn jsonl_kinds(body: &str) -> Vec<String> {
    body.lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let value: Value = serde_json::from_str(line).expect("jsonl line");
            value["kind"].as_str().expect("kind").to_owned()
        })
        .collect()
}
