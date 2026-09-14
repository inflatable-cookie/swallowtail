use serde_json::Value;

const IDENTITY: &str = include_str!("fixtures/cursor-agent-2026.09.10/identity.json");
const PROTOCOL: &str = include_str!("fixtures/cursor-agent-2026.09.10/protocol.json");
const DIST_INVENTORY: &str = include_str!("fixtures/cursor-agent-2026.09.10/dist-inventory.json");

fn json(source: &str) -> Value {
    serde_json::from_str(source).expect("Cursor Agent 2026.09.10 fixture is valid JSON")
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
        serde_json::json!([
            "2026.08.31-4057e58",
            "2026.09.02-c22c1a3",
            "2026.09.10-fd3934a"
        ])
    );
    assert_eq!(identity["registry"]["version"], "2026.09.10");
    assert_eq!(identity["host"]["version"], "2026.08.04-aaa8809");
    assert_eq!(
        identity["host"]["runtime_index_sha256"],
        "65cb83494b6134b1b1c78139f24ac77d12943d3ba2e540d24e45eef17ee10bef"
    );
    assert_eq!(
        identity["previous_ceiling"]["runtime_index_sha256"],
        "6aceb24b7c7ecddb1993946ebb18a7dd4d025842e6efda955eb0c13255b1e5f0"
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
        let version = hop["version"].as_str().expect("hop version is text");
        assert!(version.starts_with("2026."));
        assert_eq!(version.chars().filter(|cell| *cell == '-').count(), 1);
    }
    assert_eq!(identity["hops"][2]["version"], "2026.09.10-fd3934a");
    assert_eq!(
        identity["hops"][2]["archive_sha256"],
        "aec0b01ae056de48a02fe315fbf0580eb91377752d993307499988cbe0285423"
    );

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(decision["infer_calendar_gap"], false);
    assert_eq!(decision["new_behavior_revision"], false);
    assert_eq!(decision["mix_npm_cursor_agent_axis"], false);
    assert_eq!(decision["provider_prompt_sent"], false);
    assert_eq!(decision["host_install_changed"], false);
    assert_eq!(decision["raise_latest_qualified_to"], "2026.09.10-fd3934a");
    assert_eq!(
        decision["add_exact_milestones"],
        serde_json::json!([
            "2026.08.31-4057e58",
            "2026.09.02-c22c1a3",
            "2026.09.10-fd3934a"
        ])
    );

    assert_eq!(
        protocol["selected_cli_definitions_identical_modulo_minifier"],
        true
    );
    assert_eq!(
        protocol["acp_initialize_construction_identical_modulo_minifier"],
        true
    );
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
            "2026.08.11-e8db854",
            "2026.08.31-4057e58",
            "2026.09.02-c22c1a3",
            "2026.09.10-fd3934a"
        ])
    );
    assert_eq!(
        inventory["package_file_counts"],
        serde_json::json!({
            "2026.08.11-e8db854": 434,
            "2026.08.31-4057e58": 404,
            "2026.09.02-c22c1a3": 443,
            "2026.09.10-fd3934a": 443
        })
    );

    let expected = [
        ("from_2026_08_11_to_2026_08_31", 61, 91, 76, 267),
        ("from_2026_08_31_to_2026_09_02", 97, 58, 90, 256),
        ("from_2026_09_02_to_2026_09_10", 59, 59, 78, 306),
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
        inventory["hashes"]["runtime_index_js"]["2026.09.10-fd3934a"],
        "230df5356d29c26fae77aeed83bd705b2ddd5fd5d1abccd3eb8c8b8c24e23468"
    );
    assert_eq!(
        inventory["hashes"]["package_json"]["2026.09.10-fd3934a"],
        "625a6b5d64e45fd989706ab6fd21a95f454d4e70cab7c0c1c151d37582d8bee2"
    );
    assert_eq!(
        inventory["hashes"]["launcher_cursor_agent"]["2026.08.11-e8db854"],
        "eed61c5224668c9236334c4c68936a16aecc37374b592f59e31eb50433817831"
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
