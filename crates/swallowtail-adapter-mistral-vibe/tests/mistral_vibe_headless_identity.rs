use serde_json::Value;
use swallowtail_adapter_mistral_vibe::{
    MISTRAL_VIBE_RELEASE_VERSION, mistral_vibe_headless_claim, mistral_vibe_release_binding,
};
use swallowtail_core::InterfaceVersion;

const IDENTITY: &str = include_str!("fixtures/mistral-vibe-headless-2.24.2/identity.json");
const PROTOCOL: &str = include_str!("fixtures/mistral-vibe-headless-2.24.2/protocol.json");
const COMMAND: &str = include_str!("fixtures/mistral-vibe-headless-2.24.2/command.json");
const SUCCESS: &str = include_str!("fixtures/mistral-vibe-headless-2.24.2/success.jsonl");
const ABORT: &str = include_str!("fixtures/mistral-vibe-headless-2.24.2/abort.jsonl");
const STDERR: &str = include_str!("fixtures/mistral-vibe-headless-2.24.2/stderr-error.txt");
const LIMIT: &str = include_str!("fixtures/mistral-vibe-headless-2.24.2/limit-stderr.txt");
const ACTIVITY: &str = include_str!("fixtures/mistral-vibe-headless-2.24.2/activity.jsonl");
const NEGATIVE: &str = include_str!("fixtures/mistral-vibe-headless-2.24.2/negative-cases.json");
const CORPUS_PLAN: &str = include_str!("fixtures/mistral-vibe-headless-2.24.2/corpus-plan.json");

#[test]
fn frozen_identity_keeps_streaming_plan_separate_from_acp_and_yolo() {
    let identity: Value = serde_json::from_str(IDENTITY).expect("identity fixture");
    assert_eq!(identity["axis"], "mistral-vibe.release");
    assert_eq!(identity["route"], "mistral-vibe.headless");
    assert_eq!(
        identity["official"]["version"],
        MISTRAL_VIBE_RELEASE_VERSION
    );
    assert_eq!(
        identity["identity_decision"]["flatten_onto_vibe_acp"],
        false
    );
    assert_eq!(identity["identity_decision"]["flatten_onto_tui"], false);
    assert_eq!(
        identity["identity_decision"]["pass_auto_approve_or_yolo"],
        false
    );
    assert_eq!(identity["identity_decision"]["pass_agent_plan"], true);
    assert_eq!(identity["identity_decision"]["require_trust"], true);
    assert_eq!(identity["identity_decision"]["require_max_turns"], true);
    assert_eq!(
        identity["identity_decision"]["selected_output"],
        "streaming"
    );
    assert_eq!(identity["identity_decision"]["claim_change_in_card"], false);

    let protocol: Value = serde_json::from_str(PROTOCOL).expect("protocol fixture");
    assert_eq!(
        protocol["authority"]["swallowtail_passes_auto_approve"],
        false
    );
    assert_eq!(protocol["authority"]["swallowtail_passes_yolo"], false);
    assert_eq!(protocol["authority"]["swallowtail_passes_agent_plan"], true);
    assert_eq!(protocol["authority"]["require_trust"], true);
    assert_eq!(protocol["acp_is_separate_binary"], true);
    assert_eq!(
        protocol["docs_programmatic_auto_approve_default"],
        serde_json::Value::Null
    );
    assert_eq!(
        protocol["authority"]["docs_programmatic_auto_approve_default"],
        "stale"
    );

    assert!(mistral_vibe_release_binding(MISTRAL_VIBE_RELEASE_VERSION).is_some());
    let claim = mistral_vibe_headless_claim();
    let version = InterfaceVersion::new(MISTRAL_VIBE_RELEASE_VERSION).expect("qualified version");
    assert!(claim.assess(&version).is_permitted());
    assert!(
        !claim
            .assess(&InterfaceVersion::new("2.24.3").expect("newer"))
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
    assert_eq!(argv[0], "vibe");
    assert!(argv.contains(&"--prompt"));
    assert!(argv.contains(&"--output"));
    assert!(argv.contains(&"streaming"));
    assert!(argv.contains(&"--trust"));
    assert!(argv.contains(&"--agent"));
    assert!(argv.contains(&"plan"));
    assert!(argv.contains(&"--max-turns"));
    for forbidden in command["forbidden_argv"]
        .as_array()
        .expect("forbidden argv")
        .iter()
        .map(|value| value.as_str().expect("flag"))
    {
        assert!(
            !argv.contains(&forbidden),
            "{forbidden} must not appear on the headless example argv"
        );
    }

    let plan: Value = serde_json::from_str(CORPUS_PLAN).expect("corpus plan");
    assert_eq!(plan["route"], "mistral-vibe.headless");
    assert_eq!(plan["no_production_claim_in_card"], 274);
    assert_eq!(plan["create_package_in_card"], 275);

    let negatives: Value = serde_json::from_str(NEGATIVE).expect("negatives");
    let ids: Vec<_> = negatives["cases"]
        .as_array()
        .expect("cases")
        .iter()
        .map(|case| case["id"].as_str().expect("id"))
        .collect();
    for required in [
        "vibe-acp-binary-is-not-headless",
        "acp-jsonrpc-is-not-headless",
        "json-dump-array-wrong-wire",
        "auto-approve-not-swallowtail-authority",
        "yolo-not-swallowtail-authority",
        "omit-agent-inherits-default-agent",
        "omit-trust",
        "omit-max-turns-unbounded",
        "continue-unmapped",
        "teleport-unmapped",
        "malformed-json",
        "oversized-frame",
        "missing-prompt-fails",
    ] {
        assert!(ids.contains(&required), "missing negative case {required}");
    }
}

#[test]
fn success_stream_is_completed_public_history_not_acp() {
    let types = jsonl_types(SUCCESS);
    assert_eq!(
        types.iter().map(String::as_str).collect::<Vec<_>>(),
        ["message", "message"]
    );
    for line in SUCCESS.lines().filter(|line| !line.is_empty()) {
        let value: Value = serde_json::from_str(line).expect("success line");
        assert_eq!(value["generationStatus"], "completed");
        assert_ne!(value.get("jsonrpc"), Some(&Value::from("2.0")));
    }

    let abort_lines: Vec<_> = ABORT.lines().filter(|line| !line.is_empty()).collect();
    assert!(serde_json::from_str::<Value>(abort_lines[0]).is_ok());
    assert!(serde_json::from_str::<Value>(abort_lines[1]).is_err());

    assert!(STDERR.contains("No prompt provided for programmatic mode"));
    assert!(LIMIT.contains("The configured conversation limit was reached"));

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
            .any(|row| row["case"] == "json-dump-array-wrong-wire")
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
