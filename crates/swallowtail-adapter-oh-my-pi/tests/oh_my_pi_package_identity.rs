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
    assert_eq!(OH_MY_PI_PACKAGE_LATEST_QUALIFIED_VERSION, "17.3.7");
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
    assert!(matches!(
        claim.assess(&version("17.3.8")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert_eq!(
        oh_my_pi_package_binding("17.3.7")
            .expect("version binds")
            .axis()
            .as_str(),
        OH_MY_PI_PACKAGE_AXIS
    );
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}
