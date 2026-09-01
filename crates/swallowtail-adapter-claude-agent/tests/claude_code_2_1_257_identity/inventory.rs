use super::support::{DIST_INVENTORY, assert_exact_string_set, assert_sha256, json};

#[test]
fn wrapper_and_platform_inventories_are_exact() {
    let inventory = json(DIST_INVENTORY);
    assert_eq!(
        inventory["compared"],
        serde_json::json!(["2.1.252", "2.1.257"])
    );
    assert_eq!(inventory["package_file_counts"]["wrapper-2.1.252"], 7);
    assert_eq!(inventory["package_file_counts"]["wrapper-2.1.257"], 7);
    assert_eq!(inventory["package_file_counts"]["darwin-arm64-2.1.252"], 4);
    assert_eq!(inventory["package_file_counts"]["darwin-arm64-2.1.257"], 4);
    assert_eq!(inventory["package_file_counts"]["linux-x64-2.1.252"], 4);
    assert_eq!(inventory["package_file_counts"]["linux-x64-2.1.257"], 4);
    assert_eq!(inventory["not_a_complete_semantic_changelog"], true);

    let wrapper = &inventory["from_2_1_252_to_2_1_257_wrapper"];
    assert_exact_string_set(&wrapper["added"], &[]);
    assert_exact_string_set(&wrapper["removed"], &[]);
    assert_exact_string_set(&wrapper["changed"], &["package.json", "sdk-tools.d.ts"]);
    assert_exact_string_set(
        &wrapper["identical"],
        &[
            "LICENSE.md",
            "README.md",
            "bin/claude.exe",
            "cli-wrapper.cjs",
            "install.cjs",
        ],
    );

    let darwin = &inventory["from_2_1_252_to_2_1_257_darwin_arm64"];
    assert_exact_string_set(&darwin["added"], &[]);
    assert_exact_string_set(&darwin["removed"], &[]);
    assert_exact_string_set(&darwin["changed"], &["claude", "package.json"]);
    assert_exact_string_set(&darwin["identical"], &["LICENSE.md", "README.md"]);

    let linux = &inventory["from_2_1_252_to_2_1_257_linux_x64"];
    assert_exact_string_set(&linux["added"], &[]);
    assert_exact_string_set(&linux["removed"], &[]);
    assert_exact_string_set(&linux["changed"], &["claude", "package.json"]);
    assert_exact_string_set(&linux["identical"], &["LICENSE.md", "README.md"]);
}

#[test]
fn mapped_feeding_wrapper_files_stay_byte_identical_except_metadata() {
    let hashes = &json(DIST_INVENTORY)["hashes"];
    assert_sha256(
        &hashes["wrapper.cli-wrapper.cjs"]["2.1.252"],
        "61ad63033d9c8155d5e60a29f45dc4665afa07631c0b108e62cc83bf45ba490e",
    );
    assert_eq!(
        hashes["wrapper.cli-wrapper.cjs"]["2.1.252"],
        hashes["wrapper.cli-wrapper.cjs"]["2.1.257"]
    );
    assert_sha256(
        &hashes["wrapper.install.cjs"]["2.1.252"],
        "5cbab1670597f492cd4eeb946f3c344ebcb1fbd43c623ba192c9b33744461b85",
    );
    assert_eq!(
        hashes["wrapper.install.cjs"]["2.1.252"],
        hashes["wrapper.install.cjs"]["2.1.257"]
    );
    assert_ne!(
        hashes["wrapper.sdk-tools.d.ts"]["2.1.252"],
        hashes["wrapper.sdk-tools.d.ts"]["2.1.257"]
    );
    assert_ne!(
        hashes["darwin-arm64.claude"]["2.1.252"],
        hashes["darwin-arm64.claude"]["2.1.257"]
    );
    assert_sha256(
        &hashes["darwin-arm64.claude"]["2.1.257"],
        "64590d7d9d9c189d33fb3dfa58c5408eaf2a10fe556bd84155d95efaab46b60e",
    );
    assert_sha256(
        &hashes["linux-x64.claude"]["2.1.257"],
        "9a64bda9d8722a1fa05bef9a5961d07e0331b99597eda9e2f6a732f3a0ff7f05",
    );
}
