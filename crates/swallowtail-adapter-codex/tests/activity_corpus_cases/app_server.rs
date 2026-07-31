use super::support::{APP_SERVER, case, case_names, json_lines, methods};

#[test]
fn app_server_corpus_covers_rich_activity_and_disclosure_boundaries() {
    let cases = json_lines(APP_SERVER);
    let names = case_names(&cases);
    for required in [
        "item-lifecycle",
        "assistant-commentary",
        "assistant-final",
        "assistant-phase-unknown",
        "plan-replacement",
        "plan-item",
        "reasoning-summary",
        "raw-reasoning-excluded",
        "command-output",
        "file-change-and-turn-diff",
        "deprecated-file-output",
        "mcp-tool",
        "dynamic-tool",
        "collaboration",
        "subagent-activity",
        "search-image-review",
        "compaction-replacement",
        "compaction-deprecated",
        "hook-lifecycle",
        "hook-prompt",
        "approvals-and-resolution",
        "permission-approval",
        "task-list",
        "item-timestamps",
        "namespaced-unknown-item",
        "qualified-current-additive",
        "request-id-union",
        "malformed-item",
        "foreign-item",
    ] {
        assert!(
            names.contains(required),
            "missing app-server case {required}"
        );
    }

    let reasoning = case(&cases, "reasoning-summary");
    assert_eq!(reasoning["expected"]["kind"], "reasoning_summary");
    assert!(methods(reasoning).contains("item/reasoning/summaryTextDelta"));
    let raw = case(&cases, "raw-reasoning-excluded");
    assert_eq!(raw["expected"]["portable"], "excluded");
    assert!(methods(raw).contains("item/reasoning/textDelta"));

    let replacement = case(&cases, "plan-replacement");
    assert_eq!(
        replacement["expected"]["update"],
        "authoritative_replacement"
    );
    assert_eq!(methods(replacement).len(), 1);

    let newer = case(&cases, "qualified-current-additive");
    assert_eq!(newer["expected"]["profile"], "0.146.0-guarantee");
    assert_eq!(
        case(&cases, "request-id-union")["expected"]["request_id_representations"],
        serde_json::json!(["string", "integer"])
    );
}

#[test]
fn app_server_identity_failures_and_unknowns_are_explicit() {
    let cases = json_lines(APP_SERVER);
    assert_eq!(
        case(&cases, "namespaced-unknown-item")["expected"]["kind"],
        "unknown"
    );
    for name in ["malformed-item", "foreign-item"] {
        assert_eq!(case(&cases, name)["expected"]["portable"], "fail_closed");
    }

    for fixture in &cases {
        for message in fixture["messages"]
            .as_array()
            .expect("messages are an array")
        {
            assert!(message["method"].is_string(), "method is required");
            assert!(
                message["params"].is_object(),
                "notification or request params are required"
            );
        }
    }
}
