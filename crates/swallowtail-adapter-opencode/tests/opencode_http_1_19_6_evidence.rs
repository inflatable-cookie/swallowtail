use sha2::{Digest, Sha256};
use swallowtail_adapter_opencode::{opencode_http_claim, opencode_server_binding};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceNewerVersionPosture, InterfaceSupportStatus,
};

#[test]
fn frozen_hop_ledgers_retain_the_reviewed_artifact_and_path_sets() {
    let ledgers = [
        (
            "identity",
            include_bytes!("fixtures/opencode-http-1.19.6/identity.json").as_slice(),
            "fdd0bb24788e450cfd59ad4da8dcc80535d6d99cbe9de7442caaae18e7b730c1",
        ),
        (
            "native inventory",
            include_bytes!("fixtures/opencode-http-1.19.6/native-inventory.json").as_slice(),
            "61c1532ca4a8eab94a8e115921de8c1bf67aff741f13854300d1e4c0ec5422a1",
        ),
        (
            "source inventory",
            include_bytes!("fixtures/opencode-http-1.19.6/source-inventory.json").as_slice(),
            "2b249846dca35d14babf0e478daaa4fe7a2ba9dc903126b2a392861f25296750",
        ),
        (
            "1.19.6 source delta",
            include_bytes!("fixtures/opencode-http-1.19.6/source-delta-1.19.6.tsv").as_slice(),
            "e96ec3888034f4fe15705d4b19e2087cacf4d25b7eaebe1bf5b6df013174a2de",
        ),
    ];
    for (name, contents, expected_sha256) in ledgers {
        assert_eq!(
            hex::encode(Sha256::digest(contents)),
            expected_sha256,
            "{name}"
        );
    }
}

#[test]
fn stopped_http_hops_remain_unverified_above_the_retained_ceiling() {
    let claim = opencode_http_claim();
    assert_eq!(claim.axis().as_str(), "opencode.server");
    assert_eq!(claim.latest_qualified().as_str(), "1.19.4");
    assert_eq!(
        claim.newer_version_posture(),
        InterfaceNewerVersionPosture::AllowUnverified
    );

    for version in ["1.18.11", "1.18.35", "1.19.0", "1.19.4"] {
        let binding = opencode_server_binding(version).expect("valid server version");
        let InterfaceCompatibilityAssessment::Qualified(segment) = claim.assess(binding.version())
        else {
            panic!("{version} must retain its qualified behavior");
        };
        assert_eq!(
            segment.behavior_revision().as_str(),
            "opencode.http-sse.surface-19"
        );
        assert_eq!(segment.support_status(), InterfaceSupportStatus::Maintained);
    }

    let gap = opencode_server_binding("1.18.36").expect("valid gap version");
    assert!(matches!(
        claim.assess(gap.version()),
        InterfaceCompatibilityAssessment::Incompatible
    ));

    for version in ["1.19.5", "1.19.6", "1.19.7"] {
        let binding = opencode_server_binding(version).expect("valid server version");
        assert!(
            matches!(
                claim.assess(binding.version()),
                InterfaceCompatibilityAssessment::UnverifiedNewer(_)
            ),
            "{version} must remain unverified"
        );
    }
}
