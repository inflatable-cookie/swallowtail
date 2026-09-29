use serde_json::{Value, json};
use swallowtail_adapter_claude_agent::{
    CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION,
    CLAUDE_CODE_RESPONSE_ONLY_LATEST_QUALIFIED_VERSION, claude_code_headless_claim,
    claude_code_response_only_claim,
};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

const IDENTITY: &str = include_str!("fixtures/claude-code-2.1.283/identity.json");
const INVENTORY: &str = include_str!("fixtures/claude-code-2.1.283/dist-inventory.json");
const PROTOCOL: &str = include_str!("fixtures/claude-code-2.1.283/protocol.json");

fn fixture(value: &str) -> Value {
    serde_json::from_str(value).expect("frozen JSON is valid")
}

#[test]
fn official_hops_and_compatible_extension_are_explicit() {
    let identity = fixture(IDENTITY);
    assert_eq!(identity["official_latest_at_observation"], "2.1.283");
    assert_eq!(identity["published_hops"], json!(["2.1.282", "2.1.283"]));
    assert_eq!(identity["unpublished_between"], json!([]));
    assert_eq!(identity["first_unpublished_after_latest"], "2.1.284");
    assert_eq!(identity["downloaded_binaries_executed"], false);
    assert_eq!(identity["provider_prompt_sent"], false);
    assert_eq!(identity["live_session"], false);
    assert_eq!(identity["host_install_changed"], false);
    assert_eq!(
        identity["claim_at_observation"]["headless_latest_qualified"],
        "2.1.281"
    );
    assert_eq!(
        identity["claim_at_observation"]["response_only_latest_qualified"],
        "2.1.281"
    );
    assert_eq!(
        identity["identity_decision"]["segment_shape"],
        "compatible-extension"
    );
    assert_eq!(identity["identity_decision"]["identity_stop"], "none");
    assert_eq!(
        identity["identity_decision"]["raise_headless_latest_qualified_to"],
        "2.1.283"
    );
    assert_eq!(
        identity["identity_decision"]["raise_response_only_latest_qualified_to"],
        "2.1.283"
    );
    assert_eq!(
        identity["identity_decision"]["synthetic_unverified_newer"],
        "2.1.284"
    );
    assert_eq!(identity["identity_decision"]["flatten_onto_claude_agent_acp"], false);
    assert_eq!(
        identity["identity_decision"]["flatten_onto_claude_agent_sdk"],
        false
    );
    assert_eq!(CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION, "2.1.283");
    assert_eq!(
        CLAUDE_CODE_RESPONSE_ONLY_LATEST_QUALIFIED_VERSION,
        "2.1.283"
    );
    let headless = claude_code_headless_claim();
    let response = claude_code_response_only_claim();
    for published in ["2.1.281", "2.1.282", "2.1.283"] {
        assert!(headless.supports(&InterfaceVersion::new(published).unwrap()));
        assert!(matches!(
            response.assess(&InterfaceVersion::new(published).unwrap()),
            InterfaceCompatibilityAssessment::Qualified(matched)
                if matched.behavior_revision().as_str()
                    == "claude-code.response-only.stream-json.v2"
        ));
    }
    assert!(!headless.permits(&InterfaceVersion::new("2.1.279").unwrap()));
    assert!(!response.permits(&InterfaceVersion::new("2.1.279").unwrap()));
    assert!(matches!(
        headless.assess(&InterfaceVersion::new("2.1.284").unwrap()),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert!(matches!(
        response.assess(&InterfaceVersion::new("2.1.284").unwrap()),
        InterfaceCompatibilityAssessment::UnverifiedNewer(newer)
            if newer.behavior_revision().as_str()
                == "claude-code.response-only.stream-json.v2"
    ));
}

#[test]
fn shipped_file_inventory_is_frozen() {
    let inventory = fixture(INVENTORY);
    assert_eq!(
        inventory["compared"],
        json!(["2.1.281", "2.1.282", "2.1.283"])
    );
    for version in ["2.1.281", "2.1.282", "2.1.283"] {
        assert_eq!(inventory["package_file_counts"]["claude-code"][version], 7);
        assert_eq!(
            inventory["package_file_counts"]["claude-code-darwin-arm64"][version],
            4
        );
        assert_eq!(
            inventory["package_file_counts"]["claude-code-linux-x64"][version],
            4
        );
    }
    assert_eq!(
        inventory["identical_through_2_1_281_2_1_282_2_1_283"]["claude-code"],
        json!([
            "LICENSE.md",
            "README.md",
            "bin/claude.exe",
            "cli-wrapper.cjs",
            "install.cjs",
            "sdk-tools.d.ts"
        ])
    );
    for platform in ["claude-code-darwin-arm64", "claude-code-linux-x64"] {
        assert_eq!(
            inventory["identical_through_2_1_281_2_1_282_2_1_283"][platform],
            json!(["LICENSE.md", "README.md"])
        );
        for hop in ["2.1.281_to_2.1.282", "2.1.282_to_2.1.283"] {
            assert_eq!(
                inventory["file_delta"][platform][hop]["changed"],
                json!(["claude", "package.json"])
            );
            assert_eq!(inventory["file_delta"][platform][hop]["added"], json!([]));
            assert_eq!(inventory["file_delta"][platform][hop]["removed"], json!([]));
            assert_eq!(
                inventory["file_delta"][platform][hop]["identical"],
                json!(["LICENSE.md", "README.md"])
            );
        }
    }
    for hop in ["2.1.281_to_2.1.282", "2.1.282_to_2.1.283"] {
        assert_eq!(
            inventory["file_delta"]["claude-code"][hop]["changed"],
            json!(["package.json"])
        );
        assert_eq!(inventory["file_delta"]["claude-code"][hop]["added"], json!([]));
        assert_eq!(
            inventory["file_delta"]["claude-code"][hop]["removed"],
            json!([])
        );
    }
    assert_eq!(
        inventory["hashes"]["claude-code/sdk-tools.d.ts"]["2.1.281"],
        inventory["hashes"]["claude-code/sdk-tools.d.ts"]["2.1.283"]
    );
    assert_eq!(
        inventory["packages"]["claude-code-darwin-arm64"]["2.1.281"]["files"]["claude"],
        "a922981f6f3b55a251ef9f9dbaa0621a5f99cbcb5ca67f8a797476ccfc83f626"
    );
    assert_eq!(
        inventory["packages"]["claude-code-linux-x64"]["2.1.281"]["files"]["claude"],
        "56fe3da88458465fb27d7e9299dddb3fead55750fb9c2de795f233b5eea6dce1"
    );
    assert_eq!(
        inventory["packages"]["claude-code-darwin-arm64"]["2.1.283"]["files"]["claude"],
        "d8cb1e5c79684cc12a8bfc813e3a2073406921b6245744b3009be3ab5651d21e"
    );
    assert_eq!(
        inventory["packages"]["claude-code-linux-x64"]["2.1.283"]["files"]["claude"],
        "1859583ce32920595c61ef868bee52e1b1594f7486db209935e01f1e5e804ae2"
    );
}

#[test]
fn selected_protocol_bounds_are_frozen() {
    let protocol = fixture(PROTOCOL);
    assert_eq!(protocol["selected_mapped_subset_unchanged"], true);
    assert_eq!(protocol["output_format_choices"], json!(["text", "json", "stream-json"]));
    assert_eq!(protocol["input_format_choices"], json!(["text", "stream-json"]));
    assert_eq!(
        protocol["effort_choices"],
        json!(["low", "medium", "high", "xhigh", "max"])
    );
    assert_eq!(
        protocol["permission_mode_wire_values"],
        json!([
            "acceptEdits",
            "auto",
            "bypassPermissions",
            "default",
            "dontAsk",
            "plan"
        ])
    );
    assert_eq!(protocol["headless_selected_permission_mode"], "plan");
    assert_eq!(protocol["headless_passes_safe_mode"], false);
    assert_eq!(protocol["response_only_passes_permission_mode"], false);
    assert_eq!(
        protocol["selected_response_only_args"],
        json!([
            "--tools",
            "",
            "--safe-mode",
            "--disable-slash-commands",
            "--no-chrome",
            "--prompt-suggestions",
            "false",
            "--mcp-config",
            "{\"mcpServers\":{}}",
            "--strict-mcp-config"
        ])
    );
    assert_eq!(protocol["agents_md_default_mode"], "claude-md-or-agents-md");
    assert_eq!(
        protocol["agents_md_hook_events"],
        json!(["session.start", "prompt.context", "agent.spawn", "tool.call"])
    );
    assert_eq!(protocol["agents_md_is_on_by_default"]["2.1.283"], true);
    assert_eq!(protocol["safe_mode_keeps_builtin_suffix"]["2.1.283"], true);
    assert_eq!(protocol["safe_mode_builtin_sentinel"], "builtin");
    assert_eq!(protocol["unmapped_new_flags"], json!(["--client-data-url"]));
    assert_eq!(
        protocol["unmapped_new_flag_present"]["--client-data-url"]["2.1.282"],
        false
    );
    assert_eq!(
        protocol["unmapped_new_flag_present"]["--client-data-url"]["2.1.283"],
        true
    );
    assert_eq!(protocol["unmapped_preexisting_flags"], json!(["--bare"]));
    assert_eq!(protocol["sdk_tools_d_ts_byte_identical"], true);
    assert_eq!(protocol["init_schema_keys_equal"], true);
    assert_eq!(protocol["provider_prompt_sent"], false);
    assert_eq!(protocol["downloaded_binaries_executed"], false);
}
