use std::collections::{BTreeMap, BTreeSet};

use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use swallowtail_adapter_oh_my_pi::{
    OH_MY_PI_PACKAGE_AXIS, OH_MY_PI_PACKAGE_LATEST_QUALIFIED_VERSION, oh_my_pi_rpc_claim,
};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

const IDENTITY: &str = include_str!("fixtures/oh-my-pi-18.8.3/identity.json");
const PROTOCOL: &str = include_str!("fixtures/oh-my-pi-18.8.3/protocol.json");
const INVENTORY: &str = include_str!("fixtures/oh-my-pi-18.8.3/complete-tree-inventory.json");
const HOPS: &str = include_str!("fixtures/oh-my-pi-18.8.3/hop-ledger.json");

const IDENTITY_SHA256: &str = "ec35f23ee8e5771e55b9e1b938f679f78d6b3c14bf0d2848c15b7fc573058e8a";
const PROTOCOL_SHA256: &str = "68e227ad13778e85d8877eff494348b6b2295f0ebb7487db5a774d2d69cb5f5b";
const INVENTORY_SHA256: &str = "402fa5d7f54e2f348d2627afb9fd0fbad9f0b696afbe6b77cc5eef6dc10cea69";
const HOPS_SHA256: &str = "de22ae7b293ab2565af2ea8656c0b70d9a8cebeb6ac447dfc770f0e12d97afc4";

#[test]
fn official_18_8_3_identity_and_every_published_hop_are_frozen() {
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
            "promotion_brief_target",
            "published_stable_hop_count",
            "published_stable_hops",
            "runtime_observations",
            "schema_version",
            "source_identity",
            "synthetic_later_stable_after_official",
            "synthetic_later_stable_note",
            "unpublished_prior_synthetic_note",
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
    assert_keys(
        &inventory,
        &[
            "root_definition",
            "schema_version",
            "tree_digest_definition",
            "version_order",
            "versions",
        ],
    );
    assert_keys(
        &hops,
        &[
            "axis",
            "classification_policy",
            "from",
            "gaps",
            "hop_count",
            "hops",
            "schema_version",
            "source_snapshot_version_path_count",
            "to",
            "version_order",
        ],
    );

    assert_eq!(identity["axis"], OH_MY_PI_PACKAGE_AXIS);
    assert_eq!(identity["npm_package"], "@oh-my-pi/pi-coding-agent");
    assert_eq!(identity["official_latest_at_identity"], "18.8.3");
    assert_eq!(identity["previous_qualified_ceiling"], "18.2.7");
    assert_eq!(identity["synthetic_later_stable_after_official"], "18.8.4");
    assert_eq!(identity["published_stable_hop_count"], 32);
    assert_eq!(identity["observed_at"], "2026-10-08T02:28:44.734Z");
    assert_eq!(identity["artifact"]["version"], "18.8.3");
    assert_eq!(identity["artifact"]["name"], "@oh-my-pi/pi-coding-agent");
    assert_eq!(identity["artifact"]["npm_git_head"], Value::Null);
    assert_eq!(
        identity["artifact"]["tarball_integrity"],
        "sha512-v36DfSnLgOQkoglFIh9/5GCZPAwVLD0WHfMEoIf3jE6gKyXC30DJdGPTI7aNx+rqVRS9lkxDc/nUoqyi1qmZug=="
    );
    assert_eq!(
        identity["artifact"]["tarball_sha256"],
        "90ee818ddd5000405446e2396d4d7baeadabf04aff141fc311d47c99f7de17d2"
    );
    assert_eq!(
        identity["artifact"]["dist_cli_sha256"],
        "ab1482816804bc7b73d1b1407f62e06656657f9d9636e860466ab004bd20cef2"
    );
    assert_eq!(identity["artifact"]["package_file_count"], 3179);
    assert_eq!(identity["artifact"]["engines"]["bun"], ">=1.3.14");
    assert_eq!(identity["official_channels"]["channel_agreement"], true);
    assert_eq!(identity["official_channels"]["npm"]["latest"], "18.8.3");
    assert_eq!(
        identity["official_channels"]["github_releases"]["latest_release_tag"],
        "v18.8.3"
    );
    assert_eq!(
        identity["official_channels"]["latest_movement_before_identity_commit"]["new_hop_included"],
        "18.8.3"
    );
    assert_eq!(
        identity["source_identity"]["target_github_commit"],
        "3e3c488a58d294e3a10051da588628e2cfb9d35c"
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
        "18.0.0..=18.8.3"
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
        identity["identity_decision"]["qualify_all_published_intermediates"],
        true
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
        protocol["source_verification"]["unmatched_changed_source_paths"],
        0
    );
    assert_eq!(
        protocol["source_verification"]["verified_version_path_pairs"],
        473
    );

    assert_eq!(protocol["version"], "18.8.3");
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
    let tool_schema = &protocol["selected_tool_schema"];
    assert_keys(tool_schema, &["ask", "glob", "grep", "read", "todo"]);
    assert_keys(&tool_schema["read"], &["optional", "required"]);
    assert_eq!(tool_schema["read"]["required"], json!(["path"]));
    assert_eq!(tool_schema["read"]["optional"], json!([]));
    assert_keys(&tool_schema["grep"], &["optional", "required"]);
    assert_eq!(tool_schema["grep"]["required"], json!(["pattern"]));
    assert_eq!(
        tool_schema["grep"]["optional"],
        json!(["path", "case", "gitignore", "skip"])
    );
    assert_keys(&tool_schema["glob"], &["optional", "required"]);
    assert_eq!(tool_schema["glob"]["required"], json!([]));
    assert_eq!(
        tool_schema["glob"]["optional"],
        json!(["path", "hidden", "gitignore", "limit"])
    );
    assert_keys(&tool_schema["todo"], &["op_enum", "optional", "required"]);
    assert_eq!(tool_schema["todo"]["required"], json!(["op"]));
    assert_eq!(
        tool_schema["todo"]["op_enum"],
        json!([
            "init", "start", "done", "rm", "drop", "block", "unblock", "append", "view"
        ])
    );
    assert_eq!(
        tool_schema["todo"]["optional"],
        json!(["list", "task", "phase", "items", "reason"])
    );
    assert_keys(
        &tool_schema["ask"],
        &["option_fields", "question_fields", "required"],
    );
    assert_eq!(tool_schema["ask"]["required"], json!(["questions"]));
    assert_eq!(
        tool_schema["ask"]["question_fields"],
        json!([
            "id",
            "question",
            "header",
            "options",
            "multi",
            "recommended"
        ])
    );
    assert_eq!(
        tool_schema["ask"]["option_fields"],
        json!(["label", "description", "preview"])
    );
    assert_eq!(
        protocol["selected_wire_invariants"]["negotiated_protocolVersion"],
        2
    );
    assert_eq!(
        protocol["selected_wire_invariants"]["maximum_physical_frame_bytes"],
        1_048_576
    );
    assert_eq!(
        protocol["selected_wire_invariants"]["maximum_reassembled_frame_bytes"],
        67_108_864
    );
    assert_keys(
        &protocol["selected_wire_invariants"],
        &[
            "maximum_physical_frame_bytes",
            "maximum_reassembled_frame_bytes",
            "negotiated_protocolVersion",
            "provider_failure_marker",
            "ready_protocolVersion",
            "ready_supportedProtocolVersions",
            "response_fields",
            "rpc_chunk_fields",
            "strict_lf_jsonl",
            "terminal_event",
            "terminal_non_true",
            "tool_event_fields",
            "ui_dialog_methods",
            "ui_display_methods",
            "usage_fields",
        ],
    );
    assert_keys(
        &protocol["internal_read_resource_boundary"],
        &[
            "baseline_18_2_7_schemes",
            "cfg_read",
            "cfg_write",
            "claim_boundary",
            "proc_read",
            "proc_write",
            "read_tier",
            "selected_tool_exclusions",
            "target_18_8_3_schemes",
        ],
    );
    assert_eq!(
        protocol["internal_read_resource_boundary"]["baseline_18_2_7_schemes"],
        json!([
            "agent", "artifact", "history", "issue", "local", "mcp", "memory", "omp", "pr", "rule",
            "security", "skill", "ssh", "vault", "xd"
        ])
    );
    assert_eq!(
        protocol["internal_read_resource_boundary"]["target_18_8_3_schemes"],
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
        protocol["internal_read_resource_boundary"]["cfg_read"],
        "credential values are redacted"
    );
    assert_eq!(
        protocol["internal_read_resource_boundary"]["read_tier"],
        "read by default; ssh remains exec"
    );
    assert_eq!(
        protocol["internal_read_resource_boundary"]["selected_tool_exclusions"],
        json!(["write", "edit", "bash", "task"])
    );

    assert_eq!(OH_MY_PI_PACKAGE_LATEST_QUALIFIED_VERSION, "18.8.3");
    let claim = oh_my_pi_rpc_claim();
    for version in ["18.0.0", "18.2.7", "18.2.8", "18.2.9", "18.8.2", "18.8.3"] {
        assert!(claim.supports(&version_binding(version)), "{version}");
    }
    for gap in ["18.0.2", "18.1.7", "18.4.7", "18.6.2"] {
        assert!(matches!(
            claim.assess(&version_binding(gap)),
            InterfaceCompatibilityAssessment::Incompatible
        ));
    }
    assert!(matches!(
        claim.assess(&version_binding("18.8.4")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
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
            .assess(&version_binding("18.0.0"))
            .behavior_revision()
            .expect("maintained 18.x behavior")
            .as_str(),
        "oh-my-pi.rpc-v2-v18.0.0"
    );
    for version in string_array(&identity["compared_versions"]) {
        assert!(
            claim.supports(&version_binding(&version)),
            "published {version}"
        );
    }

    assert_inventory_and_hops(&identity, &protocol, &inventory, &hops);
}

fn assert_inventory_and_hops(identity: &Value, protocol: &Value, inventory: &Value, hops: &Value) {
    assert_eq!(inventory["schema_version"], "1.0.0");
    assert_eq!(
        inventory["tree_digest_definition"],
        "SHA-256 over sorted UTF-8 path, NUL, lowercase file SHA-256, LF records"
    );
    assert_eq!(hops["from"], "18.2.7");
    assert_eq!(hops["to"], "18.8.3");
    assert_eq!(hops["hop_count"], 32);
    assert_eq!(hops["source_snapshot_version_path_count"], 473);

    let version_order = string_array(&inventory["version_order"]);
    assert_eq!(version_order, string_array(&hops["version_order"]));
    assert_eq!(version_order, string_array(&identity["compared_versions"]));
    assert_eq!(version_order.len(), 33);
    assert_eq!(string_array(&identity["published_stable_hops"]).len(), 32);

    let mut trees = BTreeMap::new();
    for version in &version_order {
        let row = &inventory["versions"][version];
        assert_keys(row, &["file_count", "files", "tree_sha256"]);
        let files = string_map(&row["files"]);
        assert_eq!(
            row["file_count"].as_u64().unwrap() as usize,
            files.len(),
            "{version}"
        );
        assert_eq!(row["tree_sha256"], tree_digest(&files), "{version}");
        trees.insert(version.clone(), files);
    }
    assert_eq!(trees["18.2.7"].len(), 2700);
    assert_eq!(trees["18.8.3"].len(), 3179);
    assert_eq!(
        inventory["versions"]["18.8.3"]["tree_sha256"],
        "05d484a933e70894931f66fec4a9ef6bcf493584872f83e230063f011d85c551"
    );

    let hop_rows = hops["hops"].as_array().expect("hop ledger is an array");
    assert_eq!(hop_rows.len(), 32);
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
        assert_keys(
            row,
            &[
                "classified_file_changes",
                "from",
                "published_at",
                "rpc_command_additions",
                "rpc_command_cases",
                "rpc_command_removals",
                "selected_surface_assessment",
                "selected_surface_class",
                "to",
                "tree_delta",
            ],
        );
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
        assert_eq!(
            row["tree_delta"]["added"],
            json!(added),
            "{from} → {to} added"
        );
        assert_eq!(
            row["tree_delta"]["removed"],
            json!(removed),
            "{from} → {to} removed"
        );
        assert_eq!(
            row["tree_delta"]["changed"],
            json!(changed),
            "{from} → {to} changed"
        );

        let command_cases = string_array(&row["rpc_command_cases"]);
        assert_eq!(
            command_cases,
            string_array(&protocol["rpc_command_cases_by_version"][to]),
            "{to} RPC command cases"
        );
        let current_cases = command_cases.into_iter().collect::<BTreeSet<_>>();
        let additions = current_cases
            .difference(&previous_command_cases)
            .cloned()
            .collect::<Vec<_>>();
        let removals = previous_command_cases
            .difference(&current_cases)
            .cloned()
            .collect::<Vec<_>>();
        assert_eq!(
            row["rpc_command_additions"],
            json!(additions),
            "{to} command additions"
        );
        assert_eq!(
            row["rpc_command_removals"],
            json!(removals),
            "{to} command removals"
        );
        previous_command_cases = current_cases;

        for change in row["classified_file_changes"]
            .as_array()
            .expect("file classifications are an array")
        {
            classified_change_count += 1;
            assert_keys(
                change,
                &[
                    "change",
                    "classification",
                    "from_sha256",
                    "note",
                    "path",
                    "to_sha256",
                ],
            );
            let path = change["path"].as_str().expect("classified path is text");
            expected_source_paths.insert(path.to_owned());
            match change["change"].as_str().expect("change kind is text") {
                "added" => {
                    assert_eq!(change["from_sha256"], Value::Null);
                    assert_eq!(change["to_sha256"], after[path]);
                    assert!(!before.contains_key(path));
                }
                "removed" => {
                    assert_eq!(change["from_sha256"], before[path]);
                    assert_eq!(change["to_sha256"], Value::Null);
                    assert!(!after.contains_key(path));
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
    assert_eq!(classified_change_count, 360);
    assert_eq!(expected_source_paths.len(), 89);
    assert_eq!(
        expected_source_paths,
        string_array(&protocol["selected_source_file_union"])
            .into_iter()
            .collect::<BTreeSet<_>>()
    );
    let selected = string_array(&protocol["selected_rpc_commands"])
        .into_iter()
        .collect::<BTreeSet<_>>();
    let target_cases = string_array(&protocol["rpc_command_cases_by_version"]["18.8.3"])
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
