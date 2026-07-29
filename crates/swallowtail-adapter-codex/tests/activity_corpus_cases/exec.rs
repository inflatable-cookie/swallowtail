use super::support::{EXEC, case, case_names, json_lines};

#[test]
fn exec_corpus_preserves_its_thinner_item_lifecycle_truth() {
    let cases = json_lines(EXEC);
    let names = case_names(&cases);
    for required in [
        "agent-message-completion",
        "reasoning-summary-completion",
        "command-lifecycle",
        "file-change-completion",
        "mcp-tool-lifecycle",
        "search-lifecycle",
        "search-deferred-query-lifecycle",
        "collaboration-lifecycle",
        "todo-list-lifecycle",
        "warning-completion",
        "additive-event",
        "malformed-item",
    ] {
        assert!(names.contains(required), "missing exec case {required}");
    }

    for name in [
        "agent-message-completion",
        "reasoning-summary-completion",
        "file-change-completion",
        "warning-completion",
    ] {
        assert_eq!(
            case(&cases, name)["expected"]["lifecycle"],
            "completion_only"
        );
    }
    for name in [
        "command-lifecycle",
        "mcp-tool-lifecycle",
        "search-lifecycle",
        "search-deferred-query-lifecycle",
        "collaboration-lifecycle",
        "todo-list-lifecycle",
    ] {
        assert_eq!(case(&cases, name)["expected"]["lifecycle"], "complete");
    }
    assert_eq!(
        case(&cases, "malformed-item")["expected"]["portable"],
        "fail_closed"
    );
    assert_eq!(
        case(&cases, "search-deferred-query-lifecycle")["expected"]["started_content"],
        "none"
    );
}
