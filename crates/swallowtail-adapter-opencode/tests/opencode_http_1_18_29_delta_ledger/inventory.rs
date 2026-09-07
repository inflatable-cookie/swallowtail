use serde_json::Value;

use super::{INVENTORY, assert_exact_object_keys, assert_exact_strings, json};

#[test]
fn artifact_tree_delta_ledger_is_mutation_sensitive() {
    let inventory = json(INVENTORY);
    assert_exact_strings(&inventory["compared"], &["1.18.28", "1.18.29"]);
    assert_eq!(inventory["npm_package_file_counts"]["1.18.28"], 4);
    assert_eq!(inventory["npm_package_file_counts"]["1.18.29"], 4);
    assert_eq!(
        inventory["implementation_source_file_counts"]["1.18.28"],
        407
    );
    assert_eq!(
        inventory["implementation_source_file_counts"]["1.18.29"],
        407
    );
    assert_exact_object_keys(
        inventory["source_deltas"].as_object().unwrap(),
        &["1.18.28_to_1.18.29"],
    );
    assert_delta(
        &inventory,
        "1.18.28_to_1.18.29",
        &[],
        &["packages/opencode/src/plugin/openai/codex.ts"],
    );
    assert_exact_strings(
        &inventory["npm_identical_through_1.18.28_to_1.18.29"],
        &["LICENSE", "bin/opencode.exe", "postinstall.mjs"],
    );
    assert_eq!(
        inventory["openapi_sha256"]["1.18.28"],
        "00502bd13e9c86f3ca9e765e99a57e06fa9f434ca16f2a714766d1444f8d37f3"
    );
    assert_eq!(
        inventory["openapi_sha256"]["1.18.29"],
        "00502bd13e9c86f3ca9e765e99a57e06fa9f434ca16f2a714766d1444f8d37f3"
    );
    assert_eq!(
        inventory["npm_invariant_hashes"]["LICENSE"],
        "625f0f619133f89bbbb2abe37369613dfa1885eba1e50d02170deb62bb42cb6b"
    );
    assert_eq!(
        inventory["npm_invariant_hashes"]["bin/opencode.exe"],
        "21c366f53283d5b5e1cdbbec2aafc98286b4c1075a4c3d5afd5ce1f5c9bf46dd"
    );
    assert_eq!(
        inventory["npm_invariant_hashes"]["postinstall.mjs"],
        "5a7c990fe552e76b16422cdba3f4b0550590c7a487f7932c773362f74317c87b"
    );
    assert_eq!(
        inventory["changed_source_hashes"]["packages/opencode/src/plugin/openai/codex.ts"]["1.18.28"],
        "a1e214a02cb93cb4833a35b5049bbee4707a0eece265df9902755546adf636b2"
    );
    assert_eq!(
        inventory["changed_source_hashes"]["packages/opencode/src/plugin/openai/codex.ts"]["1.18.29"],
        "6450893522f00fae9039ce6306d461b223580e0c0e4eb6fc627ea1e5795ec5ca"
    );
}

fn assert_delta(inventory: &Value, hop: &str, added: &[&str], changed: &[&str]) {
    let delta = &inventory["source_deltas"][hop];
    assert_exact_strings(&delta["added"], added);
    assert_exact_strings(&delta["changed"], changed);
    assert_exact_strings(&delta["removed"], &[]);
}
