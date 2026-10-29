use serde_json::{Value, json};
use swallowtail_adapter_claude_agent::{
    CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION, CLAUDE_CODE_RESPONSE_ONLY_DENIED_VERSIONS,
    CLAUDE_CODE_RESPONSE_ONLY_LATEST_QUALIFIED_VERSION, claude_code_headless_claim,
    claude_code_response_only_claim,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const HEADLESS_IDENTITY: &str = include_str!("fixtures/claude-code-2.1.312/headless-identity.json");
const RESPONSE_IDENTITY: &str =
    include_str!("fixtures/claude-code-2.1.312/response-only-identity.json");
const INVENTORY: &str = include_str!("fixtures/claude-code-2.1.312/dist-inventory.json");
const MARKERS: &str = include_str!("fixtures/claude-code-2.1.312/marker-ledger.json");
const CONTROL_FLOW: &str = include_str!("fixtures/claude-code-2.1.312/selected-control-flow.json");
const ISOLATION: &str = include_str!("fixtures/claude-code-2.1.312/response-only-isolation.json");
const HEADLESS_PROTOCOL: &str = include_str!("fixtures/claude-code-2.1.312/headless-protocol.json");
const RESPONSE_PROTOCOL: &str =
    include_str!("fixtures/claude-code-2.1.312/response-only-protocol.json");

fn fixture(value: &str) -> Value {
    serde_json::from_str(value).expect("frozen JSON is valid")
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("version")
}

#[test]
fn official_stable_is_2_1_312_and_the_two_claims_stay_separate() {
    let headless = fixture(HEADLESS_IDENTITY);
    let response = fixture(RESPONSE_IDENTITY);
    assert_ne!(headless["axis"], response["axis"]);
    assert_eq!(headless["axis"], "claude-code.headless-stream-json");
    assert_eq!(response["axis"], "claude-code.response-only-stream-json");
    for identity in [&headless, &response] {
        assert_eq!(identity["official_latest_at_observation"], "2.1.312");
        assert_eq!(identity["previous_ceiling"], "2.1.310");
        assert_eq!(
            identity["npm"]["dist_tags"],
            json!({"stable": "2.1.308", "next": "2.1.312", "latest": "2.1.312"})
        );
        assert_eq!(identity["npm"]["stable_dist_tag_is_delayed_channel"], true);
        assert_eq!(identity["github"]["latest_non_prerelease"], "v2.1.312");
        assert_eq!(
            identity["github"]["tag_commits"]["2.1.310"],
            "b70dc4113ad72e0ee44f7b1b95afaf510b53ee98"
        );
        assert_eq!(
            identity["github"]["tag_commits"]["2.1.311"],
            "08c6a4f5121ae140d3c1b5c96e3e5ae53587b54b"
        );
        assert_eq!(
            identity["github"]["tag_commits"]["2.1.312"],
            "0e1f4789406477380b267a01f3ddd50b0211e5b3"
        );
        assert_eq!(
            identity["published_hops_after_previous_ceiling"],
            json!(["2.1.311", "2.1.312"])
        );
        assert_eq!(identity["first_unpublished_after_latest"], "2.1.313");
        assert_eq!(identity["npm_2_1_313_http_status"], 404);
        assert_eq!(identity["github_tag_v2_1_313_present"], false);
        assert_eq!(identity["host"]["present"], false);
        assert_eq!(identity["identity_decision"], "compatible-extension");
        assert_eq!(identity["downloaded_artifacts_executed"], false);
        assert_eq!(identity["provider_prompt_sent"], false);
        assert_eq!(identity["live_session"], false);
        assert_eq!(identity["channels_agree_on_official_stable"], true);
        assert_eq!(
            identity["claim_at_observation"]["latest_qualified"],
            "2.1.310"
        );
        assert_eq!(identity["after_claim"]["latest_qualified"], "2.1.312");
        assert_eq!(identity["after_claim"]["first_unverified_newer"], "2.1.313");
    }
    assert_eq!(
        headless["claim_at_observation"]["claim_id"],
        "claude-code.headless.window-1"
    );
    assert_eq!(
        headless["after_claim"]["maintained_behavior_revision"],
        "claude-code.headless.stream-json.v2"
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
    assert_ne!(headless["axis"], "claude-agent.acp-adapter");
    assert_ne!(response["axis"], "claude-agent.sdk.package");
}

#[test]
fn both_claims_qualify_2_1_311_and_2_1_312_on_the_existing_revisions() {
    assert_eq!(CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION, "2.1.312");
    assert_eq!(
        CLAUDE_CODE_RESPONSE_ONLY_LATEST_QUALIFIED_VERSION,
        "2.1.312"
    );
    assert_eq!(
        CLAUDE_CODE_RESPONSE_ONLY_DENIED_VERSIONS,
        &[
            "2.1.244", "2.1.249", "2.1.253", "2.1.254", "2.1.255", "2.1.256", "2.1.262", "2.1.264",
            "2.1.279", "2.1.297", "2.1.298",
        ]
    );
    let headless = claude_code_headless_claim();
    let response = claude_code_response_only_claim();
    assert_ne!(headless.axis(), response.axis());
    for point in ["2.1.310", "2.1.311", "2.1.312"] {
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
    for gap in ["2.1.297", "2.1.298", "2.1.244", "2.1.279"] {
        assert!(!headless.permits(&version(gap)), "{gap}");
        assert!(!response.permits(&version(gap)), "{gap}");
    }
    assert!(matches!(
        headless.assess(&version("2.1.313")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert!(matches!(
        response.assess(&version("2.1.313")),
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
        json!(["2.1.310", "2.1.311", "2.1.312"])
    );
    assert_eq!(inventory["ceiling_2_1_310_matches_research_445"], true);
    assert_eq!(
        inventory["packages"]["claude-code"]["2.1.310"]["tarball_sha256"],
        "d696314076114df499ba88e117000a24a9d7ab4f502dcaefba52c83fd22acfbd"
    );
    assert_eq!(
        inventory["packages"]["claude-code"]["2.1.311"]["tarball_sha256"],
        "d97fea08695f32b13b3dec072687f2acc550484caecaf5005d4ebad86c5bf199"
    );
    assert_eq!(
        inventory["packages"]["claude-code"]["2.1.312"]["tarball_sha256"],
        "7bbcaf79930440d62daa309ae4fea63ba3df998dd6736b8d1c9a5767bf0bc7c1"
    );
    assert_eq!(
        inventory["packages"]["claude-code-darwin-arm64"]["2.1.312"]["files"]["claude"],
        "6b5ec691195e551b02c41df9f7d723f4ea61eaa03c2df90acefe3736f1aeb620"
    );
    assert_eq!(
        inventory["packages"]["claude-code-linux-x64"]["2.1.312"]["files"]["claude"],
        "4f4178ce41d83e23341ebfbaecdbcf8ea6bf7baea0690025abe49eeec3a5d01d"
    );
    let packages = [
        "claude-code",
        "claude-code-darwin-arm64",
        "claude-code-linux-x64",
    ];
    for package in packages {
        for point in ["2.1.310", "2.1.311", "2.1.312"] {
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
    for hop in ["2.1.310_to_2.1.311", "2.1.311_to_2.1.312"] {
        let wrapper = &inventory["file_delta"]["claude-code"][hop];
        assert_eq!(wrapper["added"], json!([]));
        assert_eq!(wrapper["removed"], json!([]));
        assert_eq!(
            wrapper["changed"],
            json!(["package.json", "sdk-tools.d.ts"])
        );
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
    assert_eq!(
        inventory["sdk_tools_declaration_delta"]["selected_by_claude_code_headless"],
        false
    );
    assert_eq!(
        inventory["sdk_tools_declaration_delta"]["selected_by_claude_code_response_only"],
        false
    );
    let wrapper = &inventory["packages"]["claude-code"];
    for name in [
        "bin/claude.exe",
        "cli-wrapper.cjs",
        "install.cjs",
        "LICENSE.md",
        "README.md",
    ] {
        assert_eq!(
            wrapper["2.1.311"]["files"][name], wrapper["2.1.310"]["files"][name],
            "{name}"
        );
        assert_eq!(
            wrapper["2.1.312"]["files"][name], wrapper["2.1.310"]["files"][name],
            "{name}"
        );
    }
}

#[test]
fn selected_v2_tokens_hold_and_neither_hop_is_a_mapped_change() {
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
        headless["selected_behavior_boundary"]["tool_names"],
        json!(["Read", "Glob", "Grep"])
    );
    assert_eq!(response["selected_behavior_boundary"]["safe_mode"], true);
    assert_eq!(response["selected_behavior_boundary"]["tools_empty"], true);
    assert_eq!(
        response["selected_behavior_boundary"]["permission_mode_expected_on_init"],
        "default"
    );
    assert_eq!(headless["maximum_turns_extended"], false);
    assert_eq!(
        flow["structure_identical_across_2_1_310_through_2_1_312"],
        true
    );
    assert_eq!(flow["downloaded_artifacts_executed"], false);
    assert_eq!(flow["provider_prompt_sent"], false);
    assert_eq!(flow["live_session"], false);
    for package in ["claude-code-darwin-arm64", "claude-code-linux-x64"] {
        for point in ["2.1.310", "2.1.311", "2.1.312"] {
            let row = &markers["versions"][point][package];
            assert_eq!(row["static_markers_observed"], markers["markers"]);
            assert_eq!(row["help_lines_observed"], markers["help_lines"]);
            assert_eq!(
                row["artifact_sha256"],
                inventory["packages"][package][point]["files"]["claude"]
            );
            let selected = &flow["hop_comparison"]["per_version"][point][package]["selected_js"];
            let snippet = |key: &str| selected[key].as_str().expect("snippet");
            assert!(snippet("prompt_ok_false_blocks").contains("outcome:\"blocking\""));
            assert!(snippet("prompt_ok_false_blocks").contains("continueOnBlock!==!0"));
            assert!(
                snippet("pretooluse_deny_before_dispatch")
                    .contains("Execution stopped by PreToolUse hook")
            );
            assert!(snippet("pretooluse_deny_before_dispatch").contains("is_error:!0"));
            assert!(snippet("rewrite_recheck_push").contains("hookRewrites.push"));
            assert!(snippet("rewrite_recheck_ask_to_deny").contains("hookUpdatedInput:"));
            assert!(
                snippet("rewrite_recheck_ask_to_deny")
                    .contains("behavior===\"ask\"?{behavior:\"deny\"")
            );
            assert!(snippet("stop_hook_active_reentry").contains("stopHookActive:!0"));
            assert!(
                snippet("stop_block_cap_default_8").contains("CLAUDE_CODE_STOP_HOOK_BLOCK_CAP??8")
            );
            assert_eq!(
                flow["hop_comparison"]["per_version"][point][package]["artifact_sha256"],
                inventory["packages"][package][point]["files"]["claude"]
            );
        }
        assert_eq!(
            flow["v2_selected_path_at_2_1_312"][package]["prompt_schema_required_ok"],
            "required:[\"ok\"],additionalProperties:!1"
        );
    }
    for protocol in [&headless, &response] {
        let hops = protocol["per_hop_classification"].as_array().unwrap();
        assert_eq!(hops.len(), 2);
        for hop in hops {
            assert_eq!(hop["selected_mapped_change"], false);
            assert_eq!(hop["decision"], "compatible-extension");
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
}

#[test]
fn response_only_isolation_keeps_safe_mode_and_records_the_gated_builtin() {
    let inventory = fixture(INVENTORY);
    let isolation = fixture(ISOLATION);
    assert_eq!(isolation["downloaded_artifacts_executed"], false);
    assert_eq!(
        isolation["disable_slash_commands_help"],
        "Disable all skills and commands"
    );
    assert_eq!(isolation["easel_still_present"], true);
    assert_eq!(
        isolation["empty_tools_and_mcp_leave_easel_autocharts_unselected"],
        true
    );
    let platform = &isolation["claude_platform"];
    assert_eq!(platform["present_at_2_1_310"], false);
    assert_eq!(platform["registered_from"], "2.1.311");
    assert_eq!(platform["availability_flag"], "tengu_peppy_crescent");
    assert_eq!(platform["availability_flag_default_argument"], false);
    assert_eq!(platform["availability_requires_flag_true"], true);
    assert_eq!(platform["selected_mapped_change"], false);
    assert_eq!(platform["classification"], "gated-builtin-unselected");
    assert_eq!(platform["mcp_url"], "https://platform.claude.com/docs/mcp");
    assert_eq!(isolation["easel_registration"]["at_2_1_310"], "if(e)");
    assert_eq!(
        isolation["easel_registration"]["selected_routes_are_remote_cowork"],
        false
    );
    assert_eq!(
        isolation["easel_registration"]["selected_mapped_change"],
        false
    );
    let ids_310 = &isolation["cc_plugin_ids_at_2_1_310"];
    let ids_later = &isolation["cc_plugin_ids_from_2_1_311"];
    assert!(
        !ids_310
            .as_array()
            .unwrap()
            .iter()
            .any(|id| id == "claude-platform")
    );
    assert!(
        ids_later
            .as_array()
            .unwrap()
            .iter()
            .any(|id| id == "claude-platform")
    );
    let mut safe_mode = None;
    for point in ["2.1.310", "2.1.311", "2.1.312"] {
        for package in ["claude-code-darwin-arm64", "claude-code-linux-x64"] {
            let row = &isolation["per_version"][point][package];
            assert_eq!(
                row["artifact_sha256"],
                inventory["packages"][package][point]["files"]["claude"]
            );
            let option = row["safe_mode_option"].as_str().unwrap();
            assert!(option.contains("installed plugins, hooks, MCP servers"));
            assert!(option.contains("Sets CLAUDE_CODE_SAFE_MODE=1."));
            match safe_mode {
                None => safe_mode = Some(option.to_owned()),
                Some(ref previous) => assert_eq!(option, previous),
            }
            let init = row["init_fields"].as_str().unwrap();
            assert!(init.contains("tools:"));
            assert!(init.contains("mcp_servers:"));
            assert!(init.contains("permissionMode:"));
            let tail = row["builtin_registration_tail"].as_str().unwrap();
            assert!(tail.contains("CLAUDE_CODE_ENTRYPOINT!==\"local-agent\""));
            assert!(tail.contains("cc-plugin-easel"));
            if point == "2.1.310" {
                assert_eq!(&row["cc_plugin_ids"], ids_310);
                assert!(row["claude_platform_module"].is_null());
                assert!(!tail.contains("remote_cowork"));
                assert!(!tail.contains("cc-plugin-claude-platform"));
            } else {
                assert_eq!(&row["cc_plugin_ids"], ids_later);
                let module = row["claude_platform_module"].as_str().unwrap();
                let (getter, setter, detector, availability, safe_gate) = match (point, package) {
                    ("2.1.311", "claude-code-darwin-arm64") => ("qd", "F5e", "dlt", "pot", "cr"),
                    ("2.1.311", _) => ("qd", "I6e", "olt", "aot", "dr"),
                    ("2.1.312", "claude-code-darwin-arm64") => ("Xd", "J5e", "Alt", "Aot", "or"),
                    _ => ("Xd", "G6e", "vlt", "vot", "or"),
                };
                assert!(module.contains(&format!("return {getter}()&&")));
                assert!(module.contains(&format!("!{safe_gate}(\"mcpAutoDiscovered\")")));
                assert!(module.contains("return t()&&e===!0"));
                assert!(
                    row["interactive_getter"]
                        .as_str()
                        .unwrap()
                        .starts_with(&format!("function {getter}()"))
                );
                assert!(
                    row["interactive_getter"]
                        .as_str()
                        .unwrap()
                        .contains("launchOptions.isInteractive()")
                );
                assert!(
                    row["interactive_setter"]
                        .as_str()
                        .unwrap()
                        .contains("launchOptions.replaceIsInteractive(e)")
                );
                let launch = row["launch_interactivity"].as_str().unwrap();
                assert!(launch.contains(&format!("||{detector}(e)")));
                assert!(launch.contains(&format!("{setter}(!h)")));
                let detection = row["print_detection"].as_str().unwrap();
                assert!(detection.contains("\"-p\",e)"));
                assert!(detection.contains("return n||t||o||!process.stdout.isTTY"));
                assert!(
                    row["availability_dispatch"]
                        .as_str()
                        .unwrap()
                        .contains("return t.isAvailable()")
                );
                assert!(
                    row["builtin_availability_skip"]
                        .as_str()
                        .unwrap()
                        .starts_with(&format!("if(!{availability}(r))continue;"))
                );
                assert!(
                    row["builtin_availability_skip"]
                        .as_str()
                        .unwrap()
                        .contains("mcpServers:n.mcpServers")
                );
                assert!(
                    row["safe_mode_detector"]
                        .as_str()
                        .unwrap()
                        .contains("\"--safe-mode\"")
                );
                assert!(
                    row["safe_mode_mcp_gate"]
                        .as_str()
                        .unwrap()
                        .contains("&&!l[e])return!0")
                );
                assert!(
                    row["safe_mode_mcp_policy"]
                        .as_str()
                        .unwrap()
                        .contains("mcpAutoDiscovered:!1")
                );
                assert!(module.contains("return\"tengu_peppy_crescent\""));
                assert!(module.contains(",!1)===!0"));
                assert!(module.contains("===\"tst\""));
                assert!(module.contains("https://platform.claude.com/docs/mcp"));
                assert!(module.contains("CLAUDE_CODE_DISABLE_CLAUDE_API_SKILL"));
                assert!(module.contains("mayConnectMcpServer:"));
                assert!(tail.contains("===\"remote_cowork\""));
                assert!(tail.contains("cc-plugin-claude-platform"));
            }
        }
    }
}
