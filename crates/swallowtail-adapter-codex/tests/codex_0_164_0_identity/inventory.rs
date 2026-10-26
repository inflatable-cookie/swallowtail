use super::support::{
    DIST_INVENTORY, HOPS, SOURCE_INVENTORY, SOURCE_STATUS, fixture_dir, json, table_rows,
};
use std::collections::BTreeMap;
use std::fs;

#[test]
fn native_artifact_inventories_match_declared_file_counts() {
    let identity = json(super::support::IDENTITY);
    let inventory_dir = fixture_dir().join("artifact-inventory");
    let hops = identity["hops"].as_array().expect("hops");
    for hop in hops {
        let version = hop["version"].as_str().expect("version");
        for (native, key) in [("darwin-arm64", "darwinArm64"), ("linux-x64", "linuxX64")] {
            let expected = hop[key]["files"].as_u64().expect("files") as usize;
            let listing =
                fs::read_to_string(inventory_dir.join(format!("{version}-{native}.sha256")))
                    .unwrap_or_else(|error| panic!("{version}-{native}: {error}"));
            let lines = listing
                .lines()
                .filter(|line| !line.trim().is_empty())
                .collect::<Vec<_>>();
            assert_eq!(lines.len(), expected, "{version} {native}");
            for line in lines {
                let digest = line.split_whitespace().next().expect("digest");
                assert_eq!(digest.len(), 64, "{line}");
            }
        }
        let wrapper = fs::read_to_string(inventory_dir.join(format!("{version}-wrapper.sha256")))
            .unwrap_or_else(|error| panic!("{version}-wrapper: {error}"));
        assert_eq!(
            wrapper
                .lines()
                .filter(|line| !line.trim().is_empty())
                .count(),
            3,
            "{version} wrapper"
        );
    }
    assert_eq!(
        json(DIST_INVENTORY)["packageFileCounts"]["0.163.0"]["darwin-arm64"],
        43
    );
    assert_eq!(
        json(DIST_INVENTORY)["packageFileCounts"]["0.163.0"]["linux-x64"],
        45
    );
    assert_eq!(
        json(DIST_INVENTORY)["wrapperCodexJsSha256"],
        "61b0194f3bb6534439c8d26a3ed57d0805f84b884588b761795323eeb92fcf70"
    );
}

#[test]
fn source_inventories_are_complete_and_mutation_sensitive() {
    let source = json(SOURCE_INVENTORY);
    let expected = [
        (
            "0.161.0",
            "c2b373a72c6635fa3de9484e812b67eab9fa52b1027ca3c61fa780e3df3a38f9",
            8899,
        ),
        (
            "0.162.0",
            "558767aea1737612d65455a2778ee0e16e47e4f8b5f8f9fedb5690d6c44f89b1",
            9017,
        ),
        (
            "0.162.1",
            "f94abf708cc1fd70e3bc877f5fce1c94d6611f8d004ae6ace514cdf3a5fd1e63",
            9018,
        ),
        (
            "0.163.0",
            "67df9d79168efe090f6713f994f57b6a7dab53324b6d22666ee8d4adfd959b45",
            9176,
        ),
        (
            "0.164.0",
            "02b1e9b3c637008eddbd903b6a9ee9441705fef44732cf5a80ed5c63622b3030",
            9438,
        ),
    ];
    for (hop, digest, count) in expected {
        assert_eq!(source["inventoryDigests"][hop]["digest"], digest);
        assert_eq!(source["inventoryDigests"][hop]["fileCount"], count);
    }

    let mut counts: BTreeMap<(&str, &str), usize> = BTreeMap::new();
    for row in table_rows(SOURCE_STATUS) {
        assert_eq!(row.len(), 3, "{row:?}");
        let from_to = row[0];
        let status = row[1];
        assert!(matches!(status, "A" | "D" | "M"), "{status}");
        *counts.entry((from_to, status)).or_insert(0) += 1;
    }
    let expected_hops = [
        ("0.161.0-0.162.0", 188, 70, 1400),
        ("0.162.0-0.162.1", 1, 0, 9),
        ("0.162.1-0.163.0", 193, 35, 1080),
        ("0.163.0-0.164.0", 299, 37, 1591),
    ];
    for (label, added, removed, changed) in expected_hops {
        assert_eq!(
            counts.get(&(label, "A")).copied().unwrap_or(0),
            added,
            "{label} A"
        );
        assert_eq!(
            counts.get(&(label, "D")).copied().unwrap_or(0),
            removed,
            "{label} D"
        );
        assert_eq!(
            counts.get(&(label, "M")).copied().unwrap_or(0),
            changed,
            "{label} M"
        );
    }
    for (older, newer) in HOPS.iter().zip(HOPS.iter().skip(1)) {
        let key = format!("{older}->{newer}");
        let json_counts = &source["hopCounts"][&key];
        let label = format!("{older}-{newer}");
        assert_eq!(
            json_counts["added"],
            counts.get(&(label.as_str(), "A")).copied().unwrap_or(0)
        );
        assert_eq!(
            json_counts["removed"],
            counts.get(&(label.as_str(), "D")).copied().unwrap_or(0)
        );
        assert_eq!(
            json_counts["changed"],
            counts.get(&(label.as_str(), "M")).copied().unwrap_or(0)
        );
    }
    assert!(SOURCE_STATUS.contains(
        "0.161.0-0.162.0\tM\tcodex-rs/app-server/src/request_processors/thread_processor.rs"
    ));
    assert!(SOURCE_STATUS.contains(
        "0.162.1-0.163.0\tM\tcodex-rs/app-server/src/request_processors/thread_processor.rs"
    ));
    assert!(SOURCE_STATUS.contains(
        "0.163.0-0.164.0\tM\tcodex-rs/app-server/src/request_processors/thread_processor.rs"
    ));
}

#[test]
fn dist_inventory_pins_native_cli_and_tarball_digests() {
    let dist = json(DIST_INVENTORY);
    assert_eq!(
        dist["cliSha256"]["0.164.0"]["darwin-arm64"],
        "d8990e3ee7acfbc26f91265b12a1a83f0fee40d0e0f8f1f40a37d4ec21f499cf"
    );
    assert_eq!(
        dist["cliSha256"]["0.164.0"]["linux-x64"],
        "b93f6811d3f653f0fb83be4050d857fb9abbb111dfa2ce160563327daff7f306"
    );
    assert_eq!(
        dist["tarballSha256"]["0.161.0"]["darwin-arm64"],
        "21c99b848b25ed91b92afadabe3751cc33878d2684f3e2d01cc1782e6d0abc7a"
    );
    assert_eq!(dist["downloadedBinariesExecuted"], false);
    let hop = &dist["artifactHops"]["0.162.1->0.163.0"]["darwin-arm64"];
    assert_eq!(
        hop["removed"],
        serde_json::json!(["vendor/aarch64-apple-darwin/codex-resources/zsh/bin/zsh"])
    );
}
