//! Research 403 freezes the current official Kimi Code ACP identity and stop.

use serde_json::Value;
use sha2::{Digest, Sha256};
use swallowtail_adapter_kimi::{
    KIMI_CODE_BASELINE_VERSION, KIMI_CODE_LATEST_QUALIFIED_VERSION, kimi_acp_claim,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceNewerVersionPosture, InterfaceSupportStatus,
    InterfaceVersion,
};

const IDENTITY: &[u8] = include_bytes!("fixtures/kimi-code-2.1.1-acp/identity.json");
const DIST_INVENTORY: &[u8] = include_bytes!("fixtures/kimi-code-2.1.1-acp/dist-inventory.json");
const PROTOCOL: &[u8] = include_bytes!("fixtures/kimi-code-2.1.1-acp/protocol.json");

const HOPS: [&str; 13] = [
    "0.39.0", "0.39.1", "0.40.0", "0.40.1", "0.41.0", "0.42.0", "0.43.0", "0.43.1", "2.0.0",
    "2.0.1", "2.0.2", "2.1.0", "2.1.1",
];
const DIST_POINTS: [&str; 7] = [
    "0.43.0", "0.43.1", "2.0.0", "2.0.1", "2.0.2", "2.1.0", "2.1.1",
];
const RUNNER_REGION_SHA256: &str =
    "5ea9279b7c8ef6e78192514225a319497c9fa6573273dd647564e9155f9f3067";

fn json(bytes: &[u8]) -> Value {
    serde_json::from_slice(bytes).expect("frozen currentness JSON is valid")
}

fn text<'a>(value: &'a Value, path: &[&str]) -> &'a str {
    let mut cursor = value;
    for key in path {
        cursor = &cursor[*key];
    }
    cursor
        .as_str()
        .unwrap_or_else(|| panic!("{} is text", path.join(".")))
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn assert_sha256(bytes: &[u8], expected: &str) {
    assert_eq!(sha256(bytes), expected);
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("frozen version is valid")
}

fn changed_modules(before: &Value, after: &Value) -> Vec<String> {
    let before = before.as_object().expect("module map is an object");
    let after = after.as_object().expect("module map is an object");
    before
        .keys()
        .chain(after.keys())
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .filter(|name| before.get(*name) != after.get(*name))
        .map(|name| name.to_string())
        .collect()
}

#[test]
fn identity_inventories_and_selected_bundle_changes_are_frozen() {
    assert_sha256(
        IDENTITY,
        "53c98a5744196239d2c248e3e573f438cdb224b72115437ba0f2f2aa53c123bd",
    );
    assert_sha256(
        DIST_INVENTORY,
        "a2379bc93e72d15e7f0f80ec9f2feb1994a4e70a16654f50ac32aa7761041b53",
    );
    assert_sha256(
        PROTOCOL,
        "22bc8681cdd3a5f8cac83de28f07a442d97d50dc4140268b1ed089e0acdda7cb",
    );

    let identity = json(IDENTITY);
    assert_eq!(identity["npm_latest"], "2.1.1");
    assert_eq!(
        text(&identity, &["github_latest_release", "tag_name"]),
        "@moonshot-ai/kimi-code@2.1.1"
    );
    assert_eq!(
        text(&identity, &["identity_decision", "latest_qualified_stays"]),
        "0.38.0"
    );
    let hops = identity["official_stable_hops_after_ceiling"]
        .as_array()
        .expect("hop ledger is an array");
    assert_eq!(
        hops.iter()
            .map(|hop| hop["version"].as_str().expect("version is text"))
            .collect::<Vec<_>>(),
        HOPS
    );
    for hop in hops {
        assert!(text(hop, &["npm_integrity"]).starts_with("sha512-"));
        assert_eq!(text(hop, &["npm_shasum"]).len(), 40);
        assert_eq!(text(hop, &["npm_tarball_sha256"]).len(), 64);
        assert_eq!(text(hop, &["github_annotated_tag_object"]).len(), 40);
        assert_eq!(text(hop, &["github_commit"]).len(), 40);
        assert_eq!(text(hop, &["github_tree"]).len(), 40);
        assert_eq!(hop["npm_bin"]["kimi"], "dist/main.mjs");
        assert_eq!(hop["node_engine"], ">=22.19.0");
    }
    assert_eq!(
        identity["release_adjacency"]["unpublished_or_absent_stable_points"],
        serde_json::json!([
            "0.38.1",
            "0.39.2",
            "0.40.2",
            "0.41.1",
            "0.42.1",
            "0.43.2",
            "no stable 1.x release",
            "2.0.3",
            "2.1.2"
        ])
    );

    let current = &identity["official_stable_hops_after_ceiling"][12];
    assert_eq!(
        current["npm_tarball_sha256"],
        "6690a29d7b5e14812754dd100136b7f4ee2577add8895f00fe27025b4412049f"
    );
    assert_eq!(
        current["github_commit"],
        "f67e6398fb3210ad8ace970e2dfd5bcc984ed61f"
    );

    let inventory = json(DIST_INVENTORY);
    let points = inventory["points"].as_object().expect("package point map");
    assert_eq!(
        points.keys().map(String::as_str).collect::<Vec<_>>(),
        DIST_POINTS
    );
    for (name, count) in [
        ("0.43.0", 543),
        ("0.43.1", 545),
        ("2.0.0", 541),
        ("2.0.1", 541),
        ("2.0.2", 541),
        ("2.1.0", 541),
        ("2.1.1", 541),
    ] {
        let files = points[name]["files"]
            .as_array()
            .expect("complete file list");
        assert_eq!(files.len(), count);
        let mut paths = files
            .iter()
            .map(|entry| entry[0].as_str().expect("path is text"))
            .collect::<Vec<_>>();
        paths.sort_unstable();
        paths.dedup();
        assert_eq!(paths.len(), count);
    }
    let transitions = inventory["transitions"]
        .as_array()
        .expect("published hop inventory");
    assert_eq!(transitions.len(), 6);
    assert_eq!(
        transitions
            .iter()
            .map(|hop| (
                hop["added"].as_array().unwrap().len(),
                hop["removed"].as_array().unwrap().len(),
                hop["changed"].as_array().unwrap().len(),
                hop["unchanged_count"].as_u64().unwrap(),
            ))
            .collect::<Vec<_>>(),
        [
            (75, 73, 4, 466),
            (69, 73, 3, 469),
            (0, 0, 3, 538),
            (69, 69, 3, 469),
            (69, 69, 3, 469),
            (0, 0, 2, 539),
        ]
    );

    let protocol = json(PROTOCOL);
    let authority = &protocol["terminal_authority"];
    assert_eq!(
        authority["branch_source_contains_terminal_disabled_guard"],
        true
    );
    assert_eq!(authority["adapter_capabilities"]["terminal"], false);
    assert_eq!(authority["adapter_capabilities"]["auth_terminal"], false);
    assert_eq!(
        text(authority, &["branch_condition"]),
        "!this.connection.terminalEnabled || !isBashToolInvocation(args, options)"
    );
    assert_eq!(
        text(authority, &["local_spawn_expression"]),
        "this.local.spawn(command, args, { ...options, cwd: ... })"
    );
    assert_eq!(
        authority["github_blob"],
        "9016d48b643f35b263449d98dee25597a9a24d30"
    );
    assert_eq!(
        text(
            authority,
            &["adapter_capabilities", "protocol_fake_test_scope"]
        ),
        "adapter-side ACP fake only; does not execute the Kimi provider-local process runner or prove its child lifecycle"
    );
    let runner_points = authority["npm_region_sha256_by_point"]
        .as_object()
        .expect("runner region map");
    assert_eq!(runner_points.len(), 7);
    assert!(
        runner_points
            .values()
            .all(|value| value.as_str() == Some(RUNNER_REGION_SHA256))
    );
    let modules = &protocol["acp_server_modules"];
    for point in DIST_POINTS {
        assert_eq!(modules[point].as_object().unwrap().len(), 25);
    }
    assert_eq!(
        changed_modules(&modules["0.43.1"], &modules["2.0.0"]),
        ["slash.ts"]
    );
    assert_eq!(
        changed_modules(&modules["2.0.0"], &modules["2.0.1"]),
        ["server.ts"]
    );
    assert_eq!(
        changed_modules(&modules["2.0.1"], &modules["2.0.2"]),
        ["index.ts", "model-catalog.ts", "server.ts", "session.ts"]
    );
    assert!(changed_modules(&modules["2.0.2"], &modules["2.1.0"]).is_empty());
    assert!(changed_modules(&modules["2.1.0"], &modules["2.1.1"]).is_empty());
    let hop_notes = protocol["all_hop_classification"]
        .as_array()
        .expect("all published hops are classified");
    assert_eq!(hop_notes.len(), 13);
    assert!(hop_notes[7..].iter().all(|hop| hop["research"] == "403"));
    assert_eq!(hop_notes[8]["from"], "0.43.1");
    assert_eq!(hop_notes[8]["to"], "2.0.0");
    assert!(text(&hop_notes[8], &["class"]).contains("available-command discovery"));
    assert_eq!(
        changed_modules(&modules["0.43.1"], &modules["2.0.0"]),
        ["slash.ts"]
    );

    let spawn = &protocol["spawn_control_investigation"];
    assert_eq!(
        text(spawn, &["source_commit"]),
        "f67e6398fb3210ad8ace970e2dfd5bcc984ed61f"
    );
    let source_files = spawn["source_files"]
        .as_object()
        .expect("pinned source file and blob map");
    assert_eq!(source_files.len(), 10);
    for (path, blob) in [
        (
            "packages/acp-server/src/acp-terminal/acpTerminalRunner.ts",
            "9016d48b643f35b263449d98dee25597a9a24d30",
        ),
        (
            "packages/acp-server/src/acp-fs/acpConnection.ts",
            "9d12773ec1339219d66bb1e05712157c51da61b2",
        ),
        (
            "packages/acp-server/src/server.ts",
            "c2157face164225d307fa434f413abdbe8cac02d",
        ),
        (
            "packages/acp-server/src/config-options.ts",
            "beeb4300833e6db38bd16f35dd5e57607c841fad",
        ),
        (
            "packages/acp-server/src/start.ts",
            "5b60cac574d754878f02b8cac47414fec3da7802",
        ),
        (
            "docs/en/reference/kimi-acp.md",
            "9c24d7a0e9482877872089e26eb06a3d60de9577",
        ),
        (
            "docs/en/reference/kimi-command.md",
            "092a67e1f2a57d709db3ef2d8c2bc8640ded94ec",
        ),
        (
            "docs/en/configuration/config-files.md",
            "19a644393f9dfaef9d73858783b2a6956ac8f21d",
        ),
        (
            "docs/en/configuration/overrides.md",
            "512ba61d9f9302694ee7487e10a92b5ce6be0365",
        ),
        (
            "docs/en/customization/hooks.md",
            "72ac0d77a7c4ec90495f6afe0b6af52872511131",
        ),
    ] {
        assert_eq!(text(spawn, &["source_files", path]), blob);
    }
    let branches = spawn["branches"].as_array().expect("spawn branch ledger");
    assert_eq!(branches.len(), 3);
    assert_eq!(text(&branches[0], &["id"]), "terminal_disabled");
    assert_eq!(
        text(&branches[0], &["guard"]),
        "!this.connection.terminalEnabled"
    );
    assert_eq!(text(&branches[1], &["id"]), "non_recognized_shape_fallback");
    assert_eq!(
        text(&branches[1], &["guard"]),
        "!isBashToolInvocation(args, options)"
    );
    assert_eq!(
        text(&branches[1], &["predicate_shape"]),
        "args.length === 2; args[0] === '-c'; env.NO_COLOR === '1'; env.TERM === 'dumb'"
    );
    assert_eq!(
        branches[1]["predicate_checks_executable"].as_bool(),
        Some(false)
    );
    assert_eq!(
        text(&branches[0], &["action"]),
        text(&branches[1], &["action"])
    );
    assert_eq!(text(&branches[2], &["id"]), "predicate_match_with_terminal");
    assert_eq!(
        branches[2]["predicate_checks_executable"].as_bool(),
        Some(false)
    );
    assert_eq!(
        text(&branches[2], &["action"]),
        "this.connection.get().createTerminal(...) then AcpTerminalProcess"
    );

    let controls = spawn["shipped_controls"]
        .as_array()
        .expect("shipped control ledger");
    assert_eq!(controls.len(), 6);
    assert_eq!(
        controls
            .iter()
            .map(|control| text(control, &["id"]))
            .collect::<Vec<_>>(),
        [
            "clientCapabilities.terminal",
            "tools.enabled_disabled",
            "permission.rules",
            "PreToolUse hooks",
            "ACP session config options",
            "kimi acp and KIMI_CODE_HOME",
        ]
    );
    assert!(
        controls
            .iter()
            .all(|control| control["covers_every_spawn_branch"].as_bool() == Some(false))
    );
    assert_eq!(
        text(spawn, &["result"]),
        "No qualifying shipped per-session control was identified that disables or host-mediates both provider-local spawn branches."
    );
}

#[test]
fn the_production_claim_keeps_its_old_points_and_fail_closed_ceiling() {
    let claim = kimi_acp_claim();
    assert_eq!(claim.id().as_str(), "kimi.acp.executable-window-5");
    assert_eq!(
        claim.newer_version_posture(),
        InterfaceNewerVersionPosture::QualifiedOnly
    );
    assert_eq!(KIMI_CODE_BASELINE_VERSION, "0.28.1");
    assert_eq!(KIMI_CODE_LATEST_QUALIFIED_VERSION, "0.38.0");

    let milestones = claim.milestones().collect::<Vec<_>>();
    assert_eq!(milestones.len(), 2);
    assert_eq!(milestones[0].minimum().as_str(), "0.28.1");
    assert_eq!(milestones[0].maximum().as_str(), "0.28.1");
    assert_eq!(
        milestones[0].support_status(),
        InterfaceSupportStatus::Deprecated
    );
    assert_eq!(milestones[1].minimum().as_str(), "0.29.0");
    assert_eq!(milestones[1].maximum().as_str(), "0.38.0");
    assert_eq!(claim.latest_qualified().as_str(), "0.38.0");
    assert_eq!(
        claim
            .exclusions()
            .map(InterfaceVersion::as_str)
            .collect::<Vec<_>>(),
        ["0.39.0", "0.39.1"]
    );

    for point in HOPS {
        assert_eq!(
            claim.assess(&version(point)),
            InterfaceCompatibilityAssessment::Incompatible,
            "{point} fails closed under the unchanged claim"
        );
    }
    for point in ["0.39.0", "0.39.1"] {
        assert!(claim.exclusions().any(|value| value.as_str() == point));
    }
    for point in ["0.40.0", "0.43.1", "2.0.0", "2.1.1"] {
        assert!(!claim.exclusions().any(|value| value.as_str() == point));
    }
}
