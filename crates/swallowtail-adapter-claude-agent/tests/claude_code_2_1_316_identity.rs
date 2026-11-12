use serde_json::{Value, json};
use swallowtail_adapter_claude_agent::{
    CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION, CLAUDE_CODE_RESPONSE_ONLY_DENIED_VERSIONS,
    CLAUDE_CODE_RESPONSE_ONLY_LATEST_QUALIFIED_VERSION, claude_code_headless_claim,
    claude_code_response_only_claim,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const HEADLESS_IDENTITY: &str = include_str!("fixtures/claude-code-2.1.316/headless-identity.json");
const RESPONSE_IDENTITY: &str =
    include_str!("fixtures/claude-code-2.1.316/response-only-identity.json");
const INVENTORY: &str = include_str!("fixtures/claude-code-2.1.316/dist-inventory.json");
const MARKERS: &str = include_str!("fixtures/claude-code-2.1.316/marker-ledger.json");
const CONTROL_FLOW: &str = include_str!("fixtures/claude-code-2.1.316/selected-control-flow.json");
const ISOLATION: &str = include_str!("fixtures/claude-code-2.1.316/response-only-isolation.json");
const HEADLESS_PROTOCOL: &str = include_str!("fixtures/claude-code-2.1.316/headless-protocol.json");
const RESPONSE_PROTOCOL: &str =
    include_str!("fixtures/claude-code-2.1.316/response-only-protocol.json");
const FORKED_SURFACE: &str =
    include_str!("fixtures/claude-code-2.1.316/forked-stream-surface.json");

const VERSIONS: [&str; 4] = ["2.1.312", "2.1.314", "2.1.315", "2.1.316"];
const PLATFORMS: [&str; 2] = ["claude-code-darwin-arm64", "claude-code-linux-x64"];

fn fixture(value: &str) -> Value {
    serde_json::from_str(value).expect("frozen JSON is valid")
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("version")
}

fn function_name(snippet: &str) -> &str {
    let rest = snippet
        .strip_prefix("function ")
        .unwrap_or_else(|| panic!("selected snippet is a function: {snippet}"));
    rest.split('(').next().expect("function name")
}

#[test]
fn official_stable_is_2_1_316_and_the_two_claims_stay_separate() {
    let headless = fixture(HEADLESS_IDENTITY);
    let response = fixture(RESPONSE_IDENTITY);
    assert_ne!(headless["axis"], response["axis"]);
    assert_eq!(headless["axis"], "claude-code.headless-stream-json");
    assert_eq!(response["axis"], "claude-code.response-only-stream-json");
    for identity in [&headless, &response] {
        assert_eq!(identity["official_latest_at_observation"], "2.1.316");
        assert_eq!(identity["previous_ceiling"], "2.1.312");
        assert_eq!(
            identity["npm"]["dist_tags"],
            json!({"stable": "2.1.309", "next": "2.1.317", "latest": "2.1.316"})
        );
        assert_eq!(identity["npm"]["stable_dist_tag_is_delayed_channel"], true);
        assert_eq!(
            identity["npm"]["published"]["2.1.314"],
            "2026-10-30T00:12:56.911Z"
        );
        assert_eq!(
            identity["npm"]["published"]["2.1.315"],
            "2026-10-30T21:05:24.080Z"
        );
        assert_eq!(
            identity["npm"]["published"]["2.1.316"],
            "2026-10-31T00:42:05.743Z"
        );
        assert_eq!(identity["github"]["latest_non_prerelease"], "v2.1.316");
        assert_eq!(
            identity["github"]["tag_commits"]["2.1.316"],
            "80c6959d28c4c30ecd14fc1cdf172f19d9adad81"
        );
        assert_eq!(
            identity["github"]["tag_commits"]["2.1.314"],
            "b3f291d131924810e2b7713ededc144843a229d5"
        );
        assert!(identity["github"]["tag_commits"]["2.1.315"].is_null());
        assert_eq!(
            identity["github"]["tags_or_releases_absent"],
            json!(["2.1.313", "2.1.315"])
        );
        assert_eq!(
            identity["published_hops_after_previous_ceiling"],
            json!(["2.1.314", "2.1.315", "2.1.316"])
        );
        assert_eq!(identity["unpublished_between"], json!(["2.1.313"]));
        assert_eq!(identity["first_unpublished_after_latest"], "2.1.318");
        assert_eq!(identity["npm_2_1_313_http_status"], 404);
        assert_eq!(identity["npm_2_1_318_http_status"], 404);
        assert_eq!(identity["github_tag_v2_1_313_present"], false);
        assert_eq!(identity["github_tag_v2_1_315_present"], false);
        assert_eq!(identity["github_tag_v2_1_318_present"], false);
        let later = &identity["later_stable_outside_official_channel"]["2.1.317"];
        assert_eq!(later["npm_dist_tags"], json!(["next"]));
        assert_eq!(later["published_at"], "2026-11-02T23:57:53.069Z");
        assert_eq!(later["github_tag_present"], false);
        assert_eq!(later["official_stable"], false);
        assert_eq!(later["posture"], "UnverifiedNewer");
        assert_eq!(identity["host"]["present"], true);
        assert_eq!(identity["host"]["version"], "2.1.314");
        assert_eq!(
            identity["host"]["artifact_sha256"],
            "21d6e7b9a6f07b4cfcb7df13760b3b72aa25e084e13a5354413382987ba832ed"
        );
        assert_eq!(
            identity["host"]["owned_by"],
            "claude-code-darwin-arm64@2.1.314"
        );
        assert_eq!(identity["host"]["version_probe_only"], true);
        assert_eq!(identity["host"]["provider_work_executed"], false);
        assert_eq!(identity["identity_decision"], "compatible-extension");
        assert_eq!(identity["downloaded_artifacts_executed"], false);
        assert_eq!(identity["provider_prompt_sent"], false);
        assert_eq!(identity["live_session"], false);
        assert_eq!(identity["host_install_changed"], false);
        assert_eq!(identity["channels_agree_on_official_stable"], true);
        assert_eq!(
            identity["claim_at_observation"]["latest_qualified"],
            "2.1.312"
        );
        assert_eq!(identity["after_claim"]["latest_qualified"], "2.1.316");
        assert_eq!(identity["after_claim"]["first_unverified_newer"], "2.1.317");
    }
    let gaps = json!([
        "2.1.244", "2.1.249", "2.1.253", "2.1.254", "2.1.255", "2.1.256", "2.1.262", "2.1.264",
        "2.1.279", "2.1.297", "2.1.298", "2.1.313"
    ]);
    assert_eq!(headless["after_claim"]["unpublished_exclusions"], gaps);
    assert_eq!(response["after_claim"]["denied_versions"], gaps);
    assert_eq!(
        headless["claim_at_observation"]["claim_id"],
        "claude-code.headless.window-1"
    );
    assert_eq!(
        headless["after_claim"]["maintained_segment"],
        "2.1.287..=2.1.316"
    );
    assert_eq!(
        headless["after_claim"]["maintained_behavior_revision"],
        "claude-code.headless.stream-json.v2"
    );
    assert_eq!(
        headless["after_claim"]["deprecated_segment_unchanged"],
        "2.1.220..=2.1.286"
    );
    assert_eq!(headless["after_claim"]["maximum_turns_extended"], false);
    assert_eq!(
        response["claim_at_observation"]["claim_id"],
        "claude-code.response-only.window-1"
    );
    assert_eq!(
        response["after_claim"]["maintained_behavior_revision"],
        "claude-code.response-only.stream-json.v3"
    );
    assert_eq!(
        response["after_claim"]["maintained_segment"],
        "2.1.282..=2.1.316"
    );
    assert_eq!(response["after_claim"]["earlier_segments_unchanged"], true);
    assert_ne!(headless["axis"], "claude-agent.acp-adapter");
    assert_ne!(response["axis"], "claude-agent.sdk.package");
}

#[test]
fn both_claims_qualify_every_published_hop_through_2_1_316_on_the_existing_revisions() {
    assert_eq!(CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION, "2.1.317");
    assert_eq!(
        CLAUDE_CODE_RESPONSE_ONLY_LATEST_QUALIFIED_VERSION,
        "2.1.319"
    );
    assert_eq!(
        CLAUDE_CODE_RESPONSE_ONLY_DENIED_VERSIONS,
        &[
            "2.1.244", "2.1.249", "2.1.253", "2.1.254", "2.1.255", "2.1.256", "2.1.262", "2.1.264",
            "2.1.279", "2.1.297", "2.1.298", "2.1.313", "2.1.320",
        ]
    );
    let headless = claude_code_headless_claim();
    let response = claude_code_response_only_claim();
    assert_ne!(headless.axis(), response.axis());
    assert_eq!(headless.latest_qualified().as_str(), "2.1.317");
    assert_eq!(response.latest_qualified().as_str(), "2.1.319");
    for point in ["2.1.312", "2.1.314", "2.1.315", "2.1.316"] {
        assert!(
            matches!(
                headless.assess(&version(point)),
                InterfaceCompatibilityAssessment::Qualified(matched)
                    if matched.behavior_revision().as_str()
                        == "claude-code.headless.stream-json.v2"
                        && matched.support_status() == InterfaceSupportStatus::Maintained
            ),
            "{point}"
        );
        assert!(
            matches!(
                response.assess(&version(point)),
                InterfaceCompatibilityAssessment::Qualified(matched)
                    if matched.behavior_revision().as_str()
                        == "claude-code.response-only.stream-json.v3"
                        && matched.support_status() == InterfaceSupportStatus::Maintained
            ),
            "{point}"
        );
    }
    for gap in ["2.1.244", "2.1.279", "2.1.297", "2.1.298", "2.1.313"] {
        assert!(!headless.permits(&version(gap)), "{gap}");
        assert!(!response.permits(&version(gap)), "{gap}");
    }
    assert!(matches!(
        headless.assess(&version("2.1.286")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.behavior_revision().as_str() == "claude-code.headless.stream-json.v1"
                && matched.support_status() == InterfaceSupportStatus::Deprecated
    ));
    assert!(matches!(
        response.assess(&version("2.1.278")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.behavior_revision().as_str() == "claude-code.response-only.stream-json.v1"
                && matched.support_status() == InterfaceSupportStatus::Deprecated
    ));
    for newer_point in ["2.1.318", "2.1.322"] {
        assert!(
            matches!(
                headless.assess(&version(newer_point)),
                InterfaceCompatibilityAssessment::UnverifiedNewer(newer)
                    if newer.behavior_revision().as_str()
                        == "claude-code.headless.stream-json.v2"
            ),
            "{newer_point}"
        );
    }
    for newer_point in ["2.1.321", "2.1.322", "2.1.323"] {
        assert!(matches!(
            response.assess(&version(newer_point)),
            InterfaceCompatibilityAssessment::UnverifiedNewer(newer)
                if newer.behavior_revision().as_str()
                    == "claude-code.response-only.stream-json.v3"
        ));
    }
    assert!(matches!(
        response.assess(&version("2.1.323")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(newer)
            if newer.behavior_revision().as_str()
                == "claude-code.response-only.stream-json.v3"
    ));
}

#[test]
fn package_trees_change_only_version_metadata_sdk_declarations_and_the_runtime() {
    let inventory = fixture(INVENTORY);
    assert_eq!(
        inventory["compared"],
        json!(["2.1.312", "2.1.314", "2.1.315", "2.1.316"])
    );
    assert_eq!(inventory["previous_ceiling"], "2.1.312");
    assert_eq!(inventory["official_latest"], "2.1.316");
    assert_eq!(
        inventory["unpublished_between_previous_ceiling_and_latest"],
        json!(["2.1.313"])
    );
    assert_eq!(inventory["ceiling_2_1_312_matches_research_459"], true);
    assert_eq!(
        inventory["packages"]["claude-code"]["2.1.312"]["tarball_sha256"],
        "7bbcaf79930440d62daa309ae4fea63ba3df998dd6736b8d1c9a5767bf0bc7c1"
    );
    assert_eq!(
        inventory["packages"]["claude-code"]["2.1.316"]["tarball_sha256"],
        "0b23c8ff73a58c263e88aa740bd8647723141f70dad089df63061858e019c4c3"
    );
    assert_eq!(
        inventory["packages"]["claude-code-darwin-arm64"]["2.1.312"]["files"]["claude"],
        "6b5ec691195e551b02c41df9f7d723f4ea61eaa03c2df90acefe3736f1aeb620"
    );
    assert_eq!(
        inventory["packages"]["claude-code-linux-x64"]["2.1.312"]["files"]["claude"],
        "4f4178ce41d83e23341ebfbaecdbcf8ea6bf7baea0690025abe49eeec3a5d01d"
    );
    assert_eq!(
        inventory["packages"]["claude-code-darwin-arm64"]["2.1.316"]["files"]["claude"],
        "64c327e92b2c5e0f78c0a980c5a518121e1f0d140ca771ca5dd4488d1a2dcb3f"
    );
    assert_eq!(
        inventory["packages"]["claude-code-linux-x64"]["2.1.316"]["files"]["claude"],
        "fd13c70000fec3f2116dfbcbb6121210b4a29328bf0e46210a59dd594a67d824"
    );
    for package in [
        "claude-code",
        "claude-code-darwin-arm64",
        "claude-code-linux-x64",
    ] {
        for point in VERSIONS {
            let count = inventory["packages"][package][point]["files"]
                .as_object()
                .expect("files")
                .len();
            assert_eq!(
                inventory["package_file_counts"][package][point], count,
                "{package} {point}"
            );
        }
    }
    for hop in ["2.1.314_to_2.1.315", "2.1.315_to_2.1.316"] {
        let wrapper = &inventory["file_delta"]["claude-code"][hop];
        assert_eq!(wrapper["added"], json!([]), "{hop}");
        assert_eq!(wrapper["removed"], json!([]), "{hop}");
        assert_eq!(wrapper["changed"], json!(["package.json"]), "{hop}");
    }
    let first = &inventory["file_delta"]["claude-code"]["2.1.312_to_2.1.314"];
    assert_eq!(first["added"], json!([]));
    assert_eq!(first["removed"], json!([]));
    assert_eq!(first["changed"], json!(["package.json", "sdk-tools.d.ts"]));
    for hop in [
        "2.1.312_to_2.1.314",
        "2.1.314_to_2.1.315",
        "2.1.315_to_2.1.316",
    ] {
        for package in ["claude-code-darwin-arm64", "claude-code-linux-x64"] {
            let delta = &inventory["file_delta"][package][hop];
            assert_eq!(delta["added"], json!([]), "{package} {hop}");
            assert_eq!(delta["removed"], json!([]), "{package} {hop}");
            assert_eq!(
                delta["changed"],
                json!(["claude", "package.json"]),
                "{package} {hop}"
            );
        }
    }
    let wrapper = &inventory["packages"]["claude-code"];
    for name in [
        "bin/claude.exe",
        "cli-wrapper.cjs",
        "install.cjs",
        "LICENSE.md",
        "README.md",
    ] {
        for point in ["2.1.314", "2.1.315", "2.1.316"] {
            assert_eq!(
                wrapper[point]["files"][name], wrapper["2.1.312"]["files"][name],
                "{name} {point}"
            );
        }
    }
    assert_eq!(
        inventory["sdk_tools_declaration_delta"]["2.1.312_to_2.1.314"],
        json!([
            "outsidersMayHaveWritten?: true on a shared-file declaration",
            "backgroundWakesOwner?: true on a backgrounded-command declaration"
        ])
    );
    assert_eq!(
        inventory["sdk_tools_declaration_delta"]["2.1.314_to_2.1.315"],
        json!([])
    );
    assert_eq!(
        inventory["sdk_tools_declaration_delta"]["selected_by_claude_code_headless"],
        false
    );
    assert_eq!(
        inventory["sdk_tools_declaration_delta"]["selected_by_claude_code_response_only"],
        false
    );
    assert_eq!(
        inventory["cli_option_inventory"]["added"],
        json!({
            "2.1.312_to_2.1.314": ["--agents-view-if-allowed"],
            "2.1.314_to_2.1.315": ["--gateway-json"],
            "2.1.315_to_2.1.316": []
        })
    );
    for hop in [
        "2.1.312_to_2.1.314",
        "2.1.314_to_2.1.315",
        "2.1.315_to_2.1.316",
    ] {
        assert_eq!(
            inventory["cli_option_inventory"]["removed"][hop],
            json!([]),
            "{hop}"
        );
    }
    assert_eq!(
        inventory["embedded_runtime_growth"]["selected_surface_effect"],
        false
    );
}

#[test]
fn selected_v2_tokens_hold_on_every_hop_and_no_hop_is_a_mapped_change() {
    let inventory = fixture(INVENTORY);
    let markers = fixture(MARKERS);
    let flow = fixture(CONTROL_FLOW);
    let headless = fixture(HEADLESS_PROTOCOL);
    let response = fixture(RESPONSE_PROTOCOL);
    assert_eq!(headless["decision"], "compatible-extension");
    assert_eq!(response["decision"], "compatible-extension");
    assert_eq!(
        headless["behavior_revision"],
        "claude-code.headless.stream-json.v2"
    );
    assert_eq!(
        response["behavior_revision"],
        "claude-code.response-only.stream-json.v3"
    );
    assert_eq!(
        headless["selected_behavior_boundary"]["permission_mode"],
        "plan"
    );
    assert_eq!(headless["selected_behavior_boundary"]["safe_mode"], false);
    assert_eq!(
        headless["selected_behavior_boundary"]["hooks_disabled"],
        false
    );
    assert_eq!(
        headless["selected_behavior_boundary"]["tool_names"],
        json!(["Read", "Glob", "Grep"])
    );
    assert_eq!(
        headless["selected_behavior_boundary"]["setting_sources"],
        json!(["user", "project", "local"])
    );
    assert_eq!(response["selected_behavior_boundary"]["safe_mode"], true);
    assert_eq!(response["selected_behavior_boundary"]["tools_empty"], true);
    assert_eq!(
        response["selected_behavior_boundary"]["mcp_servers_empty"],
        true
    );
    assert_eq!(
        response["selected_behavior_boundary"]["permission_mode_expected_on_init"],
        "default"
    );
    assert_eq!(headless["maximum_turns_extended"], false);
    assert_eq!(headless["skipped_unpublished"], json!(["2.1.313"]));
    assert_eq!(response["skipped_unpublished"], json!(["2.1.313"]));
    assert_eq!(
        flow["hop_comparison"]["structure_identical_across_2_1_312_through_2_1_316"],
        true
    );
    assert_eq!(flow["downloaded_artifacts_executed"], false);
    assert_eq!(flow["provider_prompt_sent"], false);
    assert_eq!(flow["live_session"], false);
    let literals = flow["identifier_stable_literals"]
        .as_object()
        .expect("literal table");
    for (name, literal) in literals {
        assert!(
            literal.as_str().is_some_and(|text| !text.is_empty()),
            "{name}"
        );
    }
    for version in VERSIONS {
        for package in PLATFORMS {
            let row = &markers["versions"][version][package];
            assert_eq!(row["static_markers_observed"], markers["markers"]);
            assert_eq!(row["help_lines_observed"], markers["help_lines"]);
            assert_eq!(
                row["artifact_sha256"],
                inventory["packages"][package][version]["files"]["claude"]
            );
            let counts = &flow["identifier_stable_literal_counts"][version][package];
            for name in literals.keys() {
                assert!(
                    counts[name].as_u64().is_some_and(|count| count > 0),
                    "{version} {package} {name}"
                );
            }
            let selected = &flow["hop_comparison"]["per_version"][version][package]["selected_js"];
            assert_eq!(
                flow["hop_comparison"]["per_version"][version][package]["artifact_sha256"],
                inventory["packages"][package][version]["files"]["claude"]
            );
            let snippet = |key: &str| selected[key].as_str().expect("snippet");
            let prompt = snippet("prompt_ok_false_blocks");
            assert!(prompt.contains("Hooks: Prompt hook condition was not met:"));
            assert!(prompt.contains("outcome:\"blocking\""));
            assert!(prompt.contains("preventContinuation:"));
            assert!(prompt.contains("continueOnBlock!==!0"));
            let deny = snippet("pretooluse_deny_before_dispatch");
            assert!(deny.contains("Execution stopped by PreToolUse hook"));
            assert!(deny.contains("is_error:!0"));
            let rewrite = snippet("rewrite_recheck_push");
            assert!(rewrite.contains("hookRewrites.push"));
            assert!(rewrite.contains("updatedInput"));
            let ask = snippet("rewrite_recheck_ask_to_deny");
            assert!(ask.contains("hookUpdatedInput:void 0"));
            assert!(ask.contains("behavior===\"ask\"?{behavior:\"deny\""));
            let cap = snippet("stop_block_cap_default_8");
            assert!(cap.contains("CLAUDE_CODE_STOP_HOOK_BLOCK_CAP??8"));
            assert!(cap.contains("hit_cap:!0"));
            let reentry = snippet("stop_hook_active_reentry");
            assert!(reentry.contains("stopHookActive:!0"));
            assert!(reentry.contains("stopHookBlockingCount"));
            let active = snippet("stop_hook_active_input");
            assert!(active.contains("hook_event_name:\"SubagentStop\",stop_hook_active:"));
            assert!(active.contains("hook_event_name:\"Stop\",stop_hook_active:"));
            assert!(
                snippet("prompt_schema_required_ok")
                    .starts_with("required:[\"ok\"],additionalProperties:!1")
            );
        }
    }
    for protocol in [&headless, &response] {
        let hops = protocol["per_hop_classification"].as_array().expect("hops");
        assert_eq!(hops.len(), 3);
        for hop in hops {
            assert_eq!(hop["selected_mapped_change"], false);
            assert_eq!(hop["decision"], "compatible-extension");
            assert!(
                !hop["release_note_review"]
                    .as_array()
                    .expect("release note review")
                    .is_empty()
            );
        }
    }
    assert_eq!(
        headless["per_hop_classification"][0]["applies_to"],
        "claude-code.headless"
    );
    assert_eq!(
        response["per_hop_classification"][0]["applies_to"],
        "claude-code.response-only"
    );
    assert_eq!(
        headless["v2_rules_retained"][0],
        "instruction-form prompt hook ok false blocks"
    );
}

#[test]
fn response_only_isolation_keeps_safe_mode_and_the_gated_builtin() {
    let inventory = fixture(INVENTORY);
    let isolation = fixture(ISOLATION);
    assert_eq!(isolation["downloaded_artifacts_executed"], false);
    assert_eq!(
        isolation["disable_slash_commands_help"],
        "Disable all skills and commands"
    );
    assert_eq!(
        isolation["disable_slash_commands_help_byte_identical_from_2_1_312"],
        true
    );
    assert_eq!(
        isolation["tools_option_description_byte_identical_from_2_1_312"],
        true
    );
    assert_eq!(
        isolation["safe_mode_option_description_byte_identical_from_2_1_312"],
        true
    );
    let parts = isolation["safe_mode_option_description_parts"]
        .as_array()
        .expect("parts");
    assert_eq!(parts.len(), 3);
    assert_eq!(
        parts[1],
        "\\u2014 useful for troubleshooting a broken configuration. "
    );
    assert_eq!(isolation["easel_still_present"], true);
    assert_eq!(
        isolation["empty_tools_and_mcp_leave_easel_autocharts_unselected"],
        true
    );
    let platform = &isolation["claude_platform"];
    assert_eq!(platform["present_at_2_1_312"], true);
    assert_eq!(platform["registered_from"], "2.1.311");
    assert_eq!(platform["availability_flag"], "tengu_peppy_crescent");
    assert_eq!(platform["availability_requires_flag_true"], true);
    assert_eq!(platform["availability_requires_interactive"], true);
    assert_eq!(platform["selected_mapped_change"], false);
    assert_eq!(platform["classification"], "gated-builtin-unselected");
    assert_eq!(platform["mcp_url"], "https://platform.claude.com/docs/mcp");
    assert_eq!(
        isolation["easel_registration"]["selected_routes_are_remote_cowork"],
        false
    );
    assert_eq!(
        isolation["interactivity_bookkeeping"]["present_from"],
        "2.1.314"
    );
    assert_eq!(
        isolation["interactivity_bookkeeping"]["selected_mapped_change"],
        false
    );
    let ids = isolation["cc_plugin_ids"].as_array().expect("plugin ids");
    assert_eq!(ids.len(), 13);
    assert!(ids.iter().any(|id| id == "claude-platform"));
    assert!(ids.iter().any(|id| id == "easel"));
    for id in isolation["research_378_nine_remain"]
        .as_array()
        .expect("builtin nine")
    {
        assert!(ids.contains(id), "{id}");
    }
    for version in VERSIONS {
        for package in PLATFORMS {
            let row = &isolation["per_version"][version][package];
            assert_eq!(
                row["artifact_sha256"],
                inventory["packages"][package][version]["files"]["claude"]
            );
            assert_eq!(row["cc_plugin_ids"], isolation["cc_plugin_ids"]);
            let option = row["safe_mode_option"].as_str().expect("option");
            for part in parts {
                assert!(
                    option.contains(part.as_str().expect("part")),
                    "{version} {package}"
                );
            }
            assert!(option.contains("--safe-mode"));
            let filter = row["safe_mode_filter"].as_str().expect("filter");
            assert!(filter.contains("built-in plugins load regardless"));
            assert!(filter.contains("e.filter((n)=>"));
            let detector = row["safe_mode_detector"].as_str().expect("detector");
            assert!(detector.contains("process.env.CLAUDE_CODE_SAFE_MODE"));
            assert!(detector.contains("(\"--safe-mode\")"));
            let gate = row["safe_mode_mcp_gate"].as_str().expect("gate");
            assert!(gate.contains("[e])return!0"));
            assert!(gate.contains("explicitlyRequested"));
            let policy = row["safe_mode_mcp_policy"].as_str().expect("policy");
            assert!(policy.contains("mcpAutoDiscovered:!1"));
            assert!(policy.contains("mcpClaudeAi:!1"));
            let init = row["init_fields"].as_str().expect("init");
            assert!(init.contains("type:\"system\",subtype:\"init\""));
            assert!(init.contains("tools:e.tools.map("));
            assert!(init.contains("mcp_servers:e.mcpClients.map("));
            assert!(init.contains("permissionMode:"));
            assert!(
                row["permission_mode_default_mapper"]
                    .as_str()
                    .expect("mapper")
                    .contains("??\"default\"")
            );
            assert!(
                row["permission_mode_treats_undefined_as_default"]
                    .as_str()
                    .expect("undefined mapper")
                    .contains("e===\"default\"||e===void 0")
            );
            assert!(
                row["builtin_predicate"]
                    .as_str()
                    .expect("predicate")
                    .contains("endsWith(`@${")
            );
            let tail = row["builtin_registration_tail"].as_str().expect("tail");
            assert!(tail.contains("CLAUDE_CODE_ENTRYPOINT!==\"local-agent\""));
            assert!(tail.contains("cc-plugin-easel"));
            assert!(tail.contains("cc-plugin-claude-platform"));
            assert!(tail.contains("remote_cowork"));
            let module = row["claude_platform_module"].as_str().expect("module");
            assert!(module.contains("return\"tengu_peppy_crescent\""));
            assert!(module.contains("https://platform.claude.com/docs/mcp"));
            assert!(module.contains("CLAUDE_CODE_DISABLE_CLAUDE_API_SKILL"));
            assert!(module.contains("mayConnectMcpServer:"));
            assert!(module.contains("PLUGIN_NAME"));
            assert!(module.contains("mcpAutoDiscovered"));
            assert!(module.contains(",!1)===!0"));
            let getter = function_name(row["interactive_getter"].as_str().expect("getter"));
            assert!(module.contains(&format!("{getter}()&&")));
            let setter = function_name(row["interactive_setter"].as_str().expect("setter"));
            let launch = row["launch_interactivity"].as_str().expect("launch");
            assert!(launch.contains("kind===\"non-interactive\""));
            assert!(launch.contains(&format!("{setter}(!")));
            let detection = row["print_detection"].as_str().expect("detection");
            let detector_name = function_name(detection);
            assert!(detection.contains("(\"-p\",e)"));
            assert!(detection.contains("(\"--print\",e)"));
            assert!(detection.contains("return n||t||o||!process.stdout.isTTY"));
            assert!(launch.contains(&format!("||{detector_name}(e)")));
            assert!(
                row["availability_dispatch"]
                    .as_str()
                    .expect("dispatch")
                    .contains("builtinPluginFailures.has(e))return!1")
            );
            let skip = row["builtin_availability_skip"].as_str().expect("skip");
            assert!(skip.contains("continue;let s=`${r}@${"));
            assert!(skip.contains("mcpServers:n.mcpServers"));
            assert!(skip.contains("isBuiltin:!0"));
            assert_eq!(row["stop_hook_event_types"]["hook_started"], 11);
            assert_eq!(row["stop_hook_event_types"]["hook_response"], 38);
            assert_eq!(row["stream_json_event_types"]["subtype:\"init\""], 1);
            assert_eq!(row["stream_json_event_types"]["type:\"result\""], 12);
            if version == "2.1.312" {
                assert_eq!(row["interactivity_decided_bookkeeping"], 0);
            } else {
                assert_eq!(row["interactivity_decided_bookkeeping"], 4);
            }
        }
    }
}

#[test]
fn forked_stream_surface_stopped_on_by_research_374_is_rechecked_on_every_hop() {
    let inventory = fixture(INVENTORY);
    let forked = fixture(FORKED_SURFACE);
    assert_eq!(forked["stopped_by"], "Research 374");
    assert_eq!(
        forked["stop_rows"],
        json!([
            "2.1.286_to_2.1.287: --output-format stream-json turn streaming for a context:fork skill invoked as the prompt",
            "2.1.289_to_2.1.290: permission and safety checks after a PreToolUse hook rewrites tool input"
        ])
    );
    assert_eq!(forked["downloaded_artifacts_executed"], false);
    assert_eq!(forked["provider_prompt_sent"], false);
    assert_eq!(forked["live_session"], false);
    assert_eq!(forked["selected_mapped_change"], false);
    assert_eq!(
        forked["assistant_frame_description_byte_identical_from_2_1_312"],
        true
    );
    assert_eq!(
        forked["nullable_parent_tool_use_id_schema_field"],
        "parent_tool_use_id:o().nullable()"
    );
    assert_eq!(forked["nullable_parent_tool_use_id_schema_field_count"], 4);
    assert_eq!(
        forked["forked_context_definition_tokens"],
        json!([
            "context:\"fork\"",
            "userInvocable:!1",
            "agent:\"claude-test:author\""
        ])
    );
    let description = forked["assistant_frame_description"]
        .as_str()
        .expect("description");
    for clause in forked["assistant_frame_description_required_clauses"]
        .as_array()
        .expect("clauses")
    {
        assert!(
            description.contains(clause.as_str().expect("clause")),
            "{clause}"
        );
    }
    let forward = &forked["forward_subagent_text_option"];
    assert_eq!(forward["option"], "--forward-subagent-text");
    assert_eq!(
        forward["description"],
        "Forward subagent text and thinking blocks as assistant/user messages with parent_tool_use_id set (only works with --print and --output-format=stream-json)"
    );
    assert_eq!(forward["classification"], "unmapped-opt-in");
    assert_eq!(forward["passed_by_claude_code_headless"], false);
    assert_eq!(forward["passed_by_claude_code_response_only"], false);
    assert_eq!(
        forked["adapter_projection"]["test"],
        "claude_code_activity::tests::forked_assistant_frame_accepts_null_stop_reason_without_final_activity"
    );
    assert_eq!(
        forked["adapter_projection"]["included_in_validate_selector"],
        "validate:current-claude-code-headless"
    );
    for hop in [
        "2.1.312_to_2.1.314",
        "2.1.314_to_2.1.315",
        "2.1.315_to_2.1.316",
    ] {
        assert_eq!(forked["hop_classification"][hop], false, "{hop}");
    }
    for version in VERSIONS {
        for package in PLATFORMS {
            let row = &forked["per_version"][version][package];
            assert_eq!(
                row["artifact_sha256"],
                inventory["packages"][package][version]["files"]["claude"]
            );
            assert_eq!(row["assistant_frame_description"], description);
            assert_eq!(row["nullable_parent_tool_use_id_count"], 4);
            assert_eq!(row["context_fork_occurrences"], 2);
            let definitions = row["forked_context_definition"]
                .as_str()
                .expect("definition window");
            assert_eq!(definitions.matches("context:\"fork\"").count(), 2);
            for token in forked["forked_context_definition_tokens"]
                .as_array()
                .expect("tokens")
            {
                assert!(
                    definitions.contains(token.as_str().expect("token")),
                    "{version} {package} {token}"
                );
            }
        }
    }
}
