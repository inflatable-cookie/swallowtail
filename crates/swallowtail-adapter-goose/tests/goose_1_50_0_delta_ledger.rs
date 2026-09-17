//! Historical identity and currentness evidence for the Goose ACP route.
//!
//! The historical corpus records the original `1.46.0` stop. Currentness
//! extends it through official `1.50.1`; at the hop `1.46.0..1.47.0`
//! provider-authentication failure stops being generic text plus `end_turn`
//! and becomes a typed JSON-RPC `auth_required` error on the selected
//! `session/new` and `session/prompt` paths. The assertions here are
//! mutation-sensitive: they fail if the tagged source trees, the per-hop
//! module ledger, the selected-surface classification, or the unchanged
//! production claim drifts.

use serde_json::Value;
use swallowtail_adapter_goose::{GOOSE_RELEASE_VERSION, goose_acp_claim, goose_release_binding};
use swallowtail_core::InterfaceVersion;

const IDENTITY: &str = include_str!("fixtures/goose-acp-1.50.0/identity.json");
const LEDGER: &str = include_str!("fixtures/goose-acp-1.50.0/tree-ledger.json");
const PROTOCOL: &str = include_str!("fixtures/goose-acp-1.50.0/protocol.json");
const CURRENTNESS: &str = include_str!("fixtures/goose-acp-1.50.1/currentness.json");

const BASELINE: &str = "1.46.0";
const HISTORICAL_OFFICIAL: &str = "1.50.0";
const OFFICIAL: &str = "1.50.1";

fn fixture(body: &str, name: &str) -> Value {
    serde_json::from_str(body).unwrap_or_else(|error| panic!("{name}: {error}"))
}

fn hop<'a>(ledger: &'a Value, name: &str) -> &'a serde_json::Map<String, Value> {
    ledger["from_hop_to_hop"][name]
        .as_object()
        .unwrap_or_else(|| panic!("hop {name}"))
}

fn string_set(value: &Value, field: &str) -> std::collections::BTreeSet<String> {
    value[field]
        .as_array()
        .unwrap_or_else(|| panic!("{field}"))
        .iter()
        .map(|entry| {
            entry
                .as_str()
                .unwrap_or_else(|| panic!("{field} entry"))
                .to_owned()
        })
        .collect()
}

#[test]
fn identity_freezes_the_baseline_and_every_published_stable_successor() {
    let identity = fixture(IDENTITY, "identity");
    assert_eq!(identity["axis"], "goose.release");
    assert_eq!(identity["route"], "goose.acp");
    assert_eq!(identity["official"]["version"], HISTORICAL_OFFICIAL);
    assert_eq!(identity["official"]["published_at"], "2026-09-08T19:32:41Z");
    assert_eq!(
        identity["official"]["github_tag_object"],
        "4cc49a8485f7960efeceba37ac5f572459c52c7c"
    );
    assert_eq!(
        identity["official"]["workspace_version"],
        HISTORICAL_OFFICIAL
    );
    assert_eq!(identity["published_stable_count"], 4);
    let compared: Vec<&str> = identity["compared"]
        .as_array()
        .expect("compared")
        .iter()
        .map(|entry| entry.as_str().expect("version"))
        .collect();
    assert_eq!(
        compared,
        vec![BASELINE, "1.47.0", "1.48.0", "1.49.0", HISTORICAL_OFFICIAL]
    );
    let points = identity["points"].as_object().expect("points");
    assert_eq!(points.len(), 5);
    assert_eq!(
        points["1.46.0"]["github_tag_object"],
        "98c11ce2ee7b9b302978aa64b1eab7d0895607c7"
    );
    assert_eq!(
        points["1.47.0"]["github_tag_object"],
        "f9c7aaccde4834810dfd13d5efa8f0d39ba28a20"
    );
    assert_eq!(
        points["1.48.0"]["github_tag_object"],
        "25021517f12cab87c94bed0874fe7d28168dc264"
    );
    assert_eq!(
        points["1.49.0"]["github_tag_object"],
        "71fc4be1ed729e26b1dc0a4466abdd03be548a53"
    );
    assert_eq!(
        points[HISTORICAL_OFFICIAL]["github_tag_object"],
        "4cc49a8485f7960efeceba37ac5f572459c52c7c"
    );
    assert_eq!(identity["host"]["present"], false);
    assert_eq!(identity["later_stable_after_official"], "none published");

    let claim = &identity["claim_at_observation"];
    assert_eq!(claim["point"], BASELINE);
    assert_eq!(claim["posture"], "QualifiedOnly");
    assert_eq!(claim["behavior_revision"], "goose.acp.stdio-v1");
    assert_eq!(claim["claim_id"], "goose.acp.release-window-1");
}

#[test]
fn identity_decision_is_the_typed_stop_and_carries_no_claim_change() {
    let identity = fixture(IDENTITY, "identity");
    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "stop");
    assert_eq!(
        decision["smallest_counterexample_hop"],
        serde_json::json!([BASELINE, "1.47.0"])
    );
    assert_eq!(decision["move_exact_point_to_1_50_0"], false);
    assert_eq!(decision["claim_change"], "none");
    assert_eq!(decision["map_session_delete"], false);
    assert_eq!(decision["provider_prompt_sent"], false);
    assert_eq!(decision["live_acp_initialize"], false);
    assert_eq!(decision["host_install_changed"], false);
    assert_eq!(decision["platform_archive_extracted"], false);
    assert_eq!(decision["downloaded_artifact_executed"], false);
}

#[test]
fn ledger_freezes_the_mapped_module_closure_and_per_hop_sets() {
    let ledger = fixture(LEDGER, "tree-ledger");
    let closure = ledger["closure_files"].as_array().expect("closure_files");
    assert_eq!(closure.len(), 26);
    assert_eq!(
        string_set(&ledger, "byte_identical_across_all_hops"),
        [
            "crates/goose-mcp/src/lib.rs",
            "crates/goose-provider-types/src/goose_mode.rs",
            "crates/goose/src/acp/common.rs",
            "crates/goose/src/acp/server/prompts.rs",
            "crates/goose/src/agents/platform_extensions/mod.rs",
            "crates/goose/src/builtin_extension.rs",
            "crates/goose/src/permission/permission_inspector.rs",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect()
    );

    let first = hop(&ledger, "1.46.0_to_1.47.0");
    assert_eq!(string_set(&Value::Object(first.clone()), "added").len(), 0);
    assert_eq!(
        string_set(&Value::Object(first.clone()), "removed").len(),
        0
    );
    let changed = string_set(&Value::Object(first.clone()), "changed");
    assert_eq!(changed.len(), 12);
    for path in [
        "crates/goose/src/acp/server.rs",
        "crates/goose/src/acp/server/dispatch.rs",
        "crates/goose/src/acp/server/new_session.rs",
        "crates/goose/src/agents/agent.rs",
        "crates/goose-provider-types/src/conversation/message.rs",
    ] {
        assert!(changed.contains(path), "{path} must change at the stop hop");
    }
    let identical = string_set(&Value::Object(first.clone()), "identical");
    assert_eq!(identical.len(), 14);
    assert!(identical.contains("crates/goose-cli/src/cli.rs"));
    assert!(identical.contains("crates/goose/src/acp/server/prompts.rs"));

    let last = hop(&ledger, "1.49.0_to_1.50.0");
    assert_eq!(
        string_set(&Value::Object(last.clone()), "removed"),
        ["documentation/docs/guides/acp-clients.md"]
            .into_iter()
            .map(str::to_owned)
            .collect()
    );
    assert_eq!(string_set(&Value::Object(last.clone()), "changed").len(), 4);
    assert_eq!(
        string_set(&Value::Object(last.clone()), "identical").len(),
        21
    );
}

#[test]
fn ledger_pins_the_stop_hop_hashes_and_the_stable_core() {
    let ledger = fixture(LEDGER, "tree-ledger");
    let hashes = &ledger["hashes"];
    assert_eq!(
        hashes["crates/goose/src/acp/server.rs"]["1.46.0"],
        "3eb611c4ae0f37a1d4affde82a4c29a5d56e9d70cb796d8cc7cc6082bc13b47f"
    );
    assert_eq!(
        hashes["crates/goose/src/acp/server.rs"]["1.47.0"],
        "bcb0a1711d2893a0bbbdcce34127c472f98f4a4c37c658e4ef4efc312efb84c6"
    );
    assert_eq!(
        hashes["crates/goose/src/acp/server.rs"]["1.50.0"],
        "5637f55188eb6b2ad050d73d70e858c866444c85e39d4ee7392878fa7c09f1a7"
    );
    assert_eq!(
        hashes["crates/goose/src/acp/server/prompts.rs"]["1.50.0"],
        "2d8c4dce604944673669ae435b2f7592041998d4141d4651d5a0396ad996e750"
    );
    assert_eq!(
        hashes["crates/goose-provider-types/src/goose_mode.rs"]["1.50.0"],
        "d5b0ae31b4882085b572ca7b595f9153f595c417f797d4ef6a50bcba1e7ebdd7"
    );
    assert_eq!(
        hashes["crates/goose/src/permission/permission_inspector.rs"]["1.50.0"],
        "c9944a12742c443bb834abead6f3a5120f6b09d666a41d80ab1491752b545390"
    );
    assert_eq!(
        hashes["documentation/docs/guides/acp-clients.md"]["1.50.0"],
        Value::Null
    );
}

#[test]
fn protocol_freezes_the_stop_boundary_and_the_stable_surface() {
    let protocol = fixture(PROTOCOL, "protocol");
    assert_eq!(
        protocol["selected_command"],
        serde_json::json!(["goose", "acp"])
    );
    assert_eq!(
        protocol["selected_methods"],
        serde_json::json!([
            "initialize",
            "session/new",
            "session/prompt",
            "session/cancel"
        ])
    );
    let stop = &protocol["stop_boundary"];
    assert_eq!(stop["hop"], serde_json::json!([BASELINE, "1.47.0"]));
    assert_eq!(stop["frozen_1_46_0_negative_cases_have_no_auth_code"], true);
    assert_eq!(
        protocol["advertised_unmapped_added_by_1_50_0"],
        serde_json::json!(["session/delete"])
    );
    assert_eq!(protocol["decoder_corpus"], "goose-acp-1.46.0");
    assert_eq!(
        protocol["decoder_corpus_change"],
        "none; frozen 1.46.0 specimens stand"
    );
    assert_eq!(protocol["allow_always_unselected"], true);
    assert_eq!(protocol["auto_mode_unselected"], true);
}

#[test]
fn production_claim_reopens_at_exact_1_50_1_with_typed_auth_revision() {
    assert_eq!(GOOSE_RELEASE_VERSION, OFFICIAL);
    let claim = goose_acp_claim();
    let baseline = InterfaceVersion::new(BASELINE).expect("baseline");
    let official = InterfaceVersion::new(OFFICIAL).expect("official");
    assert!(!claim.assess(&baseline).is_permitted());
    let assessment = claim.assess(&official);
    assert!(assessment.is_permitted());
    assert_eq!(
        assessment.behavior_revision().expect("behavior").as_str(),
        "goose.acp.stdio-v2.auth-required"
    );
    assert!(goose_release_binding(OFFICIAL).is_some());
}

#[test]
fn currentness_fixture_reproves_typed_failure_binding_through_official_patch() {
    let currentness = fixture(CURRENTNESS, "currentness");
    assert_eq!(currentness["official"]["version"], OFFICIAL);
    assert_eq!(
        currentness["official"]["github_tag_object"],
        "881f96c00d618ab6fca9b2aaa3a0abf07673cc2c"
    );
    assert_eq!(currentness["previous"]["version"], HISTORICAL_OFFICIAL);
    assert_eq!(
        currentness["compared"],
        serde_json::json!([
            BASELINE,
            "1.47.0",
            "1.48.0",
            "1.49.0",
            HISTORICAL_OFFICIAL,
            OFFICIAL
        ])
    );
    assert_eq!(
        currentness["failure_binding"]["session_new"],
        "auth_required"
    );
    assert_eq!(
        currentness["failure_binding"]["session_prompt"],
        "auth_required"
    );
    assert_eq!(currentness["failure_binding"]["mutation_sensitive"], true);
    assert_eq!(
        currentness["patch_hop"]["classification"],
        "unmapped; mcpServers is always empty on the selected route"
    );
    assert_eq!(
        currentness["patch_hop"]["changed_mapped_files"],
        serde_json::json!(["Cargo.toml", "crates/goose/src/agents/agent.rs"])
    );
    assert_eq!(
        currentness["patch_hop"]["source_sha256"]["crates/goose/src/acp/server.rs"],
        "5637f55188eb6b2ad050d73d70e858c866444c85e39d4ee7392878fa7c09f1a7"
    );
    assert_eq!(
        currentness["patch_hop"]["source_sha256"]["crates/goose/src/agents/agent.rs"][OFFICIAL],
        "9171a3c9a980d06b2c1a035b0b0b1a39317157f645d06d1a74fbcef6baf13fa9"
    );
    assert_eq!(
        currentness["claim"]["behavior_revision"],
        "goose.acp.stdio-v2.auth-required"
    );
}
