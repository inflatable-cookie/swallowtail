use serde_json::Value;
use swallowtail_adapter_bedrock::{
    CATALOGUE_SDK_VERSION, SDK_VERSION, bedrock_catalogue_interface_bindings,
    bedrock_catalogue_interface_claims, bedrock_runtime_interface_bindings,
    bedrock_runtime_interface_claims,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceCompatibilityClaim, InterfaceNewerVersionPosture,
    InterfaceSupportStatus, InterfaceVersion, InterfaceVersionScheme,
};

const HISTORICAL_PROTOCOL: &str = include_str!("fixtures/bedrock-runtime-1.136.0/protocol.json");

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}

#[test]
fn runtime_sdk_claim_preserves_the_old_point_gaps_and_yanked_hole() {
    let [sdk, service] = bedrock_runtime_interface_claims();
    assert_eq!(sdk.id().as_str(), "amazon-bedrock.runtime-sdk-window-1");
    assert_eq!(sdk.axis().as_str(), "amazon-bedrock.runtime-rust-sdk");
    assert_eq!(sdk.scheme(), InterfaceVersionScheme::Semantic);
    assert_eq!(
        sdk.newer_version_posture(),
        InterfaceNewerVersionPosture::AllowUnverified
    );
    assert_eq!(sdk.baseline().as_str(), "1.136.0");
    assert_eq!(sdk.latest_qualified().as_str(), SDK_VERSION);
    assert_eq!(sdk.latest_qualified().as_str(), "1.148.0");

    let segments = sdk.milestones().collect::<Vec<_>>();
    assert_eq!(segments.len(), 2);
    assert_eq!(segments[0].minimum().as_str(), "1.136.0");
    assert_eq!(segments[0].maximum().as_str(), "1.136.0");
    assert_eq!(segments[1].minimum().as_str(), "1.139.0");
    assert_eq!(segments[1].maximum().as_str(), "1.148.0");
    assert!(segments.iter().all(|segment| {
        segment.behavior_revision().as_str() == "amazon-bedrock.runtime-sdk-1"
            && segment.support_status() == InterfaceSupportStatus::Maintained
    }));
    assert_eq!(
        sdk.exclusions()
            .map(InterfaceVersion::as_str)
            .collect::<Vec<_>>(),
        ["1.144.0"]
    );

    assert!(matches!(
        sdk.assess(&version("1.136.0")),
        InterfaceCompatibilityAssessment::Qualified(_)
    ));
    for gap in ["1.137.0", "1.138.0", "1.144.0"] {
        assert_eq!(
            sdk.assess(&version(gap)),
            InterfaceCompatibilityAssessment::Incompatible
        );
    }
    for stable in [
        "1.139.0", "1.140.0", "1.141.0", "1.142.0", "1.143.0", "1.145.0", "1.146.0", "1.147.0",
        "1.148.0",
    ] {
        assert!(matches!(
            sdk.assess(&version(stable)),
            InterfaceCompatibilityAssessment::Qualified(_)
        ));
    }
    assert!(matches!(
        sdk.assess(&version("1.149.0")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(newer)
            if newer.latest_qualified().as_str() == "1.148.0"
    ));

    assert_eq!(
        service.id().as_str(),
        "amazon-bedrock.runtime-service-window-1"
    );
    assert_exact_opaque_claim(
        &service,
        "amazon-bedrock.runtime-service-api",
        "bedrock-runtime-converse-stream",
        "amazon-bedrock.runtime-service-1",
    );
    let runtime_bindings = bedrock_runtime_interface_bindings();
    assert_eq!(runtime_bindings[0].version().as_str(), SDK_VERSION);
    assert_eq!(
        runtime_bindings[1].version().as_str(),
        "bedrock-runtime-converse-stream"
    );
}

#[test]
fn catalogue_claims_and_historical_runtime_fixture_remain_exact() {
    let [catalogue_sdk, catalogue_service] = bedrock_catalogue_interface_claims();
    assert_eq!(
        catalogue_sdk.id().as_str(),
        "amazon-bedrock.catalogue-sdk-window-1"
    );
    assert_exact_opaque_claim(
        &catalogue_sdk,
        "amazon-bedrock.control-plane-rust-sdk",
        CATALOGUE_SDK_VERSION,
        "amazon-bedrock.catalogue-sdk-1",
    );
    assert_eq!(
        catalogue_service.id().as_str(),
        "amazon-bedrock.catalogue-service-window-1"
    );
    assert_exact_opaque_claim(
        &catalogue_service,
        "amazon-bedrock.control-plane-service-api",
        "bedrock-list-foundation-models",
        "amazon-bedrock.catalogue-service-1",
    );
    let catalogue_bindings = bedrock_catalogue_interface_bindings();
    assert_eq!(
        catalogue_bindings[0].version().as_str(),
        CATALOGUE_SDK_VERSION
    );
    assert_eq!(
        catalogue_bindings[1].version().as_str(),
        "bedrock-list-foundation-models"
    );

    let historical: Value =
        serde_json::from_str(HISTORICAL_PROTOCOL).expect("historical fixture remains valid");
    assert_eq!(historical["sdk_version"], "1.136.0");
}

fn assert_exact_opaque_claim(
    claim: &InterfaceCompatibilityClaim,
    axis: &str,
    exact_version: &str,
    behavior: &str,
) {
    assert_eq!(claim.axis().as_str(), axis);
    assert_eq!(claim.scheme(), InterfaceVersionScheme::Opaque);
    assert_eq!(
        claim.newer_version_posture(),
        InterfaceNewerVersionPosture::QualifiedOnly
    );
    assert_eq!(claim.baseline().as_str(), exact_version);
    assert_eq!(claim.latest_qualified().as_str(), exact_version);
    assert_eq!(claim.milestones().len(), 1);
    assert_eq!(
        claim
            .milestones()
            .next()
            .unwrap()
            .behavior_revision()
            .as_str(),
        behavior
    );
    assert!(claim.exclusions().next().is_none());
}
