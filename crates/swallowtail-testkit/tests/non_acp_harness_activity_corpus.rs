use serde_json::Value;
use std::collections::BTreeSet;

const INVENTORY: &str = include_str!("fixtures/non-acp-harness-activity.json");
const OPENCODE_COMPATIBILITY: &str = include_str!(
    "../../swallowtail-adapter-opencode/tests/fixtures/opencode-v1.14.48-v1.18.4/compatibility.json"
);
const OPENCODE_RICH: &str = include_str!(
    "../../swallowtail-adapter-opencode/tests/fixtures/opencode-v1.14.48-v1.18.4/activity-rich.sse"
);
const OPENCODE_GAP: &str = include_str!(
    "../../swallowtail-adapter-opencode/tests/fixtures/opencode-v1.14.48-v1.18.4/activity-gap-1.14.51.sse"
);
const PI_ACTIVITY: &str =
    include_str!("../../swallowtail-adapter-pi/tests/fixtures/pi-rpc-0.80.10/activity.jsonl");
const KIMI_LOCAL_ACTIVITY: &str = include_str!(
    "../../swallowtail-adapter-kimi/tests/fixtures/kimi-local-server-0.28.1-0.29.0/activity.jsonl"
);
const MANAGED_ACTIVITY: &str = include_str!(
    "../../swallowtail-adapter-anthropic/tests/fixtures/managed-agents-2026-04-01/activity.sse"
);
const CLAUDE_COMPLETE: &str = include_str!(
    "../../swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.220/headless-tools.jsonl"
);
const GEMINI_COMPLETE: &str = include_str!(
    "../../swallowtail-adapter-gemini/tests/fixtures/gemini-headless-0.51.0-0.52.0/success.jsonl"
);
const KIMI_COMPLETE: &str = include_str!(
    "../../swallowtail-adapter-kimi/tests/fixtures/kimi-code-0.29.1-0.29.2/headless-tools.jsonl"
);
const QWEN_PARTIAL: &str = include_str!(
    "../../swallowtail-adapter-qwen/tests/fixtures/qwen-code-v0.19.11/activity-tools.jsonl"
);
const PI_MALFORMED: &str =
    include_str!("../../swallowtail-adapter-pi/tests/fixtures/pi-rpc-0.80.10/malformed.jsonl");
const KIMI_LOCAL_MALFORMED: &str = include_str!(
    "../../swallowtail-adapter-kimi/tests/fixtures/kimi-local-server-0.28.1-0.29.0/ws-malformed-activity.json"
);
const CLAUDE_MALFORMED: &str = include_str!(
    "../../swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.220/headless-malformed.jsonl"
);
const GEMINI_MALFORMED: &str = include_str!(
    "../../swallowtail-adapter-gemini/tests/fixtures/gemini-headless-0.51.0-0.52.0/malformed.jsonl"
);
const KIMI_MALFORMED: &str = include_str!(
    "../../swallowtail-adapter-kimi/tests/fixtures/kimi-code-0.29.1-0.29.2/headless-malformed.jsonl"
);
const QWEN_MALFORMED: &str = include_str!(
    "../../swallowtail-adapter-qwen/tests/fixtures/qwen-code-v0.19.11/malformed.jsonl"
);

#[test]
fn inventory_accounts_for_every_non_acp_production_harness_route() {
    let inventory = json(INVENTORY);
    assert_eq!(
        inventory["contract"],
        "044-observable-agent-activity-and-disclosure"
    );
    let routes = inventory["routes"].as_array().expect("routes are an array");
    assert_eq!(routes.len(), 8);

    let ids: BTreeSet<_> = routes
        .iter()
        .map(|route| route["id"].as_str().expect("route id is text"))
        .collect();
    assert_eq!(
        ids,
        BTreeSet::from([
            "anthropic.managed-agents",
            "claude-code.headless",
            "gemini-cli.headless",
            "kimi-code.headless",
            "kimi.local-server",
            "opencode.http-sse",
            "pi.rpc",
            "qwen-code.headless",
        ])
    );

    for route in routes {
        for field in [
            "qualified",
            "current",
            "profile",
            "lifecycle",
            "disclosure",
            "correlation",
            "tool_ownership",
            "unknown_posture",
        ] {
            assert!(
                route[field].as_str().is_some_and(|value| !value.is_empty()),
                "{} is missing {field}",
                route["id"]
            );
        }
        assert!(
            !route["activity_kinds"]
                .as_array()
                .expect("activity kinds are an array")
                .is_empty()
        );
        assert!(
            !route["exact_absences"]
                .as_array()
                .expect("exact absences are an array")
                .is_empty()
        );
    }

    let all_routes = inventory["fixture_classes"]["unknown"]
        .as_array()
        .expect("unknown coverage is an array");
    assert_eq!(all_routes.len(), 8);
    assert_eq!(
        inventory["fixture_classes"]["malformed"]
            .as_array()
            .expect("malformed coverage is an array")
            .len(),
        8
    );
    assert_eq!(
        inventory["fixture_classes"]["failure"]
            .as_array()
            .expect("failure coverage is an array")
            .len(),
        8
    );
}

#[test]
fn server_and_rpc_corpora_freeze_native_lifecycle_and_ownership_boundaries() {
    let rich = sse_json(OPENCODE_RICH);
    let rich_types = types(&rich);
    assert!(rich_types.contains("message.part.updated"));
    assert!(OPENCODE_RICH.contains("\"status\":\"pending\""));
    assert!(OPENCODE_RICH.contains("\"status\":\"running\""));
    assert!(OPENCODE_RICH.contains("\"status\":\"completed\""));

    let gap = sse_json(OPENCODE_GAP);
    assert_eq!(
        types(&gap),
        BTreeSet::from(["message.part.delta", "session.idle", "session.status"])
    );
    assert!(!OPENCODE_GAP.contains("\"type\":\"tool\""));
    assert!(!OPENCODE_GAP.contains("\"type\":\"reasoning\""));

    let pi = json_lines(PI_ACTIVITY);
    let pi_types = types(&pi);
    for required in [
        "agent_start",
        "message_start",
        "message_update",
        "tool_execution_start",
        "tool_execution_update",
        "tool_execution_end",
        "message_end",
        "agent_settled",
    ] {
        assert!(pi_types.contains(required), "missing Pi event {required}");
    }
    assert!(PI_ACTIVITY.contains("\"toolCallId\":\"tool-fixture\""));

    let kimi = json_lines(KIMI_LOCAL_ACTIVITY);
    let kimi_types = types(&kimi);
    for required in [
        "turn.started",
        "turn.step.started",
        "thinking.delta",
        "assistant.delta",
        "tool.call.started",
        "tool.progress",
        "tool.result",
        "subagent.spawned",
        "subagent.completed",
        "turn.ended",
    ] {
        assert!(
            kimi_types.contains(required),
            "missing Kimi event {required}"
        );
    }
    assert!(KIMI_LOCAL_ACTIVITY.contains("\"toolCallId\":\"tool-fixture\""));

    let managed = sse_json(MANAGED_ACTIVITY);
    let managed_types = types(&managed);
    for required in [
        "agent.message",
        "agent.thinking",
        "agent.tool_use",
        "agent.tool_result",
        "agent.mcp_tool_use",
        "agent.mcp_tool_result",
        "agent.custom_tool_use",
    ] {
        assert!(
            managed_types.contains(required),
            "missing Managed Agents event {required}"
        );
    }
    assert!(MANAGED_ACTIVITY.contains("\"custom_tool_use_id\""));
}

#[test]
fn headless_corpora_keep_partial_and_completion_only_routes_distinct() {
    let claude = json_lines(CLAUDE_COMPLETE);
    assert!(types(&claude).contains("assistant"));
    assert!(!CLAUDE_COMPLETE.contains("\"type\":\"stream_event\""));

    let gemini = json_lines(GEMINI_COMPLETE);
    assert!(types(&gemini).contains("message"));
    assert!(types(&gemini).contains("tool_use"));
    assert!(!GEMINI_COMPLETE.contains("\"type\":\"tool_progress\""));

    let kimi = json_lines(KIMI_COMPLETE);
    let roles: BTreeSet<_> = kimi
        .iter()
        .filter_map(|record| record["role"].as_str())
        .collect();
    assert!(roles.contains("assistant"));
    assert!(roles.contains("tool"));
    assert!(!KIMI_COMPLETE.contains("\"status\":\"running\""));

    let qwen = json_lines(QWEN_PARTIAL);
    assert!(types(&qwen).contains("stream_event"));
    for required in [
        "message_start",
        "content_block_start",
        "content_block_delta",
        "content_block_stop",
        "message_stop",
    ] {
        assert!(QWEN_PARTIAL.contains(&format!("\"type\":\"{required}\"")));
    }
    assert!(QWEN_PARTIAL.contains("\"id\":\"activity-tool\""));
}

#[test]
fn exact_opencode_release_window_and_malformed_cases_are_machine_checked() {
    let compatibility = json(OPENCODE_COMPATIBILITY);
    assert_eq!(
        compatibility["releases"]
            .as_array()
            .expect("release array")
            .len(),
        45
    );
    assert_eq!(
        compatibility["surface_revisions"]
            .as_array()
            .expect("surface revisions are an array")
            .len(),
        18
    );
    assert!(
        compatibility["segments"]
            .as_array()
            .expect("segments are an array")
            .iter()
            .any(|segment| {
                segment["minimum"] == "1.14.51"
                    && segment["maximum"] == "1.14.51"
                    && segment["surface"] == "surface-04"
            })
    );

    for malformed in [
        PI_MALFORMED,
        KIMI_LOCAL_MALFORMED,
        CLAUDE_MALFORMED,
        KIMI_MALFORMED,
        QWEN_MALFORMED,
    ] {
        assert!(
            malformed
                .lines()
                .any(|line| serde_json::from_str::<Value>(line).is_err()),
            "malformed fixture unexpectedly parses"
        );
    }
    let gemini_mismatch = json_lines(GEMINI_MALFORMED);
    assert_eq!(gemini_mismatch[0]["session_id"], "wrong-session");
}

fn json(input: &str) -> Value {
    serde_json::from_str(input).expect("fixture is valid JSON")
}

fn json_lines(input: &str) -> Vec<Value> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(json)
        .collect()
}

fn sse_json(input: &str) -> Vec<Value> {
    input
        .lines()
        .filter_map(|line| line.strip_prefix("data: "))
        .map(json)
        .collect()
}

fn types(events: &[Value]) -> BTreeSet<&str> {
    events
        .iter()
        .filter_map(|event| event["type"].as_str())
        .collect()
}
