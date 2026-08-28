use serde_json::Value;

const WATCHER_SEAM: &str =
    include_str!("fixtures/claude-code-2.1.241/headless-watcher-seam.json");

#[test]
fn watcher_seam_evidence_fixture_is_secret_free_and_admits_complete_mechanism() {
    let evidence: Value =
        serde_json::from_str(WATCHER_SEAM).expect("watcher seam corpus is valid JSON");

    assert_eq!(evidence["axis"], "claude-code.headless-stream-json");
    assert_eq!(evidence["route"], "claude-code.headless");
    assert_eq!(evidence["qualified_window"], "2.1.220..=2.1.241");
    assert_eq!(evidence["provider_prompt_sent"], false);
    assert_eq!(evidence["credentials_used"], false);
    assert_eq!(evidence["host_install_changed"], false);
    assert_eq!(evidence["ambient_configuration_mutated"], false);
    assert_eq!(evidence["decision"], "complete_candidate_mechanism");
    assert_eq!(evidence["blocks_cards_010_011"], false);
    assert_eq!(evidence["empty_deliver_now_for_production_binding"], true);

    let table = evidence["mechanism_table"]
        .as_array()
        .expect("mechanism table is an array");
    assert_eq!(table.len(), 6);
    for row in table {
        let status = row["status"].as_str().expect("status");
        assert!(
            status == "admitted" || status == "admitted_docs" || status == "admitted_docs_and_package",
            "unexpected status {status}"
        );
    }

    assert_eq!(
        evidence["current_production_argv"]["mcp_config"],
        "{\"mcpServers\":{}}"
    );
    assert_eq!(evidence["current_production_argv"]["strict_mcp_config"], true);
    assert_eq!(evidence["current_production_argv"]["bare"], false);
    assert_eq!(evidence["candidate_composition"]["bare"], true);

    let same_turn = evidence["mechanism_table"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["id"] == "same-turn-reentry")
        .expect("same-turn row");
    assert_eq!(same_turn["live_observed"], false);
    assert_eq!(same_turn["anti_loop"]["consecutive_block_cap"], 8);
    assert_eq!(
        same_turn["anti_loop"]["cap_override_env"],
        "CLAUDE_CODE_STOP_HOOK_BLOCK_CAP"
    );

    assert_eq!(
        evidence["wrapper_sha256"]["2.1.220"],
        "df33087481fcf5fe9b848b3f7ae7ee6bb1b893c327b0793f052987f9c5b4eee3"
    );
    assert_eq!(
        evidence["wrapper_sha256"]["2.1.241"],
        "752252ff9a65431c356ce1ae54b7ded74a138aaa7b93148573d97ff541a2e7e6"
    );
    assert_eq!(
        evidence["darwin_arm64_binary_sha256"]["2.1.220"],
        "8addc857f3fe64d5a0368af9ee50321b50afb4a6918ba3ef018ab84f5dbbe081"
    );
    assert_eq!(
        evidence["darwin_arm64_binary_sha256"]["2.1.241"],
        "1495eb7c42d3b4451f5f1cd38b6d498d22a4a38c802bc2be5c1cf1795e64820d"
    );
}
