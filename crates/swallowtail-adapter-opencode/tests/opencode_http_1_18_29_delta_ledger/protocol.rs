use super::{PROTOCOL, assert_exact_strings, json};

#[test]
fn mapped_and_unmapped_protocol_deltas_are_exact() {
    let protocol = json(PROTOCOL);
    assert_exact_strings(
        &protocol["selected_routes"],
        &[
            "global.health",
            "provider.list",
            "session.create",
            "session.prompt_async",
            "event.subscribe",
            "session.abort",
            "session.delete",
            "session.list",
            "session.status",
            "session.get",
            "session.messages",
        ],
    );
    assert_eq!(
        protocol["selected_route_and_handler_files_byte_identical"],
        true
    );
    assert_exact_strings(
        &protocol["mapped_stable_files"],
        &[
            "packages/opencode/src/server/routes/instance/httpapi/groups/event.ts",
            "packages/opencode/src/server/routes/instance/httpapi/groups/provider.ts",
            "packages/opencode/src/server/routes/instance/httpapi/groups/session.ts",
            "packages/opencode/src/server/routes/instance/httpapi/handlers/event.ts",
            "packages/opencode/src/server/routes/instance/httpapi/handlers/provider.ts",
            "packages/opencode/src/server/routes/instance/httpapi/handlers/session.ts",
            "packages/opencode/src/server/routes/instance/httpapi/lifecycle.ts",
            "packages/opencode/src/session/compaction.ts",
            "packages/opencode/src/session/message-v2.ts",
            "packages/opencode/src/session/session.ts",
            "packages/opencode/src/session/status.ts",
        ],
    );
    assert_eq!(
        protocol["openapi_deltas"]["selected_operation_objects_changed"],
        false
    );
    assert_eq!(
        protocol["classified_mapped_internal_deltas"]
            .as_array()
            .unwrap()
            .len(),
        0
    );
    let unmapped = protocol["classified_unmapped_deltas"]
        .as_array()
        .expect("unmapped delta array");
    assert_eq!(unmapped.len(), 1);
    assert_eq!(unmapped[0]["hop"], "1.18.28_to_1.18.29");
    assert_eq!(
        unmapped[0]["file"],
        "packages/opencode/src/plugin/openai/codex.ts"
    );
    assert_exact_strings(
        &protocol["bounded_unmapped_delta_categories"],
        &["openai-codex-oauth-model-filter"],
    );
    assert_eq!(protocol["downloaded_artifact_executed"], false);
    assert_eq!(protocol["live_server_started"], false);
    assert_eq!(protocol["host_install_changed"], false);
}
