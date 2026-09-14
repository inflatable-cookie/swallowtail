//! Production claim state before and inside the identity card.
//!
//! The identity card freezes evidence only. These assertions record that the
//! production ACP claim still stops at `1.0.5` until the claim card lands; the
//! claim card rewrites this module to the after state.

use super::identity::{HOPS, OFFICIAL_STABLE, PREVIOUS_CEILING};
use super::support::{IDENTITY, json, version};
use swallowtail_adapter_grok::{
    GROK_BUILD_ACP_LATEST_QUALIFIED_VERSION, grok_build_acp_claim, grok_build_model_for_version,
};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceSupportStatus};

#[test]
fn production_claim_still_stops_at_the_previous_ceiling_inside_the_identity_card() {
    let claim = grok_build_acp_claim();
    assert_eq!(GROK_BUILD_ACP_LATEST_QUALIFIED_VERSION, PREVIOUS_CEILING);
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
        (
            "1.0.4",
            "grok-build.acp-v1.cached-token-model-4-6-v3",
            InterfaceSupportStatus::Maintained,
        ),
        (
            "1.0.5",
            "grok-build.acp-v1.cached-token-model-4-6-v3",
            InterfaceSupportStatus::Maintained,
        ),
    ] {
        let InterfaceCompatibilityAssessment::Qualified(matched) = claim.assess(&version(point))
        else {
            panic!("{point} must stay qualified inside the identity card");
        };
        assert_eq!(matched.behavior_revision().as_str(), behavior);
        assert_eq!(matched.support_status(), status);
    }
    assert_eq!(
        grok_build_model_for_version(&version("1.0.5")),
        Some("grok-4.6")
    );
    for point in HOPS {
        let InterfaceCompatibilityAssessment::UnverifiedNewer(unverified) =
            claim.assess(&version(point))
        else {
            panic!("{point} must stay unverified newer inside the identity card");
        };
        assert_eq!(unverified.latest_qualified().as_str(), PREVIOUS_CEILING);
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
    let decision = &json(IDENTITY)["identity_decision"];
    assert_eq!(decision["raise_latest_qualified_to"], OFFICIAL_STABLE);
}
