use serde_json::Value;
use swallowtail_adapter_kimi::{
    kimi_acp_claim, kimi_code_binding, kimi_headless_claim, kimi_local_server_claim,
    KIMI_CODE_AXIS, KIMI_CODE_BASELINE_VERSION, KIMI_CODE_LATEST_QUALIFIED_VERSION,
    KIMI_HEADLESS_BASELINE_VERSION, KIMI_HEADLESS_LATEST_QUALIFIED_VERSION,
    KIMI_LOCAL_SERVER_BASELINE_VERSION, KIMI_LOCAL_SERVER_LATEST_QUALIFIED_VERSION,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const IDENTITY_0_36_1: &str = include_str!("fixtures/kimi-code-0.36.1/identity.json");
const PROTOCOL_0_36_1: &str = include_str!("fixtures/kimi-code-0.36.1/protocol.json");
const IDENTITY_0_37_2: &str = include_str!("fixtures/kimi-code-0.37.2/identity.json");
const PROTOCOL_0_37_2: &str = include_str!("fixtures/kimi-code-0.37.2/protocol.json");
const IDENTITY_0_38_0: &str = include_str!("fixtures/kimi-code-0.38.0/identity.json");
const PROTOCOL_0_38_0: &str = include_str!("fixtures/kimi-code-0.38.0/protocol.json");
const IDENTITY_0_38_0_HEADLESS_V2: &str =
    include_str!("fixtures/kimi-code-0.38.0-headless-v2/identity.json");
const PROTOCOL_0_38_0_HEADLESS_V2: &str =
    include_str!("fixtures/kimi-code-0.38.0-headless-v2/protocol.json");

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
    assert_eq!(KIMI_CODE_LATEST_QUALIFIED_VERSION, "0.38.0");
    assert_eq!(KIMI_HEADLESS_LATEST_QUALIFIED_VERSION, "0.38.0");
    assert_eq!(KIMI_LOCAL_SERVER_LATEST_QUALIFIED_VERSION, "0.38.0");
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "0.31.1"
    );

    for (claim, qualified) in [
        (
            &kimi_acp_claim(),
            ["0.32.0", "0.34.0", "0.36.1", "0.37.2", "0.38.0"].as_slice(),
        ),
        (
            &kimi_headless_claim(),
            ["0.32.0", "0.34.0", "0.36.1", "0.37.2", "0.38.0"].as_slice(),
        ),
        (
            &kimi_local_server_claim(),
            ["0.32.0", "0.34.0", "0.35.0", "0.36.1", "0.37.2", "0.38.0"].as_slice(),
        ),
    ] {
        for value in qualified {
            assert!(matches!(
                claim.assess(&version(value)),
                InterfaceCompatibilityAssessment::Qualified(matched)
                    if matched.support_status() == InterfaceSupportStatus::Maintained
            ));
        }
        assert!(matches!(
            claim.assess(&version("0.38.1")),
            InterfaceCompatibilityAssessment::UnverifiedNewer(_)
        ));
    }
    assert_eq!(
        kimi_code_binding("0.36.1")
            .expect("version binds")
            .axis()
            .as_str(),
        KIMI_CODE_AXIS
    );
}

#[test]
fn identity_and_claim_qualify_0_37_2_as_compatible_extension() {
    let identity: Value =
        serde_json::from_str(IDENTITY_0_37_2).expect("Kimi 0.37.2 identity corpus is valid JSON");
    let protocol: Value =
        serde_json::from_str(PROTOCOL_0_37_2).expect("Kimi 0.37.2 protocol corpus is valid JSON");

    assert_eq!(identity["axis"], KIMI_CODE_AXIS);
    assert_eq!(identity["npm_package"], "@moonshot-ai/kimi-code");
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(identity["not_python_kimi_cli"], true);
    assert_eq!(identity["host"]["version"], "0.34.0");
    assert_eq!(identity["official"]["version"], "0.37.2");
    assert_eq!(
        identity["official"]["npm_integrity"],
        "sha512-TAteYb84mV44MEzCaAlfz5f3TiN2yMHuwj9Kd0ePEIMBUqgjlqV1w7PvMT9TN0t87LYfv7BhIYz+ZCHDOM5aJw=="
    );
    assert_eq!(
        identity["official"]["darwin_arm64_zip_sha256"],
        "d5256d7dc5f43bda1cddbdccd810d247becbc4884d6c971e465044e3a6999c7a"
    );
    assert!(is_sha256(
        identity["official"]["extracted_executable_sha256"]
            .as_str()
            .expect("official digest is text")
    ));
    assert_eq!(identity["unpublished_0_37_3"], true);

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
        decision["local_server_keep_heartbeat_ping"],
        "0.35.0..=0.37.2"
    );
    assert_eq!(decision["raise_latest_qualified_to"], "0.37.2");
    assert_eq!(decision["map_watch_fs_runtime_id"], false);
    assert_eq!(decision["provider_prompt_sent"], false);
    assert_eq!(decision["local_server_started"], false);
    assert_eq!(decision["host_install_changed"], false);
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "0.36.1"
    );

    assert_eq!(protocol["selected_acp_command"], "acp");
    assert_eq!(protocol["acp_initialize"]["protocol_version"], 1);
    assert_eq!(
        protocol["acp_initialize"]["auth_methods"],
        serde_json::json!(["login"])
    );
    assert_eq!(protocol["acp_initialize"]["host_paths_discarded"], true);
    assert_eq!(
        protocol["selected_source"]["local_server_heartbeat"],
        "application-ping-pong-from-0.35.0"
    );
    assert_eq!(
        protocol["selected_source"]["watch_fs_optional_runtime_id_from"],
        "0.37.0"
    );

    assert_eq!(KIMI_CODE_LATEST_QUALIFIED_VERSION, "0.38.0");
    let claim = kimi_acp_claim();
    for value in ["0.37.0", "0.37.1", "0.37.2", "0.38.0"] {
        assert!(matches!(
            claim.assess(&version(value)),
            InterfaceCompatibilityAssessment::Qualified(matched)
                if matched.support_status() == InterfaceSupportStatus::Maintained
        ));
    }
    assert!(matches!(
        claim.assess(&version("0.38.1")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert_eq!(
        kimi_code_binding("0.37.2")
            .expect("version binds")
            .axis()
            .as_str(),
        KIMI_CODE_AXIS
    );
}

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
    assert_eq!(
        protocol["selected_headless"]["qualified_ceiling"],
        "0.37.2"
    );
    assert_eq!(
        protocol["selected_headless"]["default_engine_at_0_38_0"],
        "agent-core-v2-run-v2-print"
    );
    assert_eq!(protocol["selected_headless"]["v2_headless_qualified"], false);
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
    assert_eq!(KIMI_HEADLESS_LATEST_QUALIFIED_VERSION, "0.38.0");
    assert_eq!(KIMI_LOCAL_SERVER_LATEST_QUALIFIED_VERSION, "0.38.0");
    let acp_claim = kimi_acp_claim();
    assert!(matches!(
        acp_claim.assess(&version("0.38.0")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Maintained
    ));
    assert!(matches!(
        acp_claim.assess(&version("0.38.1")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    let headless_claim = kimi_headless_claim();
    assert!(matches!(
        headless_claim.assess(&version("0.37.2")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Maintained
                && matched.behavior_revision().as_str() == "kimi.headless.stream-json.v1"
    ));
    assert!(matches!(
        headless_claim.assess(&version("0.38.0")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Maintained
                && matched.behavior_revision().as_str() == "kimi.headless.stream-json.v2"
    ));
    assert!(matches!(
        headless_claim.assess(&version("0.38.1")),
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

#[test]
fn headless_v2_corpus_admits_adapter_private_milestone_at_0_38_0() {
    let identity: Value = serde_json::from_str(IDENTITY_0_38_0_HEADLESS_V2)
        .expect("Kimi 0.38.0 headless v2 identity corpus is valid JSON");
    let protocol: Value = serde_json::from_str(PROTOCOL_0_38_0_HEADLESS_V2)
        .expect("Kimi 0.38.0 headless v2 protocol corpus is valid JSON");

    assert_eq!(identity["axis"], KIMI_CODE_AXIS);
    assert_eq!(identity["official_version"], "0.38.0");
    assert_eq!(identity["default_dispatch"], "agent-core-v2-run-v2-print");
    assert_eq!(
        identity["identity_decision"]["shape"],
        "adapter-private-milestone"
    );
    assert_eq!(
        identity["identity_decision"]["behavior_revision"],
        "kimi.headless.stream-json.v2"
    );
    assert_eq!(identity["identity_decision"]["qualified_exact"], "0.38.0");
    assert_eq!(identity["identity_decision"]["preserve_v1_through"], "0.37.2");
    assert_eq!(
        protocol["selected_headless_v2"]["facade_id"],
        "kimi-headless-stream-json-v2"
    );
    assert_eq!(
        protocol["selected_headless_v2"]["preamble_meta"],
        "system.version"
    );
    assert_eq!(protocol["provider_prompt_sent"], false);
    assert_eq!(protocol["host_install_changed"], false);

    let headless_claim = kimi_headless_claim();
    assert!(matches!(
        headless_claim.assess(&version("0.38.0")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.behavior_revision().as_str() == "kimi.headless.stream-json.v2"
    ));
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version text is non-empty")
}
