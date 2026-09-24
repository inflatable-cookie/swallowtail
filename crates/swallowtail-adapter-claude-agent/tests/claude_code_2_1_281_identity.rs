use serde_json::{Value, json};
use swallowtail_adapter_claude_agent::{
    CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION,
    CLAUDE_CODE_RESPONSE_ONLY_LATEST_QUALIFIED_VERSION, claude_code_headless_claim,
    claude_code_response_only_claim,
};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

const IDENTITY: &str = include_str!("fixtures/claude-code-2.1.281/identity.json");
const INVENTORY: &str = include_str!("fixtures/claude-code-2.1.281/dist-inventory.json");
const HOOKS: &str = include_str!("fixtures/claude-code-2.1.281/builtin-hook-ledger.json");
const PROTOCOL: &str = include_str!("fixtures/claude-code-2.1.281/protocol.json");

fn fixture(value: &str) -> Value {
    serde_json::from_str(value).expect("frozen JSON is valid")
}

#[test]
fn official_hops_and_claim_stop_are_explicit() {
    let identity = fixture(IDENTITY);
    assert_eq!(identity["official_latest_at_observation"], "2.1.281");
    assert_eq!(identity["published_hops"], json!(["2.1.280", "2.1.281"]));
    assert_eq!(identity["unpublished_between"], json!(["2.1.279"]));
    assert_eq!(identity["branch"], "neither_escalate");
    assert_eq!(identity["downloaded_binaries_executed"], false);
    assert_eq!(CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION, "2.1.278");
    assert_eq!(CLAUDE_CODE_RESPONSE_ONLY_LATEST_QUALIFIED_VERSION, "2.1.278");
    for claim in [claude_code_headless_claim(), claude_code_response_only_claim()] {
        for newer in ["2.1.280", "2.1.281"] {
            assert!(matches!(
                claim.assess(&InterfaceVersion::new(newer).unwrap()),
                InterfaceCompatibilityAssessment::UnverifiedNewer(_)
            ));
        }
        assert!(matches!(
            claim.assess(&InterfaceVersion::new("2.1.279").unwrap()),
            InterfaceCompatibilityAssessment::UnverifiedNewer(_)
        ));
    }
}

#[test]
fn shipped_file_inventory_and_binary_identity_are_frozen() {
    let inventory = fixture(INVENTORY);
    assert_eq!(inventory["compared"], json!(["2.1.278", "2.1.280", "2.1.281"]));
    for platform in ["claude-code-darwin-arm64", "claude-code-linux-x64"] {
        for version in ["2.1.278", "2.1.280", "2.1.281"] {
            let files = inventory["packages"][platform][version]["files"]
                .as_object()
                .expect("platform file map");
            let names = files.keys().map(String::as_str).collect::<Vec<_>>();
            assert_eq!(names, ["LICENSE.md", "README.md", "claude", "package.json"]);
        }
        for hop in ["2.1.278_to_2.1.280", "2.1.280_to_2.1.281"] {
            assert_eq!(inventory["file_delta"][platform][hop]["changed"], json!(["claude", "package.json"]));
            assert_eq!(inventory["file_delta"][platform][hop]["added"], json!([]));
            assert_eq!(inventory["file_delta"][platform][hop]["removed"], json!([]));
            assert_eq!(inventory["file_delta"][platform][hop]["identical"], json!(["LICENSE.md", "README.md"]));
        }
    }
    let wrapper_files = inventory["packages"]["claude-code"]["2.1.281"]["files"].as_object().unwrap();
    assert_eq!(wrapper_files.keys().map(String::as_str).collect::<Vec<_>>(), ["LICENSE.md", "README.md", "bin/claude.exe", "cli-wrapper.cjs", "install.cjs", "package.json", "sdk-tools.d.ts"]);
    assert_eq!(inventory["file_delta"]["claude-code"]["2.1.278_to_2.1.280"]["changed"], json!(["package.json"]));
    assert_eq!(inventory["file_delta"]["claude-code"]["2.1.280_to_2.1.281"]["changed"], json!(["package.json", "sdk-tools.d.ts"]));
    assert_eq!(inventory["packages"]["claude-code-darwin-arm64"]["2.1.281"]["files"]["claude"], "a922981f6f3b55a251ef9f9dbaa0621a5f99cbcb5ca67f8a797476ccfc83f626");
    assert_eq!(inventory["packages"]["claude-code-linux-x64"]["2.1.281"]["files"]["claude"], "56fe3da88458465fb27d7e9299dddb3fead55750fb9c2de795f233b5eea6dce1");
}

#[test]
fn builtin_hook_set_and_decisive_prompt_context_are_frozen() {
    let hooks = fixture(HOOKS);
    let expected = [
        "sec-default", "agents-md", "telemetry", "plugin-authoring", "tips", "mermaid",
        "responsive-mode", "diff", "claude-test",
    ];
    let events = [
        json!(["agent.offer", "agent.spawn", "attribution.text", "command.describe", "prompt.context", "prompt.section", "settings.read", "skill.prompt", "tool.describe", "tool.list", "tool.register"]),
        json!(["agent.spawn", "prompt.context", "session.start", "tool.call"]),
        json!(["engine.create", "session.end", "session.start"]),
        json!([]),
        json!(["session.start"]),
        json!(["ui.render"]),
        json!(["prompt.section", "prompt.submit", "ui.render"]),
        json!(["command.run", "prompt.submit", "session.start", "tool.call", "ui.close", "ui.focus", "ui.render", "ui.scroll"]),
        json!(["command.run", "skill.prompt"]),
    ];
    for version in ["2.1.280", "2.1.281"] {
        let rows = hooks["versions"][version].as_array().expect("hook rows");
        assert_eq!(rows.iter().map(|row| row["name"].as_str().unwrap()).collect::<Vec<_>>(), expected);
        assert!(rows.iter().all(|row| row["registered_in_darwin_arm64"] == true && row["registered_in_linux_x64"] == true));
        for (row, expected_events) in rows.iter().zip(&events) {
            assert_eq!(&row["hook_events"], expected_events);
            assert_eq!(row["linux_embedded_module_sha256"].as_str().unwrap().len(), 64);
        }
    }
    assert_eq!(hooks["versions"]["2.1.280"][1]["linux_embedded_module_sha256"], "fd7b0d390f79bcfb3c49e2594053a202cc38a4f510f8f9d31d40cfe2e53820d3");
    assert_eq!(hooks["versions"]["2.1.281"][1]["linux_embedded_module_sha256"], "799d2032c65cf657de0076c93ad77cc79db9a08c3237ff1b6fd967f630a1621f");
    let protocol = fixture(PROTOCOL);
    assert_eq!(protocol["agents_md_default_enabled"], json!({"2.1.280": false, "2.1.281": true}));
    assert_eq!(protocol["enabled_plugins_builtin_override_from_cli_settings_ignored"], true);
    assert_eq!(protocol["disable_all_hooks_user_setting_keeps_builtin_plugins"], true);
    assert_eq!(protocol["branch"], "neither");
}
