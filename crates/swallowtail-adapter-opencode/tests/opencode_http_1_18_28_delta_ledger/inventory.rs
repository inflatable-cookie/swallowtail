use serde_json::Value;

use super::{INVENTORY, assert_exact_object_keys, assert_exact_strings, json};

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

fn assert_delta(inventory: &Value, hop: &str, added: &[&str], changed: &[&str]) {
    let delta = &inventory["source_deltas"][hop];
    assert_exact_strings(&delta["added"], added);
    assert_exact_strings(&delta["changed"], changed);
    assert_exact_strings(&delta["removed"], &[]);
}
