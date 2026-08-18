use serde_json::Value;
use std::collections::BTreeSet;
use swallowtail_adapter_claude_agent::{
    CLAUDE_AGENT_ACP_LATEST_QUALIFIED_VERSION, claude_agent_acp_claim,
};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

const CORPUS: &str = include_str!("fixtures/claude-agent-acp-v0.62.0-v0.64.0/compatibility.json");

fn corpus() -> Value {
    serde_json::from_str(CORPUS).expect("Claude Agent compatibility corpus is valid JSON")
}

fn records(corpus: &Value) -> &[Value] {
    corpus["stable_versions"]
        .as_array()
        .expect("stable versions are an array")
}

#[test]
fn corpus_freezes_exact_package_and_selected_source_identity() {
    let corpus = corpus();
    let records = records(&corpus);
    let versions = records
        .iter()
        .map(|record| record["version"].as_str().expect("version is text"))
        .collect::<Vec<_>>();

    assert_eq!(versions, ["0.61.0", "0.62.0", "0.63.0", "0.64.0"]);
    assert_eq!(versions.iter().copied().collect::<BTreeSet<_>>().len(), 4);
    assert!(records.iter().all(|record| {
        let commit = record["commit"].as_str().expect("commit is text");
        commit.len() == 40
            && commit.bytes().all(|byte| byte.is_ascii_hexdigit())
            && [
                "tarball_sha256",
                "package_json_sha256",
                "acp_agent_sha256",
                "elicitation_sha256",
                "tools_sha256",
            ]
            .iter()
            .all(|field| {
                let digest = record[field].as_str().expect("digest is text");
                digest.len() == 64 && digest.bytes().all(|byte| byte.is_ascii_hexdigit())
            })
    }));
}

#[test]
fn corpus_distinguishes_the_two_private_behavior_milestones() {
    let corpus = corpus();
    let records = records(&corpus);

    assert_eq!(
        records[0]["acp_agent_sha256"],
        records[1]["acp_agent_sha256"]
    );
    assert_eq!(
        records[0]["elicitation_sha256"],
        records[1]["elicitation_sha256"]
    );
    assert_eq!(records[0]["tools_sha256"], records[1]["tools_sha256"]);
    assert_eq!(records[1]["selected_output_matches_prior"], true);

    assert_eq!(records[2]["tool_subagent_correlation"], true);
    assert_eq!(records[2]["host_steering_option"], false);
    assert_eq!(records[2]["custom_answer_form_marker"], false);

    assert_eq!(records[3]["tool_subagent_correlation"], true);
    assert_eq!(records[3]["host_steering_option"], true);
    assert_eq!(records[3]["custom_answer_form_marker"], true);
}

#[test]
fn corpus_keeps_new_upstream_options_outside_swallowtail_authority() {
    let corpus = corpus();
    assert_eq!(corpus["access_posture"]["profiles_remain_separate"], true);
    assert_eq!(
        corpus["selected_absences"],
        serde_json::json!([
            "nested_transcript_capability",
            "host_owned_steering_fallback"
        ])
    );
}

#[test]
fn production_claim_keeps_the_frozen_corpus_inside_the_raised_window() {
    assert_eq!(CLAUDE_AGENT_ACP_LATEST_QUALIFIED_VERSION, "0.69.0");
    let claim = claude_agent_acp_claim();
    for candidate in ["0.62.0", "0.63.0", "0.64.0", "0.69.0"] {
        assert!(claim.supports(&version(candidate)));
    }
    assert!(matches!(
        claim.assess(&version("0.70.0")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}
