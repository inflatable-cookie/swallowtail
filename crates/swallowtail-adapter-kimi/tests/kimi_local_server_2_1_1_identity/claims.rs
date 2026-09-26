use super::support::{IDENTITY, PROTOCOL, json, version};
use swallowtail_adapter_kimi::{
    KIMI_CODE_LATEST_QUALIFIED_VERSION, KIMI_HEADLESS_LATEST_QUALIFIED_VERSION,
    KIMI_LOCAL_SERVER_BASELINE_VERSION, KIMI_LOCAL_SERVER_LATEST_QUALIFIED_VERSION, kimi_acp_claim,
    kimi_local_server_claim,
};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceNewerVersionPosture};

#[test]
fn production_local_server_claim_qualifies_through_2_1_1() {
    assert_eq!(KIMI_LOCAL_SERVER_BASELINE_VERSION, "0.28.1");
    assert_eq!(KIMI_LOCAL_SERVER_LATEST_QUALIFIED_VERSION, "2.1.1");
    let claim = kimi_local_server_claim();
    assert_eq!(claim.id().as_str(), "kimi.local-server.executable-window-6");
    assert_eq!(
        json(IDENTITY)["identity_decision"]["claim_id_becomes"],
        "kimi.local-server.executable-window-6"
    );
    assert_eq!(
        json(IDENTITY)["claim_at_observation"]["claim_id"],
        "kimi.local-server.executable-window-5"
    );
    assert_eq!(
        claim.newer_version_posture(),
        InterfaceNewerVersionPosture::AllowUnverified
    );
    for point in [
        "0.39.1", "0.40.0", "0.43.0", "0.43.1", "2.0.0", "2.0.1", "2.0.2", "2.1.0", "2.1.1",
    ] {
        assert!(
            claim.supports(&version(point)),
            "{point} must stay qualified under Q-004 B"
        );
    }
    for gap in [
        "0.39.2", "0.40.2", "0.41.1", "0.42.1", "0.43.2", "1.0.0", "2.0.3",
    ] {
        assert_eq!(
            claim.assess(&version(gap)),
            InterfaceCompatibilityAssessment::Incompatible,
            "{gap} stays a gap"
        );
        assert!(!claim.permits(&version(gap)));
    }
    assert!(matches!(
        claim.assess(&version("2.1.2")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert_eq!(
        json(IDENTITY)["identity_decision"]["widen_local_server_claim"],
        true
    );
    assert_eq!(
        json(IDENTITY)["identity_decision"]["edit_local_server_selection_rs"],
        true
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
