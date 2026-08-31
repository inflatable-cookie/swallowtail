use serde_json::Value;
use swallowtail_adapter_claude_agent::CLAUDE_CODE_HEADLESS_AXIS;

const WATCHER_TOOL_ADMISSION: &str =
    include_str!("fixtures/claude-code-2.1.251/watcher-tool-admission.json");

#[test]
fn watcher_tool_admission_evidence_rejects_the_tools_hypothesis() {
    let evidence: Value = serde_json::from_str(WATCHER_TOOL_ADMISSION)
        .expect("Claude Code 2.1.251 watcher admission corpus is valid JSON");

    assert_eq!(evidence["version"], "2.1.251");
    assert_eq!(evidence["axis"], CLAUDE_CODE_HEADLESS_AXIS);
    assert_eq!(evidence["source"]["provider_prompt_sent"], false);
    assert_eq!(evidence["source"]["credentials_used"], false);
    assert_eq!(
        evidence["source"]["host_help_sha256"],
        "5ff2e7a0bca8535fb9ec097fa0a21e9d6b735ed94104fa0d1f58ac73a841d52d"
    );
    assert_eq!(
        evidence["help_semantics"]["tools_scope"],
        "built-in-set-only"
    );
    assert_eq!(evidence["hypothesis"]["result"], "rejected");
    assert_eq!(evidence["hypothesis"]["production_command_change"], "none");
    assert_eq!(
        evidence["alternative_blocker"]["name"],
        "bare-authentication-path-unavailable"
    );
    assert_eq!(
        evidence["alternative_blocker"]["repair_authorized_by_card_026"],
        false
    );
}
