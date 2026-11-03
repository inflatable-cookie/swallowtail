use serde_json::Value;
use swallowtail_adapter_claude_agent::claude_agent_acp_claim;
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

const IDENTITY: &str = include_str!("fixtures/claude-agent-acp-0.100.0/identity.json");
const DIST_INVENTORY: &str = include_str!("fixtures/claude-agent-acp-0.100.0/dist-inventory.json");

#[test]
fn identity_0_99_1_freezes_the_middle_hop_and_the_claim_qualifies_it() {
    let identity: Value = serde_json::from_str(IDENTITY).expect("identity is valid JSON");
    let inventory: Value = serde_json::from_str(DIST_INVENTORY).expect("inventory is valid JSON");

    assert_eq!(fingerprint(IDENTITY), 0xdf65_7178_8c79_a45b);
    assert_eq!(fingerprint(DIST_INVENTORY), 0x26cd_69df_e251_dd50);

    assert_eq!(
        identity["published_stable_hops_from_previous_ceiling"],
        serde_json::json!(["0.99.0", "0.99.1", "0.100.0"])
    );
    assert_eq!(
        identity["preview_only_patch_lines"],
        serde_json::json!(["0.98.1", "0.99.2"])
    );
    assert_eq!(identity["ignored_preview_tag"], "0.100.1-preview.1");

    let artifacts = inventory["package_artifacts"].as_array().unwrap();
    let middle = artifacts
        .iter()
        .find(|a| a["version"] == "0.99.1")
        .expect("0.99.1 artifact recorded");
    assert_eq!(middle["file_count"], 288);
    assert_eq!(middle["acp_sdk"], "2.1.0");
    assert_eq!(middle["agent_sdk"], "0.3.312");
    assert_eq!(
        middle["tarball_sha256"],
        "18629662c6940585ee2344329a7f46faf9ab1419de998a6a4f94c5224ae65f5c"
    );

    let hop = &inventory["file_deltas"]
        .as_array()
        .expect("deltas are an array")[1];
    assert_eq!(hop["from"], "0.99.0");
    assert_eq!(hop["to"], "0.99.1");
    assert_eq!(hop["added"].as_array().unwrap().len(), 0);
    assert_eq!(hop["removed"].as_array().unwrap().len(), 0);
    assert_eq!(hop["changed"].as_array().unwrap().len(), 12);
    assert_eq!(hop["unchanged_file_count"], 276);
    let mut mapped_adjacent = 0;
    let mut provider_internal = 0;
    for file in hop["changed"].as_array().unwrap() {
        match file["classification"].as_str().unwrap() {
            "mapped_adjacent" => mapped_adjacent += 1,
            "provider_internal" => provider_internal += 1,
            other => panic!("unexpected file classification: {other}"),
        }
    }
    assert_eq!((mapped_adjacent, provider_internal), (7, 5));
    let package = hop["changed"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["path"] == "package.json")
        .expect("package.json changed");
    assert_eq!(package["classification"], "mapped_adjacent");
    let native = hop["changed"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["path"] == "dist/native-subagents.js")
        .expect("native-subagents.js changed");
    assert_eq!(native["classification"], "provider_internal");
    let tree = &inventory["tree_sha256"]["0.99.1"];
    assert_eq!(
        tree["dist/acp-agent.js"],
        "aa41834c24f2c09a39af0b10731cc10fb882968f75a12c3735e50a9d080b20c4"
    );
    assert_eq!(
        tree["dist/session-index/connection.js"],
        "74fb2547255707afe1b99f29b8fc04dc04ba5efeffe7f8b85139088beb8d4e28"
    );

    for version in ["0.99.1", "0.100.0"] {
        assert!(matches!(
            claude_agent_acp_claim().assess(&version_value(version)),
            InterfaceCompatibilityAssessment::Qualified(_)
        ));
    }
}

fn fingerprint(value: &str) -> u64 {
    value.bytes().fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3)
    })
}

fn version_value(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}
