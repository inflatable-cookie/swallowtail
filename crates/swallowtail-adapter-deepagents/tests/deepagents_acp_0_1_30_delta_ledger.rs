//! Identity and compatible-extension evidence for official npm
//! `deepagents-acp@0.1.30` (g05.074, Research 308 family fourteen).
//!
//! This corpus freezes the exact `0.1.25` baseline plus all five published
//! stable successors through `0.1.30`, retrieved as exact npm tarballs into
//! `/tmp`, hashed against registry digests, and statically inspected together
//! with the exact-pinned `deepagents@1.12.4..=1.13.4` runtime dependencies
//! without executing a downloaded artifact. The assertions are
//! mutation-sensitive: they fail if a package identity, the stale ACP
//! registry metadata, the selected-surface classification, the
//! unreachable-CLI-delta finding, or the exact-pin decision drifts.

use serde_json::Value;

const IDENTITY: &str = include_str!("fixtures/deepagents-acp-0.1.30/identity.json");
const SURFACE: &str = include_str!("fixtures/deepagents-acp-0.1.30/surface-ledger.json");

const BASELINE: &str = "0.1.25";
const OFFICIAL: &str = "0.1.30";
const BASELINE_CLI: &str = "68b7d6cb31d181a399f623a4c6486892bf7d408aec61cac0a3ea9e033baa2319";
const CURRENT_CLI: &str = "508603abc388abac1dbe7b1d57e45ebf611d2ec6644fe4e3349c5b335064d830";
const CURRENT_INDEX_JS: &str = "6ce345349857c6f74965512cfa9fe5baaa4ae42f0401cbbdd5643c724d161358";
const CURRENT_INDEX_CJS: &str = "742f5b4e60da0d11db04aeb54da2e18d87e2babc821a03b8f7b51e4fa5ddc3fe";
const CURRENT_INDEX_D_TS: &str = "21449c09ce02096a073e4c4f77233a6d0326b48092aa54a44ae519b0e87de20b";
const BASELINE_DEEPAGENTS: &str =
    "cea1b79d542a9ee695d103d4371b5265b83eac157cbbef8101c431f7062654ec";
const CURRENT_DEEPAGENTS: &str = "a00d90a3041794cc4f80186b0eaf519f4fdbccea16dc8576771aeb0d8b3c800c";

const NPM_POINTS: [(&str, &str, &str); 6] = [
    (
        "0.1.25",
        "6a56fa60e985a0681217cd20b1e21c0f7782fb10ebed6728f2865346ba137141",
        "cddb5563aafc9fc22e67760c2ac906187c69e83d1ed73c36ae13db04c35cdb5e",
    ),
    (
        "0.1.26",
        "876f0fe25bfde92548cabe42b5b8647a9f23ddcd55d0caa27c348f0072a46a9e",
        "403ebcacf91e8ed7115a795d408d18e57d3ea78f0bf41ab446a23adbb0729501",
    ),
    (
        "0.1.27",
        "ea953abd732e564146de7f9009c82d6c2a67d3d50e63c5234a5367fc0b355d6e",
        "30b8c2010da4f41c48fdb28c3535fdeb1919c4263fb56ef1533242d49bc24fde",
    ),
    (
        "0.1.28",
        "00206a23b2075fdb49ade06001425a9c41371129bb669a76af8937762c902053",
        "305444ba90aaa58228c8643255639fd024b774082dc1a485c99be3673ea82f76",
    ),
    (
        "0.1.29",
        "ed5b44b6efc27f450fb02f80188266f952e360de20bfde81bdea997dee5ef010",
        "5f22d067b62b6f87252c98585bc73bfe949803efb6209fe88639600386bc5649",
    ),
    (
        "0.1.30",
        "c9bc8b95d779fc120d0614bbc37f8d6d35361376b19a5c77d0ae4e870823a661",
        "1f7f4b7af7d87dff92bcd612ca019524aee4be8eb59df373c9397780243311b8",
    ),
];

const DEEPAGENTS_POINTS: [(&str, &str, &str); 6] = [
    (
        "1.12.4",
        "cea1b79d542a9ee695d103d4371b5265b83eac157cbbef8101c431f7062654ec",
        "c8e0779d4097894f460a0126c1e45e21d4b3bc6a456c8b7d8823c8930f1586ba",
    ),
    (
        "1.13.0",
        "e5e99d838f176f99bab8e7e35969fb183f96eadb070babbd5e82676e1e4f7e19",
        "7f6b67257714217528a40334f84c50abfa270539ce6091b673973cd931c319bf",
    ),
    (
        "1.13.1",
        "ea8d29d6d1510ceb4326abd38a66d0f1835c685c848f87940b486e4980db1a1c",
        "cbbf2ba702505431ee841775a243cb9a6207c9ae0de90cdaf01b8ed16e09748c",
    ),
    (
        "1.13.2",
        "5af18e3eb2515318610d61f2d8592bf34089288ebb2392e4e2173a35d29ec727",
        "4bc6b4b323071a0c075ed453d4fe4e696945ce901b62e334b581ef8d723861b3",
    ),
    (
        "1.13.3",
        "6b2e09c9c7a9e38c999c0f07b9ba88df0664779b07b72a683eccbfc5c8a03eab",
        "1b627a746b2f6ef10536af9f41afb60f9f65e2dfadaacf3a344a22e34cc183fc",
    ),
    (
        "1.13.4",
        "a00d90a3041794cc4f80186b0eaf519f4fdbccea16dc8576771aeb0d8b3c800c",
        "03d74e6e1c03414ada4c27a942bdb9e94a769cbc699911921b8ca2e1f37c0d6f",
    ),
];

fn fixture(body: &str, name: &str) -> Value {
    serde_json::from_str(body).unwrap_or_else(|error| panic!("{name}: {error}"))
}

#[test]
fn identity_freezes_the_baseline_and_every_published_successor() {
    let identity = fixture(IDENTITY, "identity");
    assert_eq!(identity["axis"], "deepagents-acp.package");
    assert_eq!(identity["route"], "deepagents.acp");
    assert_eq!(identity["official"]["version"], OFFICIAL);
    assert_eq!(
        identity["official"]["published"],
        "2026-09-09T17:34:53.193Z"
    );
    assert_eq!(
        identity["official"]["integrity"],
        "sha512-moNlTkJN7JyowNXETdEaGJenpFtZ45xUO+oJYUYBOuebi4FuB0ChGhq7jblPY/JdnFbtX44Waiqotq/3O1fM7A=="
    );
    assert_eq!(identity["official"]["cli_js_sha256"], CURRENT_CLI);
    assert_eq!(identity["official"]["index_js_sha256"], CURRENT_INDEX_JS);
    assert_eq!(identity["official"]["index_cjs_sha256"], CURRENT_INDEX_CJS);
    assert_eq!(
        identity["official"]["index_d_ts_sha256"],
        CURRENT_INDEX_D_TS
    );
    assert_eq!(identity["host"]["present"], false);
    assert_eq!(identity["official"]["tarball_installed"], false);

    let reproduction = &identity["baseline_reproduction"];
    assert_eq!(reproduction["version"], BASELINE);
    assert_eq!(
        reproduction["tarball_sha256"],
        "6a56fa60e985a0681217cd20b1e21c0f7782fb10ebed6728f2865346ba137141"
    );
    assert_eq!(
        reproduction["npm_integrity"],
        "sha512-5S6Rpd74vV3YKVxAEqQkXKek+y1ChTpL0D2xf+WLaAYneJQZ9haZ4lPgjPy2VvszqErVsSr+T5tq8vdjuAWShQ=="
    );
    assert_eq!(
        reproduction["package_json_sha256"],
        "cddb5563aafc9fc22e67760c2ac906187c69e83d1ed73c36ae13db04c35cdb5e"
    );
    assert_eq!(reproduction["matches_research_157_and_206"], true);
    assert_eq!(
        reproduction["runtime_dependency_tarball_sha256"],
        BASELINE_DEEPAGENTS
    );
    assert_eq!(
        reproduction["runtime_dependency_matches_research_206"],
        true
    );

    let claim = &identity["claim_at_observation"];
    assert_eq!(claim["point"], BASELINE);
    assert_eq!(claim["posture"], "QualifiedOnly");
    assert_eq!(claim["behavior_revision"], "deepagents.acp.stdio-v1");
    assert_eq!(claim["claim_id"], "deepagents.acp.package-window-1");

    let ledger = fixture(SURFACE, "surface-ledger");
    let packages = ledger["npm_packages"].as_object().expect("npm packages");
    assert_eq!(packages.len(), 6, "all six published points freeze");
    for (version, tarball, package_json) in NPM_POINTS {
        let entry = &packages[version];
        assert_eq!(
            entry["tarball_sha256"], tarball,
            "{version} tarball drifted"
        );
        assert_eq!(
            entry["package_json_sha256"], package_json,
            "{version} package.json drifted"
        );
        assert_eq!(
            entry["tarball_sha256"].as_str().unwrap_or_default().len(),
            64
        );
    }
    assert_eq!(
        packages["0.1.25"]["cli_js_sha256"], BASELINE_CLI,
        "baseline CLI must keep reproducing Research 157"
    );
    assert_eq!(packages["0.1.26"]["cli_js_sha256"], BASELINE_CLI);
    assert_eq!(packages["0.1.27"]["cli_js_sha256"], BASELINE_CLI);
    assert_eq!(packages["0.1.28"]["cli_js_sha256"], CURRENT_CLI);
    assert_eq!(packages["0.1.29"]["cli_js_sha256"], CURRENT_CLI);
    assert_eq!(packages["0.1.30"]["cli_js_sha256"], CURRENT_CLI);
    assert_eq!(
        packages["0.1.25"]["agentclientprotocol_sdk_dependency"],
        packages["0.1.30"]["agentclientprotocol_sdk_dependency"]
    );

    let deps = ledger["runtime_dependencies"]
        .as_object()
        .expect("runtime dependencies");
    assert_eq!(deps.len(), 6, "all six deepagents points freeze");
    for (version, tarball, package_json) in DEEPAGENTS_POINTS {
        let entry = &deps[version];
        assert_eq!(
            entry["tarball_sha256"], tarball,
            "deepagents {version} tarball drifted"
        );
        assert_eq!(
            entry["package_json_sha256"], package_json,
            "deepagents {version} package.json drifted"
        );
    }
    assert_eq!(
        deps["1.13.1"]["selected_chunk_sha256"], deps["1.13.0"]["selected_chunk_sha256"],
        "1.13.1 implementation chunks must stay byte-identical to 1.13.0"
    );
    assert_eq!(
        identity["runtime_dependency"]["tarball_sha256"],
        CURRENT_DEEPAGENTS
    );
    assert_eq!(
        identity["runtime_dependency"]["selected_chunk_sha256"],
        "52d0b741683a27a2a08709e7aae290c35e0a7838d050f4647c158baac1672a93"
    );
    assert_eq!(
        identity["runtime_dependency"]["filesystem_backend_chunk_sha256"],
        "9709b83d8170cfadfd76791e1c9cb78e927e0d312786f8be967e0bc85f2ba834"
    );
}

#[test]
fn stale_registry_and_constructor_defaults_never_reach_the_claim() {
    let ledger = fixture(SURFACE, "surface-ledger");
    let registry = &ledger["acp_registry_staleness"];
    assert_eq!(registry["deepagents_entry_version"], "0.1.7");
    assert_eq!(
        registry["deepagents_entry_npx_package"],
        "deepagents-acp@0.1.7"
    );
    assert_eq!(registry["registry_agents"], 41);
    assert!(
        registry["disposition"]
            .as_str()
            .expect("registry disposition")
            .contains("discovery-only")
    );

    let identity = fixture(IDENTITY, "identity");
    let defaults = &identity["cli_constructor_defaults"];
    assert_eq!(defaults["agent_name"], "deepagents");
    assert_eq!(defaults["server_name"], "deepagents-acp");
    assert_eq!(defaults["server_version"], "0.0.1");
    assert_eq!(
        defaults["server_version_is_constructor_default_not_npm"],
        true
    );
    assert_eq!(defaults["cli_passes_server_version"], false);
    assert!(
        defaults["backend"]
            .as_str()
            .expect("backend note")
            .contains("ACPFilesystemBackend is never constructed")
    );

    let decision = &identity["identity_decision"];
    assert_eq!(decision["treat_agentinfo_version_as_package"], false);
    assert_eq!(decision["inherit_registry_0_1_7"], false);
}

#[test]
fn every_changed_selected_input_hop_is_classified() {
    let ledger = fixture(SURFACE, "surface-ledger");
    let hops = ledger["per_hop_changed_selected_inputs"]
        .as_object()
        .expect("per-hop classification");
    let expected = [
        "deepagents-acp_0.1.25..0.1.26",
        "deepagents-acp_0.1.26..0.1.27",
        "deepagents-acp_0.1.27..0.1.28",
        "deepagents-acp_0.1.28..0.1.29",
        "deepagents-acp_0.1.29..0.1.30",
        "deepagents_1.12.4..1.13.0",
        "deepagents_1.13.0..1.13.1",
        "deepagents_1.13.1..1.13.2",
        "deepagents_1.13.2..1.13.3",
        "deepagents_1.13.3..1.13.4",
    ];
    assert_eq!(
        hops.keys().map(String::as_str).collect::<Vec<_>>(),
        expected,
        "every published hop needs a classification entry"
    );
    for (hop, entry) in hops {
        assert!(
            !entry.as_object().expect(hop).is_empty(),
            "{hop} classification is empty"
        );
    }

    let invariants = &ledger["classification_invariants"];
    for name in [
        "argument_parsing",
        "initialize_shape",
        "session_new",
        "prompt_field",
        "stop_reasons",
        "permission_options",
        "authentication",
        "working_resource_authority",
        "streaming_updates",
        "deadline",
        "task_ownership",
        "cleanup",
        "session_persistence",
    ] {
        assert!(
            invariants[name].as_str().is_some(),
            "missing classification invariant {name}"
        );
    }
}

#[test]
fn the_only_cli_body_delta_is_dead_selected_route_code() {
    let ledger = fixture(SURFACE, "surface-ledger");
    let cli_delta = &ledger["cli_delta_0_1_27_to_0_1_28"];
    let sole_change = cli_delta["sole_change"].as_str().expect("sole CLI change");
    assert!(sole_change.contains("normalizeReadPagination"));
    assert!(sole_change.contains("ACPFilesystemBackend.read"));
    let unreachable = cli_delta["unreachable_on_selected_route"]
        .as_str()
        .expect("unreachability evidence");
    assert!(unreachable.contains("explicit backend"));
    assert!(unreachable.contains("fs false"));
    let changed = cli_delta["changed_files"]
        .as_array()
        .expect("changed files");
    assert!(changed.iter().any(|file| file == "dist/cli.js"));
    assert!(
        changed
            .iter()
            .all(|file| file.as_str().unwrap_or_default().starts_with("dist/"))
    );

    let stable = ledger["selected_cli_functions_byte_stable_through_0_1_27"]
        .as_array()
        .expect("stable functions");
    for required in [
        "handleInitialize",
        "handleNewSession",
        "handlePrompt",
        "handleCancel",
        "requestToolPermission",
        "createBackend",
    ] {
        assert!(
            stable
                .iter()
                .any(|entry| entry.as_str().unwrap_or_default().starts_with(required)),
            "missing stable surface {required}"
        );
    }
}

#[test]
fn the_exact_point_rebinds_without_widening_the_selected_surface() {
    let ledger = fixture(SURFACE, "surface-ledger");
    let delete_hop = &ledger["per_hop_changed_selected_inputs"]["deepagents_1.12.4..1.13.0"];
    let selected = delete_hop["selected_changes"]
        .as_array()
        .expect("1.13.0 changes");
    assert!(
        selected
            .iter()
            .any(|change| change.as_str().unwrap_or_default().contains("delete tool")),
        "the additive builtin delete tool must be classified"
    );
    let authority = delete_hop["authority_note"]
        .as_str()
        .expect("authority note");
    assert!(authority.contains("recursive deletion"));
    assert!(authority.contains("not a Swallowtail bounded-write claim"));

    let read_hop = &ledger["per_hop_changed_selected_inputs"]["deepagents_1.13.1..1.13.2"];
    let read_changes = read_hop["selected_changes"]
        .as_array()
        .expect("1.13.2 changes");
    assert!(read_changes.iter().any(|change| {
        change
            .as_str()
            .unwrap_or_default()
            .contains("normalizeReadPagination")
    }));
    let read_wire = read_hop["wire_classification"]
        .as_str()
        .expect("1.13.2 wire classification");
    assert!(read_wire.contains("pagination footer"));
    assert!(read_wire.contains("decoder-invisible"));

    let identity = fixture(IDENTITY, "identity");
    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-exact-point-rebind");
    assert_eq!(decision["move_exact_point_to_0_1_30"], true);
    assert_eq!(decision["behavior_revision_change"], "none");
    assert_eq!(decision["argv_change"], "none");
    assert_eq!(decision["wrap_npx"], false);
    assert_eq!(decision["pass_model_flag"], false);
    assert_eq!(decision["pass_workspace_flag"], false);
    assert_eq!(decision["pass_name_flag"], false);
    assert_eq!(decision["map_session_load"], false);
    assert_eq!(decision["map_slash_commands"], false);
    assert_eq!(decision["select_allow_always"], false);
    assert_eq!(decision["bind_anthropic_api_key_lease"], false);
    assert_eq!(decision["second_exact_point_retained"], false);
    assert_eq!(decision["range_inferred"], false);
    assert_eq!(decision["provider_prompt_sent"], false);
    assert_eq!(decision["live_acp_initialize"], false);
    assert_eq!(decision["downloaded_artifact_executed"], false);
    assert_eq!(
        decision["entrypoint"],
        serde_json::json!(["deepagents-acp"])
    );
}
