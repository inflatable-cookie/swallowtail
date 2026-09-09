use super::support::{DIST_INVENTORY, PROTOCOL, json, strings};

#[test]
fn shipped_tree_inventory_ledgers_are_exact() {
    let inventory = json(DIST_INVENTORY);
    let protocol = json(PROTOCOL);
    assert_eq!(
        strings(&inventory["compared"]),
        ["0.84.4", "0.85.0", "0.85.1"]
    );
    assert_eq!(inventory["package_file_counts"]["0.84.4"], 1044);
    assert_eq!(inventory["package_file_counts"]["0.85.0"], 1249);
    assert_eq!(inventory["package_file_counts"]["0.85.1"], 1056);
    let first = &inventory["from_0_84_4_to_0_85_0"];
    assert_eq!(first["added_count"], 246);
    assert_eq!(first["removed_count"], 41);
    assert_eq!(first["changed_count"], 207);
    assert_eq!(first["identical_count"], 796);
    assert_eq!(
        first["added_sha256"],
        "540e43fa15c1ca99dae97ec9a345020ebc1403a24d8b3799971cb25404d8a5f6"
    );
    assert_eq!(
        first["removed_sha256"],
        "2fda9732dae9ccc474292779d85ad68e4678b59935178e67a27ea1946b293e4c"
    );
    assert_eq!(
        first["changed_sha256"],
        "7d10e22518873d5090d38dafe820a177c7705274385c47b5e3cadb2a2ef41000"
    );
    assert_eq!(
        first["identical_sha256"],
        "952d8ae957afd244d9ebac2b2e3a1b2365198e80903f42674575a5fe785eb162"
    );
    assert_eq!(first["added"].as_array().expect("added").len(), 246);
    assert_eq!(first["removed"].as_array().expect("removed").len(), 41);
    assert_eq!(first["changed"].as_array().expect("changed").len(), 207);
    assert_eq!(first["identical"].as_array().expect("identical").len(), 796);
    let second = &inventory["from_0_85_0_to_0_85_1"];
    assert_eq!(second["added_count"], 20);
    assert_eq!(second["removed_count"], 213);
    assert_eq!(second["changed_count"], 48);
    assert_eq!(second["identical_count"], 988);
    assert_eq!(
        second["added_sha256"],
        "a19ed70561e67ccc39caaf7ae054455940be29eb774b178a219eb63149dd2704"
    );
    assert_eq!(
        second["removed_sha256"],
        "bbccea440258662f443a6f699dee228c40673cebf78cadea67d51c566496a4c8"
    );
    assert_eq!(
        second["changed_sha256"],
        "fa8abb675896ed3f58dff39bfe0b5b240cacfe9b2481e276789f60e408f8f1b7"
    );
    assert_eq!(
        second["identical_sha256"],
        "f65ba401c4e5f1738f78927c29e22cfd29a2162596b2891816e0d69d3ed55af7"
    );
    assert_eq!(inventory["identical_through_0_84_4_0_85_1_count"], 788);
    assert_eq!(
        strings(&inventory["mapped_shipped_files_byte_identical_through_0_84_4_0_85_1"]),
        [
            "dist/modes/rpc/rpc-types.js",
            "dist/modes/rpc/rpc-mode.js",
            "dist/modes/rpc/jsonl.js",
            "dist/core/session-cwd.js",
            "dist/modes/json-event.js",
        ]
    );
    assert_eq!(inventory["args_js_byte_identical_0_84_4_and_0_85_1"], true);
    assert_eq!(
        protocol["packaging_deltas"]["package_file_counts"]["0.85.1"],
        1056
    );
    assert_eq!(
        protocol["packaging_deltas"]["from_0_84_4_to_0_85_0"]["experimental_files_added"],
        172
    );
    assert_eq!(
        protocol["packaging_deltas"]["from_0_85_0_to_0_85_1"]["experimental_files_removed"],
        172
    );
    assert_eq!(
        protocol["packaging_deltas"]["rpc_dispatch_command_count"],
        33
    );
    assert_eq!(
        protocol["packaging_deltas"]["rpc_dispatch_commands_unchanged_from_0_84_4"],
        true
    );
}

#[test]
fn mapped_file_hashes_are_frozen_across_hops() {
    let inventory = json(DIST_INVENTORY);
    let hashes = &inventory["hashes"];
    for path in [
        "dist/modes/rpc/rpc-types.js",
        "dist/modes/rpc/rpc-mode.js",
        "dist/modes/rpc/jsonl.js",
        "dist/core/session-cwd.js",
        "dist/modes/json-event.js",
    ] {
        assert_eq!(
            hashes[path]["0.84.4"], hashes[path]["0.85.0"],
            "{path} changed 0.84.4 -> 0.85.0"
        );
        assert_eq!(
            hashes[path]["0.85.0"], hashes[path]["0.85.1"],
            "{path} changed 0.85.0 -> 0.85.1"
        );
    }
    assert_eq!(
        hashes["dist/cli/args.js"]["0.84.4"],
        hashes["dist/cli/args.js"]["0.85.1"]
    );
    assert_ne!(
        hashes["dist/cli/args.js"]["0.84.4"],
        hashes["dist/cli/args.js"]["0.85.0"]
    );
    assert_eq!(
        hashes["dist/core/agent-session.js"]["0.85.0"],
        hashes["dist/core/agent-session.js"]["0.85.1"]
    );
    assert_ne!(
        hashes["dist/core/agent-session.js"]["0.84.4"],
        hashes["dist/core/agent-session.js"]["0.85.0"]
    );
    assert_eq!(
        hashes["docs/rpc.md"]["0.85.0"],
        hashes["docs/rpc.md"]["0.85.1"]
    );
}
