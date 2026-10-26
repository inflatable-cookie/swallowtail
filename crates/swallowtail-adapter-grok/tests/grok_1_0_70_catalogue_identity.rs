//! Frozen selection-window evidence for the Grok Build model catalogue.
//!
//! Research 443 refreshed the official `latest` channel to `1.0.70`, decoded
//! the shipped catalogue formatter, and qualified one same-contract mapping
//! extension. This test pins the corpus shape, the reproduction of the earlier
//! stop corpus, and the resulting claim window.

use serde_json::Value;
use std::collections::BTreeMap;
use swallowtail_adapter_grok::{
    GROK_BUILD_ACP_LATEST_QUALIFIED_VERSION, GROK_BUILD_CATALOGUE_VERSION, grok_build_acp_claim,
    grok_build_catalogue_claim,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const IDENTITY: &str = include_str!("fixtures/grok-1.0.70-catalogue/identity.json");
const DIST_INVENTORY: &str = include_str!("fixtures/grok-1.0.70-catalogue/dist-inventory.json");
const RUNTIME_SURFACE: &str = include_str!("fixtures/grok-1.0.70-catalogue/runtime-surface.json");
const FROZEN_IDENTITY: &str = include_str!("fixtures/grok-1.0.46-catalogue/identity.json");
const FROZEN_INVENTORY: &str = include_str!("fixtures/grok-1.0.46-catalogue/dist-inventory.json");
const FROZEN_RUNTIME: &str = include_str!("fixtures/grok-1.0.46-catalogue/runtime-surface.json");
const FROZEN_ACP_IDENTITY: &str = include_str!("fixtures/grok-1.0.46/identity.json");

fn json(source: &str) -> Value {
    serde_json::from_str(source).expect("frozen fixture is valid JSON")
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}

/// The frozen 1.0.46 corpus spells its newline-bearing literal key with a
/// literal `\n` escape; both corpora count the same shipped bytes.
fn normalised_counts(value: &Value) -> BTreeMap<String, u64> {
    value
        .as_object()
        .expect("literal counts are an object")
        .iter()
        .map(|(key, count)| {
            (
                key.replace("\\n", "\n"),
                count.as_u64().expect("literal count is an integer"),
            )
        })
        .collect()
}

fn window() -> Vec<String> {
    (30..=70).map(|minor| format!("1.0.{minor}")).collect()
}

#[test]
fn refreshed_channel_freezes_every_selected_stable_hop() {
    let identity = json(IDENTITY);
    assert_eq!(identity["schema"], "swallowtail.grok.catalogue.identity.v2");
    assert_eq!(identity["observed_at"], "2026-10-26");
    assert_eq!(identity["axis"], "grok-build.executable");
    assert_eq!(identity["route"], "grok-build.catalogue");
    assert_eq!(
        identity["official_channel"]["package"],
        "@xai-official/grok"
    );
    assert_eq!(identity["official_channel"]["selected_tag"], "latest");
    assert_eq!(identity["official_channel"]["official_stable"], "1.0.70");
    assert_eq!(
        identity["official_channel"]["dist_tags"]["latest"],
        "1.0.70"
    );
    assert_eq!(identity["official_channel"]["dist_tags"]["alpha"], "1.0.70");
    assert_eq!(identity["no_provider_work"], true);
    assert_eq!(
        identity["preserved_exact_point"]["claim_id"],
        "grok-build.catalogue.executable-1-0-30"
    );
    assert_eq!(
        identity["preserved_exact_point"]["behavior_revision"],
        "grok-build.catalogue.models-text-v1"
    );

    let expected = window();
    assert_eq!(
        identity["compared_artifact_versions"],
        serde_json::json!(expected)
    );
    assert_eq!(
        identity["published_selected_channel_hops"],
        serde_json::json!(expected[1..].to_vec())
    );

    let hops = identity["hops"].as_array().expect("hops are an array");
    assert_eq!(hops.len(), expected.len());
    for (hop, expected_version) in hops.iter().zip(&expected) {
        assert_eq!(hop["version"], expected_version.as_str());
        assert_eq!(hop["wrapper"]["package"], "@xai-official/grok");
        assert_eq!(hop["wrapper"]["file_count"], 5);
        assert_eq!(
            hop["darwin_arm64"]["package"],
            "@xai-official/grok-darwin-arm64"
        );
        assert_eq!(hop["darwin_arm64"]["file_count"], 4);
        for artifact in [&hop["wrapper"], &hop["darwin_arm64"]] {
            assert!(
                artifact["integrity"]
                    .as_str()
                    .unwrap()
                    .starts_with("sha512-")
            );
            assert_eq!(artifact["tarball_sha256"].as_str().unwrap().len(), 64);
            assert_eq!(artifact["shasum"].as_str().unwrap().len(), 40);
            assert!(artifact["tarball_bytes"].as_u64().unwrap() > 0);
            assert!(artifact["unpacked_size"].as_u64().unwrap() > 0);
        }
        assert_eq!(
            hop["wrapper"]["optional_platform_versions"]["@xai-official/grok-darwin-arm64"],
            expected_version.as_str(),
            "the wrapper pins its own platform package version"
        );
        assert_eq!(
            hop["darwin_arm64"]["runtime_sha256"]
                .as_str()
                .unwrap()
                .len(),
            64
        );
        assert!(hop["darwin_arm64"]["runtime_bytes"].as_u64().unwrap() > 0);
        // Each executable embeds exactly one `<version> (<revision>)` banner.
        let build_id = hop["darwin_arm64"]["runtime_build_id"]
            .as_str()
            .expect("frozen build id");
        assert!(
            build_id.len() == 12 && build_id.bytes().all(|byte| byte.is_ascii_hexdigit()),
            "{expected_version} build id shape"
        );
        assert_eq!(
            hop["darwin_arm64"]["version_banner_fragment"],
            format!("{expected_version} ({build_id})"),
            "{expected_version} embedded version banner"
        );
    }

    // The overlapping ACP identity corpus independently freezes the same
    // self-reported revisions, so the carve reproduces prior evidence.
    let acp = json(FROZEN_ACP_IDENTITY);
    let mut compared = 0;
    for acp_hop in acp["hops"].as_array().unwrap() {
        let acp_version = acp_hop["version"].as_str().unwrap();
        let recorded = acp_hop["runtime_build_id"].as_str().unwrap();
        let ours = hops
            .iter()
            .find(|candidate| candidate["version"] == acp_version)
            .expect("the selection window contains the ACP hop");
        assert_eq!(
            ours["darwin_arm64"]["runtime_build_id"], recorded,
            "{acp_version} self-reported build id"
        );
        compared += 1;
    }
    assert!(compared >= 5);
}

#[test]
fn formatter_classes_bracket_the_two_segments() {
    let runtime = json(RUNTIME_SURFACE);
    assert_eq!(
        runtime["schema"],
        "swallowtail.grok-build-catalogue.runtime-surface.v2"
    );
    assert_eq!(runtime["target"], "darwin-arm64");
    let observations = runtime["observations"].as_object().unwrap();
    assert_eq!(observations.len(), window().len());
    for expected_version in window() {
        let observation = &observations[expected_version.as_str()];
        let minor: u32 = expected_version
            .trim_start_matches("1.0.")
            .parse()
            .expect("minor version");
        let (profile, pieces, arguments, note_form) = if minor <= 45 {
            (
                "listed-default-only",
                serde_json::json!(["Default model: ", "\n"]),
                1,
                "absent",
            )
        } else if minor <= 61 {
            (
                "default-not-in-list-piece",
                serde_json::json!(["Default model: ", " (not in the list below)\n"]),
                1,
                "piece",
            )
        } else {
            (
                "default-not-in-list-argument",
                serde_json::json!(["Default model: ", "", "\n\nAvailable models:", "\n"]),
                3,
                "argument",
            )
        };
        let formatter = &observation["catalogue_formatter"];
        assert_eq!(
            formatter["profile"], profile,
            "{expected_version} formatter class"
        );
        assert_eq!(
            formatter["header_piece_forms"], pieces,
            "{expected_version} header pieces"
        );
        assert_eq!(
            formatter["header_argument_count"], arguments,
            "{expected_version} header argument count"
        );
        assert_eq!(
            formatter["default_note_form"], note_form,
            "{expected_version} note presentation"
        );
        assert_eq!(
            observation["literal_counts"]["not in the list below)"],
            if minor >= 46 { 1 } else { 0 }
        );
        assert_eq!(observation["literal_counts"]["Default model: "], 3);
        assert_eq!(observation["literal_counts"]["Available models:"], 1);
        assert_eq!(observation["literal_counts"]["  * "], 3);
        assert_eq!(
            observation["literal_counts"]["xai-grok-pager/src/models.rs"],
            1
        );
        assert_eq!(
            observation["literal_counts"]["List available models and exit"],
            1
        );
        assert_eq!(observation["literal_counts"]["--no-auto-update"], 4);
        for preamble in [
            "You are logged in with ",
            "You are using XAI_API_KEY.",
            " is using its own API key.",
            "You are authenticated via deployment key.",
            "You are not authenticated.",
        ] {
            assert_eq!(observation["literal_counts"][preamble], 1, "{preamble}");
        }
        let documents = observation["embedded_default_models"]
            .as_array()
            .expect("embedded documents");
        assert_eq!(documents.len(), 2);
        for document in documents {
            assert_eq!(document["default"], "grok-4.6");
            assert_eq!(
                document["model_ids"],
                serde_json::json!(["grok-4.6", "grok-4.5"])
            );
            assert_eq!(
                document["sha256"],
                "9d6924ec760a94f91902f60adb8bcf2cd9d3ab86891a1c83e8c01a092e56490a"
            );
        }
    }
}

#[test]
fn new_corpus_reproduces_the_frozen_stop_corpus() {
    let identity = json(IDENTITY);
    let frozen_identity = json(FROZEN_IDENTITY);
    let runtime = json(RUNTIME_SURFACE);
    let frozen_runtime = json(FROZEN_RUNTIME);
    let hops: BTreeMap<&str, &Value> = identity["hops"]
        .as_array()
        .unwrap()
        .iter()
        .map(|hop| (hop["version"].as_str().unwrap(), hop))
        .collect();
    let frozen_hops: BTreeMap<&str, &Value> = frozen_identity["hops"]
        .as_array()
        .unwrap()
        .iter()
        .map(|hop| (hop["version"].as_str().unwrap(), hop))
        .collect();
    for minor in 30..=46 {
        let key = format!("1.0.{minor}");
        let hop = hops[key.as_str()];
        let frozen = frozen_hops[key.as_str()];
        assert_eq!(
            hop["darwin_arm64"]["runtime_sha256"], frozen["darwin_arm64"]["runtime_sha256"],
            "{key} runtime digest"
        );
        assert_eq!(
            hop["darwin_arm64"]["runtime_bytes"], frozen["darwin_arm64"]["runtime_bytes"],
            "{key} runtime size"
        );
        assert_eq!(
            hop["wrapper"]["tarball_sha256"], frozen["wrapper"]["tarball_sha256"],
            "{key} wrapper tarball"
        );
        assert_eq!(
            hop["darwin_arm64"]["integrity"], frozen["darwin_arm64"]["integrity"],
            "{key} platform integrity"
        );
        assert_eq!(
            hop["catalogue_default_not_in_list_marker_count"],
            frozen["catalogue_default_not_in_list_marker_count"],
            "{key} marker count"
        );
        let counts = normalised_counts(&runtime["observations"][key.as_str()]["literal_counts"]);
        for (literal, count) in
            normalised_counts(&frozen_runtime["observations"][key.as_str()]["literal_counts"])
        {
            assert_eq!(
                counts.get(&literal),
                Some(&count),
                "{key} catalogue literal {literal:?}"
            );
        }
        let documents = runtime["observations"][key.as_str()]["embedded_default_models"]
            .as_array()
            .unwrap();
        let frozen_documents =
            frozen_runtime["observations"][key.as_str()]["embedded_default_models"]
                .as_array()
                .unwrap();
        assert_eq!(documents.len(), frozen_documents.len());
        for (document, frozen_document) in documents.iter().zip(frozen_documents) {
            for field in [
                "bytes",
                "default",
                "model_ids",
                "sha256",
                "sha256_with_trailing_newline",
            ] {
                assert_eq!(
                    document[field], frozen_document[field],
                    "{key} embedded model document {field}"
                );
            }
        }
    }

    let inventory = json(DIST_INVENTORY);
    let frozen_inventory = json(FROZEN_INVENTORY);
    for minor in 30..=46 {
        let key = format!("1.0.{minor}");
        for side in ["wrapper", "darwin_arm64"] {
            assert_eq!(
                inventory["files"][side][key.as_str()],
                frozen_inventory["files"][side][key.as_str()],
                "{key} {side} file inventory"
            );
        }
    }
}

#[test]
fn dist_inventory_covers_every_hop_without_path_drift() {
    let inventory = json(DIST_INVENTORY);
    assert_eq!(inventory["platform"], "darwin-arm64");
    assert_eq!(inventory["compared"], serde_json::json!(window()));
    let wrapper_paths: Vec<&str> = inventory["files"]["wrapper"]["1.0.30"]
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect();
    let platform_paths: Vec<&str> = inventory["files"]["darwin_arm64"]["1.0.30"]
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect();
    assert_eq!(wrapper_paths.len(), 5);
    assert_eq!(platform_paths.len(), 4);
    for minor in 30..=70 {
        let key = format!("1.0.{minor}");
        for side in ["wrapper", "darwin_arm64"] {
            let files = inventory["files"][side][key.as_str()].as_object().unwrap();
            let expected_paths = if side == "wrapper" {
                &wrapper_paths
            } else {
                &platform_paths
            };
            assert_eq!(
                files.keys().map(String::as_str).collect::<Vec<_>>(),
                *expected_paths,
                "{key} {side} paths"
            );
            for file in files.values() {
                assert_eq!(file["sha256"].as_str().unwrap().len(), 64);
                assert!(file["bytes"].as_u64().unwrap() > 0);
            }
        }
        assert_eq!(inventory["package_file_counts"][key.as_str()]["wrapper"], 5);
        assert_eq!(
            inventory["package_file_counts"][key.as_str()]["darwin_arm64"],
            4
        );
        for side in ["wrapper", "darwin_arm64"] {
            assert_eq!(
                inventory["special_entries"][side][key.as_str()],
                serde_json::json!([])
            );
        }
    }
    let hops = inventory["hops"].as_array().unwrap();
    assert_eq!(hops.len(), window().len() - 1);
    for hop in hops {
        for side in ["wrapper", "darwin_arm64"] {
            assert_eq!(hop[side]["added"], serde_json::json!([]));
            assert_eq!(hop[side]["removed"], serde_json::json!([]));
        }
        let wrapper_changed = hop["wrapper"]["changed"].as_array().unwrap();
        assert!(
            wrapper_changed.contains(&serde_json::json!("package.json")),
            "every hop republishes the wrapper manifest"
        );
        let platform_changed = hop["darwin_arm64"]["changed"].as_array().unwrap();
        for path in ["bin/grok.br", "package.json"] {
            assert!(
                platform_changed.contains(&serde_json::json!(path)),
                "every hop republishes {path}"
            );
        }
        // Only the wrapper README ever moves besides the two manifests; the
        // notices and launcher scripts stay byte-identical at every hop.
        assert_eq!(
            hop["darwin_arm64"]["identical"],
            serde_json::json!(["README.md", "THIRD_PARTY_NOTICES.md"])
        );
        assert_eq!(
            hop["wrapper"]["identical"].as_array().unwrap().len() + wrapper_changed.len(),
            5
        );
        assert_eq!(
            platform_changed.len() + hop["darwin_arm64"]["identical"].as_array().unwrap().len(),
            4
        );
    }
}

#[test]
fn claim_admits_every_proposed_point_and_rejects_outside_bounds() {
    assert_eq!(GROK_BUILD_CATALOGUE_VERSION, "1.0.30");
    let claim = grok_build_catalogue_claim();
    assert_eq!(claim.baseline().as_str(), "1.0.30");
    assert_eq!(claim.latest_qualified().as_str(), "1.0.70");
    for (minor, revision, status) in (30..=70).map(|minor| {
        if minor <= 45 {
            (
                minor,
                "grok-build.catalogue.models-text-v1",
                InterfaceSupportStatus::Deprecated,
            )
        } else {
            (
                minor,
                "grok-build.catalogue.models-text-v2",
                InterfaceSupportStatus::Maintained,
            )
        }
    }) {
        let candidate = format!("1.0.{minor}");
        let InterfaceCompatibilityAssessment::Qualified(matched) =
            claim.assess(&version(&candidate))
        else {
            panic!("{candidate} stays inside the catalogue window")
        };
        assert_eq!(matched.behavior_revision().as_str(), revision);
        assert_eq!(matched.support_status(), status);
    }
    for candidate in ["1.0.29", "1.0.71", "1.1.0", "0.2.117"] {
        assert_eq!(
            claim.assess(&version(candidate)),
            InterfaceCompatibilityAssessment::Incompatible,
            "{candidate} stays outside the catalogue window"
        );
    }
    // The ACP window keeps its own, independently reviewed ceiling.
    assert_eq!(GROK_BUILD_ACP_LATEST_QUALIFIED_VERSION, "1.0.46");
    assert_eq!(grok_build_acp_claim().latest_qualified().as_str(), "1.0.46");
    assert_ne!(
        grok_build_acp_claim().id(),
        grok_build_catalogue_claim().id()
    );
}
