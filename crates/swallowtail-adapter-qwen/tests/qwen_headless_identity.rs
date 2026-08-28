use serde_json::Value;
use swallowtail_adapter_qwen::{
    QWEN_CODE_AXIS, QWEN_CODE_BASELINE_VERSION, QWEN_CODE_LATEST_QUALIFIED_VERSION,
    qwen_code_binding, qwen_headless_claim,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const IDENTITY_0_21_13: &str = include_str!("fixtures/qwen-code-0.21.13/identity.json");
const PROTOCOL_0_21_13: &str = include_str!("fixtures/qwen-code-0.21.13/protocol.json");
const IDENTITY_0_21_14: &str = include_str!("fixtures/qwen-code-0.21.14/identity.json");
const PROTOCOL_0_21_14: &str = include_str!("fixtures/qwen-code-0.21.14/protocol.json");
const IDENTITY_0_21_15: &str = include_str!("fixtures/qwen-code-0.21.15/identity.json");
const PROTOCOL_0_21_15: &str = include_str!("fixtures/qwen-code-0.21.15/protocol.json");
const IDENTITY_0_22_1: &str = include_str!("fixtures/qwen-code-0.22.1/identity.json");
const PROTOCOL_0_22_1: &str = include_str!("fixtures/qwen-code-0.22.1/protocol.json");
const IDENTITY_0_22_2: &str = include_str!("fixtures/qwen-code-0.22.2/identity.json");
const PROTOCOL_0_22_2: &str = include_str!("fixtures/qwen-code-0.22.2/protocol.json");
const IDENTITY_0_22_3: &str = include_str!("fixtures/qwen-code-0.22.3/identity.json");
const PROTOCOL_0_22_3: &str = include_str!("fixtures/qwen-code-0.22.3/protocol.json");

#[test]
fn identity_and_claim_qualify_0_21_13_as_compatible_extension() {
    let identity: Value =
        serde_json::from_str(IDENTITY_0_21_13).expect("Qwen 0.21.13 identity corpus is valid JSON");
    let protocol: Value =
        serde_json::from_str(PROTOCOL_0_21_13).expect("Qwen 0.21.13 protocol corpus is valid JSON");

    assert_eq!(identity["axis"], QWEN_CODE_AXIS);
    assert_eq!(identity["npm_package"], "@qwen-code/qwen-code");
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(identity["host"]["version"], "0.21.2");
    assert_eq!(identity["official"]["version"], "0.21.13");
    assert_eq!(
        identity["official"]["npm_integrity"],
        "sha512-xXyOK166EEeTjHUh9BEdH4h7Afhz53k+jJAv5mgFxQYJbHf25oxif6WRk6jvYGwMxpEdL3vaoURP/QQiplN9lQ=="
    );
    assert_eq!(
        identity["official"]["github_commit"],
        "d959015974302fb60ebd99adb81a68c2f482eaa3"
    );
    assert!(is_sha256(
        identity["host"]["executable_sha256"]
            .as_str()
            .expect("host digest is text")
    ));
    assert_eq!(identity["unpublished_stable_0_20_2"], true);
    assert_eq!(identity["unpublished_0_21_14"], true);

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(
        decision["reuse_behavior_revision"],
        "qwen-code.headless.v0.21.0-catalogue-filter"
    );
    assert_eq!(decision["raise_latest_qualified_to"], "0.21.13");
    assert_eq!(decision["keep_baseline"], "0.19.11");
    assert_eq!(decision["qualify_intermediates"], true);
    assert_eq!(decision["new_milestone"], false);
    assert_eq!(decision["keep_0_20_2_incompatible"], true);
    assert_eq!(decision["map_goal_state"], false);
    assert_eq!(decision["provider_prompt_sent"], false);
    assert_eq!(decision["live_catalogue"], false);
    assert_eq!(decision["live_headless_session"], false);
    assert_eq!(decision["host_install_changed"], false);

    let flags = protocol["help_selected_flags_present"]
        .as_array()
        .expect("selected flags are an array");
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
        assert!(
            flags.iter().any(|flag| flag == required),
            "missing selected flag {required}"
        );
    }
    assert_eq!(protocol["catalogue_image_only_filter"], true);
    assert_eq!(protocol["stream_types_add_goal_state_from"], "0.21.4");
    assert_eq!(protocol["decoder_corpus"], "qwen-code-v0.19.11");
    assert_eq!(protocol["provider_prompt_sent"], false);

    assert_eq!(QWEN_CODE_BASELINE_VERSION, "0.19.11");
    assert_eq!(QWEN_CODE_LATEST_QUALIFIED_VERSION, "0.22.3");
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "0.21.2"
    );

    let claim = qwen_headless_claim();
    assert!(matches!(
        claim.assess(&version("0.21.2")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Deprecated
                && matched.behavior_revision().as_str()
                    == "qwen-code.headless.v0.21.0-catalogue-filter"
    ));
    for candidate in [
        "0.21.3", "0.21.4", "0.21.5", "0.21.6", "0.21.7", "0.21.8", "0.21.9", "0.21.10", "0.21.11",
        "0.21.12", "0.21.13", "0.21.14",
    ] {
        assert!(matches!(
            claim.assess(&version(candidate)),
            InterfaceCompatibilityAssessment::Qualified(matched)
                if matched.support_status() == InterfaceSupportStatus::Deprecated
                    && matched.behavior_revision().as_str()
                        == "qwen-code.headless.v0.21.0-catalogue-filter"
        ));
    }
    assert!(matches!(
        claim.assess(&version("0.21.15")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Maintained
                && matched.behavior_revision().as_str()
                    == "qwen-code.headless.v0.21.15-reasoning-control"
    ));
    assert!(!claim.permits(&version("0.20.2")));
    assert!(!claim.permits(&version("0.21.16")));
    assert_eq!(
        qwen_code_binding("0.21.13")
            .expect("version binds")
            .axis()
            .as_str(),
        QWEN_CODE_AXIS
    );
}

#[test]
fn identity_and_claim_qualify_0_21_14_as_compatible_extension() {
    let identity: Value =
        serde_json::from_str(IDENTITY_0_21_14).expect("Qwen 0.21.14 identity corpus is valid JSON");
    let protocol: Value =
        serde_json::from_str(PROTOCOL_0_21_14).expect("Qwen 0.21.14 protocol corpus is valid JSON");

    assert_eq!(identity["axis"], QWEN_CODE_AXIS);
    assert_eq!(identity["npm_package"], "@qwen-code/qwen-code");
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(identity["host"]["version"], "0.21.2");
    assert_eq!(identity["official"]["version"], "0.21.14");
    assert_eq!(
        identity["official"]["npm_integrity"],
        "sha512-+sheZkLj6K34SKN5r6lZ0yQBmJrLNWyzflUmG5UNk3Ycdha643Dr1T3tv5PI3HANNoUiBVMEjTqQzU0hHCe5kw=="
    );
    assert_eq!(
        identity["official"]["github_commit"],
        "6e20a58923b0a00baafa5a7221ff63054ad1af63"
    );
    assert_eq!(identity["selected_blobs_unchanged_from_0_21_13"], true);
    assert_eq!(identity["unpublished_stable_0_20_2"], true);
    assert_eq!(identity["unpublished_0_21_15"], true);
    assert_eq!(identity["ignored_preview"], "0.21.14-preview.0");

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(
        decision["reuse_behavior_revision"],
        "qwen-code.headless.v0.21.0-catalogue-filter"
    );
    assert_eq!(decision["raise_latest_qualified_to"], "0.21.14");
    assert_eq!(decision["keep_baseline"], "0.19.11");
    assert_eq!(decision["new_milestone"], false);
    assert_eq!(decision["map_sessions_ps"], false);
    assert_eq!(decision["provider_prompt_sent"], false);
    assert_eq!(decision["host_install_changed"], false);
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "0.21.13"
    );

    assert_eq!(
        protocol["selected_source_blobs_byte_identical_to_0_21_13"],
        true
    );
    assert_eq!(protocol["decoder_corpus"], "qwen-code-v0.19.11");
    let unused = protocol["unused_deltas"]
        .as_array()
        .expect("unused deltas are an array");
    for extra in ["qwen sessions ps", "/advisor", "live-session-registry"] {
        assert!(
            unused.iter().any(|value| value == extra),
            "missing unused delta {extra}"
        );
    }

    assert_eq!(QWEN_CODE_LATEST_QUALIFIED_VERSION, "0.22.3");
    let claim = qwen_headless_claim();
    assert!(matches!(
        claim.assess(&version("0.21.13")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Deprecated
    ));
    assert!(matches!(
        claim.assess(&version("0.21.14")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Deprecated
                && matched.behavior_revision().as_str()
                    == "qwen-code.headless.v0.21.0-catalogue-filter"
    ));
    assert!(matches!(
        claim.assess(&version("0.21.15")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Maintained
                && matched.behavior_revision().as_str()
                    == "qwen-code.headless.v0.21.15-reasoning-control"
    ));
    assert!(!claim.permits(&version("0.21.16")));
    assert_eq!(
        qwen_code_binding("0.21.14")
            .expect("version binds")
            .axis()
            .as_str(),
        QWEN_CODE_AXIS
    );
}

#[test]
fn identity_and_claim_qualify_0_21_15_as_compatible_extension() {
    let identity: Value =
        serde_json::from_str(IDENTITY_0_21_15).expect("Qwen 0.21.15 identity corpus is valid JSON");
    let protocol: Value =
        serde_json::from_str(PROTOCOL_0_21_15).expect("Qwen 0.21.15 protocol corpus is valid JSON");

    assert_eq!(identity["axis"], QWEN_CODE_AXIS);
    assert_eq!(identity["npm_package"], "@qwen-code/qwen-code");
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(identity["host"]["not_installed"], true);
    assert_eq!(identity["official"]["version"], "0.21.15");
    assert_eq!(
        identity["official"]["npm_integrity"],
        "sha512-f4ER/SRVLpwhcqzuytK3Qeq8bG9HnVhv7f7wsf3cpE/AkRfzKSvaeURnW7s7zI3nWkEqA7DM6njSLYS2s6DWDg=="
    );
    assert_eq!(
        identity["official"]["github_commit"],
        "5dce2515a778f9cf2013168962b4fbc3454636e3"
    );
    assert_eq!(
        identity["selected_types_and_catalogue_unchanged_from_0_21_14"],
        true
    );
    assert_eq!(identity["selected_config_blob_changed_from_0_21_14"], true);
    assert_eq!(identity["selected_mapped_subset_unchanged"], false);
    assert_eq!(identity["unpublished_stable_0_20_2"], true);
    assert_eq!(identity["unpublished_0_21_16"], true);
    assert_eq!(identity["ignored_preview"], "0.21.14-preview.0");

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(
        decision["reuse_behavior_revision"],
        "qwen-code.headless.v0.21.0-catalogue-filter"
    );
    assert_eq!(decision["raise_latest_qualified_to"], "0.21.15");
    assert_eq!(decision["keep_baseline"], "0.19.11");
    assert_eq!(decision["new_milestone"], true);
    assert_eq!(decision["reasoning_control_exact_version"], "0.21.15");
    assert_eq!(
        decision["reasoning_behavior_revision"],
        "qwen-code.headless.v0.21.15-reasoning-control"
    );
    assert_eq!(decision["map_session_id_casefold"], false);
    assert_eq!(decision["map_review_resume"], false);
    assert_eq!(decision["map_web_shell_goal_v3"], false);
    assert_eq!(decision["provider_prompt_sent"], false);
    assert_eq!(decision["host_install_changed"], false);
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "0.21.15"
    );

    assert_eq!(
        protocol["selected_types_and_catalogue_byte_identical_to_0_21_14"],
        true
    );
    assert_eq!(protocol["selected_mapped_subset_unchanged"], false);
    assert_eq!(protocol["decoder_corpus"], "qwen-code-v0.19.11");
    let unused = protocol["unused_deltas"]
        .as_array()
        .expect("unused deltas are an array");
    for extra in [
        "--session-id casefold occupancy",
        "/review --resume",
        "web-shell Goal v3",
        "standalone conversation isolation",
        "hybrid-model thinking toggle",
    ] {
        assert!(
            unused.iter().any(|value| value == extra),
            "missing unused delta {extra}"
        );
    }

    assert_eq!(QWEN_CODE_LATEST_QUALIFIED_VERSION, "0.22.3");
    let claim = qwen_headless_claim();
    assert!(matches!(
        claim.assess(&version("0.21.14")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Deprecated
                && matched.behavior_revision().as_str()
                    == "qwen-code.headless.v0.21.0-catalogue-filter"
    ));
    assert!(matches!(
        claim.assess(&version("0.21.15")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Maintained
                && matched.behavior_revision().as_str()
                    == "qwen-code.headless.v0.21.15-reasoning-control"
    ));
    assert!(!claim.permits(&version("0.21.16")));
    assert_eq!(
        qwen_code_binding("0.21.15")
            .expect("version binds")
            .axis()
            .as_str(),
        QWEN_CODE_AXIS
    );
}

#[test]
fn identity_and_claim_qualify_0_22_1_as_compatible_extension() {
    let identity: Value =
        serde_json::from_str(IDENTITY_0_22_1).expect("Qwen 0.22.1 identity corpus is valid JSON");
    let protocol: Value =
        serde_json::from_str(PROTOCOL_0_22_1).expect("Qwen 0.22.1 protocol corpus is valid JSON");

    assert_eq!(identity["axis"], QWEN_CODE_AXIS);
    assert_eq!(identity["npm_package"], "@qwen-code/qwen-code");
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(identity["host"]["not_installed"], true);
    assert_eq!(identity["official"]["version"], "0.22.1");
    assert_eq!(
        identity["official"]["npm_integrity"],
        "sha512-sDki8GaxUA7eEbo1SQNd15TXiP22CMmOpUmfKeDvl+vmyw5sMwX5XJunQ8R4zReRV8z+HIaqqK5u28UX807lhw=="
    );
    assert_eq!(
        identity["official"]["github_commit"],
        "2755dbe1399f94e53e24377d2e21fa86ce923529"
    );
    assert_eq!(
        identity["published_intermediate_0_22_0"]["version"],
        "0.22.0"
    );
    assert_eq!(identity["unpublished_stable_0_20_2"], true);
    assert_eq!(identity["unpublished_0_21_16"], true);
    assert_eq!(identity["unpublished_0_22_2"], true);
    assert_eq!(identity["ignored_preview"], "0.22.2-preview.1");
    assert_eq!(identity["selected_mapped_subset_unchanged"], true);
    assert_eq!(identity["cli_entry_identical_0_21_15_through_0_22_1"], true);
    assert_eq!(
        identity["reasoning_effort_identical_0_21_15_through_0_22_1"],
        true
    );

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(
        decision["reuse_behavior_revision"],
        "qwen-code.headless.v0.21.15-reasoning-control"
    );
    assert_eq!(decision["raise_latest_qualified_to"], "0.22.1");
    assert_eq!(decision["keep_baseline"], "0.19.11");
    assert_eq!(decision["add_same_revision_segment"], "0.22.0..=0.22.1");
    assert_eq!(decision["new_milestone"], false);
    assert_eq!(decision["keep_0_21_16_incompatible"], true);
    assert_eq!(decision["later_unverified_after_qualification"], "0.22.2");
    assert_eq!(decision["extend_reasoning_beyond_0_21_15"], false);
    assert_eq!(decision["extend_budgets_beyond_0_21_15"], false);
    assert_eq!(decision["map_dashscope_max_clamp"], false);
    assert_eq!(decision["provider_prompt_sent"], false);
    assert_eq!(decision["host_install_changed"], false);
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "0.21.15"
    );

    assert_eq!(protocol["selected_mapped_subset_unchanged"], true);
    assert_eq!(
        protocol["0_22_0_types_and_controller_byte_identical_to_0_21_15"],
        true
    );
    assert_eq!(protocol["decoder_corpus"], "qwen-code-v0.19.11");
    let unused = protocol["unused_deltas"]
        .as_array()
        .expect("unused deltas are an array");
    for extra in [
        "--restore-ask-user-question",
        "MCP versionNegotiation",
        "dashscope clamp max to xhigh",
        "list_directory settings opt-in default-off",
    ] {
        assert!(
            unused.iter().any(|value| value == extra),
            "missing unused delta {extra}"
        );
    }

    assert_eq!(QWEN_CODE_LATEST_QUALIFIED_VERSION, "0.22.3");
    let claim = qwen_headless_claim();
    assert!(matches!(
        claim.assess(&version("0.21.15")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Maintained
                && matched.behavior_revision().as_str()
                    == "qwen-code.headless.v0.21.15-reasoning-control"
    ));
    for candidate in ["0.22.0", "0.22.1", "0.22.2", "0.22.3"] {
        assert!(matches!(
            claim.assess(&version(candidate)),
            InterfaceCompatibilityAssessment::Qualified(matched)
                if matched.support_status() == InterfaceSupportStatus::Maintained
                    && matched.behavior_revision().as_str()
                        == "qwen-code.headless.v0.21.15-reasoning-control"
        ));
    }
    assert!(!claim.permits(&version("0.21.16")));
    assert!(matches!(
        claim.assess(&version("0.22.4")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert_eq!(
        qwen_code_binding("0.22.1")
            .expect("version binds")
            .axis()
            .as_str(),
        QWEN_CODE_AXIS
    );
}

#[test]
fn identity_and_claim_qualify_0_22_2_as_compatible_extension() {
    let identity: Value =
        serde_json::from_str(IDENTITY_0_22_2).expect("Qwen 0.22.2 identity corpus is valid JSON");
    let protocol: Value =
        serde_json::from_str(PROTOCOL_0_22_2).expect("Qwen 0.22.2 protocol corpus is valid JSON");

    assert_eq!(identity["axis"], QWEN_CODE_AXIS);
    assert_eq!(identity["npm_package"], "@qwen-code/qwen-code");
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(identity["host"]["not_installed"], true);
    assert_eq!(identity["official"]["version"], "0.22.2");
    assert_eq!(
        identity["official"]["npm_integrity"],
        "sha512-u1qzaUVDIn9GTEroUtOy5mVaGYo3eGh0P4A840Aac8PkZu9ob4wMnQFgTVVD7rtjj9BY6WLFuMqX3uP/zXw9Uw=="
    );
    assert_eq!(
        identity["official"]["github_commit"],
        "0d573e45275fdc800ebc6b458fd019ccc6e7b7bf"
    );
    assert_eq!(identity["official"]["github_tag"], "v0.22.2");
    assert_eq!(identity["unpublished_stable_0_20_2"], true);
    assert_eq!(identity["unpublished_0_21_16"], true);
    assert_eq!(identity["unpublished_0_22_3"], true);
    assert_eq!(identity["ignored_preview"], "0.22.2-preview.1");
    assert_eq!(identity["selected_mapped_subset_unchanged"], true);
    assert_eq!(identity["cli_entry_identical_0_21_15_through_0_22_2"], true);
    assert_eq!(
        identity["reasoning_effort_identical_0_21_15_through_0_22_2"],
        true
    );

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(
        decision["reuse_behavior_revision"],
        "qwen-code.headless.v0.21.15-reasoning-control"
    );
    assert_eq!(decision["raise_latest_qualified_to"], "0.22.2");
    assert_eq!(decision["keep_baseline"], "0.19.11");
    assert_eq!(decision["extend_same_revision_segment"], "0.22.0..=0.22.2");
    assert_eq!(decision["new_milestone"], false);
    assert_eq!(decision["keep_0_21_16_incompatible"], true);
    assert_eq!(decision["later_unverified_after_qualification"], "0.22.3");
    assert_eq!(decision["extend_plan_exact_list_to_include_0_22_2"], true);
    assert_eq!(decision["extend_reasoning_beyond_0_21_15"], false);
    assert_eq!(decision["extend_budgets_beyond_0_21_15"], false);
    assert_eq!(decision["map_provisional_workspace"], false);
    assert_eq!(decision["provider_prompt_sent"], false);
    assert_eq!(decision["host_install_changed"], false);
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "0.22.1"
    );

    assert_eq!(protocol["selected_mapped_subset_unchanged"], true);
    assert_eq!(
        protocol["0_22_2_types_controller_session_reasoning_dashscope_byte_identical_to_0_22_1"],
        true
    );
    assert_eq!(
        protocol["0_22_2_plan_mode_blobs_byte_identical_to_0_22_1"],
        true
    );
    assert_eq!(protocol["decoder_corpus"], "qwen-code-v0.19.11");
    let unused = protocol["unused_deltas"]
        .as_array()
        .expect("unused deltas are an array");
    for extra in [
        "hostPolicy.provisionalWorkspace",
        "node-repl standalone MCP server",
        "--restore-ask-user-question",
        "MCP versionNegotiation",
    ] {
        assert!(
            unused.iter().any(|value| value == extra),
            "missing unused delta {extra}"
        );
    }

    assert_eq!(QWEN_CODE_LATEST_QUALIFIED_VERSION, "0.22.3");
    let claim = qwen_headless_claim();
    assert!(matches!(
        claim.assess(&version("0.21.15")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Maintained
                && matched.behavior_revision().as_str()
                    == "qwen-code.headless.v0.21.15-reasoning-control"
    ));
    for candidate in ["0.22.0", "0.22.1", "0.22.2", "0.22.3"] {
        assert!(matches!(
            claim.assess(&version(candidate)),
            InterfaceCompatibilityAssessment::Qualified(matched)
                if matched.support_status() == InterfaceSupportStatus::Maintained
                    && matched.behavior_revision().as_str()
                        == "qwen-code.headless.v0.21.15-reasoning-control"
        ));
    }
    assert!(!claim.permits(&version("0.21.16")));
    assert!(matches!(
        claim.assess(&version("0.22.4")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert_eq!(
        qwen_code_binding("0.22.2")
            .expect("version binds")
            .axis()
            .as_str(),
        QWEN_CODE_AXIS
    );
}

#[test]
fn identity_and_claim_qualify_0_22_3_as_compatible_extension() {
    let identity: Value =
        serde_json::from_str(IDENTITY_0_22_3).expect("Qwen 0.22.3 identity corpus is valid JSON");
    let protocol: Value =
        serde_json::from_str(PROTOCOL_0_22_3).expect("Qwen 0.22.3 protocol corpus is valid JSON");

    assert_eq!(identity["axis"], QWEN_CODE_AXIS);
    assert_eq!(identity["npm_package"], "@qwen-code/qwen-code");
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(identity["host"]["not_installed"], true);
    assert_eq!(identity["official"]["version"], "0.22.3");
    assert_eq!(
        identity["official"]["npm_integrity"],
        "sha512-8Ngy/ZEn+idOyN3k52K9TNu/XSkNfS2hyzsikeSDe79kRd2/eMYbWLOZq6LHSGVYXVNpY6ktpfZLthxY5AHWeA=="
    );
    assert_eq!(
        identity["official"]["github_commit"],
        "09825973e7d3c3fd07e17909c396aa62f48ce51f"
    );
    assert_eq!(identity["official"]["github_tag"], "v0.22.3");
    assert_eq!(
        identity["published_intermediate_0_22_2"]["version"],
        "0.22.2"
    );
    assert_eq!(identity["unpublished_stable_0_20_2"], true);
    assert_eq!(identity["unpublished_0_21_16"], true);
    assert_eq!(identity["unpublished_0_22_4"], true);
    assert_eq!(identity["ignored_preview"], "0.22.2-preview.1");
    assert_eq!(identity["selected_mapped_subset_unchanged"], true);
    assert_eq!(identity["cli_entry_identical_0_21_15_through_0_22_3"], true);
    assert_eq!(
        identity["reasoning_effort_identical_0_21_15_through_0_22_3"],
        true
    );

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(
        decision["reuse_behavior_revision"],
        "qwen-code.headless.v0.21.15-reasoning-control"
    );
    assert_eq!(decision["raise_latest_qualified_to"], "0.22.3");
    assert_eq!(decision["keep_baseline"], "0.19.11");
    assert_eq!(decision["extend_same_revision_segment"], "0.22.0..=0.22.3");
    assert_eq!(decision["new_milestone"], false);
    assert_eq!(decision["keep_0_21_16_incompatible"], true);
    assert_eq!(decision["later_unverified_after_qualification"], "0.22.4");
    assert_eq!(
        decision["extend_plan_exact_list_to_include_0_22_2_and_0_22_3"],
        true
    );
    assert_eq!(decision["extend_reasoning_beyond_0_21_15"], false);
    assert_eq!(decision["extend_budgets_beyond_0_21_15"], false);
    assert_eq!(decision["map_tools_eager"], false);
    assert_eq!(decision["map_memory_rename"], false);
    assert_eq!(decision["map_session_comment_rename"], false);
    assert_eq!(decision["provider_prompt_sent"], false);
    assert_eq!(decision["host_install_changed"], false);
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "0.22.1"
    );

    assert_eq!(protocol["selected_mapped_subset_unchanged"], true);
    assert_eq!(
        protocol["0_22_3_types_controller_reasoning_dashscope_byte_identical_to_0_22_2"],
        true
    );
    assert_eq!(
        protocol["0_22_3_plan_mode_blobs_byte_identical_to_0_22_2"],
        true
    );
    assert_eq!(protocol["decoder_corpus"], "qwen-code-v0.19.11");
    let unused = protocol["unused_deltas"]
        .as_array()
        .expect("unused deltas are an array");
    for extra in [
        "tools.eager",
        "GeminiMd to Memory rename",
        "session comment rename gemini.tsx to llm.tsx",
        "hostPolicy.provisionalWorkspace",
        "Channels named sessions",
        "daemon session APIs",
    ] {
        assert!(
            unused.iter().any(|value| value == extra),
            "missing unused delta {extra}"
        );
    }

    assert_eq!(QWEN_CODE_LATEST_QUALIFIED_VERSION, "0.22.3");
    let claim = qwen_headless_claim();
    assert!(matches!(
        claim.assess(&version("0.21.15")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Maintained
                && matched.behavior_revision().as_str()
                    == "qwen-code.headless.v0.21.15-reasoning-control"
    ));
    for candidate in ["0.22.0", "0.22.1", "0.22.2", "0.22.3"] {
        assert!(matches!(
            claim.assess(&version(candidate)),
            InterfaceCompatibilityAssessment::Qualified(matched)
                if matched.support_status() == InterfaceSupportStatus::Maintained
                    && matched.behavior_revision().as_str()
                        == "qwen-code.headless.v0.21.15-reasoning-control"
        ));
    }
    assert!(!claim.permits(&version("0.21.16")));
    assert!(matches!(
        claim.assess(&version("0.22.4")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert_eq!(
        qwen_code_binding("0.22.3")
            .expect("version binds")
            .axis()
            .as_str(),
        QWEN_CODE_AXIS
    );
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}
