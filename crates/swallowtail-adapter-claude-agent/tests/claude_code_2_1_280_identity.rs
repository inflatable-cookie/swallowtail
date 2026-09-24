use serde_json::Value;
use swallowtail_adapter_claude_agent::{
    CLAUDE_CODE_HEADLESS_BASELINE_VERSION, CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION,
    CLAUDE_CODE_RESPONSE_ONLY_BASELINE_VERSION, CLAUDE_CODE_RESPONSE_ONLY_DENIED_VERSIONS,
    CLAUDE_CODE_RESPONSE_ONLY_LATEST_QUALIFIED_VERSION, claude_code_headless_claim,
    claude_code_response_only_claim,
};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

const IDENTITY: &str = include_str!("fixtures/claude-code-2.1.280/identity.json");
const PROTOCOL: &str = include_str!("fixtures/claude-code-2.1.280/protocol.json");
const DIST_INVENTORY: &str = include_str!("fixtures/claude-code-2.1.280/dist-inventory.json");

fn json(value: &str) -> Value {
    serde_json::from_str(value).expect("frozen corpus JSON is valid")
}

fn strings(value: &Value) -> Vec<&str> {
    value
        .as_array()
        .expect("value is an array")
        .iter()
        .map(|value| value.as_str().expect("array value is text"))
        .collect()
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}

#[test]
fn identity_stops_before_any_claim_edit() {
    let identity = json(IDENTITY);
    let decision = &identity["identity_decision"];
    assert_eq!(identity["version"], "2.1.280");
    assert_eq!(identity["npm_channel"], "latest");
    assert_eq!(identity["npm_dist_tags"]["latest"], "2.1.280");
    assert_eq!(identity["npm_dist_tags"]["stable"], "2.1.267");
    assert_eq!(identity["previous_ceiling"], "2.1.278");
    assert_eq!(
        identity["published_stables_from_previous_ceiling"],
        json(r#"["2.1.280"]"#)
    );
    assert_eq!(
        identity["unpublished_between_ceiling_and_official"],
        json(r#"["2.1.279"]"#)
    );
    assert_eq!(identity["unpublished_2_1_279"], true);
    assert_eq!(identity["first_unpublished_after_official"], "2.1.281");
    assert_eq!(
        identity["github_tag_commit"],
        "56f36532530f88b572854538d685fcf781141e8c"
    );
    assert_eq!(identity["github_tag_annotated"], false);
    assert_eq!(identity["downloaded_official_binaries_executed"], false);
    assert_eq!(identity["host"]["not_installed"], true);
    assert_eq!(decision["shape"], "stop");
    assert_eq!(
        decision["stop_reason"],
        "response-only-safe-mode-plugin-hook-filter"
    );
    assert_eq!(decision["raise_latest_qualified"], false);
    assert_eq!(decision["claim_edited"], false);
    assert_eq!(decision["qualify_published_intermediates"], false);
    assert_eq!(decision["keep_allow_unverified"], true);
    assert_eq!(decision["keep_headless_baseline"], "2.1.220");
    assert_eq!(decision["keep_response_only_baseline"], "2.1.227");
    assert_eq!(decision["keep_headless_latest_qualified"], "2.1.278");
    assert_eq!(decision["keep_response_only_latest_qualified"], "2.1.278");
    assert_eq!(decision["add_2_1_279_to_deny_list"], false);
    assert_eq!(decision["flatten_to_claude_agent_acp"], false);
    assert_eq!(decision["flatten_to_claude_agent_sdk"], false);
    assert_eq!(decision["new_milestone"], false);
    assert_eq!(decision["widen_watcher_authorization"], false);
    assert_eq!(decision["provider_prompt_sent"], false);
    assert_eq!(decision["host_install_changed"], false);
    assert_eq!(
        identity["reproduced_2_1_278"]["linux_x64_binary_sha256"],
        "5c4735937844e84f8a93306e841a5b0e12252909b07870f789b190468da147ab"
    );
    assert_eq!(
        identity["reproduced_2_1_278"]["darwin_arm64_binary_sha256"],
        "bd245662fb8a0e321b3bf133e930371d6563c387527885f30b2613aef3ba14d6"
    );
    assert_eq!(
        identity["linux_x64_binary_sha256"],
        "1e08503dbdf3c2cb0d706d32f3408277388d1c76ef108673e8fe42c1b322925b"
    );
    assert_eq!(
        identity["darwin_arm64_binary_sha256"],
        "387a5c5dcdbb815085edf0baf79591f9d8894efe922bceaf3d75b1b08055229d"
    );
}

#[test]
fn safe_mode_plugin_hook_filter_is_the_selected_surface_change() {
    let protocol = json(PROTOCOL);
    let hooks = &protocol["safe_mode_plugin_hooks"];
    assert_eq!(protocol["selected_mapped_subset_unchanged"], false);
    assert_eq!(protocol["identity_decision_shape"], "stop");
    assert_eq!(
        protocol["changed_selected_flags"],
        json(r#"["--safe-mode"]"#)
    );
    assert_eq!(protocol["headless_passes_safe_mode"], false);
    assert_eq!(protocol["response_only_passes_safe_mode"], true);
    assert_eq!(protocol["sdk_tools_d_ts_byte_identical"], true);
    assert_eq!(protocol["claim_edited"], false);
    assert_eq!(hooks["platforms_agree"], true);
    assert_eq!(
        hooks["2.1.278_linux_early_return"],
        r#"if(kr()){t("Safe mode: skipping plugin hook registration");return}"#
    );
    assert_eq!(
        hooks["2.1.280_linux_filter"],
        r#"function lar(e){if(!Er())return[...e];return t("Safe mode: installed plugins are disabled, none of their hooks or hooks modules load; built-in plugins load regardless"),e.filter((n)=>jO(n.source))}"#
    );
    assert_eq!(
        hooks["2.1.280_linux_builtin_predicate"],
        "function jO(e){return e.endsWith(`@${ou}`)}"
    );
    assert_eq!(hooks["2.1.280_linux_builtin_constant"], r#"ou="builtin""#);
    assert_eq!(
        hooks["2.1.280_darwin_filter"],
        r#"function zir(e){if(!vr())return[...e];return t("Safe mode: installed plugins are disabled, none of their hooks or hooks modules load; built-in plugins load regardless"),e.filter((n)=>XO(n.source))}"#
    );
    assert_eq!(
        hooks["2.1.280_darwin_builtin_predicate"],
        "function XO(e){return e.endsWith(`@${iu}`)}"
    );
    assert_eq!(hooks["2.1.280_darwin_builtin_constant"], r#"iu="builtin""#);
    assert_eq!(
        strings(&protocol["input_format_choices"]),
        ["text", "stream-json"]
    );
    assert_eq!(
        strings(&protocol["output_format_choices"]),
        ["text", "json", "stream-json"]
    );
    assert_eq!(
        strings(&protocol["effort_choices"]),
        ["low", "medium", "high", "xhigh", "max"]
    );
    assert_eq!(
        strings(&protocol["permission_mode_wire_values"]),
        [
            "acceptEdits",
            "auto",
            "bypassPermissions",
            "default",
            "dontAsk",
            "plan"
        ]
    );
    assert_eq!(
        strings(&protocol["stream_json_init_keys"]),
        [
            "type",
            "subtype",
            "cwd",
            "session_id",
            "tools",
            "mcp_servers",
            "model",
            "permissionMode",
            "slash_commands",
            "claude_code_version",
        ]
    );
    let help_278 = protocol["safe_mode_help"]["2.1.278"]
        .as_str()
        .expect("help text");
    let help_280 = protocol["safe_mode_help"]["2.1.280_concatenated"]
        .as_str()
        .expect("help text");
    assert!(help_278.contains("skills, plugins, hooks"));
    assert!(help_278.contains("built-in tools, and permissions"));
    assert!(help_280.contains("skills, installed plugins, hooks"));
    assert!(help_280.contains("built-in tools and plugins, and permissions"));
}

#[test]
fn dist_inventory_changes_only_version_pins_and_the_binary() {
    let inventory = json(DIST_INVENTORY);
    assert_eq!(inventory["compared"], json(r#"["2.1.278","2.1.280"]"#));
    assert_eq!(inventory["package_file_counts"]["wrapper-2.1.278"], 7);
    assert_eq!(inventory["package_file_counts"]["wrapper-2.1.280"], 7);
    assert_eq!(inventory["package_file_counts"]["linux-x64-2.1.280"], 4);
    assert_eq!(inventory["package_file_counts"]["darwin-arm64-2.1.280"], 4);
    assert_eq!(
        strings(&inventory["from_2_1_278_to_2_1_280_wrapper"]["changed"]),
        ["package.json"]
    );
    assert_eq!(
        strings(&inventory["from_2_1_278_to_2_1_280_wrapper"]["identical"]),
        [
            "LICENSE.md",
            "README.md",
            "bin/claude.exe",
            "cli-wrapper.cjs",
            "install.cjs",
            "sdk-tools.d.ts"
        ]
    );
    for platform in ["darwin_arm64", "linux_x64"] {
        let key = format!("from_2_1_278_to_2_1_280_{platform}");
        assert_eq!(
            strings(&inventory[&key]["changed"]),
            ["claude", "package.json"]
        );
        assert_eq!(
            strings(&inventory[&key]["identical"]),
            ["LICENSE.md", "README.md"]
        );
        assert_eq!(strings(&inventory[&key]["added"]), Vec::<&str>::new());
        assert_eq!(strings(&inventory[&key]["removed"]), Vec::<&str>::new());
    }
    assert_eq!(
        inventory["hashes"]["wrapper.sdk-tools.d.ts"]["2.1.278"],
        inventory["hashes"]["wrapper.sdk-tools.d.ts"]["2.1.280"]
    );
    assert_ne!(
        inventory["hashes"]["linux-x64.claude"]["2.1.278"],
        inventory["hashes"]["linux-x64.claude"]["2.1.280"]
    );
    assert_eq!(
        inventory["published_hop_tarball_sha256"]["wrapper"]["2.1.278"],
        "08c6dfcf3dafcfd30e09b2926c596e274f0fa20844a5801ada7f1c8e6227157e"
    );
    assert_eq!(
        inventory["published_hop_tarball_sha256"]["wrapper"]["2.1.280"],
        "1326e6b8cf00404fc3f9bd101d806b3fdec9588264e5a1aa8d98d1f7afe50170"
    );
}

#[test]
fn production_ceilings_stay_at_2_1_278() {
    let identity = json(IDENTITY);
    let decision = &identity["identity_decision"];
    assert_eq!(
        CLAUDE_CODE_HEADLESS_BASELINE_VERSION,
        decision["keep_headless_baseline"]
            .as_str()
            .expect("baseline")
    );
    assert_eq!(
        CLAUDE_CODE_RESPONSE_ONLY_BASELINE_VERSION,
        decision["keep_response_only_baseline"]
            .as_str()
            .expect("baseline")
    );
    assert_eq!(CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION, "2.1.278");
    assert_eq!(
        CLAUDE_CODE_RESPONSE_ONLY_LATEST_QUALIFIED_VERSION,
        "2.1.278"
    );
    assert_eq!(
        CLAUDE_CODE_RESPONSE_ONLY_DENIED_VERSIONS,
        [
            "2.1.244", "2.1.249", "2.1.253", "2.1.254", "2.1.255", "2.1.256", "2.1.262", "2.1.264",
        ]
    );
    let headless = claude_code_headless_claim();
    let response = claude_code_response_only_claim();
    assert!(headless.supports(&version("2.1.278")));
    assert!(response.supports(&version("2.1.278")));
    for later in ["2.1.279", "2.1.280", "2.1.281"] {
        assert!(
            matches!(
                headless.assess(&version(later)),
                InterfaceCompatibilityAssessment::UnverifiedNewer(_)
            ),
            "{later}"
        );
        assert!(
            matches!(
                response.assess(&version(later)),
                InterfaceCompatibilityAssessment::UnverifiedNewer(_)
            ),
            "{later}"
        );
    }
    assert!(!headless.permits(&version("2.1.244")));
    assert!(!response.permits(&version("2.1.244")));
}
