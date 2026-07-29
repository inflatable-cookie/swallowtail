use super::support::{RANGE, assert_segments, json, strings};
use std::collections::BTreeSet;

#[test]
fn range_freezes_separate_app_server_and_exec_schema_segments() {
    let range = json(RANGE);
    assert_eq!(range["axis"], "codex.cli");
    assert_eq!(range["facade"], "codex-app-server-v2");
    assert_eq!(range["qualified_upper"], "0.145.0");
    assert_eq!(range["current_external_release"]["version"], "0.146.0");
    assert_eq!(
        range["current_external_release"]["classification"],
        "unverified-newer"
    );
    assert_eq!(
        range["current_external_release"]["execution"],
        "permitted-without-profile-widening"
    );

    assert_segments(
        &range["app_server_segments"],
        &[
            "core-items",
            "collaboration-item",
            "user-input-request",
            "dynamic-request-and-search-action",
            "plan-and-compaction-items",
            "assistant-message-phase",
            "dynamic-tool-item",
            "server-request-resolution",
            "image-generation-unknown-item",
            "permission-request-and-tool-metadata",
            "hook-lifecycle",
            "approval-review-lifecycle",
            "assistant-memory-citation-additive",
            "hook-prompt-item",
            "file-patch-snapshot",
            "item-timestamps-and-retired-file-output",
            "subagent-activity-item",
            "sleep-unknown-item",
            "mcp-app-context-additive",
            "extension-backed-search-item",
        ],
    );
    assert_segments(
        &range["exec_segments"],
        &[
            "jsonl-core",
            "jsonl-collaboration-and-search-action",
            "jsonl-reasoning-usage-additive",
            "jsonl-mcp-meta-additive",
            "jsonl-cache-write-usage-additive",
        ],
    );
}

#[test]
fn range_records_replacements_exclusions_and_raw_reasoning_boundary() {
    let range = json(RANGE);
    let replacements: BTreeSet<_> = range["replacement_milestones"]
        .as_array()
        .expect("replacement milestones are an array")
        .iter()
        .map(|entry| entry["version"].as_str().expect("version is text"))
        .collect();
    assert_eq!(
        replacements,
        BTreeSet::from(["0.93.0", "0.123.0", "0.129.0"])
    );

    let excluded = strings(&range["disclosure"]["excluded"]);
    assert!(excluded.contains("item/reasoning/textDelta"));
    assert!(excluded.contains("reasoning.content"));
    assert!(excluded.contains("raw-provider-envelope"));

    let version_exclusions: BTreeSet<_> = range["version_exclusions"]
        .as_array()
        .expect("version exclusions are an array")
        .iter()
        .map(|entry| entry["range"].as_str().expect("range is text"))
        .collect();
    assert_eq!(
        version_exclusions,
        BTreeSet::from([
            "0.82.0..=0.83.0",
            "0.108.0..=0.109.0",
            "0.146.0-alpha.4",
            "not-a-version",
        ])
    );
}
