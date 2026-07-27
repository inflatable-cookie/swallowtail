mod support;

use serde_json::json;
use support::{Direction, methods, parse_json, parse_transcript};

const ROOT: &str = "fixtures/acp-v1-claude-agent-acp-0.53.0-0.61.0";
const LIFECYCLE: &str =
    include_str!("fixtures/acp-v1-claude-agent-acp-0.53.0-0.61.0/lifecycle-corpus.json");
const CLOSE_ACTIVE: &str =
    include_str!("fixtures/acp-v1-claude-agent-acp-0.53.0-0.61.0/close-active.ndjson");
const CLOSE_MISSING: &str =
    include_str!("fixtures/acp-v1-claude-agent-acp-0.53.0-0.61.0/close-missing.ndjson");
const DELETE_INACTIVE: &str =
    include_str!("fixtures/acp-v1-claude-agent-acp-0.53.0-0.61.0/delete-inactive.ndjson");
const DELETE_ACTIVE: &str =
    include_str!("fixtures/acp-v1-claude-agent-acp-0.53.0-0.61.0/delete-active.ndjson");
const DELETE_MISSING: &str =
    include_str!("fixtures/acp-v1-claude-agent-acp-0.53.0-0.61.0/delete-missing.ndjson");

#[test]
fn every_qualified_milestone_and_the_exclusion_freeze_lifecycle_source() {
    let corpus = parse_json(LIFECYCLE);
    let milestones = corpus["milestones"].as_array().expect("milestones");
    assert_eq!(
        milestones
            .iter()
            .map(|point| point["adapter"].as_str().expect("adapter"))
            .collect::<Vec<_>>(),
        ["0.53.0", "0.54.0", "0.58.0", "0.60.0", "0.61.0"]
    );
    assert_eq!(milestones[2]["publication"], "excluded_unpublished_package");

    for point in milestones {
        for digest in [
            "acp_sdk_archive_sha256",
            "agent_sdk_archive_sha256",
            "agent_source_sha256",
            "agent_tests_sha256",
        ] {
            assert_eq!(
                point[digest].as_str().expect("digest").len(),
                64,
                "{} {digest}",
                point["adapter"]
            );
        }
    }
}

#[test]
fn current_newer_release_does_not_extend_the_qualified_range() {
    let corpus = parse_json(LIFECYCLE);
    assert_eq!(corpus["qualified_range"]["baseline"], "0.53.0");
    assert_eq!(corpus["qualified_range"]["latest"], "0.61.0");
    assert_eq!(
        corpus["qualified_range"]["newer_observed"]["version"],
        "0.62.0"
    );
    assert_eq!(
        corpus["qualified_range"]["newer_observed"]["qualification"],
        "unverified_newer_not_range_evidence"
    );
}

#[test]
fn close_is_active_only_preserves_history_and_is_not_idempotent() {
    let active = parse_transcript(CLOSE_ACTIVE).expect("active close parses");
    let missing = parse_transcript(CLOSE_MISSING).expect("missing close parses");
    assert_eq!(methods(&active), ["session/close"]);
    assert_eq!(active[0].direction(), Direction::ClientToAgent);
    assert_eq!(active[1].direction(), Direction::AgentToClient);
    assert_eq!(active[0].id(), active[1].id());
    assert_eq!(active[1].message()["result"], json!({}));
    assert_eq!(methods(&missing), ["session/close"]);
    assert_eq!(missing[0].id(), missing[1].id());
    assert_eq!(missing[1].message()["error"]["code"], -32603);

    let close = &parse_json(LIFECYCLE)["behavior"]["close"];
    assert_eq!(
        close["active"],
        "interrupt_abort_dispose_remove_in_memory_return_empty"
    );
    assert_eq!(close["persistent_history"], "preserved");
    assert_eq!(close["missing"], "internal_error");
    assert_eq!(close["idempotent"], false);
}

#[test]
fn delete_handles_active_and_inactive_targets_but_missing_is_an_error() {
    for transcript in [DELETE_INACTIVE, DELETE_ACTIVE] {
        let frames = parse_transcript(transcript).expect("delete success parses");
        assert_eq!(methods(&frames), ["session/delete"]);
        assert_eq!(frames[0].id(), frames[1].id());
        assert_eq!(frames[1].message()["result"], json!({}));
    }

    let missing = parse_transcript(DELETE_MISSING).expect("missing delete parses");
    assert_eq!(methods(&missing), ["session/delete"]);
    assert_eq!(missing[0].id(), missing[1].id());
    assert_eq!(missing[1].message()["error"]["code"], -32603);
    assert_eq!(missing[1].message()["error"]["message"], "Internal error");

    let delete = &parse_json(LIFECYCLE)["behavior"]["delete"];
    assert_eq!(delete["active"], "teardown_then_sdk_delete");
    assert_eq!(delete["inactive"], "sdk_delete");
    assert_eq!(delete["missing"], "internal_error");
    assert_eq!(delete["repeated"], "internal_error");
    assert_eq!(delete["idempotent"], false);
}

#[test]
fn claude_deletes_harness_session_data_without_claiming_hard_or_api_erasure() {
    let corpus = parse_json(LIFECYCLE);
    let delete = &corpus["behavior"]["delete"];
    assert_eq!(delete["deletion_strength"], "provider_data_deleted");
    assert_eq!(delete["affected_scope"], "provider_defined_descendants");
    assert_eq!(
        delete["deleted_data"],
        json!([
            "primary_nonempty_session_jsonl",
            "sibling_session_directory_recursively"
        ])
    );
    let excluded = delete["not_claimed"].as_array().expect("exclusions");
    for claim in [
        "secure_erasure",
        "anthropic_api_service_data_deletion",
        "account_analytics_deletion",
        "backup_deletion",
    ] {
        assert!(excluded.iter().any(|candidate| candidate == claim));
    }

    let sdk = &corpus["sdk_delete_implementation"];
    assert_eq!(sdk["consistent_across_pinned_sdk_points"], true);
    assert_eq!(sdk["removes_primary_jsonl"], true);
    assert_eq!(sdk["removes_sibling_session_directory_recursively"], true);
    assert_eq!(sdk["missing_target_throws"], true);
}

#[test]
fn lifecycle_fixtures_are_bounded_and_safe() {
    for fixture in [
        LIFECYCLE,
        CLOSE_ACTIVE,
        CLOSE_MISSING,
        DELETE_INACTIVE,
        DELETE_ACTIVE,
        DELETE_MISSING,
    ] {
        assert!(fixture.len() < 256 * 1024);
        for forbidden in [
            "/Users/",
            "ANTHROPIC_API_KEY",
            "sk-ant-",
            "Bearer ",
            "Toms-MacBook-Pro",
        ] {
            assert!(!fixture.contains(forbidden), "{ROOT} leaked {forbidden}");
        }
    }
}
