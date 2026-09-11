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
    assert_eq!(unmapped.len(), 9);
    let files = unmapped
        .iter()
        .map(|entry| entry["file"].as_str().expect("unmapped file"))
        .collect::<Vec<_>>();
    assert_eq!(
        files,
        [
            "packages/opencode/src/provider/provider.ts",
            "packages/core/src/plugin/provider/amazon-bedrock.ts",
            "packages/opencode/src/provider/transform.ts",
            "packages/opencode/src/session/system.ts",
            "packages/opencode/src/session/prompt/gpt-astra.txt",
            "packages/opencode/package.json",
            "packages/core/package.json",
            "patches/@ai-sdk%2Fopenai@3.0.88.patch",
            "package.json",
        ]
    );
    for entry in unmapped {
        assert_eq!(entry["hop"], "1.18.29_to_1.18.30");
        assert!(
            entry["classification"]
                .as_str()
                .expect("classification")
                .len()
                > 20
        );
    }
    assert_exact_strings(
        &protocol["bounded_unmapped_delta_categories"],
        &[
            "bedrock-model-id-resolution",
            "gitlab-reasoning-option-transform",
            "gpt-6-system-prompt-selection",
            "provider-sdk-dependency-bumps",
            "openai-explicit-service-tier-patch",
            "package-version-metadata",
            "console-web-and-docs-sites",
            "repository-lockfile-and-nix-hashes",
        ],
    );
    assert_eq!(
        protocol["provider_catalogue_basis"],
        "provider.list derives Info and Model from models.dev plus connected providers; Model and Info schemas and toPublicInfo are byte-identical across the hop, so capability advertisement shape is unchanged"
    );
    assert_eq!(protocol["decoder_corpus"], "opencode-1.14.48");
    assert_eq!(protocol["downloaded_artifact_executed"], false);
    assert_eq!(protocol["live_server_started"], false);
    assert_eq!(protocol["host_install_changed"], false);
    assert_eq!(protocol["provider_prompt_sent"], false);
}
