use serde_json::Value;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use swallowtail_adapter_ollama::{OLLAMA_BASELINE_VERSION, OLLAMA_LATEST_QUALIFIED_VERSION};

const ROOT: &str = "tests/fixtures/ollama-native-v0.14.0-v0.32.1";

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

fn json_lines(name: &str) -> Vec<Value> {
    text(name)
        .lines()
        .map(|line| serde_json::from_str(line).expect("fixture line is valid JSON"))
        .collect()
}

#[test]
fn interactive_corpus_covers_the_maintained_window_and_exclusion() {
    let corpus = json("interactive-session.json");
    let points: BTreeSet<_> = corpus["qualification_points"]
        .as_array()
        .expect("qualification points are an array")
        .iter()
        .map(|point| point["version"].as_str().expect("version is text"))
        .collect();

    assert_eq!(
        points,
        BTreeSet::from(["0.14.0", "0.18.0", "0.30.0", "0.32.1"])
    );
    assert!(points.contains(OLLAMA_BASELINE_VERSION));
    assert_eq!(OLLAMA_LATEST_QUALIFIED_VERSION, "0.32.14");
    assert_eq!(
        corpus["excluded_versions"],
        serde_json::json!(["0.32.2", "0.32.10"])
    );
    assert_eq!(corpus["behavior_revision"], "ollama.native-text-v1");
}

#[test]
fn second_request_replays_only_committed_ordered_history() {
    let first = json("interactive-turn-1-request.json");
    let second = json("interactive-turn-2-request.json");
    let first_messages = first["messages"].as_array().expect("messages are an array");
    let second_messages = second["messages"]
        .as_array()
        .expect("messages are an array");

    assert_eq!(first_messages.len(), 1);
    assert_eq!(second_messages.len(), 3);
    assert_eq!(second_messages[0], first_messages[0]);
    assert_eq!(second_messages[1]["role"], "assistant");
    assert_eq!(second_messages[1]["content"], "First answer");
    assert_eq!(second_messages[2]["role"], "user");
    assert_eq!(second["model"], first["model"]);
    assert_eq!(second["options"], first["options"]);
    assert_eq!(second["stream"], true);
}

#[test]
fn success_and_failure_streams_freeze_transaction_boundaries() {
    for fixture in [
        "interactive-turn-1-success.ndjson",
        "interactive-turn-2-success.ndjson",
    ] {
        let events = json_lines(fixture);
        assert!(events.len() >= 2);
        assert_eq!(events.last().unwrap()["done"], true);
        assert!(events.last().unwrap()["prompt_eval_count"].is_number());
        assert!(events.last().unwrap()["eval_count"].is_number());
    }

    let failure = json_lines("interactive-turn-2-error.ndjson");
    assert_eq!(failure[0]["done"], false);
    assert!(failure.last().unwrap()["error"].is_string());

    let corpus = json("interactive-session.json");
    assert_eq!(
        corpus["transaction"]["success_commit"],
        "candidate_user_plus_complete_assistant"
    );
    for key in [
        "provider_failure_commit",
        "protocol_failure_commit",
        "disconnect_commit",
        "cancellation_commit",
        "deadline_commit",
    ] {
        assert_eq!(corpus["transaction"][key], "unchanged");
    }
    assert_eq!(corpus["transaction"]["automatic_retry"], false);
}

#[test]
fn bounds_and_cleanup_preserve_attached_runtime_authority() {
    let corpus = json("interactive-session.json");
    let bounds = &corpus["bounds"];
    let capabilities = &corpus["capabilities"];

    assert_eq!(bounds["maximum_turns"], 24);
    assert_eq!(bounds["maximum_messages"], 48);
    assert_eq!(bounds["maximum_private_history_bytes"], 1_048_576);
    assert_eq!(bounds["maximum_request_bytes"], 1_048_576);
    assert_eq!(bounds["maximum_stream_records_per_turn"], 4096);
    assert_eq!(bounds["maximum_line_bytes"], 1_048_576);
    assert_eq!(bounds["maximum_output_tokens_per_turn"], 8);

    for key in [
        "public_load_session",
        "public_resume_session",
        "provider_session_reference",
        "native_session_close",
        "consumer_tool_exchange",
        "attachments",
        "realtime_media",
        "billed_cost",
        "owned_runtime_lifecycle",
    ] {
        assert_eq!(capabilities[key], false);
    }
    assert_eq!(corpus["cleanup"]["private_transcript_cleared"], true);
    assert_eq!(corpus["cleanup"]["attached_runtime_stopped"], false);
    assert_eq!(corpus["cleanup"]["model_unloaded"], false);
    assert_eq!(
        corpus["cleanup"]["runtime_managed_residency_preserved"],
        true
    );
}
