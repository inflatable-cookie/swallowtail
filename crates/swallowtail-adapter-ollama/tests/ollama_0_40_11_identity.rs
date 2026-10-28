use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use swallowtail_adapter_ollama::{
    OLLAMA_BASELINE_VERSION, OLLAMA_LATEST_QUALIFIED_VERSION, ollama_runtime_claim,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const IDENTITY: &str = include_str!("fixtures/ollama-0.40.11/identity.json");
const PROTOCOL: &str = include_str!("fixtures/ollama-0.40.11/protocol.json");
const INVENTORY: &str = include_str!("fixtures/ollama-0.40.11/dist-inventory.json");
const HISTORICAL: &str = include_str!("fixtures/ollama-0.40.1/identity.json");

const MAINTAINED: &[&str] = &[
    "0.40.0", "0.40.1", "0.40.2", "0.40.3", "0.40.4", "0.40.5", "0.40.6", "0.40.7", "0.40.8",
    "0.40.9", "0.40.10", "0.40.11",
];

#[test]
fn identity_freezes_official_hops_from_the_0401_ceiling_through_04011() {
    let identity: Value = serde_json::from_str(IDENTITY).expect("identity corpus parses");
    let protocol: Value = serde_json::from_str(PROTOCOL).expect("protocol corpus parses");
    let inventory: Value = serde_json::from_str(INVENTORY).expect("tree inventory parses");
    let historical: Value =
        serde_json::from_str(HISTORICAL).expect("historical qualification parses");

    assert_eq!(identity["axis"], "ollama.runtime");
    assert_eq!(identity["package"], "swallowtail-adapter-ollama");
    assert_eq!(identity["research"], 448);
    assert_eq!(identity["previous_ceiling"], "0.40.1");
    assert_eq!(identity["host_observation"]["client_on_path"], false);
    assert_eq!(identity["host_observation"]["host_not_mutated"], true);
    assert_eq!(identity["selected_channel"]["latest_version"], "0.40.11");
    assert_eq!(identity["selected_channel"]["draft"], false);
    assert_eq!(identity["selected_channel"]["prerelease"], false);
    assert_eq!(identity["selected_channel"]["release_id"], 422414238);
    assert_eq!(
        identity["selected_channel"]["tag_commit"],
        "675150d6ad2747df1e218ef59de9194bc4a7e538"
    );
    assert_eq!(
        identity["selected_channel"]["tree"],
        "1dd0e013eeb9ff6acc9deaa37f965bb7a98e62fc"
    );
    assert_eq!(
        identity["selected_channel"]["source_archive_sha256"],
        "9733d6230d8c8a75ccdf368c53132d90ee49009ce677776d897e96b50cc1c654"
    );
    assert_eq!(
        identity["selected_channel"]["pre_push_reprobe_at"],
        "2026-10-26T10:35:48Z"
    );
    assert_eq!(
        identity["selected_channel"]["pre_push_reprobe_version"],
        "0.40.11"
    );
    assert_eq!(
        identity["selected_channel"]["pre_push_reprobe_release_id"],
        422414238
    );
    assert_eq!(
        identity["selected_channel"]["pre_push_reprobe_tag_commit"],
        "675150d6ad2747df1e218ef59de9194bc4a7e538"
    );
    assert_eq!(
        historical["decision"]["latest_qualified_after_change"],
        "0.40.1"
    );
    assert_eq!(
        identity["current_claim_at_observation"]["latest_qualified"],
        "0.40.1"
    );
    assert_eq!(
        identity["decision"]["shape"],
        "compatible-extension-through-0.40.11"
    );
    assert_eq!(identity["decision"]["new_public_operation"], false);
    assert_eq!(identity["decision"]["provider_prompt_sent"], false);
    assert_eq!(identity["decision"]["attached_server_started"], false);

    let published = identity["published_stables_after_previous_ceiling"]
        .as_array()
        .unwrap();
    assert_eq!(
        published
            .iter()
            .map(|release| release["version"].as_str().unwrap())
            .collect::<Vec<_>>(),
        [
            "0.40.2", "0.40.3", "0.40.4", "0.40.5", "0.40.6", "0.40.7", "0.40.8", "0.40.9",
            "0.40.10", "0.40.11",
        ]
    );
    let promoted = published
        .iter()
        .find(|release| release["version"] == "0.40.2")
        .expect("0.40.2 release");
    assert_eq!(promoted["release_id"], 407063639);
    assert_eq!(promoted["prerelease"], false);
    assert_eq!(
        promoted["commit"],
        "b061384d90ff455462bc32745dd0de479de717a3"
    );

    let compared = ["0.40.1"]
        .into_iter()
        .chain(
            published
                .iter()
                .map(|release| release["version"].as_str().unwrap()),
        )
        .collect::<Vec<_>>();
    assert_eq!(
        inventory["points"]
            .as_array()
            .unwrap()
            .iter()
            .map(|point| point["version"].as_str().unwrap())
            .collect::<Vec<_>>(),
        compared
    );
    for release in std::iter::once(&serde_json::json!({
        "version": "0.40.1",
        "commit": "cf2a313a298066d572c36812e5ad30a21c0db13b",
        "tree": "6ae219abaf2a32dc26c8dc14d71357b92532dec2",
        "source_archive_sha256": "675cfca761a964d3b40d1331c006fd8a630bdb0b5a9b605ace3494e7d47c3e88"
    }))
    .chain(published.iter())
    {
        let version = release["version"].as_str().unwrap();
        let point = inventory["points"]
            .as_array()
            .unwrap()
            .iter()
            .find(|point| point["version"] == version)
            .unwrap_or_else(|| panic!("tree inventory omits {version}"));
        assert_eq!(release["commit"], point["commit"], "commit at {version}");
        assert_eq!(release["tree"], point["tree"], "tree at {version}");
        assert_eq!(
            release["source_archive_sha256"], point["source_archive_sha256"],
            "archive digest at {version}"
        );
        assert_eq!(point["draft"], false);
        assert_eq!(point["prerelease"], false);
        let entries = point["entries"].as_array().expect("entries");
        assert_eq!(
            entries.len(),
            point["tree_entry_count"].as_u64().unwrap() as usize
        );
        assert_eq!(
            entries.iter().filter(|row| row[2] == "blob").count(),
            point["file_count"].as_u64().unwrap() as usize
        );
    }

    let mut ledger = String::new();
    for hop in inventory["version_ordered_hops"].as_array().unwrap() {
        let from = hop["from"].as_str().unwrap();
        let to = hop["to"].as_str().unwrap();
        let before = blob_map(&inventory, from);
        let after = blob_map(&inventory, to);
        let derived_added = after
            .keys()
            .filter(|path| !before.contains_key(*path))
            .cloned()
            .collect::<BTreeSet<_>>();
        let derived_removed = before
            .keys()
            .filter(|path| !after.contains_key(*path))
            .cloned()
            .collect::<BTreeSet<_>>();
        let derived_changed = before
            .iter()
            .filter(|(path, blob)| {
                after
                    .get(*path)
                    .is_some_and(|after_blob| after_blob != *blob)
            })
            .map(|(path, _)| path.clone())
            .collect::<BTreeSet<_>>();
        let derived_unchanged = before
            .iter()
            .filter(|(path, blob)| after.get(*path) == Some(*blob))
            .map(|(path, _)| path.clone())
            .collect::<BTreeSet<_>>();
        assert_eq!(
            hop_paths(&hop["added"]),
            derived_added,
            "added {from}->{to}"
        );
        assert_eq!(
            hop_paths(&hop["removed"]),
            derived_removed,
            "removed {from}->{to}"
        );
        assert_eq!(
            hop_paths(&hop["changed"]),
            derived_changed,
            "changed {from}->{to}"
        );
        assert_eq!(hop["added_file_count"], derived_added.len());
        assert_eq!(hop["removed_file_count"], derived_removed.len());
        assert_eq!(hop["changed_file_count"], derived_changed.len());
        assert_eq!(hop["unchanged_file_count"], derived_unchanged.len());
        assert_eq!(
            hop["unchanged_path_list_sha256"],
            path_list_sha256(&derived_unchanged)
        );
        for (section, rows) in [
            ("added", &hop["added"]),
            ("removed", &hop["removed"]),
            ("changed", &hop["changed"]),
        ] {
            for row in rows.as_array().unwrap() {
                let path = row["path"].as_str().unwrap();
                let class = row["classification"].as_str().expect("classification");
                assert!(!class.is_empty(), "{path} needs a classification");
                assert!(
                    row["reason"]
                        .as_str()
                        .is_some_and(|reason| !reason.is_empty()),
                    "{path} needs a reason"
                );
                if section == "changed" {
                    ledger.push_str(&format!(
                        "{from}\t{to}\tchanged\t{path}\t{class}\t{}\t{}\n",
                        row["before_blob"].as_str().unwrap(),
                        row["after_blob"].as_str().unwrap()
                    ));
                } else {
                    ledger.push_str(&format!(
                        "{from}\t{to}\t{section}\t{path}\t{class}\t{}\n",
                        row["blob"].as_str().unwrap()
                    ));
                }
            }
        }
    }
    assert_eq!(
        sha256_hex(ledger.as_bytes()),
        "11e5948c6e040db9d3c75354874b4f782e6c8e32c0bcd5441677c01069c74eff"
    );
    assert_eq!(
        protocol["classification_ledger_sha256"],
        "11e5948c6e040db9d3c75354874b4f782e6c8e32c0bcd5441677c01069c74eff"
    );
    assert_eq!(protocol["migration_source_byte_identical"], true);
    assert_eq!(protocol["tags_preferred_child_from"], "0.40.2");
    assert_eq!(protocol["migration_call_commented_from"], "0.40.3");
    assert_eq!(protocol["new_public_swallowtail_operation"], false);
    assert_eq!(
        protocol["byte_identical_selected_files"]["compatmigrate/migrate.go"],
        "d6a5ce939218e4f6092b30322cb39b1fc9dad35e"
    );
    assert_eq!(
        protocol["byte_identical_selected_files"]["manifest/paths.go"],
        "cf3bfa2569bf1a3fbc69ef12455566e259515a2d"
    );
    for version in &compared {
        let point = fingerprint(&inventory, version);
        assert_eq!(
            point["compatmigrate/migrate.go"]["blob"],
            "d6a5ce939218e4f6092b30322cb39b1fc9dad35e"
        );
        assert_eq!(
            point["manifest/paths.go"]["blob"],
            "cf3bfa2569bf1a3fbc69ef12455566e259515a2d"
        );
    }
    assert_eq!(
        fingerprint(&inventory, "0.40.1")["server/model_list.go"]["blob"],
        "9976b4b6688c8a6261f154746cc5abb34fc61d48"
    );
    assert_eq!(
        fingerprint(&inventory, "0.40.1")["api/types.go"]["blob"],
        "e19de536fdaf350af615f24b52a2af9fedf44e54"
    );
    assert_eq!(
        fingerprint(&inventory, "0.40.1")["server/routes.go"]["blob"],
        "addc6c189b2455554a00b1a08f906a04945b4551"
    );
    for version in &compared[1..] {
        assert_eq!(
            fingerprint(&inventory, version)["server/model_list.go"]["blob"],
            "6be2862816f5ba2d6101812a1a03ee98fc079b8d",
            "model_list at {version}"
        );
    }
    assert_eq!(
        fingerprint(&inventory, "0.40.11")["api/types.go"]["blob"],
        "1aa71342a372fc2a0902cc55f43afe91c3a55d06"
    );
    assert_eq!(
        fingerprint(&inventory, "0.40.11")["server/routes.go"]["blob"],
        "fdf7a18f954ab7c249bb0e95a859b399105c7289"
    );
}

#[test]
fn claim_extends_manifest_list_runner_through_04011() {
    assert_eq!(OLLAMA_BASELINE_VERSION, "0.14.0");
    assert_eq!(OLLAMA_LATEST_QUALIFIED_VERSION, "0.40.11");
    let claim = ollama_runtime_claim();
    assert_eq!(claim.id().as_str(), "ollama.native-runtime-window-2");
    for version in ["0.14.0", "0.34.4", "0.35.0", "0.35.1"] {
        assert!(matches!(
            claim.assess(&InterfaceVersion::new(version).unwrap()),
            InterfaceCompatibilityAssessment::Qualified(point)
                if point.support_status() == InterfaceSupportStatus::Deprecated
                    && point.behavior_revision().as_str() == "ollama.native-text-v1"
        ));
    }
    for version in MAINTAINED {
        assert!(
            matches!(
                claim.assess(&InterfaceVersion::new(*version).unwrap()),
                InterfaceCompatibilityAssessment::Qualified(point)
                    if point.support_status() == InterfaceSupportStatus::Maintained
                        && point.behavior_revision().as_str()
                            == "ollama.native-text-v1.manifest-list-runner"
            ),
            "{version} must stay on the manifest-list runner revision"
        );
    }
    for version in [
        "0.32.2",
        "0.32.10",
        "0.34.5",
        "0.35.2",
        "0.39.0",
        "0.40.2-rc.0",
    ] {
        assert!(!claim.permits(&InterfaceVersion::new(version).unwrap()));
    }
    for version in ["0.40.12", "0.41.0"] {
        assert!(
            matches!(
                claim.assess(&InterfaceVersion::new(version).unwrap()),
                InterfaceCompatibilityAssessment::UnverifiedNewer(newer)
                    if newer.behavior_revision().as_str()
                        == "ollama.native-text-v1.manifest-list-runner"
            ),
            "{version} must stay unverified newer"
        );
    }
}

fn fingerprint<'a>(inventory: &'a Value, version: &str) -> &'a Value {
    &inventory["selected_path_fingerprints"]
        .as_array()
        .unwrap()
        .iter()
        .find(|point| point["version"] == version)
        .unwrap_or_else(|| panic!("missing fingerprints at {version}"))["files"]
}

fn blob_map(inventory: &Value, version: &str) -> BTreeMap<String, String> {
    inventory["points"]
        .as_array()
        .unwrap()
        .iter()
        .find(|point| point["version"] == version)
        .unwrap_or_else(|| panic!("inventory omits {version}"))["entries"]
        .as_array()
        .expect("tree entries are complete")
        .iter()
        .filter(|row| row[2] == "blob")
        .map(|row| {
            (
                row[0].as_str().expect("path is text").to_owned(),
                row[3].as_str().expect("blob is text").to_owned(),
            )
        })
        .collect()
}

fn hop_paths(rows: &Value) -> BTreeSet<String> {
    rows.as_array()
        .expect("hop rows are an array")
        .iter()
        .map(|row| row["path"].as_str().expect("path is text").to_owned())
        .collect()
}

fn path_list_sha256(paths: &BTreeSet<String>) -> String {
    let mut bytes = paths
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .join("\n")
        .into_bytes();
    bytes.push(b'\n');
    sha256_hex(&bytes)
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
