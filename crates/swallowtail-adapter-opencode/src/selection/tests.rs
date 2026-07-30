use super::{
    OPENCODE_BASELINE_VERSION, OPENCODE_LATEST_QUALIFIED_VERSION, opencode_http_claim,
    opencode_server_binding,
};
use std::collections::BTreeSet;
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceNewerVersionPosture, InterfaceSupportStatus,
};

const CORPUS: &str =
    include_str!("../../tests/fixtures/opencode-v1.14.48-v1.18.10/compatibility.json");

#[test]
fn claim_matches_every_frozen_release_and_surface() {
    let fixture: serde_json::Value = serde_json::from_str(CORPUS).expect("corpus parses");
    let claim = opencode_http_claim();
    assert_eq!(claim.id().as_str(), fixture["claim_id"]);
    assert_eq!(claim.axis().as_str(), fixture["axis"]);
    assert_eq!(claim.baseline().as_str(), OPENCODE_BASELINE_VERSION);
    assert_eq!(
        claim.latest_qualified().as_str(),
        OPENCODE_LATEST_QUALIFIED_VERSION
    );
    assert_eq!(
        claim.newer_version_posture(),
        InterfaceNewerVersionPosture::AllowUnverified
    );

    let releases = fixture["releases"].as_array().expect("release array");
    assert_eq!(releases.len(), 51);
    let mut versions = BTreeSet::new();
    for release in releases {
        let version = release["version"].as_str().expect("release version");
        assert!(versions.insert(version));
        let matched = claim
            .classify(
                opencode_server_binding(version)
                    .expect("frozen version is safe")
                    .version(),
            )
            .expect("frozen release is supported");
        assert_eq!(
            matched.behavior_revision().as_str(),
            format!(
                "opencode.http-sse.{}",
                release["surface"].as_str().expect("surface id")
            )
        );
        assert_eq!(matched.support_status(), InterfaceSupportStatus::Maintained);
        assert_eq!(release["commit"].as_str().expect("commit").len(), 40);
        assert_eq!(
            release["openapi_sha256"]
                .as_str()
                .expect("OpenAPI digest")
                .len(),
            64
        );
    }
    let release = |version: &str| {
        releases
            .iter()
            .find(|release| release["version"] == version)
            .expect("exact release is frozen")
    };
    assert_eq!(
        release("1.18.8")["artifact_delta"],
        "unselected-oauth-callback-optional-iss"
    );
    assert_eq!(
        release("1.18.9")["artifact_delta"],
        "unselected-oauth-callback-optional-iss-reverted"
    );
    assert_eq!(release("1.18.8")["surface"], release("1.18.10")["surface"]);
    assert_ne!(
        release("1.18.8")["openapi_sha256"],
        release("1.18.10")["openapi_sha256"]
    );
}

#[test]
fn claim_preserves_unpublished_and_outer_gaps() {
    let claim = opencode_http_claim();
    for rejected in [
        "1.14.47",
        "1.14.52",
        "1.15.8",
        "1.15.14",
        "1.16.1",
        "1.16.3",
        "1.17.21",
        "1.18.4-rc.1",
        "1.18.11-rc.1",
    ] {
        let binding = opencode_server_binding(rejected).expect("rejection is safe");
        assert!(!claim.supports(binding.version()), "{rejected} passed");
        assert!(!claim.permits(binding.version()), "{rejected} permitted");
    }
    let newer = opencode_server_binding("1.18.11").expect("newer version is safe");
    assert!(!claim.supports(newer.version()));
    assert!(claim.permits(newer.version()));
    let InterfaceCompatibilityAssessment::UnverifiedNewer(unverified) =
        claim.assess(newer.version())
    else {
        panic!("newer stable version must remain unverified");
    };
    assert_eq!(unverified.version(), newer.version());
    assert_eq!(unverified.latest_qualified().as_str(), "1.18.10");
    assert_eq!(
        unverified.behavior_revision().as_str(),
        "opencode.http-sse.surface-18"
    );
    for malformed in ["", " current", "current ", "current", "1.18.10\n"] {
        assert!(opencode_server_binding(malformed).is_none());
    }
    assert!(opencode_server_binding(&"1".repeat(65)).is_none());
}

#[test]
fn candidate_segments_match_the_recursively_closed_manifest() {
    let fixture: serde_json::Value = serde_json::from_str(CORPUS).expect("corpus parses");
    let claim = opencode_http_claim();
    let expected = fixture["segments"].as_array().expect("segment array");
    assert_eq!(claim.milestones().len(), expected.len());
    for (actual, expected) in claim.milestones().zip(expected) {
        assert_eq!(actual.minimum().as_str(), expected["minimum"]);
        assert_eq!(actual.maximum().as_str(), expected["maximum"]);
        assert_eq!(actual.behavior_revision().as_str(), expected["behavior"]);
    }
    assert_eq!(
        fixture["surface_revisions"]
            .as_array()
            .expect("surface revisions")
            .len(),
        18
    );
}
