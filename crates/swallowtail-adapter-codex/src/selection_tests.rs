use super::lifecycle::{
    ARCHIVE_BEHAVIOR, DESCENDANT_ARCHIVE_BEHAVIOR, HARD_DELETE_BEHAVIOR, NOTIFICATIONS_BEHAVIOR,
    RESTORE_BEHAVIOR,
};
use super::*;
use swallowtail_core::{InterfaceSupportStatus, InterfaceVersion};
use swallowtail_testkit::{ClosedSemanticWindowCase, assert_closed_semantic_compatibility_window};

fn binding(version: &str) -> InterfaceVersionBinding {
    codex_cli_binding(version).expect("fixture Codex version is valid")
}

#[test]
fn exec_claim_is_closed_at_the_corpus_boundaries() {
    let case = ClosedSemanticWindowCase::new(
        InterfaceVersion::new("0.80.0").unwrap(),
        InterfaceVersion::new("0.161.0").unwrap(),
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
        InterfaceVersion::new("0.145.0").unwrap(),
        InterfaceVersion::new("0.146.0").unwrap(),
        InterfaceVersion::new("0.147.0").unwrap(),
        InterfaceVersion::new("0.148.0").unwrap(),
        InterfaceVersion::new("0.149.0").unwrap(),
        InterfaceVersion::new("0.150.0").unwrap(),
        InterfaceVersion::new("0.150.1").unwrap(),
        InterfaceVersion::new("0.151.0").unwrap(),
        InterfaceVersion::new("0.152.0").unwrap(),
        InterfaceVersion::new("0.155.0").unwrap(),
        InterfaceVersion::new("0.155.1").unwrap(),
        InterfaceVersion::new("0.156.0").unwrap(),
        InterfaceVersion::new("0.156.1").unwrap(),
        InterfaceVersion::new("0.157.0").unwrap(),
        InterfaceVersion::new("0.157.1").unwrap(),
        InterfaceVersion::new("0.158.0").unwrap(),
        InterfaceVersion::new("0.159.0").unwrap(),
        InterfaceVersion::new("0.159.1").unwrap(),
        InterfaceVersion::new("0.159.2").unwrap(),
        InterfaceVersion::new("0.159.3").unwrap(),
        InterfaceVersion::new("0.160.0").unwrap(),
        InterfaceVersion::new("0.160.1").unwrap(),
        InterfaceVersion::new("0.161.0").unwrap(),
    ])
    .with_rejected([
        InterfaceVersion::new("0.79.0").unwrap(),
        InterfaceVersion::new("0.82.0").unwrap(),
        InterfaceVersion::new("0.83.0").unwrap(),
        InterfaceVersion::new("0.108.0").unwrap(),
        InterfaceVersion::new("0.109.0").unwrap(),
        InterfaceVersion::new("0.146.0-alpha.4").unwrap(),
        InterfaceVersion::new("0.149.2").unwrap(),
        InterfaceVersion::new("0.150.2").unwrap(),
        InterfaceVersion::new("0.151.1").unwrap(),
        InterfaceVersion::new("0.152.2").unwrap(),
        InterfaceVersion::new("0.154.1").unwrap(),
        InterfaceVersion::new("0.155.2").unwrap(),
    ]);
    assert_closed_semantic_compatibility_window(&codex_exec_claim(), &case);
    assert_eq!(
        codex_exec_claim()
            .classify(binding("0.121.0").version())
            .unwrap()
            .support_status(),
        InterfaceSupportStatus::Deprecated
    );
}

#[test]
fn exec_currentness_hops_reuse_the_maintained_wire_behavior() {
    for version in [
        "0.155.1", "0.156.0", "0.156.1", "0.157.0", "0.157.1", "0.158.0", "0.159.0", "0.159.1",
        "0.159.2", "0.159.3", "0.160.0", "0.160.1", "0.161.0",
    ] {
        let matched = codex_exec_claim()
            .classify(binding(version).version())
            .expect("qualified exec point has a segment");
        assert_eq!(matched.support_status(), InterfaceSupportStatus::Maintained);
        assert_eq!(matched.behavior_revision().as_str(), CODEX_EXEC_BEHAVIOR);
    }
    let InterfaceCompatibilityAssessment::UnverifiedNewer(newer) =
        codex_exec_claim().assess(binding("0.161.1").version())
    else {
        panic!("first stable release above the qualified ceiling stays visible");
    };
    assert_eq!(newer.latest_qualified().as_str(), "0.161.0");
}

#[test]
fn app_server_claim_dispatches_at_workspace_root_milestone() {
    let claim = codex_app_server_claim();
    for version in ["0.80.0", "0.81.0", "0.84.0", "0.94.0", "0.99.0"] {
        let matched = claim.classify(binding(version).version()).unwrap();
        assert_eq!(
            matched.behavior_revision().as_str(),
            CODEX_APP_SERVER_LEGACY_DEFAULT_BEHAVIOR
        );
        assert_eq!(matched.support_status(), InterfaceSupportStatus::Deprecated);
    }
    for version in ["0.100.0", "0.107.0"] {
        let matched = claim.classify(binding(version).version()).unwrap();
        assert_eq!(
            matched.behavior_revision().as_str(),
            CODEX_APP_SERVER_LEGACY_EXPLICIT_BEHAVIOR
        );
        assert_eq!(matched.support_status(), InterfaceSupportStatus::Deprecated);
    }
    for version in ["0.110.0", "0.120.0", "0.130.0"] {
        assert_eq!(
            claim
                .classify(binding(version).version())
                .unwrap()
                .behavior_revision()
                .as_str(),
            CODEX_APP_SERVER_BASE_BEHAVIOR
        );
    }
    for version in [
        "0.131.0", "0.140.0", "0.144.6", "0.145.0", "0.146.0", "0.147.0", "0.148.0", "0.149.0",
        "0.149.1", "0.150.0", "0.150.1", "0.151.0", "0.152.0", "0.152.1", "0.153.0", "0.153.1",
        "0.153.2", "0.153.3", "0.153.4", "0.154.0", "0.155.0", "0.155.1",
    ] {
        assert_eq!(
            claim
                .classify(binding(version).version())
                .unwrap()
                .behavior_revision()
                .as_str(),
            CODEX_APP_SERVER_WORKSPACE_BEHAVIOR
        );
    }
    for version in ["0.156.0", "0.156.1"] {
        let matched = claim.classify(binding(version).version()).unwrap();
        assert_eq!(
            matched.behavior_revision().as_str(),
            CODEX_APP_SERVER_MANAGED_POLICY_BEHAVIOR
        );
        assert_eq!(matched.support_status(), InterfaceSupportStatus::Deprecated);
    }
    for version in ["0.157.0", "0.157.1"] {
        let matched = claim.classify(binding(version).version()).unwrap();
        assert_eq!(
            matched.behavior_revision().as_str(),
            CODEX_APP_SERVER_MANAGED_NETWORK_BEHAVIOR
        );
        assert_eq!(matched.support_status(), InterfaceSupportStatus::Deprecated);
    }
    let path_alias = claim.classify(binding("0.158.0").version()).unwrap();
    assert_eq!(
        path_alias.behavior_revision().as_str(),
        CODEX_APP_SERVER_PATH_ALIAS_BEHAVIOR
    );
    assert_eq!(
        path_alias.support_status(),
        InterfaceSupportStatus::Deprecated
    );
    for version in [
        "0.159.0", "0.159.1", "0.159.2", "0.159.3", "0.160.0", "0.160.1", "0.161.0",
    ] {
        let matched = claim.classify(binding(version).version()).unwrap();
        assert_eq!(
            matched.behavior_revision().as_str(),
            CODEX_APP_SERVER_PROTECTED_AWS_BEHAVIOR
        );
        assert_eq!(matched.support_status(), InterfaceSupportStatus::Maintained);
    }
    for version in [
        "0.79.0",
        "0.82.0",
        "0.83.0",
        "0.108.0",
        "0.109.0",
        "0.146.0-alpha.4",
        "0.149.2",
        "0.150.2",
        "0.151.1",
        "0.152.2",
        "0.154.1",
        "0.155.2",
    ] {
        assert!(!claim.supports(binding(version).version()));
    }
    let InterfaceCompatibilityAssessment::UnverifiedNewer(unverified) =
        claim.assess(binding("0.161.1").version())
    else {
        panic!("first newer stable above ceiling should be unverified-newer");
    };
    assert_eq!(unverified.latest_qualified().as_str(), "0.161.0");
    assert_eq!(
        unverified.behavior_revision().as_str(),
        CODEX_APP_SERVER_PROTECTED_AWS_BEHAVIOR
    );
}

#[test]
fn app_server_lifecycle_claim_preserves_session_range_with_narrower_capabilities() {
    let claim = codex_app_server_lifecycle_claim();
    let cases = [
        (
            "0.80.0",
            ARCHIVE_BEHAVIOR,
            InterfaceSupportStatus::Deprecated,
        ),
        (
            "0.91.0",
            ARCHIVE_BEHAVIOR,
            InterfaceSupportStatus::Deprecated,
        ),
        (
            "0.92.0",
            RESTORE_BEHAVIOR,
            InterfaceSupportStatus::Deprecated,
        ),
        (
            "0.104.0",
            NOTIFICATIONS_BEHAVIOR,
            InterfaceSupportStatus::Deprecated,
        ),
        (
            "0.110.0",
            NOTIFICATIONS_BEHAVIOR,
            InterfaceSupportStatus::Maintained,
        ),
        (
            "0.123.0",
            DESCENDANT_ARCHIVE_BEHAVIOR,
            InterfaceSupportStatus::Maintained,
        ),
        (
            "0.140.0",
            HARD_DELETE_BEHAVIOR,
            InterfaceSupportStatus::Maintained,
        ),
        (
            "0.145.0",
            HARD_DELETE_BEHAVIOR,
            InterfaceSupportStatus::Maintained,
        ),
        (
            "0.146.0",
            HARD_DELETE_BEHAVIOR,
            InterfaceSupportStatus::Maintained,
        ),
        (
            "0.147.0",
            HARD_DELETE_BEHAVIOR,
            InterfaceSupportStatus::Maintained,
        ),
        (
            "0.148.0",
            HARD_DELETE_BEHAVIOR,
            InterfaceSupportStatus::Maintained,
        ),
        (
            "0.149.0",
            HARD_DELETE_BEHAVIOR,
            InterfaceSupportStatus::Maintained,
        ),
        (
            "0.149.1",
            HARD_DELETE_BEHAVIOR,
            InterfaceSupportStatus::Maintained,
        ),
        (
            "0.150.0",
            HARD_DELETE_BEHAVIOR,
            InterfaceSupportStatus::Maintained,
        ),
        (
            "0.150.1",
            HARD_DELETE_BEHAVIOR,
            InterfaceSupportStatus::Maintained,
        ),
        (
            "0.151.0",
            HARD_DELETE_BEHAVIOR,
            InterfaceSupportStatus::Maintained,
        ),
        (
            "0.152.0",
            HARD_DELETE_BEHAVIOR,
            InterfaceSupportStatus::Maintained,
        ),
        (
            "0.153.0",
            HARD_DELETE_BEHAVIOR,
            InterfaceSupportStatus::Maintained,
        ),
        (
            "0.153.1",
            HARD_DELETE_BEHAVIOR,
            InterfaceSupportStatus::Maintained,
        ),
        (
            "0.153.2",
            HARD_DELETE_BEHAVIOR,
            InterfaceSupportStatus::Maintained,
        ),
        (
            "0.153.3",
            HARD_DELETE_BEHAVIOR,
            InterfaceSupportStatus::Maintained,
        ),
        (
            "0.153.4",
            HARD_DELETE_BEHAVIOR,
            InterfaceSupportStatus::Maintained,
        ),
        (
            "0.154.0",
            HARD_DELETE_BEHAVIOR,
            InterfaceSupportStatus::Maintained,
        ),
        (
            "0.155.0",
            HARD_DELETE_BEHAVIOR,
            InterfaceSupportStatus::Maintained,
        ),
        (
            "0.155.1",
            HARD_DELETE_BEHAVIOR,
            InterfaceSupportStatus::Maintained,
        ),
        (
            "0.161.0",
            HARD_DELETE_BEHAVIOR,
            InterfaceSupportStatus::Maintained,
        ),
    ];

    for (version, behavior, status) in cases {
        let matched = claim
            .classify(binding(version).version())
            .expect("qualified version has a lifecycle segment");
        assert_eq!(matched.behavior_revision().as_str(), behavior);
        assert_eq!(matched.support_status(), status);
    }

    for version in [
        "0.82.0", "0.83.0", "0.108.0", "0.109.0", "0.149.2", "0.150.2", "0.151.1", "0.152.2",
        "0.154.1", "0.155.2",
    ] {
        assert!(
            codex_app_server_claim().supports(binding(version).version())
                == claim.supports(binding(version).version())
        );
        assert!(!claim.supports(binding(version).version()));
    }

    let InterfaceCompatibilityAssessment::UnverifiedNewer(unverified) =
        claim.assess(binding("0.161.1").version())
    else {
        panic!("newer lifecycle version remains visible");
    };
    assert_eq!(unverified.latest_qualified().as_str(), "0.161.0");
    assert_eq!(
        unverified.behavior_revision().as_str(),
        HARD_DELETE_BEHAVIOR
    );
}

#[test]
fn blank_or_non_semantic_version_text_fails_closed_instead_of_panicking() {
    for version in ["", "   ", " \t ", "\n", "current", "v0.146.0", "0.146.0\n"] {
        assert_eq!(
            codex_cli_binding(version),
            None,
            "provider-observed text must never panic the binding helper"
        );
    }
    assert!(codex_cli_binding("0.146.0").is_some());
}
