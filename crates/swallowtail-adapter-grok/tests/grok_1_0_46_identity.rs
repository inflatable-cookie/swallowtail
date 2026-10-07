//! Frozen package, runtime, mapped-surface, and claim identity for Grok Build ACP 1.0.46.

use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use swallowtail_adapter_grok::{GROK_BUILD_ACP_LATEST_QUALIFIED_VERSION, grok_build_acp_claim};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceNewerVersionPosture, InterfaceSupportStatus,
    InterfaceVersion, InterfaceVersionScheme,
};

const IDENTITY: &str = include_str!("fixtures/grok-1.0.46/identity.json");
const INVENTORY: &str = include_str!("fixtures/grok-1.0.46/dist-inventory.json");
const PROTOCOL: &str = include_str!("fixtures/grok-1.0.46/protocol.json");
const PREVIOUS_PROTOCOL: &str = include_str!("fixtures/grok-1.0.41/protocol.json");
const HOPS: &[&str] = &["1.0.42", "1.0.43", "1.0.44", "1.0.45", "1.0.46"];
const COMPARED: &[&str] = &["1.0.41", "1.0.42", "1.0.43", "1.0.44", "1.0.45", "1.0.46"];
const BATCH_MEMORY_DREAM: &str =
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/batch_memory_dream.rs";

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

fn exact_strings(value: &Value, expected: &[&str]) {
    assert_eq!(strings(value), expected);
}

fn sha256_lines(values: &[String]) -> String {
    let mut bytes = values.join("\n");
    bytes.push('\n');
    format!("{:x}", Sha256::digest(bytes.as_bytes()))
}

#[test]
fn official_latest_and_every_selected_stable_hop_are_frozen() {
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], "grok-build.executable");
    assert_eq!(identity["npm_package"], "@xai-official/grok");
    assert_eq!(identity["previous_ceiling"], "1.0.41");
    assert_eq!(identity["official_stable"], "1.0.46");
    assert_eq!(identity["npm_dist_tags"]["latest"], "1.0.46");
    assert_eq!(identity["npm_dist_tags"]["alpha"], "1.0.50");
    assert_eq!(identity["observed_at"], "2026-10-08");
    exact_strings(&identity["published_selected_channel_hops"], HOPS);
    assert_eq!(identity["hops"].as_array().unwrap().len(), HOPS.len());

    let mut versions = Vec::new();
    for (index, hop) in identity["hops"].as_array().unwrap().iter().enumerate() {
        let version = hop["version"].as_str().expect("version");
        versions.push(version.to_owned());
        assert_eq!(version, HOPS[index]);
        assert_eq!(
            hop["version_banner_literal"],
            format!(
                "clicurrentVersion{version} ({})channel",
                hop["runtime_build_id"].as_str().unwrap()
            )
        );
        assert!(
            hop["wrapper"]["integrity"]
                .as_str()
                .unwrap()
                .starts_with("sha512-")
        );
        assert_eq!(hop["wrapper"]["file_count"], 5);
        for platform in ["linux_x64", "darwin_arm64"] {
            let artifact = &hop[platform];
            assert!(
                artifact["integrity"]
                    .as_str()
                    .unwrap()
                    .starts_with("sha512-")
            );
            assert_eq!(artifact["file_count"], 4);
            assert_eq!(artifact["executable_sha256"].as_str().unwrap().len(), 64);
            assert_eq!(artifact["brotli_sha256"].as_str().unwrap().len(), 64);
        }
        let other = hop["other_platform_integrities"].as_object().unwrap();
        assert_eq!(other.len(), 6);
        assert!(
            other
                .values()
                .all(|value| value.as_str().unwrap().starts_with("sha512-"))
        );
    }
    assert_eq!(versions, HOPS);
    assert!(identity["hops"][0]["gitHead"].is_null());
    assert_eq!(identity["hops"][0]["runtime_build_id"], "4651fbdf9f13");
    for hop in identity["hops"].as_array().unwrap().iter().skip(1) {
        assert_eq!(
            &hop["runtime_build_id"].as_str().unwrap()[..12],
            &hop["gitHead"].as_str().unwrap()[..12]
        );
    }
    let later: Vec<&str> = identity["published_newer_without_latest_tag"]
        .as_array()
        .unwrap()
        .iter()
        .map(|item| item["version"].as_str().unwrap())
        .collect();
    assert_eq!(later, ["1.0.47", "1.0.48", "1.0.49"]);
    assert_eq!(identity["alpha_dist_tag_exclusion"]["version"], "1.0.50");
    assert_eq!(identity["host_observation"]["grok_on_path"], true);
    assert_eq!(identity["host_observation"]["version_probed"], false);
}

#[test]
fn complete_package_trees_freeze_every_file_and_each_hop_delta() {
    let inventory = json(INVENTORY);
    exact_strings(&inventory["compared"], COMPARED);
    let package_names = ["wrapper", "linux_x64", "darwin_arm64"];
    for version in COMPARED {
        let trees = inventory["complete_trees"][*version].as_object().unwrap();
        assert_eq!(trees.len(), package_names.len());
        for package in package_names {
            let files = trees[package].as_object().unwrap();
            let expected = if package == "wrapper" { 5 } else { 4 };
            assert_eq!(files.len(), expected, "{version} {package}");
            assert!(
                files
                    .values()
                    .all(|digest| digest.as_str().unwrap().len() == 64)
            );
        }
    }
    for version in HOPS {
        for package in package_names {
            let delta = &inventory["per_hop_delta"][*version][package];
            exact_strings(&delta["added"], &[]);
            exact_strings(&delta["removed"], &[]);
            if package == "wrapper" {
                exact_strings(&delta["changed"], &["package.json"]);
                exact_strings(
                    &delta["identical"],
                    &[
                        "README.md",
                        "bin/grok",
                        "bin/grok-bootstrap.js",
                        "bin/postinstall.js",
                    ],
                );
            } else {
                exact_strings(&delta["changed"], &["bin/grok.br", "package.json"]);
                exact_strings(
                    &delta["identical"],
                    &["README.md", "THIRD_PARTY_NOTICES.md"],
                );
            }
        }
    }
    assert_eq!(
        inventory["package_identity"]["1.0.42"]["wrapper"]["gitHead"],
        Value::Null
    );
}

#[test]
fn selected_wire_keys_models_and_acp_path_changes_are_exact() {
    let protocol = json(PROTOCOL);
    let previous = json(PREVIOUS_PROTOCOL);
    assert_eq!(
        protocol["selected_literal_presence"],
        previous["selected_literal_presence"]
    );
    assert_eq!(protocol["mapped_surface"], previous["mapped_surface"]);
    assert_eq!(
        protocol["selected_literal_presence_digest"],
        "4cceb3e6fc7893dd2e26e8487b5e7266c4040d2fd189ac38ea4569365a10b311"
    );
    assert_eq!(
        protocol["darwin_arm64_selected_literal_presence_digest"],
        "4a548e4dc7687941e640dc410146a0ed99dc8b95724871317ffe37a14fe3dfb3"
    );
    exact_strings(&protocol["compared"], COMPARED);
    let per_hop = protocol["per_hop"].as_object().unwrap();
    assert_eq!(
        per_hop.keys().map(String::as_str).collect::<Vec<_>>(),
        [
            "1.0.40", "1.0.41", "1.0.42", "1.0.43", "1.0.44", "1.0.45", "1.0.46",
        ]
    );
    let baseline = strings(&per_hop["1.0.41"]["acp_module_paths"]);
    let first = strings(&per_hop["1.0.42"]["acp_module_paths"]);
    assert_eq!(baseline.len(), 85);
    assert_eq!(first.len(), 86);
    assert!(first.windows(2).all(|pair| pair[0] < pair[1]));
    let baseline_set: BTreeSet<_> = baseline.iter().copied().collect();
    let first_set: BTreeSet<_> = first.iter().copied().collect();
    assert_eq!(
        first_set
            .difference(&baseline_set)
            .copied()
            .collect::<Vec<_>>(),
        [BATCH_MEMORY_DREAM]
    );
    assert!(baseline_set.difference(&first_set).next().is_none());
    let first_paths: Vec<String> = first.iter().map(|path| (*path).to_owned()).collect();
    let first_digest = sha256_lines(&first_paths);
    assert_eq!(
        first_digest,
        "d53c7e9109615dbaf853d8051a3df7a26330e080a77cc2f786338142ad6577e5"
    );
    for version in ["1.0.42", "1.0.43", "1.0.44", "1.0.45", "1.0.46"] {
        assert_eq!(
            per_hop[version]["acp_module_paths"],
            per_hop["1.0.42"]["acp_module_paths"]
        );
        assert_eq!(
            per_hop[version]["selected_literal_presence_digest"],
            protocol["selected_literal_presence_digest"]
        );
        assert_eq!(
            per_hop[version]["darwin_arm64_selected_literal_presence_digest"],
            protocol["darwin_arm64_selected_literal_presence_digest"]
        );
        let linux = &per_hop[version]["linux_x64_selected_literal_presence"];
        let darwin = &per_hop[version]["darwin_arm64_selected_literal_presence"];
        assert_eq!(linux, &protocol["selected_literal_presence"]);
        assert_eq!(darwin, &protocol["darwin_arm64_selected_literal_presence"]);
        assert_eq!(linux.as_object().unwrap().len(), 92);
        assert_eq!(darwin.as_object().unwrap().len(), 92);
        assert_eq!(darwin["agentVersion"], true);
        let linux_present: Vec<String> = linux
            .as_object()
            .unwrap()
            .iter()
            .filter(|(_, value)| value.as_bool().expect("presence boolean"))
            .map(|(key, _)| key.clone())
            .collect();
        let darwin_present: Vec<String> = darwin
            .as_object()
            .unwrap()
            .iter()
            .filter(|(_, value)| value.as_bool().expect("presence boolean"))
            .map(|(key, _)| key.clone())
            .collect();
        assert_eq!(
            sha256_lines(&linux_present),
            protocol["selected_literal_presence_digest"]
        );
        assert_eq!(
            sha256_lines(&darwin_present),
            protocol["darwin_arm64_selected_literal_presence_digest"]
        );
    }
    exact_strings(
        &protocol["hop_delta_from_1_0_41"]["added"],
        &[BATCH_MEMORY_DREAM],
    );
    exact_strings(&protocol["hop_delta_from_1_0_41"]["removed"], &[]);
    let models = &protocol["model_document"];
    assert_eq!(
        models["digests"]["9d6924ec760a94f91902f60adb8bcf2cd9d3ab86891a1c83e8c01a092e56490a"],
        serde_json::json!(COMPARED)
    );
    exact_strings(&models["model_ids"], &["grok-4.6", "grok-4.5"]);
    assert_eq!(models["default"], "grok-4.6");
    assert_eq!(
        models["reasoning_effort_values"],
        previous["model_document"]["reasoning_effort_values"]
    );
}

#[test]
fn production_claim_extends_the_existing_milestone_without_widening_other_routes() {
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
    assert_eq!(milestones.len(), 3);
    assert_eq!(
        (
            milestones[0].minimum().as_str(),
            milestones[0].maximum().as_str()
        ),
        ("0.2.114", "0.2.116")
    );
    assert_eq!(
        milestones[0].support_status(),
        InterfaceSupportStatus::Deprecated
    );
    assert_eq!(
        (
            milestones[1].minimum().as_str(),
            milestones[1].maximum().as_str()
        ),
        ("0.2.117", "0.2.117")
    );
    assert_eq!(
        milestones[1].support_status(),
        InterfaceSupportStatus::Deprecated
    );
    assert_eq!(
        (
            milestones[2].minimum().as_str(),
            milestones[2].maximum().as_str()
        ),
        ("1.0.4", "1.0.46")
    );
    assert_eq!(
        milestones[2].support_status(),
        InterfaceSupportStatus::Maintained
    );
    assert!(claim.exclusions().next().is_none());
    for candidate in ["0.2.118", "0.2.121", "1.0.0", "1.0.3"] {
        assert_eq!(
            claim.assess(&InterfaceVersion::new(candidate).unwrap()),
            InterfaceCompatibilityAssessment::Incompatible
        );
    }
    for candidate in ["1.0.47", "1.0.48", "1.0.49"] {
        assert!(
            matches!(
                claim.assess(&InterfaceVersion::new(candidate).unwrap()),
                InterfaceCompatibilityAssessment::UnverifiedNewer(_)
            ),
            "{candidate} stays outside the qualified window"
        );
    }
}
