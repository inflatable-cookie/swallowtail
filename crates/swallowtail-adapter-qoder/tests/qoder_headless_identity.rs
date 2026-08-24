use serde_json::Value;
use swallowtail_adapter_qoder::{
    QODER_PACKAGE_VERSION, qoder_headless_claim, qoder_package_binding,
};
use swallowtail_core::InterfaceVersion;

const IDENTITY: &str = include_str!("fixtures/qoder-headless-1.1.25/identity.json");
const PROTOCOL: &str = include_str!("fixtures/qoder-headless-1.1.25/protocol.json");
const COMMAND: &str = include_str!("fixtures/qoder-headless-1.1.25/command.json");
const SUCCESS: &str = include_str!("fixtures/qoder-headless-1.1.25/success.jsonl");
const ABORT: &str = include_str!("fixtures/qoder-headless-1.1.25/abort.jsonl");
const LIMIT: &str = include_str!("fixtures/qoder-headless-1.1.25/limit.jsonl");
const ACTIVITY: &str = include_str!("fixtures/qoder-headless-1.1.25/activity.jsonl");
const NEGATIVE: &str = include_str!("fixtures/qoder-headless-1.1.25/negative-cases.json");
const CORPUS_PLAN: &str = include_str!("fixtures/qoder-headless-1.1.25/corpus-plan.json");

#[test]
fn frozen_identity_keeps_print_stream_json_separate_from_acp_sdk_and_yolo() {
    let identity: Value = serde_json::from_str(IDENTITY).expect("identity fixture");
    assert_eq!(identity["axis"], "qoder.package");
    assert_eq!(identity["route"], "qoder.headless");
    assert_eq!(identity["official"]["version"], QODER_PACKAGE_VERSION);
    assert_eq!(identity["identity_decision"]["flatten_onto_acp"], false);
    assert_eq!(
        identity["identity_decision"]["flatten_onto_sdk_stdio"],
        false
    );
    assert_eq!(identity["identity_decision"]["flatten_onto_tui"], false);
    assert_eq!(identity["identity_decision"]["pass_yolo_or_bypass"], false);
    assert_eq!(
        identity["identity_decision"]["pass_permission_mode_dont_ask"],
        true
    );
    assert_eq!(identity["identity_decision"]["require_max_turns"], true);
    assert_eq!(
        identity["identity_decision"]["max_turns_argv_historical_inert"],
        true
    );
    assert_eq!(
        identity["identity_decision"]["max_turns_argv_sets_agent_loop"],
        false
    );
    assert_eq!(
        identity["identity_decision"]["max_turns_agent_loop_factory"],
        1000
    );
    assert_eq!(
        identity["identity_decision"]["require_no_session_persistence"],
        true
    );
    assert_eq!(
        identity["identity_decision"]["selected_output"],
        "stream-json"
    );
    assert_eq!(identity["identity_decision"]["claim_change_in_card"], false);
    assert_eq!(identity["official"]["bin_qodercli"], "bundle/qodercli.js");
    assert_eq!(
        identity["official"]["bin_qoder"],
        "bundle/qoder-npm-dispatcher.cjs"
    );

    let protocol: Value = serde_json::from_str(PROTOCOL).expect("protocol fixture");
    assert_eq!(protocol["authority"]["swallowtail_passes_yolo"], false);
    assert_eq!(
        protocol["authority"]["swallowtail_passes_permission_mode_dont_ask"],
        true
    );
    assert_eq!(protocol["authority"]["require_max_turns"], true);
    assert_eq!(
        protocol["authority"]["max_turns_argv_historical_inert"],
        true
    );
    assert_eq!(
        protocol["authority"]["max_turns_argv_sets_agent_loop"],
        false
    );
    assert_eq!(protocol["authority"]["max_turns_agent_loop_factory"], 1000);
    assert_eq!(protocol["limit_fixture_decoder_only"], true);
    assert_eq!(protocol["limit_fixture_does_not_prove_argv_bound"], true);
    assert_eq!(
        protocol["authority"]["require_no_session_persistence"],
        true
    );
    assert_eq!(protocol["acp_wins_over_print"], true);
    assert_eq!(protocol["stream_event_unselected"], true);
    assert_eq!(protocol["text_output_unselected"], true);
    assert_eq!(
        protocol["json_dump_at_end"]["not_the_streaming_decoder"],
        true
    );

    assert!(qoder_package_binding(QODER_PACKAGE_VERSION).is_some());
    let claim = qoder_headless_claim();
    let version = InterfaceVersion::new(QODER_PACKAGE_VERSION).expect("qualified version");
    assert!(claim.assess(&version).is_permitted());
    assert!(
        !claim
            .assess(&InterfaceVersion::new("1.1.26").expect("newer"))
            .is_permitted()
    );
}

#[test]
fn named_corpus_files_stay_wired_to_the_first_driver() {
    for (name, body) in [
        ("command.json", COMMAND),
        ("negative-cases.json", NEGATIVE),
        ("corpus-plan.json", CORPUS_PLAN),
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
    assert_eq!(argv[0], "qodercli");
    assert!(argv.contains(&"--print"));
    assert!(argv.contains(&"--output-format"));
    assert!(argv.contains(&"stream-json"));
    assert!(argv.contains(&"--permission-mode"));
    assert!(argv.contains(&"dont_ask"));
    assert!(argv.contains(&"--max-turns"));
    assert!(argv.contains(&"--no-session-persistence"));
    assert!(argv.contains(&"--cwd"));
    assert!(!argv.contains(&"--input-format"));
    for forbidden in command["forbidden_argv"]
        .as_array()
        .expect("forbidden argv")
        .iter()
        .map(|value| value.as_str().expect("flag"))
    {
        if forbidden == "stream-json" {
            continue;
        }
        assert!(
            !argv.contains(&forbidden),
            "{forbidden} must not appear on the headless example argv"
        );
    }

    let plan: Value = serde_json::from_str(CORPUS_PLAN).expect("corpus plan");
    assert_eq!(plan["route"], "qoder.headless");
    assert_eq!(plan["no_production_claim_in_card"], 278);
    assert_eq!(plan["create_package_in_card"], 279);

    let negatives: Value = serde_json::from_str(NEGATIVE).expect("negatives");
    let ids: Vec<_> = negatives["cases"]
        .as_array()
        .expect("cases")
        .iter()
        .map(|case| case["id"].as_str().expect("id"))
        .collect();
    for required in [
        "acp-flag-is-not-headless",
        "print-plus-acp-is-acp",
        "acp-jsonrpc-is-not-headless",
        "sdk-input-stream-json-unmapped",
        "output-json-is-not-streaming-decoder",
        "yolo-not-swallowtail-authority",
        "bypass-not-swallowtail-authority",
        "accept-edits-not-swallowtail-authority",
        "omit-permission-mode-inherits-host-default",
        "omit-max-turns-not-route-argv",
        "omit-no-session-persistence",
        "ide-dispatcher-not-this-route",
        "malformed-json",
        "oversized-frame",
        "missing-prompt-fails",
    ] {
        assert!(ids.contains(&required), "missing negative case {required}");
    }
}

#[test]
fn success_limit_and_abort_streams_are_stream_json_not_acp() {
    let types = jsonl_types(SUCCESS);
    assert_eq!(
        types.iter().map(String::as_str).collect::<Vec<_>>(),
        ["system", "assistant", "result"]
    );
    for line in SUCCESS.lines().filter(|line| !line.is_empty()) {
        let value: Value = serde_json::from_str(line).expect("success line");
        assert_ne!(value.get("jsonrpc"), Some(&Value::from("2.0")));
    }

    let abort_types = jsonl_types(ABORT);
    assert_eq!(
        abort_types.iter().map(String::as_str).collect::<Vec<_>>(),
        ["system", "result"]
    );
    let abort: Value =
        serde_json::from_str(ABORT.lines().nth(1).expect("abort result")).expect("abort result");
    assert_eq!(abort["subtype"], "error_during_execution");
    assert_eq!(abort["terminal_reason"], "aborted_streaming");

    let limit: Value =
        serde_json::from_str(LIMIT.lines().nth(1).expect("limit result")).expect("limit result");
    assert_eq!(limit["subtype"], "error_max_turns");
    assert_eq!(limit["num_turns"], 1);
    // Synthetic decoder fixture only; AgentLoop factory ceiling is 1000.

    let activity_cases: Vec<_> = ACTIVITY
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_str::<Value>(line).expect("activity line"))
        .collect();
    assert!(
        activity_cases
            .iter()
            .any(|row| row["case"] == "acp-jsonrpc-is-not-headless")
    );
    assert!(
        activity_cases
            .iter()
            .any(|row| row["case"] == "stream-event-unselected")
    );
}

fn jsonl_types(body: &str) -> Vec<String> {
    body.lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let value: Value = serde_json::from_str(line).expect("jsonl line");
            value["type"].as_str().expect("type").to_owned()
        })
        .collect()
}
