use super::support::{IDENTITY, PROTOCOL, json, strings, text, version};
use swallowtail_adapter_kimi::{
    KIMI_CODE_AXIS, KIMI_CODE_BASELINE_VERSION, KIMI_CODE_LATEST_QUALIFIED_VERSION,
    KIMI_HEADLESS_BASELINE_VERSION, KIMI_HEADLESS_LATEST_QUALIFIED_VERSION,
    KIMI_LOCAL_SERVER_BASELINE_VERSION, KIMI_LOCAL_SERVER_LATEST_QUALIFIED_VERSION, kimi_acp_claim,
    kimi_acp_descriptor, kimi_headless_claim, kimi_headless_descriptor, kimi_local_server_claim,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceNewerVersionPosture, InterfaceSupportStatus,
    InterfaceVersionAxis, OperationShape,
};

#[test]
fn the_fixture_decision_is_the_shape_production_actually_encodes() {
    let identity = json(IDENTITY);
    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "split");
    assert_eq!(decision["new_behavior_revision"], false);
    assert_eq!(decision["new_public_operation"], false);
    assert_eq!(decision["public_api_change"], false);

    assert_eq!(decision["acp_axis"]["verdict"], "stop");
    assert_eq!(
        decision["acp_axis"]["latest_qualified_stays"],
        KIMI_CODE_LATEST_QUALIFIED_VERSION
    );
    assert_eq!(decision["headless_v1_axis"]["verdict"], "correct-down");
    assert_eq!(decision["headless_v1_axis"]["ceiling_was"], "0.37.2");
    assert_eq!(decision["headless_v1_axis"]["ceiling_becomes"], "0.32.0");
    assert_eq!(
        decision["headless_v2_axis"]["verdict"],
        "correct-down-and-extend"
    );
    assert_eq!(decision["headless_v2_axis"]["baseline_was"], "0.38.0");
    assert_eq!(decision["headless_v2_axis"]["baseline_becomes"], "0.33.0");
    assert_eq!(
        decision["headless_v2_axis"]["raise_range_to"],
        "0.33.0..=0.39.1"
    );
    assert_eq!(
        decision["raise_headless_latest_qualified_to"],
        KIMI_HEADLESS_LATEST_QUALIFIED_VERSION
    );
    assert_eq!(
        decision["synthetic_later_unverified_newer_headless"],
        "0.39.2"
    );
    assert_eq!(
        decision["host_0_34_0_reclassified_to"],
        "qualified_maintained_v2"
    );
}

#[test]
fn acp_holds_at_0_38_0_with_the_baseline_and_legacy_point_intact() {
    let claim = kimi_acp_claim();
    assert_eq!(claim.id().as_str(), "kimi.acp.executable-window-5");
    assert_eq!(
        claim.newer_version_posture(),
        InterfaceNewerVersionPosture::QualifiedOnly
    );
    assert_eq!(KIMI_CODE_BASELINE_VERSION, "0.28.1");

    let segments = claim.milestones().collect::<Vec<_>>();
    assert_eq!(segments.len(), 2);
    assert_eq!(segments[0].minimum().as_str(), "0.28.1");
    assert_eq!(segments[0].maximum().as_str(), "0.28.1");
    assert_eq!(
        segments[0].support_status(),
        InterfaceSupportStatus::Deprecated
    );
    assert_eq!(segments[1].minimum().as_str(), "0.29.0");
    assert_eq!(segments[1].maximum().as_str(), "0.38.0");
    assert_eq!(
        segments[1].support_status(),
        InterfaceSupportStatus::Maintained
    );
    assert_eq!(claim.latest_qualified().as_str(), "0.38.0");
}

#[test]
fn headless_v1_corrects_down_and_v2_corrects_down_and_extends() {
    let claim = kimi_headless_claim();
    assert_eq!(claim.id().as_str(), "kimi.headless.executable-window-2");
    assert_eq!(KIMI_HEADLESS_BASELINE_VERSION, "0.29.0");

    let segments = claim.milestones().collect::<Vec<_>>();
    assert_eq!(segments.len(), 2);
    assert_eq!(segments[0].minimum().as_str(), "0.29.0");
    assert_eq!(
        segments[0].maximum().as_str(),
        "0.32.0",
        "v1 ends before the v2-default boundary"
    );
    assert_eq!(
        segments[0].behavior_revision().as_str(),
        "kimi.headless.stream-json.v1"
    );
    assert_eq!(segments[1].minimum().as_str(), "0.33.0");
    assert_eq!(segments[1].maximum().as_str(), "0.39.1");
    assert_eq!(
        segments[1].behavior_revision().as_str(),
        "kimi.headless.stream-json.v2"
    );
    assert_eq!(claim.latest_qualified().as_str(), "0.39.1");

    let identity = json(IDENTITY);
    let reason = text(
        &identity,
        &["identity_decision", "headless_v1_axis", "reason"],
    );
    assert!(reason.contains("KIMI_CODE_EXPERIMENTAL_FLAG"));
    assert!(reason.contains("KIMI_CODE_LEGACY_FLAG"));
    assert!(reason.contains("0.33.0"));
}

#[test]
fn the_two_headless_segments_leave_no_gap_between_published_points() {
    let claim = kimi_headless_claim();
    // 0.32.0 and 0.33.0 are consecutive published points, so the correction
    // opens no unsupported hole between the revisions.
    for adjacent in ["0.32.0", "0.33.0"] {
        assert!(claim.supports(&version(adjacent)));
    }
    let identity = json(IDENTITY);
    let published_intermediates =
        strings(&identity["identity_decision"]["qualify_headless_intermediates"]);
    for point in &published_intermediates {
        assert!(
            claim.supports(&version(point)),
            "{point} is named as qualified and must be"
        );
    }
}

#[test]
fn a_later_exact_stable_stays_unverified_newer_on_the_headless_axis() {
    assert_eq!(
        json(IDENTITY)["publication_adjacency"]["unpublished_0_39_2"],
        true
    );
    let InterfaceCompatibilityAssessment::UnverifiedNewer(newer) =
        kimi_headless_claim().assess(&version("0.39.2"))
    else {
        panic!("0.39.2 stays unverified newer");
    };
    assert_eq!(
        newer.behavior_revision().as_str(),
        "kimi.headless.stream-json.v2"
    );
    assert_eq!(newer.latest_qualified().as_str(), "0.39.1");
}

#[test]
fn exact_negative_points_survive_the_correction() {
    let acp = kimi_acp_claim();
    for rejected in ["0.28.0", "0.28.2", "0.27.9", "0.29.0-rc.1"] {
        assert!(
            !acp.permits(&version(rejected)),
            "{rejected} stays outside ACP"
        );
    }
    let headless = kimi_headless_claim();
    for rejected in ["0.28.0", "0.28.1", "0.28.2"] {
        assert!(
            !headless.permits(&version(rejected)),
            "{rejected} stays outside headless"
        );
    }
}

#[test]
fn the_local_server_family_does_not_move_with_the_installed_harness_axes() {
    assert_eq!(KIMI_LOCAL_SERVER_BASELINE_VERSION, "0.28.1");
    assert_eq!(KIMI_LOCAL_SERVER_LATEST_QUALIFIED_VERSION, "0.38.0");
    let claim = kimi_local_server_claim();
    assert!(matches!(
        claim.assess(&version("0.38.1")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert_eq!(
        json(IDENTITY)["identity_decision"]["widen_local_server_claim"],
        false
    );
    assert!(
        json(PROTOCOL)["other_family_observations_not_acted_on"]["kimi_code_local_server"]
            .is_object(),
        "local-server deltas are recorded as observations only"
    );
}

#[test]
fn this_run_adds_no_public_operation_that_a_projection_gate_would_have_to_rank() {
    // The g05.009 gate is about provider-operation vocabulary. What this crate
    // can assert without reaching outside its own package is the thing that
    // gate would actually care about: the route's public operation surface is
    // unchanged, and no sibling claim moved.
    assert_eq!(
        json(IDENTITY)["identity_decision"]["new_public_operation"],
        false
    );
    assert_eq!(
        json(IDENTITY)["identity_decision"]["touch_g05_009_card_034"],
        false
    );

    let acp = kimi_acp_descriptor();
    assert!(acp.supports_operation_shape(OperationShape::InteractiveSession));
    assert!(!acp.supports_operation_shape(OperationShape::StructuredRun));

    let headless = kimi_headless_descriptor();
    assert!(headless.supports_operation_shape(OperationShape::StructuredRun));
    assert!(!headless.supports_operation_shape(OperationShape::InteractiveSession));

    // The claim each descriptor advertises must be the one this run edited,
    // so a descriptor cannot quietly carry a different window.
    assert_eq!(
        acp.interface_compatibility(&axis())
            .expect("ACP claim")
            .latest_qualified()
            .as_str(),
        "0.38.0"
    );
    assert_eq!(
        headless
            .interface_compatibility(&axis())
            .expect("headless claim")
            .latest_qualified()
            .as_str(),
        "0.39.1"
    );
}

fn axis() -> InterfaceVersionAxis {
    InterfaceVersionAxis::new(KIMI_CODE_AXIS).expect("static axis is valid")
}
