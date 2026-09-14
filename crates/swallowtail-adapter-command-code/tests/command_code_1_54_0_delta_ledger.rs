use serde_json::Value;

const IDENTITY: &str = include_str!("fixtures/command-code-1.54.0/identity.json");
const INVENTORY: &str = include_str!("fixtures/command-code-1.54.0/dist-inventory.json");
const PROTOCOL: &str = include_str!("fixtures/command-code-1.54.0/protocol.json");

#[test]
fn identity_freezes_the_baseline_and_every_published_successor() {
    let identity: Value = serde_json::from_str(IDENTITY).expect("identity fixture");
    let compared = identity["compared"].as_array().expect("compared versions");
    assert_eq!(compared.len(), 68);
    assert_eq!(compared.first().and_then(Value::as_str), Some("1.15.1"));
    assert_eq!(compared.last().and_then(Value::as_str), Some("1.54.0"));
    assert_eq!(identity["published_stable_count"], 67);
    assert_eq!(
        identity["published_stables_from_previous_ceiling"]
            .as_array()
            .expect("published successors")
            .len(),
        67
    );
    assert_eq!(identity["first_unpublished_later_stable"], "1.54.1");
    assert_eq!(identity["first_unpublished_verified"], true);
    assert_eq!(identity["official"]["version"], "1.54.0");
    assert_eq!(identity["official"]["file_count"], 71);
    assert_eq!(identity["official"]["engines_node"], ">=22");
    assert_eq!(identity["claim_at_observation"]["posture"], "QualifiedOnly");
    assert_eq!(
        identity["identity_decision"]["segment"],
        "compatible-extension"
    );
    assert_eq!(identity["identity_decision"]["raise_no_range"], true);
    assert_eq!(identity["live_evidence"]["transfers"], false);
}

#[test]
fn inventory_is_complete_and_has_no_unexpected_tree_additions_or_removals() {
    let inventory: Value = serde_json::from_str(INVENTORY).expect("inventory fixture");
    let compared = inventory["compared"].as_array().expect("compared versions");
    let counts = inventory["package_file_counts"]
        .as_object()
        .expect("package file counts");
    assert_eq!(compared.len(), 68);
    assert_eq!(counts.len(), compared.len());
    assert_eq!(inventory["from_hop_to_hop"].as_object().unwrap().len(), 67);
    assert_eq!(inventory["hashes"].as_object().unwrap().len(), 5);

    let additions = inventory["from_hop_to_hop"]
        .as_object()
        .unwrap()
        .values()
        .flat_map(|diff| diff["added"].as_array().unwrap())
        .map(|value| value.as_str().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(
        additions,
        [
            "dist/bundled/config/references/permissions.md",
            "dist/bundled/config/references/settings.md",
            "dist/bundled/config/SKILL.md",
            "dist/bundled/command-code-knowledge/reference/plans.md",
            "dist/bundled/command-code-knowledge/reference/tools.md",
            "dist/bundled/design/references/accessibility.md",
            "dist/bundled/design/references/severity.md",
            "dist/bundled/command-code-knowledge/reference/byok.md",
        ]
    );
    assert!(
        inventory["from_hop_to_hop"]
            .as_object()
            .unwrap()
            .values()
            .all(|diff| diff["removed"].as_array().unwrap().is_empty())
    );
    assert_eq!(
        inventory["hashes"]["dist/index.mjs"]["1.15.1"],
        inventory["hashes"]["dist/index.mjs"]["1.54.0"]
    );
}

#[test]
fn selected_protocol_keys_are_present_on_every_hop_and_unmapped_additions_stay_bounded() {
    let protocol: Value = serde_json::from_str(PROTOCOL).expect("protocol fixture");
    assert_eq!(protocol["artifact_revision"], "1.54.0");
    assert_eq!(protocol["route_id"], "command-code.headless");
    assert_eq!(
        protocol["protocol_facade_revision"],
        "command-code.agent-event-ndjson-v1"
    );
    assert!(
        protocol["selected_presence_all_hops"]
            .as_object()
            .unwrap()
            .values()
            .all(|value| value == true)
    );
    assert_eq!(
        protocol["selected_invocation"]["argv"],
        serde_json::json!([
            "-p",
            "--output-format",
            "json",
            "--permission-mode",
            "plan",
            "--skip-onboarding",
            "--no-session",
            "--no-auto-update",
            "--trust",
            "--no-skills",
            "--max-turns",
            "8",
            "-m",
            "<model>"
        ])
    );
    assert_eq!(
        protocol["unmapped_boundaries"]["cli_options"],
        serde_json::json!([
            "--tools-all",
            "--tools-enable <names>",
            "--local-only",
            "--effort",
            "--config",
            "--skill",
            "--mod",
            "--mod-option",
            "--ide-setup",
            "--worktree",
            "--session",
            "--continue",
            "--fork-session"
        ])
    );
    assert_eq!(
        protocol["live_evidence_disposition"]["live_authenticated_completion"],
        false
    );
    assert_eq!(
        protocol["live_evidence_disposition"]["live_tool_lifecycle"],
        false
    );
    assert_eq!(protocol["live_evidence_disposition"]["live_usage"], false);
    assert_eq!(
        protocol["live_evidence_disposition"]["live_interactive_continuation"],
        false
    );
}
