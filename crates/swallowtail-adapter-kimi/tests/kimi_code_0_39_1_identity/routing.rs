//! Engine-routing boundary proofs.
//!
//! These fail if the boundary point moves, if the legacy-flag logic is
//! inverted, if `system.version` is treated as a v1 line, if an ambient
//! legacy-flag assumption is invented, or if a historical corpus is left
//! asserting the superseded `0.38.0` boundary.

use super::support::{
    FROZEN_0_37_2_PROTOCOL, FROZEN_0_37_2_README, FROZEN_0_38_0_README, ROUTING_IDENTITY,
    ROUTING_PROTOCOL, json, strings, text, version,
};
use swallowtail_adapter_kimi::{kimi_headless_claim, kimi_local_server_claim};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceSupportStatus};

/// The exact published point where the default `-p` engine becomes v2.
const BOUNDARY: &str = "0.33.0";
/// The newest published point whose default `-p` engine is still v1.
const LAST_V1: &str = "0.32.0";

#[test]
fn the_routing_boundary_is_exact_and_the_two_points_are_adjacent() {
    let routing = json(ROUTING_IDENTITY);
    assert_eq!(routing["answer"], BOUNDARY);
    assert_eq!(routing["last_v1_default"]["version"], LAST_V1);
    assert_eq!(routing["first_v2_default"]["version"], BOUNDARY);
    assert_eq!(
        routing["last_v1_default"]["default_engine"],
        "agent-core-v1-print"
    );
    assert_eq!(
        routing["first_v2_default"]["default_engine"],
        "agent-core-v2-run-v2-print"
    );
}

#[test]
fn the_gate_env_var_and_its_polarity_are_pinned_on_both_sides() {
    let routing = json(ROUTING_IDENTITY);
    assert_eq!(
        routing["last_v1_default"]["engine_gate_env"],
        "KIMI_CODE_EXPERIMENTAL_FLAG"
    );
    assert_eq!(
        routing["first_v2_default"]["engine_gate_env"],
        "KIMI_CODE_LEGACY_FLAG"
    );
    // Polarity matters as much as the name: before the boundary the flag opts
    // *in* to v2, after it the flag opts *out*.
    assert!(
        text(&routing, &["last_v1_default", "engine_gate_semantics"])
            .contains("true only when KIMI_CODE_EXPERIMENTAL_FLAG is truthy")
    );
    assert!(
        text(&routing, &["first_v2_default", "engine_gate_semantics"])
            .contains("!isLegacyEnabled()")
    );
    // The legacy flag cannot be assumed ambient before it exists.
    assert_eq!(
        routing["last_v1_default"]["legacy_env_present_in_bundle"],
        false
    );
    assert_eq!(
        routing["first_v2_default"]["legacy_env_present_in_bundle"],
        true
    );
    assert_eq!(routing["swallowtail_argv_sets_legacy_flag"], false);
}

#[test]
fn the_production_claim_splits_at_exactly_that_boundary() {
    let claim = kimi_headless_claim();
    let segments = claim.milestones().collect::<Vec<_>>();
    assert_eq!(segments.len(), 2);
    assert_eq!(segments[0].maximum().as_str(), LAST_V1);
    assert_eq!(segments[1].minimum().as_str(), BOUNDARY);

    let InterfaceCompatibilityAssessment::Qualified(last_v1) = claim.assess(&version(LAST_V1))
    else {
        panic!("{LAST_V1} qualifies");
    };
    assert_eq!(
        last_v1.behavior_revision().as_str(),
        "kimi.headless.stream-json.v1"
    );
    let InterfaceCompatibilityAssessment::Qualified(first_v2) = claim.assess(&version(BOUNDARY))
    else {
        panic!("{BOUNDARY} qualifies");
    };
    assert_eq!(
        first_v2.behavior_revision().as_str(),
        "kimi.headless.stream-json.v2"
    );
    assert_eq!(
        first_v2.support_status(),
        InterfaceSupportStatus::Maintained
    );
}

#[test]
fn the_previously_broken_span_is_now_v2_including_the_recorded_host() {
    let claim = kimi_headless_claim();
    // Every point that g04.064 left claimed as v1 while shipping a v2 default.
    for repaired in [
        "0.33.0", "0.34.0", "0.35.0", "0.36.0", "0.36.1", "0.37.0", "0.37.1", "0.37.2",
    ] {
        let InterfaceCompatibilityAssessment::Qualified(matched) = claim.assess(&version(repaired))
        else {
            panic!("{repaired} qualifies");
        };
        assert_eq!(
            matched.behavior_revision().as_str(),
            "kimi.headless.stream-json.v2",
            "{repaired} must not be classified v1"
        );
    }
    let routing = json(ROUTING_IDENTITY);
    assert_eq!(routing["host_at_observation"]["version"], "0.34.0");
    assert_eq!(
        routing["host_at_observation"]["corrected_classification"],
        "qualified maintained kimi.headless.stream-json.v2"
    );
}

#[test]
fn the_v1_decoder_rejecting_the_v2_preamble_is_why_the_span_was_broken() {
    let routing = json(ROUTING_PROTOCOL);
    assert_eq!(routing["swallowtail_v1_decoder_rejects_v2_preamble"], true);
    assert_eq!(routing["swallowtail_sets_legacy_flag"], false);
}

#[test]
fn the_mapped_v2_emission_surfaces_are_stable_across_the_whole_admitted_range() {
    let protocol = json(ROUTING_PROTOCOL);
    let surfaces = protocol["v2_mapped_surface_digests"]
        .as_object()
        .expect("surface map");
    assert!(!surfaces.is_empty());
    for (name, entry) in surfaces {
        let digest = entry["digest"].as_str().expect("digest is text");
        assert_eq!(digest.len(), 64, "{name} digest is sha256");
        // Each surface must be stable at or before the boundary and remain so
        // through the newest admitted point.
        let from = entry["stable_from"].as_str().expect("stable_from is text");
        let through = entry["stable_through"]
            .as_str()
            .expect("stable_through is text");
        assert!(from <= BOUNDARY, "{name} is not stable by the boundary");
        assert_eq!(through, "0.39.1", "{name} is not stable to the ceiling");
    }
}

#[test]
fn the_one_moving_v2_surface_is_recorded_as_a_retype_not_smoothed_over() {
    let retype = &json(ROUTING_PROTOCOL)["v2_dispatch_retype_at_0_37_0"];
    let before = retype["digest_0_33_0_through_0_36_1"]
        .as_str()
        .expect("digest is text");
    let after = retype["digest_0_37_0_through_0_39_1"]
        .as_str()
        .expect("digest is text");
    assert_ne!(before, after);
    assert_eq!(retype["kind"], "typescript-retype-only");
    assert_eq!(retype["emitted_jsonl_unchanged"], true);
    assert_eq!(retype["case_labels_identical_0_33_0_through_0_39_1"], true);
    // The case labels are the mapped grammar; naming them makes a silent
    // addition or removal fail here.
    assert_eq!(
        strings(&retype["case_labels"]),
        [
            "turn.step.started",
            "turn.step.interrupted",
            "turn.step.retrying",
            "assistant.delta",
            "hook.result",
            "thinking.delta",
            "tool.call.started",
            "tool.call.delta",
            "tool.result",
            "tool.progress",
        ]
    );
}

#[test]
fn historical_corpora_no_longer_assert_the_superseded_boundary() {
    // The 0.37.2 corpus read `experimental_v2_selected: false` as "default is
    // v1". It must now carry the correction rather than the bare flag.
    let errata = &json(FROZEN_0_37_2_PROTOCOL)["selected_headless"]["errata_2026_09_01"];
    assert_eq!(errata["corrected_by"], "Research 270");
    assert_eq!(
        errata["actual_default_engine_at_0_37_2"],
        "agent-core-v2-run-v2-print"
    );
    assert_eq!(
        errata["routing_corpus"],
        "kimi-code-0.33.0-headless-routing"
    );
    for readme in [FROZEN_0_37_2_README, FROZEN_0_38_0_README] {
        assert!(
            readme.contains("Errata (2026-09-01, Research 270)"),
            "historical README must carry the correction"
        );
        assert!(readme.contains("`0.33.0`"));
    }
    assert!(
        !FROZEN_0_37_2_README.contains("Default headless stays off\nthe experimental v2 runner"),
        "the superseded claim must be gone, not merely annotated"
    );
}

#[test]
fn the_correction_does_not_move_the_local_server_family() {
    let claim = kimi_local_server_claim();
    assert!(claim.supports(&version("0.32.0")));
    assert!(claim.supports(&version("0.33.0")));
    assert_eq!(claim.latest_qualified().as_str(), "0.38.0");
}
