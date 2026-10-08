use serde_json::Value;
use sha2::{Digest, Sha256};

const IDENTITY: &str = include_str!("fixtures/cursor-agent-2026.10.01/identity.json");
const PROTOCOL: &str = include_str!("fixtures/cursor-agent-2026.10.01/protocol.json");
const DIST_INVENTORY: &str = include_str!("fixtures/cursor-agent-2026.10.01/dist-inventory.json");

fn json(source: &str) -> Value {
    serde_json::from_str(source).expect("Cursor Agent 2026.10.01 fixture is valid JSON")
}

fn sha256_hex(source: &str) -> String {
    Sha256::digest(source.as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<Vec<_>>()
        .join("")
}

#[test]
fn identity_freezes_official_hops_and_the_catalogue_only_claim() {
    assert_eq!(
        sha256_hex(IDENTITY),
        "55a51a3d3192285fe07ff115150a2008aba94f66fe2f705e398b353a902af231"
    );
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], "cursor-agent.release-date");
    assert_eq!(identity["qualified_route"], "cursor-agent.catalogue");
    assert_eq!(
        identity["published_stables_after_ceiling"],
        serde_json::json!([
            "2026.09.26-dd393fe",
            "2026.09.28-64d2043",
            "2026.10.01-14929f9"
        ])
    );
    assert_eq!(identity["registry"]["version"], "2026.10.01");
    assert_eq!(identity["registry"]["latest_stable"], "2026.10.01-14929f9");
    assert_eq!(
        identity["registry"]["observed_repository_main"],
        "a1c535174151467335d39083cc2167b210b7e3c1"
    );
    assert_eq!(
        identity["previous_ceiling"]["version"],
        "2026.09.18-9a7762b"
    );
    assert_eq!(identity["host"]["version"], "2026.09.18-9a7762b");

    let hops = identity["hops"].as_array().expect("hops are an array");
    assert_eq!(hops.len(), 3);
    for (hop, version) in hops.iter().zip([
        "2026.09.26-dd393fe",
        "2026.09.28-64d2043",
        "2026.10.01-14929f9",
    ]) {
        assert_eq!(hop["version"], version);
        for field in [
            "darwin_arm64_archive_sha256",
            "linux_x64_archive_sha256",
            "darwin_arm64_runtime_index_sha256",
            "linux_x64_runtime_index_sha256",
            "darwin_arm64_node_runtime_sha256",
            "linux_x64_node_runtime_sha256",
        ] {
            let digest = hop[field].as_str().expect("artifact digest is text");
            assert_eq!(digest.len(), 64, "{version} {field}");
            assert!(digest.bytes().all(|byte| byte.is_ascii_hexdigit()));
        }
    }

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(
        decision["reuse_behavior"],
        "cursor-agent.catalogue.calendar-release-v1"
    );
    assert_eq!(decision["keep_baseline"], "2026.07.01-41b2de7");
    assert_eq!(decision["raise_latest_qualified_to"], "2026.10.01-14929f9");
    assert_eq!(
        decision["preserve_acp_headless_claims_at"],
        "2026.09.18-9a7762b"
    );
    assert_eq!(decision["infer_calendar_gap"], false);
    assert_eq!(decision["new_behavior_revision"], false);
    assert_eq!(decision["new_public_operation"], false);
    assert_eq!(decision["consumer_visible_narrowing"], false);
    assert_eq!(
        decision["independently_unqualified_older_published"],
        serde_json::json!(["2026.08.25-3e8eec8", "2026.09.08-6caf4ff"])
    );
    assert_eq!(decision["downloaded_artifacts_executed"], false);
    assert_eq!(decision["authenticated_catalogue_called"], false);
}

#[test]
fn selected_catalogue_path_keeps_its_existing_adapter_boundary() {
    assert_eq!(
        sha256_hex(PROTOCOL),
        "ab2f1e98bc9556bff9fd2a04e21672c3a410d1dcf2a5a802eb77e3994b96e554"
    );
    let protocol = json(PROTOCOL);
    assert_eq!(protocol["selected_route"], "cursor-agent.catalogue");
    assert_eq!(protocol["selected_cli_command"], "models");
    assert_eq!(protocol["selected_cli_command_present_at_every_hop"], true);
    assert_eq!(
        protocol["selected_catalogue_fields"]["model_id"],
        "displayModelId || modelId"
    );
    assert_eq!(
        protocol["selected_catalogue_fields"]["display_name"],
        "displayName || displayNameShort"
    );
    assert_eq!(
        protocol["selected_request_path"]["account_options"],
        serde_json::json!(["endpoint", "apiKey", "authToken", "insecure"])
    );
    assert_eq!(
        protocol["compatible_surface_review"]["mapped_fields_unchanged"],
        true
    );
    assert_eq!(
        protocol["compatible_surface_review"]["no_new_operation_or_authority"],
        true
    );
    assert_eq!(
        protocol["sibling_axes"]["acp_qualified_ceiling"],
        "2026.09.18-9a7762b"
    );
    assert_eq!(
        protocol["sibling_axes"]["headless_qualified_ceiling"],
        "2026.09.18-9a7762b"
    );
    assert_eq!(protocol["sibling_axes"]["evidence_transferred"], false);
    assert_eq!(protocol["safety"]["archive_executed"], false);
    assert_eq!(protocol["safety"]["credentials_used"], false);
}

#[test]
fn inventory_freezes_every_file_and_each_hop_classification() {
    assert_eq!(
        sha256_hex(DIST_INVENTORY),
        "84473ea98c85ed1474b9d9ea14921030a48821ac9ae99d8343d6dd259994d4fc"
    );
    let inventory = json(DIST_INVENTORY);
    assert_eq!(
        inventory["versions"],
        serde_json::json!([
            "2026.09.18-9a7762b",
            "2026.09.26-dd393fe",
            "2026.09.28-64d2043",
            "2026.10.01-14929f9"
        ])
    );
    assert_eq!(
        inventory["platforms"],
        serde_json::json!(["darwin-arm64", "linux-x64"])
    );

    for (platform, counts) in [
        ("darwin-arm64", [445, 446, 447, 447]),
        ("linux-x64", [453, 454, 455, 455]),
    ] {
        for (version, count) in inventory["versions"]
            .as_array()
            .expect("versions are an array")
            .iter()
            .zip(counts)
        {
            let version = version.as_str().expect("version is text");
            let tree = &inventory["archive_trees"][version][platform];
            assert_eq!(tree["file_count"], count, "{platform} {version}");
            assert_eq!(
                tree["files"].as_array().expect("files are an array").len(),
                count
            );
        }
    }

    let expected = [
        (
            "darwin-arm64",
            "2026.09.18-9a7762b_to_2026.09.26-dd393fe",
            [63, 62, 54, 329],
        ),
        (
            "darwin-arm64",
            "2026.09.26-dd393fe_to_2026.09.28-64d2043",
            [65, 64, 82, 300],
        ),
        (
            "darwin-arm64",
            "2026.09.28-64d2043_to_2026.10.01-14929f9",
            [60, 60, 80, 307],
        ),
        (
            "linux-x64",
            "2026.09.18-9a7762b_to_2026.09.26-dd393fe",
            [5, 4, 44, 405],
        ),
        (
            "linux-x64",
            "2026.09.26-dd393fe_to_2026.09.28-64d2043",
            [64, 63, 23, 368],
        ),
        (
            "linux-x64",
            "2026.09.28-64d2043_to_2026.10.01-14929f9",
            [0, 0, 20, 435],
        ),
    ];
    for (platform, hop, counts) in expected {
        for (category, count) in ["added", "removed", "changed", "identical"]
            .into_iter()
            .zip(counts)
        {
            let paths = inventory["hop_deltas"][platform][hop][category]["paths"]
                .as_array()
                .expect("classified paths are an array");
            assert_eq!(paths.len(), count, "{platform} {hop} {category}");
        }
    }
}
