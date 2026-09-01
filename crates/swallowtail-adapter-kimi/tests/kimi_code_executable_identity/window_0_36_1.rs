use super::support::{IDENTITY_0_36_1, PROTOCOL_0_36_1, is_sha256, version};
use serde_json::Value;
use swallowtail_adapter_kimi::{
    KIMI_CODE_AXIS, KIMI_CODE_BASELINE_VERSION, KIMI_CODE_LATEST_QUALIFIED_VERSION,
    KIMI_HEADLESS_BASELINE_VERSION, KIMI_HEADLESS_LATEST_QUALIFIED_VERSION,
    KIMI_LOCAL_SERVER_BASELINE_VERSION, KIMI_LOCAL_SERVER_LATEST_QUALIFIED_VERSION, kimi_acp_claim,
    kimi_code_binding, kimi_headless_claim, kimi_local_server_claim,
};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceSupportStatus};

#[test]
fn identity_and_claim_qualify_0_36_1_as_compatible_extension() {
    let identity: Value =
        serde_json::from_str(IDENTITY_0_36_1).expect("Kimi 0.36.1 identity corpus is valid JSON");
    let protocol: Value =
        serde_json::from_str(PROTOCOL_0_36_1).expect("Kimi 0.36.1 protocol corpus is valid JSON");

    assert_eq!(identity["axis"], KIMI_CODE_AXIS);
    assert_eq!(identity["npm_package"], "@moonshot-ai/kimi-code");
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(identity["not_python_kimi_cli"], true);
    assert_eq!(identity["host"]["version"], "0.34.0");
    assert_eq!(identity["official"]["version"], "0.36.1");
    assert_eq!(
        identity["official"]["npm_integrity"],
        "sha512-dAYvA0qIZ/nPOtf+8X0axRP3Supa06oP9xK/JlY/DsrID5IVmDRc2fKTdASNBvSs1XPUbPFwD1cDNXMoEDQfEA=="
    );
    assert_eq!(
        identity["official"]["darwin_arm64_zip_sha256"],
        "14a09fb898742be77eb2bf41fc7fe0d78fdbdc73a4aa8fd3c80b04ebf6bee193"
    );
    assert!(is_sha256(
        identity["host"]["executable_sha256"]
            .as_str()
            .expect("host digest is text")
    ));
    assert!(is_sha256(
        identity["official"]["extracted_executable_sha256"]
            .as_str()
            .expect("official digest is text")
    ));
    assert_eq!(identity["unpublished_patch_in_0_32_through_0_36_1"], false);

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(
        decision["acp_reuse_behavior"],
        "kimi.acp.reasoning.declared-effort-v2"
    );
    assert_eq!(
        decision["headless_reuse_behavior"],
        "kimi.headless.stream-json.v1"
    );
    assert_eq!(
        decision["local_server_add_optional_meta_flags"],
        "0.32.0..=0.34.0"
    );
    assert_eq!(
        decision["local_server_add_heartbeat_ping"],
        "0.35.0..=0.36.1"
    );
    assert_eq!(decision["raise_latest_qualified_to"], "0.36.1");
    assert_eq!(decision["keep_baseline_acp"], "0.28.1");
    assert_eq!(decision["mix_python_kimi_cli_axis"], false);
    assert_eq!(decision["flatten_acp_onto_local_server"], false);
    assert_eq!(decision["provider_prompt_sent"], false);
    assert_eq!(decision["local_server_started"], false);
    assert_eq!(decision["host_install_changed"], false);

    let flags = protocol["help_selected_flags_present"]
        .as_array()
        .expect("selected flags are an array");
    for required in ["-p", "--prompt", "--output-format", "acp", "web"] {
        assert!(
            flags.iter().any(|flag| flag == required),
            "missing selected flag {required}"
        );
    }
    assert_eq!(protocol["selected_acp_command"], "acp");
    assert_eq!(protocol["selected_local_server_command"], "web");
    assert_eq!(protocol["acp_initialize"]["protocol_version"], 1);
    assert_eq!(
        protocol["acp_initialize"]["auth_methods"],
        serde_json::json!(["login"])
    );
    assert_eq!(
        protocol["acp_initialize"]["advertised_close_delete_mapped"],
        false
    );
    assert_eq!(protocol["acp_initialize"]["stderr_bytes"], 0);
    assert_eq!(
        protocol["selected_source"]["local_server_heartbeat"],
        "application-ping-pong-from-0.35.0"
    );
    assert_eq!(protocol["provider_prompt_sent"], false);
    assert_eq!(protocol["local_server_started"], false);

    assert_eq!(KIMI_CODE_BASELINE_VERSION, "0.28.1");
    assert_eq!(KIMI_HEADLESS_BASELINE_VERSION, "0.29.0");
    assert_eq!(KIMI_LOCAL_SERVER_BASELINE_VERSION, "0.28.1");
    // ACP stops at 0.38.0 for the 0.39 process-authority delta; headless
    // extends. The local-server family shares the npm package and moves with
    // neither.
    assert_eq!(KIMI_CODE_LATEST_QUALIFIED_VERSION, "0.38.0");
    assert_eq!(KIMI_HEADLESS_LATEST_QUALIFIED_VERSION, "0.39.1");
    assert_eq!(KIMI_LOCAL_SERVER_LATEST_QUALIFIED_VERSION, "0.38.0");
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "0.31.1"
    );

    for (claim, qualified, first_newer) in [
        (
            &kimi_acp_claim(),
            ["0.32.0", "0.34.0", "0.36.1", "0.37.2", "0.38.0"].as_slice(),
            "0.38.1",
        ),
        (
            &kimi_headless_claim(),
            [
                "0.33.0", "0.34.0", "0.36.1", "0.37.2", "0.38.0", "0.39.0", "0.39.1",
            ]
            .as_slice(),
            "0.39.2",
        ),
        (
            &kimi_local_server_claim(),
            ["0.32.0", "0.34.0", "0.35.0", "0.36.1", "0.37.2", "0.38.0"].as_slice(),
            "0.38.1",
        ),
    ] {
        for value in qualified {
            assert!(matches!(
                claim.assess(&version(value)),
                InterfaceCompatibilityAssessment::Qualified(matched)
                    if matched.support_status() == InterfaceSupportStatus::Maintained
            ));
        }
        if claim.id() == kimi_acp_claim().id() {
            assert_eq!(
                claim.assess(&version(first_newer)),
                InterfaceCompatibilityAssessment::Incompatible
            );
        } else {
            assert!(matches!(
                claim.assess(&version(first_newer)),
                InterfaceCompatibilityAssessment::UnverifiedNewer(_)
            ));
        }
    }
    assert_eq!(
        kimi_code_binding("0.36.1")
            .expect("version binds")
            .axis()
            .as_str(),
        KIMI_CODE_AXIS
    );
}
