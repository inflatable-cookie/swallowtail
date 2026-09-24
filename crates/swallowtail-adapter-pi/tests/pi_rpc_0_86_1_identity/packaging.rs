use super::support::{DIST_INVENTORY, PROTOCOL, json, strings};

#[test]
fn shipped_tree_inventory_ledgers_are_exact() {
    let inventory = json(DIST_INVENTORY);
    let protocol = json(PROTOCOL);
    assert_eq!(
        strings(&inventory["compared"]),
        ["0.85.1", "0.86.0", "0.86.1"]
    );
    assert_eq!(inventory["package_file_counts"]["0.85.1"], 1056);
    assert_eq!(inventory["package_file_counts"]["0.86.0"], 1094);
    assert_eq!(inventory["package_file_counts"]["0.86.1"], 1100);
    let first = &inventory["from_0_85_1_to_0_86_0"];
    assert_eq!(first["added_count"], 61);
    assert_eq!(first["removed_count"], 23);
    assert_eq!(first["changed_count"], 233);
    assert_eq!(first["identical_count"], 800);
    assert_eq!(
        first["added_sha256"],
        "21e437850cd9765853f3ed45924757c2d6e985f6441603bd0b7f2747bc64c889"
    );
    assert_eq!(
        first["removed_sha256"],
        "fe35a1e6b57b50ada9d9731606be98fc19a2c8febeec4f20f356298d831d755e"
    );
    assert_eq!(
        first["changed_sha256"],
        "36c8eb70d69a4f19c9424992e90863e077211b845c52f76f3267818c9570b1cb"
    );
    assert_eq!(
        first["identical_sha256"],
        "a15d0aa58e7699c4b864945bcf7e9be992e6990d3ca57e0c8da0c8d826dfde44"
    );
    assert_eq!(first["added"].as_array().expect("added").len(), 61);
    assert_eq!(first["removed"].as_array().expect("removed").len(), 23);
    assert_eq!(first["changed"].as_array().expect("changed").len(), 233);
    assert_eq!(first["identical"].as_array().expect("identical").len(), 800);
    let second = &inventory["from_0_86_0_to_0_86_1"];
    assert_eq!(second["added_count"], 10);
    assert_eq!(second["removed_count"], 4);
    assert_eq!(second["changed_count"], 41);
    assert_eq!(second["identical_count"], 1049);
    assert_eq!(
        second["added_sha256"],
        "4493a2328d51869e5766b323bb18ab31aaa5e1fc56022c388f26f9b4a81571b0"
    );
    assert_eq!(
        second["removed_sha256"],
        "bacefdf3754f88cb8a9360cf589a672d3b990941dfe4457103b052e01acd8fca"
    );
    assert_eq!(
        second["changed_sha256"],
        "07ed26251d5bebdaa3f423279309032058a5dbc20acab98ea161d2e8ec75ed4e"
    );
    assert_eq!(
        second["identical_sha256"],
        "06bca2bf15fd5141bff0326e837834db7cedb67d8df3cc64c28e7ce42c766cf1"
    );
    assert_eq!(inventory["identical_through_0_85_1_0_86_1_count"], 793);
    assert_eq!(
        strings(&inventory["mapped_shipped_files_byte_identical_through_0_85_1_0_86_1"]),
        [
            "dist/modes/rpc/rpc-types.js",
            "dist/modes/rpc/jsonl.js",
            "dist/core/session-cwd.js",
            "dist/modes/json-event.js",
            "docs/rpc.md",
        ]
    );
    assert_eq!(inventory["args_js_byte_identical_0_85_1_and_0_86_0"], true);
    assert_eq!(
        inventory["agent_session_js_byte_identical_0_86_0_and_0_86_1"],
        true
    );
    assert_eq!(
        protocol["packaging_deltas"]["package_file_counts"]["0.86.1"],
        1100
    );
    assert_eq!(
        protocol["packaging_deltas"]["rpc_dispatch_command_count"],
        33
    );
    assert_eq!(
        protocol["packaging_deltas"]["rpc_dispatch_commands_unchanged_from_0_85_1"],
        true
    );
    assert_eq!(
        protocol["packaging_deltas"]["cli_js_digest_unchanged_0_85_1_through_0_86_1"],
        true
    );
}

#[test]
fn mapped_file_hashes_are_frozen_across_hops() {
    let inventory = json(DIST_INVENTORY);
    let hashes = &inventory["hashes"];
    for path in [
        "dist/modes/rpc/rpc-types.js",
        "dist/modes/rpc/jsonl.js",
        "dist/core/session-cwd.js",
        "dist/modes/json-event.js",
        "docs/rpc.md",
        "dist/cli.js",
        "dist/cli/setup.js",
    ] {
        assert_eq!(
            hashes[path]["0.85.1"], hashes[path]["0.86.0"],
            "{path} changed 0.85.1 -> 0.86.0"
        );
        assert_eq!(
            hashes[path]["0.86.0"], hashes[path]["0.86.1"],
            "{path} changed 0.86.0 -> 0.86.1"
        );
    }
    assert_eq!(
        hashes["dist/cli/args.js"]["0.85.1"],
        hashes["dist/cli/args.js"]["0.86.0"]
    );
    assert_ne!(
        hashes["dist/cli/args.js"]["0.86.0"],
        hashes["dist/cli/args.js"]["0.86.1"]
    );
    assert_ne!(
        hashes["dist/modes/rpc/rpc-mode.js"]["0.85.1"],
        hashes["dist/modes/rpc/rpc-mode.js"]["0.86.0"]
    );
    assert_eq!(
        hashes["dist/modes/rpc/rpc-mode.js"]["0.86.0"],
        hashes["dist/modes/rpc/rpc-mode.js"]["0.86.1"]
    );
    assert_eq!(
        hashes["dist/core/agent-session.js"]["0.86.0"],
        hashes["dist/core/agent-session.js"]["0.86.1"]
    );
    assert_ne!(
        hashes["dist/core/agent-session.js"]["0.85.1"],
        hashes["dist/core/agent-session.js"]["0.86.0"]
    );
}
