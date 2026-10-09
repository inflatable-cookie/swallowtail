//! Frozen identity evidence for the Grok Build catalogue currentness stop.
//!
//! The official stable channel reaches `1.0.46`, whose Darwin ARM64 artifact
//! adds a default-not-in-list header the current catalogue contract cannot
//! represent. The ledger preserves all package identities and complete file
//! inventories through the hop; the production claim deliberately stays at
//! exact `1.0.30` until a same-contract adaptation or ruling lands.

use serde_json::Value;
use swallowtail_adapter_grok::{GROK_BUILD_CATALOGUE_VERSION, grok_build_catalogue_claim};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

const IDENTITY: &str = include_str!("fixtures/grok-1.0.46-catalogue/identity.json");
const DIST_INVENTORY: &str = include_str!("fixtures/grok-1.0.46-catalogue/dist-inventory.json");
const RUNTIME_SURFACE: &str = include_str!("fixtures/grok-1.0.46-catalogue/runtime-surface.json");

fn json(source: &str) -> Value {
    serde_json::from_str(source).expect("frozen identity fixture is valid JSON")
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}

#[test]
fn official_latest_identity_and_hop_ledgers_freeze_the_stop() {
    let identity = json(IDENTITY);
    assert_eq!(identity["schema"], "swallowtail.grok.catalogue.identity.v1");
    assert_eq!(identity["observed_at"], "2026-10-08");
    assert_eq!(identity["axis"], "grok-build.executable");
    assert_eq!(
        identity["official_channel"]["package"],
        "@xai-official/grok"
    );
    assert_eq!(identity["official_channel"]["selected_tag"], "latest");
    assert_eq!(identity["official_channel"]["official_stable"], "1.0.46");
    assert_eq!(identity["official_channel"]["dist_tags"]["alpha"], "1.0.50");
    assert_eq!(
        identity["published_selected_channel_hops"]
            .as_array()
            .unwrap()
            .len(),
        16
    );
    assert_eq!(
        identity["compared_artifact_versions"]
            .as_array()
            .unwrap()
            .len(),
        17
    );

    let hops = identity["hops"].as_array().expect("hops are an array");
    assert_eq!(hops.len(), 17);
    for (index, hop) in hops.iter().enumerate() {
        let expected_minor = index + 30;
        let expected_version = format!("1.0.{expected_minor}");
        assert_eq!(hop["version"], expected_version);
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
        }
        assert_eq!(
            hop["catalogue_default_not_in_list_marker_count"],
            if expected_minor == 46 { 1 } else { 0 }
        );
    }

    let inventory = json(DIST_INVENTORY);
    assert_eq!(inventory["platform"], "darwin-arm64");
    assert_eq!(inventory["compared"].as_array().unwrap().len(), 17);
    assert_eq!(inventory["files"].as_object().unwrap().len(), 2);
    let wrapper = inventory["files"]["wrapper"].as_object().unwrap();
    let platform = inventory["files"]["darwin_arm64"].as_object().unwrap();
    assert_eq!(wrapper.len(), 17);
    assert_eq!(platform.len(), 17);
    for minor in 30..=46 {
        let version = format!("1.0.{minor}");
        let wrapper_files = wrapper[&version].as_object().unwrap();
        let platform_files = platform[&version].as_object().unwrap();
        assert_eq!(wrapper_files.len(), 5);
        assert_eq!(platform_files.len(), 4);
        assert_eq!(
            wrapper_files["package.json"]["sha256"]
                .as_str()
                .unwrap()
                .len(),
            64
        );
        assert_eq!(
            platform_files["bin/grok.br"]["sha256"]
                .as_str()
                .unwrap()
                .len(),
            64
        );
    }
    for hop in inventory["hops"].as_array().unwrap() {
        assert_eq!(hop["wrapper"]["added"].as_array().unwrap().len(), 0);
        assert_eq!(hop["wrapper"]["removed"].as_array().unwrap().len(), 0);
        assert_eq!(
            hop["wrapper"]["changed"],
            serde_json::json!(["package.json"])
        );
        assert_eq!(hop["wrapper"]["identical"].as_array().unwrap().len(), 4);
        assert_eq!(hop["darwin_arm64"]["added"].as_array().unwrap().len(), 0);
        assert_eq!(hop["darwin_arm64"]["removed"].as_array().unwrap().len(), 0);
        assert_eq!(
            hop["darwin_arm64"]["changed"],
            serde_json::json!(["bin/grok.br", "package.json"])
        );
        assert_eq!(
            hop["darwin_arm64"]["identical"].as_array().unwrap().len(),
            2
        );
    }

    let runtime = json(RUNTIME_SURFACE);
    assert_eq!(
        runtime["schema"],
        "swallowtail.grok-build-catalogue.runtime-surface.v1"
    );
    assert_eq!(runtime["target"], "darwin-arm64");
    let observations = runtime["observations"].as_object().unwrap();
    assert_eq!(observations.len(), 17);
    for minor in 30..=46 {
        let version = format!("1.0.{minor}");
        let observation = &observations[version.as_str()];
        let expected_model = serde_json::json!(["grok-4.6", "grok-4.5"]);
        for embedded in observation["embedded_default_models"].as_array().unwrap() {
            assert_eq!(embedded["default"], "grok-4.6");
            assert_eq!(embedded["model_ids"], expected_model);
        }
        if minor == 46 {
            assert_eq!(
                observation["runtime_sha256"],
                "e8daa302364c9c3b6a5546d511cfbd1ab5e5d407a9b04282f660665ea405f9f3"
            );
        }
    }

    assert_eq!(identity["decision"]["identity_decision"], "stop");
    assert_eq!(identity["decision"]["latest_qualified_after"], "1.0.30");
    assert_eq!(identity["decision"]["claim_after"], "unchanged");
}

#[test]
fn exact_catalogue_claim_stays_at_1_0_30_and_fails_closed_after_it() {
    assert_eq!(GROK_BUILD_CATALOGUE_VERSION, "1.0.30");
    let claim = grok_build_catalogue_claim();
    assert!(matches!(
        claim.assess(&version("1.0.30")),
        InterfaceCompatibilityAssessment::Qualified(_)
    ));
    for rejected in ["1.0.31", "1.0.46"] {
        assert_eq!(
            claim.assess(&version(rejected)),
            InterfaceCompatibilityAssessment::Incompatible,
            "{rejected} remains outside the exact catalogue claim"
        );
    }
}
