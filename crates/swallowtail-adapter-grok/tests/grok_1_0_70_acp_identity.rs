//! Frozen package, runtime, and selected-surface identity for Grok Build ACP 1.0.70.
//!
//! Official `1.0.47` through `1.0.70` stay `UnverifiedNewer`. Interned dispatch
//! tables match `1.0.46`; selected handler xref bodies do not.

use std::collections::BTreeSet;

use serde_json::Value;
use sha2::{Digest, Sha256};
use swallowtail_adapter_grok::{
    GROK_BUILD_ACP_LATEST_QUALIFIED_VERSION, grok_build_acp_claim, grok_build_catalogue_claim,
    registered_tool,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceNewerVersionPosture, InterfaceSupportStatus,
    InterfaceVersion, InterfaceVersionScheme,
};
use swallowtail_runtime::RegisteredToolRouteQualification;

const IDENTITY: &str = include_str!("fixtures/grok-1.0.70-acp/identity.json");
const INVENTORY: &str = include_str!("fixtures/grok-1.0.70-acp/dist-inventory.json");
const PROTOCOL: &str = include_str!("fixtures/grok-1.0.70-acp/protocol.json");
const CONTROL_FLOW: &str = include_str!("fixtures/grok-1.0.70-acp/control-flow.json");
const PRIOR_IDENTITY: &str = include_str!("fixtures/grok-1.0.46/identity.json");
const PRIOR_PROTOCOL: &str = include_str!("fixtures/grok-1.0.46/protocol.json");

const LINUX_LITERAL_DIGEST: &str =
    "4cceb3e6fc7893dd2e26e8487b5e7266c4040d2fd189ac38ea4569365a10b311";
const DARWIN_LITERAL_DIGEST: &str =
    "4a548e4dc7687941e640dc410146a0ed99dc8b95724871317ffe37a14fe3dfb3";

fn json(source: &str) -> Value {
    serde_json::from_str(source).expect("identity fixture is valid JSON")
}

fn strings(value: &Value) -> Vec<&str> {
    value
        .as_array()
        .expect("array")
        .iter()
        .map(|value| value.as_str().expect("string value"))
        .collect()
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("version")
}

fn sha256_lines(values: &[String]) -> String {
    let mut bytes = values.join("\n");
    bytes.push('\n');
    format!("{:x}", Sha256::digest(bytes.as_bytes()))
}

#[test]
fn official_latest_and_every_selected_stable_hop_are_frozen() {
    let identity = json(IDENTITY);
    let prior = json(PRIOR_IDENTITY);
    assert_eq!(identity["axis"], "grok-build.executable");
    assert_eq!(identity["npm_package"], "@xai-official/grok");
    assert_eq!(identity["previous_ceiling"], "1.0.46");
    assert_eq!(identity["official_stable"], "1.0.70");
    assert_eq!(identity["npm_dist_tags"]["latest"], "1.0.70");
    assert_eq!(identity["npm_dist_tags"]["alpha"], "1.0.70");
    assert_eq!(identity["unpublished_successor"], "1.0.71");
    assert_eq!(identity["observed_at"], "2026-10-26");
    assert_eq!(
        identity["checkpoint_main"],
        "f3f72db8b9abd8e8b4c21fada42a5e3414678c8d"
    );
    let hops = identity["hops"].as_array().expect("hops");
    assert_eq!(hops.len(), 24);
    let mut previous = 46;
    for hop in hops {
        let current = hop["version"].as_str().expect("version");
        let minor: u32 = current.trim_start_matches("1.0.").parse().expect("minor");
        assert_eq!(minor, previous + 1);
        previous = minor;
        let build_id = hop["runtime_build_id"].as_str().expect("build id");
        assert_eq!(build_id.len(), 12);
        assert!(build_id.bytes().all(|byte| byte.is_ascii_hexdigit()));
        assert_eq!(
            hop["version_banner_literal"],
            format!("clicurrentVersion{current} ({build_id})channel")
        );
        assert_eq!(hop["observed_banner"], format!("{current} ({build_id})"));
        assert_eq!(hop["wrapper"]["file_count"], 5);
        assert!(
            hop["wrapper"]["integrity"]
                .as_str()
                .unwrap()
                .starts_with("sha512-")
        );
        if (47..=49).contains(&minor) {
            assert_eq!(&hop["gitHead"].as_str().unwrap()[..12], build_id);
        } else {
            assert!(hop["gitHead"].is_null(), "{current} omits gitHead");
        }
        for platform in ["linux_x64", "darwin_arm64"] {
            let artifact = &hop[platform];
            assert_eq!(artifact["file_count"], 4);
            assert!(
                artifact["integrity"]
                    .as_str()
                    .unwrap()
                    .starts_with("sha512-")
            );
            assert_eq!(artifact["executable_sha256"].as_str().unwrap().len(), 64);
            assert_eq!(artifact["brotli_sha256"].as_str().unwrap().len(), 64);
        }
    }
    assert_eq!(previous, 70);

    let prior_hop = prior["hops"]
        .as_array()
        .unwrap()
        .iter()
        .find(|hop| hop["version"] == "1.0.46")
        .expect("prior 1.0.46 hop");
    let reproduction = &identity["prior_ceiling_reproduction"];
    assert_eq!(reproduction["matches_research_385_identity"], true);
    assert_eq!(
        reproduction["linux_x64_executable_sha256"],
        prior_hop["linux_x64"]["executable_sha256"]
    );
    assert_eq!(
        reproduction["darwin_arm64_executable_sha256"],
        prior_hop["darwin_arm64"]["executable_sha256"]
    );
    assert_eq!(
        reproduction["linux_x64_brotli_sha256"],
        prior_hop["linux_x64"]["brotli_sha256"]
    );
    assert_eq!(
        reproduction["darwin_arm64_brotli_sha256"],
        prior_hop["darwin_arm64"]["brotli_sha256"]
    );
    assert_eq!(
        reproduction["scanned_linux_executable_sha256"],
        reproduction["linux_x64_executable_sha256"]
    );
    assert_eq!(
        reproduction["scanned_darwin_executable_sha256"],
        reproduction["darwin_arm64_executable_sha256"]
    );
    assert_eq!(identity["host_observation"]["grok_on_path"], false);
    assert_eq!(identity["host_observation"]["version_probed"], false);
    assert_eq!(identity["decision"]["shape"], "stop");
    assert_eq!(identity["decision"]["keep_latest_qualified"], "1.0.46");
    assert_eq!(identity["decision"]["flatten_with_catalogue"], false);
    assert_eq!(identity["decision"]["new_milestone"], false);
    assert_eq!(
        identity["decision"]["official_stable_classification"],
        "unverified_newer"
    );
    assert_eq!(
        identity["catalogue_hash_reproduction"]["darwin_runtime_and_inventory_match_every_hop"],
        true
    );
    assert_eq!(
        identity["catalogue_hash_reproduction"]["linux_x64_inventory_in_catalogue_corpus"],
        false
    );
}

#[test]
fn package_trees_change_only_the_named_files() {
    let inventory = json(INVENTORY);
    assert_eq!(inventory["baseline"], "1.0.46");
    let compared = strings(&inventory["compared"]);
    assert_eq!(compared.first().copied(), Some("1.0.46"));
    assert_eq!(compared.last().copied(), Some("1.0.70"));
    assert_eq!(compared.len(), 25);
    let trees = inventory["complete_trees"].as_object().unwrap();
    for version in &compared {
        for (group, count) in [("wrapper", 5), ("linux_x64", 4), ("darwin_arm64", 4)] {
            assert_eq!(
                trees[*version][group].as_object().unwrap().len(),
                count,
                "{version} {group}"
            );
        }
    }
    let mut previous = "1.0.46";
    for hop in inventory["hops"].as_array().unwrap() {
        assert_eq!(hop["from"], previous);
        let version = hop["to"].as_str().unwrap();
        for side in ["wrapper", "linux_x64", "darwin_arm64"] {
            assert_eq!(
                hop[side]["added"],
                serde_json::json!([]),
                "{version} {side}"
            );
            assert_eq!(
                hop[side]["removed"],
                serde_json::json!([]),
                "{version} {side}"
            );
            assert_eq!(
                inventory["package_file_counts"][previous][side],
                trees[previous][side].as_object().unwrap().len()
            );
            assert_eq!(
                hop[side]["changed"].as_array().unwrap().len()
                    + hop[side]["identical"].as_array().unwrap().len(),
                trees[previous][side].as_object().unwrap().len(),
                "{version} {side} changed+identical"
            );
        }
        assert_eq!(
            strings(&hop["linux_x64"]["changed"]),
            ["bin/grok.br", "package.json"]
        );
        assert_eq!(
            strings(&hop["darwin_arm64"]["changed"]),
            ["bin/grok.br", "package.json"]
        );
        assert_eq!(
            strings(&hop["linux_x64"]["identical"]),
            ["README.md", "THIRD_PARTY_NOTICES.md"]
        );
        assert_eq!(
            strings(&hop["darwin_arm64"]["identical"]),
            ["README.md", "THIRD_PARTY_NOTICES.md"]
        );
        let wrapper = strings(&hop["wrapper"]["changed"]);
        if version == "1.0.56" {
            assert_eq!(wrapper, ["README.md", "package.json"]);
        } else {
            assert_eq!(wrapper, ["package.json"]);
        }
        previous = version;
    }
    assert_eq!(previous, "1.0.70");
    assert_eq!(inventory["readme_change"]["acp_wire"], false);
    assert_eq!(inventory["package_file_counts"]["1.0.70"]["wrapper"], 5);
    assert_eq!(inventory["package_file_counts"]["1.0.70"]["linux_x64"], 4);
    assert_eq!(
        inventory["package_file_counts"]["1.0.70"]["darwin_arm64"],
        4
    );
}

#[test]
fn selected_surface_and_acp_paths_stay_inside_the_prior_mapping() {
    let protocol = json(PROTOCOL);
    let prior = json(PRIOR_PROTOCOL);
    assert_eq!(
        protocol["selected_literal_presence"],
        prior["selected_literal_presence"]
    );
    assert_eq!(
        protocol["darwin_arm64_selected_literal_presence"],
        prior["darwin_arm64_selected_literal_presence"]
    );
    assert_eq!(
        protocol["selected_literal_presence_digest"],
        "4cceb3e6fc7893dd2e26e8487b5e7266c4040d2fd189ac38ea4569365a10b311"
    );
    assert_eq!(
        protocol["darwin_arm64_selected_literal_presence_digest"],
        "4a548e4dc7687941e640dc410146a0ed99dc8b95724871317ffe37a14fe3dfb3"
    );
    assert_eq!(protocol["selected_literal_key_count"], 92);
    assert_eq!(
        protocol["launch_argv"],
        serde_json::json!(["--no-auto-update", "agent", "stdio"])
    );
    let public = &protocol["public_operation_substrings"];
    assert!(
        strings(&public["linux_x64_absent_at_1_0_46_and_later"])
            .contains(&"session/request_user_input")
    );
    assert!(
        strings(&public["darwin_arm64_absent_at_1_0_46_and_later"])
            .contains(&"session/request_user_input")
    );
    assert!(!strings(&public["linux_x64_present_at_1_0_46_and_later"]).contains(&"agentVersion"));
    assert!(strings(&public["darwin_arm64_present_at_1_0_46_and_later"]).contains(&"agentVersion"));

    let models = &protocol["model_document"];
    assert_eq!(
        models["sha256"],
        "9d6924ec760a94f91902f60adb8bcf2cd9d3ab86891a1c83e8c01a092e56490a"
    );
    assert_eq!(models["bytes"], 2323);
    assert_eq!(models["default"], "grok-4.6");
    assert_eq!(
        models["model_ids"],
        serde_json::json!(["grok-4.6", "grok-4.5"])
    );
    assert_eq!(
        models["reasoning_effort_values"],
        prior["model_document"]["reasoning_effort_values"]
    );
    assert_eq!(models["copy_count_samples"]["unique_spans"], 2);

    let mut paths: BTreeSet<String> = strings(&protocol["acp_module_paths_at_1_0_46"])
        .into_iter()
        .map(str::to_owned)
        .collect();
    assert_eq!(paths.len(), 86);
    let per_hop = protocol["per_hop"].as_object().unwrap();
    assert_eq!(
        per_hop["1.0.46"]["acp_module_path_set_digest"],
        sha256_lines(&paths.iter().cloned().collect::<Vec<_>>())
    );
    let mut cursor = "1.0.46";
    for version in strings(&protocol["compared"]).into_iter().skip(1) {
        if let Some(delta) = protocol["acp_path_deltas_after_1_0_46"]
            .as_array()
            .unwrap()
            .iter()
            .find(|delta| delta["to"] == version)
        {
            assert_eq!(delta["from"], cursor);
            for removed in strings(&delta["removed"]) {
                assert!(paths.remove(removed), "missing removal {removed}");
            }
            for added in strings(&delta["added"]) {
                assert!(paths.insert(added.to_owned()), "duplicate add {added}");
                assert!(protocol["acp_path_change_classification"][added].is_string());
            }
            for removed in strings(&delta["removed"]) {
                assert!(protocol["acp_path_change_classification"][removed].is_string());
            }
        }
        let owned: Vec<String> = paths.iter().cloned().collect();
        assert_eq!(per_hop[version]["acp_module_path_count"], owned.len());
        assert_eq!(
            per_hop[version]["acp_module_path_set_digest"],
            sha256_lines(&owned)
        );
        assert_eq!(
            per_hop[version]["linux_selected_literal_presence_digest"],
            LINUX_LITERAL_DIGEST
        );
        assert_eq!(
            per_hop[version]["darwin_selected_literal_presence_digest"],
            DARWIN_LITERAL_DIGEST
        );
        assert_eq!(per_hop[version]["model_document_sha256"], models["sha256"]);
        assert_eq!(
            per_hop[version]["linux_public_operation_substring_digest"],
            per_hop["1.0.46"]["linux_public_operation_substring_digest"]
        );
        assert_eq!(
            per_hop[version]["darwin_public_operation_substring_digest"],
            per_hop["1.0.46"]["darwin_public_operation_substring_digest"]
        );
        cursor = version;
    }
    assert_eq!(
        paths.iter().map(String::as_str).collect::<Vec<_>>(),
        strings(&protocol["acp_module_paths_at_1_0_70"])
    );
    assert_eq!(paths.len(), 96);
}

fn xref_bodies(record: &Value, table: &str, name: &str) -> Vec<(u64, String)> {
    let mut bodies: Vec<(u64, String)> = record["xref"][table][name]["functions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|function| {
            (
                function["size"].as_u64().unwrap(),
                function["body_sha256"].as_str().unwrap().to_owned(),
            )
        })
        .collect();
    bodies.sort();
    bodies
}

#[test]
fn selected_acp_interned_tables_match_and_handler_bodies_differ() {
    let flow = json(CONTROL_FLOW);
    let identity = json(IDENTITY);
    assert_eq!(flow["baseline"], "1.0.46");
    assert_eq!(flow["official_stable"], "1.0.70");
    assert_eq!(flow["no_execution"], true);
    assert!(
        flow["compatibility_predicate"]
            .as_str()
            .unwrap()
            .contains("do not qualify")
    );
    assert_eq!(
        flow["selected_stdio_methods"],
        serde_json::json!([
            "initialize",
            "authenticate",
            "session/new",
            "session/load",
            "session/prompt",
            "session/cancel",
            "session/set_model"
        ])
    );
    assert_eq!(
        flow["selected_client_callbacks"],
        serde_json::json!([
            "session/request_permission",
            "session/update",
            "fs/write_text_file",
            "fs/read_text_file",
            "terminal/create",
            "terminal/output",
            "terminal/wait_for_exit",
            "terminal/kill"
        ])
    );
    assert_eq!(
        flow["selected_permission_kinds"],
        serde_json::json!(["allow_once", "allow_always", "reject_once", "reject_always"])
    );
    assert!(
        flow["swallowtail_process_lifecycle"]["join_drain_release"]
            .as_str()
            .unwrap()
            .contains("close_stdin")
    );
    let hops = flow["hops"].as_object().unwrap();
    assert_eq!(hops.len(), 25);
    let baseline_linux = &hops["1.0.46"]["linux_x64"];
    let baseline_darwin = &hops["1.0.46"]["darwin_arm64"];
    let selected_xrefs = [
        ("stdio", "initialize"),
        ("stdio", "session/new"),
        ("stdio", "session/prompt"),
        ("stdio", "session/cancel"),
        ("client", "session/request_permission"),
        ("client", "session/update"),
    ];
    let mut linux_update_1872 = BTreeSet::new();
    let mut linux_permission_3648 = BTreeSet::new();
    for version in strings(&json(PROTOCOL)["compared"]) {
        for (platform, baseline, digest) in [
            ("linux_x64", baseline_linux, LINUX_LITERAL_DIGEST),
            ("darwin_arm64", baseline_darwin, DARWIN_LITERAL_DIGEST),
        ] {
            let record = &hops[version][platform];
            assert_eq!(
                record["stdio_pack"], baseline["stdio_pack"],
                "{version} {platform} stdio pack"
            );
            assert_eq!(
                record["client_pack"], baseline["client_pack"],
                "{version} {platform} client pack"
            );
            assert_eq!(
                record["permission_pack"], baseline["permission_pack"],
                "{version} {platform} permission pack"
            );
            assert_eq!(
                record["serde_element_counts"], baseline["serde_element_counts"],
                "{version} {platform} serde"
            );
            for field in record["field_packs_present"].as_object().unwrap().values() {
                assert_eq!(field, true, "{version} {platform} field pack");
            }
            for variant in record["stop_reason_variants_present"]
                .as_object()
                .unwrap()
                .values()
            {
                assert_eq!(variant, true, "{version} {platform} stopReason");
            }
            assert_eq!(
                record["selected_literal_presence_digest"], digest,
                "{version} {platform} literal digest"
            );
            for extra in record["extra_method_packs"].as_array().unwrap() {
                assert_ne!(
                    extra["classification"], "new-or-unknown",
                    "{version} {platform} extra pack {:?}",
                    extra["methods"]
                );
            }
            for (table, name) in selected_xrefs {
                let bodies = xref_bodies(record, table, name);
                assert!(
                    !bodies.is_empty(),
                    "{version} {platform} missing xref for {name}"
                );
                if version != "1.0.46" {
                    assert_ne!(
                        bodies,
                        xref_bodies(baseline, table, name),
                        "{version} {platform} {name} xref bodies match 1.0.46"
                    );
                }
            }
            if platform == "linux_x64" {
                for (size, body) in xref_bodies(record, "client", "session/update") {
                    if size == 1872 {
                        linux_update_1872.insert(body);
                    }
                }
                for (size, body) in xref_bodies(record, "client", "session/request_permission") {
                    if size == 3648 {
                        linux_permission_3648.insert(body);
                    }
                }
            }
        }
    }
    assert_eq!(linux_update_1872.len(), 21);
    assert_eq!(linux_permission_3648.len(), 21);
    let darwin70 = &hops["1.0.70"]["darwin_arm64"];
    let prompt = xref_bodies(darwin70, "stdio", "session/prompt");
    assert_eq!(prompt, xref_bodies(darwin70, "stdio", "session/cancel"));
    assert_eq!(prompt, xref_bodies(darwin70, "stdio", "session/set_model"));
    assert_eq!(prompt.len(), 1);
    assert_eq!(prompt[0].0, 78624);
    let hop70 = identity["hops"]
        .as_array()
        .unwrap()
        .iter()
        .find(|hop| hop["version"] == "1.0.70")
        .unwrap();
    assert_eq!(
        hops["1.0.70"]["linux_x64"]["executable_sha256"],
        hop70["linux_x64"]["executable_sha256"]
    );
    assert_eq!(
        hops["1.0.70"]["darwin_arm64"]["executable_sha256"],
        hop70["darwin_arm64"]["executable_sha256"]
    );
}

#[test]
fn production_claim_keeps_1_0_46_while_catalogue_names_1_0_70() {
    let claim = grok_build_acp_claim();
    assert_eq!(GROK_BUILD_ACP_LATEST_QUALIFIED_VERSION, "1.0.46");
    assert_eq!(claim.id().as_str(), "grok-build.acp.executable-window-2");
    assert_eq!(claim.axis().as_str(), "grok-build.executable");
    assert_eq!(claim.scheme(), InterfaceVersionScheme::Semantic);
    assert_eq!(
        claim.newer_version_posture(),
        InterfaceNewerVersionPosture::AllowUnverified
    );
    assert_eq!(claim.baseline().as_str(), "0.2.114");
    assert_eq!(claim.latest_qualified().as_str(), "1.0.46");
    let milestones: Vec<_> = claim.milestones().collect();
    assert_eq!(
        (
            milestones[2].minimum().as_str(),
            milestones[2].maximum().as_str(),
            milestones[2].support_status(),
        ),
        ("1.0.4", "1.0.46", InterfaceSupportStatus::Maintained,)
    );
    let InterfaceCompatibilityAssessment::Qualified(matched) = claim.assess(&version("1.0.46"))
    else {
        panic!("1.0.46 stays in the maintained window");
    };
    assert_eq!(
        matched.behavior_revision().as_str(),
        "grok-build.acp-v1.cached-token-model-4-6-v3"
    );
    for candidate in ["1.0.47", "1.0.50", "1.0.70", "1.0.71", "1.1.0"] {
        let InterfaceCompatibilityAssessment::UnverifiedNewer(newer) =
            claim.assess(&version(candidate))
        else {
            panic!("{candidate} stays unverified");
        };
        assert_eq!(newer.latest_qualified().as_str(), "1.0.46");
    }
    for candidate in ["0.2.118", "1.0.0", "1.0.3"] {
        assert_eq!(
            claim.assess(&version(candidate)),
            InterfaceCompatibilityAssessment::Incompatible
        );
    }

    let catalogue = grok_build_catalogue_claim();
    assert_ne!(catalogue.id(), claim.id());
    assert_eq!(
        catalogue.newer_version_posture(),
        InterfaceNewerVersionPosture::QualifiedOnly
    );
    assert!(matches!(
        catalogue.assess(&version("1.0.70")),
        InterfaceCompatibilityAssessment::Qualified(_)
    ));
    assert_eq!(
        catalogue.assess(&version("1.0.71")),
        InterfaceCompatibilityAssessment::Incompatible
    );
    for candidate in ["1.0.4", "1.0.5"] {
        assert_eq!(
            registered_tool::grok_build_acp_registered_tool_qualification(&version(candidate)),
            RegisteredToolRouteQualification::Qualified(
                registered_tool::GROK_ACP_REGISTERED_TOOL_ROUTE
            )
        );
    }
    for candidate in ["1.0.47", "1.0.70", "1.0.71"] {
        assert_eq!(
            registered_tool::grok_build_acp_registered_tool_qualification(&version(candidate)),
            RegisteredToolRouteQualification::Unqualified
        );
    }
}
