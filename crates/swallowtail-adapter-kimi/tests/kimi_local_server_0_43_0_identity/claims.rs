use super::support::{IDENTITY, PROTOCOL, json, version};
use swallowtail_adapter_kimi::{
    KIMI_CODE_LATEST_QUALIFIED_VERSION, KIMI_HEADLESS_LATEST_QUALIFIED_VERSION,
    KIMI_LOCAL_SERVER_BASELINE_VERSION, KIMI_LOCAL_SERVER_LATEST_QUALIFIED_VERSION, kimi_acp_claim,
    kimi_headless_claim, kimi_local_server_claim,
};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceNewerVersionPosture};

#[test]
fn production_local_server_claim_keeps_the_0_39_x_prefix_after_q004_b() {
    assert_eq!(KIMI_LOCAL_SERVER_BASELINE_VERSION, "0.28.1");
    assert_eq!(KIMI_LOCAL_SERVER_LATEST_QUALIFIED_VERSION, "2.1.1");
    let claim = kimi_local_server_claim();
    assert_eq!(claim.id().as_str(), "kimi.local-server.executable-window-6");
    assert_eq!(
        json(IDENTITY)["identity_decision"]["claim_id_becomes"],
        "kimi.local-server.executable-window-5"
    );
    assert_eq!(
        claim.newer_version_posture(),
        InterfaceNewerVersionPosture::AllowUnverified
    );
    assert!(claim.supports(&version("0.39.0")));
    assert!(claim.supports(&version("0.39.1")));
    // Q-004 B later qualified the published 0.40.0..=0.43.1 span. Unpublished
    // 0.39.2 stays a gap between the 0.39.1 and 0.40.0 segments.
    assert_eq!(
        claim.assess(&version("0.39.2")),
        InterfaceCompatibilityAssessment::Incompatible
    );
    for point in ["0.40.0", "0.40.1", "0.41.0", "0.42.0", "0.43.0", "0.43.1"] {
        assert!(
            claim.supports(&version(point)),
            "{point} is qualified under Q-004 B"
        );
    }
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
fn frozen_326_decision_still_records_fail_closed_at_observation() {
    let decision = &json(IDENTITY)["identity_decision"];
    assert_eq!(decision["posture_becomes"], "qualified_only");
    assert_eq!(decision["latest_qualified_becomes"], "0.39.1");
    assert_eq!(decision["rejected_gap"], "0.40.0..=0.43.0");
    let live = kimi_local_server_claim();
    assert_eq!(
        live.newer_version_posture(),
        InterfaceNewerVersionPosture::AllowUnverified
    );
    assert!(live.supports(&version("0.40.0")));
    assert!(live.supports(&version("0.43.0")));
}

#[test]
fn sibling_kimi_families_do_not_move() {
    assert_eq!(KIMI_CODE_LATEST_QUALIFIED_VERSION, "0.38.0");
    assert_eq!(KIMI_HEADLESS_LATEST_QUALIFIED_VERSION, "2.1.1");
    assert_eq!(
        kimi_acp_claim().assess(&version("0.43.0")),
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
