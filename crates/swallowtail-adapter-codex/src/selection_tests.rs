use super::*;
use swallowtail_core::{InterfaceSupportStatus, InterfaceVersion};
use swallowtail_testkit::{ClosedSemanticWindowCase, assert_closed_semantic_compatibility_window};

#[test]
fn exec_claim_is_closed_at_the_corpus_boundaries() {
    let case = ClosedSemanticWindowCase::new(
        InterfaceVersion::new("0.80.0").unwrap(),
        InterfaceVersion::new("0.145.0").unwrap(),
    )
    .with_accepted([
        InterfaceVersion::new("0.81.0").unwrap(),
        InterfaceVersion::new("0.84.0").unwrap(),
        InterfaceVersion::new("0.98.0").unwrap(),
        InterfaceVersion::new("0.99.0").unwrap(),
        InterfaceVersion::new("0.121.0").unwrap(),
        InterfaceVersion::new("0.122.0").unwrap(),
        InterfaceVersion::new("0.130.0").unwrap(),
        InterfaceVersion::new("0.144.6").unwrap(),
    ])
    .with_rejected([
        InterfaceVersion::new("0.79.0").unwrap(),
        InterfaceVersion::new("0.82.0").unwrap(),
        InterfaceVersion::new("0.83.0").unwrap(),
        InterfaceVersion::new("0.108.0").unwrap(),
        InterfaceVersion::new("0.109.0").unwrap(),
        InterfaceVersion::new("0.146.0-alpha.4").unwrap(),
        InterfaceVersion::new("0.146.0").unwrap(),
    ]);
    assert_closed_semantic_compatibility_window(&codex_exec_claim(), &case);
    assert_eq!(
        codex_exec_claim()
            .classify(codex_cli_binding("0.121.0").version())
            .unwrap()
            .support_status(),
        InterfaceSupportStatus::Deprecated
    );
}

#[test]
fn app_server_claim_dispatches_at_workspace_root_milestone() {
    let claim = codex_app_server_claim();
    for version in ["0.80.0", "0.81.0", "0.84.0", "0.94.0", "0.99.0"] {
        let matched = claim
            .classify(codex_cli_binding(version).version())
            .unwrap();
        assert_eq!(
            matched.behavior_revision().as_str(),
            CODEX_APP_SERVER_LEGACY_DEFAULT_BEHAVIOR
        );
        assert_eq!(matched.support_status(), InterfaceSupportStatus::Deprecated);
    }
    for version in ["0.100.0", "0.107.0"] {
        let matched = claim
            .classify(codex_cli_binding(version).version())
            .unwrap();
        assert_eq!(
            matched.behavior_revision().as_str(),
            CODEX_APP_SERVER_LEGACY_EXPLICIT_BEHAVIOR
        );
        assert_eq!(matched.support_status(), InterfaceSupportStatus::Deprecated);
    }
    for version in ["0.110.0", "0.120.0", "0.130.0"] {
        assert_eq!(
            claim
                .classify(codex_cli_binding(version).version())
                .unwrap()
                .behavior_revision()
                .as_str(),
            CODEX_APP_SERVER_BASE_BEHAVIOR
        );
    }
    for version in ["0.131.0", "0.140.0", "0.144.6", "0.145.0"] {
        assert_eq!(
            claim
                .classify(codex_cli_binding(version).version())
                .unwrap()
                .behavior_revision()
                .as_str(),
            CODEX_APP_SERVER_WORKSPACE_BEHAVIOR
        );
    }
    for version in [
        "0.79.0",
        "0.82.0",
        "0.83.0",
        "0.108.0",
        "0.109.0",
        "0.146.0-alpha.4",
        "0.146.0",
    ] {
        assert!(!claim.supports(codex_cli_binding(version).version()));
    }
}
