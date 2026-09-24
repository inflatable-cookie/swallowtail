use serde_json::Value;
use swallowtail_adapter_qwen::qwen_headless_claim;
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

const IDENTITY: &str = include_str!("fixtures/qwen-code-0.24.2/identity.json");
const PROTOCOL: &str = include_str!("fixtures/qwen-code-0.24.2/protocol.json");
const DIST_INVENTORY: &str = include_str!("fixtures/qwen-code-0.24.2/dist-inventory.json");

fn json(source: &str) -> Value {
    serde_json::from_str(source).expect("Qwen Code 0.24.2 fixture is valid JSON")
}

#[test]
fn identity_freezes_official_hops_and_preserves_claim_boundary() {
    let identity = json(IDENTITY);
    let protocol = json(PROTOCOL);

    assert_eq!(identity["axis"], "qwen-code.package");
    assert_eq!(identity["npm_package"], "@qwen-code/qwen-code");
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(identity["host"]["not_installed"], true);
    assert_eq!(identity["official"]["version"], "0.24.2");
    assert_eq!(
        identity["official"]["github_commit"],
        "1026c4a50f4a32f77da98bdacfba2e5faa8cc70a"
    );
    assert_eq!(
        identity["published_stables_from_previous_ceiling"],
        serde_json::json!(["0.23.4", "0.24.0", "0.24.1", "0.24.2"])
    );
    assert_eq!(identity["unpublished_stable_0_20_2"], true);
    assert_eq!(identity["unpublished_0_21_16"], true);
    assert_eq!(identity["unpublished_0_22_4"], true);
    assert_eq!(identity["unpublished_0_23_5"], true);
    assert_eq!(identity["unpublished_0_24_3"], true);

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(
        decision["reuse_behavior_revision"],
        "qwen-code.headless.v0.21.15-reasoning-control"
    );
    assert_eq!(decision["raise_latest_qualified_to"], "0.24.2");
    assert_eq!(decision["extend_same_revision_segment"], "0.22.0..=0.24.2");
    assert_eq!(decision["keep_exact_0_21_15"], true);
    assert_eq!(
        decision["extend_plan_exact_list_to_include_0_24_hops"],
        false
    );
    assert_eq!(decision["extend_reasoning_beyond_0_21_15"], false);
    assert_eq!(decision["extend_budgets_beyond_0_21_15"], false);
    assert_eq!(decision["flatten_to_model_studio_or_acp"], false);
    assert_eq!(decision["new_public_mapped_operation"], false);
    assert_eq!(decision["provider_prompt_sent"], false);

    assert_eq!(protocol["selected_mapped_subset_unchanged"], true);
    assert_eq!(protocol["catalogue_image_only_filter"], true);
    assert_eq!(protocol["decoder_corpus"], "qwen-code-v0.19.11");
    assert_eq!(protocol["active_goal_removed_from"], "0.24.1");
    assert_eq!(
        protocol["0_24_2_plan_mode_blobs_byte_identical_to_0_22_3"],
        false
    );
    assert_eq!(
        protocol["0_24_2_plan_policy_and_exit_byte_identical_to_0_23_3"],
        true
    );
    for flag in [
        "--safe-mode",
        "--approval-mode",
        "--core-tools",
        "--exclude-tools",
        "--max-wall-time",
        "--max-tool-calls",
        "--max-session-turns",
        "--include-partial-messages",
        "--input-format",
        "--output-format",
        "--resume",
    ] {
        assert!(
            protocol["help_selected_flags_present"]
                .as_array()
                .expect("selected flags are an array")
                .iter()
                .any(|value| value == flag),
            "missing selected flag {flag}"
        );
    }
}

#[test]
fn production_claim_excludes_the_documented_unpublished_interior_gaps() {
    let decision = &json(IDENTITY)["identity_decision"];
    assert_eq!(decision["keep_0_22_4_incompatible"], true);
    assert_eq!(decision["keep_0_23_5_incompatible"], true);

    let claim = qwen_headless_claim();
    for point in [
        "0.22.0", "0.22.1", "0.22.2", "0.22.3", "0.23.0", "0.23.1", "0.23.2", "0.23.3", "0.23.4",
        "0.24.0", "0.24.1", "0.24.2",
    ] {
        assert!(
            claim.supports(&version(point)),
            "published stable {point} stays qualified"
        );
    }
    for gap in ["0.22.4", "0.23.5"] {
        assert_eq!(
            claim.assess(&version(gap)),
            InterfaceCompatibilityAssessment::Incompatible,
            "unpublished interior {gap} stays incompatible"
        );
    }
    assert!(matches!(
        claim.assess(&version("0.24.3")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
}

#[test]
fn complete_hop_inventory_has_mutation_sensitive_counts_and_hashes() {
    let inventory = json(DIST_INVENTORY);
    assert_eq!(
        inventory["compared"],
        serde_json::json!(["0.23.3", "0.23.4", "0.24.0", "0.24.1", "0.24.2"])
    );
    assert_eq!(
        inventory["package_file_counts"],
        serde_json::json!({
            "0.23.3": 1050,
            "0.23.4": 1067,
            "0.24.0": 1128,
            "0.24.1": 1267,
            "0.24.2": 1267
        })
    );

    let expected = [
        ("from_0_23_3_to_0_23_4", 377, 360, 37, 653),
        ("from_0_23_4_to_0_24_0", 449, 388, 31, 648),
        ("from_0_24_0_to_0_24_1", 507, 368, 27, 733),
        ("from_0_24_1_to_0_24_2", 489, 489, 29, 749),
    ];
    for (hop, added, removed, changed, identical) in expected {
        let entry = &inventory[hop];
        assert_eq!(entry["added"].as_array().expect("added paths").len(), added);
        assert_eq!(
            entry["removed"].as_array().expect("removed paths").len(),
            removed
        );
        assert_eq!(
            entry["changed"].as_array().expect("changed paths").len(),
            changed
        );
        assert_eq!(
            entry["identical"]
                .as_array()
                .expect("identical paths")
                .len(),
            identical
        );

        let mut all = Vec::new();
        for key in ["added", "removed", "changed", "identical"] {
            all.extend(
                entry[key]
                    .as_array()
                    .expect("inventory category is an array")
                    .iter()
                    .map(|path| path.as_str().expect("inventory path is text")),
            );
        }
        all.sort_unstable();
        all.dedup();
        assert_eq!(all.len(), added + removed + changed + identical);
    }

    assert_eq!(
        inventory["hashes"]["cli.js"]["0.24.2"],
        "01ec0c6cb09f1f04caf0da15d45cd70e56191778365d93e198d502213d774f7f"
    );
    assert_eq!(
        inventory["hashes"]["cli-entry.js"]["0.24.2"],
        "68cb29eb7ccc936d78ece5564ef55cae41a55b630e6657dc417c1f2e561cf4c9"
    );
    assert_eq!(
        inventory["hashes"]["package.json"]["0.24.2"],
        "422bcccbd36bc8c6eb3b9308f0825a3fc0fddb2f47c41a3451186c030b66bb45"
    );
    for version in ["0.23.3", "0.23.4", "0.24.0", "0.24.1", "0.24.2"] {
        assert_eq!(
            inventory["hashes"]["cli-entry.js"][version],
            "68cb29eb7ccc936d78ece5564ef55cae41a55b630e6657dc417c1f2e561cf4c9"
        );
    }
    assert_eq!(inventory["not_a_complete_semantic_changelog"], true);
    assert!(
        inventory["named_unmapped_with_reason"]
            .as_object()
            .unwrap()
            .len()
            >= 10
    );
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}
