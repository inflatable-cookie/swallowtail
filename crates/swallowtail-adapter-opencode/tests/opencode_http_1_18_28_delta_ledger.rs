use std::collections::BTreeSet;

use serde_json::{Map, Value};

const IDENTITY: &str = include_str!("fixtures/opencode-1.18.28/identity.json");
const PROTOCOL: &str = include_str!("fixtures/opencode-1.18.28/protocol.json");
const INVENTORY: &str = include_str!("fixtures/opencode-1.18.28/dist-inventory.json");

#[test]
fn official_hop_identity_is_exact_and_identity_first() {
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], "opencode.server");
    assert_eq!(identity["version"], "1.18.28");
    assert_eq!(identity["npm_package"], "opencode-ai");
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(identity["github_latest_tag"], "v1.18.28");
    assert_eq!(identity["official_channels_agree"], true);
    assert_eq!(identity["host"]["version"], "1.18.18");
    assert_eq!(identity["host"]["size"], 143_182_562);
    assert_eq!(identity["unpublished_next"], "1.18.29");

    assert_exact_strings(
        &identity["published_stables_from_previous_ceiling"],
        &[
            "1.18.21", "1.18.22", "1.18.23", "1.18.24", "1.18.25", "1.18.26", "1.18.27", "1.18.28",
        ],
    );

    let hops = identity["official_hops"].as_array().expect("hop array");
    assert_eq!(hops.len(), 9);
    assert_eq!(
        hops.iter()
            .map(|hop| hop["version"].as_str().unwrap())
            .collect::<Vec<_>>(),
        [
            "1.18.20", "1.18.21", "1.18.22", "1.18.23", "1.18.24", "1.18.25", "1.18.26", "1.18.27",
            "1.18.28",
        ]
    );
    for hop in hops {
        for key in [
            "npm_published_at",
            "npm_integrity",
            "npm_shasum",
            "npm_tarball_sha256",
            "github_tag_commit",
            "github_release_published_at",
            "source_archive_sha256",
        ] {
            assert_nonempty_string(hop, key);
        }
    }

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(decision["raise_latest_qualified_to"], "1.18.28");
    assert_eq!(decision["claim_changed_in_identity_card"], false);
    assert_eq!(decision["downloaded_artifact_executed"], false);
    assert_eq!(decision["host_install_changed"], false);
    assert_eq!(decision["claim_card"], "g05 batch card 078");
}

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
        5
    );
    assert_exact_strings(
        &protocol["bounded_unmapped_delta_categories"],
        &[
            "account-device-login-url-validation",
            "azure-plugin-and-resource-selection",
            "cloudflare-provider-routing",
            "config-v2-lowering",
            "global-upgrade-request",
            "github-cli-and-copilot-plugin",
            "model-provider-transforms",
            "provider-upsell-copy",
            "tool-apply-patch-optional-move-path",
        ],
    );
    assert_eq!(protocol["downloaded_artifact_executed"], false);
    assert_eq!(protocol["live_server_started"], false);
}

#[test]
fn artifact_tree_delta_ledger_is_mutation_sensitive() {
    let inventory = json(INVENTORY);
    assert_exact_strings(
        &inventory["compared"],
        &[
            "1.18.20", "1.18.21", "1.18.22", "1.18.23", "1.18.24", "1.18.25", "1.18.26", "1.18.27",
            "1.18.28",
        ],
    );
    assert_exact_object_keys(
        inventory["source_deltas"].as_object().unwrap(),
        &[
            "1.18.20_to_1.18.21",
            "1.18.21_to_1.18.22",
            "1.18.22_to_1.18.23",
            "1.18.23_to_1.18.24",
            "1.18.24_to_1.18.25",
            "1.18.25_to_1.18.26",
            "1.18.26_to_1.18.27",
            "1.18.27_to_1.18.28",
        ],
    );
    for delta in inventory["source_deltas"].as_object().unwrap().values() {
        assert_exact_object_keys(delta.as_object().unwrap(), &["added", "changed", "removed"]);
    }
    assert_delta(
        &inventory,
        "1.18.20_to_1.18.21",
        &[],
        &[
            "packages/opencode/src/provider/provider.ts",
            "packages/opencode/src/session/prompt.ts",
        ],
    );
    assert_delta(
        &inventory,
        "1.18.21_to_1.18.22",
        &[],
        &[
            "packages/opencode/src/account/account.ts",
            "packages/opencode/src/provider/transform.ts",
            "packages/opencode/src/server/routes/instance/httpapi/groups/global.ts",
            "packages/opencode/src/server/routes/instance/httpapi/handlers/global.ts",
            "packages/opencode/src/session/retry.ts",
        ],
    );
    assert_delta(
        &inventory,
        "1.18.22_to_1.18.23",
        &[],
        &[
            "packages/opencode/src/cli/cmd/github.handler.ts",
            "packages/opencode/src/provider/provider.ts",
            "packages/opencode/src/session/llm/request.ts",
        ],
    );
    assert_delta(
        &inventory,
        "1.18.23_to_1.18.24",
        &["packages/opencode/src/config/v2-compat.ts"],
        &[
            "packages/opencode/src/config/config.ts",
            "packages/opencode/src/plugin/azure.ts",
            "packages/opencode/src/provider/provider.ts",
            "packages/opencode/src/provider/transform.ts",
        ],
    );
    assert_delta(
        &inventory,
        "1.18.24_to_1.18.25",
        &[],
        &["packages/opencode/src/plugin/azure.ts"],
    );
    assert_delta(
        &inventory,
        "1.18.25_to_1.18.26",
        &[],
        &[
            "packages/opencode/src/plugin/azure.ts",
            "packages/opencode/src/provider/transform.ts",
            "packages/opencode/src/session/processor.ts",
            "packages/opencode/src/session/tools.ts",
            "packages/opencode/src/tool/apply_patch.ts",
        ],
    );
    assert_delta(
        &inventory,
        "1.18.26_to_1.18.27",
        &[],
        &[
            "packages/opencode/src/provider/provider.ts",
            "packages/opencode/src/provider/transform.ts",
        ],
    );
    assert_delta(
        &inventory,
        "1.18.27_to_1.18.28",
        &[],
        &["packages/opencode/src/plugin/github-copilot/copilot.ts"],
    );
    assert_exact_strings(
        &inventory["npm_identical_through_1.18.20_to_1.18.28"],
        &["LICENSE", "bin/opencode.exe", "postinstall.mjs"],
    );
    assert_exact_object_keys(
        inventory["openapi_sha256"].as_object().unwrap(),
        &[
            "1.18.20", "1.18.21", "1.18.22", "1.18.23", "1.18.24", "1.18.25", "1.18.26", "1.18.27",
            "1.18.28",
        ],
    );
    assert_eq!(
        inventory["openapi_sha256"]["1.18.28"],
        "00502bd13e9c86f3ca9e765e99a57e06fa9f434ca16f2a714766d1444f8d37f3"
    );
}

fn json(input: &str) -> Value {
    serde_json::from_str(input).expect("fixture is valid JSON")
}

fn assert_nonempty_string(value: &Value, key: &str) {
    assert!(
        value[key].as_str().is_some_and(|text| !text.is_empty()),
        "missing {key}"
    );
}

fn assert_exact_strings(actual: &Value, expected: &[&str]) {
    let actual = actual.as_array().expect("string array");
    assert_eq!(actual.len(), expected.len());
    let actual = actual
        .iter()
        .map(|value| value.as_str().unwrap())
        .collect::<BTreeSet<_>>();
    let expected = expected.iter().copied().collect::<BTreeSet<_>>();
    assert_eq!(actual, expected);
}

fn assert_exact_object_keys(actual: &Map<String, Value>, expected: &[&str]) {
    let actual = actual.keys().map(String::as_str).collect::<BTreeSet<_>>();
    let expected = expected.iter().copied().collect::<BTreeSet<_>>();
    assert_eq!(actual, expected);
}

fn assert_delta(inventory: &Value, hop: &str, added: &[&str], changed: &[&str]) {
    let delta = &inventory["source_deltas"][hop];
    assert_exact_strings(&delta["added"], added);
    assert_exact_strings(&delta["changed"], changed);
    assert_exact_strings(&delta["removed"], &[]);
}
