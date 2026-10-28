use std::collections::BTreeSet;

use serde_json::{Map, Value};
use swallowtail_adapter_opencode::{
    OPENCODE_LATEST_QUALIFIED_VERSION, opencode_acp_claim, opencode_http_claim,
    opencode_server_binding,
};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceSupportStatus};

const IDENTITY: &str = include_str!("fixtures/opencode-1.19.5/identity.json");
const INVENTORY: &str = include_str!("fixtures/opencode-1.19.5/dist-inventory.json");
const PROTOCOL: &str = include_str!("fixtures/opencode-1.19.5/protocol.json");
const CLAIM: &str = include_str!("fixtures/opencode-1.19.5/claim.json");

#[test]
fn official_channels_agree_on_one_patch_and_leave_opencode_2_aside() {
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], "opencode.server");
    assert_eq!(identity["selected_route_family"], "opencode.http");
    assert_eq!(identity["companion_route_family"], "opencode.acp");
    assert_eq!(identity["version"], "1.19.5");
    assert_eq!(identity["research"], 464);
    assert_eq!(identity["npm_package"], "opencode-ai");
    assert_eq!(identity["npm_latest"], "1.19.5");
    assert_eq!(identity["github_latest_tag"], "v1.19.5");
    assert_eq!(identity["official_channels_agree"], true);
    assert_eq!(identity["previous_qualified_ceiling"], "1.19.4");
    assert_eq!(identity["unpublished_gap_preserved"], "1.18.36");
    assert_eq!(identity["first_unpublished_later_stable"], "1.19.6");
    assert_eq!(identity["prereleases_ignored"], true);
    assert_eq!(identity["non_latest_dist_tags_ignored"], true);
    assert_exact_strings(
        &identity["official_stable_hops_from_previous_qualified_ceiling"],
        &["1.19.5"],
    );
    assert_exact_strings(&identity["unpublished_or_withdrawn_hops"], &[]);
    let registry = &identity["acp_registry_not_the_axis"];
    assert_eq!(registry["opencode_version"], "2.2.0");
    assert_eq!(registry["opencode_v2_version"], "2.2.0");
    assert_eq!(registry["github_release"], "v2.2.0");
    assert_eq!(registry["npm_opencode_ai_has_2_x"], false);
    assert_eq!(registry["distinct_from_opencode_ai_latest"], true);
    assert_eq!(registry["qualified_from_this_entry"], false);
    assert_eq!(
        identity["identity_decision"]["shape"],
        "compatible-extension"
    );
    assert_eq!(
        identity["identity_decision"]["raise_http_latest_qualified_to"],
        "1.19.5"
    );
    assert_eq!(
        identity["identity_decision"]["raise_acp_latest_qualified_to"],
        "1.19.5"
    );
    assert_eq!(
        identity["identity_decision"]["keep_claims_unflattened"],
        true
    );
    assert_eq!(
        identity["identity_decision"]["claim_changed_in_identity_card"],
        false
    );
    assert_eq!(identity["identity_decision"]["new_public_operation"], false);
    assert_eq!(identity["host"]["present"], false);
    assert_eq!(identity["host"]["on_path"], false);
    assert_eq!(identity["host"]["installed_or_updated"], false);

    let releases = identity["release_artifacts"].as_array().expect("releases");
    assert_eq!(releases.len(), 2);
    let previous = &releases[0];
    let current = &releases[1];
    assert_eq!(previous["version"], "1.19.4");
    assert_eq!(
        previous["npm"]["tarball_sha256"],
        "2ceca45c7e0bbd50ddd3f4d6d651ed43645647d74279d656e7c7747a4b6a9137"
    );
    assert_eq!(previous["npm"]["gitHead"], Value::Null);
    assert_eq!(
        previous["github"]["tag_commit"],
        "148842ef7cb1bc34192b3fff57d0f9721ab9c801"
    );
    assert_eq!(
        previous["github"]["source_archive_sha256"],
        "2e553ed005b2df4a7efb9edd26db8fc3caafb1b4dfc3c5c3869192334584d8bb"
    );
    assert_eq!(
        previous["github"]["linux_x64_asset_sha256"],
        "849bd6b3d959a1d2ef200ed1634a51c79a45ca236aa26665d75debd694805cce"
    );
    assert_eq!(current["version"], "1.19.5");
    assert_eq!(current["npm"]["latest_at_observation"], true);
    assert_eq!(current["npm"]["published_at"], "2026-10-28T06:17:11.100Z");
    assert_eq!(
        current["npm"]["tarball_sha256"],
        "1f97e49381f8a42a629482eff8be98aa96d62e2e94410a8ec98e738f9c787680"
    );
    assert_eq!(
        current["npm"]["shasum"],
        "4fb49717dfc8ca5fa97ab9823dadf4be3e244e82"
    );
    assert_eq!(current["npm"]["gitHead"], Value::Null);
    let files = current["npm"]["files"].as_object().unwrap();
    assert_exact_object_keys(
        files,
        &[
            "LICENSE",
            "bin/opencode.exe",
            "package.json",
            "postinstall.mjs",
        ],
    );
    assert_eq!(files["LICENSE"], previous["npm"]["files"]["LICENSE"]);
    assert_eq!(
        files["bin/opencode.exe"],
        previous["npm"]["files"]["bin/opencode.exe"]
    );
    assert_eq!(
        files["postinstall.mjs"],
        previous["npm"]["files"]["postinstall.mjs"]
    );
    assert_ne!(
        files["package.json"],
        previous["npm"]["files"]["package.json"]
    );
    let linux = current["npm_linux_x64"]["files"].as_object().unwrap();
    assert_exact_object_keys(linux, &["bin/opencode", "package.json"]);
    assert_ne!(
        linux["bin/opencode"],
        previous["npm_linux_x64"]["files"]["bin/opencode"]
    );
    assert_eq!(current["github"]["published_at"], "2026-10-28T06:17:49Z");
    assert_eq!(
        current["github"]["tag_commit"],
        "810c0eba1a7348036f8bdf61e6632d0c36c6876c"
    );
    assert_eq!(current["github"]["release_target_commitish"], "dev");
    assert_eq!(current["github"]["release_target_commitish_kind"], "branch");
    assert_eq!(
        current["github"]["source_archive_sha256"],
        "7d2ac43b5083a3a4687fe23b79ab153b8d93a6fb186fa07209ed28d4731e5c7a"
    );
    assert_eq!(
        current["github"]["linux_x64_asset_sha256"],
        "588310a336fa3b9df36b3f0225b26f679d453efaad0cf3d66c3651ef3ec1647a"
    );
    assert_eq!(current["github"]["linux_x64_asset_size"], 79651896);
    assert_eq!(
        current["github"]["linux_x64_asset_digest_source"],
        "github-release-api"
    );
}

#[test]
fn repository_hop_classifies_every_changed_path_without_moving_selected_files() {
    let inventory = json(INVENTORY);
    let protocol = json(PROTOCOL);
    assert_exact_strings(&inventory["compared"], &["1.19.4", "1.19.5"]);
    assert_eq!(
        protocol["selected_openapi_path_objects_byte_identical"],
        true
    );
    assert_eq!(
        protocol["selected_route_declaration_and_handler_files_byte_identical"],
        true
    );
    assert_eq!(protocol["openapi_info_version"], "1.0.0");
    assert_eq!(protocol["openapi_path_count"], 162);
    assert_eq!(protocol["acp_sdk_pin"], "0.21.0");
    assert_eq!(protocol["bundled_bun"], "1.4.4");
    assert_eq!(protocol["provider_prompt_sent"], false);
    assert_eq!(protocol["live_server_started"], false);
    assert_eq!(protocol["downloaded_artifact_executed"], false);
    assert_eq!(
        protocol["classified_mapped_deltas"]
            .as_array()
            .unwrap()
            .len(),
        0
    );

    let trees = inventory["complete_repository_tree_inventory"]["per_version"]
        .as_object()
        .unwrap();
    assert_eq!(trees["1.19.4"]["files"], 6576);
    assert_eq!(
        trees["1.19.4"]["canonical_sha256"],
        "ae0d6884639fda9deb79082e5b27483c9b6a6f45fb8868dadd89714722cbf287"
    );
    assert_eq!(trees["1.19.5"]["entries"], 6627);
    assert_eq!(trees["1.19.5"]["files"], 6567);
    assert_eq!(trees["1.19.5"]["symlinks"], 60);
    assert_eq!(trees["1.19.5"]["opencode_src_files"], 400);
    assert_eq!(
        trees["1.19.5"]["canonical_sha256"],
        "98d156015c7d31198a689d93fbe18bdc9a11c833228473e8334ee36c9973a56b"
    );
    assert_eq!(
        inventory["openapi_sha256"]["1.19.4"],
        inventory["openapi_sha256"]["1.19.5"]
    );
    assert_eq!(
        inventory["openapi_sha256"]["1.19.5"],
        "d224c1a7506ae981c8d4fc1a8d1f58ac185e27324f94d4daf1b8f84fc2552d26"
    );

    let stable = inventory["mapped_stable_file_sha256"].as_object().unwrap();
    assert_exact_object_keys(
        stable,
        protocol["mapped_stable_files"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_str().unwrap())
            .collect::<Vec<_>>()
            .as_slice(),
    );
    for (path, versions) in stable {
        assert_eq!(versions["1.19.4"], versions["1.19.5"], "{path}");
    }
    for (path, versions) in inventory["gitlab_provider_source_sha256"]
        .as_object()
        .unwrap()
    {
        assert_eq!(versions["1.19.4"], versions["1.19.5"], "{path}");
    }

    let hops = inventory["hops"].as_array().unwrap();
    assert_eq!(hops.len(), 1);
    let hop = &hops[0];
    assert_eq!(hop["hop"], "1.19.4_to_1.19.5");
    assert_eq!(hop["repository_delta_counts"]["added"], 4);
    assert_eq!(hop["repository_delta_counts"]["removed"], 13);
    assert_eq!(hop["repository_delta_counts"]["changed"], 83);
    assert_eq!(hop["repository_delta_counts"]["identical"], 6540);
    assert_eq!(
        hop["repository_path_sha256"]["added"],
        "afe3f85b786e88bbf1d92505f987bdf05d09b9f7add5ce67a7d96ad6c2c6968a"
    );
    assert_eq!(
        hop["repository_path_sha256"]["removed"],
        "11a9113311a6949ed859e968a95b19fa93e9c0209147a37e4aa364e5f98d9adc"
    );
    assert_eq!(
        hop["repository_path_sha256"]["changed"],
        "4491c152d5e7302a381c25b020e351a15ae651133c37b5647469d76d0bc9f97f"
    );

    let mut delta_paths = BTreeSet::new();
    for key in [
        "repository_added",
        "repository_removed",
        "repository_changed",
    ] {
        for path in hop[key].as_array().unwrap() {
            assert!(delta_paths.insert(path.as_str().unwrap().to_owned()));
        }
    }
    assert_eq!(delta_paths.len(), 100);
    let partition = hop["path_partition"].as_object().unwrap();
    let mut assigned = BTreeSet::new();
    for (bucket, paths) in partition {
        for path in paths.as_array().unwrap() {
            let path = path.as_str().unwrap();
            assert!(assigned.insert(path.to_owned()), "{path} is in two buckets");
            if requires_classification(path) {
                assert_eq!(bucket, "classified", "{path} must be classified");
            }
        }
    }
    assert_eq!(assigned, delta_paths);
    let classified = string_set(&partition["classified"]);
    let implementation = hop["implementation_file_deltas"].as_object().unwrap();
    assert_exact_object_keys(
        implementation,
        classified
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
            .as_slice(),
    );
    assert!(classified.contains("packages/opencode/src/util/rpc.ts"));
    assert!(classified.contains(
        "packages/opencode/src/server/routes/instance/httpapi/middleware/authorization.ts"
    ));
    assert!(!classified.contains("packages/sdk/openapi.json"));
    assert!(!delta_paths.contains("packages/opencode/src/acp/agent.ts"));
    assert!(!delta_paths.contains("packages/opencode/src/cli/cmd/acp.ts"));

    let mut classified_protocol = BTreeSet::new();
    for item in protocol["classified_unmapped_deltas"].as_array().unwrap() {
        let text = item["classification"].as_str().unwrap();
        assert!(text.len() > 40, "{}", item["file"]);
        assert_eq!(item["hop"], "1.19.4_to_1.19.5");
        assert!(classified_protocol.insert(item["file"].as_str().unwrap().to_owned()));
    }
    assert_eq!(classified_protocol, classified);
    for (path, delta) in implementation {
        if delta["change"] == "removed" {
            assert!(delta.get("after_sha256").is_none(), "{path}");
            assert!(delta["before_sha256"].as_str().unwrap().len() == 64);
        } else {
            assert_eq!(delta["change"], "changed");
            assert_ne!(delta["before_sha256"], delta["after_sha256"]);
        }
    }
}

#[test]
fn admitted_claim_extends_surface_19_through_1_19_5() {
    let fixture = json(CLAIM);
    let claim = opencode_http_claim();
    let acp = opencode_acp_claim();
    assert_eq!(OPENCODE_LATEST_QUALIFIED_VERSION, "1.19.5");
    assert_eq!(claim.latest_qualified().as_str(), "1.19.5");
    assert_eq!(claim.id().as_str(), fixture["claim_id"]);
    assert_eq!(claim.id().as_str(), "opencode.http.server-window-1");
    assert_eq!(claim.axis().as_str(), "opencode.server");
    assert_ne!(claim.axis().as_str(), acp.axis().as_str());
    assert_ne!(claim.id().as_str(), acp.id().as_str());
    assert_eq!(fixture["preserved_segment"], "1.18.11..=1.18.35");
    assert_eq!(fixture["extended_segment"], "1.19.0..=1.19.5");
    assert_eq!(fixture["behavior_revision"], "opencode.http-sse.surface-19");
    assert_eq!(fixture["unpublished_gap"], "1.18.36");
    assert_eq!(fixture["unverified_newer"], "1.19.6");
    assert_exact_strings(&fixture["newly_qualified"], &["1.19.5"]);
    assert_exact_strings(
        &fixture["historical_gaps_preserved"],
        &[
            "1.14.52", "1.15.8", "1.15.14", "1.16.1", "1.16.3", "1.17.21",
        ],
    );
    for version in ["1.19.4", "1.19.5"] {
        let binding = opencode_server_binding(version).unwrap();
        let InterfaceCompatibilityAssessment::Qualified(matched) = claim.assess(binding.version())
        else {
            panic!("{version} is not qualified");
        };
        assert_eq!(
            matched.behavior_revision().as_str(),
            "opencode.http-sse.surface-19"
        );
        assert_eq!(matched.support_status(), InterfaceSupportStatus::Maintained);
    }
    let gap = opencode_server_binding("1.18.36").unwrap();
    assert!(matches!(
        claim.assess(gap.version()),
        InterfaceCompatibilityAssessment::Incompatible
    ));
    let later = opencode_server_binding("1.19.6").unwrap();
    assert!(matches!(
        claim.assess(later.version()),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
}

fn requires_classification(path: &str) -> bool {
    path == "bun.lock"
        || path == "packages/opencode/package.json"
        || path == "packages/core/package.json"
        || path.starts_with("packages/opencode/src/")
        || path.starts_with("packages/core/src/")
        || path.starts_with("packages/server/src/")
        || path.starts_with("packages/protocol/src/")
}

fn json(input: &str) -> Value {
    serde_json::from_str(input).expect("fixture is valid JSON")
}

fn string_set(actual: &Value) -> BTreeSet<String> {
    actual
        .as_array()
        .expect("string array")
        .iter()
        .map(|value| value.as_str().expect("string").to_owned())
        .collect()
}

fn assert_exact_strings(actual: &Value, expected: &[&str]) {
    let actual = string_set(actual);
    assert_eq!(
        actual,
        expected.iter().map(|value| (*value).to_owned()).collect()
    );
}

fn assert_exact_object_keys(actual: &Map<String, Value>, expected: &[&str]) {
    let actual = actual.keys().map(String::as_str).collect::<BTreeSet<_>>();
    let expected = expected.iter().copied().collect::<BTreeSet<_>>();
    assert_eq!(actual, expected);
}
