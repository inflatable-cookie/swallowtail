use super::support::{IDENTITY, PROTOCOL, json, version};
use swallowtail_adapter_kimi::{
    KIMI_CODE_LATEST_QUALIFIED_VERSION, KIMI_HEADLESS_LATEST_QUALIFIED_VERSION,
    KIMI_LOCAL_SERVER_BASELINE_VERSION, KIMI_LOCAL_SERVER_LATEST_QUALIFIED_VERSION, kimi_acp_claim,
    kimi_headless_claim, kimi_local_server_claim,
};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceNewerVersionPosture};

#[test]
fn production_local_server_claim_lands_the_0_39_1_ceiling_and_fails_closed() {
    assert_eq!(KIMI_LOCAL_SERVER_BASELINE_VERSION, "0.28.1");
    assert_eq!(KIMI_LOCAL_SERVER_LATEST_QUALIFIED_VERSION, "0.39.1");
    let claim = kimi_local_server_claim();
    assert_eq!(
        claim.newer_version_posture(),
        InterfaceNewerVersionPosture::QualifiedOnly
    );
    assert!(claim.supports(&version("0.39.0")));
    assert!(claim.supports(&version("0.39.1")));
    // Every point above the ceiling fails closed, including unpublished
    // points and the whole published 0.40.0..=0.43.0 gap.
    for point in [
        "0.39.2", "0.40.0", "0.40.1", "0.41.0", "0.42.0", "0.43.0", "0.43.1",
    ] {
        assert_eq!(
            claim.assess(&version(point)),
            InterfaceCompatibilityAssessment::Incompatible,
            "{point} must fail closed"
        );
        assert!(!claim.permits(&version(point)));
        assert!(!claim.supports(&version(point)));
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
fn flipping_local_server_back_to_allow_unverified_fails_the_closed_proof() {
    let original = kimi_local_server_claim();
    let mutated = swallowtail_core::InterfaceCompatibilityClaim::new(
        original.id().clone(),
        original.axis().clone(),
        original.scheme(),
        InterfaceNewerVersionPosture::AllowUnverified,
        original.milestones().cloned(),
        original.exclusions().cloned(),
    )
    .expect("mutated claim stays structurally valid");
    for point in ["0.40.0", "0.43.0"] {
        assert!(
            matches!(
                mutated.assess(&version(point)),
                InterfaceCompatibilityAssessment::UnverifiedNewer(_)
            ),
            "{point} would pass through AllowUnverified; the fail-closed proof would fail"
        );
        assert_eq!(
            original.assess(&version(point)),
            InterfaceCompatibilityAssessment::Incompatible
        );
    }
}

#[test]
fn sibling_kimi_families_do_not_move() {
    assert_eq!(KIMI_CODE_LATEST_QUALIFIED_VERSION, "0.38.0");
    assert_eq!(KIMI_HEADLESS_LATEST_QUALIFIED_VERSION, "0.43.0");
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
