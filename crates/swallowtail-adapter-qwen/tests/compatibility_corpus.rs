use serde_json::Value;
use std::collections::BTreeSet;
use swallowtail_adapter_qwen::{QWEN_CODE_LATEST_QUALIFIED_VERSION, qwen_headless_claim};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

const CORPUS: &str = include_str!("fixtures/qwen-code-v0.19.11-v0.21.2/compatibility.json");

fn corpus() -> Value {
    serde_json::from_str(CORPUS).expect("Qwen compatibility corpus is valid JSON")
}

fn records(corpus: &Value) -> &[Value] {
    corpus["stable_versions"]
        .as_array()
        .expect("stable versions are an array")
}

#[test]
fn corpus_classifies_every_stable_point_and_exact_package_identity() {
    let corpus = corpus();
    let records = records(&corpus);
    let versions = records
        .iter()
        .map(|record| record["version"].as_str().expect("version is text"))
        .collect::<Vec<_>>();

    assert_eq!(
        versions,
        [
            "0.19.11", "0.19.12", "0.20.0", "0.20.1", "0.21.0", "0.21.1", "0.21.2",
        ]
    );
    assert_eq!(versions.iter().copied().collect::<BTreeSet<_>>().len(), 7);
    assert!(records.iter().all(|record| {
        let commit = record["commit"].as_str().expect("commit is text");
        let integrity = record["npm_integrity"].as_str().expect("integrity is text");
        commit.len() == 40
            && commit.bytes().all(|byte| byte.is_ascii_hexdigit())
            && integrity.starts_with("sha512-")
            && !record["version"]
                .as_str()
                .expect("version is text")
                .contains('-')
    }));
}

#[test]
fn stream_contract_is_unchanged_and_catalogue_filter_starts_at_0_21_0() {
    let corpus = corpus();
    let records = records(&corpus);
    let stream_blobs = records
        .iter()
        .map(|record| record["stream_types_blob"].as_str().unwrap())
        .collect::<BTreeSet<_>>();
    assert_eq!(stream_blobs.len(), 1);

    for record in records {
        let version = record["version"].as_str().unwrap();
        let later_catalogue = version >= "0.21.0";
        assert_eq!(record["catalogue_image_only_filter"], later_catalogue);
        assert_eq!(
            record["candidate_behavior_revision"],
            if later_catalogue {
                "qwen-code.headless.v0.21.0-catalogue-filter"
            } else {
                "qwen-code.headless.v0.19.11"
            }
        );
    }
}

#[test]
fn every_selected_safety_protocol_and_resume_surface_exists_across_the_interval() {
    let corpus = corpus();
    let flags = corpus["selected_flags"].as_array().unwrap();
    let controls = corpus["selected_controls"].as_array().unwrap();
    let tools = corpus["selected_read_tools"].as_array().unwrap();

    for required in [
        "--safe-mode",
        "--approval-mode",
        "--core-tools",
        "--exclude-tools",
        "--max-wall-time",
        "--max-tool-calls",
        "--max-session-turns",
        "--include-partial-messages",
        "--input-format",
        "--output-format",
        "--resume",
    ] {
        assert!(flags.iter().any(|value| value == required));
    }
    for required in ["can_get_available_models", "get_available_models"] {
        assert!(controls.iter().any(|value| value == required));
    }
    assert_eq!(
        tools
            .iter()
            .map(|value| value.as_str().expect("tool is text"))
            .collect::<Vec<_>>(),
        ["read_file", "grep_search", "glob", "list_directory", "lsp"]
    );
}

#[test]
fn production_claim_keeps_the_frozen_corpus_inside_the_raised_window() {
    assert_eq!(QWEN_CODE_LATEST_QUALIFIED_VERSION, "0.21.15");
    let claim = qwen_headless_claim();
    for candidate in [
        "0.19.11", "0.19.12", "0.20.0", "0.20.1", "0.21.0", "0.21.1", "0.21.2", "0.21.13",
        "0.21.14", "0.21.15",
    ] {
        assert!(claim.supports(&version(candidate)));
    }
    assert!(!claim.permits(&version("0.20.2")));
    assert!(matches!(
        claim.assess(&version("0.21.16")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert!(!claim.permits(&version("0.21.14-rc.1")));
    assert!(!claim.permits(&version("0.21.14-preview.0")));
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}
