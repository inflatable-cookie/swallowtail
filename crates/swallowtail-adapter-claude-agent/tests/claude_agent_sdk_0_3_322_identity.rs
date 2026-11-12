//! Frozen evidence for the published Claude Agent SDK hops above `0.3.317`
//! (Research 480). The hops are identity evidence only: no pair above
//! `0.3.317`/`2.1.317` is qualified, because every newer native binary changed
//! without selected native behavior proof. These tests keep that boundary
//! mutation-sensitive: the evidence must match its pins, and the claims must
//! keep refusing every published point above the accepted pair.

use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use swallowtail_adapter_claude_agent::sdk::{
    CLAUDE_AGENT_SDK_NATIVE_VERSION, CLAUDE_AGENT_SDK_VERSION, ClaudeAgentSdkPackageNativePair,
    claude_agent_sdk_native_claim, claude_agent_sdk_package_claim,
};
use swallowtail_core::InterfaceVersion;

const IDENTITY: &[u8] = include_bytes!("fixtures/claude-agent-sdk-0.3.322/identity.json");
const DIST_INVENTORY: &[u8] =
    include_bytes!("fixtures/claude-agent-sdk-0.3.322/dist-inventory.json");
const RETAINED_317_INVENTORY: &[u8] =
    include_bytes!("fixtures/claude-agent-sdk-0.3.317/dist-inventory.json");
const IDENTITY_SHA256: &str = "d9f93134e45039da2c913c95128d35857893e4fff661e85a3dd6c4d09ef7405f";
const DIST_INVENTORY_SHA256: &str =
    "f6ac4709ad5cc0ef00b0a03356f68f17663eb610e13d45ea61bcc3918517b470";

const HOP_VERSIONS: [&str; 5] = ["0.3.317", "0.3.318", "0.3.319", "0.3.321", "0.3.322"];
const UNPUBLISHED_ABOVE_ACCEPTED: [&str; 2] = ["0.3.320", "0.3.323"];

fn json(bytes: &[u8]) -> Value {
    serde_json::from_slice(bytes).expect("frozen SDK hop evidence is valid JSON")
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn is_sha256_hex(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

#[test]
fn evidence_files_match_their_pinned_digests() {
    assert_eq!(sha256_hex(IDENTITY), IDENTITY_SHA256);
    assert_eq!(sha256_hex(DIST_INVENTORY), DIST_INVENTORY_SHA256);
}

#[test]
fn official_channel_names_latest_and_the_absent_interior_points() {
    let identity = json(IDENTITY);
    assert_eq!(identity["route"], "claude-agent.sdk");
    assert_eq!(identity["research"], "Research 480");
    let channel = &identity["official_channel"];
    assert_eq!(channel["selected"], "dist-tags.latest");
    assert_eq!(channel["latest"], "0.3.322");
    assert_eq!(channel["next"], "0.3.322");
    assert_eq!(channel["absent_from_npm"], serde_json::json!(["0.3.320"]));
    assert_eq!(channel["published_above_latest"], serde_json::json!([]));
}

#[test]
fn published_hops_bind_wrapper_manifest_and_optional_native_identity() {
    let identity = json(IDENTITY);
    let hops = identity["hops"].as_array().expect("hops are an array");
    let versions: Vec<&str> = hops
        .iter()
        .map(|hop| hop["sdk_version"].as_str().expect("version is a string"))
        .collect();
    assert_eq!(versions, HOP_VERSIONS);
    for version in UNPUBLISHED_ABOVE_ACCEPTED {
        assert!(!versions.contains(&version));
    }

    for hop in hops {
        let sdk = hop["sdk_version"].as_str().unwrap();
        let native = hop["native_version"].as_str().unwrap();
        assert!(is_sha256_hex(hop["tarball_sha256"].as_str().unwrap()));
        assert_eq!(hop["file_count"], 21);
        // The wrapper pins its native and every optional platform package to
        // the exact same published version.
        assert_eq!(hop["package_json_claudeCodeVersion"], native);
        assert_eq!(hop["optional_dependencies_exact"], serde_json::json!([sdk]));
        // The shipped manifest names the native version it carries.
        assert_eq!(hop["native_manifest"]["version"], native);
        let platforms = hop["native_manifest"]["platforms"].as_object().unwrap();
        assert_eq!(platforms.len(), 8);
        for platform in platforms.values() {
            assert!(is_sha256_hex(platform["sha256"].as_str().unwrap()));
            assert!(platform["bytes"].as_u64().unwrap() > 0);
        }
    }
}

#[test]
fn accepted_base_and_cli_bindings_stay_exact() {
    let identity = json(IDENTITY);
    let hops = identity["hops"].as_array().unwrap();
    let base = &hops[0];
    assert_eq!(base["sdk_version"], CLAUDE_AGENT_SDK_VERSION);
    assert_eq!(base["native_version"], CLAUDE_AGENT_SDK_NATIVE_VERSION);
    assert_eq!(
        base["tarball_sha256"],
        identity["accepted_base"]["tarball_sha256"]
    );
    assert_eq!(
        base["native_manifest"]["platforms"]["darwin-arm64"]["sha256"],
        "09f153b46b5df9e8b3454716d682a91901de020cbca630f61dec1a015b32b5e9"
    );
    // Darwin arm64 and Linux x64 manifest digests equal the retained CLI
    // executables for every version a CLI fixture binds.
    for hop in hops {
        if let Some(binding) = hop.get("cli_native_binding") {
            if binding.get("status").is_some() {
                assert_eq!(hop["native_version"], "2.1.318");
                continue;
            }
            assert_eq!(binding["darwin-arm64_matches_cli_fixture"], true);
            assert_eq!(binding["linux-x64_matches_cli_fixture"], true);
        }
    }
}

#[test]
fn sdk_entry_flags_and_adjacent_hop_ledger_stay_frozen() {
    let identity = json(IDENTITY);
    assert_eq!(identity["no_execution"], true);
    assert_eq!(identity["no_live_provider_work"], true);
    assert_eq!(identity["no_native_download"], true);
    assert!(
        identity["selected_entry"]
            .as_str()
            .unwrap()
            .contains("sdk.mjs")
    );
    let ledger = json(DIST_INVENTORY);
    let adjacent = ledger["adjacent"].as_object().unwrap();
    assert_eq!(
        adjacent.keys().map(String::as_str).collect::<BTreeSet<_>>(),
        BTreeSet::from([
            "0.3.317->0.3.318",
            "0.3.318->0.3.319",
            "0.3.319->0.3.321",
            "0.3.321->0.3.322",
        ])
    );
}

#[test]
fn file_ledger_reproduces_the_retained_0_3_317_per_file_hashes() {
    let ledger = json(DIST_INVENTORY);
    let per_file = ledger["per_file_sha256"].as_object().unwrap();
    let retained = json(RETAINED_317_INVENTORY);
    let retained_files = retained["file_sha256"].as_object().unwrap();
    let ledger_317 = per_file["0.3.317"].as_object().unwrap();
    assert_eq!(ledger_317.len(), 21);
    // The retained ledger spans older hops too; only paths present at 0.3.317
    // are compared, and there must be exactly the 21 files of that package.
    let retained_317: Vec<(&String, &str)> = retained_files
        .iter()
        .filter_map(|(path, hashes)| {
            hashes
                .get("0.3.317")
                .and_then(Value::as_str)
                .map(|hash| (path, hash))
        })
        .collect();
    assert_eq!(retained_317.len(), 21);
    for (path, expected) in retained_317 {
        assert_eq!(
            ledger_317.get(path).and_then(Value::as_str),
            Some(expected),
            "{path}"
        );
    }
    for version in HOP_VERSIONS {
        assert_eq!(per_file[version].as_object().unwrap().len(), 21);
    }
}

#[test]
fn adjacent_hop_deltas_keep_the_selected_wrapper_files_classified() {
    let ledger = json(DIST_INVENTORY);
    let adjacent = &ledger["adjacent"];
    // sdk.mjs changes at every adjacent hop except 0.3.321 to 0.3.322,
    // where only the version stamp and one description string changed.
    for (hop, changed_sdk) in [
        ("0.3.317->0.3.318", true),
        ("0.3.318->0.3.319", true),
        ("0.3.319->0.3.321", true),
        ("0.3.321->0.3.322", true),
    ] {
        let changed: Vec<&str> = adjacent[hop]["changed"]
            .as_array()
            .unwrap()
            .iter()
            .map(|path| path.as_str().unwrap())
            .collect();
        assert_eq!(changed.contains(&"sdk.mjs"), changed_sdk, "{hop}");
    }
}

#[test]
fn claims_keep_refusing_every_published_point_above_the_accepted_pair() {
    let package = claude_agent_sdk_package_claim();
    let native = claude_agent_sdk_native_claim();
    for version in ["0.3.318", "0.3.319", "0.3.321", "0.3.322", "0.3.323"] {
        let package_version = InterfaceVersion::new(version).unwrap();
        assert!(
            !package.assess(&package_version).is_permitted(),
            "package {version} must stay unqualified"
        );
    }
    for version in ["2.1.318", "2.1.319", "2.1.321", "2.1.322", "2.1.323"] {
        let native_version = InterfaceVersion::new(version).unwrap();
        assert!(
            !native.assess(&native_version).is_permitted(),
            "native {version} must stay unqualified"
        );
    }
    assert!(
        package
            .assess(&InterfaceVersion::new("0.3.317").unwrap())
            .is_permitted()
    );
    assert!(
        native
            .assess(&InterfaceVersion::new("2.1.317").unwrap())
            .is_permitted()
    );
}

#[test]
fn maintained_pair_map_has_no_point_above_the_accepted_pair() {
    let last = ClaudeAgentSdkPackageNativePair::MAINTAINED
        .last()
        .copied()
        .expect("the maintained pair map is not empty");
    assert_eq!(last.package_version(), "0.3.317");
    assert_eq!(last.native_version(), "2.1.317");
    for pair in ClaudeAgentSdkPackageNativePair::MAINTAINED {
        assert!(!UNPUBLISHED_ABOVE_ACCEPTED.contains(&pair.package_version()));
        assert!(!HOP_VERSIONS[1..].contains(&pair.package_version()));
    }
    assert_eq!(ClaudeAgentSdkPackageNativePair::MAINTAINED.len(), 31);
}
