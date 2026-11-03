use serde_json::Value;
use swallowtail_adapter_claude_agent::claude_agent_acp_claim;
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

const IDENTITY: &str = include_str!("fixtures/claude-agent-acp-0.100.0/identity.json");
const PROTOCOL: &str = include_str!("fixtures/claude-agent-acp-0.100.0/protocol.json");
const DIST_INVENTORY: &str = include_str!("fixtures/claude-agent-acp-0.100.0/dist-inventory.json");

#[test]
fn identity_0_99_0_freezes_the_first_hop_and_the_claim_qualifies_it() {
    let identity: Value = serde_json::from_str(IDENTITY).expect("identity is valid JSON");
    let protocol: Value = serde_json::from_str(PROTOCOL).expect("protocol is valid JSON");
    let inventory: Value = serde_json::from_str(DIST_INVENTORY).expect("inventory is valid JSON");

    assert_eq!(fingerprint(IDENTITY), 0xdf65_7178_8c79_a45b);
    assert_eq!(fingerprint(PROTOCOL), 0xfb1c_6c45_b837_906e);
    assert_eq!(fingerprint(DIST_INVENTORY), 0x26cd_69df_e251_dd50);

    assert_eq!(
        identity["official_channel_observation"]["npm_latest"],
        "0.100.0"
    );
    assert_eq!(
        identity["official_current"]["version"],
        "0.100.0"
    );
    assert_eq!(
        identity["official_current"]["github_commit"],
        "981bf9a28ac3afef26e7877a2a638814c34f1679"
    );
    assert_eq!(identity["previous_ceiling"], "0.98.0");
    assert_eq!(
        identity["previous_ceiling_tarball_sha256"],
        "94cdee902d7d4ddb0360b330c0be9f728aae3370b3028b7d835856ab33e20c8b"
    );
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "0.98.0"
    );
    assert_eq!(
        identity["identity_decision"]["extend_v7"],
        "0.66.0..=0.100.0"
    );
    assert_eq!(
        identity["identity_decision"]["later_stable_unverified"],
        "0.100.1"
    );
    assert_eq!(
        identity["inspection_boundary"]["host_binary"],
        "not-on-path-in-this-observation-environment"
    );

    let hop = &inventory["file_deltas"]
        .as_array()
        .expect("deltas are an array")[0];
    assert_eq!(hop["from"], "0.98.0");
    assert_eq!(hop["to"], "0.99.0");
    assert_eq!(hop["added"].as_array().unwrap().len(), 3);
    assert_eq!(hop["removed"].as_array().unwrap().len(), 0);
    assert_eq!(hop["changed"].as_array().unwrap().len(), 37);
    assert_eq!(hop["unchanged_file_count"], 248);
    let mut mapped_adjacent = 0;
    let mut provider_internal = 0;
    for file in hop["added"].as_array().unwrap().iter().chain(hop["changed"].as_array().unwrap()) {
        assert_keys(
            file,
            if file["before_sha256"].is_null() {
                &["path", "sha256", "classification", "reason"]
            } else {
                &[
                    "path",
                    "before_sha256",
                    "after_sha256",
                    "classification",
                    "reason",
                ]
            },
        );
        match file["classification"].as_str().unwrap() {
            "mapped_adjacent" => mapped_adjacent += 1,
            "provider_internal" => provider_internal += 1,
            other => panic!("unexpected file classification: {other}"),
        }
    }
    assert_eq!((mapped_adjacent, provider_internal), (23, 17));
    let acp_agent = hop["changed"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["path"] == "dist/acp-agent.js")
        .expect("acp-agent.js changed");
    assert_eq!(acp_agent["before_sha256"], "12c05702b3295d80a838135ee15802dd25739f8871a19eae6e664efcf3e2ed0f");
    assert_eq!(acp_agent["after_sha256"], "278e5980431a47057f7405d9a4b1889739bc8a31e1b2d138a922b4bffc865864");
    let ultracode = hop["changed"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["path"] == "dist/session-ultracode.js")
        .expect("session-ultracode.js changed");
    assert_eq!(ultracode["classification"], "mapped_adjacent");
    let blob = hop["added"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["path"] == "dist/blob-resources.js")
        .expect("blob-resources.js added");
    assert_eq!(blob["classification"], "mapped_adjacent");

    assert_eq!(
        protocol["byte_identical_selected_files_0_96_0_through_0_100_0"],
        serde_json::json!(["dist/index.js", "dist/serve.js", "dist/tools.js"])
    );
    assert_eq!(
        protocol["failure_codes"]["session_not_found_becomes_protocol_error_code_from"],
        "0.99.0"
    );
    assert_eq!(
        protocol["failure_codes"]["swallowtail_maps_every_json_rpc_error_to_one_rejected_class"],
        true
    );
    assert_eq!(
        protocol["failure_codes"]["mapped_failure_class_changed"],
        false
    );
    assert_eq!(
        protocol["configuration_and_usage"]["model_current_value_always_listed_from"],
        "0.99.0"
    );
    assert_eq!(
        protocol["tool_exchange"]["mcp_server_startup_failure_agent_message_chunk_fallback_from"],
        "0.99.0"
    );

    for version in ["0.99.0", "0.99.1", "0.100.0"] {
        assert!(matches!(
            claude_agent_acp_claim().assess(&version_value(version)),
            InterfaceCompatibilityAssessment::Qualified(_)
        ));
    }
}

fn assert_keys(value: &Value, expected: &[&str]) {
    let actual = value
        .as_object()
        .expect("expected JSON object")
        .keys()
        .map(String::as_str)
        .collect::<std::collections::BTreeSet<_>>();
    let expected = expected.iter().copied().collect::<std::collections::BTreeSet<_>>();
    assert_eq!(actual, expected);
}

fn fingerprint(value: &str) -> u64 {
    value.bytes().fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3)
    })
}

fn version_value(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}
