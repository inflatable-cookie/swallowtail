use serde_json::Value;

const IDENTITY: &str = include_str!("fixtures/qwen-code-0.23.3/identity.json");
const PROTOCOL: &str = include_str!("fixtures/qwen-code-0.23.3/protocol.json");
const DIST_INVENTORY: &str = include_str!("fixtures/qwen-code-0.23.3/dist-inventory.json");

fn json(source: &str) -> Value {
    serde_json::from_str(source).expect("Qwen Code 0.23.3 fixture is valid JSON")
}

#[test]
fn identity_freezes_official_hops_and_preserves_claim_boundary() {
    let identity = json(IDENTITY);
    let protocol = json(PROTOCOL);

    assert_eq!(identity["axis"], "qwen-code.package");
    assert_eq!(identity["npm_package"], "@qwen-code/qwen-code");
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(identity["host"]["version"], "0.21.2");
    assert_eq!(identity["official"]["version"], "0.23.3");
    assert_eq!(
        identity["official"]["github_commit"],
        "b695664b8df06d06c625db3e30b97045d82092c7"
    );
    assert_eq!(
        identity["published_stables_from_previous_ceiling"],
        serde_json::json!(["0.23.0", "0.23.1", "0.23.2", "0.23.3"])
    );
    assert_eq!(identity["unpublished_stable_0_20_2"], true);
    assert_eq!(identity["unpublished_0_21_16"], true);
    assert_eq!(identity["unpublished_0_22_4"], true);
    assert_eq!(identity["unpublished_0_23_4"], true);

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(
        decision["reuse_behavior_revision"],
        "qwen-code.headless.v0.21.15-reasoning-control"
    );
    assert_eq!(decision["raise_latest_qualified_to"], "0.23.3");
    assert_eq!(decision["extend_same_revision_segment"], "0.22.0..=0.23.3");
    assert_eq!(decision["keep_exact_0_21_15"], true);
    assert_eq!(
        decision["extend_plan_exact_list_to_include_0_23_hops"],
        false
    );
    assert_eq!(decision["extend_reasoning_beyond_0_21_15"], false);
    assert_eq!(decision["extend_budgets_beyond_0_21_15"], false);
    assert_eq!(decision["flatten_to_model_studio_or_acp"], false);
    assert_eq!(decision["provider_prompt_sent"], false);

    assert_eq!(protocol["selected_mapped_subset_unchanged"], true);
    assert_eq!(protocol["catalogue_image_only_filter"], true);
    assert_eq!(protocol["decoder_corpus"], "qwen-code-v0.19.11");
    assert_eq!(
        protocol["0_23_3_plan_mode_blobs_byte_identical_to_0_22_3"],
        false
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
fn complete_hop_inventory_has_mutation_sensitive_counts_and_hashes() {
    let inventory = json(DIST_INVENTORY);
    assert_eq!(
        inventory["compared"],
        serde_json::json!(["0.22.3", "0.23.0", "0.23.1", "0.23.2", "0.23.3"])
    );
    assert_eq!(
        inventory["package_file_counts"],
        serde_json::json!({
            "0.22.3": 1005,
            "0.23.0": 1023,
            "0.23.1": 1033,
            "0.23.2": 1034,
            "0.23.3": 1050
        })
    );

    let expected = [
        ("from_0_22_3_to_0_23_0", 402, 384, 37, 584),
        ("from_0_23_0_to_0_23_1", 378, 368, 37, 618),
        ("from_0_23_1_to_0_23_2", 261, 260, 14, 759),
        ("from_0_23_2_to_0_23_3", 345, 329, 19, 686),
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
        inventory["hashes"]["cli.js"]["0.23.3"],
        "d2556c3877e4f697c14e1fb8f532561b583ae99f638b07b9412c6ed491e315be"
    );
    assert_eq!(
        inventory["hashes"]["cli-entry.js"]["0.23.3"],
        "68cb29eb7ccc936d78ece5564ef55cae41a55b630e6657dc417c1f2e561cf4c9"
    );
    assert_eq!(
        inventory["hashes"]["package.json"]["0.23.3"],
        "45f92de42988a9fa8a53db96b127eca8d8e19f0ce4f669435e7e555a04db1fd1"
    );
    assert_eq!(inventory["not_a_complete_semantic_changelog"], true);
    assert!(
        inventory["named_unmapped_with_reason"]
            .as_object()
            .unwrap()
            .len()
            >= 10
    );
}
