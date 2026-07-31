use serde_json::Value;
use std::collections::BTreeSet;
use swallowtail_adapter_pi::{PI_PACKAGE_LATEST_QUALIFIED_VERSION, pi_rpc_claim};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

const CORPUS: &str = include_str!("fixtures/pi-rpc-0.80.10-0.83.0/compatibility.json");

fn corpus() -> Value {
    serde_json::from_str(CORPUS).expect("Pi compatibility corpus is valid JSON")
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
        ["0.80.10", "0.81.0", "0.81.1", "0.82.0", "0.82.1", "0.83.0"]
    );
    assert_eq!(versions.iter().copied().collect::<BTreeSet<_>>().len(), 6);
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
fn corpus_freezes_exact_milestones_and_the_unchanged_cwd_gate() {
    let corpus = corpus();
    let records = records(&corpus);
    assert_eq!(
        records
            .iter()
            .map(|record| record["session_cwd_blob"].as_str().unwrap())
            .collect::<BTreeSet<_>>()
            .len(),
        1
    );

    for record in records {
        let version = record["version"].as_str().unwrap();
        assert_eq!(record["thinking_levels_command"], version >= "0.81.0");
        assert_eq!(record["nested_usage"], version >= "0.81.0");
        assert_eq!(record["summarization_retry_events"], version >= "0.81.1");
        assert_eq!(
            record["direct_bash_update_correlation"],
            version >= "0.82.0"
        );
        assert_eq!(record["direct_bash_extension_hook"], version >= "0.83.0");
    }
}

#[test]
fn corpus_keeps_unselected_commands_and_extensions_outside_authority() {
    let corpus = corpus();
    let selected = corpus["selected_commands"].as_array().unwrap();
    let absent = corpus["selected_absences"].as_array().unwrap();

    for command in [
        "prompt",
        "get_state",
        "get_available_models",
        "set_auto_retry",
    ] {
        assert!(selected.iter().any(|value| value == command));
    }
    for command in ["bash", "switch_session", "fork", "clone", "extensions"] {
        assert!(absent.iter().any(|value| value == command));
        assert!(!selected.iter().any(|value| value == command));
    }
}

#[test]
fn production_claim_matches_the_frozen_exact_segments() {
    assert_eq!(PI_PACKAGE_LATEST_QUALIFIED_VERSION, "0.83.0");
    let claim = pi_rpc_claim();
    for candidate in ["0.80.10", "0.81.0", "0.81.1", "0.82.0", "0.82.1", "0.83.0"] {
        assert!(claim.supports(&version(candidate)));
    }
    for unsupported in ["0.80.11", "0.81.2", "0.82.2"] {
        assert!(!claim.permits(&version(unsupported)));
    }
    assert!(matches!(
        claim.assess(&version("0.83.1")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert!(!claim.permits(&version("0.83.1-rc.1")));
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}
