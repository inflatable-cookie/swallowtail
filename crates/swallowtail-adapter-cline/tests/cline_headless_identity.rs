use serde_json::Value;
use swallowtail_adapter_cline::{
    CLINE_PACKAGE_VERSION, cline_headless_claim, cline_package_binding,
};
use swallowtail_core::InterfaceVersion;

const IDENTITY: &str = include_str!("fixtures/cline-headless-3.0.55/identity.json");
const PROTOCOL: &str = include_str!("fixtures/cline-headless-3.0.55/protocol.json");
const COMMAND: &str = include_str!("fixtures/cline-headless-3.0.55/command.json");
const SUCCESS: &str = include_str!("fixtures/cline-headless-3.0.55/success.jsonl");
const ABORT: &str = include_str!("fixtures/cline-headless-3.0.55/abort.jsonl");
const STDERR: &str = include_str!("fixtures/cline-headless-3.0.55/stderr-error.jsonl");
const ACTIVITY: &str = include_str!("fixtures/cline-headless-3.0.55/activity.jsonl");
const NEGATIVE: &str = include_str!("fixtures/cline-headless-3.0.55/negative-cases.json");
const CORPUS_PLAN: &str = include_str!("fixtures/cline-headless-3.0.55/corpus-plan.json");

#[test]
fn frozen_identity_keeps_headless_json_separate_from_acp_and_auto_approve() {
    let identity: Value = serde_json::from_str(IDENTITY).expect("identity fixture");
    assert_eq!(identity["axis"], "cline.package");
    assert_eq!(identity["route"], "cline.headless");
    assert_eq!(identity["official"]["version"], CLINE_PACKAGE_VERSION);
    assert_eq!(identity["identity_decision"]["flatten_onto_acp"], false);
    assert_eq!(
        identity["identity_decision"]["flatten_onto_docs_ask_say_schema"],
        false
    );
    assert_eq!(
        identity["identity_decision"]["pass_auto_approve_true"],
        false
    );
    assert_eq!(
        identity["identity_decision"]["pass_auto_approve_false"],
        true
    );
    assert_eq!(identity["identity_decision"]["map_id_resume"], false);
    assert_eq!(identity["identity_decision"]["claim_change_in_card"], false);
    assert_eq!(
        identity["identity_decision"]["entrypoint"],
        serde_json::json!(["cline", "--json", "--auto-approve", "false"])
    );

    let protocol: Value = serde_json::from_str(PROTOCOL).expect("protocol fixture");
    assert_eq!(
        protocol["selected_command"],
        serde_json::json!(["cline", "--json", "--auto-approve", "false"])
    );
    assert_eq!(
        protocol["docs_ask_say_schema"],
        "stale-documentation-not-this-wire"
    );
    assert_eq!(
        protocol["authority"]["swallowtail_passes_auto_approve_true"],
        false
    );
    assert_eq!(
        protocol["authority"]["swallowtail_passes_auto_approve_false"],
        true
    );
    assert_eq!(protocol["acp_is_mutually_exclusive"], true);

    assert!(cline_package_binding(CLINE_PACKAGE_VERSION).is_some());
    let claim = cline_headless_claim();
    let version = InterfaceVersion::new(CLINE_PACKAGE_VERSION).expect("qualified version");
    assert!(claim.assess(&version).is_permitted());
    assert!(
        !claim
            .assess(&InterfaceVersion::new("3.0.56").expect("newer"))
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
    assert_eq!(argv[0], "cline");
    assert!(argv.contains(&"--json"));
    assert!(argv.contains(&"--auto-approve"));
    assert!(argv.contains(&"false"));
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
    assert!(
        !argv
            .windows(2)
            .any(|pair| pair == ["--auto-approve", "true"]),
        "example argv must not pass --auto-approve true"
    );
    assert!(
        !argv.contains(&"--plan"),
        "omit example argv must not place --plan"
    );

    let plan_argv = command["plan_example_argv"]
        .as_array()
        .expect("plan example argv")
        .iter()
        .map(|value| value.as_str().expect("argv text"))
        .collect::<Vec<_>>();
    assert_eq!(
        plan_argv,
        [
            "cline",
            "--json",
            "--auto-approve",
            "false",
            "--plan",
            "-c",
            "opaque-working-resource",
            "opaque fixture prompt"
        ]
    );
    for forbidden in ["--acp", "--id", "--yolo", "--zen", "-p"] {
        assert!(
            !plan_argv.contains(&forbidden),
            "{forbidden} must not appear on the headless Plan example argv"
        );
    }

    let plan: Value = serde_json::from_str(CORPUS_PLAN).expect("corpus plan");
    assert_eq!(plan["route"], "cline.headless");
    assert_eq!(plan["no_production_claim_in_card"], 304);
    assert_eq!(plan["no_headless_claim_until_card"], 305);

    let negatives: Value = serde_json::from_str(NEGATIVE).expect("negatives");
    let ids: Vec<_> = negatives["cases"]
        .as_array()
        .expect("cases")
        .iter()
        .map(|case| case["id"].as_str().expect("id"))
        .collect();
    for required in [
        "acp-flag-is-not-headless",
        "json-plus-acp-is-acp",
        "auto-approve-true-not-default",
        "omit-auto-approve-inherits-cli-true",
        "id-resume-unmapped",
        "docs-ask-say-wrong-wire",
        "acp-jsonrpc-is-not-headless",
        "malformed-json",
        "oversized-frame",
        "missing-prompt-fails",
    ] {
        assert!(ids.contains(&required), "missing negative case {required}");
    }
}

#[test]
fn success_and_abort_streams_are_envelope_ndjson_not_ask_say() {
    let types = jsonl_types(SUCCESS);
    assert_eq!(
        types.iter().map(String::as_str).collect::<Vec<_>>(),
        [
            "run_start",
            "agent_event",
            "agent_event",
            "agent_event",
            "run_result"
        ]
    );
    for line in SUCCESS.lines().filter(|line| !line.is_empty()) {
        let value: Value = serde_json::from_str(line).expect("success line");
        assert!(value.get("ts").is_some(), "emitJsonLine always writes ts");
        assert_ne!(value["type"], "ask");
        assert_ne!(value["type"], "say");
        assert_ne!(value.get("jsonrpc"), Some(&Value::from("2.0")));
    }

    let abort_types = jsonl_types(ABORT);
    assert_eq!(
        abort_types.iter().map(String::as_str).collect::<Vec<_>>(),
        ["run_start", "run_abort_requested", "run_aborted"]
    );

    let stderr: Value =
        serde_json::from_str(STDERR.lines().next().expect("stderr line")).expect("stderr envelope");
    assert_eq!(stderr["type"], "error");

    let activity_cases: Vec<_> = ACTIVITY
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_str::<Value>(line).expect("activity line"))
        .collect();
    assert!(
        activity_cases
            .iter()
            .any(|row| row["case"] == "docs-ask-say-wrong-wire")
    );
    assert!(
        activity_cases
            .iter()
            .any(|row| row["case"] == "team-event-unmapped")
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
