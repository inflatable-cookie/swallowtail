use serde_json::Value;

const CURRENTNESS: &str = include_str!("fixtures/antigravity-cli-1.3.1/headless-currentness.json");
const DENIAL_EVIDENCE: &str =
    include_str!("fixtures/antigravity-cli-1.3.1/headless-denial-1.2.15-evidence.json");
const WINDOWS_EVIDENCE: &str =
    include_str!("fixtures/antigravity-cli-1.3.1/headless-windows-1.2.17-evidence.json");
const RUN_ERROR: &str = include_str!("fixtures/antigravity-cli-1.3.1/headless-run-error.jsonl");
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
            "selected_failure_scope_evidence",
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

    let identity_sources = &currentness["identity_sources"];
    assert_exact_keys(
        identity_sources,
        &[
            "release_manifest",
            "linux_and_mac_complete_platform_trees",
            "platforms_downloaded_and_verified",
            "published_windows_assets_unpacked",
            "windows_runtime_exercised",
            "binaries_executed",
        ],
    );
    assert_exact_string_array(
        &identity_sources["platforms_downloaded_and_verified"],
        &["linux_x64", "mac_arm64", "windows_x64", "windows_arm64"],
    );
    assert_eq!(identity_sources["published_windows_assets_unpacked"], true);
    assert_eq!(identity_sources["windows_runtime_exercised"], false);
    assert_eq!(identity_sources["binaries_executed"], false);

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
            "1_2_17_windows_artifact_fixture",
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
    assert_eq!(
        mapping["1_2_17_windows_artifact_fixture"],
        "headless-windows-1.2.17-evidence.json"
    );
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

    let windows_evidence: Value =
        serde_json::from_str(WINDOWS_EVIDENCE).expect("1.2.17 Windows artifact evidence");
    assert_exact_keys(
        &windows_evidence,
        &[
            "schema",
            "fixture_kind",
            "scope",
            "release",
            "platform",
            "release_commit",
            "platform_artifacts",
            "release_note",
            "static_string_matches",
            "static_string_platforms",
            "static_evidence_limits",
            "remaining_proof",
        ],
    );
    assert_eq!(windows_evidence["release"], "1.2.17");
    assert_eq!(
        windows_evidence["platform"],
        "windows_x64_and_windows_arm64"
    );
    assert_eq!(windows_evidence["release_commit"], "274d81b9929aaa2b91a7266106d0d0b7f19adf52");
    assert_exact_keys(
        &windows_evidence["platform_artifacts"],
        &["windows_x64", "windows_arm64"],
    );
    for (
        platform,
        asset_name,
        archive_digest,
        archive_size,
        executable_digest,
        executable_size,
        executable_format,
    ) in [
        (
            "windows_x64",
            "agy_cli_windows_x64.zip",
            "c31af7472fa87a8e6a95158eabc2b18cab236ef4e4cc53b9c7bbaae6e1747c16",
            57_981_858,
            "6224a54c35d31b9b85232bbfb23e3306002c418481930442519743fab585f65a",
            189_556_376,
            "PE32+ executable, x86-64, Windows console",
        ),
        (
            "windows_arm64",
            "agy_cli_windows_arm64.zip",
            "204e469b9be795dd8ab61070db3730a6c1fb29ed66995a17344401786069c32f",
            53_034_452,
            "5f7ce5bccdccb72da9c5882262a3bc433b8836ae6b88f74112d5ca14b7f5e755",
            178_700_440,
            "PE32+ executable, AArch64, Windows console",
        ),
    ] {
        let platform_evidence = &windows_evidence["platform_artifacts"][platform];
        assert_exact_keys(platform_evidence, &["release_archive", "archive_member"]);
        let release_archive = &platform_evidence["release_archive"];
        assert_exact_keys(release_archive, &["name", "url", "size", "sha256"]);
        assert_eq!(release_archive["name"], asset_name);
    assert_eq!(
        release_archive["url"],
        format!(
            "https://github.com/google-antigravity/antigravity-cli/releases/download/1.2.17/{asset_name}"
        )
    );
        assert_eq!(release_archive["size"], archive_size);
        assert_eq!(release_archive["sha256"], archive_digest);
        let published_asset = identity["artifacts"]["1.2.17"]["complete_published_asset_manifest"]
            .as_array()
            .expect("published asset manifest")
            .iter()
            .find(|asset| asset["name"] == asset_name)
            .expect("Windows asset in official manifest");
        assert_eq!(release_archive["size"], published_asset["size"]);
        assert_eq!(release_archive["sha256"], published_asset["sha256"]);
        assert_exact_keys(
            &platform_evidence["archive_member"],
            &["path", "size", "sha256", "format"],
        );
        assert_eq!(platform_evidence["archive_member"]["path"], "antigravity.exe");
        assert_eq!(platform_evidence["archive_member"]["size"], executable_size);
        assert_eq!(platform_evidence["archive_member"]["sha256"], executable_digest);
        assert_eq!(platform_evidence["archive_member"]["format"], executable_format);
    }
    assert_exact_string_array(
        &windows_evidence["static_string_matches"],
        &[
            "AGY_CLI_MODEL_API_MAX_RETRIES",
            "gemini_api_key",
            "GeminiAPIKeyAuthProvider",
            "Print mode: enabling terminal sandbox for this session",
            "Enables terminal sandbox restrictions.",
        ],
    );
    assert_exact_string_array(
        &windows_evidence["static_string_platforms"],
        &["windows_x64", "windows_arm64"],
    );
    assert_exact_keys(
        &windows_evidence["static_evidence_limits"],
        &[
            "retry_zero_semantics_proven",
            "windows_sandbox_runtime_proven",
            "binary_executed",
            "reason",
        ],
    );
    assert_eq!(
        windows_evidence["static_evidence_limits"]["retry_zero_semantics_proven"],
        false
    );
    assert_eq!(
        windows_evidence["static_evidence_limits"]["windows_sandbox_runtime_proven"],
        false
    );
    assert_eq!(windows_evidence["static_evidence_limits"]["binary_executed"], false);

    let failure_scopes = &currentness["selected_failure_scope_evidence"];
    assert_exact_keys(
        failure_scopes,
        &[
            "tool_error",
            "run_error",
            "child_error",
            "soft_permission_denial",
        ],
    );
    assert_exact_keys(
        &failure_scopes["tool_error"],
        &[
            "documented_fields",
            "documented_pointer",
            "scope",
            "projection",
            "fixture",
            "provider_capture",
        ],
    );
    assert_eq!(
        failure_scopes["tool_error"]["documented_pointer"],
        "/step_update/tool_info/error"
    );
    assert_eq!(
        failure_scopes["tool_error"]["projection"],
        "ActivityStatus::Failed"
    );
    assert_eq!(failure_scopes["tool_error"]["provider_capture"], false);
    assert_exact_string_array(
        &failure_scopes["tool_error"]["documented_fields"],
        &["type", "message"],
    );
    assert_exact_keys(
        &failure_scopes["run_error"],
        &[
            "documented_fields",
            "documented_pointer",
            "scope",
            "projection",
            "fixture",
            "provider_capture",
        ],
    );
    assert_eq!(failure_scopes["run_error"]["documented_pointer"], "/result");
    assert_eq!(
        failure_scopes["run_error"]["projection"],
        "TerminalStatus::ProviderFailed"
    );
    assert_eq!(failure_scopes["run_error"]["provider_capture"], false);
    assert_exact_string_array(
        &failure_scopes["run_error"]["documented_fields"],
        &["status", "error"],
    );
    assert_exact_keys(
        &failure_scopes["child_error"],
        &[
            "documented_container",
            "documented_child_fields",
            "status_or_error_field_documented",
            "exact_selected_error_stream_available",
            "release_note_surfaces",
            "projection_when_status_is_unavailable",
            "fixture",
            "provider_capture",
        ],
    );
    assert_eq!(
        failure_scopes["child_error"]["documented_container"],
        "/step_update/subagent_info/subagents"
    );
    assert_eq!(
        failure_scopes["child_error"]["status_or_error_field_documented"],
        false
    );
    assert_eq!(
        failure_scopes["child_error"]["exact_selected_error_stream_available"],
        false
    );
    assert_eq!(
        failure_scopes["child_error"]["projection_when_status_is_unavailable"],
        "SubagentStatus::Unknown"
    );
    assert_eq!(failure_scopes["child_error"]["fixture"], "headless-run-error.jsonl");
    assert_eq!(failure_scopes["child_error"]["provider_capture"], false);
    assert_exact_string_array(
        &failure_scopes["child_error"]["documented_child_fields"],
        &[
            "type_name",
            "role",
            "conversation_id",
            "log_uri",
            "workspace_uris",
        ],
    );
    assert_exact_string_array(
        &failure_scopes["child_error"]["release_note_surfaces"],
        &["/agents", "running-agent list"],
    );
    assert_exact_keys(
        &failure_scopes["soft_permission_denial"],
        &[
            "release",
            "run_continues",
            "exit_code",
            "notice_stream",
            "structured_denial_projection",
            "run_completion_proves_all_requested_tools_executed",
            "stderr_parsed_as_denial",
            "alternate_tool_workaround",
            "permission_bypass",
            "fixture",
            "provider_capture",
        ],
    );
    assert_eq!(failure_scopes["soft_permission_denial"]["release"], "1.2.15");
    assert_eq!(failure_scopes["soft_permission_denial"]["run_continues"], true);
    assert_eq!(failure_scopes["soft_permission_denial"]["exit_code"], 0);
    assert_eq!(failure_scopes["soft_permission_denial"]["notice_stream"], "stderr");
    assert_eq!(
        failure_scopes["soft_permission_denial"]["structured_denial_projection"],
        "unclaimed"
    );
    assert_eq!(
        failure_scopes["soft_permission_denial"]["run_completion_proves_all_requested_tools_executed"],
        false
    );
    assert_eq!(failure_scopes["soft_permission_denial"]["stderr_parsed_as_denial"], false);
    assert_eq!(failure_scopes["soft_permission_denial"]["alternate_tool_workaround"], false);
    assert_eq!(failure_scopes["soft_permission_denial"]["permission_bypass"], false);
    assert_eq!(
        failure_scopes["soft_permission_denial"]["fixture"],
        "headless-denial-1.2.15-evidence.json"
    );
    assert_eq!(failure_scopes["soft_permission_denial"]["provider_capture"], false);

    let run_error_events = RUN_ERROR
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).expect("synthetic run-error stream line"))
        .collect::<Vec<_>>();
    assert_eq!(run_error_events.len(), 3);
    let child = &run_error_events[1]["step_update"]["subagent_info"]["subagents"][0];
    assert_exact_keys(
        child,
        &[
            "type_name",
            "role",
            "conversation_id",
            "log_uri",
            "workspace_uris",
        ],
    );
    assert_eq!(run_error_events[2]["result"]["status"], "ERROR");
    assert_eq!(run_error_events[2]["result"]["error"], "fixture whole-run failure");

    let denial_evidence: Value =
        serde_json::from_str(DENIAL_EVIDENCE).expect("1.2.15 denial evidence fixture");
    assert_exact_keys(
        &denial_evidence,
        &[
            "schema",
            "fixture_kind",
            "scope",
            "release",
            "platform",
            "release_archive_sha256",
            "extracted_cli_sha256",
            "selected_vendor_path",
            "denial_record_callers",
            "official_headless_documentation",
            "selected_adapter_path",
            "exact_denial_stdout_event",
            "accepted_route_limit",
        ],
    );
    assert_eq!(denial_evidence["release"], "1.2.15");
    assert_eq!(denial_evidence["platform"], "mac_arm64");
    assert_eq!(
        denial_evidence["release_archive_sha256"],
        inventory["versions"]["1.2.15"]["mac_arm64"]["published_digest"]
    );
    assert_eq!(
        denial_evidence["extracted_cli_sha256"],
        inventory["versions"]["1.2.15"]["mac_arm64"]["archive_members"][0]["sha256"]
    );
    assert_exact_string_array(
        &denial_evidence["selected_vendor_path"],
        &[
            "printmode.session.runTurn",
            "store.(*Manager).HeadlessDenials",
            "printmode.headlessDenialNotice",
        ],
    );
    assert_exact_string_array(
        &denial_evidence["denial_record_callers"],
        &[
            "UpdateSubagentSteps",
            "addFromDiff",
            "handleToolConfirmation",
        ],
    );
    assert_exact_keys(
        &denial_evidence["official_headless_documentation"],
        &[
            "continues",
            "exit_code",
            "notice_stream",
            "notice_names_tool",
            "notice_includes_allow_guidance",
            "url",
        ],
    );
    assert_eq!(
        denial_evidence["official_headless_documentation"]["continues"],
        true
    );
    assert_eq!(
        denial_evidence["official_headless_documentation"]["exit_code"],
        0
    );
    assert_eq!(
        denial_evidence["official_headless_documentation"]["notice_stream"],
        "stderr"
    );
    assert_exact_keys(
        &denial_evidence["selected_adapter_path"],
        &[
            "output_format",
            "parses_stdout_events",
            "parses_stderr_events",
            "permission_approval_exchange",
            "dangerously_skip_permissions_argument",
        ],
    );
    assert_eq!(
        denial_evidence["selected_adapter_path"]["output_format"],
        "stream-json"
    );
    assert_eq!(
        denial_evidence["selected_adapter_path"]["parses_stderr_events"],
        false
    );
    assert_eq!(
        denial_evidence["selected_adapter_path"]["permission_approval_exchange"],
        false
    );
    assert_eq!(
        denial_evidence["selected_adapter_path"]["dangerously_skip_permissions_argument"],
        false
    );
    assert_exact_keys(
        &denial_evidence["exact_denial_stdout_event"],
        &["proven", "fixture", "reason"],
    );
    assert_eq!(
        denial_evidence["exact_denial_stdout_event"]["proven"],
        false
    );
    assert_eq!(
        denial_evidence["exact_denial_stdout_event"]["fixture"],
        Value::Null
    );
    assert_exact_keys(
        &denial_evidence["accepted_route_limit"],
        &[
            "run_completion_proves_all_requested_tools_executed",
            "structured_denial_projection",
            "stderr_notice_parsed_as_denial",
            "alternate_tool_workaround",
            "approval_bypass",
            "ruling",
        ],
    );
    assert_eq!(
        denial_evidence["accepted_route_limit"]["run_completion_proves_all_requested_tools_executed"],
        false
    );
    assert_eq!(
        denial_evidence["accepted_route_limit"]["structured_denial_projection"],
        "unclaimed"
    );
    assert_eq!(denial_evidence["accepted_route_limit"]["stderr_notice_parsed_as_denial"], false);
    assert_eq!(denial_evidence["accepted_route_limit"]["alternate_tool_workaround"], false);
    assert_eq!(denial_evidence["accepted_route_limit"]["approval_bypass"], false);

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
            "private_adapter_milestones",
            "contract_036_impact",
            "soft_denial_mapping",
            "remaining_evidence_gates",
            "provider_prompt_sent",
            "live_provider_call",
            "credential_accessed",
            "host_install_or_update",
            "binary_executed",
        ],
    );
    assert_eq!(identity_sources["published_windows_assets_unpacked"], true);
    assert_eq!(identity_sources["windows_runtime_exercised"], false);
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
    assert_exact_string_array(
        &currentness["result"]["private_adapter_milestones"],
        &[
            "preserve child identity with SubagentStatus::Unknown when selected fields contain no child status",
            "document that 1.2.15 soft denial may end with outer success while requested tool execution is not established",
        ],
    );
    assert_exact_keys(
        &currentness["result"]["contract_036_impact"],
        &["classification", "basis", "release_line"],
    );
    assert_eq!(
        currentness["result"]["contract_036_impact"]["classification"],
        "consumer-visible child-lifecycle correction; breaking under the documented weakening-lifecycle rule"
    );
    assert_eq!(
        currentness["result"]["contract_036_impact"]["basis"],
        "Contract 036 says weakening lifecycle behavior is breaking; replacing inferred Completed with Unknown changes the existing public activity projection"
    );
    assert_eq!(
        currentness["result"]["contract_036_impact"]["release_line"],
        "requires a pre-1.0 minor release if shipped; this task has no version-bump or release authority"
    );
    assert_eq!(
        currentness["result"]["soft_denial_mapping"],
        "unclaimed; do not parse unspecified stderr or invent a stdout event"
    );
    assert_exact_string_array(
        &currentness["result"]["remaining_evidence_gates"],
        &[
            "selected 1.3.0 and 1.3.1 special-path and project-custom-agent mapping",
            "Windows 1.2.17 ProviderEnforced --sandbox runtime behavior",
            "AGY_CLI_MODEL_API_MAX_RETRIES=0 semantics on affected newer artifacts",
            "approved-environment evidence for GEMINI_API_KEY absence",
        ],
    );
    assert_eq!(currentness["result"]["provider_prompt_sent"], false);
    assert_eq!(currentness["result"]["live_provider_call"], false);
    assert_eq!(currentness["result"]["credential_accessed"], false);
    assert_eq!(currentness["result"]["host_install_or_update"], false);
    assert_eq!(currentness["result"]["binary_executed"], false);
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
