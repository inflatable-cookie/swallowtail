use super::support::{IDENTITY_0_38_0, PROTOCOL_0_38_0, is_sha256, version};
use serde_json::Value;
use swallowtail_adapter_kimi::{
    KIMI_CODE_AXIS, KIMI_CODE_LATEST_QUALIFIED_VERSION, KIMI_HEADLESS_LATEST_QUALIFIED_VERSION,
    KIMI_LOCAL_SERVER_LATEST_QUALIFIED_VERSION, kimi_acp_claim, kimi_code_binding,
    kimi_headless_claim,
};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceSupportStatus};

#[test]
fn identity_0_38_0_qualifies_acp_and_local_server_retracts_headless() {
    let identity: Value =
        serde_json::from_str(IDENTITY_0_38_0).expect("Kimi 0.38.0 identity corpus is valid JSON");
    let protocol: Value =
        serde_json::from_str(PROTOCOL_0_38_0).expect("Kimi 0.38.0 protocol corpus is valid JSON");

    assert_eq!(identity["axis"], KIMI_CODE_AXIS);
    assert_eq!(identity["npm_package"], "@moonshot-ai/kimi-code");
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(identity["not_python_kimi_cli"], true);
    assert_eq!(identity["not_kimi_platform_chat"], true);
    assert_eq!(identity["host"]["source"], "not-installed");
    assert_eq!(identity["official"]["version"], "0.38.0");
    assert_eq!(
        identity["official"]["npm_integrity"],
        "sha512-O/z6sfjFdoDPPeTnoXzdsJ2U8IqP6K2gD3LsT+Nu8BAlHwdhCjdCQFkFTjIbLBun+aZT6x81ha5FiFt7trEilg=="
    );
    assert_eq!(
        identity["official"]["linux_x64_zip_sha256"],
        "2278e0c90283985c4df46b775bf0f163d07684a7b1bfc83ee3b42844f6fccdfb"
    );
    assert_eq!(
        identity["official"]["darwin_arm64_zip_sha256"],
        "48f534fcbf2d42c0cf80334c1c89e8253d4c198a149980e234b6e927c2759fda"
    );
    assert!(is_sha256(
        identity["official"]["extracted_linux_x64_sha256"]
            .as_str()
            .expect("official linux digest is text")
    ));
    assert!(is_sha256(
        identity["official"]["extracted_darwin_arm64_sha256"]
            .as_str()
            .expect("official darwin digest is text")
    ));
    assert_eq!(identity["unpublished_0_37_3"], true);
    assert_eq!(identity["unpublished_0_38_1"], true);
    assert_eq!(identity["pypi_kimi_cli"], "1.49.0");

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(decision["private_milestone"], false);
    assert_eq!(
        decision["acp_reuse_behavior"],
        "kimi.acp.reasoning.declared-effort-v2"
    );
    assert_eq!(
        decision["headless_reuse_behavior"],
        "kimi.headless.stream-json.v1"
    );
    assert_eq!(
        decision["local_server_keep_heartbeat_ping"],
        "0.35.0..=0.38.0"
    );
    assert_eq!(decision["raise_latest_qualified_to"], "0.38.0");
    assert_eq!(decision["map_acp_login_region"], false);
    assert_eq!(decision["map_wait_for_tool"], false);
    assert_eq!(decision["mix_python_kimi_cli_axis"], false);
    assert_eq!(decision["flatten_onto_kimi_platform_chat"], false);
    assert_eq!(decision["provider_prompt_sent"], false);
    assert_eq!(decision["local_server_started"], false);
    assert_eq!(decision["host_install_changed"], false);
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "0.37.2"
    );
    assert_eq!(
        identity["claim_at_observation"]["classification_of_0_38_0_headless"],
        "unverified_newer"
    );
    assert_eq!(
        identity["claim_at_observation"]["classification_of_0_38_0_acp_and_local_server"],
        "qualified_maintained"
    );

    assert_eq!(protocol["selected_acp_command"], "acp");
    assert_eq!(protocol["acp_initialize"]["protocol_version"], 1);
    assert_eq!(
        protocol["acp_initialize"]["auth_methods"],
        serde_json::json!(["login"])
    );
    assert_eq!(
        protocol["acp_initialize"]["extracted_agent_version"],
        "0.38.0"
    );
    assert_eq!(protocol["selected_headless"]["qualified_ceiling"], "0.37.2");
    assert_eq!(
        protocol["selected_headless"]["default_engine_at_0_38_0"],
        "agent-core-v2-run-v2-print"
    );
    assert_eq!(
        protocol["selected_headless"]["v2_headless_qualified"],
        false
    );
    assert_eq!(protocol["acp_initialize"]["host_paths_discarded"], true);
    assert_eq!(
        protocol["selected_source"]["local_server_heartbeat"],
        "application-ping-pong-from-0.35.0"
    );
    assert_eq!(
        protocol["selected_source"]["events_map_blob_unchanged_0_31_1_through_0_38_0"],
        "0448f2eb9cb111755c5b0855f5ec72bf4d6bcd4c"
    );
    let unused = protocol["unused_deltas"]
        .as_array()
        .expect("unused deltas are an array");
    for required in ["acp --region", "WaitFor agent tool"] {
        assert!(
            unused.iter().any(|delta| delta == required),
            "missing unused delta {required}"
        );
    }

    assert_eq!(KIMI_CODE_LATEST_QUALIFIED_VERSION, "0.38.0");
    assert_eq!(KIMI_HEADLESS_LATEST_QUALIFIED_VERSION, "0.39.1");
    assert_eq!(KIMI_LOCAL_SERVER_LATEST_QUALIFIED_VERSION, "0.38.0");
    let acp_claim = kimi_acp_claim();
    assert!(matches!(
        acp_claim.assess(&version("0.38.0")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Maintained
    ));
    assert_eq!(
        acp_claim.assess(&version("0.38.1")),
        InterfaceCompatibilityAssessment::Incompatible
    );
    assert_eq!(
        acp_claim.assess(&version("0.39.2")),
        InterfaceCompatibilityAssessment::Incompatible
    );
    let headless_claim = kimi_headless_claim();
    // Research 270 corrected this: 0.37.2's default -p engine is v2, so the
    // last Deprecated v1 point is 0.32.0, not 0.37.2.
    assert!(matches!(
        headless_claim.assess(&version("0.32.0")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Deprecated
                && matched.behavior_revision().as_str() == "kimi.headless.stream-json.v1"
    ));
    assert!(matches!(
        headless_claim.assess(&version("0.37.2")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Maintained
                && matched.behavior_revision().as_str() == "kimi.headless.stream-json.v2"
    ));
    assert!(matches!(
        headless_claim.assess(&version("0.38.0")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Maintained
                && matched.behavior_revision().as_str() == "kimi.headless.stream-json.v2"
    ));
    assert!(matches!(
        headless_claim.assess(&version("0.39.2")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(newer)
            if newer.behavior_revision().as_str() == "kimi.headless.stream-json.v2"
    ));
    assert_eq!(
        kimi_code_binding("0.38.0")
            .expect("version binds")
            .axis()
            .as_str(),
        KIMI_CODE_AXIS
    );
}
