use serde_json::Value;

const IDENTITY: &str = include_str!("fixtures/cursor-agent-2026.09.18/identity.json");
const PROTOCOL: &str = include_str!("fixtures/cursor-agent-2026.09.18/protocol.json");
const DIST_INVENTORY: &str = include_str!("fixtures/cursor-agent-2026.09.18/dist-inventory.json");

fn json(source: &str) -> Value {
    serde_json::from_str(source).expect("Cursor Agent 2026.09.18 fixture is valid JSON")
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

#[test]
fn identity_freezes_official_hops_and_preserves_claim_boundary() {
    let identity = json(IDENTITY);
    let protocol = json(PROTOCOL);

    assert_eq!(identity["axis"], "cursor-agent.release-date");
    assert_eq!(
        identity["published_stables_after_ceiling"],
        serde_json::json!(["2026.09.15-d2fe57e", "2026.09.18-9a7762b"])
    );
    assert_eq!(identity["registry"]["version"], "2026.09.18");
    assert_eq!(identity["host"]["present"], false);
    assert_eq!(
        identity["previous_ceiling"]["runtime_index_sha256"],
        "230df5356d29c26fae77aeed83bd705b2ddd5fd5d1abccd3eb8c8b8c24e23468"
    );
    assert_eq!(
        identity["previous_ceiling"]["archive_sha256"],
        "aec0b01ae056de48a02fe315fbf0580eb91377752d993307499988cbe0285423"
    );

    for hop in identity["hops"].as_array().expect("hops are an array") {
        assert!(is_sha256(
            hop["archive_sha256"]
                .as_str()
                .expect("archive digest is text")
        ));
        assert!(is_sha256(
            hop["runtime_index_sha256"]
                .as_str()
                .expect("runtime digest is text")
        ));
        assert!(is_sha256(
            hop["linux_x64_archive_sha256"]
                .as_str()
                .expect("linux archive digest is text")
        ));
        let version = hop["version"].as_str().expect("hop version is text");
        assert!(version.starts_with("2026."));
        assert_eq!(version.chars().filter(|cell| *cell == '-').count(), 1);
    }
    assert_eq!(identity["hops"][0]["version"], "2026.09.15-d2fe57e");
    assert_eq!(identity["hops"][1]["version"], "2026.09.18-9a7762b");
    assert_eq!(
        identity["hops"][1]["archive_sha256"],
        "4e67b9ac80cc4a56e0a91b3b437894e0ba489ef7ec37f120d8084d2bfd02095d"
    );

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(decision["infer_calendar_gap"], false);
    assert_eq!(decision["new_behavior_revision"], false);
    assert_eq!(decision["mix_npm_cursor_agent_axis"], false);
    assert_eq!(decision["provider_prompt_sent"], false);
    assert_eq!(decision["host_install_changed"], false);
    assert_eq!(decision["raise_latest_qualified_to"], "2026.09.18-9a7762b");
    assert_eq!(
        decision["add_exact_milestones"],
        serde_json::json!(["2026.09.15-d2fe57e", "2026.09.18-9a7762b"])
    );
    assert_eq!(
        decision["independently_unqualified_older_published"],
        serde_json::json!(["2026.08.25-3e8eec8", "2026.09.08-6caf4ff"])
    );

    assert_eq!(
        protocol["selected_cli_definitions_identical_modulo_minifier"],
        true
    );
    assert_eq!(protocol["acp_initialize_selected_subset_identical"], true);
    assert_eq!(
        protocol["acp_initialize_construction_identical_modulo_minifier"],
        false
    );
    assert_eq!(protocol["session_capabilities_subagents_unmapped"], true);
    assert_eq!(
        protocol["stream_json_event_keys_identical_modulo_minifier"],
        true
    );
    assert_eq!(protocol["selected_catalogue_command"], "models");
    assert_eq!(protocol["selected_acp_command"], "acp");
    assert_eq!(
        protocol["load_session_posture"],
        "advertised-without-proven-replay"
    );
    assert_eq!(protocol["continuation_recovery"], "blocked");
    assert_eq!(protocol["downloaded_artifacts_executed"], false);
    assert_eq!(protocol["provider_prompt_sent"], false);
    assert_eq!(protocol["authenticated_catalogue_called"], false);
    assert_eq!(
        protocol["stream_json_event_keys"],
        serde_json::json!([
            "type",
            "subtype",
            "call_id",
            "tool_call",
            "model_call_id",
            "session_id",
            "timestamp_ms"
        ])
    );
}

#[test]
fn complete_hop_inventory_has_mutation_sensitive_counts_and_hashes() {
    let inventory = json(DIST_INVENTORY);
    assert_eq!(
        inventory["compared"],
        serde_json::json!([
            "2026.09.10-fd3934a",
            "2026.09.15-d2fe57e",
            "2026.09.18-9a7762b"
        ])
    );
    assert_eq!(
        inventory["package_file_counts"],
        serde_json::json!({
            "2026.09.10-fd3934a": 443,
            "2026.09.15-d2fe57e": 444,
            "2026.09.18-9a7762b": 445
        })
    );

    let expected = [
        ("from_2026_09_10_to_2026_09_15", 62, 61, 55, 327),
        ("from_2026_09_15_to_2026_09_18", 61, 60, 77, 307),
    ];
    for (hop, added, removed, changed, identical) in expected {
        let entry = &inventory[hop];
        assert_eq!(
            entry["added"].as_array().expect("added paths").len(),
            added,
            "{hop} added"
        );
        assert_eq!(
            entry["removed"].as_array().expect("removed paths").len(),
            removed,
            "{hop} removed"
        );
        assert_eq!(
            entry["changed"].as_array().expect("changed paths").len(),
            changed,
            "{hop} changed"
        );
        assert_eq!(
            entry["identical"]
                .as_array()
                .expect("identical paths")
                .len(),
            identical,
            "{hop} identical"
        );
    }

    assert_eq!(
        inventory["identical_through_2026_09_10_2026_09_15_2026_09_18"]
            .as_array()
            .expect("identical-through paths")
            .len(),
        305
    );
    assert_eq!(
        inventory["hashes"]["runtime_index_js"]["2026.09.18-9a7762b"],
        "19af19274b07dcb8ad9bd86915320b7b49a62de763af10c6857ab45257c8a14b"
    );
    assert_eq!(
        inventory["hashes"]["package_json"]["2026.09.18-9a7762b"],
        "625a6b5d64e45fd989706ab6fd21a95f454d4e70cab7c0c1c151d37582d8bee2"
    );
    assert_eq!(
        inventory["hashes"]["launcher_cursor_agent"]["2026.09.10-fd3934a"],
        "2ccc9a8e167797641448b5e5c936f006ba137a2555f117f38c5eb76a5238a233"
    );
    assert_eq!(
        inventory["hashes"]["launcher_cursor_agent"]["2026.09.18-9a7762b"],
        "2ccc9a8e167797641448b5e5c936f006ba137a2555f117f38c5eb76a5238a233"
    );
    assert_eq!(inventory["not_a_complete_semantic_changelog"], true);
    assert!(
        inventory["named_unmapped_with_reason"]
            .as_array()
            .expect("unmapped bounds are an array")
            .len()
            >= 5
    );
}
