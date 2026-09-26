use super::support::{IDENTITY, PROTOCOL, json, version};
use swallowtail_adapter_kimi::{
    KIMI_CODE_LATEST_QUALIFIED_VERSION, KIMI_HEADLESS_LATEST_QUALIFIED_VERSION,
    KIMI_LOCAL_SERVER_BASELINE_VERSION, KIMI_LOCAL_SERVER_LATEST_QUALIFIED_VERSION, kimi_acp_claim,
    kimi_local_server_claim,
};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceNewerVersionPosture};

#[test]
fn production_local_server_claim_stays_at_0_39_1() {
    assert_eq!(KIMI_LOCAL_SERVER_BASELINE_VERSION, "0.28.1");
    assert_eq!(KIMI_LOCAL_SERVER_LATEST_QUALIFIED_VERSION, "0.39.1");
    let claim = kimi_local_server_claim();
    assert_eq!(claim.id().as_str(), "kimi.local-server.executable-window-5");
    assert_eq!(
        json(IDENTITY)["identity_decision"]["claim_id_stays"],
        "kimi.local-server.executable-window-5"
    );
    assert_eq!(
        claim.newer_version_posture(),
        InterfaceNewerVersionPosture::QualifiedOnly
    );
    assert!(claim.supports(&version("0.39.1")));
    for point in [
        "0.40.0", "0.43.0", "0.43.1", "2.0.0", "2.0.1", "2.0.2", "2.1.0", "2.1.1", "2.1.2",
    ] {
        assert_eq!(
            claim.assess(&version(point)),
            InterfaceCompatibilityAssessment::Incompatible,
            "{point} must fail closed"
        );
        assert!(!claim.permits(&version(point)));
    }
    assert_eq!(
        json(IDENTITY)["identity_decision"]["widen_local_server_claim"],
        false
    );
    assert_eq!(
        json(IDENTITY)["identity_decision"]["edit_local_server_selection_rs"],
        false
    );
}

#[test]
fn sibling_kimi_families_do_not_move() {
    assert_eq!(KIMI_CODE_LATEST_QUALIFIED_VERSION, "0.38.0");
    assert_eq!(KIMI_HEADLESS_LATEST_QUALIFIED_VERSION, "0.43.0");
    assert_eq!(
        kimi_acp_claim().assess(&version("2.1.1")),
        InterfaceCompatibilityAssessment::Incompatible
    );
    assert_eq!(
        json(IDENTITY)["identity_decision"]["flatten_onto_acp"],
        false
    );
    assert_eq!(
        json(IDENTITY)["identity_decision"]["flatten_onto_headless"],
        false
    );
    assert_eq!(
        json(PROTOCOL)["other_family_observations_not_acted_on"]["kimi_code_headless"],
        "headless claims, corpora, and conclusions untouched"
    );
}
