use super::support::{IDENTITY, assert_exact_string_set, json, version};
use swallowtail_adapter_codex::{codex_app_server_claim, codex_exec_claim};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceSupportStatus};

#[test]
fn production_exec_claim_preserves_prior_points_and_qualifies_current_stable() {
    let exec = codex_exec_claim();
    for point in [
        "0.150.1", "0.152.0", "0.152.1", "0.153.0", "0.153.1", "0.153.2", "0.153.3", "0.153.4",
        "0.154.0", "0.156.0", "0.156.1", "0.157.0", "0.157.1", "0.158.0", "0.159.0", "0.159.1",
        "0.159.2", "0.159.3", "0.160.0", "0.160.1", "0.161.0",
    ] {
        let qualified = exec.assess(&version(point));
        let InterfaceCompatibilityAssessment::Qualified(matched) = qualified else {
            panic!("{point} must be qualified after the claim card");
        };
        assert_eq!(matched.behavior_revision().as_str(), "codex.exec.jsonl-v1");
        assert_eq!(matched.support_status(), InterfaceSupportStatus::Maintained);
    }
    assert_eq!(
        exec.assess(&version("0.155.2")),
        InterfaceCompatibilityAssessment::Incompatible,
        "unpublished 0.155.2 remains a gap"
    );
    let InterfaceCompatibilityAssessment::UnverifiedNewer(unverified) =
        exec.assess(&version("0.161.1"))
    else {
        panic!("0.161.1 remains visible as UnverifiedNewer");
    };
    assert_eq!(unverified.latest_qualified().as_str(), "0.161.0");
    for gap in [
        "0.149.2", "0.150.2", "0.151.1", "0.152.2", "0.154.1", "0.108.0", "0.109.0",
    ] {
        assert_eq!(
            exec.assess(&version(gap)),
            InterfaceCompatibilityAssessment::Incompatible,
            "{gap} must stay incompatible after the claim card"
        );
    }
}

#[test]
fn production_app_server_claim_preserves_previously_qualified_points() {
    let app_server = codex_app_server_claim();
    for point in [
        "0.150.1", "0.152.0", "0.152.1", "0.153.0", "0.153.1", "0.153.2", "0.153.3", "0.153.4",
        "0.154.0",
    ] {
        let qualified = app_server.assess(&version(point));
        let InterfaceCompatibilityAssessment::Qualified(matched) = qualified else {
            panic!("{point} must remain qualified");
        };
        assert_eq!(
            matched.behavior_revision().as_str(),
            "codex.app-server.v2.workspace-roots"
        );
        assert_eq!(matched.support_status(), InterfaceSupportStatus::Maintained);
    }
    assert_eq!(
        app_server.assess(&version("0.155.2")),
        InterfaceCompatibilityAssessment::Incompatible,
        "unpublished 0.155.2 remains a gap for app-server"
    );
    for gap in ["0.149.2", "0.150.2", "0.151.1", "0.152.2", "0.154.1"] {
        assert_eq!(
            app_server.assess(&version(gap)),
            InterfaceCompatibilityAssessment::Incompatible,
            "{gap} must stay incompatible"
        );
    }
}

#[test]
fn identity_decision_names_compatible_extension_without_raising_the_claim() {
    let decision = &json(IDENTITY)["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(decision["exec_behavior"], "codex.exec.jsonl-v1");
    assert_eq!(
        decision["app_server_behavior"],
        "codex.app-server.v2.workspace-roots"
    );
    assert_eq!(
        decision["lifecycle_behavior"],
        "codex.app-server.lifecycle.v1.strict-descendant-hard-delete"
    );
    assert_eq!(decision["raise_latest_qualified_to"], "0.152.1");
    assert_eq!(decision["keep_baseline"], true);
    assert_eq!(
        decision["qualify_intermediates"],
        serde_json::json!(["0.152.1"])
    );
    assert_eq!(decision["new_milestone"], false);
    assert_eq!(decision["later_unverified_after_qualification"], "0.152.2");
    assert_eq!(
        decision["thread_resume_exclude_turns"],
        "already-selected-mapped"
    );
    assert_eq!(
        decision["thread_resume_params_byte_identical_to_0_152_0"],
        true
    );
    for unmapped in [
        "map_exec_fork",
        "map_thread_fork",
        "map_top_level_fork",
        "map_thread_source",
        "map_thread_turns_list",
        "map_thread_items_list",
        "map_thread_shell_command",
        "map_code_mode_host",
        "map_auth_recovery_notifications",
        "map_guardian_auto_review_node_repl_policy",
        "widen_feature_exact_pins",
    ] {
        assert_eq!(decision[unmapped], false, "{unmapped} must stay false");
    }
    assert_exact_string_set(
        &decision["keep_gaps"],
        &[
            "0.82.0..=0.83.0",
            "0.108.0",
            "0.109.0",
            "0.149.2",
            "0.150.2",
            "0.151.1",
        ],
    );
}
