//! Mutation-sensitive freeze for the Oh My Pi `18.1.22` major-line
//! qualification.
//!
//! The corpus freezes official npm identity from the previous ceiling
//! `17.4.0` through official `18.1.22`, the complete shipped-tree delta for
//! every published hop, and one classification per changed mapped hop. Each
//! assertion fails if the frozen file sets, digests, counts, or classifications
//! mutate independently of the self-authored decision booleans.

use serde_json::Value;
use std::collections::BTreeSet;
use swallowtail_adapter_oh_my_pi::OH_MY_PI_PACKAGE_AXIS;

const IDENTITY: &str = include_str!("fixtures/oh-my-pi-18.1.22/identity.json");
const PROTOCOL: &str = include_str!("fixtures/oh-my-pi-18.1.22/protocol.json");
const DIST_INVENTORY: &str = include_str!("fixtures/oh-my-pi-18.1.22/dist-inventory.json");

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

const PUBLISHED_LEDGER: [(&str, &str, i64); 36] = [
    ("17.4.0", "2026-08-20T06:42:13.785Z", 2908),
    ("17.4.1", "2026-08-21T14:57:58.994Z", 2924),
    ("17.4.2", "2026-08-21T20:39:23.544Z", 2982),
    ("18.0.0", "2026-08-22T11:04:56.289Z", 3000),
    ("18.0.1", "2026-08-23T03:25:59.678Z", 3012),
    ("18.0.3", "2026-08-23T09:20:14.973Z", 3017),
    ("18.0.4", "2026-08-24T04:09:18.006Z", 3041),
    ("18.0.5", "2026-08-25T16:45:58.711Z", 3069),
    ("18.0.6", "2026-08-26T08:28:41.651Z", 3084),
    ("18.0.7", "2026-08-27T11:08:50.396Z", 3087),
    ("18.0.8", "2026-08-27T17:52:51.504Z", 3087),
    ("18.0.9", "2026-08-28T06:17:51.492Z", 3091),
    ("18.0.10", "2026-08-28T19:03:05.429Z", 3115),
    ("18.0.11", "2026-08-29T18:35:06.542Z", 3125),
    ("18.1.0", "2026-09-01T14:04:49.247Z", 3133),
    ("18.1.1", "2026-09-01T16:25:01.931Z", 3133),
    ("18.1.2", "2026-09-01T20:30:05.424Z", 3136),
    ("18.1.3", "2026-09-02T14:10:38.804Z", 3137),
    ("18.1.4", "2026-09-02T17:27:44.920Z", 3137),
    ("18.1.5", "2026-09-03T02:24:28.832Z", 3143),
    ("18.1.6", "2026-09-03T11:58:30.193Z", 3115),
    ("18.1.8", "2026-09-04T00:31:13.079Z", 3102),
    ("18.1.9", "2026-09-04T06:57:37.802Z", 3106),
    ("18.1.10", "2026-09-04T10:12:21.333Z", 3106),
    ("18.1.11", "2026-09-05T16:06:19.814Z", 3106),
    ("18.1.12", "2026-09-06T14:30:39.970Z", 3109),
    ("18.1.13", "2026-09-07T00:50:09.083Z", 3109),
    ("18.1.14", "2026-09-07T18:46:59.646Z", 3109),
    ("18.1.15", "2026-09-08T22:44:51.673Z", 3109),
    ("18.1.16", "2026-09-09T19:10:36.391Z", 3120),
    ("18.1.17", "2026-09-10T19:45:18.022Z", 3122),
    ("18.1.18", "2026-09-11T21:57:09.611Z", 3126),
    ("18.1.19", "2026-09-13T00:06:43.759Z", 3148),
    ("18.1.20", "2026-09-13T20:30:16.410Z", 3156),
    ("18.1.21", "2026-09-14T05:40:04.342Z", 3156),
    ("18.1.22", "2026-09-14T19:41:55.446Z", 3156),
];

const PUBLISHED_TARBALL_SHA256: [(&str, &str); 36] = [
    (
        "17.4.0",
        "0b0d36501c14560f0ec24b51422365fe597e0bcf69fbb6df9923741310c035e2",
    ),
    (
        "17.4.1",
        "87971be9717291182dcf9a21a26419bda8d17f6c7c094abafbe3cf5122bc67e2",
    ),
    (
        "17.4.2",
        "58dd5a58e172cbdc66faccdb7e24849a2b340309ffdf6f385c75817d533128b0",
    ),
    (
        "18.0.0",
        "e61154cfb3447b0716ba4c44bfcc07486ef847bc155429195809d5555a30b012",
    ),
    (
        "18.0.1",
        "52f5c7bb1b4404a1851a3d7d0cf7078db64ac66f6bb31ccc6322f7eed1c4a110",
    ),
    (
        "18.0.3",
        "3b7faa6ff418847c58835e3a1f7ec6dd3e72d57ec26d46cf7975bcb31161785f",
    ),
    (
        "18.0.4",
        "18ca784d262c0edb3c4b7d5e57c0344bffe287b4c0751700e21577a09b37638d",
    ),
    (
        "18.0.5",
        "7d9745c2a3cfa4cec84363b952e8a7a649f9906021d22aa5b415c4bc6fbaf6f0",
    ),
    (
        "18.0.6",
        "c9084a2cb88346a1c5810006f3c30fe4fcfe903f825e340a21e383ecd2f17feb",
    ),
    (
        "18.0.7",
        "21cd6cbe97dcc325e4fb2054b772d5fec6fbbea585d75a558c31450aafd13975",
    ),
    (
        "18.0.8",
        "90ad62db02ad27b2d0cf1ce77cd3762b8c0909dad8ff5f07a9dd3e46bcd4d414",
    ),
    (
        "18.0.9",
        "1da3da7909c6b7c20ee22d193940d15770265f4d743d747ae53e77391bfdd129",
    ),
    (
        "18.0.10",
        "14c8c14434edd6454e1ce0e6739e7e2dc2fbf35543ed229e7e1e9481e653b37a",
    ),
    (
        "18.0.11",
        "a5757dccfc4b2c6101cd3a4e09e98a51e795e700e0e4f6be930ce9de595f33b6",
    ),
    (
        "18.1.0",
        "64226beec2b1d1b48475b3bdbcf2172780b009536530ca6bc1f11d604c6a656c",
    ),
    (
        "18.1.1",
        "28e4c5eeab59bd7494b2e881e679b57aa8b5b01139471c2dc2d5805a01c03c50",
    ),
    (
        "18.1.2",
        "b8fdd73367e92ffc343b51cb606be8b045a2cddd95095420a5913b85cc6766f1",
    ),
    (
        "18.1.3",
        "0925092af9d9417c989f1e61c8a57359dc4a1165217e87e0eed7fe2c4f6fb97b",
    ),
    (
        "18.1.4",
        "e0e68f0d82507343013addc615629e8dd5e8cd9526aad8e4d4ac1ddf0248a675",
    ),
    (
        "18.1.5",
        "d9040198f3436093aa08b679ba36ef91357d4624a65e5c6b8e834c247d5f7355",
    ),
    (
        "18.1.6",
        "edd4db797d6ae25a703bc003851d044d48182be0c39323cefa86fabd1bfa6e70",
    ),
    (
        "18.1.8",
        "020677e1ce37d2df16bca1e1771f5701731310d091219b6a669ec9d5efb3d97d",
    ),
    (
        "18.1.9",
        "247208e2baece4a74e634fb128859ab0e3c4efdb30c2f7e5b7329d2d5c9c2343",
    ),
    (
        "18.1.10",
        "e082f186e702b657125d9ae1679dfd23f5574604ad922e3cc60a24fd31d46894",
    ),
    (
        "18.1.11",
        "3300efbee331a0e40d3837f4a945353a7f83b393d3cb2ba5fe13a7fdb2f3d891",
    ),
    (
        "18.1.12",
        "d31c63045dbbd7679b7aeae421a313be5e9265fa4d62200c6790f40e57b89efc",
    ),
    (
        "18.1.13",
        "b76c5abd9a9b48a0dbc4d07d92d7db438ac84567e4050b4a9913a74a8a101c5f",
    ),
    (
        "18.1.14",
        "3730edaac7c87245b5244fa1878b6314614c69f784dec8c962fb0737be7e250f",
    ),
    (
        "18.1.15",
        "6247811119c5af37bfab9a983d20ccd3e4d2a608c0ec1eebd6946062af30745b",
    ),
    (
        "18.1.16",
        "cadfd9651a53537b3831ef925ec9b9eea4640d059f4cb9ec8fbb339eb864a49b",
    ),
    (
        "18.1.17",
        "cf12c50c85627122beeab2e5ed6572137226c7ecaec069dfc7a18eef36949eda",
    ),
    (
        "18.1.18",
        "edb4fec4544c28e709923e6ae4861dcd3aa546228acb4e7d10790a7e80c7b02b",
    ),
    (
        "18.1.19",
        "3c13a4804fa7b6ebb7f8c7a539ef96fdb8bfe5e401c416d068e076274bc7b670",
    ),
    (
        "18.1.20",
        "3cc96c50956795786fbe8f7b1a5494c82e7c9252e0cd4c5bdfd95f46a897ce7d",
    ),
    (
        "18.1.21",
        "d1754eda81cdd071e1ae944249a750c561fddb621b5681d90317390364809e31",
    ),
    (
        "18.1.22",
        "6eac4319763089c6fab2fd5c60469bd7697988cb4134367f89db518a31ae80ac",
    ),
];

/// Per published hop: `(added, removed, changed)` counts from the complete
/// shipped-tree inventory.
const HOP_DELTAS: [(&str, u64, u64, u64); 35] = [
    ("from_17_4_0_to_17_4_1", 19, 3, 146),
    ("from_17_4_1_to_17_4_2", 59, 1, 61),
    ("from_17_4_2_to_18_0_0", 21, 3, 96),
    ("from_18_0_0_to_18_0_1", 13, 1, 173),
    ("from_18_0_1_to_18_0_3", 6, 1, 40),
    ("from_18_0_3_to_18_0_4", 25, 1, 116),
    ("from_18_0_4_to_18_0_5", 29, 1, 143),
    ("from_18_0_5_to_18_0_6", 41, 26, 29),
    ("from_18_0_6_to_18_0_7", 4, 1, 165),
    ("from_18_0_7_to_18_0_8", 1, 1, 58),
    ("from_18_0_8_to_18_0_9", 9, 5, 155),
    ("from_18_0_9_to_18_0_10", 25, 1, 71),
    ("from_18_0_10_to_18_0_11", 11, 1, 77),
    ("from_18_0_11_to_18_1_0", 11, 3, 249),
    ("from_18_1_0_to_18_1_1", 1, 1, 3),
    ("from_18_1_1_to_18_1_2", 4, 1, 40),
    ("from_18_1_2_to_18_1_3", 3, 2, 44),
    ("from_18_1_3_to_18_1_4", 0, 0, 2),
    ("from_18_1_4_to_18_1_5", 8, 2, 88),
    ("from_18_1_5_to_18_1_6", 23, 51, 186),
    ("from_18_1_6_to_18_1_8", 20, 33, 147),
    ("from_18_1_8_to_18_1_9", 21, 17, 177),
    ("from_18_1_9_to_18_1_10", 2, 2, 12),
    ("from_18_1_10_to_18_1_11", 1, 1, 17),
    ("from_18_1_11_to_18_1_12", 4, 1, 49),
    ("from_18_1_12_to_18_1_13", 1, 1, 3),
    ("from_18_1_13_to_18_1_14", 1, 1, 27),
    ("from_18_1_14_to_18_1_15", 1, 1, 68),
    ("from_18_1_15_to_18_1_16", 12, 1, 73),
    ("from_18_1_16_to_18_1_17", 3, 1, 52),
    ("from_18_1_17_to_18_1_18", 5, 1, 83),
    ("from_18_1_18_to_18_1_19", 23, 1, 107),
    ("from_18_1_19_to_18_1_20", 9, 1, 35),
    ("from_18_1_20_to_18_1_21", 1, 1, 20),
    ("from_18_1_21_to_18_1_22", 1, 1, 58),
];

#[test]
fn official_published_ledger_is_exact_and_complete() {
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], OH_MY_PI_PACKAGE_AXIS);
    assert_eq!(identity["npm_package"], "@oh-my-pi/pi-coding-agent");
    assert_eq!(identity["npm_latest"], "18.1.22");
    assert_eq!(identity["github_latest_release"], "v18.1.22");
    assert_eq!(
        identity["npm_latest_published_at"],
        "2026-09-14T19:41:55.446Z"
    );
    assert_eq!(
        identity["npm_latest_shasum"],
        "73e07a27460436b19eb0cae8dd293210b3971212"
    );
    assert_eq!(
        identity["npm_latest_integrity"],
        "sha512-r/6rrx3PdCjcjaUvXDm8iFMIE/qHA9ryNo4RYXJuJmGXFnpxZNTHV4Ud0O8uGPMUx1YWRs0HdP6xUkG0XjZ44Q=="
    );
    assert_eq!(identity["npm_git_head"], Value::Null);
    assert_eq!(identity["published_stable_count"], 36);
    assert_eq!(identity["previous_ceiling"], "17.4.0");
    assert_eq!(identity["frozen_corpus_version"], "17.2.9");
    assert_eq!(
        identity["next_unpublished_stable_after_official"],
        "18.1.23"
    );
    assert_eq!(identity["pi_package_latest"], "0.85.1");
    assert_eq!(identity["installed_host"]["version_output"], "omp/18.1.16");
    assert_eq!(
        identity["installed_host"]["sha256"],
        "99eed6d45d984d2f13d76832f78782b9aa07862921ab4e98e2d8e31ca8129795"
    );

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
    for gap in ["17.4.3", "17.4.4", "18.0.2", "18.1.7"] {
        assert!(
            !published.contains(gap),
            "unpublished {gap} must stay absent"
        );
    }
    assert_eq!(published.len(), 36);
}

#[test]
fn npm_unpublished_gaps_and_pi_axis_stay_explicit() {
    let identity = json(IDENTITY);
    assert_eq!(
        identity["npm_unpublished_stables_inside_window"],
        serde_json::json!(["17.4.3", "17.4.4", "18.0.2", "18.1.7"])
    );
    assert_eq!(
        identity["github_only_tag_gaps"],
        serde_json::json!({
            "17.4.3": "028c0a4a7c785c989b7cce6c91319662be43a31e",
            "17.4.4": "a7e19be81039c110ba943af1281666dc7b0810b0",
            "18.0.2": "b214f5a1b6ddda185c618f1339d5fd4ccd45f7c0",
            "18.1.7": "c4da0d08e8275659f3e09cf381c7df7018a19025",
        })
    );
    assert_eq!(
        identity["identity_decision"]["exclusions_added"],
        serde_json::json!(["18.0.2", "18.1.7"])
    );
    assert_eq!(
        identity["identity_decision"]["exclusions_absent_between_segments"],
        serde_json::json!(["17.4.3", "17.4.4"])
    );
    assert_eq!(identity["identity_decision"]["mix_pi_package_axis"], false);
    assert_eq!(identity["identity_decision"]["provider_prompt_sent"], false);
    assert_eq!(identity["identity_decision"]["host_install_changed"], false);
    assert_eq!(
        identity["identity_decision"]["downloaded_artifact_executed"],
        false
    );
    assert_eq!(
        identity["identity_decision"]["new_public_operation_required"],
        false
    );
    assert_eq!(
        identity["identity_decision"]["decoder_corpus_retained"],
        "oh-my-pi-rpc-17.2.9"
    );
    assert_eq!(
        identity["identity_decision"]["seventeen_x_segment_becomes_deprecated"],
        true
    );
    assert_eq!(
        identity["identity_decision"]["new_claim_id"],
        "oh-my-pi.rpc.package-window-2"
    );
    assert_eq!(
        identity["identity_decision"]["eighteen_x_behavior_revision"],
        "oh-my-pi.rpc-v2-v18.0.0"
    );
}

#[test]
fn research_217_identity_reproduces_with_the_recorded_correction() {
    let identity = json(IDENTITY);
    let reproduction = &identity["research_217_reproduction"];
    assert_eq!(reproduction["reproduced"], true);
    assert_eq!(reproduction["assigned_official"], "18.0.5");
    assert_eq!(
        reproduction["npm_18_0_5_integrity"],
        "sha512-4bDndTceC6R5gFLS+FnkSiDBrlVbAt2EjL9ca4K29Qd5R+fpxOaad3dOQSenKXd1y3Ot/MfoNGrfH2dXr5hpSA=="
    );
    assert_eq!(
        reproduction["npm_18_0_5_shasum"],
        "415d8e183449fea482382268fcc4b8063bfd04e8"
    );
    assert_eq!(
        reproduction["extracted_18_0_5_cli_sha256"],
        "3edd2768e2ace4fdc034c8c2f8579d8e21c97954e2605ace51e86283a8be9651"
    );
    assert_eq!(reproduction["extracted_18_0_5_cli_size"], 19316793);
    assert_eq!(
        reproduction["github_tag_v18_0_5_commit"],
        "eab72e88e447a4be45bea2bc302995844c0c51a2"
    );
    assert_eq!(
        reproduction["mapped_rpc_mode_identical_v17_4_2_through_v18_0_6"],
        true
    );
    assert_eq!(
        reproduction["rpc_types_identical_v17_4_2_through_v18_1_22"],
        true
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

    // Mutation guard: every hop's mapped change set must equal the declared
    // classification exactly, and no changed mapped hop may be unclassified.
    for (hop, delta) in inventory.as_object().expect("inventory is an object") {
        let Some(hop) = hop.strip_prefix("from_") else {
            continue;
        };
        let Some((from, to)) = hop.split_once("_to_") else {
            continue;
        };
        let hop = format!("{}->{}", from.replace('_', "."), to.replace('_', "."));
        let changed: BTreeSet<&str> = strings(&delta["changed"]).into_iter().collect();
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
    }
    assert_eq!(classification.len(), 12);
}

#[test]
fn shipped_tree_inventory_counts_are_exact() {
    let inventory = json(DIST_INVENTORY);
    let compared = strings(&inventory["compared"]);
    assert_eq!(compared.len(), 36);

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
    assert_eq!(hop_keys.len(), 35);

    assert_eq!(inventory["identical_through_17_4_0_18_1_22_count"], 1773);
    assert_eq!(
        inventory["identical_through_17_4_0_18_1_22_sha256"],
        "2844f8eaab5fad6fe429e053705a072b3b57901a6eef1781812370bf4a046cbc"
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
    assert_eq!(protocol["option_details_is_additive_and_ignored"], true);
    assert_eq!(
        protocol["option_details_cannot_change_mapped_options"],
        true
    );
    assert_eq!(protocol["provider_prompt_sent"], false);
    assert_eq!(protocol["host_install_changed"], false);
    assert_eq!(protocol["downloaded_artifact_executed"], false);
    assert_eq!(protocol["host_cli_observed"], "omp/18.1.16");
    assert_eq!(
        protocol["docs_rpc_md_blob_v17_4_2_through_v18_1_22"],
        "310b44702de66b4eecd6da37660f9fc40075d973"
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
    let identical: BTreeSet<&str> = strings(&inventory["identical_through_17_4_0_18_1_22"])
        .into_iter()
        .collect();
    let declared: BTreeSet<&str> =
        strings(&protocol["byte_identical_mapped_files_across_all_compared"])
            .into_iter()
            .collect();
    assert_eq!(
        declared,
        BTreeSet::from([
            "src/jsonrpc/message-framing.ts",
            "src/modes/rpc/rpc-input.ts",
            "src/modes/rpc/rpc-messages.ts",
            "src/modes/rpc/host-uris.ts",
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

    let expected_rpc_mode: [(&str, usize); 8] = [
        ("5ce7071445fd", 1),
        ("beafadbe5479", 1),
        ("a428097440d1", 7),
        ("14b49872d4e5", 2),
        ("9a7395a8cbf6", 3),
        ("8a64ee44e1f3", 3),
        ("2e7c38538d69", 14),
        ("79878b6a2f95", 5),
    ];
    let rpc_mode = groups["src/modes/rpc/rpc-mode.ts"]
        .as_array()
        .expect("rpc-mode groups are an array");
    assert_eq!(rpc_mode.len(), expected_rpc_mode.len());
    for (group, (prefix, count)) in rpc_mode.iter().zip(expected_rpc_mode) {
        let digest = group["sha256"].as_str().expect("digest is text");
        assert!(
            digest.starts_with(prefix),
            "{digest} does not start with {prefix}"
        );
        assert_eq!(
            group["versions"].as_array().map(Vec::len),
            Some(count),
            "{prefix} group size drifted"
        );
    }

    let rpc_types: Vec<usize> = groups["src/modes/rpc/rpc-types.ts"]
        .as_array()
        .expect("rpc-types groups are an array")
        .iter()
        .map(|group| {
            group["versions"]
                .as_array()
                .expect("versions are an array")
                .len()
        })
        .collect();
    assert_eq!(rpc_types, vec![2, 34]);

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
        assert_eq!(total, 36, "{path} must cover every compared version once");
    }
}

#[test]
fn claim_decision_excludes_merge_and_names_both_segments() {
    let identity = json(IDENTITY);
    let decision = &identity["identity_decision"];
    assert_eq!(
        decision["seventeen_x_hop_decisions"]["17.4.0->17.4.1"],
        "compatible-extension"
    );
    assert_eq!(
        decision["seventeen_x_hop_decisions"]["17.4.1->17.4.2"],
        "compatible-extension"
    );
    assert_eq!(decision["eighteen_x_baseline"], "18.0.0");
    assert_eq!(decision["eighteen_x_latest"], "18.1.22");
    assert_eq!(decision["private_milestone_checked"], true);
    assert_eq!(decision["mapped_selected_flags_present"], true);
    assert_eq!(decision["mapped_selected_commands_present"], true);
    assert_eq!(decision["historical_specimens_retained"], true);
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "17.4.0"
    );
    assert_eq!(
        identity["claim_at_observation"]["claim_id"],
        "oh-my-pi.rpc.package-window-1"
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
