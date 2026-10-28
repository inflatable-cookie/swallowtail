use std::collections::{BTreeMap, BTreeSet};

use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use swallowtail_adapter_oh_my_pi::{
    OH_MY_PI_PACKAGE_AXIS, OH_MY_PI_PACKAGE_LATEST_QUALIFIED_VERSION, oh_my_pi_rpc_claim,
};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

const IDENTITY: &str = include_str!("fixtures/oh-my-pi-18.13.2/identity.json");
const PROTOCOL: &str = include_str!("fixtures/oh-my-pi-18.13.2/protocol.json");
const INVENTORY: &str = include_str!("fixtures/oh-my-pi-18.13.2/complete-tree-inventory.json");
const HOPS: &str = include_str!("fixtures/oh-my-pi-18.13.2/hop-ledger.json");

const IDENTITY_SHA256: &str = "e6c6dbee0aaad53c2672d1dfc057b43a3f335674e13644dfc1fcb5459347df5c";
const PROTOCOL_SHA256: &str = "80a47a3cec619f8d7578920c78f93633a009d492d71c1d6ad282f4b949873737";
const INVENTORY_SHA256: &str = "85ee5a5df317107b5b0ce72ddd33743a0b99e18c19d7f6d404689b3026ba1dd6";
const HOPS_SHA256: &str = "4579b94b71c76ae0977f6847088d0471cc28ab0dab96f98fe6af2c4c1073013a";

#[test]
fn official_18_13_2_identity_and_every_published_hop_are_frozen() {
    assert_sha256(IDENTITY.as_bytes(), IDENTITY_SHA256);
    assert_sha256(PROTOCOL.as_bytes(), PROTOCOL_SHA256);
    assert_sha256(INVENTORY.as_bytes(), INVENTORY_SHA256);
    assert_sha256(HOPS.as_bytes(), HOPS_SHA256);

    let identity = parse(IDENTITY);
    let protocol = parse(PROTOCOL);
    let inventory = parse(INVENTORY);
    let hops = parse(HOPS);

    assert_keys(
        &identity,
        &[
            "all_18x_exclusions",
            "artifact",
            "artifact_ledger",
            "axis",
            "claim_at_observation",
            "compared_versions",
            "evidence_controls",
            "github_only_unpublished_stable_tag_gaps",
            "host_observation",
            "identity_decision",
            "npm_only_published_stables_without_github_tag",
            "npm_package",
            "observed_at",
            "official_channels",
            "official_latest_at_identity",
            "oldest_17x_segment",
            "previous_historical_exclusions",
            "previous_qualified_ceiling",
            "published_stable_hop_count",
            "published_stable_hops",
            "runtime_observations",
            "schema_version",
            "source_identity",
            "synthetic_later_stable_after_official",
            "synthetic_later_stable_note",
        ],
    );
    assert_keys(
        &protocol,
        &[
            "axis",
            "catalogue_argv",
            "internal_read_resource_boundary",
            "observed_at",
            "package",
            "rpc_command_cases_by_version",
            "schema_version",
            "selected_argv",
            "selected_rpc_commands",
            "selected_source_file_union",
            "selected_tool_schema",
            "selected_tools",
            "selected_wire_invariants",
            "source_hash_authority",
            "source_verification",
            "unmapped_boundary",
            "unselected_rpc_command_cases_at_target",
            "version",
        ],
    );

    assert_eq!(identity["axis"], OH_MY_PI_PACKAGE_AXIS);
    assert_eq!(identity["npm_package"], "@oh-my-pi/pi-coding-agent");
    assert_eq!(identity["official_latest_at_identity"], "18.13.2");
    assert_eq!(identity["previous_qualified_ceiling"], "18.12.6");
    assert_eq!(identity["synthetic_later_stable_after_official"], "18.13.3");
    assert_eq!(identity["published_stable_hop_count"], 3);
    assert_eq!(identity["observed_at"], "2026-10-28T09:40:00Z");
    assert_eq!(identity["artifact"]["version"], "18.13.2");
    assert_eq!(identity["artifact"]["npm_git_head"], Value::Null);
    assert_eq!(
        identity["artifact"]["tarball_integrity"],
        "sha512-RNaGEOpLtG5GgPYP9ssOxtWKAVIVEF2frSjd2tLbmK43uXGoohIm+pXIVKPsxK+sonJ8blXM+bPYyvG+eMauCA=="
    );
    assert_eq!(
        identity["artifact"]["tarball_sha256"],
        "0a2e716dafea4d7e49964ae344db662f92491bb1bd4c9fbde63d2df4d83a16b5"
    );
    assert_eq!(
        identity["artifact"]["dist_cli_sha256"],
        "0b7355fb5489a0bdec1289353093ac51e015f438e5c89e8a036bcf955752cfeb"
    );
    assert_eq!(identity["artifact"]["package_file_count"], 3366);
    assert_eq!(identity["artifact"]["engines"]["bun"], ">=1.3.14");
    assert_eq!(identity["official_channels"]["channel_agreement"], true);
    assert_eq!(identity["official_channels"]["npm"]["latest"], "18.13.2");
    assert_eq!(
        identity["official_channels"]["github_releases"]["latest_release_tag"],
        "v18.13.2"
    );
    assert_eq!(
        identity["source_identity"]["target_github_commit"],
        "e996e1c7ac476f60e02b8ed9764cf30fa8c00a8e"
    );
    assert_eq!(
        identity["source_identity"]["baseline_tree_sha256_matches_research_447"],
        "13057a74c47625b83d803e0b401789685a11a77c646233ff4cec571568a9de32"
    );
    assert_eq!(
        identity["source_identity"]["target_tree_sha256"],
        "ffd81f9fcd5b79fca84ae993cfc7953dd889d2be9172d9a78e762fece8c6cb75"
    );
    assert_eq!(
        identity["all_18x_exclusions"],
        json!(["18.0.2", "18.1.7", "18.4.7", "18.6.2"])
    );
    assert_eq!(
        identity["identity_decision"]["shape"],
        "compatible-extension"
    );
    assert_eq!(
        identity["identity_decision"]["new_18x_range"],
        "18.0.0..=18.13.2"
    );
    assert_eq!(
        identity["identity_decision"]["claim_id_stays"],
        "oh-my-pi.rpc.package-window-2"
    );
    assert_eq!(
        identity["identity_decision"]["behavior_revision"],
        "oh-my-pi.rpc-v2-v18.0.0"
    );
    assert_eq!(
        identity["identity_decision"]["security_or_authority_tier_change"],
        false
    );
    assert_eq!(
        identity["identity_decision"]["new_public_operation_required"],
        false
    );
    assert_eq!(
        identity["identity_decision"]["pi_rpc_or_sdk_evidence_transferred"],
        false
    );
    assert_eq!(
        identity["identity_decision"]["unselected_rpc_additions"],
        json!([
            "abort_branch_summary",
            "dictation_start",
            "dictation_stop",
            "navigate_tree",
            "set_label"
        ])
    );
    assert_eq!(
        identity["claim_at_observation"]["posture"],
        "allow_unverified"
    );
    assert_eq!(
        identity["claim_at_observation"]["segments"][1]["range"],
        "18.0.0..=18.12.6"
    );
    for control in [
        "artifacts_executed",
        "credentials_used",
        "host_updated_or_mutated",
        "live_catalogue_or_session_used",
        "package_installed",
        "provider_prompt_sent",
    ] {
        assert_eq!(identity["evidence_controls"][control], false, "{control}");
    }
    assert_eq!(
        identity["evidence_controls"]["selected_command_case_set_stable"],
        false
    );
    assert_eq!(
        identity["evidence_controls"]["selected_fourteen_commands_unchanged"],
        true
    );
    assert_eq!(protocol["source_verification"]["source_hash_mismatches"], 0);
    assert_eq!(
        protocol["source_verification"]["verified_version_path_pairs"],
        73
    );
    assert_eq!(
        protocol["source_verification"]["selected_command_cases_unchanged"],
        false
    );
    assert_eq!(
        protocol["source_verification"]["selected_fourteen_commands_unchanged"],
        true
    );
    assert_eq!(
        protocol["source_verification"]["selected_tool_schemas_unchanged"],
        true
    );
    assert_eq!(
        protocol["source_verification"]["frame_caps_unchanged"],
        true
    );
    assert_eq!(
        protocol["source_verification"]["identical_mapped_files"]["src/modes/rpc/rpc-frame.ts"],
        "9592c30c7bf12660584f67860f7abd64b752aeea15d1ce58430cfa3e5795bef3"
    );

    assert_eq!(protocol["version"], "18.13.2");
    assert_eq!(
        protocol["selected_tools"],
        json!(["read", "grep", "glob", "todo", "ask"])
    );
    assert_eq!(
        protocol["selected_rpc_commands"],
        json!([
            "negotiate_protocol",
            "set_model",
            "set_thinking_level",
            "set_auto_retry",
            "set_auto_compaction",
            "set_steering_mode",
            "set_follow_up_mode",
            "set_interrupt_mode",
            "get_state",
            "get_available_models",
            "prompt",
            "steer",
            "follow_up",
            "abort"
        ])
    );
    assert_eq!(
        protocol["selected_argv"],
        json!([
            "--mode",
            "rpc",
            "--no-session",
            "--provider",
            "<configured-provider>",
            "--model",
            "<configured-model>",
            "--tools",
            "read,grep,glob,todo,ask",
            "--no-extensions",
            "--no-skills",
            "--no-rules",
            "--no-prewalk",
            "--approval-mode",
            "always-ask"
        ])
    );
    let wire = &protocol["selected_wire_invariants"];
    assert_eq!(wire["negotiated_protocolVersion"], 2);
    assert_eq!(wire["ready_protocolVersion"], 1);
    assert_eq!(wire["ready_supportedProtocolVersions"], json!([1, 2]));
    assert_eq!(wire["maximum_physical_frame_bytes"], 1_048_576);
    assert_eq!(wire["maximum_reassembled_frame_bytes"], 67_108_864);
    assert_eq!(
        wire["response_fields"],
        json!(["id", "command", "success", "data"])
    );
    assert_eq!(
        wire["get_state_startup_fields"],
        json!([
            "model.provider",
            "model.id",
            "thinkingLevel",
            "isStreaming",
            "isCompacting",
            "steeringMode",
            "followUpMode",
            "interruptMode",
            "autoCompactionEnabled",
            "queuedMessageCount"
        ])
    );
    let boundary = &protocol["internal_read_resource_boundary"];
    assert_eq!(boundary["cfg_read"], "credential values are redacted");
    assert_eq!(boundary["read_tier"], "read by default; ssh remains exec");
    assert_eq!(
        boundary["baseline_18_12_6_schemes"],
        boundary["target_18_13_2_schemes"]
    );
    assert_eq!(
        protocol["unmapped_boundary"]["new_rpc_cases"],
        "18.13.2 adds unselected abort_branch_summary, dictation_start, dictation_stop, navigate_tree, and set_label. The selected 14 commands stay. Dictation frames emit only after dictation_start. Swallowtail maps none of them."
    );
    assert_eq!(
        protocol["selected_tool_schema"]["read"]["required"],
        json!(["path"])
    );

    assert_eq!(OH_MY_PI_PACKAGE_LATEST_QUALIFIED_VERSION, "18.13.2");
    let claim = oh_my_pi_rpc_claim();
    for version in string_array(&identity["compared_versions"]) {
        assert!(
            claim.supports(&version_binding(&version)),
            "published {version}"
        );
    }
    assert!(claim.supports(&version_binding("18.12.7")));
    for gap in ["18.0.2", "18.1.7", "18.4.7", "18.6.2", "17.4.3", "17.4.4"] {
        assert!(matches!(
            claim.assess(&version_binding(gap)),
            InterfaceCompatibilityAssessment::Incompatible
        ));
    }
    assert!(matches!(
        claim.assess(&version_binding("18.13.3")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert!(!claim.permits(&version_binding("18.13.3-rc.1")));
    assert!(!claim.permits(&version_binding("18.12.7-rc.1")));
    assert_eq!(
        claim
            .assess(&version_binding("17.4.2"))
            .behavior_revision()
            .expect("retained 17.x behavior")
            .as_str(),
        "oh-my-pi.rpc-v2-v17.2.9"
    );
    assert_eq!(
        claim
            .assess(&version_binding("18.13.2"))
            .behavior_revision()
            .expect("maintained 18.x behavior")
            .as_str(),
        "oh-my-pi.rpc-v2-v18.0.0"
    );

    assert_inventory_and_hops(&identity, &protocol, &inventory, &hops);
}

fn assert_inventory_and_hops(identity: &Value, protocol: &Value, inventory: &Value, hops: &Value) {
    assert_eq!(inventory["schema_version"], "1.0.0");
    assert_eq!(
        inventory["tree_digest_definition"],
        "SHA-256 over sorted UTF-8 path, NUL, lowercase file SHA-256, LF records"
    );
    assert_eq!(hops["from"], "18.12.6");
    assert_eq!(hops["to"], "18.13.2");
    assert_eq!(hops["hop_count"], 3);
    assert_eq!(hops["source_snapshot_version_path_count"], 73);

    let version_order = string_array(&inventory["version_order"]);
    assert_eq!(version_order, string_array(&hops["version_order"]));
    assert_eq!(version_order, string_array(&identity["compared_versions"]));
    assert_eq!(version_order.len(), 4);
    assert_eq!(string_array(&identity["published_stable_hops"]).len(), 3);

    let mut trees = BTreeMap::new();
    for version in &version_order {
        let row = &inventory["versions"][version];
        let files = string_map(&row["files"]);
        assert_eq!(
            row["file_count"].as_u64().unwrap() as usize,
            files.len(),
            "{version}"
        );
        assert_eq!(row["tree_sha256"], tree_digest(&files), "{version}");
        trees.insert(version.clone(), files);
    }
    assert_eq!(trees["18.12.6"].len(), 3342);
    assert_eq!(trees["18.13.0"].len(), 3352);
    assert_eq!(trees["18.13.1"].len(), 3352);
    assert_eq!(trees["18.13.2"].len(), 3366);
    assert_eq!(
        inventory["versions"]["18.12.6"]["tree_sha256"],
        "13057a74c47625b83d803e0b401789685a11a77c646233ff4cec571568a9de32"
    );
    assert_eq!(
        inventory["versions"]["18.13.2"]["tree_sha256"],
        "ffd81f9fcd5b79fca84ae993cfc7953dd889d2be9172d9a78e762fece8c6cb75"
    );

    let hop_rows = hops["hops"].as_array().expect("hop ledger is an array");
    assert_eq!(hop_rows.len(), 3);
    let mut expected_source_paths = BTreeSet::new();
    let mut classified_change_count = 0;
    let mut listed_published_hops = Vec::new();
    let mut previous_command_cases = protocol["rpc_command_cases_by_version"][&version_order[0]]
        .as_array()
        .expect("baseline command cases are an array")
        .iter()
        .map(|case| case.as_str().unwrap().to_owned())
        .collect::<BTreeSet<_>>();
    let mut window_additions = BTreeSet::new();
    for (index, row) in hop_rows.iter().enumerate() {
        let from = row["from"].as_str().expect("from version is text");
        let to = row["to"].as_str().expect("to version is text");
        assert_eq!(from, version_order[index]);
        assert_eq!(to, version_order[index + 1]);
        listed_published_hops.push(to.to_owned());

        let before = &trees[from];
        let after = &trees[to];
        let added = after
            .keys()
            .filter(|path| !before.contains_key(*path))
            .cloned()
            .collect::<Vec<_>>();
        let removed = before
            .keys()
            .filter(|path| !after.contains_key(*path))
            .cloned()
            .collect::<Vec<_>>();
        let changed = after
            .iter()
            .filter(|(path, digest)| before.get(*path).is_some_and(|old| old != *digest))
            .map(|(path, _)| path.clone())
            .collect::<Vec<_>>();
        assert_eq!(row["tree_delta"]["added"], json!(added), "{from} → {to}");
        assert_eq!(
            row["tree_delta"]["removed"],
            json!(removed),
            "{from} → {to}"
        );
        assert_eq!(
            row["tree_delta"]["changed"],
            json!(changed),
            "{from} → {to}"
        );

        let command_cases = string_array(&row["rpc_command_cases"]);
        assert_eq!(
            command_cases,
            string_array(&protocol["rpc_command_cases_by_version"][to]),
            "{to} RPC command cases"
        );
        let current_cases = command_cases.into_iter().collect::<BTreeSet<_>>();
        let expected_len = if to == "18.13.2" { 72 } else { 67 };
        assert_eq!(current_cases.len(), expected_len, "{to}");
        let additions = current_cases
            .difference(&previous_command_cases)
            .cloned()
            .collect::<Vec<_>>();
        let removals = previous_command_cases
            .difference(&current_cases)
            .cloned()
            .collect::<Vec<_>>();
        assert_eq!(row["rpc_command_additions"], json!(additions), "{to}");
        assert_eq!(row["rpc_command_removals"], json!(removals), "{to}");
        window_additions.extend(additions);
        previous_command_cases = current_cases;

        for change in row["classified_file_changes"]
            .as_array()
            .expect("file classifications are an array")
        {
            classified_change_count += 1;
            let path = change["path"].as_str().expect("classified path is text");
            expected_source_paths.insert(path.to_owned());
            match change["change"].as_str().expect("change kind is text") {
                "added" => {
                    assert_eq!(change["from_sha256"], Value::Null);
                    assert_eq!(change["to_sha256"], after[path]);
                }
                "removed" => {
                    assert_eq!(change["from_sha256"], before[path]);
                    assert_eq!(change["to_sha256"], Value::Null);
                }
                "changed" => {
                    assert_eq!(change["from_sha256"], before[path]);
                    assert_eq!(change["to_sha256"], after[path]);
                    assert_ne!(before[path], after[path]);
                }
                other => panic!("unknown classified change kind {other}"),
            }
        }
    }
    assert_eq!(
        listed_published_hops,
        string_array(&identity["published_stable_hops"])
    );
    assert_eq!(classified_change_count, 73);
    assert_eq!(expected_source_paths.len(), 49);
    assert_eq!(
        window_additions,
        BTreeSet::from([
            "abort_branch_summary".to_owned(),
            "dictation_start".to_owned(),
            "dictation_stop".to_owned(),
            "navigate_tree".to_owned(),
            "set_label".to_owned(),
        ])
    );
    assert_eq!(
        expected_source_paths,
        string_array(&protocol["selected_source_file_union"])
            .into_iter()
            .collect::<BTreeSet<_>>()
    );
    let selected = string_array(&protocol["selected_rpc_commands"])
        .into_iter()
        .collect::<BTreeSet<_>>();
    let target_cases = string_array(&protocol["rpc_command_cases_by_version"]["18.13.2"])
        .into_iter()
        .collect::<BTreeSet<_>>();
    assert_eq!(target_cases.len(), 72);
    assert!(selected.is_subset(&target_cases));
    assert_eq!(
        target_cases
            .difference(&selected)
            .cloned()
            .collect::<Vec<_>>(),
        string_array(&protocol["unselected_rpc_command_cases_at_target"])
    );
}

fn parse(source: &str) -> Value {
    serde_json::from_str(source).expect("identity fixture is valid JSON")
}

fn assert_keys(value: &Value, expected: &[&str]) {
    let actual = value
        .as_object()
        .expect("value is an object")
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let expected = expected.iter().copied().collect::<BTreeSet<_>>();
    assert_eq!(actual, expected);
}

fn string_array(value: &Value) -> Vec<String> {
    value
        .as_array()
        .expect("value is an array")
        .iter()
        .map(|item| item.as_str().expect("array item is text").to_owned())
        .collect()
}

fn string_map(value: &Value) -> BTreeMap<String, String> {
    value
        .as_object()
        .expect("value is an object")
        .iter()
        .map(|(path, digest)| {
            let digest = digest.as_str().expect("file digest is text").to_owned();
            assert!(is_sha256(&digest), "{path} has a SHA-256 digest");
            (path.clone(), digest)
        })
        .collect()
}

fn tree_digest(files: &BTreeMap<String, String>) -> String {
    let mut digest = Sha256::new();
    for (path, file_sha256) in files {
        digest.update(path.as_bytes());
        digest.update([0]);
        digest.update(file_sha256.as_bytes());
        digest.update(b"\n");
    }
    hex_digest(&digest.finalize())
}

fn assert_sha256(contents: &[u8], expected: &str) {
    let digest = Sha256::digest(contents);
    assert_eq!(hex_digest(&digest), expected);
}

fn hex_digest(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn version_binding(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("valid version")
}
