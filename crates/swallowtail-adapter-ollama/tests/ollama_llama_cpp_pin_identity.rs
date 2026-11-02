use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use swallowtail_adapter_ollama::{OLLAMA_LATEST_QUALIFIED_VERSION, ollama_runtime_claim};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const IDENTITY: &str = include_str!("fixtures/ollama-llama-cpp-pin/identity.json");
const DELTA: &str = include_str!("fixtures/ollama-llama-cpp-pin/delta-inventory.json");
const CMAKE: &str = include_str!("fixtures/ollama-llama-cpp-pin/llama-server-cmake.txt");
const PINNED_TREES: &str = include_str!("fixtures/ollama-0.40.12/dist-inventory.json");

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
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

#[test]
fn identity_freezes_both_llama_cpp_pins_as_immutable_commits() {
    let identity: Value = serde_json::from_str(IDENTITY).expect("identity corpus parses");

    assert_eq!(identity["schema"], 1);
    assert_eq!(
        identity["task"],
        "ollama.attached selected llama.cpp pin qualification"
    );
    assert_eq!(identity["research"], 466);
    assert_eq!(identity["axis"], "ollama.runtime");
    assert_eq!(
        identity["github_repositories"]["llama_cpp"],
        "ggml-org/llama.cpp"
    );
    assert_eq!(identity["host_observation"]["vendor_executed"], false);
    assert_eq!(identity["host_observation"]["host_not_mutated"], true);
    assert_eq!(
        identity["host_observation"]["live_inference_performed"],
        false
    );

    assert_eq!(
        identity["ollama_version_file"]["blob_at_0_40_11"],
        "80291d7f1423525ef37228948f78fb69f5407aa2"
    );
    assert_eq!(
        identity["ollama_version_file"]["blob_at_0_40_12"],
        "98bc1f1660e3264d1df83952b9e8a452e22a6088"
    );
    assert_eq!(
        identity["ollama_version_file"]["content_at_0_40_11"],
        "b11845"
    );
    assert_eq!(
        identity["ollama_version_file"]["content_at_0_40_12"],
        "b11901"
    );

    let pins = &identity["pins"];
    assert_eq!(pins["b11845"]["tag_object_type"], "commit");
    assert_eq!(
        pins["b11845"]["commit"],
        "9ff0d7e90f0faaa7c16017e6dcf9cf91c50a100e"
    );
    assert_eq!(
        pins["b11845"]["tree"],
        "60bf42018da47404b7260ee2bebd108b6c1b511f"
    );
    assert_eq!(pins["b11901"]["tag_object_type"], "commit");
    assert_eq!(
        pins["b11901"]["commit"],
        "5320c87ff0d994845b44ce7976f10055ec697447"
    );
    assert_eq!(
        pins["b11901"]["tree"],
        "3e71dcf38c6806ff99c75d3b1879f6501cb0834f"
    );
    assert_ne!(pins["b11845"]["commit"], pins["b11901"]["commit"]);

    // The frozen pin identities agree with the frozen Ollama tree inventories.
    let inventory: Value = serde_json::from_str(PINNED_TREES).expect("tree inventory parses");
    let before = blob_map(&inventory, "0.40.11");
    let after = blob_map(&inventory, "0.40.12");
    assert_eq!(
        before["LLAMA_CPP_VERSION"],
        identity["ollama_version_file"]["blob_at_0_40_11"]
    );
    assert_eq!(
        after["LLAMA_CPP_VERSION"],
        identity["ollama_version_file"]["blob_at_0_40_12"]
    );
    assert_eq!(
        before["llama/server/CMakeLists.txt"],
        identity["pin_consumer"]["blob"]
    );
    assert_eq!(
        after["llama/server/CMakeLists.txt"],
        identity["pin_consumer"]["blob"]
    );
    for (tree, blob) in [
        ("llama", "80f61a2e8dbebf291381aa4a60d181519837d3f4"),
        ("llama/compat", "461267684d054bac7066f11ed98d7e2b86be8be5"),
        ("llama/server", "197a2991c4a12874c8dfcfb8773cdbc5a54fcf03"),
    ] {
        assert_eq!(
            identity["llama_tree_identical_across_hop"][tree], blob,
            "{tree} must record the one identical tree across the hop"
        );
    }
    assert_eq!(
        identity["compat_layer"]["hook_patch"]["blob"],
        "6610052094be088cc8f2452b4dc7960be7e1f42f"
    );
    assert_eq!(
        identity["compat_layer"]["hook_patch"]["blob_identical_across_hop"],
        true
    );
    assert_eq!(identity["compat_layer"]["tree_identical_across_hop"], true);
    for (path, blob) in [
        (
            "server/sched.go",
            "2c8d818d1907d73a13db1174bb568718be87f3b6",
        ),
        ("llm/server.go", "d0b90396fcd79186aa320a8c6173b08ed5e5d0fe"),
        (
            "llm/llama_server.go",
            "5805edd64d99f17dc979cc17f480f87509db926d",
        ),
        (
            "server/prompt.go",
            "17b50efc652865d9098801d0ab27570be7060cc0",
        ),
        (
            "server/renderer_resolution.go",
            "53f87355a7689a5627a23c8e0696f2945a49852f",
        ),
        (
            "server/model_thinking.go",
            "88d1347a226b00d579ee490068c4f9caa6d12f87",
        ),
        (
            "server/model_show_cache.go",
            "f829e477497da4d23e2dd8496951ea61b96a00ed",
        ),
        (
            "envconfig/config.go",
            "0379a2aec2a9573d542deb840766a93694f63ffe",
        ),
    ] {
        assert_eq!(
            identity["selected_go_surface_byte_identical_across_hop"][path],
            blob
        );
        assert_eq!(
            before[path], blob,
            "{path} blob must match the frozen 0.40.11 tree"
        );
        assert_eq!(
            after[path], blob,
            "{path} blob must match the frozen 0.40.12 tree"
        );
    }
}

#[test]
fn pin_consumer_freezes_the_fetchcontent_contract() {
    let identity: Value = serde_json::from_str(IDENTITY).expect("identity corpus parses");

    assert_eq!(
        sha256_hex(CMAKE.as_bytes()),
        identity["pin_consumer"]["fixture_content_sha256"]
    );
    assert_eq!(
        identity["pin_consumer"]["fetchcontent_git_repository"],
        "https://github.com/ggml-org/llama.cpp.git"
    );
    assert_eq!(identity["pin_consumer"]["git_shallow"], true);
    assert!(CMAKE.contains(r#"GIT_REPOSITORY "https://github.com/ggml-org/llama.cpp.git""#));
    assert!(CMAKE.contains("LLAMA_CPP_GIT_TAG"));
    assert!(CMAKE.contains("../../LLAMA_CPP_VERSION"));
    assert!(CMAKE.contains("OLLAMA_LLAMA_CPP_SOURCE"));
    assert!(CMAKE.contains("FETCHCONTENT_SOURCE_DIR_LLAMA_CPP"));
    assert!(CMAKE.contains("OLLAMA_LLAMA_CPP_SKIP_COMPAT_PATCH"));
    assert_eq!(
        identity["pin_consumer"]["compat_runtime_disable_env"],
        "OLLAMA_LLAMA_CPP_COMPAT"
    );
}

#[test]
fn delta_ledger_classifies_every_pin_delta_file() {
    let identity: Value = serde_json::from_str(IDENTITY).expect("identity corpus parses");
    let delta: Value = serde_json::from_str(DELTA).expect("delta corpus parses");

    assert_eq!(delta["schema"], 1);
    assert_eq!(delta["research"], 466);
    assert_eq!(delta["commit_span"], 56);
    assert_eq!(
        delta["from"]["commit"],
        "9ff0d7e90f0faaa7c16017e6dcf9cf91c50a100e"
    );
    assert_eq!(
        delta["to"]["commit"],
        "5320c87ff0d994845b44ce7976f10055ec697447"
    );
    assert_eq!(delta["file_counts"]["added"], 20);
    assert_eq!(delta["file_counts"]["removed"], 1);
    assert_eq!(delta["file_counts"]["modified"], 242);
    assert_eq!(delta["file_counts"]["total"], 263);

    let rows = delta["rows"].as_array().expect("rows");
    assert_eq!(rows.len(), 263);
    let mut ledger = String::new();
    let mut counts = BTreeMap::new();
    for row in rows {
        let path = row["path"].as_str().unwrap();
        let status = row["status"].as_str().unwrap();
        let class = row["classification"].as_str().expect("classification");
        assert!(!class.is_empty(), "{path} needs a classification");
        assert!(
            row["reason"]
                .as_str()
                .is_some_and(|reason| !reason.is_empty()),
            "{path} needs a reason"
        );
        assert!(
            matches!(status, "added" | "removed" | "modified"),
            "{path} has unknown status {status}"
        );
        if status == "added" {
            assert_eq!(
                row["before_blob"],
                Value::Null,
                "{path} must have no before blob"
            );
        }
        if status == "removed" {
            assert_eq!(
                row["after_blob"],
                Value::Null,
                "{path} must have no after blob"
            );
        }
        *counts.entry((status, class)).or_insert(0usize) += 1;
        ledger.push_str(&format!(
            "b11845\tb11901\t{status}\t{path}\t{class}\t{}\t{}\n",
            row["before_blob"].as_str().unwrap_or("none"),
            row["after_blob"].as_str().unwrap_or("none"),
        ));
    }
    assert_eq!(
        sha256_hex(ledger.as_bytes()),
        delta["classification_ledger_sha256"]
    );
    assert_eq!(
        sha256_hex(ledger.as_bytes()),
        identity["delta_ledger_sha256"]
    );
    assert_eq!(identity["delta_file_counts"], delta["file_counts"]);

    for ((status, class), expected) in [
        (("modified", "selected-ggml-backend"), 77),
        (("modified", "selected-llama-core"), 24),
        (("modified", "selected-mtmd-media"), 17),
        (("modified", "selected-server-surface"), 9),
        (("modified", "selected-common-jinja"), 6),
        (("modified", "selected-common-grammar"), 5),
        (("modified", "selected-common"), 3),
        (("modified", "selected-common-chat"), 3),
        (("modified", "selected-build-graph"), 3),
        (("modified", "selected-server-docs"), 2),
        (("modified", "unselected-ui"), 52),
        (("modified", "unselected-tests"), 15),
        (("modified", "unselected-ci-contrib"), 10),
        (("modified", "unselected-download-cache"), 4),
        (("modified", "unselected-python"), 6),
        (("modified", "unselected-quantize"), 3),
        (("modified", "unselected-tools"), 2),
        (("modified", "selected-server-test"), 1),
        (("added", "selected-mtmd-media"), 1),
        (("added", "selected-llama-core"), 1),
        (("added", "unselected-download-cache"), 2),
        (("added", "unselected-python"), 2),
        (("added", "unselected-ui"), 5),
        (("added", "selected-ggml-backend"), 8),
        (("added", "unselected-tests"), 1),
        (("removed", "unselected-ui"), 1),
    ] {
        assert_eq!(
            counts.get(&(status, class)),
            Some(&expected),
            "{status}/{class} count drifted"
        );
    }

    let class_of = |path: &str| {
        rows.iter()
            .find(|row| row["path"] == path)
            .unwrap_or_else(|| panic!("delta omits {path}"))["classification"]
            .as_str()
            .unwrap()
            .to_owned()
    };
    assert_eq!(
        class_of("tools/server/server.cpp"),
        "selected-server-surface"
    );
    assert_eq!(
        class_of("tools/server/server-schema.cpp"),
        "selected-server-surface"
    );
    assert_eq!(class_of("src/llama-vocab.cpp"), "selected-llama-core");
    assert_eq!(class_of("src/llama-sampler.cpp"), "selected-llama-core");
    assert_eq!(class_of("src/llama-grammar.cpp"), "selected-llama-core");
    assert_eq!(class_of("src/models/deepseek41.cpp"), "selected-llama-core");
    assert_eq!(class_of("ggml/src/ggml.c"), "selected-ggml-backend");
    assert_eq!(class_of("common/jinja/parser.cpp"), "selected-common-jinja");
    assert_eq!(class_of("common/chat.cpp"), "selected-common-chat");
    assert_eq!(
        class_of("common/json-schema-to-grammar.cpp"),
        "selected-common-grammar"
    );
    assert_eq!(class_of("common/arg.cpp"), "selected-common");
    assert_eq!(class_of("common/CMakeLists.txt"), "selected-build-graph");
    assert_eq!(
        class_of("tools/mtmd/CMakeLists.txt"),
        "selected-build-graph"
    );
    assert_eq!(
        class_of("ggml/src/ggml-opencl/CMakeLists.txt"),
        "selected-build-graph"
    );
    assert_eq!(class_of("tools/mtmd/clip.cpp"), "selected-mtmd-media");
    assert_eq!(class_of("common/download.cpp"), "unselected-download-cache");
    assert_eq!(class_of("tools/ui/src/app.css"), "unselected-ui");
    assert_eq!(class_of("src/llama-quant.cpp"), "unselected-quantize");
    assert_eq!(class_of("tests/test-chat.cpp"), "unselected-tests");
    assert_eq!(class_of("tools/server/README.md"), "selected-server-docs");
    assert_eq!(
        class_of("tools/server/tests/utils.py"),
        "selected-server-test"
    );
}

#[test]
fn claim_ceiling_stays_04011_and_04012_stays_unverified() {
    assert_eq!(OLLAMA_LATEST_QUALIFIED_VERSION, "0.40.11");
    let identity: Value = serde_json::from_str(IDENTITY).expect("identity corpus parses");
    assert_eq!(identity["decision"]["claim_ceiling"], "0.40.11");
    assert_eq!(
        identity["decision"]["claim_id"],
        "ollama.native-runtime-window-2"
    );
    assert_eq!(
        identity["decision"]["behavior_revision"],
        "ollama.native-text-v1.manifest-list-runner"
    );
    assert_eq!(identity["decision"]["claim_changed"], false);
    assert_eq!(identity["decision"]["new_public_operation"], false);

    let claim = ollama_runtime_claim();
    assert!(matches!(
        claim.assess(&InterfaceVersion::new("0.40.11").unwrap()),
        InterfaceCompatibilityAssessment::Qualified(point)
            if point.support_status() == InterfaceSupportStatus::Maintained
                && point.behavior_revision().as_str()
                    == "ollama.native-text-v1.manifest-list-runner"
    ));
    for version in ["0.40.12", "0.40.13"] {
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
