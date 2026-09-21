//! Mutation-sensitive freeze for the Oh My Pi `18.2.7` useful-newer hop.
//!
//! The corpus freezes official npm identity from the previous ceiling
//! `18.1.22` through official `18.2.7`, the complete shipped-tree delta for
//! every published hop, and one classification per changed mapped hop. Each
//! assertion fails if the frozen file sets, digests, counts, or classifications
//! mutate independently of the self-authored decision booleans.

use serde_json::Value;
use std::collections::BTreeSet;
use swallowtail_adapter_oh_my_pi::OH_MY_PI_PACKAGE_AXIS;

const IDENTITY: &str = include_str!("fixtures/oh-my-pi-18.2.7/identity.json");
const PROTOCOL: &str = include_str!("fixtures/oh-my-pi-18.2.7/protocol.json");
const DIST_INVENTORY: &str = include_str!("fixtures/oh-my-pi-18.2.7/dist-inventory.json");

const MAPPED_RPC_FILES: [&str; 10] = [
    "src/jsonrpc/message-framing.ts",
    "src/modes/rpc/host-tools.ts",
    "src/modes/rpc/host-uris.ts",
    "src/modes/rpc/rpc-client.ts",
    "src/modes/rpc/rpc-frame.ts",
    "src/modes/rpc/rpc-input.ts",
    "src/modes/rpc/rpc-messages.ts",
    "src/modes/rpc/rpc-mode.ts",
    "src/modes/rpc/rpc-subagents.ts",
    "src/modes/rpc/rpc-types.ts",
];

const MAPPED_SUPPORT_FILES: [&str; 2] = ["src/cli/flag-tables.ts", "src/modes/index.ts"];

const PUBLISHED_LEDGER: [(&str, &str, i64); 9] = [
    ("18.1.22", "2026-09-14T19:41:55.446Z", 3156),
    ("18.2.0", "2026-09-15T04:50:56.953Z", 3174),
    ("18.2.1", "2026-09-15T23:46:51.333Z", 3186),
    ("18.2.2", "2026-09-16T16:50:54.319Z", 3189),
    ("18.2.3", "2026-09-17T01:24:49.189Z", 3194),
    ("18.2.4", "2026-09-17T09:20:11.726Z", 3196),
    ("18.2.5", "2026-09-17T23:15:15.288Z", 2676),
    ("18.2.6", "2026-09-18T18:17:53.560Z", 2676),
    ("18.2.7", "2026-09-21T03:12:39.903Z", 2700),
];

const PUBLISHED_TARBALL_SHA256: [(&str, &str); 9] = [
    (
        "18.1.22",
        "6eac4319763089c6fab2fd5c60469bd7697988cb4134367f89db518a31ae80ac",
    ),
    (
        "18.2.0",
        "9570de0fc5dc6f67bd84b950a62f5c4f745a0e38ad7637c89c2f7de4ee58f010",
    ),
    (
        "18.2.1",
        "7fa079eef9edb0ebe8532392647b55ec416264414a642a7690087c76110ac4ef",
    ),
    (
        "18.2.2",
        "585b2d99d5b25fc656526411008aae1216007c8438523027279fb900d11f080f",
    ),
    (
        "18.2.3",
        "5433b265fb264e2e6198c9291a7f5a36b0c1a75ad80b697399851ec0e2534e2b",
    ),
    (
        "18.2.4",
        "bad179a89496b878baa373d275b90ea91d78c70b8815e99e52015d05d27f0a63",
    ),
    (
        "18.2.5",
        "41b3e698f5ac124775a336e8fa3afb6bb58b5ecc5d6f89b1ab51fd521f149acc",
    ),
    (
        "18.2.6",
        "759c6f5963d64550cb5b015805bf2b2efab6125620532aeb4fc64c657f5ffef1",
    ),
    (
        "18.2.7",
        "dab312bfb1dc236fe80c7db2e9d422c9629c833f6f5b1d45f421921921efa1d1",
    ),
];

const HOP_DELTAS: [(&str, u64, u64, u64); 8] = [
    ("from_18_1_22_to_18_2_0", 19, 1, 150),
    ("from_18_2_0_to_18_2_1", 14, 2, 429),
    ("from_18_2_1_to_18_2_2", 4, 1, 47),
    ("from_18_2_2_to_18_2_3", 8, 3, 83),
    ("from_18_2_3_to_18_2_4", 8, 6, 33),
    ("from_18_2_4_to_18_2_5", 50, 570, 623),
    ("from_18_2_5_to_18_2_6", 1, 1, 8),
    ("from_18_2_6_to_18_2_7", 35, 11, 209),
];

#[test]
fn official_published_ledger_is_exact_and_complete() {
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], OH_MY_PI_PACKAGE_AXIS);
    assert_eq!(identity["npm_package"], "@oh-my-pi/pi-coding-agent");
    assert_eq!(identity["npm_latest"], "18.2.7");
    assert_eq!(identity["github_latest_release"], "v18.2.7");
    assert_eq!(
        identity["npm_latest_published_at"],
        "2026-09-21T03:12:39.903Z"
    );
    assert_eq!(
        identity["npm_latest_shasum"],
        "d85297e98eb050522ccc65151c67a11c04d99790"
    );
    assert_eq!(
        identity["npm_latest_integrity"],
        "sha512-lVNXaRv5US4DTa79djiicoL3Mil284nKiZLReMd2j7XcwfVI2Bnx9yy0UQ1NevbhF/e0LtQ+fk6OSgEy9N3tew=="
    );
    assert_eq!(identity["npm_git_head"], Value::Null);
    assert_eq!(identity["published_stable_count"], 8);
    assert_eq!(identity["previous_ceiling"], "18.1.22");
    assert_eq!(identity["frozen_corpus_version"], "17.2.9");
    assert_eq!(identity["next_unpublished_stable_after_official"], "18.2.8");
    assert_eq!(identity["previous_synthetic_later_stable"], "18.1.23");
    assert_eq!(identity["pi_package_latest"], "0.86.1");
    assert_eq!(identity["installed_host"]["present"], false);

    let ledger = identity["published_ledger"]
        .as_array()
        .expect("published ledger is an array");
    assert_eq!(ledger.len(), PUBLISHED_LEDGER.len());
    for (row, (version, published_at, file_count)) in ledger.iter().zip(PUBLISHED_LEDGER) {
        assert_eq!(row["version"], version, "ledger row order must be exact");
        assert_eq!(row["package_json_version"], version);
        assert_eq!(row["published_at"], published_at);
        assert_eq!(row["file_count"].as_i64(), Some(file_count));
        assert_eq!(row["github_tag"], format!("v{version}"));
        assert!(
            row["github_tag_commit"]
                .as_str()
                .is_some_and(|commit| commit.len() == 40),
            "{version} carries a 40-hex GitHub tag commit"
        );
        assert!(
            row["npm_integrity"]
                .as_str()
                .is_some_and(|value| value.starts_with("sha512-")),
            "{version} carries an npm integrity"
        );
        assert!(
            row["npm_shasum"]
                .as_str()
                .is_some_and(|value| value.len() == 40),
            "{version} carries an npm shasum"
        );
        assert_eq!(row["tarball_sha256"].as_str().map(str::len), Some(64));
        assert_eq!(row["dist_cli_sha256"].as_str().map(str::len), Some(64));
        assert!(row["dist_cli_size"].as_u64().is_some_and(|size| size > 0));
    }

    let published: BTreeSet<String> = ledger
        .iter()
        .map(|row| row["version"].as_str().expect("version is text").to_owned())
        .collect();
    for (version, _) in PUBLISHED_TARBALL_SHA256 {
        assert!(published.contains(version), "{version} is published");
    }
    for gap in ["18.0.2", "18.1.7", "18.1.23", "18.2.8"] {
        assert!(
            !published.contains(gap),
            "unpublished {gap} must stay absent"
        );
    }
    assert_eq!(published.len(), 9);
}

#[test]
fn gaps_and_pi_axis_stay_explicit() {
    let identity = json(IDENTITY);
    assert_eq!(
        identity["npm_unpublished_stables_inside_window"],
        serde_json::json!(["18.0.2", "18.1.7"])
    );
    assert_eq!(
        identity["identity_decision"]["exclusions_kept"],
        serde_json::json!(["18.0.2", "18.1.7"])
    );
    assert_eq!(identity["identity_decision"]["mix_pi_package_axis"], false);
    assert_eq!(
        identity["identity_decision"]["would_flatten_families"],
        false
    );
    assert_eq!(identity["identity_decision"]["major_line_reset"], false);
    assert_eq!(identity["identity_decision"]["new_driver_or_facade"], false);
    assert_eq!(
        identity["identity_decision"]["new_public_operation_required"],
        false
    );
    assert_eq!(identity["identity_decision"]["provider_prompt_sent"], false);
    assert_eq!(identity["identity_decision"]["host_install_changed"], false);
    assert_eq!(
        identity["identity_decision"]["downloaded_artifact_executed"],
        false
    );
    assert_eq!(
        identity["identity_decision"]["decoder_corpus_retained"],
        "oh-my-pi-rpc-17.2.9"
    );
    assert_eq!(
        identity["identity_decision"]["eighteen_x_behavior_revision"],
        "oh-my-pi.rpc-v2-v18.0.0"
    );
    assert_eq!(
        identity["identity_decision"]["claim_id_stays"],
        "oh-my-pi.rpc.package-window-2"
    );
}

#[test]
fn research_327_ceiling_reproduces() {
    let identity = json(IDENTITY);
    let reproduction = &identity["research_327_reproduction"];
    assert_eq!(reproduction["reproduced"], true);
    assert_eq!(reproduction["previous_ceiling"], "18.1.22");
    assert_eq!(
        reproduction["npm_18_1_22_integrity"],
        "sha512-r/6rrx3PdCjcjaUvXDm8iFMIE/qHA9ryNo4RYXJuJmGXFnpxZNTHV4Ud0O8uGPMUx1YWRs0HdP6xUkG0XjZ44Q=="
    );
    assert_eq!(
        reproduction["npm_18_1_22_shasum"],
        "73e07a27460436b19eb0cae8dd293210b3971212"
    );
    assert_eq!(
        reproduction["extracted_18_1_22_tarball_sha256"],
        "6eac4319763089c6fab2fd5c60469bd7697988cb4134367f89db518a31ae80ac"
    );
    assert_eq!(
        reproduction["extracted_18_1_22_cli_sha256"],
        "b8bfd4f19a36b8bcae884be7ebed68133397868c8461167f3759acf1630a2499"
    );
    assert_eq!(reproduction["extracted_18_1_22_cli_size"], 22437945);
    assert_eq!(reproduction["extracted_18_1_22_file_count"], 3156);
    assert_eq!(
        reproduction["github_tag_v18_1_22_commit"],
        "23a5b9ae38864d3f785dc6cbc96eb6d674a1d32d"
    );
}

#[test]
fn mapped_source_ledger_classifies_every_changed_mapped_hop_exactly() {
    let protocol = json(PROTOCOL);
    let inventory = json(DIST_INVENTORY);

    let mapped: BTreeSet<&str> = MAPPED_RPC_FILES.into_iter().collect();
    assert_eq!(
        strings(&protocol["mapped_rpc_files"])
            .into_iter()
            .collect::<BTreeSet<_>>(),
        mapped
    );

    let classification = protocol["hop_classification"]
        .as_object()
        .expect("hop classification is an object");

    for (hop, delta) in inventory.as_object().expect("inventory is an object") {
        let Some(hop) = hop.strip_prefix("from_") else {
            continue;
        };
        let Some((from, to)) = hop.split_once("_to_") else {
            continue;
        };
        let hop = format!("{}->{}", from.replace('_', "."), to.replace('_', "."));
        let changed: BTreeSet<&str> = strings(&delta["changed"]).into_iter().collect();
        let added: BTreeSet<&str> = strings(&delta["added"]).into_iter().collect();
        let actual: BTreeSet<&str> = changed
            .iter()
            .copied()
            .filter(|path| mapped.contains(path) || MAPPED_SUPPORT_FILES.contains(path))
            .collect();
        let declared: BTreeSet<&str> = classification
            .get(hop.as_str())
            .map(|entry| {
                let mut paths: BTreeSet<&str> =
                    strings(&entry["mapped_files"]).into_iter().collect();
                if let Some(support) = entry.get("support_files") {
                    paths.extend(strings(support));
                }
                paths
            })
            .unwrap_or_default();
        assert_eq!(actual, declared, "mapped change set drifted for {hop}");
        if !actual.is_empty() {
            assert!(
                classification.contains_key(hop.as_str()),
                "{hop} changes mapped sources and must carry a classification"
            );
        }
        let added_selected: BTreeSet<&str> = added
            .iter()
            .copied()
            .filter(|path| {
                mapped.contains(path)
                    || MAPPED_SUPPORT_FILES.contains(path)
                    || *path == "src/modes/rpc/rpc-output.ts"
            })
            .collect();
        let declared_added: BTreeSet<&str> = classification
            .get(hop.as_str())
            .and_then(|entry| entry.get("added_selected_files"))
            .map(|value| strings(value).into_iter().collect())
            .unwrap_or_default();
        assert_eq!(
            added_selected, declared_added,
            "added selected set drifted for {hop}"
        );
    }
    assert_eq!(classification.len(), 6);
}

#[test]
fn shipped_tree_inventory_counts_are_exact() {
    let inventory = json(DIST_INVENTORY);
    let compared = strings(&inventory["compared"]);
    assert_eq!(compared.len(), 9);

    for (version, _, file_count) in PUBLISHED_LEDGER {
        assert_eq!(
            inventory["package_file_counts"][version].as_i64(),
            Some(file_count),
            "shipped file count drifted for {version}"
        );
    }

    let mut hop_keys: BTreeSet<&str> = BTreeSet::new();
    for (hop, added, removed, changed) in HOP_DELTAS {
        let delta = &inventory[hop];
        assert_eq!(delta["added_count"].as_u64(), Some(added), "{hop} added");
        assert_eq!(
            delta["removed_count"].as_u64(),
            Some(removed),
            "{hop} removed"
        );
        assert_eq!(
            delta["changed_count"].as_u64(),
            Some(changed),
            "{hop} changed"
        );
        assert_eq!(
            delta["added"].as_array().map(Vec::len),
            Some(added as usize)
        );
        assert_eq!(
            delta["removed"].as_array().map(Vec::len),
            Some(removed as usize)
        );
        assert_eq!(
            delta["changed"].as_array().map(Vec::len),
            Some(changed as usize)
        );
        hop_keys.insert(hop);
    }
    assert_eq!(hop_keys.len(), 8);

    assert_eq!(inventory["identical_through_18_1_22_18_2_7_count"], 1683);
    assert_eq!(
        inventory["identical_through_18_1_22_18_2_7_sha256"],
        "56c3d8deb001a043cb4c11616e679e06831db6f6d5193787eb7ed45cffc8de74"
    );
    for (version, digest) in PUBLISHED_TARBALL_SHA256 {
        assert_eq!(
            inventory["published_hop_tarball_sha256"][version], digest,
            "tarball digest drifted for {version}"
        );
    }
}

#[test]
fn selected_protocol_and_wire_invariants_are_unchanged() {
    let protocol = json(PROTOCOL);
    assert_eq!(protocol["selected_mode"], "rpc");
    assert_eq!(protocol["selected_approval_mode"], "always-ask");
    assert_eq!(protocol["selected_tools"], "read,grep,glob,todo,ask");
    assert_eq!(protocol["mapped_decoder_corpus"], "oh-my-pi-rpc-17.2.9");
    assert_eq!(protocol["decoder_corpus_retained"], true);
    assert_eq!(
        protocol["mapped_wire_lifecycle_failure_resource_change_found"],
        false
    );
    assert_eq!(protocol["provider_prompt_sent"], false);
    assert_eq!(protocol["host_install_changed"], false);
    assert_eq!(protocol["downloaded_artifact_executed"], false);
    assert_eq!(protocol["host_cli_observed"], Value::Null);
    assert_eq!(
        protocol["docs_rpc_md_blob_v18_1_22_through_v18_2_0"],
        "310b44702de66b4eecd6da37660f9fc40075d973"
    );
    assert_eq!(
        protocol["docs_rpc_md_blob_v18_2_3_through_v18_2_7"],
        "b91b20a85aca2eacc7272cac7f1a55370449bc05"
    );

    for flag in [
        "--mode",
        "--no-session",
        "--provider",
        "--model",
        "--tools",
        "--no-extensions",
        "--no-skills",
        "--no-rules",
        "--no-prewalk",
        "--approval-mode",
    ] {
        assert!(
            strings(&protocol["selected_argv_flags"]).contains(&flag),
            "missing selected flag {flag}"
        );
    }
    for command in [
        "negotiate_protocol",
        "prompt",
        "steer",
        "follow_up",
        "abort",
        "get_state",
        "get_available_models",
        "set_model",
        "set_thinking_level",
        "set_steering_mode",
        "set_follow_up_mode",
        "set_interrupt_mode",
        "set_auto_compaction",
        "set_auto_retry",
    ] {
        assert!(
            strings(&protocol["selected_commands"]).contains(&command),
            "missing selected command {command}"
        );
    }

    assert_eq!(
        protocol["selected_wire_invariants"]["ready_protocolVersion"],
        1
    );
    assert_eq!(
        protocol["selected_wire_invariants"]["ready_supportedProtocolVersions"],
        serde_json::json!([1, 2])
    );
    assert_eq!(
        protocol["selected_wire_invariants"]["negotiated_protocolVersion"],
        2
    );
    assert_eq!(
        protocol["selected_wire_invariants"]["maximum_physical_frame_bytes"],
        1048576
    );
    assert_eq!(
        protocol["selected_wire_invariants"]["maximum_reassembled_frame_bytes"],
        67108864
    );
    assert_eq!(
        protocol["selected_wire_invariants"]["usage_fields"],
        serde_json::json!(["input", "output", "cacheRead", "cacheWrite"])
    );
    assert_eq!(
        protocol["selected_wire_invariants"]["terminal_event"],
        "agent_end with isTerminal true"
    );
    assert_eq!(
        protocol["selected_wire_invariants"]["provider_failure_marker"],
        "assistant message_end with stopReason error"
    );
}

#[test]
fn byte_identical_mapped_files_are_exact() {
    let protocol = json(PROTOCOL);
    let inventory = json(DIST_INVENTORY);
    let identical: BTreeSet<&str> = strings(&inventory["identical_through_18_1_22_18_2_7"])
        .into_iter()
        .collect();
    let declared: BTreeSet<&str> =
        strings(&protocol["byte_identical_mapped_files_across_all_compared"])
            .into_iter()
            .collect();
    assert_eq!(
        declared,
        BTreeSet::from([
            "src/modes/rpc/host-uris.ts",
            "src/modes/rpc/rpc-frame.ts",
            "src/modes/rpc/rpc-input.ts",
            "src/modes/rpc/rpc-messages.ts",
        ])
    );
    for path in &declared {
        assert!(
            identical.contains(path),
            "{path} must be byte-identical across every compared version"
        );
    }
    for path in MAPPED_RPC_FILES {
        if !declared.contains(path) {
            assert!(
                !identical.contains(path),
                "{path} changes and must not claim byte identity"
            );
        }
    }
}

#[test]
fn digest_groups_are_mutation_sensitive() {
    let protocol = json(PROTOCOL);
    let groups = &protocol["mapped_rpc_digest_groups"];

    let rpc_frame = groups["src/modes/rpc/rpc-frame.ts"]
        .as_array()
        .expect("rpc-frame groups are an array");
    assert_eq!(rpc_frame.len(), 1);
    assert_eq!(
        rpc_frame[0]["sha256"],
        "255a7512fcb67bc36170edc0669f7957ba97c90d005be712587f1b2b7d7a8297"
    );
    assert_eq!(rpc_frame[0]["versions"].as_array().map(Vec::len), Some(9));

    let rpc_mode_counts: Vec<usize> = groups["src/modes/rpc/rpc-mode.ts"]
        .as_array()
        .expect("rpc-mode groups are an array")
        .iter()
        .map(|group| group["versions"].as_array().expect("versions").len())
        .collect();
    assert_eq!(rpc_mode_counts, vec![1, 1, 2, 2, 3]);

    for path in MAPPED_RPC_FILES {
        let total: usize = groups[path]
            .as_array()
            .expect("groups are an array")
            .iter()
            .map(|group| {
                group["versions"]
                    .as_array()
                    .expect("versions are an array")
                    .len()
            })
            .sum();
        assert_eq!(total, 9, "{path} must cover every compared version once");
    }

    assert_eq!(
        protocol["added_selected_files"]["src/modes/rpc/rpc-output.ts"]
            ["sha256_18_2_1_through_18_2_7"],
        "10529f7fbd9e6303f103a99a5d4fdaf5f9268dd29219463fdf638f4122cec433"
    );
}

#[test]
fn claim_decision_is_compatible_extension_of_the_18_x_segment() {
    let identity = json(IDENTITY);
    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(decision["eighteen_x_baseline"], "18.0.0");
    assert_eq!(decision["eighteen_x_latest"], "18.2.7");
    assert_eq!(decision["synthetic_unverified_newer"], "18.2.8");
    assert_eq!(decision["private_milestone_checked"], true);
    assert_eq!(decision["mapped_selected_flags_present"], true);
    assert_eq!(decision["mapped_selected_commands_present"], true);
    assert_eq!(decision["historical_specimens_retained"], true);
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "18.1.22"
    );
    assert_eq!(
        identity["claim_at_observation"]["claim_id"],
        "oh-my-pi.rpc.package-window-2"
    );
}

fn json(value: &str) -> Value {
    serde_json::from_str(value).expect("frozen corpus JSON is valid")
}

fn strings(value: &Value) -> Vec<&str> {
    value
        .as_array()
        .expect("value is an array")
        .iter()
        .map(|value| value.as_str().expect("array value is text"))
        .collect()
}
