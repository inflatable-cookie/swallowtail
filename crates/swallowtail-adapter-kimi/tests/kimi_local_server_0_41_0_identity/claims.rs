use super::support::{IDENTITY, PROTOCOL, json, version};
use swallowtail_adapter_kimi::{
    KIMI_CODE_LATEST_QUALIFIED_VERSION, KIMI_HEADLESS_LATEST_QUALIFIED_VERSION,
    KIMI_LOCAL_SERVER_BASELINE_VERSION, KIMI_LOCAL_SERVER_LATEST_QUALIFIED_VERSION, kimi_acp_claim,
    kimi_headless_claim, kimi_local_server_claim,
};
use swallowtail_core::InterfaceCompatibilityAssessment;

#[test]
fn production_local_server_claim_bytes_stay_on_the_0_38_0_ceiling() {
    assert_eq!(KIMI_LOCAL_SERVER_BASELINE_VERSION, "0.28.1");
    assert_eq!(KIMI_LOCAL_SERVER_LATEST_QUALIFIED_VERSION, "0.38.0");
    let claim = kimi_local_server_claim();
    assert!(claim.supports(&version("0.38.0")));
    assert!(matches!(
        claim.assess(&version("0.41.0")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
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
    assert_eq!(KIMI_HEADLESS_LATEST_QUALIFIED_VERSION, "0.39.1");
    assert_eq!(
        kimi_acp_claim().assess(&version("0.41.0")),
        InterfaceCompatibilityAssessment::Incompatible
    );
    assert!(matches!(
        kimi_headless_claim().assess(&version("0.39.1")),
        InterfaceCompatibilityAssessment::Qualified(_)
    ));
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

#[test]
fn this_run_adds_no_public_operation_or_behavior_revision() {
    let decision = &json(IDENTITY)["identity_decision"];
    assert_eq!(decision["new_public_operation"], false);
    assert_eq!(decision["new_behavior_revision"], false);
    assert_eq!(decision["public_api_change"], false);
    assert_eq!(decision["touch_g05_009_card_034"], false);
}
