use serde_json::Value;
use std::path::{Path, PathBuf};
use swallowtail_adapter_qwen::{PINNED_QWEN_CODE_COMMIT, PINNED_QWEN_CODE_VERSION};

const ROOT: &str = "tests/fixtures/qwen-code-v0.19.11";
const SESSION_ID: &str = "123e4567-e89b-12d3-a456-426614174000";

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
fn interactive_route_keeps_restarted_harness_continuity_exact() {
    let corpus = json("interactive-session.json");

    assert_eq!(corpus["source"]["version"], PINNED_QWEN_CODE_VERSION);
    assert_eq!(corpus["source"]["commit"], PINNED_QWEN_CODE_COMMIT);
    assert_eq!(corpus["route"]["operation_shape"], "interactive_session");
    assert_eq!(
        corpus["route"]["continuity_owner"],
        "harness_retained_project_transcript"
    );
    assert_eq!(
        corpus["route"]["turn_transport"],
        "one_owned_child_per_turn"
    );
    assert_eq!(
        corpus["route"]["provider_state_policy"],
        "durable_provider_session_preserved"
    );
    assert_eq!(corpus["bounds"]["maximum_turns"], 24);
    assert_eq!(corpus["bounds"]["maximum_stream_records_per_turn"], 4096);
    assert_eq!(corpus["bounds"]["maximum_line_bytes"], 1_048_576);
}

#[test]
fn first_turn_has_no_selector_and_continued_turn_uses_only_exact_resume() {
    let corpus = json("interactive-session.json");
    let first = corpus["invocation"]["first_turn_additional_arguments"]
        .as_array()
        .expect("first-turn arguments are an array");
    let continued = corpus["invocation"]["continued_turn_additional_arguments"]
        .as_array()
        .expect("continued-turn arguments are an array");
    let forbidden = corpus["invocation"]["forbidden_arguments"]
        .as_array()
        .expect("forbidden arguments are an array");

    assert!(first.is_empty());
    assert_eq!(continued[0], "--resume");
    assert_eq!(continued[1], "{exact_provider_session_id}");
    for argument in ["--continue", "--fork-session", "--session-id", "--sandbox"] {
        assert!(forbidden.iter().any(|value| value == argument));
        assert!(!continued.iter().any(|value| value == argument));
    }
}

#[test]
fn every_success_event_preserves_the_exact_provider_session() {
    for fixture in [
        "interactive-first-turn.jsonl",
        "interactive-continued-turn.jsonl",
    ] {
        let events = json_lines(fixture);
        assert!(!events.is_empty());
        assert!(events.iter().all(|event| event["session_id"] == SESSION_ID));
        assert_eq!(events.first().unwrap()["subtype"], "session_start");
        assert_eq!(events.last().unwrap()["subtype"], "success");
        assert_eq!(events.last().unwrap()["is_error"], false);
    }

    let mismatch = json_lines("interactive-session-mismatch.jsonl");
    assert!(
        mismatch
            .iter()
            .all(|event| event["session_id"] != SESSION_ID)
    );
}

#[test]
fn failure_and_cleanup_rules_do_not_mint_public_session_authority() {
    let corpus = json("interactive-session.json");
    let commit = &corpus["commit"];
    let capabilities = &corpus["capabilities"];

    for key in [
        "provider_failure_invalidates_session",
        "protocol_failure_invalidates_session",
        "provider_reference_mismatch_invalidates_session",
        "cancellation_invalidates_session",
        "deadline_invalidates_session",
        "cleanup_uncertainty_invalidates_session",
    ] {
        assert_eq!(commit[key], true);
    }
    for key in [
        "public_load_session",
        "public_resume_session",
        "provider_session_archive",
        "provider_session_restore",
        "provider_session_delete",
        "native_session_close",
        "consumer_tool_exchange",
        "workspace_write",
        "external_search",
        "sandbox_or_containment",
        "realtime_media",
        "billed_cost",
    ] {
        assert_eq!(capabilities[key], false);
    }
    assert_eq!(
        corpus["cleanup"]["turn_child_joined_before_next_turn"],
        true
    );
    assert_eq!(
        corpus["cleanup"]["session_close_preserves_provider_state"],
        true
    );
    assert_eq!(
        corpus["cleanup"]["session_close_deletes_provider_state"],
        false
    );
}
