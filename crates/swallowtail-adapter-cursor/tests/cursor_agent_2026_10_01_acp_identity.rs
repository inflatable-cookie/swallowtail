use serde_json::Value;
use sha2::{Digest, Sha256};

const IDENTITY: &str = include_str!("fixtures/cursor-agent-acp-2026.10.01/identity.json");
const PROTOCOL: &str = include_str!("fixtures/cursor-agent-acp-2026.10.01/protocol.json");
const DIST_INVENTORY: &str =
    include_str!("fixtures/cursor-agent-acp-2026.10.01/dist-inventory.json");

fn json(source: &str) -> Value {
    serde_json::from_str(source).expect("Cursor ACP 2026.10.01 fixture is valid JSON")
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

#[test]
fn identity_freezes_all_published_acp_registry_hops_and_exact_artifacts() {
    let identity = json(IDENTITY);
    let protocol = json(PROTOCOL);

    assert_eq!(identity["axis"], "cursor-agent.release-date");
    assert_eq!(identity["selected_route"], "cursor-agent.acp");
    assert_eq!(identity["observed_at"], "2026-10-08");
    assert_eq!(identity["registry"]["id"], "cursor");
    assert_eq!(identity["registry"]["current_version"], "2026.10.01");
    assert_eq!(identity["registry"]["build_revision"], "14929f9");
    assert_eq!(
        identity["previous_claim_at_observation"]["latest_qualified"],
        "2026-09-18"
    );
    assert_eq!(
        identity["published_stables_after_ceiling"],
        serde_json::json!([
            "2026.09.26-dd393fe",
            "2026.09.28-64d2043",
            "2026.10.01-14929f9"
        ])
    );

    let hops = identity["hops"].as_array().expect("hops are an array");
    assert_eq!(hops.len(), 4);
    for hop in hops {
        for field in [
            "darwin_arm64_archive_sha256",
            "linux_x64_archive_sha256",
            "runtime_index_sha256",
            "acp_command_chunk_sha256",
            "acp_sdk_chunk_sha256",
            "launcher_sha256",
        ] {
            let digest = hop[field].as_str().expect("artifact digest is text");
            assert_eq!(digest.len(), 64, "{field}");
            assert!(digest.bytes().all(|byte| byte.is_ascii_hexdigit()));
        }
    }
    assert_eq!(hops[1]["version"], "2026.09.26-dd393fe");
    assert_eq!(hops[2]["version"], "2026.09.28-64d2043");
    assert_eq!(hops[3]["version"], "2026.10.01-14929f9");
    assert_eq!(
        hops[3]["darwin_arm64_archive_sha256"],
        "778d04e542adc5c8b6760fda3ebe0757f903b1764f2792c232ef9a35e6e2151b"
    );
    assert_eq!(
        hops[3]["linux_x64_archive_sha256"],
        "ba9a855f8f813c91b9f2707127572d2dc9ae5a62818e1c36719625d0fb8bd452"
    );

    assert_eq!(
        identity["separate_official_general_installer_channel"]["observed_build"],
        "2026.10.01-e373342"
    );
    assert_eq!(
        identity["separate_official_general_installer_channel"]["selected_for_this_route"],
        false
    );
    assert_eq!(
        identity["identity_decision"]["route_only"],
        "cursor-agent.acp"
    );
    assert_eq!(
        identity["identity_decision"]["catalogue_claim_changed"],
        false
    );
    assert_eq!(
        identity["identity_decision"]["headless_claim_changed"],
        false
    );
    assert_eq!(identity["identity_decision"]["new_public_operation"], false);
    assert_eq!(identity["identity_decision"]["provider_prompt_sent"], false);
    assert_eq!(
        identity["identity_decision"]["downloaded_artifacts_executed"],
        false
    );

    assert_eq!(protocol["selected_acp_command"], "acp");
    assert_eq!(protocol["acp_initialize_selected_subset_identical"], true);
    assert_eq!(
        protocol["acp_initialize_shape"]["auth_methods"],
        ["cursor_login"]
    );
    assert_eq!(protocol["acp_initialize_shape"]["agent_info"], Value::Null);
    assert_eq!(
        protocol["acp_initialize_shape"]["mcp_capabilities"],
        serde_json::json!({"http": true, "sse": true})
    );
    assert_eq!(
        protocol["acp_initialize_shape"]["prompt_capabilities"],
        serde_json::json!({"audio": false, "embeddedContext": false, "image": true})
    );
    assert_eq!(
        protocol["load_session_posture"],
        "advertised-without-proven-replay"
    );
    assert_eq!(protocol["continuation_recovery"], "blocked");
    assert_eq!(protocol["catalogue_or_headless_transfer"], false);
    assert_eq!(protocol["downloaded_artifacts_executed"], false);
}

#[test]
fn complete_file_sets_and_selected_surface_deltas_are_mutation_sensitive() {
    let inventory = json(DIST_INVENTORY);
    assert_eq!(
        inventory["compared"],
        serde_json::json!([
            "2026.09.18-9a7762b",
            "2026.09.26-dd393fe",
            "2026.09.28-64d2043",
            "2026.10.01-14929f9"
        ])
    );
    assert_eq!(
        inventory["package_file_counts"],
        serde_json::json!({
            "2026.09.18-9a7762b": 445,
            "2026.09.26-dd393fe": 446,
            "2026.09.28-64d2043": 447,
            "2026.10.01-14929f9": 447
        })
    );

    let expected = [
        (
            "from_2026_09_18_to_2026_09_26",
            63,
            62,
            54,
            329,
            [
                "9fff652f353e86d58a84db84e082c3b92d0b1e5a8dd098a687f81b0472ecb94d",
                "a77d8b3b61385446accaf11695a917747657815dc1a2c0039cf81fa8313310d5",
                "4c8335fbca6c79acd31612b0912d5ee92c4036097db69a17b134a9233dc26ba2",
                "bdd234afd9cfa29728dee159c7ec600bedeba9987d428ef0f5c67f31d6263620",
            ],
        ),
        (
            "from_2026_09_26_to_2026_09_28",
            65,
            64,
            82,
            300,
            [
                "41dc94e27fa98f68a583f17f89f10b25b2d8941f8abbbaeaf98aca83a15bfe1d",
                "da203c21fc0aa3fce1e397b41157b926228748d4e85121a932e08e1006b5ffdc",
                "1754c27e0d313de4127df2dcfd55be9959d925e64c180c9813ef5168f2b3336e",
                "2776c504a4a41e4b554d688f4cddaca320724f24e73322a7b728c3058c2246c2",
            ],
        ),
        (
            "from_2026_09_28_to_2026_10_01",
            60,
            60,
            80,
            307,
            [
                "4f50dbdd9d47292191007d9e6b3897f45e93c6eb4464697ee57c0255b2090b13",
                "475c05b8f64774b721686290e584daa05ff45ac45c21657e1cb4788ff5c215e8",
                "ba80eee6ec87b535315a6b17ca16cb10e0d25207622e836ad65d4f1258fe7d3f",
                "51c88698995ca9d002a499f1c09abb90d79958f0103927f8763353dac848275a",
            ],
        ),
    ];
    for (hop, added, removed, changed, identical, digests) in expected {
        let entry = &inventory[hop];
        for (name, count, digest) in [
            ("added", added, digests[0]),
            ("removed", removed, digests[1]),
            ("changed", changed, digests[2]),
            ("identical", identical, digests[3]),
        ] {
            let paths = entry[name]
                .as_array()
                .expect("complete path set is an array");
            assert_eq!(paths.len(), count, "{hop} {name} count");
            assert!(
                paths.windows(2).all(|pair| pair[0] < pair[1]),
                "{hop} {name} is sorted"
            );
            assert_eq!(
                sha256(&serde_json::to_vec(paths).expect("path set encodes")),
                digest,
                "{hop} {name} path set"
            );
            assert_eq!(
                entry["path_set_sha256"][name], digest,
                "{hop} {name} recorded digest"
            );
        }
    }

    assert_eq!(
        inventory["identical_through_all_four"]
            .as_array()
            .unwrap()
            .len(),
        299
    );
    assert_eq!(
        inventory["acp_command_chunks"]["2026.10.01-14929f9"]["path"],
        "3990.index.js"
    );
    assert_eq!(
        inventory["acp_command_chunks"]["2026.10.01-14929f9"]["sha256"],
        "c3669b3d0800ef8666adfdd8c4c469a57edeb538ac8a0dbac0b23bf5dcd84d42"
    );
    assert_eq!(
        inventory["hashes"]["runtime_index_js"]["2026.10.01-14929f9"]["sha256"],
        "ad1d9d915946a57ff1bf0d8364a82b91870035a09f19c502ffac9bb5b95edc5d"
    );
    assert_eq!(
        inventory["hashes"]["acp_sdk_8096_js"]["2026.10.01-14929f9"]["sha256"],
        "b70c253f7d60c4bcc5fe5d0e539865ea6663913df14c27d54b3e25612090042f"
    );
    assert_eq!(
        inventory["selected_file_classifications"]
            .as_array()
            .unwrap()
            .len(),
        3
    );
    assert_eq!(inventory["not_a_complete_semantic_changelog"], true);
}
