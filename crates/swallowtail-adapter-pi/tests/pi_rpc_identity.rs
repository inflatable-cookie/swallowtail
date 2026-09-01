use serde_json::Value;
use swallowtail_adapter_pi::{
    PI_PACKAGE_AXIS, PI_PACKAGE_BASELINE_VERSION, PI_PACKAGE_LATEST_QUALIFIED_VERSION,
    pi_package_binding, pi_rpc_claim,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const PRIOR_IDENTITY: &str = include_str!("fixtures/pi-rpc-0.84.2/identity.json");
const IDENTITY: &str = include_str!("fixtures/pi-rpc-0.84.3/identity.json");
const PROTOCOL: &str = include_str!("fixtures/pi-rpc-0.84.3/protocol.json");

#[test]
fn identity_and_claim_qualify_0_84_3_as_compatible_extension() {
    let prior: Value =
        serde_json::from_str(PRIOR_IDENTITY).expect("Pi 0.84.2 identity corpus is valid JSON");
    let identity: Value =
        serde_json::from_str(IDENTITY).expect("Pi 0.84.3 identity corpus is valid JSON");
    let protocol: Value =
        serde_json::from_str(PROTOCOL).expect("Pi 0.84.3 protocol corpus is valid JSON");

    assert_eq!(prior["official"]["version"], "0.84.2");
    assert_eq!(identity["axis"], PI_PACKAGE_AXIS);
    assert_eq!(identity["npm_package"], "@earendil-works/pi-coding-agent");
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(identity["not_oh_my_pi"], true);
    assert_eq!(identity["host"]["not_installed"], true);
    assert_eq!(identity["official"]["version"], "0.84.3");
    assert_eq!(
        identity["official"]["npm_integrity"],
        "sha512-Yr2p9PubrbFZmYEPYI+C8KmZP9xlFuLDnAG64RtU0ZDgrdiXYWa+y7WGyJO5OlqPliOkVCMd9IzVszO3/t0D0w=="
    );
    assert_eq!(
        identity["official"]["github_tag_commit"],
        "4e58f324fae8ebfa98a3d45181fb248072a2afac"
    );
    assert_eq!(
        identity["official"]["npm_git_head"],
        "bfb004d4418ff05c6f909eaaab856cbe75c1fde0"
    );
    assert!(is_sha256(
        identity["official"]["tarball_sha256"]
            .as_str()
            .expect("tarball digest is text")
    ));
    assert_eq!(identity["unpublished_0_83_1"], true);
    assert_eq!(identity["unpublished_0_84_4"], true);

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(decision["keep_v0_83_0_exact"], true);
    assert_eq!(decision["extend_private_v0_84_0"], "0.84.0..=0.84.3");
    assert_eq!(
        decision["v0_84_0_behavior"],
        "pi.rpc.strict-lf-v0.84.0-message-update-delta"
    );
    assert_eq!(decision["raise_latest_qualified_to"], "0.84.3");
    assert_eq!(decision["keep_baseline"], "0.80.10");
    assert_eq!(decision["keep_unpublished_0_83_1"], true);
    assert_eq!(decision["later_unverified_after_qualification"], "0.84.4");
    assert_eq!(decision["map_streaming_usage"], false);
    assert_eq!(decision["map_toolcall_start_id_and_tool_name"], false);
    assert_eq!(decision["flatten_to_oh_my_pi"], false);
    assert_eq!(decision["new_milestone"], false);
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
    assert_eq!(protocol["rpc_types_identical_0_81_0_through_0_84_3"], true);
    assert_eq!(protocol["jsonl_identical_0_83_0_through_0_84_3"], true);
    assert_eq!(protocol["session_cwd_identical"], true);
    assert_eq!(
        protocol["message_update_drops_cumulative_snapshot_from"],
        "0.84.0"
    );
    assert_eq!(protocol["toolcall_start_id_and_tool_name_from"], "0.84.3");
    assert_eq!(protocol["toolcall_start_classifies_as_progress"], true);
    assert_eq!(protocol["decoder_corpus"], "pi-rpc-0.80.10");
    assert_eq!(protocol["provider_prompt_sent"], false);

    assert_eq!(PI_PACKAGE_BASELINE_VERSION, "0.80.10");
    assert_eq!(PI_PACKAGE_LATEST_QUALIFIED_VERSION, "0.84.4");
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "0.84.2"
    );

    let claim = pi_rpc_claim();
    assert!(matches!(
        claim.assess(&version("0.83.0")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Deprecated
                && matched.behavior_revision().as_str()
                    == "pi.rpc.strict-lf-v0.83.0-bash-extension-hook"
    ));
    for candidate in ["0.84.0", "0.84.1", "0.84.2", "0.84.3", "0.84.4"] {
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
        claim.assess(&version("0.84.5")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert_eq!(
        pi_package_binding("0.84.3")
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
