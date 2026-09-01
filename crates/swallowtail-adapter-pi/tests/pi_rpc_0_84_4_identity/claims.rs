use super::support::{IDENTITY, json, strings, version};
use swallowtail_adapter_pi::sidecar::PI_SDK_SIDECAR_SDK_VERSION;
use swallowtail_adapter_pi::{
    PI_PACKAGE_BASELINE_VERSION, PI_PACKAGE_LATEST_QUALIFIED_VERSION, PI_SDK_SIDECAR_PACKAGE_AXIS,
    pi_package_binding, pi_rpc_claim, pi_sdk_sidecar_package_claim,
};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceSupportStatus};

#[test]
fn identity_freezes_official_0_84_4_before_any_claim_edit() {
    let identity = json(IDENTITY);
    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(decision["keep_v0_83_0_exact"], true);
    assert_eq!(decision["extend_private_v0_84_0"], "0.84.0..=0.84.4");
    assert_eq!(
        decision["v0_84_0_behavior"],
        "pi.rpc.strict-lf-v0.84.0-message-update-delta"
    );
    assert_eq!(decision["raise_latest_qualified_to"], "0.84.4");
    assert_eq!(decision["keep_baseline"], "0.80.10");
    assert_eq!(decision["new_milestone"], false);
    assert_eq!(decision["flatten_to_oh_my_pi"], false);
    assert_eq!(decision["raise_sdk_sidecar"], false);
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "0.84.3"
    );
    assert_eq!(
        identity["claim_at_observation"]["classification_of_0_84_4"],
        "unverified_newer"
    );
    assert_eq!(PI_PACKAGE_BASELINE_VERSION, "0.80.10");
    assert_eq!(PI_PACKAGE_LATEST_QUALIFIED_VERSION, "0.84.3");

    let claim = pi_rpc_claim();
    assert!(matches!(
        claim.assess(&version("0.83.0")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Deprecated
                && matched.behavior_revision().as_str()
                    == "pi.rpc.strict-lf-v0.83.0-bash-extension-hook"
    ));
    for candidate in ["0.84.0", "0.84.1", "0.84.2", "0.84.3"] {
        assert!(matches!(
            claim.assess(&version(candidate)),
            InterfaceCompatibilityAssessment::Qualified(matched)
                if matched.support_status() == InterfaceSupportStatus::Maintained
                    && matched.behavior_revision().as_str()
                        == "pi.rpc.strict-lf-v0.84.0-message-update-delta"
        ));
    }
    assert!(matches!(
        claim.assess(&version("0.84.4")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert_eq!(
        pi_package_binding("0.84.4")
            .expect("version binds")
            .axis()
            .as_str(),
        swallowtail_adapter_pi::PI_PACKAGE_AXIS
    );
}

#[test]
fn unpublished_gaps_and_later_0_84_5_stay_classified() {
    let identity = json(IDENTITY);
    assert_eq!(
        strings(&identity["published_stables_from_previous_ceiling"]),
        ["0.84.4"]
    );
    assert_eq!(identity["unpublished_0_83_1"], true);
    assert_eq!(identity["unpublished_0_84_5"], true);
    assert_eq!(
        identity["identity_decision"]["keep_unpublished_0_83_1"],
        true
    );
    assert_eq!(
        identity["identity_decision"]["later_unverified_after_qualification"],
        "0.84.5"
    );
    assert_eq!(
        identity["identity_decision"]["later_unverified_published"],
        false
    );
    let claim = pi_rpc_claim();
    assert!(!claim.permits(&version("0.83.1")));
    assert!(!claim.permits(&version("0.84.5-rc.1")));
}

#[test]
fn sdk_sidecar_stays_exact_0_84_2() {
    let identity = json(IDENTITY);
    assert_eq!(identity["sidecar_package_at_observation"], "0.84.2");
    assert_eq!(
        identity["claim_at_observation"]["sidecar_latest_qualified"],
        "0.84.2"
    );
    assert_eq!(PI_SDK_SIDECAR_SDK_VERSION, "0.84.2");
    let sidecar = pi_sdk_sidecar_package_claim();
    assert_eq!(sidecar.axis().as_str(), PI_SDK_SIDECAR_PACKAGE_AXIS);
    assert!(matches!(
        sidecar.assess(&version("0.84.2")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Maintained
    ));
    assert!(!sidecar.permits(&version("0.84.4")));
    assert!(!matches!(
        sidecar.assess(&version("0.84.4")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
}
