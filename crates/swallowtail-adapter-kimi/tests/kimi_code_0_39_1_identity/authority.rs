//! ACP process-authority proofs.
//!
//! These fail if the excluded `0.39` points become admissible, if the
//! exclusion mechanism stops producing `Incompatible`, if the containment
//! trace is quietly reversed without evidence, or if the stop leaks onto the
//! headless or local-server families.

use super::support::{AUTHORITY, json, strings, text, version};
use swallowtail_adapter_kimi::{
    KIMI_CODE_LATEST_QUALIFIED_VERSION, kimi_acp_claim, kimi_headless_claim,
    kimi_local_server_claim,
};
use swallowtail_core::InterfaceCompatibilityAssessment;

/// The exact ACP points this route refuses.
const EXCLUDED: [&str; 2] = ["0.39.0", "0.39.1"];
/// The revision a permitted ACP point must carry. The stop adds no new one.
const ACP_REVISION: &str = "kimi.acp.reasoning.declared-effort-v2";

#[test]
fn the_excluded_points_classify_incompatible_not_unverified_newer() {
    let claim = kimi_acp_claim();
    for excluded in EXCLUDED {
        assert_eq!(
            claim.assess(&version(excluded)),
            InterfaceCompatibilityAssessment::Incompatible,
            "{excluded} must be refused outright"
        );
        assert!(!claim.permits(&version(excluded)));
        assert!(!claim.supports(&version(excluded)));
    }
}

#[test]
fn the_acp_ladder_around_the_ceiling_is_exact() {
    // The whole point of an exact stop is that it removes two published points
    // without collapsing the axis into qualified-only. This walks every
    // classification boundary the stop touches, against the real claim.
    let claim = kimi_acp_claim();

    // Unpublished 0.38.1 sits directly above the ceiling and stays admissible:
    // it is the first point a forward attempt could reach.
    let InterfaceCompatibilityAssessment::UnverifiedNewer(first) = claim.assess(&version("0.38.1"))
    else {
        panic!("0.38.1 is the first admissible unverified-newer point above the ceiling");
    };
    assert_eq!(first.version().as_str(), "0.38.1");
    assert_eq!(first.latest_qualified().as_str(), "0.38.0");
    assert_eq!(first.behavior_revision().as_str(), ACP_REVISION);

    // The two published 0.39 points are excluded outright.
    for excluded in EXCLUDED {
        assert_eq!(
            claim.assess(&version(excluded)),
            InterfaceCompatibilityAssessment::Incompatible,
            "{excluded} carries the uncontained process-authority change"
        );
    }

    // A point above the exclusions is admissible again on the same revision and
    // the same unmoved ceiling, so the stop is exact rather than a posture flip.
    let InterfaceCompatibilityAssessment::UnverifiedNewer(later) = claim.assess(&version("0.39.2"))
    else {
        panic!("0.39.2 remains admissible above the exclusions");
    };
    assert_eq!(later.version().as_str(), "0.39.2");
    assert_eq!(later.latest_qualified().as_str(), "0.38.0");
    assert_eq!(later.behavior_revision().as_str(), ACP_REVISION);

    // Both admissible points report the same ceiling and revision: the stop
    // introduced no second ACP behavior revision.
    assert_eq!(first.latest_qualified(), later.latest_qualified());
    assert_eq!(first.behavior_revision(), later.behavior_revision());
}

#[test]
fn allow_unverified_still_holds_for_points_above_the_exclusions() {
    // The stop is exact, not a posture change: the claim keeps admitting
    // genuinely later points, so this is not a silent qualified-only flip.
    let claim = kimi_acp_claim();
    assert!(matches!(
        claim.assess(&version("0.39.2")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert_eq!(KIMI_CODE_LATEST_QUALIFIED_VERSION, "0.38.0");
    assert!(claim.supports(&version("0.38.0")));
}

#[test]
fn the_containment_trace_is_recorded_as_absent_with_its_grounds() {
    let trace = &json(AUTHORITY)["containment_trace"];
    assert_eq!(trace["adapter_control_found"], false);
    assert_eq!(trace["runtime_control_found"], false);
    assert_eq!(trace["harness_isolation_declared"], "AmbientHost");
    assert!(
        text(
            &json(AUTHORITY),
            &["containment_trace", "contract_015_clause"]
        )
        .contains("neither callback authority nor filesystem containment")
    );
    assert!(
        text(&json(AUTHORITY), &["containment_trace", "conclusion"])
            .contains("containment is absent")
    );
}

#[test]
fn the_delta_is_reachable_because_terminal_is_always_advertised_false() {
    let identity = json(AUTHORITY);
    assert_eq!(
        identity["swallowtail_advertised_client_capabilities"]["terminal"],
        false
    );
    assert_eq!(
        identity["swallowtail_advertised_client_capabilities"]["auth.terminal"],
        false
    );
    assert_eq!(
        identity["reachability"]["terminal_enabled_is_always_false"],
        true
    );
    assert_eq!(
        identity["reachability"]["therefore_the_new_local_spawn_path_is_always_taken"],
        true
    );
    assert_eq!(
        identity["reachability"]["read_only_sessions_pass_resource_io_none"],
        true
    );
}

#[test]
fn the_disposition_is_a_stop_without_a_new_revision_or_a_new_shared_type() {
    let disposition = &json(AUTHORITY)["disposition"];
    assert_eq!(disposition["shape"], "stop");
    assert_eq!(disposition["acp_latest_qualified_stays"], "0.38.0");
    assert_eq!(strings(&disposition["exclude_exact"]), EXCLUDED);
    assert_eq!(disposition["exclusion_produces"], "Incompatible");
    assert_eq!(disposition["new_acp_behavior_revision"], false);
    assert_eq!(disposition["new_shared_type_required"], false);
    assert_eq!(disposition["wire_shape_stability_is_not_sufficient"], true);
    assert_eq!(
        disposition["acp_latest_qualified_stays"],
        KIMI_CODE_LATEST_QUALIFIED_VERSION
    );
}

#[test]
fn the_acp_stop_does_not_leak_onto_headless_or_local_server() {
    let identity = json(AUTHORITY);
    assert!(identity["headless_is_unaffected"]["reason"].is_string());
    assert_eq!(
        identity["headless_is_unaffected"]["headless_agent_process_authority_unchanged_across_0_38_0_to_0_39_1"],
        true
    );
    assert_eq!(identity["local_server_untouched"], true);

    // Headless keeps qualifying both points the ACP route refuses.
    let headless = kimi_headless_claim();
    for excluded in EXCLUDED {
        assert!(
            headless.supports(&version(excluded)),
            "{excluded} stays qualified on the headless axis"
        );
    }
    // Local-server neither gains nor loses anything.
    let local = kimi_local_server_claim();
    assert_eq!(local.latest_qualified().as_str(), "0.38.0");
    for excluded in EXCLUDED {
        assert!(matches!(
            local.assess(&version(excluded)),
            InterfaceCompatibilityAssessment::UnverifiedNewer(_)
        ));
    }
}
