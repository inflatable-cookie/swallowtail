use serde_json::Value;
use swallowtail_adapter_ollama::{
    OLLAMA_BASELINE_VERSION, OLLAMA_LATEST_QUALIFIED_VERSION, ollama_runtime_binding,
    ollama_runtime_claim,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const IDENTITY: &str = include_str!("fixtures/ollama-0.32.14/identity.json");
const PROTOCOL: &str = include_str!("fixtures/ollama-0.32.14/protocol.json");

#[test]
fn identity_and_claim_qualify_0_32_14_as_compatible_extension() {
    let identity: Value =
        serde_json::from_str(IDENTITY).expect("Ollama 0.32.14 identity corpus is valid JSON");
    let protocol: Value =
        serde_json::from_str(PROTOCOL).expect("Ollama 0.32.14 protocol corpus is valid JSON");

    assert_eq!(identity["axis"], "ollama.runtime");
    assert_eq!(identity["not_ollama_cloud"], true);
    assert_eq!(identity["host"]["version"], "0.32.9");
    assert_eq!(identity["official"]["version"], "0.32.14");
    assert_eq!(identity["official"]["prerelease"], false);
    assert_eq!(
        identity["official"]["github_commit"],
        "d67ad83426633195089509347ffd4fe795120198"
    );
    assert!(is_sha256(
        identity["host"]["executable_sha256"]
            .as_str()
            .expect("host digest is text")
    ));
    assert_eq!(
        identity["unpublished_patch_in_0_32_3_through_0_32_14"],
        false
    );
    assert_eq!(
        identity["github_prerelease_plain_versions"],
        serde_json::json!(["0.32.2", "0.32.10"])
    );

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(decision["reuse_behavior"], "ollama.native-text-v1");
    assert_eq!(decision["raise_latest_qualified_to"], "0.32.14");
    assert_eq!(decision["keep_baseline"], "0.14.0");
    assert_eq!(decision["keep_exclusion_0_32_2"], true);
    assert_eq!(decision["add_exclusion_0_32_10"], true);
    assert_eq!(decision["new_public_operation"], false);
    assert_eq!(decision["flatten_to_cloud_or_generate"], false);
    assert_eq!(decision["provider_prompt_sent"], false);
    assert_eq!(decision["attached_server_started"], false);
    assert_eq!(decision["host_install_changed"], false);
    assert_eq!(decision["official_app_archive_downloaded"], false);

    let routes = protocol["selected_routes"]
        .as_array()
        .expect("selected routes are an array");
    for required in [
        "GET /api/version",
        "GET /api/tags",
        "GET /api/ps",
        "POST /api/show",
        "POST /api/chat",
    ] {
        assert!(
            routes.iter().any(|route| route == required),
            "missing selected route {required}"
        );
    }
    assert_eq!(
        protocol["selected_structs_identical_0_32_1_through_0_32_14"],
        true
    );
    assert_eq!(
        protocol["chat_request_sha256"],
        "d7035a0da458f5ab354f771d2ee3eb9239f1ff40dae6700bdcd8e9806b18ae14"
    );
    assert_eq!(protocol["new_public_selected_operation"], false);
    assert_eq!(protocol["decoder_corpus"], "ollama-native-v0.14.0-v0.32.1");
    assert_eq!(protocol["provider_prompt_sent"], false);
    assert_eq!(protocol["attached_server_started"], false);

    assert_eq!(OLLAMA_BASELINE_VERSION, "0.14.0");
    assert_eq!(OLLAMA_LATEST_QUALIFIED_VERSION, "0.32.14");
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "0.32.1"
    );

    let claim = ollama_runtime_claim();
    for version in ["0.14.0", "0.32.1", "0.32.3", "0.32.9", "0.32.11", "0.32.14"] {
        assert!(matches!(
            claim.assess(&version_value(version)),
            InterfaceCompatibilityAssessment::Qualified(matched)
                if matched.support_status() == InterfaceSupportStatus::Maintained
                    && matched.behavior_revision().as_str() == "ollama.native-text-v1"
        ));
    }
    for version in ["0.32.2", "0.32.10", "0.32.3-rc.0", "0.13.5"] {
        assert!(!claim.permits(&version_value(version)));
    }
    assert!(matches!(
        claim.assess(&version_value("0.32.15")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert_eq!(
        ollama_runtime_binding("0.32.14")
            .expect("version binds")
            .axis()
            .as_str(),
        "ollama.runtime"
    );
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn version_value(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}
