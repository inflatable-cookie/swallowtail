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
    assert_eq!(protocol["sdk_version"], "1.1.0");
    assert_eq!(protocol["initial_sdk_version"], "0.84.2");
    assert_eq!(
        protocol["sidecar_source_tag"],
        swallowtail_adapter_pi::sidecar::PI_SDK_SIDECAR_SOURCE_TAG
    );
    assert_eq!(protocol["node_runtime"], "22.23.2");
    assert_eq!(protocol["node_requirement"], ">=22.19.0");
    assert_eq!(
        protocol["compatibility_claim"],
        "qualified_only_exact_published_points"
    );
    assert_eq!(protocol["sidecar_entry_file"], "pi-sdk-sidecar.mjs");

    for (claim, axis, version) in [
        (
            pi_sdk_sidecar_package_claim(),
            PI_SDK_SIDECAR_PACKAGE_AXIS,
            "1.1.0",
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

    let qualified: Vec<String> = protocol["qualified_sdk_versions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|version| version.as_str().unwrap().to_owned())
        .collect();
    let source_versions = SIDECAR
        .split("const QUALIFIED_SDK_VERSIONS = new Set(")
        .nth(1)
        .expect("sidecar has an exact SDK version allowlist")
        .split(");")
        .next()
        .expect("sidecar SDK version allowlist is closed");
    let source_versions: Vec<String> = source_versions
        .trim()
        .strip_prefix('[')
        .expect("sidecar SDK allowlist opens as an array")
        .strip_suffix(']')
        .expect("sidecar SDK allowlist closes as an array")
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| {
            serde_json::from_str(line.trim_end_matches(','))
                .expect("SDK allowlist entry is a JSON string")
        })
        .collect();
    assert_eq!(qualified, source_versions);
    assert_eq!(qualified.last().map(String::as_str), Some("1.1.0"));

    // Exact points share the existing behavior revision. Gaps are rejected,
    // not treated as unverified-newer or inferred ranges.
    let package = pi_sdk_sidecar_package_claim();
    for point in &qualified {
        assert!(matches!(
            package.assess(&InterfaceVersion::new(point).expect("valid version")),
            InterfaceCompatibilityAssessment::Qualified(_)
        ));
    }
    for gap in [
        "0.84.5", "0.85.2", "0.86.2", "0.87.2", "0.88.0", "0.99.3", "1.0.5", "1.1.1",
    ] {
        let gap = InterfaceVersion::new(gap).expect("valid version");
        assert!(!package.permits(&gap));
        assert!(!matches!(
            package.assess(&gap),
            InterfaceCompatibilityAssessment::UnverifiedNewer(_)
        ));
    }
    let sidecar = pi_sdk_sidecar_sidecar_claim();
    assert!(
        sidecar.permits(
            &InterfaceVersion::new(protocol["sidecar_source_tag"].as_str().unwrap())
                .expect("source tag is valid")
        )
    );
    assert!(
        swallowtail_adapter_pi::sidecar::PI_SDK_SIDECAR_SOURCE_TAG
            .starts_with(protocol["sidecar_source_tag_prefix"].as_str().unwrap())
    );
    assert!(SIDECAR.contains("cacheWarming: \"off\""));
}

#[test]
fn sidecar_keeps_session_paths_inside_the_approved_directory() {
    assert!(SIDECAR.contains("state.sessionManager.listAll(state.sessionDir)"));
    assert!(SIDECAR.contains("realpath(matches[0].path)"));
    assert!(SIDECAR.contains("sessionRef: session.sessionId"));
    assert!(!SIDECAR.contains("sessionRef: session.sessionFile"));
    assert!(!SIDECAR.contains("existsSync(sessionRef)"));
}
