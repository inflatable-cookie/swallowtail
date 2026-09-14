//! Identity and compatible-extension evidence for official Kiro `2.21.4` (g05.072).
//!
//! This corpus freezes the exact `2.18.1` baseline plus all eleven published
//! stable successors through `2.21.4`, retrieved as exact official archives
//! into `/tmp`, hashed against manifest digests, and statically inspected
//! without executing a downloaded artifact. The assertions here are
//! mutation-sensitive: they fail if the artifact ledger, the selected-surface
//! classification, or the rebind decision drifts.

use serde_json::Value;
use swallowtail_adapter_kiro::{
    KIRO_CLI_RELEASE_VERSION, kiro_acp_claim, kiro_cli_release_binding,
};
use swallowtail_core::InterfaceVersion;

const IDENTITY: &str = include_str!("fixtures/kiro-acp-2.21.4/identity.json");
const ARTIFACTS: &str = include_str!("fixtures/kiro-acp-2.21.4/artifact-ledger.json");
const SURFACE: &str = include_str!("fixtures/kiro-acp-2.21.4/surface-ledger.json");
const PROTOCOL: &str = include_str!("fixtures/kiro-acp-2.21.4/protocol.json");

const BASELINE: &str = "2.18.1";
const OFFICIAL: &str = "2.21.4";

fn fixture(body: &str, name: &str) -> Value {
    serde_json::from_str(body).unwrap_or_else(|error| panic!("{name}: {error}"))
}

#[test]
fn identity_freezes_the_baseline_and_every_published_stable_successor() {
    let identity = fixture(IDENTITY, "identity");
    assert_eq!(identity["axis"], "kiro-cli.release");
    assert_eq!(identity["route"], "kiro.acp");
    assert_eq!(identity["official"]["version"], OFFICIAL);
    assert_eq!(
        identity["official"]["selected_platform_artifacts"]["macos-universal-dmg"]["sha256"],
        "954a48e8a5796507c6733c0b67e1b4a3fea97a4b24a3bdb37c43a4e9da2f95b7"
    );
    assert_eq!(
        identity["official"]["selected_platform_artifacts"]["linux-aarch64-tar-xz"]["sha256"],
        "f582eac0e002b4d11bbd061d41fcb49d1d37626a2c1fe230373fcbf97755df6f"
    );
    assert_eq!(
        identity["official"]["install_script_sha256"],
        "91a21bfa05cd7b58601cb83e0f1f187a9d0084726e5b824d4a4cf60306250908"
    );
    assert_eq!(identity["published_stable_count"], 11);
    let compared: Vec<&str> = identity["compared"]
        .as_array()
        .expect("compared")
        .iter()
        .map(|entry| entry.as_str().expect("version"))
        .collect();
    assert_eq!(
        compared,
        vec![
            BASELINE, "2.19.0", "2.19.1", "2.19.2", "2.20.0", "2.20.1", "2.20.2", "2.21.0",
            "2.21.1", "2.21.2", "2.21.3", OFFICIAL
        ]
    );
    assert_eq!(identity["later_stable_after_official"], "none published");
    assert_eq!(identity["host"]["present"], false);

    let claim = &identity["claim_at_observation"];
    assert_eq!(claim["point"], BASELINE);
    assert_eq!(claim["posture"], "QualifiedOnly");
    assert_eq!(claim["behavior_revision"], "kiro.acp.stdio-v1");
    assert_eq!(claim["claim_id"], "kiro.acp.release-window-1");
}

#[test]
fn identity_decision_is_a_compatible_extension_without_boundary_drift() {
    let identity = fixture(IDENTITY, "identity");
    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(
        decision["entrypoint"],
        serde_json::json!(["kiro-cli", "acp"])
    );
    assert_eq!(decision["move_exact_point_to_2_21_4"], true);
    assert_eq!(decision["behavior_revision_change"], "none");
    assert_eq!(decision["flatten_onto_kiro_headless"], false);
    assert_eq!(decision["flatten_onto_cloud"], false);
    assert_eq!(decision["flatten_onto_tui_chat_binary"], false);
    assert_eq!(decision["pass_agent_flag"], false);
    assert_eq!(decision["pass_v2_or_v3_harness_flag"], false);
    assert_eq!(decision["wrap_npm_or_pypi_kiro_cli"], false);
    assert_eq!(decision["run_kiro_login"], false);
    assert_eq!(decision["bind_kiro_api_key_lease"], false);
    assert_eq!(decision["map_session_load"], false);
    assert_eq!(decision["map_session_set_model"], false);
    assert_eq!(decision["map_kiro_dev_extensions"], false);
    assert_eq!(decision["send_prompt_field_content"], false);
    assert_eq!(decision["provider_prompt_sent"], false);
    assert_eq!(decision["live_acp_initialize"], false);
    assert_eq!(decision["host_install_changed"], false);
    assert_eq!(decision["downloaded_artifact_executed"], false);
}

#[test]
fn artifact_ledger_covers_every_hop_and_reproduces_the_fixed_digests() {
    let artifacts = fixture(ARTIFACTS, "artifact-ledger");
    let points = artifacts["points"].as_object().expect("points");
    assert_eq!(points.len(), 12);
    assert_eq!(
        points[BASELINE]["archive_sha256"],
        "c1f860b63f6656501dbcf8995e2687c3e31142b95755c76ce99f8d18000017df"
    );
    assert_eq!(
        points[OFFICIAL]["archive_sha256"],
        "f582eac0e002b4d11bbd061d41fcb49d1d37626a2c1fe230373fcbf97755df6f"
    );
    assert_eq!(
        points[BASELINE]["manifest_digest_match"],
        "research_156_recorded_digest"
    );
    assert_eq!(
        points[OFFICIAL]["manifest_digest_match"],
        "g05_072_planning_recorded_digest"
    );
    assert_eq!(artifacts["shipped_tree"]["no_additions"], true);
    assert_eq!(artifacts["shipped_tree"]["no_removals"], true);
    assert_eq!(
        artifacts["shipped_tree"]["byte_identical_across_all_hops"],
        serde_json::json!([
            "kirocli/install.sh",
            "kirocli/README",
            "kirocli/bin/q",
            "kirocli/bin/qchat"
        ])
    );
    assert_eq!(
        artifacts["shipped_tree"]["changed_at_every_hop"],
        serde_json::json!([
            "kirocli/BUILD-INFO",
            "kirocli/bin/kiro-cli",
            "kirocli/bin/kiro-cli-chat",
            "kirocli/bin/kiro-cli-term"
        ])
    );
    assert_eq!(
        artifacts["build_target_triple"],
        "aarch64-unknown-linux-gnu"
    );
    assert_eq!(
        artifacts["macos_universal_dmg"][BASELINE]["sha256"],
        "07893e9477c8d296ebc653192648772fd305718da4bf3a97583718e122c47061"
    );
    assert_eq!(
        artifacts["macos_universal_dmg"][OFFICIAL]["CFBundleShortVersionString"],
        OFFICIAL
    );
    assert_eq!(artifacts["downloaded_artifact_executed"], false);
    assert_eq!(artifacts["installed_on_host"], false);
    for version in [
        BASELINE, "2.19.0", "2.19.1", "2.19.2", "2.20.0", "2.20.1", "2.20.2", "2.21.0", "2.21.1",
        "2.21.2", "2.21.3", OFFICIAL,
    ] {
        let build = &points[version]["build"];
        assert_eq!(
            build["BUILD_VERSION"].as_str().expect("build version"),
            version,
            "BUILD-INFO must carry the exact release"
        );
        assert_eq!(
            build["BUILD_TARGET_TRIPLE"].as_str().expect("triple"),
            "aarch64-unknown-linux-gnu"
        );
    }
}

#[test]
fn selected_surface_is_unchanged_across_every_hop_and_unmapped_deltas_stay_bounded() {
    let surface = fixture(SURFACE, "surface-ledger");
    let counts = surface["selected_presence_counts"]
        .as_object()
        .expect("counts");
    assert_eq!(counts.len(), 12);
    for (version, row) in counts {
        assert_eq!(
            row["session_update_kinds"].as_i64().expect("update kinds"),
            1,
            "{version}: session-update kind literals must stay present"
        );
        assert_eq!(
            row["permission_option_kinds"].as_i64().expect("kinds"),
            3,
            "{version}: permission option kinds must stay present"
        );
        assert_eq!(
            row["bridge_prompt_call"].as_i64().expect("prompt call"),
            1,
            "{version}: the bridge must keep sending field `prompt`"
        );
        assert_eq!(
            row["bridge_handshake"].as_i64().expect("handshake"),
            1,
            "{version}: the ACP handshake literal must stay present"
        );
    }
    for version in [
        BASELINE, "2.19.0", "2.19.1", "2.19.2", "2.20.0", "2.20.1", "2.21.0", "2.21.1", "2.21.2",
        "2.21.3",
    ] {
        assert_eq!(
            counts[version]["kas_method_map_new"].as_i64(),
            Some(0),
            "{version}: the pre-2.21.4 bridge chunk must still be the sorted map"
        );
    }
    assert_eq!(
        counts[OFFICIAL]["kas_method_map_new"].as_i64(),
        Some(1),
        "2.21.4 rebuilds the chat JS chunk with the new KAS client map"
    );
    assert_eq!(
        counts[OFFICIAL]["serve_method_list_with_load"].as_i64(),
        Some(1),
        "2.21.4 serves the session/load-advertising method list"
    );
    assert_eq!(
        counts[BASELINE]["serve_method_list_without_load"].as_i64(),
        Some(1),
        "the 2.18.1 baseline serves the no-load method list"
    );

    let libs = &surface["acp_libraries_stable_across_all_hops"];
    assert_eq!(libs["agent_client_protocol"], "0.10.4");
    assert_eq!(libs["sacp"], "11.0.0");
    assert_eq!(libs["set_change"], "none");

    let hops = surface["from_hop_to_hop"].as_object().expect("hops");
    assert_eq!(hops.len(), 11);
    for (name, hop) in hops {
        assert_eq!(
            hop["selected"].as_str().expect("selected"),
            "unchanged",
            "{name}: the selected ACP surface must be unchanged"
        );
        assert_eq!(
            hop["executables"].as_str().expect("executables"),
            "rebuilt",
            "{name}: binaries are rebuilt every hop"
        );
    }
    assert_eq!(
        hops["2.19.1_to_2.19.2"]["unmapped_additions"]
            .as_array()
            .expect("extension additions")
            .len(),
        1,
        "2.19.2 adds the _kiro.dev extension-request handler"
    );
    assert_eq!(
        surface["selected_method_set"],
        serde_json::json!([
            "initialize",
            "session/new",
            "session/prompt",
            "session/cancel"
        ])
    );
}

#[test]
fn protocol_keeps_the_selected_wire_and_the_stale_docs_example() {
    let protocol = fixture(PROTOCOL, "protocol");
    assert_eq!(
        protocol["selected_command"],
        serde_json::json!(["kiro-cli", "acp"])
    );
    assert_eq!(protocol["prompt"]["field"], "prompt");
    assert_eq!(
        protocol["prompt"]["docs_example_field_content_rejected"],
        true
    );
    assert_eq!(
        protocol["prompt"]["docs_content_example_still_present_at_2_21_4"],
        true
    );
    assert_eq!(
        protocol["authority"]["swallowtail_passes_agent_flag"],
        false
    );
    assert_eq!(protocol["authority"]["swallowtail_passes_cloud"], false);
    assert_eq!(protocol["authority"]["swallowtail_passes_v2_v3"], false);
    assert_eq!(protocol["permission"]["allow_always_unselected"], true);
    assert_eq!(
        protocol["permission"]["option_kinds_stable_across_all_hops"],
        serde_json::json!(["allow_once", "allow_always", "reject_once", "reject_always"])
    );
    assert_eq!(
        protocol["typed_auth_required_error_code_on_selected_paths"], false,
        "Kiro ACP introduces no typed auth_required failure on the selected paths"
    );
    assert_eq!(protocol["decoder_corpus"], "kiro-acp-2.18.1");
    assert!(
        protocol["decoder_corpus_change"]
            .as_str()
            .expect("decoder corpus change")
            .contains("unchanged"),
        "the frozen 2.18.1 decoder specimens must still cover the selected wire"
    );
    let unmapped = protocol["advertised_unmapped"]
        .as_array()
        .expect("unmapped");
    for method in [
        "session/load",
        "session/set_model",
        "_kiro.dev/commands/execute",
    ] {
        assert!(
            unmapped.iter().any(|entry| entry == method),
            "{method} must stay advertised-unmapped"
        );
    }
}

#[test]
fn production_claim_rebinds_to_exact_2_21_4_qualified_only() {
    assert_eq!(KIRO_CLI_RELEASE_VERSION, OFFICIAL);
    let claim = kiro_acp_claim();
    let official = InterfaceVersion::new(OFFICIAL).expect("official");
    let baseline = InterfaceVersion::new(BASELINE).expect("baseline");
    let newer = InterfaceVersion::new("2.21.5").expect("newer");
    assert!(claim.assess(&official).is_permitted());
    assert!(!claim.assess(&baseline).is_permitted());
    assert!(!claim.assess(&newer).is_permitted());
    assert!(kiro_cli_release_binding(OFFICIAL).is_some());
    assert!(kiro_cli_release_binding(BASELINE).is_none());
}
