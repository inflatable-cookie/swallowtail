use serde_json::Value;

const CURRENTNESS: &str = include_str!("fixtures/antigravity-cli-1.3.1/headless-currentness.json");
const IDENTITY: &str = include_str!("fixtures/antigravity-cli-1.3.1/identity.json");
const DIST_INVENTORY: &str = include_str!("fixtures/antigravity-cli-1.3.1/dist-inventory.json");

const HOPS: [(&str, &str, &str, &str); 8] = [
    (
        "1.2.11",
        "1.2.12",
        "2026-09-27T05:06:09Z",
        "8cb7cd1bbab008cada5e8b27b56ed29107f45de0",
    ),
    (
        "1.2.12",
        "1.2.13",
        "2026-09-29T03:58:00Z",
        "77b1aad0cb850c184a987661b149985de41a3568",
    ),
    (
        "1.2.13",
        "1.2.14",
        "2026-09-30T04:03:27Z",
        "eaf9e06660d2ca8f20f1474ed03f10e3dbfd35e5",
    ),
    (
        "1.2.14",
        "1.2.15",
        "2026-10-02T17:00:58Z",
        "ae27bff644d78ed0c759b4d8a6b79506f1bed96d",
    ),
    (
        "1.2.15",
        "1.2.16",
        "2026-10-03T03:56:08Z",
        "65a3c69e388148c9327f307efe82ddb1c0c8d7d4",
    ),
    (
        "1.2.16",
        "1.2.17",
        "2026-10-05T06:20:11Z",
        "274d81b9929aaa2b91a7266106d0d0b7f19adf52",
    ),
    (
        "1.2.17",
        "1.3.0",
        "2026-10-06T06:05:28Z",
        "5e9c9c6c3fb1aea16dbc77918d687a842620cd1f",
    ),
    (
        "1.3.0",
        "1.3.1",
        "2026-10-07T03:22:02Z",
        "968f1170bd0e002e9d0914730975bc8a2cc65861",
    ),
];

#[test]
fn headless_currentness_record_freezes_all_hops_and_gates() {
    let currentness: Value = serde_json::from_str(CURRENTNESS).expect("currentness JSON");
    let identity: Value = serde_json::from_str(IDENTITY).expect("release identity JSON");
    let inventory: Value = serde_json::from_str(DIST_INVENTORY).expect("tree inventory JSON");

    assert_exact_keys(
        &currentness,
        &[
            "schema",
            "record",
            "scope",
            "official_stable_observed_at",
            "official_stable",
            "current_ceiling",
            "current_claim_id",
            "baseline",
            "retry_pin",
            "identity_sources",
            "static_mapping_evidence",
            "unpublished_points",
            "hops",
            "result",
        ],
    );
    assert_eq!(currentness["scope"], "antigravity.headless");
    assert_eq!(currentness["official_stable"], "1.3.1");
    assert_eq!(currentness["current_ceiling"], "1.2.11");
    assert_eq!(
        currentness["current_claim_id"],
        "antigravity.headless.release-window-2"
    );
    assert_eq!(
        currentness["retry_pin"]["name"],
        "AGY_CLI_MODEL_API_MAX_RETRIES"
    );
    assert_eq!(currentness["retry_pin"]["value"], "0");
    assert_eq!(currentness["retry_pin"]["proven_artifact"], "1.2.11");
    assert!(
        currentness["retry_pin"]["proven_newer_hops"]
            .as_array()
            .is_some_and(Vec::is_empty)
    );

    let mapping = &currentness["static_mapping_evidence"];
    assert_exact_keys(
        mapping,
        &[
            "selected_command_builder",
            "selected_pump",
            "selected_event_projection",
            "tool_error_field",
            "subagent_projection",
            "subagent_status_projection",
            "1_2_15_denial_artifact",
            "1_3_1_child_error_artifact",
        ],
    );
    assert_eq!(
        mapping["selected_command_builder"],
        "crates/swallowtail-adapter-antigravity/src/headless_command.rs::arguments"
    );
    assert_eq!(
        mapping["selected_pump"],
        "crates/swallowtail-adapter-antigravity/src/headless_pump.rs::pump_with_conversation"
    );
    assert_eq!(mapping["tool_error_field"], "/tool_info/error");
    let denial_artifact = &mapping["1_2_15_denial_artifact"];
    assert_exact_keys(
        denial_artifact,
        &[
            "platform",
            "release_archive_sha256",
            "extracted_cli_sha256",
            "selected_vendor_path",
            "denial_record_callers",
            "stdout_denial_event_proven",
        ],
    );
    assert_eq!(denial_artifact["platform"], "mac_arm64");
    assert_eq!(
        denial_artifact["release_archive_sha256"],
        inventory["versions"]["1.2.15"]["mac_arm64"]["published_digest"]
    );
    assert_eq!(
        denial_artifact["extracted_cli_sha256"],
        inventory["versions"]["1.2.15"]["mac_arm64"]["archive_members"][0]["sha256"]
    );
    assert_exact_string_array(
        &denial_artifact["selected_vendor_path"],
        &[
            "printmode.session.runTurn",
            "store.(*Manager).HeadlessDenials",
            "printmode.headlessDenialNotice",
        ],
    );
    assert_exact_string_array(
        &denial_artifact["denial_record_callers"],
        &[
            "UpdateSubagentSteps",
            "addFromDiff",
            "handleToolConfirmation",
        ],
    );
    assert_eq!(denial_artifact["stdout_denial_event_proven"], false);

    let child_error_artifact = &mapping["1_3_1_child_error_artifact"];
    assert_exact_keys(
        child_error_artifact,
        &[
            "platform",
            "release_archive_sha256",
            "extracted_cli_sha256",
            "selected_vendor_path",
            "child_error_status_field_proven",
            "disassembly_proves_success",
        ],
    );
    assert_eq!(child_error_artifact["platform"], "mac_arm64");
    assert_eq!(
        child_error_artifact["release_archive_sha256"],
        inventory["versions"]["1.3.1"]["mac_arm64"]["published_digest"]
    );
    assert_eq!(
        child_error_artifact["extracted_cli_sha256"],
        inventory["versions"]["1.3.1"]["mac_arm64"]["archive_members"][0]["sha256"]
    );
    assert_exact_string_array(
        &child_error_artifact["selected_vendor_path"],
        &[
            "PollPrintmode",
            "steps.ExtractSubagentInfo",
            "streamJSONEmitter.EmitStepUpdate",
        ],
    );
    assert_eq!(
        child_error_artifact["child_error_status_field_proven"],
        false
    );
    assert_eq!(child_error_artifact["disassembly_proves_success"], false);

    assert_exact_keys(
        &identity["official_channel"],
        &[
            "kind",
            "repository",
            "latest_tag",
            "latest_tag_commit",
            "latest_published_at",
            "latest_release_url",
            "version_command_run",
            "stable_hops_after_current_ceiling",
        ],
    );
    assert_eq!(identity["official_channel"]["latest_tag"], "1.3.1");
    assert_eq!(identity["official_channel"]["latest_tag_commit"], HOPS[7].3);
    assert_eq!(
        identity["official_channel"]["latest_published_at"],
        HOPS[7].2
    );

    let unpublished = currentness["unpublished_points"]
        .as_array()
        .expect("unpublished points");
    assert_eq!(unpublished.len(), 2);
    assert_exact_string_array(
        &Value::Array(
            unpublished
                .iter()
                .map(|point| point["version"].clone())
                .collect(),
        ),
        &["1.2.18", "1.3.2"],
    );
    for point in unpublished {
        assert_exact_keys(
            point,
            &[
                "version",
                "release_present",
                "tag_present",
                "classification",
            ],
        );
        assert_eq!(point["release_present"], false);
        assert_eq!(point["tag_present"], false);
    }

    let route_hops = currentness["hops"].as_array().expect("route hop ledger");
    let source_hops = identity["public_git_hops"]
        .as_array()
        .expect("source hop ledger");
    assert_eq!(route_hops.len(), HOPS.len());
    assert_eq!(source_hops.len(), HOPS.len());
    for (index, (from, to, published_at, commit)) in HOPS.iter().enumerate() {
        let hop = &route_hops[index];
        let source = &source_hops[index];
        assert_exact_keys(
            hop,
            &[
                "from",
                "to",
                "published_at",
                "tag_commit",
                "source_changed_paths",
                "runtime_inventory_path",
                "classification",
                "selected_change",
                "remaining_proof",
            ],
        );
        assert_exact_keys(
            source,
            &[
                "from",
                "to",
                "commit",
                "ahead_by",
                "behind_by",
                "total_commits",
                "changed_source_paths",
            ],
        );
        assert_eq!(hop["from"], *from);
        assert_eq!(hop["to"], *to);
        assert_eq!(hop["published_at"], *published_at);
        assert_eq!(hop["tag_commit"], *commit);
        assert_eq!(
            hop["runtime_inventory_path"],
            format!("dist-inventory.json#/versions/{to}")
        );
        assert_exact_string_array(&hop["source_changed_paths"], &["CHANGELOG.md"]);
        assert_eq!(source["from"], *from);
        assert_eq!(source["to"], *to);
        assert_eq!(source["commit"], *commit);
        assert_eq!(source["total_commits"], 1);
        assert_exact_string_array(&source["changed_source_paths"], &["CHANGELOG.md"]);

        let artifact = &identity["artifacts"][*to];
        assert_exact_keys(
            artifact,
            &[
                "published_at",
                "tag_commit",
                "release_url",
                "complete_published_asset_manifest",
                "downloaded_and_verified_platforms",
            ],
        );
        assert_eq!(artifact["published_at"], *published_at);
        assert_eq!(artifact["tag_commit"], *commit);
        let assets = artifact["complete_published_asset_manifest"]
            .as_array()
            .expect("complete release asset manifest");
        assert_eq!(assets.len(), 8);
        for asset in assets {
            assert_exact_keys(asset, &["name", "size", "sha256"]);
        }
        assert_exact_string_array(
            &Value::Array(assets.iter().map(|asset| asset["name"].clone()).collect()),
            &[
                "agy_cli_linux_arm64.tar.gz",
                "agy_cli_linux_arm64_musl.tar.gz",
                "agy_cli_linux_x64.tar.gz",
                "agy_cli_linux_x64_musl.tar.gz",
                "agy_cli_mac_arm64.tar.gz",
                "agy_cli_mac_x64.tar.gz",
                "agy_cli_windows_arm64.zip",
                "agy_cli_windows_x64.zip",
            ],
        );
        assert_exact_keys(
            &artifact["downloaded_and_verified_platforms"],
            &["linux_x64", "mac_arm64"],
        );
        assert!(
            !artifact["complete_published_asset_manifest"]
                .as_array()
                .expect("asset manifest")
                .is_empty()
        );

        let exact_version = &inventory["versions"][*to];
        assert_exact_keys(exact_version, &["tag_commit", "linux_x64", "mac_arm64"]);
        assert_eq!(exact_version["tag_commit"], *commit);
        assert_exact_string_array(
            &Value::Array(
                exact_version
                    .as_object()
                    .expect("runtime platforms")
                    .keys()
                    .filter(|key| key.as_str() != "tag_commit")
                    .map(|key| Value::String(key.clone()))
                    .collect(),
            ),
            &["linux_x64", "mac_arm64"],
        );
        for platform in ["linux_x64", "mac_arm64"] {
            let platform_inventory = &exact_version[platform];
            assert_exact_keys(
                platform_inventory,
                &[
                    "name",
                    "published_digest",
                    "download_sha256",
                    "size",
                    "published_digest_matched",
                    "archive_file_count",
                    "archive_members",
                ],
            );
            assert_eq!(
                platform_inventory["published_digest"],
                platform_inventory["download_sha256"]
            );
            assert_eq!(platform_inventory["published_digest_matched"], true);
            assert_eq!(platform_inventory["archive_file_count"], 1);
            assert_exact_string_array(
                &Value::Array(
                    platform_inventory["archive_members"]
                        .as_array()
                        .expect("complete archive inventory")
                        .iter()
                        .map(|member| member["path"].clone())
                        .collect(),
                ),
                &["antigravity"],
            );
            assert_eq!(
                platform_inventory["archive_members"][0]["kind"],
                "regular_file"
            );
            assert!(
                platform_inventory["archive_members"][0]["sha256"]
                    .as_str()
                    .is_some_and(|digest| digest.len() == 64)
            );
        }
    }

    assert_exact_keys(
        &currentness["result"],
        &[
            "qualification",
            "claim_changed",
            "preserve_claim_id",
            "preserve_baseline",
            "preserve_qualified_segments",
            "preserve_incompatible_hole",
            "preserve_retry_pin",
            "proposed_future_shape",
            "child_status_ruling_gate",
            "provider_prompt_sent",
            "live_provider_call",
            "credential_accessed",
            "host_install_or_update",
            "binary_executed",
        ],
    );
    assert_eq!(
        currentness["identity_sources"]["published_windows_assets_unpacked"],
        false
    );
    assert_eq!(currentness["result"]["qualification"], "blocked");
    assert_eq!(currentness["result"]["claim_changed"], false);
    assert_eq!(
        currentness["result"]["preserve_incompatible_hole"],
        "1.1.18..=1.2.10"
    );
    assert_eq!(
        currentness["result"]["preserve_retry_pin"],
        "AGY_CLI_MODEL_API_MAX_RETRIES=0"
    );
    assert_eq!(currentness["result"]["provider_prompt_sent"], false);
    assert_eq!(currentness["result"]["live_provider_call"], false);
}

fn assert_exact_keys(value: &Value, expected: &[&str]) {
    let mut actual = value
        .as_object()
        .expect("object")
        .keys()
        .map(String::as_str)
        .collect::<Vec<_>>();
    actual.sort_unstable();
    let mut expected = expected.to_vec();
    expected.sort_unstable();
    assert_eq!(actual, expected);
}

fn assert_exact_string_array(value: &Value, expected: &[&str]) {
    let actual = value.as_array().expect("string array");
    assert_eq!(actual.len(), expected.len());
    for (actual, expected) in actual.iter().zip(expected) {
        assert_eq!(actual.as_str(), Some(*expected));
    }
}
