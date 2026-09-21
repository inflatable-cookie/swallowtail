//! Frozen mapped-surface ledger for the Grok Build ACP 1.0.40 run.

use super::support::{PROTOCOL, assert_exact_string_set, digest_lines, json, strings};
use std::collections::BTreeMap;

/// Every selected-surface literal the probe searched for.
const SELECTED_LITERALS: &[&str] = &[
    "--no-auto-update",
    "XAI_API_KEY",
    "_meta",
    "_x.ai/announcements/update",
    "_x.ai/mcp/servers_updated",
    "_x.ai/mcp_initialized",
    "_x.ai/models/update",
    "_x.ai/session/close",
    "_x.ai/session/update",
    "_x.ai/session_notification",
    "_x.ai/settings/update",
    "agentCapabilities",
    "agentInfo",
    "agentVersion",
    "agent_message_chunk",
    "agent_thought_chunk",
    "allow_always",
    "allow_once",
    "authMethods",
    "authenticate",
    "availableModels",
    "available_commands_update",
    "cached_token",
    "cancelled",
    "clientCapabilities",
    "config_option_update",
    "content",
    "context_window",
    "currentModelId",
    "current_mode_update",
    "cwd",
    "defaultAuthMethodId",
    "embeddedContext",
    "end_turn",
    "fs/read_text_file",
    "grok-4.5",
    "grok-4.6",
    "grok.com",
    "headless",
    "high",
    "initialize",
    "loadSession",
    "low",
    "max_tokens",
    "max_turn_requests",
    "mcpCapabilities",
    "mcpServers",
    "medium",
    "modelState",
    "oidc",
    "optionId",
    "options",
    "plan",
    "prompt",
    "promptCapabilities",
    "protocolVersion",
    "reasoning_effort",
    "reasoning_efforts",
    "refusal",
    "reject_always",
    "reject_once",
    "session/cancel",
    "session/close",
    "session/load",
    "session/new",
    "session/prompt",
    "session/request_permission",
    "session/set_model",
    "session/update",
    "sessionCapabilities",
    "sessionId",
    "sessionUpdate",
    "session_info_update",
    "stdio",
    "stopReason",
    "supports_reasoning_effort",
    "toolCall",
    "toolCallId",
    "tool_call",
    "tool_call_update",
    "usage_update",
    "user_message_chunk",
    "x.ai/announcements/update",
    "x.ai/mcp/servers_updated",
    "x.ai/mcp_initialized",
    "x.ai/models/update",
    "x.ai/session/close",
    "x.ai/session/update",
    "x.ai/session/upsert",
    "x.ai/session_notification",
    "x.ai/settings/update",
    "xhigh",
];

/// Underscore-prefixed vendor channel spellings the vendor synthesizes at
/// runtime, so they never appear as shipped literals.
const UNDERSCORE_PREFIXED_ABSENT: &[&str] = &[
    "_x.ai/announcements/update",
    "_x.ai/mcp/servers_updated",
    "_x.ai/mcp_initialized",
    "_x.ai/models/update",
    "_x.ai/session_notification",
    "_x.ai/settings/update",
];

/// Linux-x64 already omits `agentVersion` at the 1.0.30 ceiling.
const LINUX_ABSENT_AT_CEILING: &[&str] = &["agentVersion"];

const LINUX_PRESENCE_DIGEST: &str =
    "4cceb3e6fc7893dd2e26e8487b5e7266c4040d2fd189ac38ea4569365a10b311";
const DARWIN_PRESENCE_DIGEST: &str =
    "4a548e4dc7687941e640dc410146a0ed99dc8b95724871317ffe37a14fe3dfb3";
const MODEL_DOCUMENT: &str = "9d6924ec760a94f91902f60adb8bcf2cd9d3ab86891a1c83e8c01a092e56490a";

/// ACP module paths present in every published hop.
const STABLE_CORE: &[&str] = &[
    "crates/codegen/xai-acp-lib/src/channel.rs",
    "crates/codegen/xai-acp-lib/src/gateway.rs",
    "crates/codegen/xai-acp-lib/src/line_reader.rs",
    "crates/codegen/xai-acp-lib/src/normalize.rs",
    "crates/codegen/xai-acp-lib/src/stdin_reader.rs",
    "crates/codegen/xai-grok-mcp/src/acp_transport.rs",
    "crates/codegen/xai-grok-pager/src/acp/leader_bridge.rs",
    "crates/codegen/xai-grok-pager/src/acp/mod.rs",
    "crates/codegen/xai-grok-pager/src/acp/spawn.rs",
    "crates/codegen/xai-grok-pager/src/acp/tracker.rs",
    "crates/codegen/xai-grok-pager/src/app/acp_handler/background.rs",
    "crates/codegen/xai-grok-pager/src/app/acp_handler/interactions.rs",
    "crates/codegen/xai-grok-pager/src/app/acp_handler/mcp.rs",
    "crates/codegen/xai-grok-pager/src/app/acp_handler/mod.rs",
    "crates/codegen/xai-grok-pager/src/app/acp_handler/permissions.rs",
    "crates/codegen/xai-grok-pager/src/app/acp_handler/queue.rs",
    "crates/codegen/xai-grok-pager/src/app/acp_handler/session_notification.rs",
    "crates/codegen/xai-grok-pager/src/app/acp_handler/settings.rs",
    "crates/codegen/xai-grok-shell/src/agent/mvp_agent/acp_agent.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_mcp.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session/hooks.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/auth_retry.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/cursor_describe.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/extensions.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/extensions/idle_prompt.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/goal.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/goal_support.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/hook_dispatch.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/hooks_plugins.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/image_strip.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/interjection.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/laziness.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/mcp.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/mcp_snapshot.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/memory_dream.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/model_switch.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/notification_drain.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/prompt_build.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/prompt_queue.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/recap.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/reminders.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/rewind.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/run_loop.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/sampler_turn.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/session_mode.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/session_setup.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/side_call.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/slash_exec.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/spawn.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/stop_gate.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/title_refresh.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/tool_calls.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/tool_dispatch.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/turn.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/turn_end.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/turn_end_hooks.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/turn_report_slot.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/turn_summary.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/updates.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/workflow.rs",
    "crates/codegen/xai-grok-workspace/src/file_system/acp_fs.rs",
];

const SELECTED_CORE_MODULES: &[&str] = &[
    "crates/codegen/xai-acp-lib/src/gateway.rs",
    "crates/codegen/xai-acp-lib/src/normalize.rs",
    "crates/codegen/xai-grok-mcp/src/acp_transport.rs",
    "crates/codegen/xai-grok-pager/src/app/acp_handler/mcp.rs",
    "crates/codegen/xai-grok-pager/src/app/acp_handler/permissions.rs",
    "crates/codegen/xai-grok-pager/src/app/acp_handler/session_notification.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/mcp.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/mcp_snapshot.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/model_switch.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/run_loop.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/sampler_turn.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/session_setup.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/spawn.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/tool_calls.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/turn.rs",
    "crates/codegen/xai-grok-shell/src/session/acp_session_impl/updates.rs",
    "crates/codegen/xai-grok-workspace/src/file_system/acp_fs.rs",
];

const ADDED_FROM_1_0_30: &[(&str, &[&str])] = &[
    (
        "1.0.33",
        &[
            "crates/codegen/xai-grok-shell/src/session/acp_session_impl/memory_control.rs",
            "crates/codegen/xai-grok-shell/src/session/acp_session_impl/memory_forget.rs",
        ],
    ),
    (
        "1.0.34",
        &[
            "crates/codegen/xai-grok-shell/src/session/acp_session_impl/memory_carryover.rs",
            "crates/codegen/xai-grok-shell/src/session/acp_session_impl/memory_control.rs",
            "crates/codegen/xai-grok-shell/src/session/acp_session_impl/memory_forget.rs",
        ],
    ),
    (
        "1.0.35",
        &[
            "crates/codegen/xai-grok-shell/src/session/acp_session_impl/memory_carryover.rs",
            "crates/codegen/xai-grok-shell/src/session/acp_session_impl/memory_control.rs",
            "crates/codegen/xai-grok-shell/src/session/acp_session_impl/memory_forget.rs",
        ],
    ),
    (
        "1.0.36",
        &[
            "crates/codegen/xai-grok-shell/src/session/acp_session_impl/memory_carryover.rs",
            "crates/codegen/xai-grok-shell/src/session/acp_session_impl/memory_control.rs",
            "crates/codegen/xai-grok-shell/src/session/acp_session_impl/memory_forget.rs",
        ],
    ),
    (
        "1.0.37",
        &[
            "crates/codegen/xai-grok-shell/src/session/acp_session_impl/mcp_file_input.rs",
            "crates/codegen/xai-grok-shell/src/session/acp_session_impl/memory_carryover.rs",
            "crates/codegen/xai-grok-shell/src/session/acp_session_impl/memory_control.rs",
            "crates/codegen/xai-grok-shell/src/session/acp_session_impl/memory_forget.rs",
        ],
    ),
    (
        "1.0.38",
        &[
            "crates/codegen/xai-grok-shell/src/session/acp_session_impl/mcp_file_input.rs",
            "crates/codegen/xai-grok-shell/src/session/acp_session_impl/memory_carryover.rs",
            "crates/codegen/xai-grok-shell/src/session/acp_session_impl/memory_control.rs",
            "crates/codegen/xai-grok-shell/src/session/acp_session_impl/memory_forget.rs",
        ],
    ),
    (
        "1.0.39",
        &[
            "crates/codegen/xai-grok-pager/src/app/acp_handler/prompt_origin.rs",
            "crates/codegen/xai-grok-shell/src/session/acp_session_impl/mcp_file_input.rs",
            "crates/codegen/xai-grok-shell/src/session/acp_session_impl/memory_carryover.rs",
            "crates/codegen/xai-grok-shell/src/session/acp_session_impl/memory_control.rs",
            "crates/codegen/xai-grok-shell/src/session/acp_session_impl/memory_forget.rs",
        ],
    ),
    (
        "1.0.40",
        &[
            "crates/codegen/xai-grok-pager/src/app/acp_handler/prompt_origin.rs",
            "crates/codegen/xai-grok-shell/src/session/acp_session_impl/mcp_file_input.rs",
            "crates/codegen/xai-grok-shell/src/session/acp_session_impl/memory_carryover.rs",
            "crates/codegen/xai-grok-shell/src/session/acp_session_impl/memory_control.rs",
            "crates/codegen/xai-grok-shell/src/session/acp_session_impl/memory_forget.rs",
        ],
    ),
];

const REMOVED_FROM_1_0_30: &[(&str, &[&str])] = &[
    (
        "1.0.35",
        &["crates/codegen/xai-grok-shell/src/session/acp_session_impl/memory_status.rs"],
    ),
    (
        "1.0.36",
        &["crates/codegen/xai-grok-shell/src/session/acp_session_impl/memory_status.rs"],
    ),
    (
        "1.0.37",
        &["crates/codegen/xai-grok-shell/src/session/acp_session_impl/memory_status.rs"],
    ),
    (
        "1.0.38",
        &["crates/codegen/xai-grok-shell/src/session/acp_session_impl/memory_status.rs"],
    ),
    (
        "1.0.39",
        &["crates/codegen/xai-grok-shell/src/session/acp_session_impl/memory_status.rs"],
    ),
    (
        "1.0.40",
        &["crates/codegen/xai-grok-shell/src/session/acp_session_impl/memory_status.rs"],
    ),
];

fn map<'a>(pairs: &'a [(&'a str, &'a [&'a str])]) -> BTreeMap<&'a str, &'a [&'a str]> {
    pairs.iter().copied().collect()
}

#[test]
fn selected_surface_literals_are_frozen_and_identical_at_every_hop() {
    let protocol = json(PROTOCOL);
    let presence = protocol["selected_literal_presence"]
        .as_object()
        .expect("presence map");
    let keys: Vec<&str> = presence.keys().map(String::as_str).collect();
    assert_eq!(keys, SELECTED_LITERALS);
    let present: Vec<String> = SELECTED_LITERALS
        .iter()
        .filter(|literal| presence[**literal].as_bool().expect("boolean"))
        .map(|literal| (*literal).to_owned())
        .collect();
    assert_eq!(
        present.len(),
        SELECTED_LITERALS.len() - UNDERSCORE_PREFIXED_ABSENT.len() - LINUX_ABSENT_AT_CEILING.len()
    );
    for absent in UNDERSCORE_PREFIXED_ABSENT {
        assert_eq!(presence[*absent], false, "{absent}");
    }
    for absent in LINUX_ABSENT_AT_CEILING {
        assert_eq!(presence[*absent], false, "{absent}");
    }
    let digest = digest_lines(&present);
    assert_eq!(digest, LINUX_PRESENCE_DIGEST);
    assert_eq!(
        protocol["selected_literal_presence_digest"],
        digest.as_str()
    );
    assert_eq!(
        protocol["selected_literals_identical_across_all_hops"],
        true
    );
    assert_eq!(
        protocol["linux_x64_agent_version_literal_absent_at_ceiling"],
        true
    );
    assert_eq!(
        protocol["darwin_arm64_selected_literal_presence_digest"],
        DARWIN_PRESENCE_DIGEST
    );
    assert_eq!(
        protocol["darwin_arm64_selected_literals_identical_across_all_hops"],
        true
    );
    for (version, entry) in protocol["per_hop"].as_object().expect("per hop") {
        assert_eq!(
            entry["selected_literal_presence_digest"],
            digest.as_str(),
            "{version}"
        );
        assert_eq!(
            entry["darwin_arm64_selected_literal_presence_digest"], DARWIN_PRESENCE_DIGEST,
            "{version}"
        );
    }
    for mapped in [
        "cached_token",
        "grok.com",
        "grok-4.6",
        "grok-4.5",
        "xhigh",
        "protocolVersion",
        "loadSession",
        "mcpCapabilities",
        "session/new",
        "session/prompt",
        "session/cancel",
        "session/request_permission",
        "fs/read_text_file",
        "allow_once",
        "reject_once",
        "modelState",
        "currentModelId",
        "availableModels",
        "stopReason",
        "end_turn",
        "cancelled",
        "sessionUpdate",
        "agent_message_chunk",
        "agent_thought_chunk",
        "tool_call",
        "tool_call_update",
    ] {
        assert_eq!(presence[mapped], true, "{mapped} must stay present");
    }
}

#[test]
fn model_document_stays_byte_identical_through_1_0_40() {
    let protocol = json(PROTOCOL);
    let document = &protocol["model_document"];
    assert_eq!(document["default"], "grok-4.6");
    assert_exact_string_set(&document["model_ids"], &["grok-4.6", "grok-4.5"]);
    assert_eq!(document["default_reasoning_effort"], "high");
    assert_exact_string_set(
        &document["reasoning_effort_values"],
        &["xhigh", "high", "medium", "low"],
    );
    assert_exact_string_set(&document["unmapped_fields_delta"], &[]);
    let digests = document["digests"].as_object().expect("digests");
    assert_eq!(digests.len(), 1);
    let hops: Vec<&str> = digests[MODEL_DOCUMENT]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
    assert_eq!(
        hops,
        vec![
            "1.0.30", "1.0.31", "1.0.32", "1.0.33", "1.0.34", "1.0.35", "1.0.36", "1.0.37",
            "1.0.38", "1.0.39", "1.0.40",
        ]
    );
    for (version, entry) in protocol["per_hop"].as_object().unwrap() {
        assert_eq!(
            entry["model_document_sha256"][0].as_str().unwrap(),
            MODEL_DOCUMENT,
            "{version}"
        );
    }
}

#[test]
fn acp_module_inventory_keeps_the_mapped_core_and_classifies_additions() {
    let protocol = json(PROTOCOL);
    let inventory = &protocol["acp_module_inventory"];
    assert_exact_string_set(&inventory["stable_core"], STABLE_CORE);
    for module in SELECTED_CORE_MODULES {
        assert!(
            STABLE_CORE.contains(module),
            "{module} must stay in the core"
        );
    }
    let baseline: Vec<String> = protocol["per_hop"]["1.0.30"]["acp_module_paths"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().to_owned())
        .collect();
    let added = map(ADDED_FROM_1_0_30);
    let removed = map(REMOVED_FROM_1_0_30);
    let core: Vec<String> = STABLE_CORE.iter().map(|m| (*m).to_owned()).collect();
    let mut seen = BTreeMap::new();
    for (version, entry) in protocol["per_hop"].as_object().unwrap() {
        let paths: Vec<String> = entry["acp_module_paths"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap().to_owned())
            .collect();
        assert_eq!(
            paths.len(),
            entry["acp_module_path_count"].as_u64().unwrap() as usize
        );
        assert_eq!(
            digest_lines(&paths),
            entry["acp_module_path_set_digest"].as_str().unwrap()
        );
        for module in STABLE_CORE {
            assert!(paths.iter().any(|p| p == module), "{version} lost {module}");
        }
        let mut computed_added: Vec<String> = paths
            .iter()
            .filter(|p| !baseline.contains(p))
            .cloned()
            .collect();
        let mut computed_removed: Vec<String> = baseline
            .iter()
            .filter(|p| !paths.contains(p))
            .cloned()
            .collect();
        computed_added.sort();
        computed_removed.sort();
        assert_eq!(
            computed_added,
            added
                .get(version.as_str())
                .map(|v| v.to_vec())
                .unwrap_or_default(),
            "{version} added"
        );
        assert_eq!(
            computed_removed,
            removed
                .get(version.as_str())
                .map(|v| v.to_vec())
                .unwrap_or_default(),
            "{version} removed"
        );
        assert_exact_string_set(
            &inventory["per_hop_added"][version],
            &computed_added
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
        );
        assert_exact_string_set(
            &inventory["per_hop_removed"][version],
            &computed_removed
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
        );
        for module in core.iter().filter(|m| !paths.contains(m)) {
            seen.insert(module.clone(), version.clone());
        }
    }
    assert!(
        seen.is_empty(),
        "core modules missing from some hop: {seen:?}"
    );
}

#[test]
fn unmapped_boundaries_and_run_flags_are_frozen() {
    let protocol = json(PROTOCOL);
    assert_eq!(protocol["selected_route"], "grok-build.acp");
    assert_eq!(protocol["acp_protocol_version"], 1);
    assert_exact_string_set(
        &protocol["selected_command"],
        &["--no-auto-update", "agent", "stdio"],
    );
    for flag in [
        "provider_prompt_sent",
        "live_acp_session",
        "credential_used",
        "catalogue_command_executed",
        "downloaded_artifacts_executed",
        "host_install_changed",
    ] {
        assert_eq!(protocol[flag], false, "{flag} must stay false");
    }
    let boundaries = strings(&protocol["unmapped_boundaries"]);
    assert!(!boundaries.is_empty());
    assert!(
        strings(&protocol["underscore_prefixed_vendor_forms_absent_as_literals"])
            == UNDERSCORE_PREFIXED_ABSENT
    );
}
