//! Production claim state after the 1.0.41 compatible-extension claim card.
//!
//! The ACP executable window extends through official `1.0.41` on the
//! existing behavior revision. The exact catalogue claim and the
//! registered-tool courier bounded to the accepted live capsules stay
//! independent of the ACP window.

use super::identity::{COMPARED, HOPS, OFFICIAL_STABLE, PREVIOUS_CEILING};
use super::support::version;
use swallowtail_adapter_grok::{
    GROK_BUILD_ACP_LATEST_QUALIFIED_VERSION, grok_build_acp_claim, grok_build_catalogue_claim,
    grok_build_model_for_version, registered_tool,
};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceSupportStatus};

#[test]
fn production_claim_admits_every_hop_through_1_0_41_as_maintained() {
    let claim = grok_build_acp_claim();
    assert_eq!(GROK_BUILD_ACP_LATEST_QUALIFIED_VERSION, OFFICIAL_STABLE);
    for (point, behavior, status) in [
        (
            "0.2.114",
            "grok-build.acp-v1.cached-token-activation-v1",
            InterfaceSupportStatus::Deprecated,
        ),
        (
            "0.2.116",
            "grok-build.acp-v1.cached-token-activation-v1",
            InterfaceSupportStatus::Deprecated,
        ),
        (
            "0.2.117",
            "grok-build.acp-v1.cached-token-task-control-v2",
            InterfaceSupportStatus::Deprecated,
        ),
    ] {
        let InterfaceCompatibilityAssessment::Qualified(matched) = claim.assess(&version(point))
        else {
            panic!("{point} must stay qualified");
        };
        assert_eq!(matched.behavior_revision().as_str(), behavior);
        assert_eq!(matched.support_status(), status);
    }
    for point in COMPARED {
        if *point == PREVIOUS_CEILING {
            continue;
        }
        let InterfaceCompatibilityAssessment::Qualified(matched) = claim.assess(&version(point))
        else {
            panic!("{point} must be qualified after the claim card");
        };
        assert_eq!(
            matched.behavior_revision().as_str(),
            "grok-build.acp-v1.cached-token-model-4-6-v3"
        );
        assert_eq!(matched.support_status(), InterfaceSupportStatus::Maintained);
        assert_eq!(
            grok_build_model_for_version(&version(point)),
            Some("grok-4.6")
        );
    }
    for point in ["1.0.4", "1.0.5", PREVIOUS_CEILING] {
        let InterfaceCompatibilityAssessment::Qualified(matched) = claim.assess(&version(point))
        else {
            panic!("{point} must stay qualified");
        };
        assert_eq!(matched.support_status(), InterfaceSupportStatus::Maintained);
        assert_eq!(
            grok_build_model_for_version(&version(point)),
            Some("grok-4.6")
        );
    }
    for later in ["1.0.42", "1.1.0"] {
        let InterfaceCompatibilityAssessment::UnverifiedNewer(unverified) =
            claim.assess(&version(later))
        else {
            panic!("{later} must stay unverified newer");
        };
        assert_eq!(unverified.latest_qualified().as_str(), OFFICIAL_STABLE);
        assert_eq!(
            unverified.behavior_revision().as_str(),
            "grok-build.acp-v1.cached-token-model-4-6-v3"
        );
    }
    for gap in ["0.2.118", "0.2.121", "1.0.0", "1.0.3"] {
        assert_eq!(
            claim.assess(&version(gap)),
            InterfaceCompatibilityAssessment::Incompatible,
            "{gap} must stay incompatible"
        );
    }
    assert_eq!(HOPS.len(), COMPARED.len() - 1);
}

#[test]
fn the_exact_catalogue_claim_does_not_move_with_the_acp_window() {
    let catalogue = grok_build_catalogue_claim();
    let InterfaceCompatibilityAssessment::Qualified(matched) = catalogue.assess(&version("1.0.30"))
    else {
        panic!("exact 1.0.30 stays catalogue-qualified");
    };
    assert_eq!(
        matched.behavior_revision().as_str(),
        "grok-build.catalogue.models-text-v1"
    );
    for rejected in ["1.0.4", "1.0.25", "1.0.29", "1.0.31", "1.0.40", "1.0.41"] {
        assert_eq!(
            catalogue.assess(&version(rejected)),
            InterfaceCompatibilityAssessment::Incompatible,
            "{rejected} stays outside the exact catalogue point"
        );
    }
}

#[test]
fn the_registered_tool_courier_stays_on_the_accepted_live_capsules() {
    use swallowtail_runtime::RegisteredToolRouteQualification;
    for accepted in ["1.0.4", "1.0.5"] {
        assert_eq!(
            registered_tool::grok_build_acp_registered_tool_qualification(&version(accepted)),
            RegisteredToolRouteQualification::Qualified(
                registered_tool::GROK_ACP_REGISTERED_TOOL_ROUTE
            ),
            "{accepted} carries an accepted live capsule"
        );
    }
    for later in ["1.0.6", "1.0.17", "1.0.30", "1.0.40", "1.0.41"] {
        assert_eq!(
            registered_tool::grok_build_acp_registered_tool_qualification(&version(later)),
            RegisteredToolRouteQualification::Unqualified,
            "{later} carries no accepted registered-tool evidence"
        );
    }
}
