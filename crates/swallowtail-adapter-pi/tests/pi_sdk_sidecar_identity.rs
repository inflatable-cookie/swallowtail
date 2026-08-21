use serde_json::Value;
use swallowtail_adapter_pi::{
    PI_SDK_SIDECAR_NODE_AXIS, PI_SDK_SIDECAR_PACKAGE_AXIS, PI_SDK_SIDECAR_SIDECAR_AXIS,
    PI_SDK_SIDECAR_WIRE_AXIS, pi_sdk_sidecar_node_claim, pi_sdk_sidecar_package_claim,
    pi_sdk_sidecar_sidecar_claim, pi_sdk_sidecar_wire_claim,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const PROTOCOL: &str = include_str!("fixtures/pi-sdk-sidecar-v1/protocol.json");
const SIDECAR: &str = include_str!("../sidecar/pi-sdk-sidecar.mjs");

#[test]
fn sidecar_identity_and_claims_match_the_frozen_corpus() {
    let protocol: Value =
        serde_json::from_str(PROTOCOL).expect("Pi SDK sidecar protocol corpus is valid JSON");

    assert_eq!(protocol["wire"], "swallowtail-pi-sdk-jsonl-v1");
    assert_eq!(protocol["behavior_revision"], "pi.sdk-sidecar-v1");
    assert_eq!(protocol["sdk_package"], "@earendil-works/pi-coding-agent");
    assert_eq!(protocol["sdk_version"], "0.84.2");
    assert_eq!(protocol["node_runtime"], "22.23.2");
    assert_eq!(protocol["node_requirement"], ">=22.19.0");
    assert_eq!(protocol["compatibility_claim"], "qualified_only_one_point");
    assert_eq!(protocol["sidecar_entry_file"], "pi-sdk-sidecar.mjs");

    for (claim, axis, version) in [
        (
            pi_sdk_sidecar_package_claim(),
            PI_SDK_SIDECAR_PACKAGE_AXIS,
            "0.84.2",
        ),
        (
            pi_sdk_sidecar_node_claim(),
            PI_SDK_SIDECAR_NODE_AXIS,
            "22.23.2",
        ),
        (
            pi_sdk_sidecar_wire_claim(),
            PI_SDK_SIDECAR_WIRE_AXIS,
            "swallowtail-pi-sdk-jsonl-v1",
        ),
        (
            pi_sdk_sidecar_sidecar_claim(),
            PI_SDK_SIDECAR_SIDECAR_AXIS,
            swallowtail_adapter_pi::sidecar::PI_SDK_SIDECAR_SOURCE_TAG,
        ),
    ] {
        assert_eq!(claim.axis().as_str(), axis);
        assert!(matches!(
            claim.assess(&InterfaceVersion::new(version).expect("valid version")),
            InterfaceCompatibilityAssessment::Qualified(matched)
                if matched.support_status() == InterfaceSupportStatus::Maintained
                    && matched.behavior_revision().as_str() == "pi.sdk-sidecar-v1"
        ));
    }

    // The sidecar claims inherit nothing from the RPC window: later stable
    // points are rejected, not unverified-newer.
    let package = pi_sdk_sidecar_package_claim();
    assert!(!matches!(
        package.assess(&InterfaceVersion::new("0.84.3").expect("valid version")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert!(!package.permits(&InterfaceVersion::new("0.84.3").expect("valid version")));
    assert!(
        swallowtail_adapter_pi::sidecar::PI_SDK_SIDECAR_SOURCE_TAG
            .starts_with(protocol["sidecar_source_tag_prefix"].as_str().unwrap())
    );
}

#[test]
fn sidecar_keeps_session_paths_inside_the_approved_directory() {
    assert!(SIDECAR.contains("state.sessionManager.listAll(state.sessionDir)"));
    assert!(SIDECAR.contains("realpath(matches[0].path)"));
    assert!(SIDECAR.contains("sessionRef: session.sessionId"));
    assert!(!SIDECAR.contains("sessionRef: session.sessionFile"));
    assert!(!SIDECAR.contains("existsSync(sessionRef)"));
}
