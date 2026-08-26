use serde_json::Value;
use swallowtail_adapter_oh_my_pi::{
    OH_MY_PI_PACKAGE_AXIS, OH_MY_PI_PACKAGE_BASELINE_VERSION,
    OH_MY_PI_PACKAGE_LATEST_QUALIFIED_VERSION, oh_my_pi_package_binding, oh_my_pi_rpc_claim,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const IDENTITY: &str = include_str!("fixtures/oh-my-pi-17.3.7/identity.json");
const PROTOCOL: &str = include_str!("fixtures/oh-my-pi-17.3.7/protocol.json");

#[test]
fn identity_and_claim_qualify_17_3_7_as_compatible_extension() {
    let identity: Value =
        serde_json::from_str(IDENTITY).expect("Oh My Pi 17.3.7 identity corpus is valid JSON");
    let protocol: Value =
        serde_json::from_str(PROTOCOL).expect("Oh My Pi 17.3.7 protocol corpus is valid JSON");

    assert_eq!(identity["axis"], OH_MY_PI_PACKAGE_AXIS);
    assert_eq!(identity["version"], "17.3.7");
    assert_eq!(identity["npm_package"], "@oh-my-pi/pi-coding-agent");
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(
        identity["npm_integrity"],
        "sha512-z2W77ThFqtKP9P+wqISCtjMZFUZBNbR3jddZ0odgpBPRzNeORpcVVWbyhVGLsqXRWB3YQP2vYtxy+ohsnhG1+A=="
    );
    assert_eq!(identity["local_cli"], "omp/17.2.15");
    assert!(is_sha256(
        identity["local_executable_sha256"]
            .as_str()
            .expect("host executable digest is text")
    ));
    assert!(is_sha256(
        identity["extracted_cli_sha256"]
            .as_str()
            .expect("extracted 17.3.7 digest is text")
    ));
    assert_eq!(identity["npm_17_3_6_published"], false);

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(
        decision["reuse_behavior_revision"],
        "oh-my-pi.rpc-v2-v17.2.9"
    );
    assert_eq!(decision["raise_latest_qualified_to"], "17.3.7");
    assert_eq!(decision["keep_baseline"], "17.2.9");
    assert_eq!(decision["qualify_intermediates"], true);
    assert_eq!(decision["new_milestone"], false);
    assert_eq!(decision["mix_pi_package_axis"], false);
    assert_eq!(decision["provider_prompt_sent"], false);
    assert_eq!(decision["host_install_changed"], false);

    let flags = protocol["help_selected_flags_present"]
        .as_array()
        .expect("selected flags are an array");
    for required in [
        "--mode",
        "--no-session",
        "--provider",
        "--model",
        "--tools",
        "--no-extensions",
        "--no-skills",
        "--no-rules",
        "--no-prewalk",
        "--approval-mode",
        "--no-tools",
    ] {
        assert!(
            flags.iter().any(|flag| flag == required),
            "missing selected flag {required}"
        );
    }
    assert_eq!(protocol["selected_mode"], "rpc");
    assert_eq!(protocol["selected_approval_mode"], "always-ask");
    assert_eq!(protocol["decoder_corpus"], "oh-my-pi-rpc-17.2.9");
    assert_eq!(protocol["provider_prompt_sent"], false);

    assert_eq!(OH_MY_PI_PACKAGE_BASELINE_VERSION, "17.2.9");
    assert_eq!(OH_MY_PI_PACKAGE_LATEST_QUALIFIED_VERSION, "17.4.0");
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "17.2.9"
    );

    let claim = oh_my_pi_rpc_claim();
    assert!(claim.supports(&version("17.2.9")));
    assert!(claim.supports(&version("17.2.15")));
    assert!(claim.supports(&version("17.3.0")));
    assert!(claim.supports(&version("17.3.5")));
    assert!(matches!(
        claim.assess(&version("17.3.7")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Maintained
    ));
    assert_eq!(
        oh_my_pi_package_binding("17.3.7")
            .expect("version binds")
            .axis()
            .as_str(),
        OH_MY_PI_PACKAGE_AXIS
    );
}

const IDENTITY_0_17_3_8: &str = include_str!("fixtures/oh-my-pi-17.3.8/identity.json");
const PROTOCOL_0_17_3_8: &str = include_str!("fixtures/oh-my-pi-17.3.8/protocol.json");

#[test]
fn identity_and_claim_qualify_17_3_8_as_compatible_extension() {
    let identity: Value = serde_json::from_str(IDENTITY_0_17_3_8)
        .expect("Oh My Pi 17.3.8 identity corpus is valid JSON");
    let protocol: Value = serde_json::from_str(PROTOCOL_0_17_3_8)
        .expect("Oh My Pi 17.3.8 protocol corpus is valid JSON");

    assert_eq!(identity["axis"], OH_MY_PI_PACKAGE_AXIS);
    assert_eq!(identity["version"], "17.3.8");
    assert_eq!(identity["npm_package"], "@oh-my-pi/pi-coding-agent");
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(
        identity["npm_integrity"],
        "sha512-0Qc25+97SREzKJcSYMw434/kZKFFKxWW9WZV5i9S3m+SNtQ6K1tigHBMLM9PhUM7fr2grpyGXS3asnd7owTq6Q=="
    );
    assert_eq!(identity["local_cli"], "omp/17.2.15");
    assert!(is_sha256(
        identity["local_executable_sha256"]
            .as_str()
            .expect("host executable digest is text")
    ));
    assert!(is_sha256(
        identity["extracted_cli_sha256"]
            .as_str()
            .expect("extracted 17.3.8 digest is text")
    ));
    assert_eq!(identity["npm_17_3_6_published"], false);
    assert_eq!(identity["unpublished_17_3_9"], true);
    assert_eq!(
        identity["rpc_md_blob_sha_v17_3_7"],
        identity["rpc_md_blob_sha_v17_3_8"]
    );

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(
        decision["reuse_behavior_revision"],
        "oh-my-pi.rpc-v2-v17.2.9"
    );
    assert_eq!(decision["raise_latest_qualified_to"], "17.3.8");
    assert_eq!(decision["keep_baseline"], "17.2.9");
    assert_eq!(decision["qualify_intermediates"], true);
    assert_eq!(decision["new_milestone"], false);
    assert_eq!(decision["mix_pi_package_axis"], false);
    assert_eq!(decision["map_cache_retention_setting"], false);
    assert_eq!(decision["provider_prompt_sent"], false);
    assert_eq!(decision["host_install_changed"], false);

    let flags = protocol["help_selected_flags_present"]
        .as_array()
        .expect("selected flags are an array");
    for required in [
        "--mode",
        "--no-session",
        "--provider",
        "--model",
        "--tools",
        "--no-extensions",
        "--no-skills",
        "--no-rules",
        "--no-prewalk",
        "--approval-mode",
        "--no-tools",
    ] {
        assert!(
            flags.iter().any(|flag| flag == required),
            "missing selected flag {required}"
        );
    }
    assert_eq!(protocol["selected_mode"], "rpc");
    assert_eq!(protocol["selected_approval_mode"], "always-ask");
    assert_eq!(protocol["rpc_md_unchanged_from_v17_3_7"], true);
    assert_eq!(protocol["decoder_corpus"], "oh-my-pi-rpc-17.2.9");
    assert_eq!(protocol["provider_prompt_sent"], false);

    assert_eq!(OH_MY_PI_PACKAGE_LATEST_QUALIFIED_VERSION, "17.4.0");
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "17.3.7"
    );

    let claim = oh_my_pi_rpc_claim();
    for value in ["17.2.15", "17.3.7", "17.3.8", "17.4.0"] {
        assert!(matches!(
            claim.assess(&version(value)),
            InterfaceCompatibilityAssessment::Qualified(matched)
                if matched.support_status() == InterfaceSupportStatus::Maintained
        ));
    }
    assert!(matches!(
        claim.assess(&version("17.4.1")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert_eq!(
        oh_my_pi_package_binding("17.3.8")
            .expect("version binds")
            .axis()
            .as_str(),
        OH_MY_PI_PACKAGE_AXIS
    );
}

const IDENTITY_0_17_4_0: &str = include_str!("fixtures/oh-my-pi-17.4.0/identity.json");
const PROTOCOL_0_17_4_0: &str = include_str!("fixtures/oh-my-pi-17.4.0/protocol.json");

#[test]
fn identity_and_claim_qualify_17_4_0_as_compatible_extension() {
    let identity: Value = serde_json::from_str(IDENTITY_0_17_4_0)
        .expect("Oh My Pi 17.4.0 identity corpus is valid JSON");
    let protocol: Value = serde_json::from_str(PROTOCOL_0_17_4_0)
        .expect("Oh My Pi 17.4.0 protocol corpus is valid JSON");

    assert_eq!(identity["axis"], OH_MY_PI_PACKAGE_AXIS);
    assert_eq!(identity["version"], "17.4.0");
    assert_eq!(identity["npm_package"], "@oh-my-pi/pi-coding-agent");
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(
        identity["npm_integrity"],
        "sha512-RMLu7DrF/W2lEPNgQECGR1Uw6jbhAKnDUVGGhhRXvVPp3ntx8CCwW48aC2kfp5QV/lDFYg0Rw6/CXMo/85jIBw=="
    );
    assert_eq!(identity["local_cli"], serde_json::Value::Null);
    assert!(is_sha256(
        identity["extracted_cli_sha256"]
            .as_str()
            .expect("extracted 17.4.0 digest is text")
    ));
    assert_eq!(identity["npm_17_3_6_published"], false);
    assert_eq!(identity["npm_17_3_9_published"], false);
    assert_eq!(identity["unpublished_17_4_1"], true);
    assert_eq!(
        identity["rpc_md_blob_sha_v17_3_8"],
        identity["rpc_md_blob_sha_v17_4_0"]
    );
    assert_eq!(identity["mapped_rpc_sources_identical_to_v17_3_8"], true);
    assert_eq!(identity["pi_package_latest"], "0.84.2");

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(decision["private_milestone_checked"], true);
    assert_eq!(
        decision["reuse_behavior_revision"],
        "oh-my-pi.rpc-v2-v17.2.9"
    );
    assert_eq!(decision["raise_latest_qualified_to"], "17.4.0");
    assert_eq!(decision["keep_baseline"], "17.2.9");
    assert_eq!(decision["qualify_intermediates"], true);
    assert_eq!(decision["new_milestone"], false);
    assert_eq!(decision["mix_pi_package_axis"], false);
    assert_eq!(decision["map_tokenizer_js_api"], false);
    assert_eq!(decision["map_omp_ps"], false);
    assert_eq!(decision["map_cleanse"], false);
    assert_eq!(decision["map_extended_context"], false);
    assert_eq!(decision["provider_prompt_sent"], false);
    assert_eq!(decision["host_install_changed"], false);
    assert_eq!(decision["synthetic_later_unverified_newer"], "17.4.1");

    let flags = protocol["help_selected_flags_present"]
        .as_array()
        .expect("selected flags are an array");
    for required in [
        "--mode",
        "--no-session",
        "--provider",
        "--model",
        "--tools",
        "--no-extensions",
        "--no-skills",
        "--no-rules",
        "--no-prewalk",
        "--approval-mode",
        "--no-tools",
    ] {
        assert!(
            flags.iter().any(|flag| flag == required),
            "missing selected flag {required}"
        );
    }
    assert_eq!(protocol["selected_mode"], "rpc");
    assert_eq!(protocol["selected_approval_mode"], "always-ask");
    assert_eq!(protocol["rpc_md_unchanged_from_v17_3_8"], true);
    assert_eq!(protocol["mapped_rpc_sources_identical_to_v17_3_8"], true);
    assert_eq!(protocol["decoder_corpus"], "oh-my-pi-rpc-17.2.9");
    assert_eq!(protocol["provider_prompt_sent"], false);

    assert_eq!(OH_MY_PI_PACKAGE_LATEST_QUALIFIED_VERSION, "17.4.0");
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "17.3.8"
    );

    let claim = oh_my_pi_rpc_claim();
    for value in ["17.2.15", "17.3.7", "17.3.8", "17.4.0"] {
        assert!(matches!(
            claim.assess(&version(value)),
            InterfaceCompatibilityAssessment::Qualified(matched)
                if matched.support_status() == InterfaceSupportStatus::Maintained
        ));
    }
    assert!(matches!(
        claim.assess(&version("17.4.1")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert_eq!(
        oh_my_pi_package_binding("17.4.0")
            .expect("version binds")
            .axis()
            .as_str(),
        OH_MY_PI_PACKAGE_AXIS
    );
}

const IDENTITY_0_18_0_5: &str = include_str!("fixtures/oh-my-pi-18.0.5/identity.json");
const PROTOCOL_0_18_0_5: &str = include_str!("fixtures/oh-my-pi-18.0.5/protocol.json");

#[test]
fn identity_stops_18_0_5_after_official_latest_moved() {
    let identity: Value = serde_json::from_str(IDENTITY_0_18_0_5)
        .expect("Oh My Pi 18.0.5 identity corpus is valid JSON");
    let protocol: Value = serde_json::from_str(PROTOCOL_0_18_0_5)
        .expect("Oh My Pi 18.0.5 protocol corpus is valid JSON");

    assert_eq!(identity["axis"], OH_MY_PI_PACKAGE_AXIS);
    assert_eq!(identity["version"], "18.0.5");
    assert_eq!(identity["npm_package"], "@oh-my-pi/pi-coding-agent");
    assert_eq!(identity["assigned_official_latest"], "18.0.5");
    assert_eq!(identity["npm_latest"], "18.0.6");
    assert_eq!(identity["npm_latest_moved_during_run"], true);
    assert_eq!(
        identity["npm_integrity"],
        "sha512-4bDndTceC6R5gFLS+FnkSiDBrlVbAt2EjL9ca4K29Qd5R+fpxOaad3dOQSenKXd1y3Ot/MfoNGrfH2dXr5hpSA=="
    );
    assert_eq!(identity["local_cli"], serde_json::Value::Null);
    assert!(is_sha256(
        identity["extracted_cli_sha256"]
            .as_str()
            .expect("extracted 18.0.5 digest is text")
    ));
    assert_eq!(identity["unpublished_18_0_2"], true);
    assert_eq!(identity["mapped_rpc_sources_identical_to_v17_4_0"], false);
    assert_eq!(identity["mapped_rpc_framing_identical_to_v17_4_0"], true);
    assert_eq!(identity["pi_package_latest"], "0.84.3");

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "stop");
    assert_eq!(
        decision["stop_reasons"],
        serde_json::json!(["official-latest-moved-during-run"])
    );
    assert_eq!(decision["this_run_silent_inheritance"], false);
    assert_eq!(decision["this_run_claim"], false);
    assert_eq!(decision["eighteen_segment_unsettled"], true);
    assert_eq!(decision["later_identity_needed_for"], "18.0.6");
    assert_eq!(decision["operator_segment_decision_needed"], true);
    assert_eq!(decision["raise_same_17_x_window"], false);
    assert_eq!(decision["keep_latest_qualified"], "17.4.0");
    assert_eq!(decision["claim_card"], false);
    assert_eq!(decision["mix_pi_package_axis"], false);
    assert_eq!(decision["map_option_details"], false);
    assert_eq!(decision["provider_prompt_sent"], false);
    assert_eq!(decision["host_install_changed"], false);
    assert!(decision.get("new_milestone").is_none());
    assert!(
        decision
            .get("seventeen_and_eighteen_stay_separate")
            .is_none()
    );

    let flags = protocol["help_selected_flags_present"]
        .as_array()
        .expect("selected flags are an array");
    for required in [
        "--mode",
        "--no-session",
        "--provider",
        "--model",
        "--tools",
        "--no-extensions",
        "--no-skills",
        "--no-rules",
        "--no-prewalk",
        "--approval-mode",
        "--no-tools",
    ] {
        assert!(
            flags.iter().any(|flag| flag == required),
            "missing selected flag {required}"
        );
    }
    assert_eq!(protocol["selected_mode"], "rpc");
    assert_eq!(protocol["rpc_md_unchanged_from_v17_4_0"], false);
    assert_eq!(protocol["mapped_rpc_framing_identical_to_v17_4_0"], true);
    assert_eq!(protocol["decoder_corpus"], "oh-my-pi-rpc-17.2.9");
    assert_eq!(protocol["provider_prompt_sent"], false);

    assert_eq!(OH_MY_PI_PACKAGE_LATEST_QUALIFIED_VERSION, "17.4.0");
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "17.4.0"
    );

    let claim = oh_my_pi_rpc_claim();
    assert!(matches!(
        claim.assess(&version("17.4.0")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Maintained
    ));
    for value in ["17.4.1", "17.4.2", "18.0.5", "18.0.6"] {
        assert!(
            matches!(
                claim.assess(&version(value)),
                InterfaceCompatibilityAssessment::UnverifiedNewer(_)
            ),
            "{value} stays UnverifiedNewer"
        );
    }
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}
