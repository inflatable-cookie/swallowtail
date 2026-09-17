//! Identity and stop evidence for official Qoder npm `1.1.52` (g05.070).
//!
//! This corpus is the smallest honest provider-free counterexample that keeps
//! the exact `qoder.headless` point at `1.1.25`: the selected route argv
//! `--max-turns 8` stops being inert history at `1.1.30`. The assertions here
//! are mutation-sensitive: they fail if the released tree, the selected
//! literal presence, the per-hop `--max-turns` authority classification, or
//! the unchanged production claim drifts.

use serde_json::Value;
use swallowtail_adapter_qoder::{QODER_PACKAGE_VERSION, qoder_headless_claim};
use swallowtail_core::InterfaceVersion;

const IDENTITY: &str = include_str!("fixtures/qoder-headless-1.1.52/identity.json");
const INVENTORY: &str = include_str!("fixtures/qoder-headless-1.1.52/dist-inventory.json");
const PROTOCOL: &str = include_str!("fixtures/qoder-headless-1.1.52/protocol.json");

const BASELINE: &str = "1.1.25";
const OFFICIAL: &str = "1.1.52";

fn fixture(body: &str, name: &str) -> Value {
    serde_json::from_str(body).unwrap_or_else(|error| panic!("{name}: {error}"))
}

fn hops(inventory: &Value) -> &serde_json::Map<String, Value> {
    inventory["from_hop_to_hop"]
        .as_object()
        .expect("from_hop_to_hop")
}

/// Distinct hash texts recorded for one shipped path across every hop.
fn distinct_hashes(field: &Value) -> std::collections::BTreeSet<String> {
    field
        .as_object()
        .expect("path hashes")
        .values()
        .map(|value| value.as_str().expect("hash").to_owned())
        .collect()
}

#[test]
fn identity_freezes_the_baseline_and_every_published_stable_successor() {
    let identity = fixture(IDENTITY, "identity");
    let compared = identity["compared"].as_array().expect("compared");
    assert_eq!(compared.len(), 28);
    assert_eq!(compared.first().and_then(Value::as_str), Some(BASELINE));
    assert_eq!(compared.last().and_then(Value::as_str), Some(OFFICIAL));
    assert_eq!(identity["published_stable_count"], 27);
    assert_eq!(
        identity["published_stables_from_previous_ceiling"]
            .as_array()
            .expect("successors")
            .len(),
        27
    );
    assert_eq!(identity["first_unpublished_later_stable"], "1.1.53");
    assert_eq!(identity["first_unpublished_verified"], true);
    assert_eq!(identity["npm_channel"]["latest"], OFFICIAL);
    assert_eq!(identity["npm_channel"]["beta"], "1.1.35-beta.2");
    assert_eq!(
        identity["npm_channel"]["beta_is_not_stable_authority"],
        true
    );
    assert_eq!(identity["points"].as_object().expect("points").len(), 28);

    assert_eq!(identity["baseline"]["file_count"], 33);
    assert_eq!(identity["baseline"]["unpacked_size"], 62_086_670_i64);
    assert_eq!(
        identity["baseline"]["tarball_sha256"],
        "627749221c609bfb5514f4486fb42f464597cf49472ed52c087c36a1d2fbb4ab"
    );
    assert_eq!(identity["official"]["file_count"], 28);
    assert_eq!(identity["official"]["unpacked_size"], 67_904_596_i64);

    assert_eq!(identity["axis"], "qoder.package");
    assert_eq!(identity["route"], "qoder.headless");
    assert_eq!(identity["npm_package"], "@qoder-ai/qodercli");
    assert_eq!(identity["host"]["present"], false);
    assert_eq!(identity["host"]["qodercli"], false);
    assert_eq!(identity["host"]["node"], "22.23.2");
    assert_eq!(identity["host"]["node_satisfies_engine"], true);

    let metadata = &identity["package_metadata_stable"];
    assert_eq!(metadata["bin_qodercli"], "bundle/qodercli.js");
    assert_eq!(metadata["bin_qoder"], "bundle/qoder-npm-dispatcher.cjs");
    assert_eq!(metadata["engines_node"], ">=20.0.0");
    assert_eq!(metadata["git_head"], Value::Null);
    assert_eq!(metadata["repository"], Value::Null);

    let claim = &identity["claim_at_observation"];
    assert_eq!(claim["point"], BASELINE);
    assert_eq!(claim["posture"], "QualifiedOnly");
    assert_eq!(
        claim["behavior_revision"],
        "qoder.headless.stdio-stream-json-v1"
    );
    assert_eq!(claim["claim_id"], "qoder.headless.package-window-1");
}

#[test]
fn identity_decision_is_the_typed_stop_and_carries_no_claim_change() {
    let identity = fixture(IDENTITY, "identity");
    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "stop");
    assert_eq!(decision["segment"], "stop");
    assert_eq!(decision["claim_change"], false);
    assert_eq!(decision["raise_no_range"], true);
    assert_eq!(decision["smallest_counterexample_hop"], "1.1.29..1.1.30");
    assert_eq!(decision["max_turns_argv_inert_on_baseline"], true);
    assert_eq!(decision["max_turns_argv_inert_at_official"], false);
    assert_eq!(decision["decoder_wire_shape_unchanged"], true);
    assert_eq!(decision["permission_mode_unchanged"], true);
    assert_eq!(decision["retention_unchanged"], true);
    assert_eq!(decision["cwd_and_cleanup_unchanged"], true);
    assert_eq!(decision["authentication_unchanged"], true);
    assert_eq!(
        decision["material_boundary"],
        "selected run lifecycle and bounded-limit failure semantics"
    );
    assert_eq!(decision["requires_operator_ruling"], true);
    assert!(
        decision["reason"]
            .as_str()
            .expect("reason")
            .contains("stops being historical inert history at 1.1.30")
    );

    let skill = &identity["skill_visibility"];
    assert_eq!(skill["research"], 256);
    assert_eq!(skill["deliver_now"], "empty");
    assert_eq!(skill["g05_039_g05_040_gate_remains_independent"], true);

    for field in [
        "provider_operation",
        "downloaded_artifact_executed",
        "host_install_changed",
        "credentials_used",
    ] {
        assert_eq!(identity["evidence_hygiene"][field], false, "{field}");
    }
}

#[test]
fn inventory_freezes_the_full_shipped_tree_and_consecutive_hop_deltas() {
    let inventory = fixture(INVENTORY, "inventory");
    let compared = inventory["compared"].as_array().expect("compared");
    assert_eq!(compared.len(), 28);
    assert_eq!(hops(&inventory).len(), 27);
    assert_eq!(
        inventory["path_union"]
            .as_array()
            .expect("path union")
            .len(),
        35
    );

    let counts = inventory["package_file_counts"]
        .as_object()
        .expect("file counts");
    assert_eq!(counts.len(), 28);
    assert_eq!(counts[BASELINE], 33);
    assert_eq!(counts["1.1.46"], 33);
    assert_eq!(counts["1.1.47"], 35);
    assert_eq!(counts["1.1.48"], 29);
    assert_eq!(counts["1.1.49"], 28);
    assert_eq!(counts[OFFICIAL], 28);

    let added = hops(&inventory)
        .values()
        .flat_map(|diff| diff["added"].as_array().expect("added"))
        .map(|value| value.as_str().expect("path"))
        .collect::<Vec<_>>();
    assert_eq!(
        added,
        [
            "package/bundle/vendor/qoder-security/bin/qodersec-update.cmd",
            "package/bundle/vendor/qoder-security/bin/qodersec-update.sh",
        ]
    );
    let removed = hops(&inventory)
        .values()
        .flat_map(|diff| diff["removed"].as_array().expect("removed"))
        .map(|value| value.as_str().expect("path"))
        .collect::<Vec<_>>();
    assert_eq!(
        removed,
        [
            "package/bundle/sandbox-macos-permissive-open.sb",
            "package/bundle/sandbox-macos-permissive-proxied.sb",
            "package/bundle/sandbox-macos-restrictive-open.sb",
            "package/bundle/sandbox-macos-restrictive-proxied.sb",
            "package/bundle/sandbox-macos-strict-open.sb",
            "package/bundle/sandbox-macos-strict-proxied.sb",
            "package/bundle/policies/sandbox-default.toml",
        ]
    );
}

#[test]
fn shipped_selected_files_are_hashed_per_hop_and_the_tree_refactor_is_exact() {
    let inventory = fixture(INVENTORY, "inventory");
    let hashes = inventory["hashes"].as_object().expect("hashes");

    // The CLI bundle changes at every published hop, so `1.1.25` literals can
    // never be projected onto `1.1.52`.
    let cli = hashes["package/bundle/qodercli.js"]
        .as_object()
        .expect("cli hashes");
    assert_eq!(cli.len(), 28);
    let unique_cli = distinct_hashes(&hashes["package/bundle/qodercli.js"]);
    assert_eq!(unique_cli.len(), 28);
    assert_eq!(
        cli[BASELINE],
        "77f7387974d5df79c7127bb41c9c7be8aad82aa567512ca2d9f780b2e3f73d52"
    );
    assert_eq!(
        cli[OFFICIAL],
        "ddeb47c2b4dd760b735af387a615d8946d0bc14e2455a20b75ae65409902a62b"
    );

    // The npm dispatcher is byte-identical across `1.1.25..=1.1.28` and then
    // byte-identical again across `1.1.29..=1.1.52`.
    let dispatcher = hashes["package/bundle/qoder-npm-dispatcher.cjs"]
        .as_object()
        .expect("dispatcher hashes");
    assert_eq!(
        dispatcher[BASELINE],
        "f8aabc49f26ec4a8c98302cf239d5e1a6e4c1efaccea18cdf8b2ad147de6cb6a"
    );
    assert_eq!(dispatcher["1.1.28"], dispatcher[BASELINE]);
    assert_eq!(
        dispatcher["1.1.29"],
        "37cc389f07b046d78a2c80ff8d9ab54f433d38f94bf1f221a9e7e1dea297d120"
    );
    assert_eq!(dispatcher[OFFICIAL], dispatcher["1.1.29"]);

    // `postinstall.cjs` is byte-identical across the complete chain.
    let postinstall = hashes["package/postinstall.cjs"]
        .as_object()
        .expect("postinstall hashes");
    assert_eq!(postinstall.len(), 28);
    assert_eq!(distinct_hashes(&hashes["package/postinstall.cjs"]).len(), 1);
    assert_eq!(
        postinstall[BASELINE],
        "5b9995a17600678f17b4226582ed45dce097c6f79249d9a546c7453fc5f8f220"
    );

    // `package.json` is version-stamped at every hop.
    let package_json = hashes["package/package.json"]
        .as_object()
        .expect("package.json hashes");
    assert_eq!(
        package_json[BASELINE],
        "459d820e451a6bdfd34c9799a841f2bcb66eaae155316e497cb1b12d44b53310"
    );
    assert_eq!(
        package_json[OFFICIAL],
        "cae138a1a490550553b60f2c26343de24729a197b10803faef0195158ec8996a"
    );
    assert_eq!(distinct_hashes(&hashes["package/package.json"]).len(), 28);

    // Unselected surfaces still move: the worker runtime changes at every hop
    // and the ACP proto changes at `1.1.31`. Both stay outside this route.
    let worker = hashes["package/bundle/qoder-worker-runtime.mjs"]
        .as_object()
        .expect("worker hashes");
    assert_eq!(worker.len(), 28);
    assert_eq!(
        distinct_hashes(&hashes["package/bundle/qoder-worker-runtime.mjs"]).len(),
        28
    );
    let proto = hashes["package/bundle/proto/chat.proto"]
        .as_object()
        .expect("proto hashes");
    assert_eq!(proto[BASELINE], proto["1.1.30"]);
    assert_ne!(proto["1.1.30"], proto["1.1.31"]);
    assert_eq!(proto["1.1.31"], proto[OFFICIAL]);
}

#[test]
fn selected_literals_are_present_at_every_hop() {
    let protocol = fixture(PROTOCOL, "protocol");
    let presence = protocol["selected_presence_all_hops"]
        .as_object()
        .expect("presence");
    for (literal, present) in presence {
        assert_eq!(
            present, true,
            "selected literal {literal} must hold everywhere"
        );
    }
    for required in [
        "--print",
        "--output-format",
        "stream-json",
        "--permission-mode",
        "dont_ask",
        "--max-turns",
        "--no-session-persistence",
        "--cwd",
        "--acp",
        "error_max_turns",
        "error_during_execution",
        "Operation aborted",
        "aborted_streaming",
    ] {
        assert!(
            presence.contains_key(required),
            "missing selected literal {required}"
        );
    }

    assert_eq!(protocol["artifact_revision"], OFFICIAL);
    assert_eq!(protocol["route_id"], "qoder.headless");
    assert_eq!(
        protocol["protocol_facade_revision"],
        "qoder.headless.stdio-stream-json-v1"
    );
    assert_eq!(
        protocol["selected_invocation"]["argv"],
        serde_json::json!([
            "qodercli",
            "--print",
            "--output-format",
            "stream-json",
            "--permission-mode",
            "dont_ask",
            "--max-turns",
            "8",
            "--no-session-persistence",
            "--cwd",
            "<cwd>",
            "<prompt>"
        ])
    );

    let dispatch = &protocol["mode_dispatch"];
    assert_eq!(dispatch["acp_wins_over_print"], true);
    assert_eq!(
        dispatch["sdk_requires_entrypoint_and_bidirectional_stream_json"],
        true
    );
    assert_eq!(dispatch["flatten_onto_acp"], false);
    assert_eq!(dispatch["flatten_onto_sdk_stdio"], false);
    assert_eq!(dispatch["flatten_onto_tui"], false);
    assert_eq!(protocol["init_protocol_version_decoded_by_adapter"], false);
    assert_eq!(protocol["assistant_envelope_unchanged"], true);
    assert_eq!(
        protocol["pretty_json_dump_is_not_the_streaming_decoder"],
        true
    );
}

#[test]
fn max_turns_arg_authority_is_inert_through_1_1_29_and_effective_from_1_1_30() {
    let protocol = fixture(PROTOCOL, "protocol");
    let per_hop = protocol["max_turns_authority_per_hop"]
        .as_object()
        .expect("per-hop authority");
    assert_eq!(per_hop.len(), 28);

    for hop in ["1.1.25", "1.1.26", "1.1.27", "1.1.28", "1.1.29"] {
        let row = &per_hop[hop];
        assert!(
            row["drive_query_max_turns_expression"]
                .as_str()
                .expect("expression")
                .contains("??"),
            "{hop} must keep a fixed fallback"
        );
        assert_eq!(row["drive_query_fixed_fallback_1000"], true, "{hop}");
        assert_eq!(row["drive_query_fallback_literal"], "1e3", "{hop}");
        assert_eq!(row["headless_runner_accepts_max_turns"], false, "{hop}");
        assert_eq!(
            row["headless_call_sites_forward_argv_max_turns"], false,
            "{hop}"
        );
        assert_eq!(row["cli_max_turns_has_arg_parser"], false, "{hop}");
        assert_eq!(
            row["cli_max_turns_sets_config_max_session_turns"], true,
            "{hop}"
        );
        assert_eq!(
            row["headless_session_config_uses_forwarded_max_turns"], false,
            "{hop}"
        );
        assert_eq!(row["argv_value_is_agent_loop_ceiling"], false, "{hop}");
    }
    assert_eq!(
        per_hop["1.1.25"]["drive_query_max_turns_expression"],
        "this.config.maxTurns??kN"
    );
    assert_eq!(
        per_hop["1.1.29"]["drive_query_max_turns_expression"],
        "this.config.maxTurns??HN"
    );

    for (version, row) in per_hop {
        if matches!(
            version.as_str(),
            "1.1.25" | "1.1.26" | "1.1.27" | "1.1.28" | "1.1.29"
        ) {
            continue;
        }
        assert_eq!(
            row["drive_query_max_turns_expression"], "this.config.maxTurns",
            "{version}"
        );
        assert_eq!(row["drive_query_fixed_fallback_1000"], false, "{version}");
        assert_eq!(row["headless_runner_accepts_max_turns"], true, "{version}");
        assert_eq!(
            row["headless_call_sites_forward_argv_max_turns"], true,
            "{version}"
        );
        assert_eq!(row["cli_max_turns_has_arg_parser"], true, "{version}");
        assert_eq!(
            row["cli_max_turns_sets_config_max_session_turns"], false,
            "{version}"
        );
        assert_eq!(
            row["headless_session_config_uses_forwarded_max_turns"], true,
            "{version}"
        );
        assert_eq!(row["argv_value_is_agent_loop_ceiling"], true, "{version}");
    }

    let stop = &protocol["stop_boundary"];
    assert_eq!(stop["shape"], "stop");
    assert_eq!(stop["smallest_counterexample_hop"], "1.1.29..1.1.30");
    assert_eq!(stop["last_hop_with_inert_argv_max_turns"], "1.1.29");
    assert_eq!(stop["first_hop_with_effective_argv_max_turns"], "1.1.30");
    assert_eq!(stop["claim_changed"], false);
    assert_eq!(stop["point_at_observation"], BASELINE);
    assert_eq!(stop["posture_at_observation"], "QualifiedOnly");
    assert!(
        stop["effect"]
            .as_str()
            .expect("effect")
            .contains("error_max_turns")
    );
}

#[test]
fn bounded_unmapped_surfaces_and_the_independent_skill_stop_are_recorded() {
    let protocol = fixture(PROTOCOL, "protocol");
    let classified = protocol["classified_deltas"]
        .as_object()
        .expect("classified deltas");
    assert!(
        classified["max_turns_argv_authority"]
            .as_str()
            .expect("max turns delta")
            .contains("becomes the selected AgentLoop ceiling at 1.1.30")
    );
    assert!(
        classified["init_protocol_version"]
            .as_str()
            .expect("protocol delta")
            .contains("1.2.0 -> 1.3.0")
    );
    assert!(
        classified["sandbox_policy_files"]
            .as_str()
            .expect("sandbox delta")
            .contains("unselected")
    );
    assert!(
        classified["dependency_metadata"]
            .as_str()
            .expect("dependency delta")
            .contains("@silvia-odwyer/photon-node")
    );
    assert!(
        classified["package_tree"]
            .as_str()
            .expect("tree delta")
            .contains("33 -> 35 files at 1.1.47")
    );

    let unmapped = &protocol["unmapped_boundaries"];
    let options = unmapped["cli_options"].as_array().expect("cli options");
    for option in [
        "--acp",
        "--sdk",
        "--sandbox",
        "--include-partial-messages",
        "--max-output-tokens",
    ] {
        assert!(options.iter().any(|value| value == option), "{option}");
    }

    let skill = &protocol["skill_visibility_disposition"];
    assert_eq!(skill["research"], 256);
    assert_eq!(skill["deliver_now"], "empty");
    assert_eq!(skill["package_skill_files_do_not_imply_visibility"], true);
    assert_eq!(skill["inferred_from_this_ledger"], false);
    assert_eq!(skill["g05_039_g05_040_gate_remains_independent"], true);

    let live = &protocol["live_evidence_disposition"];
    assert_eq!(live["provider_operation"], false);
    assert_eq!(live["downloaded_artifact_executed"], false);
    assert_eq!(live["host_install_changed"], false);
    assert_eq!(live["live_print_run"], false);
    assert_eq!(live["claim_change"], false);
}

#[test]
fn the_production_claim_rejects_the_historical_stop_and_binds_current_point() {
    let identity = fixture(IDENTITY, "identity");
    assert_eq!(identity["identity_decision"]["claim_change"], false);
    assert_eq!(QODER_PACKAGE_VERSION, "1.1.54");
    assert_eq!(identity["claim_at_observation"]["point"], BASELINE);

    let claim = qoder_headless_claim();
    let baseline = InterfaceVersion::new(BASELINE).expect("baseline");
    let official = InterfaceVersion::new(OFFICIAL).expect("official");
    let current = InterfaceVersion::new(QODER_PACKAGE_VERSION).expect("current");
    assert!(!claim.assess(&baseline).is_permitted());
    assert!(!claim.assess(&official).is_permitted());
    assert!(claim.assess(&current).is_permitted());
    assert!(
        !claim
            .assess(&InterfaceVersion::new("1.1.26").expect("next"))
            .is_permitted()
    );
}
