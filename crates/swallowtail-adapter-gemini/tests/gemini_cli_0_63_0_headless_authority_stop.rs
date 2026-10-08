//! Source-only authority stop for Gemini CLI 0.63.0 headless Plan Mode.

#![allow(dead_code)]

#[path = "headless_support/mod.rs"]
mod headless_support;

use serde_json::{Value, json};
use std::collections::BTreeSet;
use swallowtail_runtime::{ProcessExit, TerminalStatus};
use swallowtail_testkit::ExecutionTopologyFixture;

const AUTHORITY: &str =
    include_str!("fixtures/gemini-cli-headless-authority-0.63.0/authority-evidence.json");
const IDENTITY: &str = include_str!("fixtures/gemini-cli-0.63.0/identity.json");
const SOURCE_TREE: &str = include_str!("fixtures/gemini-cli-0.63.0/source-tree-inventory.json");

fn fixture(body: &str, name: &str) -> Value {
    serde_json::from_str(body).unwrap_or_else(|error| panic!("{name}: {error}"))
}

fn keys(value: &Value, name: &str) -> BTreeSet<String> {
    value
        .as_object()
        .unwrap_or_else(|| panic!("{name} is an object"))
        .keys()
        .cloned()
        .collect()
}

#[test]
fn selected_authority_stop_is_bound_to_the_frozen_source_tree() {
    let authority = fixture(AUTHORITY, "authority evidence");
    let identity = fixture(IDENTITY, "identity");
    let source_tree = fixture(SOURCE_TREE, "source tree");
    assert_eq!(
        keys(&authority, "authority evidence"),
        [
            "fixture_schema",
            "artifact",
            "selected_source_sha256",
            "selected_authority",
            "other_mapped_changes",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect()
    );
    assert_eq!(authority["fixture_schema"], 1);
    assert_eq!(authority["artifact"]["package"], "@google/gemini-cli");
    assert_eq!(authority["artifact"]["version"], "0.63.0");
    assert_eq!(authority["artifact"]["github_tag"], "v0.63.0");
    assert_eq!(
        authority["artifact"]["github_commit"],
        identity["official"]["github_commit"]
    );
    assert_eq!(
        authority["artifact"]["github_tree"],
        identity["official"]["github_tree"]
    );
    assert_eq!(
        authority["artifact"]["source_tree_manifest_sha256"],
        source_tree["tree_manifest_sha256"]["0.63.0"]
    );
    assert_eq!(
        authority["artifact"]["source_file_count"],
        source_tree["file_counts"]["0.63.0"]
    );

    let selected_hashes = authority["selected_source_sha256"]
        .as_object()
        .expect("selected source hashes are an object");
    assert_eq!(selected_hashes.len(), 9);
    assert_eq!(
        keys(
            &authority["selected_source_sha256"],
            "selected source hashes"
        ),
        [
            "packages/core/src/prompts/snippets.ts",
            "packages/core/src/policy/policies/plan.toml",
            "packages/core/src/policy/policy-engine.ts",
            "packages/core/src/tools/exit-plan-mode.ts",
            "packages/core/src/tools/read-file.ts",
            "packages/core/src/safety/built-in.ts",
            "packages/core/src/scheduler/tool-executor.ts",
            "packages/core/src/agents/local-executor.ts",
            "packages/cli/src/nonInteractiveCli.ts",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect()
    );
    for (path, digest) in selected_hashes {
        assert_eq!(
            source_tree["files"]["0.63.0"][path]["kind"], "file",
            "{path} remains a regular source file"
        );
        assert_eq!(
            source_tree["files"]["0.63.0"][path]["sha256"].as_str(),
            digest.as_str(),
            "{path} matches the frozen source tree"
        );
    }

    let selected = &authority["selected_authority"];
    assert_eq!(
        keys(selected, "selected authority"),
        [
            "plan_mode_prompt",
            "plan_mode_policy",
            "noninteractive_policy",
            "allowed_exit_transition",
            "stream_order",
            "selected_adapter",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect()
    );
    assert_eq!(
        selected["plan_mode_prompt"],
        json!({
            "noninteractive_strategy": "draft_autonomously",
            "transition_tool": "exit_plan_mode",
            "transition_effect": "begin_implementation"
        })
    );
    assert_eq!(
        selected["plan_mode_policy"],
        json!({
            "tool_name": "exit_plan_mode",
            "decision": "allow",
            "priority": 70,
            "modes": ["plan"],
            "interactive": false
        })
    );
    assert_eq!(
        selected["noninteractive_policy"],
        json!({
            "default_decision": "deny",
            "ask_user_decision": "deny"
        })
    );
    assert_eq!(
        selected["allowed_exit_transition"]["noninteractive_approval_mode"],
        "yolo"
    );
    assert!(
        selected["stream_order"]["tool_use_emitted_before_scheduler"]
            .as_bool()
            .expect("stream order is a boolean")
    );
    assert_eq!(
        keys(&selected["selected_adapter"], "selected adapter evidence"),
        [
            "approval_mode",
            "prompt_transport",
            "extensions_disabled",
            "external_mcp_disabled",
            "exit_plan_mode_interceptor",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect()
    );
    assert_eq!(selected["selected_adapter"]["approval_mode"], "plan");
    assert_eq!(selected["selected_adapter"]["prompt_transport"], "stdin");
    assert!(
        selected["selected_adapter"]["extensions_disabled"]
            .as_bool()
            .expect("extension posture is a boolean")
    );
    assert!(
        selected["selected_adapter"]["external_mcp_disabled"]
            .as_bool()
            .expect("MCP posture is a boolean")
    );
    assert_eq!(
        selected["selected_adapter"]["exit_plan_mode_interceptor"],
        false
    );

    assert_eq!(
        authority["other_mapped_changes"],
        json!({
            "read_file_real_path_before_access_check": true,
            "gemini_config_write_safety_decision": "ask_user",
            "noninteractive_ask_user_result": "deny",
            "stored_tool_output_cap_bytes": 65_536,
            "context_pressure": "collapse_older_function_responses",
            "stream_formatter_and_types": "byte_identical_to_0.61.0"
        })
    );
}

#[test]
fn selected_adapter_accepts_a_fake_exit_transition_and_subsequent_tool_events() {
    let request_id = "gemini-headless-0-63-plan-transition";
    let session_id = headless_support::session_id(request_id);
    let output = [
        json!({
            "type": "init",
            "session_id": session_id,
            "model": "gemini-2.5-flash"
        }),
        json!({
            "type": "message",
            "role": "assistant",
            "content": "Plan drafted.\n",
            "delta": true
        }),
        json!({
            "type": "tool_use",
            "tool_name": "exit_plan_mode",
            "tool_id": "transition",
            "parameters": {}
        }),
        json!({
            "type": "tool_result",
            "tool_id": "transition",
            "status": "success",
            "output": "Plan approved. Switching to YOLO mode."
        }),
        json!({
            "type": "tool_use",
            "tool_name": "write_file",
            "tool_id": "implementation",
            "parameters": { "file_path": "src/main.rs" }
        }),
        json!({
            "type": "tool_result",
            "tool_id": "implementation",
            "status": "success",
            "output": "write accepted"
        }),
        json!({
            "type": "message",
            "role": "assistant",
            "content": "Implementation proceeded.",
            "delta": true
        }),
        json!({
            "type": "result",
            "status": "success",
            "stats": {
                "input_tokens": 1,
                "output_tokens": 1,
                "cached": 0,
                "input": 1
            }
        }),
    ]
    .into_iter()
    .map(|event| event.to_string())
    .collect::<Vec<_>>()
    .join("\n");

    let evidence = headless_support::completed(
        &ExecutionTopologyFixture::local(),
        &output,
        ProcessExit::new(true, Some(0)),
        request_id,
    );

    assert_eq!(evidence.outcome.status(), &TerminalStatus::Completed);
    assert_eq!(
        evidence.outcome.output().map(|content| content.as_str()),
        Some("Plan drafted.\nImplementation proceeded.")
    );
    let tool_labels: BTreeSet<String> = evidence
        .events
        .iter()
        .filter_map(|event| match event.kind() {
            swallowtail_runtime::RuntimeEventKind::Activity(activity)
                if activity.kind() == &swallowtail_runtime::ActivityKind::ProviderOwnedTool =>
            {
                activity.label().map(|label| label.as_str().to_owned())
            }
            _ => None,
        })
        .collect();
    assert!(tool_labels.contains("exit_plan_mode"));
    assert!(tool_labels.contains("write_file"));
    assert!(evidence.events.iter().any(|event| matches!(
        event.kind(),
        swallowtail_runtime::RuntimeEventKind::ProviderObservation(
            swallowtail_runtime::ProviderObservation::Usage(usage)
        ) if usage.input_tokens() == Some(1) && usage.output_tokens() == Some(1)
    )));
}
