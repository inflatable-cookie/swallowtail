//! Research 325: Kimi Code `0.39.1..=0.43.0` installed currentness.
//!
//! The corpus in `tests/fixtures/kimi-code-0.43.0/` was frozen before any
//! claim moved. These tests tie it to the live claim so a fixture edit, a
//! silently dropped hop, or an uncontained ACP admission cannot pass.

use serde_json::Value;
use swallowtail_adapter_kimi::{
    KIMI_CODE_AXIS, KIMI_CODE_BASELINE_VERSION, KIMI_CODE_LATEST_QUALIFIED_VERSION,
    KIMI_HEADLESS_BASELINE_VERSION, KIMI_HEADLESS_LATEST_QUALIFIED_VERSION,
    KIMI_LOCAL_SERVER_BASELINE_VERSION, KIMI_LOCAL_SERVER_LATEST_QUALIFIED_VERSION, kimi_acp_claim,
    kimi_headless_claim, kimi_local_server_claim,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceNewerVersionPosture, InterfaceSupportStatus,
    InterfaceVersion,
};

const IDENTITY: &str = include_str!("fixtures/kimi-code-0.43.0/identity.json");
const PROTOCOL: &str = include_str!("fixtures/kimi-code-0.43.0/protocol.json");
const FROZEN_AUTHORITY: &str =
    include_str!("fixtures/kimi-code-0.39.0-acp-authority/identity.json");
const FROZEN_HEADLESS_V2: &str =
    include_str!("fixtures/kimi-code-0.38.0-headless-v2/protocol.json");

const POINTS: [&str; 6] = ["0.39.1", "0.40.0", "0.40.1", "0.41.0", "0.42.0", "0.43.0"];
/// The five stable hops this run classifies, oldest first.
const HOPS: [[&str; 2]; 5] = [
    ["0.39.1", "0.40.0"],
    ["0.40.0", "0.40.1"],
    ["0.40.1", "0.41.0"],
    ["0.41.0", "0.42.0"],
    ["0.42.0", "0.43.0"],
];
/// The A2 authority surface, byte-identical at every compared point.
const ACP_TERMINAL_RUNNER_BLOB: &str = "9016d48b643f35b263449d98dee25597a9a24d30";
/// The same surface as it appears in the executing npm bundle.
const ACP_PROCESS_SERVICE_DIGEST: &str =
    "7c58e045273d9dbcea96e38d0792d6122193810cd3b9a7c93bb02d40d21284e4";

fn json(text: &str) -> Value {
    serde_json::from_str(text).expect("frozen corpus JSON is valid")
}

fn text<'a>(value: &'a Value, path: &[&str]) -> &'a str {
    let mut cursor = value;
    for key in path {
        cursor = &cursor[*key];
    }
    cursor
        .as_str()
        .unwrap_or_else(|| panic!("{} is text", path.join(".")))
}

fn strings(value: &Value) -> Vec<&str> {
    value
        .as_array()
        .expect("value is an array")
        .iter()
        .map(|entry| entry.as_str().expect("array entry is text"))
        .collect()
}

fn assert_sha256(value: &Value, expected: Option<&str>) {
    let value = value.as_str().expect("digest is text");
    assert_eq!(value.len(), 64);
    assert!(value.bytes().all(|byte| byte.is_ascii_hexdigit()));
    if let Some(expected) = expected {
        assert_eq!(value, expected);
    }
}

fn assert_sha1(value: &str) -> &str {
    assert_eq!(value.len(), 40, "{value} is a git blob sha1");
    assert!(value.bytes().all(|byte| byte.is_ascii_hexdigit()));
    value
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}

#[test]
fn the_fixture_decision_is_the_shape_production_encodes() {
    let decision = &json(IDENTITY)["identity_decision"];
    assert_eq!(decision["shape"], "split");
    assert_eq!(decision["new_behavior_revision"], false);
    assert_eq!(decision["new_public_operation"], false);
    assert_eq!(decision["public_api_change"], false);

    assert_eq!(decision["acp"]["verdict"], "stop-unchanged");
    assert_eq!(
        decision["acp"]["latest_qualified_stays"],
        KIMI_CODE_LATEST_QUALIFIED_VERSION
    );
    assert_eq!(decision["acp"]["new_exclusions"], false);
    assert_eq!(
        strings(&decision["acp"]["published_gap"]),
        ["0.40.0", "0.40.1", "0.41.0", "0.42.0", "0.43.0"]
    );

    assert_eq!(decision["headless"]["verdict"], "compatible-extension");
    assert_eq!(
        decision["headless"]["raise_latest_qualified_to"],
        KIMI_HEADLESS_LATEST_QUALIFIED_VERSION
    );
    assert_eq!(decision["headless"]["raise_range_to"], "0.33.0..=0.43.0");
    assert_eq!(
        decision["headless"]["behavior_revision_stays"],
        "kimi.headless.stream-json.v2"
    );
    assert_eq!(decision["headless"]["posture_stays"], "AllowUnverified");
}

#[test]
fn every_identity_point_reproduces_an_npm_and_github_identity() {
    let identity = json(IDENTITY);
    assert_eq!(identity["npm_latest"], "0.43.0");
    assert_eq!(identity["github_latest"], "@moonshot-ai/kimi-code@0.43.0");

    let points = identity["points"].as_object().expect("points map");
    // The window is exactly the previous ceilings through the new stable, with
    // no skipped or invented hop.
    let mut names: Vec<&str> = points.keys().map(String::as_str).collect();
    names.sort_by_key(|value| {
        let mut parts = value.split('.').map(|part| part.parse::<u64>().unwrap());
        (
            parts.next().unwrap(),
            parts.next().unwrap(),
            parts.next().unwrap(),
        )
    });
    assert_eq!(names, POINTS);

    for point in POINTS {
        let entry = &points[point];
        assert!(text(entry, &["npm_integrity"]).starts_with("sha512-"));
        assert_eq!(text(entry, &["npm_shasum"]).len(), 40);
        assert_sha256(&entry["npm_tarball_sha256"], None);
        assert_sha256(&entry["npm_dist_main_mjs_sha256"], None);
        assert_eq!(text(entry, &["npm_bin_kimi"]), "dist/main.mjs");
        assert_eq!(
            text(entry, &["github_tag_name"]),
            format!("@moonshot-ai/kimi-code@{point}")
        );
        assert_eq!(text(entry, &["github_annotated_tag"]).len(), 40);
        assert_eq!(text(entry, &["github_commit"]).len(), 40);
        assert_eq!(text(entry, &["github_tree"]).len(), 40);
        assert_sha256(&entry["github_source_archive_sha256"], None);
        assert_sha256(&entry["github_darwin_arm64_zip_sha256_published"], None);
        assert_sha256(&entry["github_linux_x64_zip_sha256_published"], None);
        assert_sha256(&entry["github_manifest_json_sha256_published"], None);
    }

    // The newest stable is pinned exactly; a re-upload or a moved tag fails.
    let latest = &points["0.43.0"];
    assert_eq!(
        text(latest, &["npm_published_at"]),
        "2026-09-14T12:10:41.073Z"
    );
    assert_eq!(
        text(latest, &["npm_integrity"]),
        "sha512-J8GnyiHKxjBA1kGkPGnxXWY2rbQ+O4WwBDfiOOqKAYOpptVk568l9qp5edlmz1iBXUpCZZUyw3pPnJhM5hZl2A=="
    );
    assert_sha256(
        &latest["npm_tarball_sha256"],
        Some("225bc17f06243edf6bcf0fc82bbe8838cab1cd426eb8e467e9ebab93d53ace90"),
    );
    assert_eq!(
        text(latest, &["github_annotated_tag"]),
        "eb83293155ef86d7e5fccf8f764e9d698b619988"
    );
    assert_eq!(
        text(latest, &["github_commit"]),
        "ffa94fae854dedf594919acbea280d98cbe8e14e"
    );
}

#[test]
fn the_previous_ceiling_point_still_reproduces_research_270() {
    // 0.39.1 is the boundary this run must not silently rewrite.
    let entry = &json(IDENTITY)["points"]["0.39.1"];
    assert_sha256(
        &entry["npm_tarball_sha256"],
        Some("22594a76d0aec0cdabd41050fdd354381c106c48a2f8f5edf98394b4b5e987f7"),
    );
    assert_sha256(
        &entry["npm_dist_main_mjs_sha256"],
        Some("ed24f532d07e5d00777ae20b42ed18437e116b050bbcfc6ae0f3bc90affca337"),
    );
    assert_eq!(
        text(entry, &["github_annotated_tag"]),
        "1c142e2b20378bfdc92629abfcc68499946bf96f"
    );
    assert_eq!(
        text(entry, &["github_commit"]),
        "5efca0c3116743855c28426000073bfe34a4862f"
    );
}

#[test]
fn the_host_stays_the_observed_0_34_0_without_install_or_execution() {
    let host = &json(IDENTITY)["host"];
    assert_eq!(host["version"], "0.34.0");
    assert_eq!(
        host["sha256"],
        "9f4337e10da47843f6b550474012a53ba8b30dd665f83b176a5cd479c5f7e859"
    );
    assert_eq!(host["size"], 176_894_272);
    for guard in [
        "not_installed_by_this_run",
        "not_updated_by_this_run",
        "not_executed_by_this_run",
    ] {
        assert_eq!(host[guard], true, "{guard} holds");
    }
}

#[test]
fn acp_holds_at_0_38_0_and_grows_no_exclusion() {
    let claim = kimi_acp_claim();
    assert_eq!(claim.id().as_str(), "kimi.acp.executable-window-5");
    assert_eq!(
        claim.newer_version_posture(),
        InterfaceNewerVersionPosture::QualifiedOnly
    );
    assert_eq!(KIMI_CODE_BASELINE_VERSION, "0.28.1");
    assert_eq!(KIMI_CODE_LATEST_QUALIFIED_VERSION, "0.38.0");

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
    assert_eq!(claim.latest_qualified().as_str(), "0.38.0");

    // The recorded exclusions are exactly the two A2 points: the deny-list
    // does not grow to cover the posture-rejected published gap.
    let excluded: Vec<&str> = claim.exclusions().map(InterfaceVersion::as_str).collect();
    assert_eq!(excluded, ["0.39.0", "0.39.1"]);

    for point in ["0.40.0", "0.40.1", "0.41.0", "0.42.0", "0.43.0", "0.43.1"] {
        assert!(
            !claim.exclusions().any(|value| value.as_str() == point),
            "{point} must stay posture-rejected, not exclusion-recorded"
        );
        assert_eq!(
            claim.assess(&version(point)),
            InterfaceCompatibilityAssessment::Incompatible,
            "{point} fails closed under the unchanged A2 cap"
        );
        assert!(!claim.permits(&version(point)));
    }
}

#[test]
fn the_a2_authority_surface_is_byte_identical_at_every_point() {
    let ledger = &json(PROTOCOL)["selected_file_ledger"];
    let runner = &ledger["packages/acp-server/src/acp-terminal/acpTerminalRunner.ts"];
    for point in POINTS {
        assert_eq!(
            assert_sha1(text(runner, &[point])),
            ACP_TERMINAL_RUNNER_BLOB,
            "{point} keeps the same acpTerminalRunner source blob"
        );
    }
    // The same surface as extracted from the executing npm bundle, and the
    // digest the frozen Research 270 authority corpus already records.
    let oracle = &json(PROTOCOL)["bundle_oracle"];
    assert_eq!(
        text(oracle, &["acp_process_service_digest_all_six"]),
        ACP_PROCESS_SERVICE_DIGEST
    );
    assert_eq!(
        oracle["acp_process_service_matches_research_270_0_39_0_and_0_39_1"],
        true
    );
    assert_eq!(
        text(
            &json(FROZEN_AUTHORITY),
            &["changed_source", "bundle_digest_0_39_0_and_0_39_1"]
        ),
        ACP_PROCESS_SERVICE_DIGEST
    );
    assert_eq!(
        text(
            &json(FROZEN_AUTHORITY),
            &["changed_source", "blob_0_39_0_and_0_39_1"]
        ),
        ACP_TERMINAL_RUNNER_BLOB
    );
}

#[test]
fn the_a2_containment_gap_is_fail_closed_and_reachable() {
    let a2 = &json(PROTOCOL)["a2_containment_result"];
    assert_eq!(a2["disposition"]["shape"], "stop-unchanged");
    assert_eq!(a2["disposition"]["acp_latest_qualified_stays"], "0.38.0");
    assert_eq!(
        strings(&a2["disposition"]["exclude_exact"]),
        ["0.39.0", "0.39.1"]
    );
    assert_eq!(a2["disposition"]["new_acp_behavior_revision"], false);
    assert_eq!(a2["disposition"]["new_shared_type_required"], false);
    assert_eq!(a2["swallowtail_advertised_terminal"], false);
    assert_eq!(a2["swallowtail_advertised_auth_terminal"], false);
    assert_eq!(a2["terminal_enabled_is_always_false"], true);
    assert_eq!(a2["therefore_the_local_spawn_branch_is_always_taken"], true);
    assert_eq!(a2["containment_trace"]["adapter_control_found"], false);
    assert_eq!(a2["containment_trace"]["runtime_control_found"], false);
    assert_eq!(
        a2["containment_trace"]["harness_isolation_declared"],
        "AmbientHost"
    );
    assert!(
        text(a2, &["containment_trace", "contract_015_clause"])
            .contains("neither callback authority nor filesystem containment")
    );
    assert!(text(a2, &["containment_trace", "conclusion"]).contains("containment is absent"));
}

#[test]
fn headless_v2_extends_to_0_43_0_without_a_new_revision() {
    let claim = kimi_headless_claim();
    assert_eq!(claim.id().as_str(), "kimi.headless.executable-window-2");
    assert_eq!(
        claim.newer_version_posture(),
        InterfaceNewerVersionPosture::AllowUnverified
    );
    assert_eq!(KIMI_HEADLESS_BASELINE_VERSION, "0.29.0");
    assert_eq!(KIMI_HEADLESS_LATEST_QUALIFIED_VERSION, "0.43.0");

    let segments = claim.milestones().collect::<Vec<_>>();
    assert_eq!(segments.len(), 2);
    assert_eq!(segments[0].minimum().as_str(), "0.29.0");
    assert_eq!(segments[0].maximum().as_str(), "0.32.0");
    assert_eq!(
        segments[0].behavior_revision().as_str(),
        "kimi.headless.stream-json.v1"
    );
    assert_eq!(segments[1].minimum().as_str(), "0.33.0");
    assert_eq!(segments[1].maximum().as_str(), "0.43.0");
    assert_eq!(
        segments[1].behavior_revision().as_str(),
        "kimi.headless.stream-json.v2"
    );
    assert_eq!(claim.latest_qualified().as_str(), "0.43.0");

    // Every newly admitted point classifies v2 Maintained, and no ACP
    // exclusion leaks onto the headless axis.
    for point in ["0.39.1", "0.40.0", "0.40.1", "0.41.0", "0.42.0", "0.43.0"] {
        let InterfaceCompatibilityAssessment::Qualified(matched) = claim.assess(&version(point))
        else {
            panic!("{point} qualifies under v2");
        };
        assert_eq!(
            matched.behavior_revision().as_str(),
            "kimi.headless.stream-json.v2"
        );
        assert_eq!(matched.support_status(), InterfaceSupportStatus::Maintained);
    }

    let InterfaceCompatibilityAssessment::UnverifiedNewer(newer) = claim.assess(&version("0.43.1"))
    else {
        panic!("0.43.1 stays the first later unverified-newer point");
    };
    assert_eq!(newer.latest_qualified().as_str(), "0.43.0");
}

#[test]
fn the_headless_selected_surfaces_are_stable_at_every_point() {
    let ledger = &json(PROTOCOL)["selected_file_ledger"];
    for (path, expected) in [
        (
            "apps/kimi-code/src/cli/prompt-render.ts",
            "0e2f35238db066a13b53ad2cfff11bdff2f76724",
        ),
        (
            "apps/kimi-code/src/cli/options.ts",
            "004fd7cabdf622dba31ec8c5c3037c0b797fdb95",
        ),
    ] {
        for point in POINTS {
            assert_eq!(
                assert_sha1(text(&ledger[path], &[point])),
                expected,
                "{path} is byte-identical at {point}"
            );
        }
    }

    let surfaces = &json(PROTOCOL)["bundle_oracle"]["headless_surface_digests"];
    let names = [
        "prompt_json_writer",
        "prompt_transcript_writer",
        "write_experimental_version",
        "write_resume_hint",
        "stringify_tool_output",
        "run_prompt_dispatch",
        "dispatch_native_event",
    ];
    for name in names {
        let row = &surfaces[name];
        let distinct: std::collections::BTreeSet<&str> =
            POINTS.iter().map(|point| text(row, &[point])).collect();
        assert_eq!(
            distinct.len(),
            1,
            "{name} must carry one bundle digest across the six points"
        );
        for digest in distinct {
            assert_eq!(digest.len(), 64);
            assert!(digest.bytes().all(|byte| byte.is_ascii_hexdigit()));
        }
    }
    assert_eq!(
        text(&surfaces["dispatch_native_event"], &["0.43.0"]),
        text(&surfaces["dispatch_native_event"], &["0.39.1"])
    );
}

#[test]
fn the_selected_grammar_matches_the_frozen_0_38_0_decoder_corpus() {
    let selected = &json(PROTOCOL)["headless_selected_grammar"];
    let frozen = json(FROZEN_HEADLESS_V2);

    assert_eq!(strings(&selected["roles"]), strings(&frozen["jsonl_roles"]));
    assert_eq!(
        strings(&selected["meta_types"]),
        strings(&frozen["meta_types_source_proved"])
    );
    assert_eq!(
        text(selected, &["behavior_revision"]),
        text(&frozen, &["selected_headless_v2", "behavior_revision"])
    );
    assert_eq!(
        text(selected, &["public_facade_id"]),
        text(&frozen, &["selected_headless_v2", "public_facade_id"])
    );
    assert_eq!(
        text(selected, &["decoder_corpus"]),
        text(&frozen, &["decoder_corpus"]),
        "the v2 decoder specimen does not move with the range"
    );
    assert_eq!(
        strings(&selected["dispatch_case_labels_all_six"]),
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
fn every_hop_carries_an_independent_acp_and_headless_verdict() {
    let hops = json(PROTOCOL);
    let hops = hops["hop_classification"]
        .as_array()
        .expect("hop classification is an array");
    assert_eq!(hops.len(), HOPS.len(), "no stable hop may be skipped");

    let allowed = ["identical", "compatible", "compatible-selected-adjacent"];
    for (hop, expected) in hops.iter().zip(HOPS) {
        assert_eq!(text(hop, &["from"]), expected[0]);
        assert_eq!(text(hop, &["to"]), expected[1]);
        for claim in ["acp", "headless"] {
            let verdict = text(hop, &[claim, "verdict"]);
            assert!(
                allowed.contains(&verdict),
                "{claim} {verdict} is not a classified hop outcome"
            );
            assert!(!text(hop, &[claim, "note"]).is_empty());
            assert!(
                hop[claim]["changed_selected_files"].is_array(),
                "{claim} names its changed selected files"
            );
        }
    }

    // The empty hop must be provably empty on both axes, and the A2 surface
    // must not appear in any ACP changed-file list.
    let empty = hops
        .iter()
        .find(|hop| hop["from"] == "0.40.0")
        .expect("0.40.0 hop exists");
    assert_eq!(text(empty, &["acp", "verdict"]), "identical");
    assert_eq!(text(empty, &["headless", "verdict"]), "identical");
    assert!(strings(&empty["acp"]["changed_selected_files"]).is_empty());
    assert!(strings(&empty["headless"]["changed_selected_files"]).is_empty());

    for hop in hops {
        let files = strings(&hop["acp"]["changed_selected_files"]);
        assert!(
            !files.iter().any(|path| path.contains("acpTerminalRunner")),
            "the A2 authority surface must not move inside the window"
        );
    }
}

#[test]
fn the_local_server_family_does_not_move_with_the_installed_axes() {
    assert_eq!(KIMI_LOCAL_SERVER_BASELINE_VERSION, "0.28.1");
    // Research 326 moved the separate local-server family on its own
    // authority evidence: 0.39.1 QualifiedOnly ceiling, not the installed
    // headless 0.43.0 ceiling.
    assert_eq!(KIMI_LOCAL_SERVER_LATEST_QUALIFIED_VERSION, "0.39.1");
    let claim = kimi_local_server_claim();
    assert!(matches!(
        claim.assess(&version("0.39.1")),
        InterfaceCompatibilityAssessment::Qualified(_)
    ));
    assert_eq!(
        claim.assess(&version("0.40.0")),
        InterfaceCompatibilityAssessment::Incompatible
    );
    assert_eq!(
        json(IDENTITY)["identity_decision"]["widen_local_server_claim"],
        false
    );
    assert_eq!(
        json(IDENTITY)["identity_decision"]["touch_g05_009_card_034"],
        false
    );
    assert!(json(PROTOCOL)["separate_family_observations"]["kimi_code_local_server"].is_string());
}

#[test]
fn no_provider_or_execution_guard_was_flipped() {
    let guards = &json(PROTOCOL)["guards"];
    for guard in [
        "provider_prompt_sent",
        "authentication_performed",
        "live_probe_run",
        "local_server_started",
        "host_install_changed",
        "downloaded_binaries_executed",
        "contracts_edited",
        "local_server_claim_edited",
        "published_binary_executed",
    ] {
        assert_eq!(guards[guard], false, "{guard} stays false");
    }
    let decision = &json(IDENTITY)["identity_decision"];
    for guard in [
        "provider_prompt_sent",
        "authentication_performed",
        "live_probe_run",
        "local_server_started",
        "host_install_changed",
        "downloaded_binaries_executed",
    ] {
        assert_eq!(decision[guard], false, "{guard} stays false");
    }
    // The route family is Kimi Code installed, not the Python or hosted axes.
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], KIMI_CODE_AXIS);
}
