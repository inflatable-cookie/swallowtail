use serde_json::Value;
use sha2::{Digest, Sha256};
use swallowtail_adapter_cursor::{
    cursor_acp_claim, cursor_agent_release_binding, cursor_catalogue_claim, cursor_headless_claim,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

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
    let expected_hops = [
        (
            "2026.09.26-dd393fe",
            "538827d96a779bab854a865c8e42859e8e87db34b5f69770261b90fc8cfff191",
            "8085fd120f5c71f4eae7fea26a043718e5644e3071e4fab3220a0e58c51f9593",
            "f8bd1c549f844859f8aeb9f06c01420f299bee22fe136695fe891eaf18674b12",
            "5672.index.js",
            "7784c8b16d4e639814c13b12be2a687f5dc2cd98cbf29ed0a10820778ab1bf62",
            "6a7414691788dd2e514c6e30ab22b1d7851a00072afc3e2ffe982c9acbc7140f",
            177_072_900,
        ),
        (
            "2026.09.28-64d2043",
            "c0d7e9cd2e62438610b886d3439907dc1f98c2923b07b3a41416cc919aaf53c7",
            "6e4cd936a4866b8a77c50ff51a564460d715772fabc477a01aa0f0455d9559f0",
            "0d0c83f5478c3dcd806cb9697123cee3f548005fbcd3acafe31f102d659f5a7b",
            "3115.index.js",
            "290fb013b7cfeed1717aedb15e54bdf4bcfac84042f26933e083a180af51d65e",
            "6d3c3fafce1a2a86ecb0d2e84a965044f36d28530d0265b6b7cce9de6de4b0bb",
            176_795_016,
        ),
        (
            "2026.10.01-14929f9",
            "778d04e542adc5c8b6760fda3ebe0757f903b1764f2792c232ef9a35e6e2151b",
            "ba9a855f8f813c91b9f2707127572d2dc9ae5a62818e1c36719625d0fb8bd452",
            "ad1d9d915946a57ff1bf0d8364a82b91870035a09f19c502ffac9bb5b95edc5d",
            "3990.index.js",
            "c3669b3d0800ef8666adfdd8c4c469a57edeb538ac8a0dbac0b23bf5dcd84d42",
            "b70c253f7d60c4bcc5fe5d0e539865ea6663913df14c27d54b3e25612090042f",
            176_783_497,
        ),
    ];
    for (hop, expected) in hops[1..].iter().zip(expected_hops) {
        assert_eq!(hop["version"], expected.0);
        assert_eq!(hop["darwin_arm64_archive_sha256"], expected.1);
        assert_eq!(hop["linux_x64_archive_sha256"], expected.2);
        assert_eq!(hop["runtime_index_sha256"], expected.3);
        assert_eq!(hop["acp_command_chunk"], expected.4);
        assert_eq!(hop["acp_command_chunk_sha256"], expected.5);
        assert_eq!(hop["acp_sdk_chunk_sha256"], expected.6);
        assert_eq!(hop["darwin_arm64_archive_bytes"], expected.7);
    }

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
        serde_json::json!(["cursor_login"])
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
                paths.windows(2).all(|pair| {
                    pair[0].as_str().expect("path is text")
                        < pair[1].as_str().expect("path is text")
                }),
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
            .expect("identical file set is an array")
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
            .expect("selected file classifications are an array")
            .len(),
        3
    );
    let selected_paths = inventory["selected_file_classifications"]
        .as_array()
        .expect("selected file classifications are an array")
        .iter()
        .map(|hop| hop["files"].clone())
        .collect::<Vec<_>>();
    assert_eq!(
        Value::Array(selected_paths),
        serde_json::json!([
            [
                "index.js",
                "1006.index.js (removed)",
                "5672.index.js (added)",
                "8096.index.js"
            ],
            [
                "index.js",
                "5672.index.js (removed)",
                "3115.index.js (added)",
                "8096.index.js"
            ],
            [
                "index.js",
                "3115.index.js (removed)",
                "3990.index.js (added)",
                "8096.index.js"
            ]
        ])
    );
    assert_eq!(inventory["not_a_complete_semantic_changelog"], true);
}

#[test]
fn production_claim_qualifies_current_acp_hops_without_transferring_siblings() {
    let identity = json(IDENTITY);
    let acp = cursor_acp_claim();
    assert_eq!(acp.id().as_str(), "cursor-agent.acp.release-window-3");

    for hop in identity["published_stables_after_ceiling"]
        .as_array()
        .expect("published hops are an array")
    {
        let exact_build = hop.as_str().expect("published build is text");
        let binding = cursor_agent_release_binding(exact_build)
            .unwrap_or_else(|| panic!("exact official build must parse: {exact_build}"));
        let assessment = acp.assess(binding.version());
        assert!(
            matches!(
                assessment,
                InterfaceCompatibilityAssessment::Qualified(matched)
                    if matched.behavior_revision().as_str() == "cursor-agent.acp-v1.interactive-v1"
                        && matched.support_status() == InterfaceSupportStatus::Maintained
            ),
            "current selected-channel point is not qualified: {exact_build}"
        );
    }

    assert!(cursor_agent_release_binding("2026.10.01-e373342").is_none());
    let catalogue = cursor_catalogue_claim();
    let headless = cursor_headless_claim();
    for release in ["2026-09-26", "2026-09-28", "2026-10-01"] {
        assert!(
            matches!(
                catalogue.assess(&version(release)),
                InterfaceCompatibilityAssessment::Qualified(matched)
                    if matched.behavior_revision().as_str()
                        == "cursor-agent.catalogue.calendar-release-v1"
            ),
            "catalogue has its independent qualification for {release}"
        );
        let InterfaceCompatibilityAssessment::UnverifiedNewer(newer) =
            headless.assess(&version(release))
        else {
            panic!("ACP qualification must not transfer to headless {release}");
        };
        assert_eq!(newer.latest_qualified().as_str(), "2026-09-18");
    }

    for gap in [
        "2026-08-25",
        "2026-09-08",
        "2026-09-19",
        "2026-09-22",
        "2026-09-25",
        "2026-09-27",
        "2026-09-29",
        "2026-09-30",
    ] {
        assert!(!acp.permits(&version(gap)), "unqualified Cursor date {gap}");
    }

    let InterfaceCompatibilityAssessment::UnverifiedNewer(newer) =
        acp.assess(&version("2026-10-02"))
    else {
        panic!("stable after the frozen current point stays unverified newer");
    };
    assert_eq!(newer.latest_qualified().as_str(), "2026-10-01");
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("Cursor release date is a valid interface version")
}
