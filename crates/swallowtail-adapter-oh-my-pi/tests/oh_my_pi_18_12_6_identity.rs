use std::collections::{BTreeMap, BTreeSet};

use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use swallowtail_adapter_oh_my_pi::{
    OH_MY_PI_PACKAGE_AXIS, OH_MY_PI_PACKAGE_LATEST_QUALIFIED_VERSION, oh_my_pi_rpc_claim,
};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

const IDENTITY: &str = include_str!("fixtures/oh-my-pi-18.12.6/identity.json");
const PROTOCOL: &str = include_str!("fixtures/oh-my-pi-18.12.6/protocol.json");
const INVENTORY: &str = include_str!("fixtures/oh-my-pi-18.12.6/complete-tree-inventory.json");
const HOPS: &str = include_str!("fixtures/oh-my-pi-18.12.6/hop-ledger.json");

const IDENTITY_SHA256: &str = "0de09b5f76e57689bc64137006a05ee6e6a0fbebd171b13beba49168fd8cc459";
const PROTOCOL_SHA256: &str = "2eb53cd07f42a6a007c2bdeba6e077906c18d34735df2010cfe0534c2be86ad3";
const INVENTORY_SHA256: &str = "4fcb81150e9b48cfdc38a9093b51a623ea2be68964fa4e11e9f6a95142d91a1c";
const HOPS_SHA256: &str = "8db565735f622edcd92216bf2dc953f00eeee1a5beb9b9f6dfac2b6210cc6a04";

#[test]
fn official_18_12_6_identity_and_every_published_hop_are_frozen() {
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
    assert_eq!(identity["official_latest_at_identity"], "18.12.6");
    assert_eq!(identity["previous_qualified_ceiling"], "18.8.3");
    assert_eq!(identity["synthetic_later_stable_after_official"], "18.12.7");
    assert_eq!(identity["published_stable_hop_count"], 25);
    assert_eq!(identity["observed_at"], "2026-10-26T10:27:30.875Z");
    assert_eq!(identity["artifact"]["version"], "18.12.6");
    assert_eq!(identity["artifact"]["npm_git_head"], Value::Null);
    assert_eq!(
        identity["artifact"]["tarball_integrity"],
        "sha512-Wo+FcJ7A3RY/OFuafj4hcdIAXc/W7NMxbanwQh3Cmy8s57+ihLOLgAv+HROxHBkssEMg0MaScwkQ9xwqLPiC/g=="
    );
    assert_eq!(
        identity["artifact"]["tarball_sha256"],
        "8088b050a405cc4073c8f4b32d338ebb0ecc2f0d18be05852f20960b73e819e1"
    );
    assert_eq!(
        identity["artifact"]["dist_cli_sha256"],
        "4475566bfd0a89b045d4b41c7efe905360aac9662d84746a3e8ad5e735542078"
    );
    assert_eq!(identity["artifact"]["package_file_count"], 3342);
    assert_eq!(identity["artifact"]["engines"]["bun"], ">=1.3.14");
    assert_eq!(identity["official_channels"]["channel_agreement"], true);
    assert_eq!(identity["official_channels"]["npm"]["latest"], "18.12.6");
    assert_eq!(
        identity["official_channels"]["github_releases"]["latest_release_tag"],
        "v18.12.6"
    );
    assert_eq!(
        identity["source_identity"]["target_github_commit"],
        "45c35e40c46166dc726ccc52f5d27f40f2449f99"
    );
    assert_eq!(
        identity["source_identity"]["baseline_tree_sha256_matches_research_382"],
        "05d484a933e70894931f66fec4a9ef6bcf493584872f83e230063f011d85c551"
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
        "18.0.0..=18.12.6"
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
        identity["claim_at_observation"]["posture"],
        "allow_unverified"
    );
    assert_eq!(
        identity["claim_at_observation"]["segments"][1]["range"],
        "18.0.0..=18.8.3"
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
    assert_eq!(protocol["source_verification"]["source_hash_mismatches"], 0);
    assert_eq!(
        protocol["source_verification"]["verified_version_path_pairs"],
        189
    );
    assert_eq!(
        protocol["source_verification"]["selected_command_cases_unchanged"],
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

    assert_eq!(protocol["version"], "18.12.6");
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
    assert_eq!(wire["set_model_result_fields"], json!(["provider", "id"]));
    assert_eq!(
        wire["usage_fields"],
        json!(["input", "output", "cacheRead", "cacheWrite"])
    );
    assert_eq!(
        wire["ui_dialog_methods"],
        json!(["select", "confirm", "input", "editor"])
    );
    let boundary = &protocol["internal_read_resource_boundary"];
    assert_eq!(boundary["cfg_read"], "credential values are redacted");
    assert_eq!(boundary["read_tier"], "read by default; ssh remains exec");
    assert_eq!(
        boundary["target_18_12_6_schemes"],
        json!([
            "agent",
            "artifact",
            "attachment",
            "cfg",
            "conflict",
            "history",
            "issue",
            "local",
            "mcp",
            "memory",
            "mr",
            "omp",
            "pr",
            "proc",
            "rule",
            "security",
            "skill",
            "ssh",
            "vault",
            "xd"
        ])
    );
    assert_eq!(
        protocol["selected_tool_schema"]["read"]["required"],
        json!(["path"])
    );

    assert_eq!(OH_MY_PI_PACKAGE_LATEST_QUALIFIED_VERSION, "18.12.6");
    let claim = oh_my_pi_rpc_claim();
    for version in string_array(&identity["compared_versions"]) {
        assert!(
            claim.supports(&version_binding(&version)),
            "published {version}"
        );
    }
    for gap in ["18.0.2", "18.1.7", "18.4.7", "18.6.2", "17.4.3", "17.4.4"] {
        assert!(matches!(
            claim.assess(&version_binding(gap)),
            InterfaceCompatibilityAssessment::Incompatible
        ));
    }
    assert!(matches!(
        claim.assess(&version_binding("18.12.7")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert!(!claim.permits(&version_binding("18.12.7-rc.1")));
    assert!(!claim.permits(&version_binding("18.8.4-rc.1")));
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
            .assess(&version_binding("18.12.6"))
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
    assert_eq!(hops["from"], "18.8.3");
    assert_eq!(hops["to"], "18.12.6");
    assert_eq!(hops["hop_count"], 25);
    assert_eq!(hops["source_snapshot_version_path_count"], 189);

    let version_order = string_array(&inventory["version_order"]);
    assert_eq!(version_order, string_array(&hops["version_order"]));
    assert_eq!(version_order, string_array(&identity["compared_versions"]));
    assert_eq!(version_order.len(), 26);
    assert_eq!(string_array(&identity["published_stable_hops"]).len(), 25);

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
    assert_eq!(trees["18.8.3"].len(), 3179);
    assert_eq!(trees["18.12.6"].len(), 3342);
    assert_eq!(
        inventory["versions"]["18.8.3"]["tree_sha256"],
        "05d484a933e70894931f66fec4a9ef6bcf493584872f83e230063f011d85c551"
    );
    assert_eq!(
        inventory["versions"]["18.12.6"]["tree_sha256"],
        "13057a74c47625b83d803e0b401789685a11a77c646233ff4cec571568a9de32"
    );

    let hop_rows = hops["hops"].as_array().expect("hop ledger is an array");
    assert_eq!(hop_rows.len(), 25);
    let mut expected_source_paths = BTreeSet::new();
    let mut classified_change_count = 0;
    let mut listed_published_hops = Vec::new();
    let mut previous_command_cases = protocol["rpc_command_cases_by_version"][&version_order[0]]
        .as_array()
        .expect("baseline command cases are an array")
        .iter()
        .map(|case| case.as_str().unwrap().to_owned())
        .collect::<BTreeSet<_>>();
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
        assert_eq!(current_cases.len(), 67, "{to}");
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
    assert_eq!(classified_change_count, 189);
    assert_eq!(expected_source_paths.len(), 54);
    assert_eq!(
        expected_source_paths,
        string_array(&protocol["selected_source_file_union"])
            .into_iter()
            .collect::<BTreeSet<_>>()
    );
    let selected = string_array(&protocol["selected_rpc_commands"])
        .into_iter()
        .collect::<BTreeSet<_>>();
    let target_cases = string_array(&protocol["rpc_command_cases_by_version"]["18.12.6"])
        .into_iter()
        .collect::<BTreeSet<_>>();
    assert_eq!(target_cases.len(), 67);
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
