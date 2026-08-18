use serde_json::Value;
use swallowtail_adapter_pi::{
    PI_PACKAGE_AXIS, PI_PACKAGE_BASELINE_VERSION, PI_PACKAGE_LATEST_QUALIFIED_VERSION,
    pi_package_binding, pi_rpc_claim,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const IDENTITY: &str = include_str!("fixtures/pi-rpc-0.84.2/identity.json");
const PROTOCOL: &str = include_str!("fixtures/pi-rpc-0.84.2/protocol.json");

#[test]
fn identity_and_claim_qualify_0_84_2_as_compatible_extension() {
    let identity: Value =
        serde_json::from_str(IDENTITY).expect("Pi 0.84.2 identity corpus is valid JSON");
    let protocol: Value =
        serde_json::from_str(PROTOCOL).expect("Pi 0.84.2 protocol corpus is valid JSON");

    assert_eq!(identity["axis"], PI_PACKAGE_AXIS);
    assert_eq!(identity["npm_package"], "@earendil-works/pi-coding-agent");
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(identity["not_oh_my_pi"], true);
    assert_eq!(identity["host"]["version"], "0.83.0");
    assert_eq!(identity["official"]["version"], "0.84.2");
    assert_eq!(
        identity["official"]["npm_integrity"],
        "sha512-l4E+B7hgXKWddRo8bC/eSue2aWZjEgJ9xIpf5p0Og+lq8a2TArCwJ0HCoCPCgaBP/tN4zbYH/wOwvx9pJpeLCA=="
    );
    assert_eq!(
        identity["official"]["github_commit"],
        "914cf1472e715297caa30db4b9535d534a9eb718"
    );
    assert!(is_sha256(
        identity["host"]["executable_sha256"]
            .as_str()
            .expect("host digest is text")
    ));
    assert_eq!(identity["unpublished_0_83_1"], true);
    assert_eq!(identity["unpublished_0_84_3"], true);

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(decision["keep_v0_83_0_exact"], true);
    assert_eq!(decision["add_private_v0_84_0"], "0.84.0..=0.84.2");
    assert_eq!(
        decision["v0_84_0_behavior"],
        "pi.rpc.strict-lf-v0.84.0-message-update-delta"
    );
    assert_eq!(decision["raise_latest_qualified_to"], "0.84.2");
    assert_eq!(decision["keep_baseline"], "0.80.10");
    assert_eq!(decision["keep_unpublished_0_83_1"], true);
    assert_eq!(decision["map_streaming_usage"], false);
    assert_eq!(decision["flatten_to_oh_my_pi"], false);
    assert_eq!(decision["provider_prompt_sent"], false);
    assert_eq!(decision["live_rpc_session"], false);
    assert_eq!(decision["host_install_changed"], false);

    let flags = protocol["help_selected_flags_present"]
        .as_array()
        .expect("selected flags are an array");
    for required in [
        "--mode",
        "--no-session",
        "--offline",
        "--provider",
        "--model",
    ] {
        assert!(
            flags.iter().any(|flag| flag == required),
            "missing selected flag {required}"
        );
    }
    assert_eq!(protocol["selected_mode"], "rpc");
    assert_eq!(protocol["rpc_types_identical_0_81_0_through_0_84_2"], true);
    assert_eq!(protocol["jsonl_identical_0_83_0_through_0_84_2"], true);
    assert_eq!(protocol["session_cwd_identical"], true);
    assert_eq!(
        protocol["message_update_drops_cumulative_snapshot_from"],
        "0.84.0"
    );
    assert_eq!(protocol["decoder_corpus"], "pi-rpc-0.80.10");
    assert_eq!(protocol["provider_prompt_sent"], false);

    assert_eq!(PI_PACKAGE_BASELINE_VERSION, "0.80.10");
    assert_eq!(PI_PACKAGE_LATEST_QUALIFIED_VERSION, "0.84.2");
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "0.83.0"
    );

    let claim = pi_rpc_claim();
    assert!(matches!(
        claim.assess(&version("0.83.0")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Deprecated
                && matched.behavior_revision().as_str()
                    == "pi.rpc.strict-lf-v0.83.0-bash-extension-hook"
    ));
    for candidate in ["0.84.0", "0.84.1", "0.84.2"] {
        assert!(matches!(
            claim.assess(&version(candidate)),
            InterfaceCompatibilityAssessment::Qualified(matched)
                if matched.support_status() == InterfaceSupportStatus::Maintained
                    && matched.behavior_revision().as_str()
                        == "pi.rpc.strict-lf-v0.84.0-message-update-delta"
        ));
    }
    assert!(!claim.permits(&version("0.83.1")));
    assert!(matches!(
        claim.assess(&version("0.84.3")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert_eq!(
        pi_package_binding("0.84.2")
            .expect("version binds")
            .axis()
            .as_str(),
        PI_PACKAGE_AXIS
    );
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}
