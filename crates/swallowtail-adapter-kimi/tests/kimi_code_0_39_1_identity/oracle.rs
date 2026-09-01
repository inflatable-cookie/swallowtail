//! Cross-corpus oracles.
//!
//! Each mapped surface digest in `protocol.json` was taken independently from
//! three corpora — the official npm `dist/main.mjs` bundle and both extracted
//! single-executable archives — at `0.38.0`, `0.39.0`, and `0.39.1`. A
//! fabricated or self-consistently edited fixture has to survive every
//! relation below, not just look internally tidy.

use super::support::{
    AUTHORITY, FROZEN_0_38_0_HEADLESS_V2_PROTOCOL, PROTOCOL, ROUTING_PROTOCOL, json, strings,
};
use std::collections::BTreeSet;

/// Surfaces that carry exactly one digest across every corpus and version.
///
/// `dispatch_native_event` and `is_kimi_v2_enabled` are deliberately absent:
/// both move inside the admitted range and are recorded with two digests.
const HEADLESS_SURFACES: [&str; 6] = [
    "prompt_json_writer",
    "prompt_transcript_writer",
    "write_experimental_version",
    "write_resume_hint",
    "stringify_tool_output",
    "run_prompt_dispatch",
];

#[test]
fn oracle_names_three_independent_corpora_and_all_three_versions() {
    let oracle = &json(PROTOCOL)["cross_corpus_oracle"];
    assert_eq!(
        strings(&oracle["corpora"]),
        [
            "npm-dist-main-mjs",
            "github-darwin-arm64-extracted",
            "github-linux-x64-extracted"
        ]
    );
    assert_eq!(
        strings(&oracle["versions"]),
        [
            "0.32.0", "0.33.0", "0.34.0", "0.37.2", "0.38.0", "0.39.0", "0.39.1"
        ],
        "the oracle span must bracket the routing boundary on both sides"
    );
    assert_eq!(
        oracle["headless_identical_across_every_corpus_and_version"],
        true
    );
    assert_eq!(oracle["acp_identical_across_versions"], true);
}

#[test]
fn every_headless_surface_digest_is_present_distinct_and_well_formed() {
    let digests = &json(PROTOCOL)["cross_corpus_oracle"]["headless_surface_digests"];
    let mut seen = BTreeSet::new();
    for surface in HEADLESS_SURFACES {
        let value = digests[surface]
            .as_str()
            .unwrap_or_else(|| panic!("{surface} digest is text"));
        assert_eq!(value.len(), 64, "{surface} digest is sha256");
        assert!(
            value.bytes().all(|byte| byte.is_ascii_hexdigit()),
            "{surface} digest is hex"
        );
        assert!(seen.insert(value), "{surface} digest is not a copy");
    }
    assert_eq!(seen.len(), HEADLESS_SURFACES.len());
    assert_eq!(
        digests.as_object().expect("digest map").len(),
        HEADLESS_SURFACES.len(),
        "no unnamed surface is smuggled into the oracle"
    );
}

#[test]
fn both_acp_copies_are_recorded_separately_so_the_legacy_path_cannot_stand_in() {
    let oracle = &json(PROTOCOL)["cross_corpus_oracle"];
    let digests = &oracle["acp_paired_copy_digests"];
    let adapter_server = digests["acp_adapter_server_class"]
        .as_str()
        .expect("digest is text");
    let server_server = digests["acp_server_server_class"]
        .as_str()
        .expect("digest is text");
    let adapter_delta = digests["acp_adapter_assistant_delta_to_session_update"]
        .as_str()
        .expect("digest is text");
    let server_delta = digests["acp_server_assistant_delta_to_session_update"]
        .as_str()
        .expect("digest is text");
    assert_ne!(adapter_server, server_server);
    assert_ne!(adapter_delta, server_delta);
    for digest in [adapter_server, server_server, adapter_delta, server_delta] {
        assert_eq!(digest.len(), 64);
        assert!(digest.bytes().all(|byte| byte.is_ascii_hexdigit()));
    }
}

#[test]
fn the_one_differing_acp_surface_is_recorded_with_both_digests() {
    let oracle = &json(PROTOCOL)["cross_corpus_oracle"];
    let before = oracle["acp_process_service_digest_0_38_0"]
        .as_str()
        .expect("digest is text");
    let after = oracle["acp_process_service_digest_0_39_0_and_0_39_1"]
        .as_str()
        .expect("digest is text");
    assert_ne!(
        before, after,
        "the corpus must not read as a clean ACP no-op"
    );
    assert_eq!(
        oracle["acp_process_service_is_the_only_selected_acp_difference"],
        true
    );

    // The differing surface must be the one the changed-source ledger names,
    // and it must stay unmapped with a stated reachability reason.
    let changed = &json(PROTOCOL)["changed_acp_source"]["acp_server_acp_terminal_runner_ts"];
    assert_eq!(changed["mapped"], false);
    assert_eq!(changed["material"], true);
    assert_eq!(changed["reachable_under_swallowtail"], true);
    assert!(
        changed["reachable_reason"]
            .as_str()
            .expect("reason is text")
            .contains("terminal false")
    );
}

#[test]
fn headless_v2_grammar_still_matches_the_frozen_0_38_0_decoder_corpus() {
    let selected = &json(PROTOCOL)["selected_headless_v2_grammar"];
    let frozen = json(FROZEN_0_38_0_HEADLESS_V2_PROTOCOL);

    assert_eq!(strings(&selected["roles"]), strings(&frozen["jsonl_roles"]));
    assert_eq!(
        strings(&selected["meta_types"]),
        strings(&frozen["meta_types_source_proved"])
    );
    assert_eq!(
        selected["behavior_revision"],
        frozen["selected_headless_v2"]["behavior_revision"]
    );
    assert_eq!(
        selected["public_facade_id"],
        frozen["selected_headless_v2"]["public_facade_id"]
    );
    assert_eq!(
        selected["decoder_corpus"], frozen["decoder_corpus"],
        "the v2 decoder specimen does not move with the range"
    );
}

#[test]
fn the_two_moving_surfaces_are_recorded_with_both_digests() {
    let oracle = &json(PROTOCOL)["cross_corpus_oracle"];

    let gate_before = oracle["engine_gate_digests"]["through_0_32_0"]
        .as_str()
        .expect("digest is text");
    let gate_after = oracle["engine_gate_digests"]["from_0_33_0_through_0_39_1"]
        .as_str()
        .expect("digest is text");
    assert_ne!(
        gate_before, gate_after,
        "the engine gate moves; the oracle must not read as stable"
    );

    let dispatch_before = oracle["dispatch_native_event_digests"]["from_0_32_0_through_0_36_1"]
        .as_str()
        .expect("digest is text");
    let dispatch_after = oracle["dispatch_native_event_digests"]["from_0_37_0_through_0_39_1"]
        .as_str()
        .expect("digest is text");
    assert_ne!(dispatch_before, dispatch_after);
    assert_eq!(
        oracle["dispatch_native_event_digests"]["kind"],
        "typescript-retype-only"
    );
    assert_eq!(
        oracle["dispatch_native_event_digests"]["emitted_jsonl_unchanged"],
        true
    );
}

#[test]
fn the_boundary_corpus_and_the_ceiling_corpus_agree_on_every_shared_digest() {
    // Two independently written corpora record overlapping surfaces. A
    // fabricated edit to either has to be made consistently in both to pass.
    let ceiling = json(PROTOCOL);
    let routing = json(ROUTING_PROTOCOL);
    let ceiling_surfaces = &ceiling["cross_corpus_oracle"]["headless_surface_digests"];
    let routing_surfaces = &routing["v2_mapped_surface_digests"];
    let mut compared = 0;
    for (name, entry) in routing_surfaces.as_object().expect("surface map") {
        let shared = &ceiling_surfaces[name];
        if shared.is_null() {
            continue;
        }
        assert_eq!(
            shared.as_str(),
            entry["digest"].as_str(),
            "{name} disagrees between the boundary and ceiling corpora"
        );
        compared += 1;
    }
    assert!(compared >= 4, "expected real overlap, compared {compared}");

    assert_eq!(
        ceiling["cross_corpus_oracle"]["engine_gate_digests"]["through_0_32_0"],
        routing["engine_gate"]["bundle_digest_0_32_0"]
    );
    assert_eq!(
        ceiling["cross_corpus_oracle"]["engine_gate_digests"]["from_0_33_0_through_0_39_1"],
        routing["engine_gate"]["bundle_digest_0_33_0_through_0_39_1"]
    );
    assert_eq!(
        ceiling["cross_corpus_oracle"]["dispatch_native_event_digests"]["from_0_37_0_through_0_39_1"],
        routing["v2_dispatch_retype_at_0_37_0"]["digest_0_37_0_through_0_39_1"]
    );
}

#[test]
fn the_authority_corpus_and_the_ceiling_corpus_agree_on_the_acp_runner_digests() {
    let ceiling = json(PROTOCOL);
    let authority = json(AUTHORITY);
    assert_eq!(
        ceiling["cross_corpus_oracle"]["acp_process_service_digest_0_38_0"],
        authority["changed_source"]["bundle_digest_0_38_0"]
    );
    assert_eq!(
        ceiling["cross_corpus_oracle"]["acp_process_service_digest_0_39_0_and_0_39_1"],
        authority["changed_source"]["bundle_digest_0_39_0_and_0_39_1"]
    );
    assert_eq!(
        ceiling["changed_acp_source"]["acp_server_acp_terminal_runner_ts"]["from_0_38_0"],
        authority["changed_source"]["blob_0_37_2_and_0_38_0"]
    );
    assert_eq!(
        ceiling["changed_acp_source"]["acp_server_acp_terminal_runner_ts"]["at_0_39_0_and_0_39_1"],
        authority["changed_source"]["blob_0_39_0_and_0_39_1"]
    );
}
