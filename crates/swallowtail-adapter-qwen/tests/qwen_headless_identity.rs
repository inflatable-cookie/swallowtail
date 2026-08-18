use serde_json::Value;
use swallowtail_adapter_qwen::{
    QWEN_CODE_AXIS, QWEN_CODE_BASELINE_VERSION, QWEN_CODE_LATEST_QUALIFIED_VERSION,
    qwen_code_binding, qwen_headless_claim,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const IDENTITY: &str = include_str!("fixtures/qwen-code-0.21.13/identity.json");
const PROTOCOL: &str = include_str!("fixtures/qwen-code-0.21.13/protocol.json");

#[test]
fn identity_and_claim_qualify_0_21_13_as_compatible_extension() {
    let identity: Value =
        serde_json::from_str(IDENTITY).expect("Qwen 0.21.13 identity corpus is valid JSON");
    let protocol: Value =
        serde_json::from_str(PROTOCOL).expect("Qwen 0.21.13 protocol corpus is valid JSON");

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
    assert_eq!(QWEN_CODE_LATEST_QUALIFIED_VERSION, "0.21.13");
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "0.21.2"
    );

    let claim = qwen_headless_claim();
    assert!(matches!(
        claim.assess(&version("0.21.2")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Maintained
                && matched.behavior_revision().as_str()
                    == "qwen-code.headless.v0.21.0-catalogue-filter"
    ));
    for candidate in [
        "0.21.3", "0.21.4", "0.21.5", "0.21.6", "0.21.7", "0.21.8", "0.21.9", "0.21.10", "0.21.11",
        "0.21.12", "0.21.13",
    ] {
        assert!(matches!(
            claim.assess(&version(candidate)),
            InterfaceCompatibilityAssessment::Qualified(matched)
                if matched.support_status() == InterfaceSupportStatus::Maintained
                    && matched.behavior_revision().as_str()
                        == "qwen-code.headless.v0.21.0-catalogue-filter"
        ));
    }
    assert!(!claim.permits(&version("0.20.2")));
    assert!(matches!(
        claim.assess(&version("0.21.14")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert_eq!(
        qwen_code_binding("0.21.13")
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
