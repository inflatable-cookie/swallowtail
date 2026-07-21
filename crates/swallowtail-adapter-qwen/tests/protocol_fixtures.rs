use serde_json::Value;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use swallowtail_adapter_qwen::{PINNED_QWEN_CODE_COMMIT, PINNED_QWEN_CODE_VERSION};

const ROOT: &str = "tests/fixtures/qwen-code-v0.19.11";
const PRIVATE_PROMPT: &str = "fixture-private-prompt";
const PRIVATE_PROVIDER_VALUE: &str = "fixture-provider-secret-never-diagnose";

fn path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(ROOT).join(name)
}

fn text(name: &str) -> String {
    std::fs::read_to_string(path(name))
        .unwrap_or_else(|error| panic!("failed to read {name}: {error}"))
}

fn json(name: &str) -> Value {
    serde_json::from_str(&text(name))
        .unwrap_or_else(|error| panic!("failed to parse {name}: {error}"))
}

fn json_lines(name: &str) -> Result<Vec<Value>, serde_json::Error> {
    text(name).lines().map(serde_json::from_str).collect()
}

fn strings(value: &Value) -> Vec<&str> {
    value
        .as_array()
        .expect("fixture field is an array")
        .iter()
        .map(|value| value.as_str().expect("fixture array contains strings"))
        .collect()
}

#[test]
fn source_and_route_are_exact_and_separately_identified() {
    let protocol = json("protocol.json");

    assert_eq!(protocol["source"]["version"], PINNED_QWEN_CODE_VERSION);
    assert_eq!(protocol["source"]["commit"], PINNED_QWEN_CODE_COMMIT);
    assert_eq!(protocol["route"]["execution_layer"], "harness_interaction");
    assert_eq!(protocol["route"]["operation_shape"], "structured_run");
    assert_eq!(
        protocol["route"]["model_route"],
        "required_exact_preflight_binding"
    );
    assert_eq!(protocol["route"]["provider_fallback"], false);
    assert_eq!(protocol["access"]["raw_credential_read"], false);
}

#[test]
fn invocation_uses_stdin_and_an_exact_read_only_registry_without_sandboxing() {
    let protocol = json("protocol.json");
    let arguments = strings(&protocol["invocation"]["arguments"]);
    let forbidden = strings(&protocol["invocation"]["forbidden_arguments"]);
    let allowlist: BTreeSet<_> = strings(&protocol["tools"]["registry_allowlist"])
        .into_iter()
        .collect();

    assert_eq!(protocol["invocation"]["prompt_transport"], "stdin_text");
    assert!(
        arguments
            .windows(2)
            .any(|pair| pair == ["--output-format", "stream-json"])
    );
    assert!(
        arguments
            .windows(2)
            .any(|pair| pair == ["--approval-mode", "default"])
    );
    assert!(arguments.contains(&"--safe-mode"));
    assert!(arguments.contains(&"--core-tools"));
    assert!(!arguments.contains(&PRIVATE_PROMPT));
    assert!(forbidden.iter().all(|flag| !arguments.contains(flag)));
    assert_eq!(
        allowlist,
        BTreeSet::from(["glob", "grep_search", "list_directory", "lsp", "read_file"])
    );
    for denied in [
        "agent",
        "edit",
        "run_shell_command",
        "web_fetch",
        "write_file",
    ] {
        assert!(!allowlist.contains(denied));
    }
    assert_eq!(protocol["isolation"]["posture"], "ambient_host");
    assert_eq!(protocol["isolation"]["provider_sandbox"], false);
    assert_eq!(protocol["isolation"]["container_required"], false);
    assert_eq!(protocol["isolation"]["safe_mode_implies_isolation"], false);
    assert_eq!(
        protocol["isolation"]["tool_policy_implies_isolation"],
        false
    );
}

#[test]
fn retention_and_native_bounds_do_not_acquire_host_lifecycle_authority() {
    let protocol = json("protocol.json");

    assert_eq!(protocol["retention"]["policy"], "durable_allowed");
    assert_eq!(
        protocol["retention"]["provider_behavior"],
        "project_scoped_local_jsonl"
    );
    assert_eq!(protocol["retention"]["resume_authority"], false);
    assert_eq!(protocol["retention"]["delete_authority"], false);
    assert_eq!(protocol["retention"]["exit_proves_deletion"], false);
    assert_eq!(protocol["native_budgets"]["wall_time"], "60s");
    assert_eq!(protocol["native_budgets"]["top_level_tool_calls"], 16);
    assert_eq!(protocol["native_budgets"]["session_turns"], 24);
    assert_eq!(protocol["native_budgets"]["host_deadline_replaced"], false);
}

#[test]
fn success_stream_freezes_partial_full_and_terminal_shapes() {
    let messages = json_lines("success.jsonl").expect("success stream is valid JSONL");
    let types: Vec<_> = messages
        .iter()
        .map(|message| message["type"].as_str().expect("message type is text"))
        .collect();

    assert_eq!(types.first(), Some(&"system"));
    assert!(types.contains(&"stream_event"));
    assert!(types.contains(&"assistant"));
    assert_eq!(types.last(), Some(&"result"));
    assert_eq!(messages.last().unwrap()["subtype"], "success");
    assert_eq!(messages.last().unwrap()["is_error"], false);
}

#[test]
fn provider_failure_unknown_event_and_malformed_line_remain_distinct() {
    let provider = json_lines("provider-failure.jsonl").expect("provider stream is valid JSONL");
    assert_eq!(
        provider.last().unwrap()["subtype"],
        "error_during_execution"
    );
    assert_eq!(provider.last().unwrap()["is_error"], true);

    let unknown = json_lines("unknown-event.jsonl").expect("unknown stream is valid JSONL");
    assert_eq!(unknown[1]["subtype"], "future_fixture_event");
    assert_eq!(
        json("protocol.json")["stream"]["unknown_event_policy"],
        "observe_and_continue"
    );

    assert!(json_lines("malformed.jsonl").is_err());
}

#[test]
fn process_exit_truth_and_public_diagnostics_are_bounded_and_redacted() {
    let observations = json("terminal-observations.json");
    let observations = observations.as_array().expect("observations are an array");
    let classes: BTreeSet<_> = observations
        .iter()
        .map(|item| item["terminal_class"].as_str().expect("class is text"))
        .collect();

    assert_eq!(
        observations
            .iter()
            .find(|item| item["case"] == "session_turn_limit")
            .unwrap()["exit_code"],
        53
    );
    assert_eq!(
        observations
            .iter()
            .find(|item| item["case"] == "native_budget")
            .unwrap()["exit_code"],
        55
    );
    assert_eq!(
        observations
            .iter()
            .find(|item| item["case"] == "host_cancellation")
            .unwrap()["exit_code"],
        130
    );
    assert!(classes.contains("provider_failure"));
    assert!(classes.contains("provider_native_turn_limit"));
    assert!(classes.contains("provider_native_budget"));
    assert!(classes.contains("cancelled"));
    assert!(classes.contains("protocol_failure"));

    let diagnostics = observations
        .iter()
        .filter_map(|item| item["public_diagnostic"].as_str())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(!diagnostics.contains(PRIVATE_PROVIDER_VALUE));
    assert!(!diagnostics.contains(PRIVATE_PROMPT));
    assert!(!text("protocol.json").contains(PRIVATE_PROMPT));
}
