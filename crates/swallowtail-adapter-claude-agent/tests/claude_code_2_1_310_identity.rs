use serde_json::{Value, json};
use swallowtail_adapter_claude_agent::{
    CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION, CLAUDE_CODE_RESPONSE_ONLY_DENIED_VERSIONS,
    CLAUDE_CODE_RESPONSE_ONLY_LATEST_QUALIFIED_VERSION, claude_code_headless_claim,
    claude_code_response_only_claim,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const HEADLESS_IDENTITY: &str = include_str!("fixtures/claude-code-2.1.310/headless-identity.json");
const RESPONSE_IDENTITY: &str =
    include_str!("fixtures/claude-code-2.1.310/response-only-identity.json");
const INVENTORY: &str = include_str!("fixtures/claude-code-2.1.310/dist-inventory.json");
const MARKERS: &str = include_str!("fixtures/claude-code-2.1.310/marker-ledger.json");
const CONTROL_FLOW: &str = include_str!("fixtures/claude-code-2.1.310/selected-control-flow.json");
const ISOLATION: &str = include_str!("fixtures/claude-code-2.1.310/response-only-isolation.json");
const HEADLESS_PROTOCOL: &str = include_str!("fixtures/claude-code-2.1.310/headless-protocol.json");
const RESPONSE_PROTOCOL: &str =
    include_str!("fixtures/claude-code-2.1.310/response-only-protocol.json");

fn fixture(value: &str) -> Value {
    serde_json::from_str(value).expect("frozen JSON is valid")
}

fn versions() -> Vec<String> {
    (293..=310)
        .filter(|patch| !matches!(patch, 297 | 298))
        .map(|patch| format!("2.1.{patch}"))
        .collect()
}

fn published_after(start: u32) -> Vec<String> {
    ((start + 1)..=310)
        .filter(|patch| !matches!(patch, 297 | 298))
        .map(|patch| format!("2.1.{patch}"))
        .collect()
}

#[test]
fn official_stable_is_2_1_310_and_the_two_claims_stay_separate() {
    let headless = fixture(HEADLESS_IDENTITY);
    let response = fixture(RESPONSE_IDENTITY);
    assert_ne!(headless["axis"], response["axis"]);
    for identity in [&headless, &response] {
        assert_eq!(identity["official_latest_at_observation"], "2.1.310");
        assert_eq!(identity["observed_at_utc"], "2026-10-26T10:32:43Z");
        assert_eq!(
            identity["npm"]["dist_tags"],
            json!({"stable": "2.1.308", "next": "2.1.310", "latest": "2.1.310"})
        );
        assert_eq!(identity["npm"]["stable_dist_tag_is_delayed_channel"], true);
        assert_eq!(identity["github"]["latest_non_prerelease"], "v2.1.310");
        assert_eq!(
            identity["github"]["tag_commits"]["2.1.310"],
            "b70dc4113ad72e0ee44f7b1b95afaf510b53ee98"
        );
        assert_eq!(
            identity["unpublished_between"],
            json!(["2.1.297", "2.1.298"])
        );
        assert_eq!(identity["first_unpublished_after_latest"], "2.1.311");
        assert_eq!(identity["npm_2_1_297_http_status"], 404);
        assert_eq!(identity["npm_2_1_298_http_status"], 404);
        assert_eq!(identity["npm_2_1_311_http_status"], 404);
        assert_eq!(identity["github_tag_v2_1_297_present"], false);
        assert_eq!(identity["github_tag_v2_1_298_present"], false);
        assert_eq!(identity["github_tag_v2_1_311_present"], false);
        assert_eq!(identity["host"]["present"], false);
        assert_eq!(identity["identity_decision"], "compatible-extension");
        assert_eq!(identity["segment_shape"], "compatible-extension");
        assert_eq!(identity["downloaded_artifacts_executed"], false);
        assert_eq!(identity["provider_prompt_sent"], false);
        assert_eq!(identity["live_session"], false);
        assert_eq!(identity["channels_agree_on_official_stable"], true);
    }
    assert_eq!(headless["previous_ceiling"], "2.1.294");
    assert_eq!(
        headless["claim_at_observation"]["latest_qualified"],
        "2.1.294"
    );
    assert_eq!(
        headless["claim_at_observation"]["claim_id"],
        "claude-code.headless.window-1"
    );
    assert_eq!(response["previous_ceiling"], "2.1.293");
    assert_eq!(
        response["claim_at_observation"]["latest_qualified"],
        "2.1.293"
    );
    assert_eq!(
        response["claim_at_observation"]["claim_id"],
        "claude-code.response-only.window-1"
    );
    assert_eq!(
        headless["published_hops_after_previous_ceiling"],
        json!(published_after(294))
    );
    assert_eq!(
        response["published_hops_after_previous_ceiling"],
        json!(published_after(293))
    );
}

#[test]
fn both_claims_extend_through_2_1_310_and_keep_their_behavior() {
    assert_eq!(CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION, "2.1.310");
    assert_eq!(
        CLAUDE_CODE_RESPONSE_ONLY_LATEST_QUALIFIED_VERSION,
        "2.1.310"
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
    assert_eq!(headless.latest_qualified().as_str(), "2.1.310");
    assert_eq!(response.latest_qualified().as_str(), "2.1.310");
    assert_ne!(headless.axis(), response.axis());
    for patch in 295..=310 {
        let version = InterfaceVersion::new(format!("2.1.{patch}")).expect("version");
        if matches!(patch, 297 | 298) {
            assert!(!headless.permits(&version), "2.1.{patch}");
            assert!(!response.permits(&version), "2.1.{patch}");
            continue;
        }
        assert!(
            matches!(
                headless.assess(&version),
                InterfaceCompatibilityAssessment::Qualified(matched)
                    if matched.behavior_revision().as_str()
                        == "claude-code.headless.stream-json.v2"
                        && matched.support_status() == InterfaceSupportStatus::Maintained
            ),
            "2.1.{patch}"
        );
        assert!(
            matches!(
                response.assess(&version),
                InterfaceCompatibilityAssessment::Qualified(matched)
                    if matched.behavior_revision().as_str()
                        == "claude-code.response-only.stream-json.v3"
                        && matched.support_status() == InterfaceSupportStatus::Maintained
            ),
            "2.1.{patch}"
        );
    }
    let newer = InterfaceVersion::new("2.1.311").expect("version");
    assert!(matches!(
        headless.assess(&newer),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert!(matches!(
        response.assess(&newer),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert!(response.supports(&InterfaceVersion::new("2.1.294").expect("version")));
}

#[test]
fn package_trees_change_only_version_metadata_and_the_runtime() {
    let inventory = fixture(INVENTORY);
    let versions = versions();
    assert_eq!(inventory["compared"], json!(versions));
    assert_eq!(
        inventory["unpublished_between"],
        json!(["2.1.297", "2.1.298"])
    );
    let packages = [
        "claude-code",
        "claude-code-darwin-arm64",
        "claude-code-linux-x64",
    ];
    let sdk_change_hops = [
        "2.1.294_to_2.1.295",
        "2.1.295_to_2.1.296",
        "2.1.296_to_2.1.299",
        "2.1.301_to_2.1.302",
    ];
    for package in packages {
        for version in &versions {
            let count = inventory["packages"][package][version]["files"]
                .as_object()
                .expect("files")
                .len();
            assert_eq!(
                inventory["package_file_counts"][package][version], count,
                "{package} {version}"
            );
        }
        let mut previous = &versions[0];
        for version in versions.iter().skip(1) {
            let hop = format!("{previous}_to_{version}");
            let delta = &inventory["file_delta"][package][&hop];
            assert_eq!(delta["added"], json!([]), "{package} {hop}");
            assert_eq!(delta["removed"], json!([]), "{package} {hop}");
            if package == "claude-code" {
                let mut changed = vec!["package.json"];
                if sdk_change_hops.contains(&hop.as_str()) {
                    changed.push("sdk-tools.d.ts");
                }
                assert_eq!(delta["changed"], json!(changed), "{hop}");
            } else {
                assert_eq!(
                    delta["changed"],
                    json!(["claude", "package.json"]),
                    "{package} {hop}"
                );
            }
            previous = version;
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
        let first = &wrapper["2.1.294"]["files"][name];
        for version in versions
            .iter()
            .filter(|version| version.as_str() != "2.1.293")
        {
            assert_eq!(&wrapper[version]["files"][name], first, "{name} {version}");
        }
    }
    assert_eq!(
        wrapper["2.1.310"]["tarball_sha256"],
        "d696314076114df499ba88e117000a24a9d7ab4f502dcaefba52c83fd22acfbd"
    );
    assert_eq!(
        inventory["packages"]["claude-code-linux-x64"]["2.1.310"]["files"]["claude"],
        "b73676430af12f97316a798b8371f81d640f90a58bdcb8f3ce56a835bb47df56"
    );
    assert_eq!(
        inventory["packages"]["claude-code-darwin-arm64"]["2.1.310"]["files"]["claude"],
        "ffb64331bc0ee1b69efb7c2c428bd3ab516bc31b75bf60886b356c97c1c25ffd"
    );
}

#[test]
fn hook_and_stream_markers_hold_and_no_hop_is_a_selected_change() {
    let inventory = fixture(INVENTORY);
    let markers = fixture(MARKERS);
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
    assert_eq!(
        response["selected_behavior_boundary"]["permission_mode_expected_on_init"],
        "default"
    );
    assert_eq!(response["selected_behavior_boundary"]["safe_mode"], true);
    assert_eq!(headless["selected_behavior_boundary"]["safe_mode"], false);
    let scanned = published_after(294);
    assert_eq!(markers["versions_scanned"], json!(scanned));
    assert_eq!(
        markers["markers"],
        json!([
            "--input-format",
            "--output-format",
            "--permission-mode",
            "--setting-sources",
            "--strict-mcp-config",
            "--no-session-persistence",
            "--disable-slash-commands",
            "--safe-mode",
            "--mcp-config",
            "--include-hook-events",
            "--no-chrome",
            "--prompt-suggestions",
            "--tools",
            "stream-json",
            "permissionDecision",
            "stop_hook_active",
            "CLAUDE_CODE_STOP_HOOK_BLOCK_CAP",
            "parent_tool_use_id",
            "PreToolUse",
            "SubagentStop",
            "onFailure",
            "prompt.context",
            "session.start",
            "fs.ancestors",
            "instructionFiles",
            "AGENTS_NAMES",
            "CLAUDE_NAMES",
            "projectDirOf",
            "registerPlugin",
            "UserPromptSubmit",
            "updatedInput"
        ])
    );
    assert_eq!(
        markers["help_lines"],
        json!([
            "Output format (only works with --print): \"text\" (default), \"json\" (single result), or \"stream-json\" (realtime streaming)",
            "Input format (only works with --print): \"text\" (default), or \"stream-json\" (realtime streaming input)"
        ])
    );
    for version in &scanned {
        for package in ["claude-code-darwin-arm64", "claude-code-linux-x64"] {
            let row = &markers["versions"][version][package];
            assert_eq!(
                row["static_markers_observed"], markers["markers"],
                "{package} {version}"
            );
            assert_eq!(
                row["help_lines_observed"], markers["help_lines"],
                "{package} {version}"
            );
            assert_eq!(
                row["artifact_sha256"],
                inventory["packages"][package][version]["files"]["claude"]
            );
        }
    }
    let headless_hops = headless["per_hop_classification"].as_array().unwrap();
    let response_hops = response["per_hop_classification"].as_array().unwrap();
    assert_eq!(headless_hops.len(), scanned.len());
    assert_eq!(response_hops.len(), scanned.len() + 1);
    assert_eq!(response_hops[0]["hop"], "2.1.293_to_2.1.294");
    for (index, version) in scanned.iter().enumerate() {
        let previous = if index == 0 {
            "2.1.294".to_owned()
        } else {
            scanned[index - 1].clone()
        };
        let expected = format!("{previous}_to_{version}");
        for hop in [&headless_hops[index], &response_hops[index + 1]] {
            assert_eq!(hop["hop"], expected);
            assert_eq!(hop["selected_mapped_change"], false, "{expected}");
            assert_eq!(hop["decision"], "compatible-extension", "{expected}");
            assert!(!hop["release_note_review"].as_array().unwrap().is_empty());
        }
    }
    for hop in response_hops {
        assert_eq!(hop["selected_mapped_change"], false);
    }
    for (hops, classification) in [
        (headless_hops, "provider-lifecycle-tightening"),
        (response_hops, "outside-safe-mode-custom-hooks"),
    ] {
        let hop = hops
            .iter()
            .find(|hop| hop["hop"] == "2.1.303_to_2.1.304")
            .expect("the 2.1.304 hop is reviewed for each route");
        let note = hop["release_note_review"]
            .as_array()
            .unwrap()
            .iter()
            .find(|note| {
                note["surface"] == "plugin subagent continuation after Stop or ended input"
            })
            .expect("the lifecycle fix has its own route classification");
        assert_eq!(note["classification"], classification);
        assert_eq!(
            note["source"],
            "https://github.com/anthropics/claude-code/releases/tag/v2.1.304"
        );
    }
}

#[test]
fn selected_v2_control_flow_is_bound_to_exact_runtime_hashes() {
    let inventory = fixture(INVENTORY);
    let flow = fixture(CONTROL_FLOW);
    let headless = fixture(HEADLESS_PROTOCOL);
    assert_eq!(
        headless["selected_control_flow"],
        "selected-control-flow.json"
    );
    assert_eq!(flow["downloaded_artifacts_executed"], false);
    assert_eq!(flow["release_notes_are_not_compatibility_proof"], true);
    for package in ["claude-code-darwin-arm64", "claude-code-linux-x64"] {
        assert_eq!(
            flow["ceiling_artifacts"][package]["artifact_sha256"],
            inventory["packages"][package]["2.1.310"]["files"]["claude"]
        );
    }
    let darwin = &flow["v2_selected_path_at_2_1_310"]["claude-code-darwin-arm64"];
    let linux = &flow["v2_selected_path_at_2_1_310"]["claude-code-linux-x64"];
    assert!(
        darwin["prompt_ok_false_blocks"]
            .as_str()
            .unwrap()
            .starts_with("if(!gt.data.ok){")
    );
    assert!(
        linux["prompt_ok_false_blocks"]
            .as_str()
            .unwrap()
            .starts_with("if(!mt.data.ok){")
    );
    for platform in [darwin, linux] {
        assert!(
            platform["prompt_ok_false_blocks"]
                .as_str()
                .unwrap()
                .contains("outcome:\"blocking\"")
        );
        assert!(
            platform["prompt_ok_false_blocks"]
                .as_str()
                .unwrap()
                .contains("continueOnBlock!==!0")
        );
        assert_eq!(
            platform["prompt_schema_required_ok"],
            "required:[\"ok\"],additionalProperties:!1"
        );
        assert!(
            platform["pretooluse_deny_before_dispatch"]
                .as_str()
                .unwrap()
                .contains("Execution stopped by PreToolUse hook")
        );
        assert!(
            platform["pretooluse_deny_before_dispatch"]
                .as_str()
                .unwrap()
                .contains("is_error:!0")
        );
        assert!(
            platform["rewrite_recheck_push"]
                .as_str()
                .unwrap()
                .contains("hookRewrites.push")
        );
        assert!(
            platform["rewrite_recheck_ask_to_deny"]
                .as_str()
                .unwrap()
                .contains("hookUpdatedInput:")
        );
        assert!(
            platform["rewrite_recheck_ask_to_deny"]
                .as_str()
                .unwrap()
                .contains("behavior===\"ask\"?{behavior:\"deny\"")
        );
        assert!(
            platform["rewrite_ask_rule_const"]
                .as_str()
                .unwrap()
                .contains("ask rule on hook-rewritten input")
        );
        assert!(
            platform["rewrite_ask_rule_const"]
                .as_str()
                .unwrap()
                .contains("type:\"asyncAgent\"")
        );
        assert!(
            platform["stop_hook_active_input"]
                .as_str()
                .unwrap()
                .contains("hook_event_name:\"SubagentStop\",stop_hook_active:s")
        );
        assert!(
            platform["stop_hook_active_input"]
                .as_str()
                .unwrap()
                .contains("hook_event_name:\"Stop\",stop_hook_active:s")
        );
        assert!(
            platform["stop_hook_active_reentry"]
                .as_str()
                .unwrap()
                .contains("stopHookActive:!0")
        );
        assert!(
            platform["stop_hook_active_reentry"]
                .as_str()
                .unwrap()
                .contains("transition:{reason:\"stop_hook_blocking\"}")
        );
        assert!(
            platform["stop_block_cap_default_8"]
                .as_str()
                .unwrap()
                .contains("CLAUDE_CODE_STOP_HOOK_BLOCK_CAP??8")
        );
        assert!(
            platform["stop_block_cap_default_8"]
                .as_str()
                .unwrap()
                .contains("check stop_hook_active")
        );
    }
    assert_eq!(
        flow["nested_claude_md_prefix"]["official_release_note_version"],
        "2.1.309"
    );
    assert_eq!(
        flow["nested_claude_md_prefix"]["official_release_note_url"],
        "https://github.com/anthropics/claude-code/releases/tag/v2.1.309"
    );
    assert_eq!(
        flow["nested_claude_md_prefix"]["embedded_release_note_is_selected_path_js"],
        false
    );
    assert_eq!(
        flow["nested_claude_md_prefix"]["present_as_embedded_release_note_string_at_2_1_309"],
        false
    );
    assert_eq!(
        flow["nested_claude_md_prefix"]["present_as_embedded_release_note_string_at_2_1_310"],
        true
    );
}

#[test]
fn hop_comparison_records_299_and_300_fail_closed_js() {
    let inventory = fixture(INVENTORY);
    let flow = fixture(CONTROL_FLOW);
    let hops = &flow["hop_comparison"];
    assert_eq!(hops["plugin_fail_closed_first_present"], "2.1.299");
    assert_eq!(hops["reserved_key_first_present"], "2.1.300");
    assert_eq!(
        hops["selected_js_keys"],
        json!([
            "prompt_ok_false_blocks",
            "pretooluse_deny_before_dispatch",
            "rewrite_recheck_push",
            "rewrite_recheck_ask_to_deny",
            "rewrite_ask_rule_const",
            "stop_hook_active_input",
            "stop_hook_active_reentry",
            "stop_block_cap_default_8",
            "plugin_fail_closed",
            "reserved_key_deny"
        ])
    );
    let compared = hops["compared_versions"].as_array().unwrap();
    assert_eq!(compared.first().unwrap(), "2.1.294");
    assert_eq!(compared.last().unwrap(), "2.1.310");
    for version in compared {
        let version = version.as_str().unwrap();
        for package in ["claude-code-darwin-arm64", "claude-code-linux-x64"] {
            let row = &hops["per_version"][version][package];
            assert_eq!(
                row["artifact_sha256"], inventory["packages"][package][version]["files"]["claude"],
                "{package} {version}"
            );
            let js = &row["selected_js"];
            let prompt = js["prompt_ok_false_blocks"].as_str().unwrap();
            assert!(
                prompt.contains("outcome:\"blocking\""),
                "{package} {version}"
            );
            assert!(
                prompt.contains("continueOnBlock!==!0"),
                "{package} {version}"
            );
            assert!(
                js["pretooluse_deny_before_dispatch"]
                    .as_str()
                    .unwrap()
                    .contains("Execution stopped by PreToolUse hook"),
                "{package} {version}"
            );
            assert!(
                js["rewrite_recheck_push"]
                    .as_str()
                    .unwrap()
                    .contains("hookRewrites.push"),
                "{package} {version}"
            );
            let rewrite = js["rewrite_recheck_ask_to_deny"].as_str().unwrap();
            assert!(rewrite.contains("hookUpdatedInput:"), "{package} {version}");
            assert!(
                rewrite.contains("behavior===\"ask\"?{behavior:\"deny\""),
                "{package} {version}"
            );
            assert!(
                js["rewrite_ask_rule_const"]
                    .as_str()
                    .unwrap()
                    .contains("ask rule on hook-rewritten input"),
                "{package} {version}"
            );
            let stop_input = js["stop_hook_active_input"].as_str().unwrap();
            assert!(
                stop_input.contains("hook_event_name:\"SubagentStop\",stop_hook_active:s"),
                "{package} {version}"
            );
            assert!(
                stop_input.contains("hook_event_name:\"Stop\",stop_hook_active:s"),
                "{package} {version}"
            );
            let reentry = js["stop_hook_active_reentry"].as_str().unwrap();
            assert!(reentry.contains("stopHookActive:!0"), "{package} {version}");
            assert!(
                reentry.contains("transition:{reason:\"stop_hook_blocking\"}"),
                "{package} {version}"
            );
            assert!(
                js["stop_block_cap_default_8"]
                    .as_str()
                    .unwrap()
                    .contains("CLAUDE_CODE_STOP_HOOK_BLOCK_CAP??8"),
                "{package} {version}"
            );
            let patch: u32 = version.split('.').nth(2).unwrap().parse().unwrap();
            if patch >= 299 {
                let fail_closed = js["plugin_fail_closed"].as_str().unwrap();
                assert!(
                    fail_closed.contains("classic.PreToolUse"),
                    "{package} {version}"
                );
                assert!(
                    fail_closed.contains(
                        "this tool input could not be given to a hook that checks it, so the tool was not run"
                    ),
                    "{package} {version}"
                );
            } else {
                assert!(js["plugin_fail_closed"].is_null(), "{package} {version}");
            }
            if patch >= 300 {
                let reserved = js["reserved_key_deny"].as_str().unwrap();
                assert!(
                    reserved.contains("refusedBy:\"reserved_key\""),
                    "{package} {version}"
                );
                assert!(
                    reserved.contains("The tool was not run."),
                    "{package} {version}"
                );
                assert!(
                    reserved.contains("return{ok:!1,deny:"),
                    "{package} {version}"
                );
            } else {
                assert!(js["reserved_key_deny"].is_null(), "{package} {version}");
            }
        }
    }
    for package in ["claude-code-darwin-arm64", "claude-code-linux-x64"] {
        let ceiling = &flow["v2_selected_path_at_2_1_310"][package];
        let hop = &hops["per_version"]["2.1.310"][package]["selected_js"];
        for key in [
            "rewrite_recheck_ask_to_deny",
            "rewrite_ask_rule_const",
            "stop_hook_active_input",
            "stop_hook_active_reentry",
        ] {
            assert_eq!(ceiling[key], hop[key], "{package} {key}");
        }
        let fail_closed = flow["hop_2_1_299_fail_closed_at_2_1_310"][package]
            .as_str()
            .unwrap();
        assert!(fail_closed.contains("classic.PreToolUse"));
        assert!(fail_closed.contains(
            "this tool input could not be given to a hook that checks it, so the tool was not run"
        ));
        let reserved = flow["hop_2_1_300_reserved_key_at_2_1_310"][package]
            .as_str()
            .unwrap();
        assert!(reserved.contains("refusedBy:\"reserved_key\""));
        assert!(reserved.contains("The tool was not run."));
        assert!(reserved.contains("return{ok:!1,deny:"));
    }
}

#[test]
fn response_only_isolation_is_frozen_per_binary() {
    let inventory = fixture(INVENTORY);
    let isolation = fixture(ISOLATION);
    let response = fixture(RESPONSE_PROTOCOL);
    assert_eq!(
        response["isolation_evidence"],
        "response-only-isolation.json"
    );
    assert_eq!(isolation["downloaded_artifacts_executed"], false);
    assert_eq!(isolation["easel_first_present"], "2.1.302");
    let help = isolation["safe_mode_option_at_2_1_310"]["claude-code-darwin-arm64"]
        .as_str()
        .unwrap();
    assert_eq!(
        isolation["safe_mode_option_at_2_1_310"]["claude-code-linux-x64"],
        help
    );
    assert!(help.contains(".option(\"--safe-mode\""));
    assert!(help.contains("installed plugins, hooks, MCP servers"));
    assert!(help.contains("built-in tools and plugins, and permissions work normally"));
    assert!(help.contains("Sets CLAUDE_CODE_SAFE_MODE=1."));
    assert_eq!(
        isolation["safe_mode_filter_at_2_1_310"]["claude-code-darwin-arm64"],
        "function Djn(e){if(!Mr())return[...e];return t(\"Safe mode: installed plugins are disabled, none of their hooks or hooks modules load; built-in plugins load regardless\"),e.filter((n)=>wf(n.source))}"
    );
    assert_eq!(
        isolation["safe_mode_filter_at_2_1_310"]["claude-code-linux-x64"],
        "function i1n(e){if(!Mr())return[...e];return t(\"Safe mode: installed plugins are disabled, none of their hooks or hooks modules load; built-in plugins load regardless\"),e.filter((n)=>vp(n.source))}"
    );
    assert_eq!(
        isolation["builtin_predicate_at_2_1_310"]["claude-code-darwin-arm64"],
        json!({
            "constant": "Ns=\"builtin\"",
            "predicate": "function wf(e){return e.endsWith(`@${Ns}`)}"
        })
    );
    assert_eq!(
        isolation["builtin_predicate_at_2_1_310"]["claude-code-linux-x64"],
        json!({
            "constant": "Ns=\"builtin\"",
            "predicate": "function vp(e){return e.endsWith(`@${Ns}`)}"
        })
    );
    assert_eq!(
        isolation["permission_mode_default_mapper_at_2_1_310"]["claude-code-darwin-arm64"],
        "function Qx(e){return ph(e)??\"default\"}"
    );
    assert_eq!(
        isolation["permission_mode_default_mapper_at_2_1_310"]["claude-code-linux-x64"],
        "function Xx(e){return fh(e)??\"default\"}"
    );
    assert_eq!(
        isolation["permission_mode_treats_undefined_as_default_at_2_1_310"]["claude-code-darwin-arm64"],
        "function Ldi(e){return e===\"default\"||e===void 0}"
    );
    assert_eq!(
        isolation["permission_mode_treats_undefined_as_default_at_2_1_310"]["claude-code-linux-x64"],
        "function Zli(e){return e===\"default\"||e===void 0}"
    );
    for package in ["claude-code-darwin-arm64", "claude-code-linux-x64"] {
        let init = isolation["init_fields_at_2_1_310"][package]
            .as_str()
            .unwrap();
        assert!(init.contains("subtype:\"init\""));
        assert!(init.contains("tools:e.tools.map"));
        assert!(init.contains("mcp_servers:e.mcpClients.map"));
        assert!(init.contains("permissionMode:ql(e.permissionMode)"));
        assert_eq!(
            isolation["ceiling_artifacts"][package]["artifact_sha256"],
            inventory["packages"][package]["2.1.310"]["files"]["claude"]
        );
    }
    let nine = isolation["research_378_nine_remain"]
        .as_array()
        .unwrap()
        .iter()
        .map(|name| name.as_str().unwrap())
        .collect::<Vec<_>>();
    let compared = isolation["compared_versions"].as_array().unwrap();
    assert_eq!(compared.first().unwrap(), "2.1.293");
    assert_eq!(compared.last().unwrap(), "2.1.310");
    assert_eq!(
        compared.len(),
        isolation["per_version"].as_object().unwrap().len()
    );
    for version in compared {
        let version_name = version.as_str().unwrap();
        let patch: u32 = version_name.split('.').nth(2).unwrap().parse().unwrap();
        let expected = if patch >= 302 {
            &isolation["cc_plugin_ids_from_2_1_302"]
        } else {
            &isolation["cc_plugin_ids_through_2_1_301"]
        };
        for package in ["claude-code-darwin-arm64", "claude-code-linux-x64"] {
            let row = &isolation["per_version"][version_name][package];
            assert_eq!(row["cc_plugin_ids"], *expected, "{package} {version_name}");
            for name in &nine {
                assert!(
                    row["cc_plugin_ids"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .any(|plugin| plugin == name),
                    "{package} {version_name} missing {name}"
                );
            }
            assert_eq!(
                row["artifact_sha256"],
                inventory["packages"][package][version_name]["files"]["claude"],
                "{package} {version_name}"
            );
            assert!(
                row["safe_mode_option"]
                    .as_str()
                    .unwrap()
                    .contains(".option(\"--safe-mode\""),
                "{package} {version_name}"
            );
            assert!(
                row["safe_mode_option"]
                    .as_str()
                    .unwrap()
                    .contains("Sets CLAUDE_CODE_SAFE_MODE=1."),
                "{package} {version_name}"
            );
            assert!(
                row["safe_mode_filter"]
                    .as_str()
                    .unwrap()
                    .contains("Safe mode: installed plugins are disabled"),
                "{package} {version_name}"
            );
            assert!(
                row["safe_mode_filter"]
                    .as_str()
                    .unwrap()
                    .contains("e.filter((n)=>"),
                "{package} {version_name}"
            );
            let init = row["init_fields"].as_str().unwrap();
            assert!(
                init.contains("subtype:\"init\""),
                "{package} {version_name}"
            );
            assert!(
                init.contains("tools:e.tools.map"),
                "{package} {version_name}"
            );
            assert!(
                init.contains("mcp_servers:e.mcpClients.map"),
                "{package} {version_name}"
            );
            assert!(
                init.contains("permissionMode:") && init.contains("(e.permissionMode)"),
                "{package} {version_name}"
            );
            assert!(
                row["permission_mode_default_mapper"]
                    .as_str()
                    .unwrap()
                    .contains("??\"default\""),
                "{package} {version_name}"
            );
            assert!(
                row["permission_mode_treats_undefined_as_default"]
                    .as_str()
                    .unwrap()
                    .contains("e===\"default\"||e===void 0"),
                "{package} {version_name}"
            );
            assert!(
                row["builtin_constant"]
                    .as_str()
                    .unwrap()
                    .ends_with("=\"builtin\""),
                "{package} {version_name}"
            );
            assert!(
                row["builtin_predicate"]
                    .as_str()
                    .unwrap()
                    .contains("return e.endsWith(`@${"),
                "{package} {version_name}"
            );
            assert!(row.get("safe_mode_option_present").is_none());
            assert!(row.get("safe_mode_filter_log_present").is_none());
            assert!(row.get("init_subtype_present").is_none());
            assert!(row.get("permissionMode_default_literal_present").is_none());
        }
    }
    for package in ["claude-code-darwin-arm64", "claude-code-linux-x64"] {
        let hop = &isolation["per_version"]["2.1.310"][package];
        assert_eq!(
            hop["safe_mode_option"],
            isolation["safe_mode_option_at_2_1_310"][package]
        );
        assert_eq!(
            hop["safe_mode_filter"],
            isolation["safe_mode_filter_at_2_1_310"][package]
        );
        assert_eq!(
            hop["init_fields"],
            isolation["init_fields_at_2_1_310"][package]
        );
        assert_eq!(
            hop["permission_mode_default_mapper"],
            isolation["permission_mode_default_mapper_at_2_1_310"][package]
        );
        assert_eq!(
            hop["permission_mode_treats_undefined_as_default"],
            isolation["permission_mode_treats_undefined_as_default_at_2_1_310"][package]
        );
        assert_eq!(
            hop["builtin_constant"],
            isolation["builtin_predicate_at_2_1_310"][package]["constant"]
        );
        assert_eq!(
            hop["builtin_predicate"],
            isolation["builtin_predicate_at_2_1_310"][package]["predicate"]
        );
    }
}
