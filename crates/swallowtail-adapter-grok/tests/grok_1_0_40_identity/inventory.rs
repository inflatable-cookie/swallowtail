//! Frozen shipped-file inventory and per-hop deltas for Grok Build 1.0.40.

use super::support::{DIST_INVENTORY, IDENTITY, assert_exact_string_set, json};
use std::collections::BTreeMap;

const PRESENT_IN_EVERY_HOP: &[(&str, &[&str])] = &[
    (
        "wrapper",
        &[
            "README.md",
            "bin/grok",
            "bin/grok-bootstrap.js",
            "bin/postinstall.js",
            "package.json",
        ],
    ),
    (
        "platform",
        &[
            "README.md",
            "THIRD_PARTY_NOTICES.md",
            "bin/grok.br",
            "package.json",
        ],
    ),
];

const IDENTICAL_THROUGH_ALL_HOPS: &[(&str, &[&str])] = &[
    (
        "wrapper",
        &[
            "README.md",
            "bin/grok",
            "bin/grok-bootstrap.js",
            "bin/postinstall.js",
        ],
    ),
    ("platform", &["README.md", "THIRD_PARTY_NOTICES.md"]),
];

fn package_map(entry: &serde_json::Value, package: &str) -> BTreeMap<String, String> {
    entry[package]
        .as_object()
        .unwrap()
        .iter()
        .map(|(k, v)| (k.clone(), v.as_str().unwrap().to_owned()))
        .collect()
}

fn delta(
    previous: &BTreeMap<String, String>,
    current: &BTreeMap<String, String>,
) -> serde_json::Value {
    let mut added: Vec<&String> = current
        .keys()
        .filter(|k| !previous.contains_key(*k))
        .collect();
    let mut removed: Vec<&String> = previous
        .keys()
        .filter(|k| !current.contains_key(*k))
        .collect();
    let mut changed: Vec<&String> = current
        .iter()
        .filter(|(k, v)| previous.get(*k).is_some_and(|p| p != *v))
        .map(|(k, _)| k)
        .collect();
    let mut identical: Vec<&String> = current
        .iter()
        .filter(|(k, v)| previous.get(*k).is_some_and(|p| p == *v))
        .map(|(k, _)| k)
        .collect();
    added.sort();
    removed.sort();
    changed.sort();
    identical.sort();
    serde_json::json!({
        "added": added,
        "removed": removed,
        "changed": changed,
        "identical": identical,
    })
}

#[test]
fn every_hop_ships_the_frozen_package_files() {
    let inventory = json(DIST_INVENTORY);
    let present: BTreeMap<&str, &[&str]> = PRESENT_IN_EVERY_HOP.iter().copied().collect();
    let identical: BTreeMap<&str, &[&str]> = IDENTICAL_THROUGH_ALL_HOPS.iter().copied().collect();
    assert_exact_string_set(
        &inventory["present_in_every_hop"]["wrapper"],
        present["wrapper"],
    );
    assert_exact_string_set(
        &inventory["present_in_every_hop"]["platform"],
        present["platform"],
    );
    assert_exact_string_set(
        &inventory["identical_through_all_hops"]["wrapper"],
        identical["wrapper"],
    );
    assert_exact_string_set(
        &inventory["identical_through_all_hops"]["platform"],
        identical["platform"],
    );
    for (version, entry) in inventory["per_hop_files"].as_object().unwrap() {
        assert_eq!(
            package_map(entry, "wrapper").len(),
            inventory["package_file_counts"][version]["wrapper"]
                .as_u64()
                .unwrap() as usize
        );
        assert_eq!(
            package_map(entry, "platform").len(),
            inventory["package_file_counts"][version]["platform"]
                .as_u64()
                .unwrap() as usize
        );
    }
}

#[test]
fn recorded_hop_deltas_match_the_frozen_digests() {
    let inventory = json(DIST_INVENTORY);
    let compared: Vec<&str> = inventory["compared"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
    for package in ["wrapper", "platform"] {
        let baseline = package_map(&inventory["per_hop_files"][compared[0]], package);
        for version in &compared {
            let current = package_map(&inventory["per_hop_files"][*version], package);
            assert_eq!(
                delta(&baseline, &current),
                inventory["per_hop_delta_from_1_0_30"][*version][package],
                "{package} {version} cumulative delta"
            );
        }
        for index in 1..compared.len() {
            let previous = package_map(&inventory["per_hop_files"][compared[index - 1]], package);
            let current = package_map(&inventory["per_hop_files"][compared[index]], package);
            assert_eq!(
                delta(&previous, &current),
                inventory["hop_to_hop_delta"][compared[index]][package],
                "{package} {compared_index}",
                compared_index = compared[index]
            );
        }
    }
}

#[test]
fn the_native_payload_is_the_only_changed_platform_file() {
    let inventory = json(DIST_INVENTORY);
    for (version, entry) in inventory["per_hop_delta_from_1_0_30"].as_object().unwrap() {
        let platform: Vec<&str> = entry["platform"]["changed"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap())
            .collect();
        let wrapper: Vec<&str> = entry["wrapper"]["changed"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap())
            .collect();
        let added: Vec<&str> = entry["wrapper"]["added"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap())
            .collect();
        if version == "1.0.30" {
            assert!(platform.is_empty() && wrapper.is_empty() && added.is_empty());
            continue;
        }
        assert_eq!(
            platform,
            vec!["bin/grok.br", "package.json"],
            "{version} platform changes are the native payload and its package metadata"
        );
        assert_eq!(
            wrapper,
            vec!["package.json"],
            "{version} launcher metadata only"
        );
        assert!(added.is_empty(), "{version}");
    }
}

#[test]
fn identical_files_are_byte_identical_across_every_hop_and_native_payloads_are_distinct() {
    let inventory = json(DIST_INVENTORY);
    let compared: Vec<&str> = inventory["compared"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
    for package in ["wrapper", "platform"] {
        let names: Vec<&str> = inventory["identical_through_all_hops"][package]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap())
            .collect();
        for name in names {
            let digests = inventory["digests"][package][name].as_object().unwrap();
            assert_eq!(digests.len(), compared.len(), "{package} {name}");
            let unique: std::collections::BTreeSet<&str> =
                digests.values().map(|v| v.as_str().unwrap()).collect();
            assert_eq!(
                unique.len(),
                1,
                "{package} {name} is byte-identical everywhere"
            );
        }
    }
    let native = inventory["digests"]["platform"]["bin/grok.br"]
        .as_object()
        .unwrap();
    let unique: std::collections::BTreeSet<&str> =
        native.values().map(|v| v.as_str().unwrap()).collect();
    assert_eq!(
        unique.len(),
        compared.len(),
        "each hop ships a distinct native payload"
    );
    let identity = json(IDENTITY);
    for entry in identity["hops"].as_array().unwrap() {
        let version = entry["version"].as_str().unwrap();
        assert_eq!(
            native[version].as_str().unwrap(),
            entry["platform"]["brotli_sha256"].as_str().unwrap(),
            "{version} shipped payload equals the identity artifact"
        );
        assert_ne!(
            native[version].as_str().unwrap(),
            entry["platform"]["executable_sha256"].as_str().unwrap(),
            "{version} brotli payload is not the decompressed executable"
        );
    }
    let launcher = inventory["digests"]["wrapper"]["bin/grok"]
        .as_object()
        .unwrap();
    assert_eq!(launcher.len(), compared.len());
    let unique: std::collections::BTreeSet<&str> =
        launcher.values().map(|v| v.as_str().unwrap()).collect();
    assert_eq!(
        unique.len(),
        1,
        "the launcher stays byte-identical after the 1.0.14 split"
    );
}
