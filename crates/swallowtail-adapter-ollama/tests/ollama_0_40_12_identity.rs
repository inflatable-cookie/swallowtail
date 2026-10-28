use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use swallowtail_adapter_ollama::{
    OLLAMA_BASELINE_VERSION, OLLAMA_LATEST_QUALIFIED_VERSION, ollama_runtime_claim,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const IDENTITY: &str = include_str!("fixtures/ollama-0.40.12/identity.json");
const PROTOCOL: &str = include_str!("fixtures/ollama-0.40.12/protocol.json");
const INVENTORY: &str = include_str!("fixtures/ollama-0.40.12/dist-inventory.json");
const HISTORICAL: &str = include_str!("fixtures/ollama-0.40.11/identity.json");

const MAINTAINED: &[&str] = &[
    "0.40.0", "0.40.1", "0.40.2", "0.40.3", "0.40.4", "0.40.5", "0.40.6", "0.40.7", "0.40.8",
    "0.40.9", "0.40.10", "0.40.11",
];

#[test]
fn identity_freezes_the_04012_hop_from_the_04011_ceiling() {
    let identity: Value = serde_json::from_str(IDENTITY).expect("identity corpus parses");
    let protocol: Value = serde_json::from_str(PROTOCOL).expect("protocol corpus parses");
    let inventory: Value = serde_json::from_str(INVENTORY).expect("tree inventory parses");
    let historical: Value =
        serde_json::from_str(HISTORICAL).expect("historical qualification parses");

    assert_eq!(identity["axis"], "ollama.runtime");
    assert_eq!(identity["package"], "swallowtail-adapter-ollama");
    assert_eq!(identity["research"], 462);
    assert_eq!(identity["previous_ceiling"], "0.40.11");
    assert_eq!(identity["host_observation"]["client_on_path"], false);
    assert_eq!(identity["host_observation"]["host_not_mutated"], true);
    assert_eq!(identity["selected_channel"]["latest_version"], "0.40.12");
    assert_eq!(identity["selected_channel"]["draft"], false);
    assert_eq!(identity["selected_channel"]["prerelease"], false);
    assert_eq!(identity["selected_channel"]["release_id"], 424898500);
    assert_eq!(
        identity["selected_channel"]["published_at"],
        "2026-10-27T00:07:12Z"
    );
    assert_eq!(
        identity["selected_channel"]["tag_commit"],
        "f067033f9126993b6172bdfffa71a74ad693fda4"
    );
    assert_eq!(
        identity["selected_channel"]["tree"],
        "5b73640f43d4a4b0a5a03be5752693e654c0f4b8"
    );
    assert_eq!(
        identity["selected_channel"]["source_archive_sha256"],
        "b625d6dd6b4650366872b2d0263167993411b734838d9b773a22076a8cf7f3fc"
    );
    assert_eq!(
        identity["selected_channel"]["pre_push_reprobe_at"],
        "2026-10-28T09:38:37Z"
    );
    assert_eq!(
        identity["selected_channel"]["pre_push_reprobe_version"],
        "0.40.12"
    );
    assert_eq!(
        identity["selected_channel"]["pre_push_reprobe_release_id"],
        424898500
    );
    assert_eq!(
        identity["selected_channel"]["pre_push_reprobe_tag_commit"],
        "f067033f9126993b6172bdfffa71a74ad693fda4"
    );
    assert_eq!(
        historical["decision"]["latest_qualified_after_change"],
        "0.40.11"
    );
    assert_eq!(
        identity["previous_ceiling_reproduced"]["source_archive_sha256"],
        "9733d6230d8c8a75ccdf368c53132d90ee49009ce677776d897e96b50cc1c654"
    );
    assert_eq!(
        identity["previous_ceiling_reproduced"]["matches_research_448"],
        true
    );
    assert_eq!(
        identity["current_claim_at_observation"]["latest_qualified"],
        "0.40.11"
    );
    assert_eq!(
        identity["decision"]["shape"],
        "stop-selected-dependency-and-renderer-gap"
    );
    assert_eq!(
        identity["decision"]["latest_qualified_after_change"],
        "0.40.11"
    );
    assert_eq!(
        identity["decision"]["synthetic_unverified_newer"],
        "0.40.12"
    );
    assert_eq!(identity["decision"]["new_public_operation"], false);
    assert_eq!(identity["decision"]["provider_prompt_sent"], false);
    assert_eq!(identity["decision"]["attached_server_started"], false);
    let ignored = identity["ignored_prerelease_tags_present"]
        .as_array()
        .unwrap();
    assert_eq!(ignored[0]["tag"], "v0.40.12-rc0");
    assert_ne!(
        ignored[0]["commit"],
        identity["selected_channel"]["tag_commit"]
    );
    assert_eq!(ignored[1]["tag"], "v0.40.12-rc1");
    assert_eq!(
        ignored[1]["commit"],
        identity["selected_channel"]["tag_commit"]
    );

    let published = identity["published_stables_after_previous_ceiling"]
        .as_array()
        .unwrap();
    assert_eq!(
        published
            .iter()
            .map(|release| release["version"].as_str().unwrap())
            .collect::<Vec<_>>(),
        ["0.40.12"]
    );

    let compared = ["0.40.11", "0.40.12"];
    assert_eq!(
        inventory["points"]
            .as_array()
            .unwrap()
            .iter()
            .map(|point| point["version"].as_str().unwrap())
            .collect::<Vec<_>>(),
        compared
    );
    for version in compared {
        let point = inventory["points"]
            .as_array()
            .unwrap()
            .iter()
            .find(|point| point["version"] == version)
            .unwrap_or_else(|| panic!("tree inventory omits {version}"));
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
    assert_eq!(inventory["points"][0]["file_count"], 1513);
    assert_eq!(inventory["points"][0]["tree_entry_count"], 1708);
    assert_eq!(inventory["points"][1]["file_count"], 1577);
    assert_eq!(inventory["points"][1]["tree_entry_count"], 1776);

    let hop = &inventory["version_ordered_hops"][0];
    assert_eq!(hop["from"], "0.40.11");
    assert_eq!(hop["to"], "0.40.12");
    let before = blob_map(&inventory, "0.40.11");
    let after = blob_map(&inventory, "0.40.12");
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
    assert_eq!(hop_paths(&hop["added"]), derived_added);
    assert_eq!(hop_paths(&hop["removed"]), derived_removed);
    assert_eq!(hop_paths(&hop["changed"]), derived_changed);
    assert_eq!(hop["added_file_count"], derived_added.len());
    assert_eq!(hop["removed_file_count"], derived_removed.len());
    assert_eq!(hop["changed_file_count"], derived_changed.len());
    assert_eq!(hop["unchanged_file_count"], derived_unchanged.len());
    assert_eq!(
        hop["unchanged_path_list_sha256"],
        path_list_sha256(&derived_unchanged)
    );

    let mut ledger = String::new();
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
                    "0.40.11\t0.40.12\tchanged\t{path}\t{class}\t{}\t{}\n",
                    row["before_blob"].as_str().unwrap(),
                    row["after_blob"].as_str().unwrap()
                ));
            } else {
                ledger.push_str(&format!(
                    "0.40.11\t0.40.12\t{section}\t{path}\t{class}\t{}\n",
                    row["blob"].as_str().unwrap()
                ));
            }
        }
    }
    assert_eq!(
        sha256_hex(ledger.as_bytes()),
        "452ad16d14e2be87e814d996579437aaddfcb5f7710a001516c49ef7b5fa31aa"
    );
    assert_eq!(
        protocol["classification_ledger_sha256"],
        "452ad16d14e2be87e814d996579437aaddfcb5f7710a001516c49ef7b5fa31aa"
    );
    assert_eq!(
        inventory["classification_ledger_sha256"],
        protocol["classification_ledger_sha256"]
    );
    assert_eq!(protocol["api_types_go_byte_identical"], true);
    assert_eq!(protocol["migration_source_byte_identical"], true);
    assert_eq!(protocol["migration_call_still_commented_out"], true);
    assert_eq!(protocol["new_public_swallowtail_operation"], false);
    assert_eq!(
        protocol["unselected_new_operations"],
        serde_json::json!([
            "POST /v1/audio/transcriptions",
            "POST /v1/audio/translations"
        ])
    );

    let changed_class = hop["changed"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            (
                row["path"].as_str().unwrap().to_owned(),
                row["classification"].as_str().unwrap().to_owned(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    assert_eq!(
        changed_class.get("server/routes.go").map(String::as_str),
        Some("selected-native-routes-unchanged")
    );
    assert_eq!(
        changed_class.get("server/images.go").map(String::as_str),
        Some("selected-decision-public-capabilities")
    );
    assert_eq!(
        changed_class
            .get("types/model/config.go")
            .map(String::as_str),
        Some("selected-show-additive-voices")
    );
    assert_eq!(
        changed_class.get("LLAMA_CPP_VERSION").map(String::as_str),
        Some("selected-gguf-dependency-gap")
    );
    assert_eq!(
        changed_class
            .get("model/renderers/renderer.go")
            .map(String::as_str),
        Some("renderer-parser-reachability-gap")
    );
    let gap = &protocol["selected_dependency_and_renderer_gap"];
    assert_eq!(gap["pin_before"], "b11845");
    assert_eq!(gap["pin_after"], "b11901");
    assert_eq!(gap["default_go_template"], true);
    assert_eq!(
        gap["paths"],
        serde_json::json!([
            "server/sched.go",
            "llm/server.go",
            "llama/server/CMakeLists.txt",
            "server/prompt.go",
            "server/routes.go",
            "envconfig/config.go",
            "model/renderers/renderer.go"
        ])
    );
    for (version, path, blob) in [
        (
            "0.40.11",
            "envconfig/config.go",
            "0379a2aec2a9573d542deb840766a93694f63ffe",
        ),
        (
            "0.40.11",
            "llama/server/CMakeLists.txt",
            "9153c9bf019548f7015305b773a9fed2a61f3fcc",
        ),
        (
            "0.40.11",
            "llm/server.go",
            "d0b90396fcd79186aa320a8c6173b08ed5e5d0fe",
        ),
        (
            "0.40.11",
            "model/renderers/renderer.go",
            "df7fb9674fafbe934efc9a3a32eb902155a392d2",
        ),
        (
            "0.40.11",
            "server/prompt.go",
            "17b50efc652865d9098801d0ab27570be7060cc0",
        ),
        (
            "0.40.11",
            "server/routes.go",
            "fdf7a18f954ab7c249bb0e95a859b399105c7289",
        ),
        (
            "0.40.11",
            "server/sched.go",
            "2c8d818d1907d73a13db1174bb568718be87f3b6",
        ),
        (
            "0.40.12",
            "envconfig/config.go",
            "0379a2aec2a9573d542deb840766a93694f63ffe",
        ),
        (
            "0.40.12",
            "llama/server/CMakeLists.txt",
            "9153c9bf019548f7015305b773a9fed2a61f3fcc",
        ),
        (
            "0.40.12",
            "llm/server.go",
            "d0b90396fcd79186aa320a8c6173b08ed5e5d0fe",
        ),
        (
            "0.40.12",
            "model/renderers/renderer.go",
            "672df033c583b964d0a87b0241a0e2683e377111",
        ),
        (
            "0.40.12",
            "server/prompt.go",
            "17b50efc652865d9098801d0ab27570be7060cc0",
        ),
        (
            "0.40.12",
            "server/routes.go",
            "082dd1121fb999e92fc1be6bd3bede8e9e38ab38",
        ),
        (
            "0.40.12",
            "server/sched.go",
            "2c8d818d1907d73a13db1174bb568718be87f3b6",
        ),
    ] {
        assert_eq!(gap["path_blobs"][version][path], blob);
        assert_eq!(blob_map(&inventory, version)[path], blob);
    }
    assert_eq!(gap["path_blobs"]["0.40.11"].as_object().unwrap().len(), 7);
    assert_eq!(gap["path_blobs"]["0.40.12"].as_object().unwrap().len(), 7);
    assert_eq!(
        before["LLAMA_CPP_VERSION"],
        "80291d7f1423525ef37228948f78fb69f5407aa2"
    );
    assert_eq!(
        after["LLAMA_CPP_VERSION"],
        "98bc1f1660e3264d1df83952b9e8a452e22a6088"
    );
    assert!(!changed_class.contains_key("api/types.go"));
    assert!(!changed_class.contains_key("server/model_list.go"));
    assert!(!changed_class.contains_key("compatmigrate/migrate.go"));
    assert!(!changed_class.contains_key("manifest/paths.go"));

    for version in compared {
        let point = fingerprint(&inventory, version);
        assert_eq!(
            point["api/types.go"]["blob"],
            "1aa71342a372fc2a0902cc55f43afe91c3a55d06"
        );
        assert_eq!(
            point["compatmigrate/migrate.go"]["blob"],
            "d6a5ce939218e4f6092b30322cb39b1fc9dad35e"
        );
        assert_eq!(
            point["manifest/paths.go"]["blob"],
            "cf3bfa2569bf1a3fbc69ef12455566e259515a2d"
        );
        assert_eq!(
            point["server/model_list.go"]["blob"],
            "6be2862816f5ba2d6101812a1a03ee98fc079b8d"
        );
    }
    assert_eq!(
        fingerprint(&inventory, "0.40.11")["server/routes.go"]["blob"],
        "fdf7a18f954ab7c249bb0e95a859b399105c7289"
    );
    assert_eq!(
        fingerprint(&inventory, "0.40.12")["server/routes.go"]["blob"],
        "082dd1121fb999e92fc1be6bd3bede8e9e38ab38"
    );
    assert_eq!(
        fingerprint(&inventory, "0.40.11")["server/images.go"]["blob"],
        "87950edd50f67934000adb64b531fb3e09c31c2b"
    );
    assert_eq!(
        fingerprint(&inventory, "0.40.12")["server/images.go"]["blob"],
        "7c8d650a17ef5756044ae2ba47b3b3788754c738"
    );
}

#[test]
fn claim_retains_manifest_list_runner_ceiling_at_04011() {
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
        "0.40.12-rc0",
        "0.40.12-rc1",
    ] {
        assert!(
            !claim.permits(&InterfaceVersion::new(version).unwrap()),
            "{version} must stay unqualified"
        );
    }
    for version in ["0.40.12", "0.40.13", "0.41.0"] {
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
