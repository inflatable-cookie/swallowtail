mod legacy_corpus_support;

use legacy_corpus_support::{
    APP_SERVER_RELEASES, EXEC_RELEASES, assert_no_private_fields, assert_release_evidence, json,
    release_versions, releases_by_version, segments_by_id, strings,
};
use serde_json::Value;
use std::collections::BTreeSet;

#[test]
fn legacy_exec_corpus_freezes_every_behavior_segment() {
    let corpus = json(EXEC_RELEASES);
    assert_eq!(corpus["claim_state"], "evidence-only");
    assert_eq!(
        release_versions(&corpus),
        [
            "0.80.0", "0.81.0", "0.84.0", "0.94.0", "0.98.0", "0.99.0", "0.100.0", "0.110.0",
            "0.121.0", "0.122.0"
        ]
    );
    assert_eq!(
        strings(&corpus["common_arguments"]),
        [
            "exec",
            "--json",
            "--color",
            "never",
            "--skip-git-repo-check",
            "--sandbox",
            "read-only",
            "--model",
            "<model>",
            "--config",
            "approval_policy=\"never\"",
            "--config",
            "shell_environment_policy.inherit=\"none\"",
            "--config",
            "hide_agent_reasoning=false",
            "--config",
            "show_raw_agent_reasoning=false",
            "-"
        ]
    );

    let releases = releases_by_version(&corpus);
    let segments = segments_by_id(&corpus);
    for (version, expected_segment) in [
        ("0.80.0", "legacy-retained-boolean-search"),
        ("0.81.0", "legacy-retained-boolean-search"),
        ("0.84.0", "legacy-retained-search-mode"),
        ("0.94.0", "legacy-retained-search-mode"),
        ("0.98.0", "legacy-retained-search-mode"),
        ("0.99.0", "legacy-ephemeral-ambient"),
        ("0.100.0", "legacy-ephemeral-ambient"),
        ("0.110.0", "legacy-ephemeral-ambient"),
        ("0.121.0", "legacy-ephemeral-ambient"),
        ("0.122.0", "current-ephemeral-suppressed"),
    ] {
        let release = releases.get(version).expect("checkpoint is frozen");
        assert_eq!(release["segment"], expected_segment);
        assert_release_evidence(release);
        let segment = segments
            .get(expected_segment)
            .expect("release segment is frozen");
        assert!(
            strings(&segment["checkpoints"]).contains(&version),
            "{version} missing from its segment checkpoints"
        );
    }
    assert_eq!(releases["0.98.0"]["npm_artifact_version"], "0.98.0");
    assert_eq!(
        releases["0.99.0"]["npm_artifact_version"],
        "0.99.0-darwin-arm64"
    );

    assert_exec_segment(
        segments["legacy-retained-boolean-search"],
        "ambient",
        "durable-local-accepted",
        &[],
        &["--ephemeral", "--ignore-user-config", "--ignore-rules"],
        "features.web_search_request",
    );
    assert_exec_segment(
        segments["legacy-retained-search-mode"],
        "ambient",
        "durable-local-accepted",
        &[],
        &["--ephemeral", "--ignore-user-config", "--ignore-rules"],
        "web_search",
    );
    assert_exec_segment(
        segments["legacy-ephemeral-ambient"],
        "ambient",
        "prohibited",
        &["--ephemeral"],
        &["--ignore-user-config", "--ignore-rules"],
        "web_search",
    );
    assert_exec_segment(
        segments["current-ephemeral-suppressed"],
        "provider-suppressed",
        "prohibited",
        &["--ephemeral", "--ignore-user-config", "--ignore-rules"],
        &[],
        "web_search",
    );
}

#[test]
fn legacy_corpora_reject_unpublished_and_unknown_points_without_private_payload() {
    for corpus in [json(EXEC_RELEASES), json(APP_SERVER_RELEASES)] {
        assert_eq!(
            strings(&corpus["diagnostic_fields"]),
            ["version", "segment", "schema_authority", "rejection_reason"]
        );
        let rejected: BTreeSet<_> = corpus["rejections"]
            .as_array()
            .expect("rejections are an array")
            .iter()
            .map(|entry| entry["version"].as_str().expect("version is text"))
            .collect();
        assert_eq!(
            rejected,
            BTreeSet::from([
                "0.82.0",
                "0.83.0",
                "0.108.0",
                "0.109.0",
                "0.146.0",
                "not-a-version"
            ])
        );
        assert_no_private_fields(&corpus);
        let serialized = serde_json::to_string(&corpus).expect("corpus serializes");
        for private_fragment in ["/Users/", "Bearer ", "sk-", "private payload"] {
            assert!(
                !serialized.contains(private_fragment),
                "fixture leaked private fragment {private_fragment}"
            );
        }
    }
}

fn assert_exec_segment(
    segment: &Value,
    configuration_posture: &str,
    retention: &str,
    required: &[&str],
    forbidden: &[&str],
    search_key: &str,
) {
    assert_eq!(segment["configuration_posture"], configuration_posture);
    assert_eq!(segment["retention"], retention);
    assert_eq!(strings(&segment["required_arguments"]), required);
    assert_eq!(strings(&segment["forbidden_arguments"]), forbidden);
    assert_eq!(segment["search_configuration"]["key"], search_key);
    for key in ["disabled", "enabled"] {
        assert!(
            segment["search_configuration"][key]
                .as_str()
                .expect("search value is text")
                .starts_with(search_key)
        );
    }
}
