use serde_json::Value;
use swallowtail_adapter_ollama::{
    OLLAMA_BASELINE_VERSION, OLLAMA_LATEST_QUALIFIED_VERSION, ollama_runtime_binding,
    ollama_runtime_claim, protocol::ChatDecoder,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const IDENTITY: &str = include_str!("fixtures/ollama-0.34.0/identity.json");
const PROTOCOL: &str = include_str!("fixtures/ollama-0.34.0/protocol.json");

#[test]
fn identity_freezes_0_33_x_hops_and_names_the_0_33_3_stop() {
    let identity: Value =
        serde_json::from_str(IDENTITY).expect("Ollama 0.34.0 identity corpus is valid JSON");
    let protocol: Value =
        serde_json::from_str(PROTOCOL).expect("Ollama 0.34.0 protocol corpus is valid JSON");

    assert_eq!(identity["axis"], "ollama.runtime");
    assert_eq!(identity["not_ollama_cloud"], true);
    assert_eq!(identity["host"]["client_version"], "0.33.3");
    assert_eq!(identity["official"]["version"], "0.34.0");
    assert_eq!(identity["official"]["prerelease"], false);
    assert_eq!(
        identity["official"]["github_commit"],
        "d8ab4b4f0ca24b51d3a46b3bf4f462e58ce66b1f"
    );
    assert_eq!(
        identity["official"]["github_tree"],
        "61d8627adc97b18b257bdb1419bf78948a052578"
    );
    assert_eq!(
        identity["published_stables_from_previous_ceiling"],
        serde_json::json!(["0.33.0", "0.33.1", "0.33.2", "0.33.3", "0.34.0"])
    );
    assert_eq!(
        identity["github_prerelease_plain_versions"],
        serde_json::json!(["0.32.2", "0.32.10"])
    );
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "0.32.15"
    );

    let hops = identity["published_hops"]
        .as_array()
        .expect("published hops are an array");
    assert_eq!(hops.len(), 5);
    for (hop, commit, tree) in [
        (
            "0.33.0",
            "ebf200f9521da9739a576a8bbf8cbf94e0cec6e3",
            "c537fd75700f30018c0a1161e26b7f2b215807a9",
        ),
        (
            "0.33.1",
            "13f2fb8c99278469b954429d5541019f4d83a4d0",
            "b3e63b82943bb57e17d501fa137d1abae26ee1f6",
        ),
        (
            "0.33.2",
            "f96e7aa0513b9973a0ccc71be414c2ecb9d65b1a",
            "c35ae275b3fd3d6f58b3ea12f468a2212b6a87f5",
        ),
        (
            "0.33.3",
            "b79067b0db7417f20108363bc22adb97f35c966a",
            "69ea0f1f9e302e6bfabe5b00b482b9b3ff6363e1",
        ),
        (
            "0.34.0",
            "d8ab4b4f0ca24b51d3a46b3bf4f462e58ce66b1f",
            "61d8627adc97b18b257bdb1419bf78948a052578",
        ),
    ] {
        let entry = hops
            .iter()
            .find(|entry| entry["version"] == hop)
            .unwrap_or_else(|| panic!("missing published hop {hop}"));
        assert_eq!(entry["github_commit"], commit, "commit drift at {hop}");
        assert_eq!(entry["github_tree"], tree, "tree drift at {hop}");
        assert_eq!(entry["prerelease"], false, "prerelease drift at {hop}");
    }

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension-with-stop");
    assert_eq!(decision["reuse_behavior"], "ollama.native-text-v1");
    assert_eq!(decision["raise_latest_qualified_to"], "0.33.2");
    assert_eq!(decision["keep_baseline"], "0.14.0");
    assert_eq!(decision["keep_exclusion_0_32_2"], true);
    assert_eq!(decision["keep_exclusion_0_32_10"], true);
    assert_eq!(
        decision["qualified_hops"],
        serde_json::json!(["0.33.0", "0.33.1", "0.33.2"])
    );
    assert_eq!(
        decision["stop_at_0_33_3"]["key"],
        "prompt_eval_cached_count"
    );
    assert_eq!(decision["leave_0_33_3_and_0_34_0"], "unverified_newer");
    assert_eq!(decision["new_public_operation"], false);
    assert_eq!(decision["flatten_to_cloud_or_generate"], false);
    assert_eq!(decision["flatten_to_llama_cpp"], false);
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
        protocol["selected_structs_identical_0_32_15_through_0_34_0"],
        true
    );
    for (key, digest) in [
        (
            "chat_request",
            "d7035a0da458f5ab354f771d2ee3eb9239f1ff40dae6700bdcd8e9806b18ae14",
        ),
        (
            "chat_response",
            "f9deac910407655aa7ecbded4820f8de5d404bb2f49e261b1a64b7db56f2b936",
        ),
        (
            "message",
            "bfc165a63e64996e5b843072e8be965a9798e214dfc1c88cc5974bc802d0c9f9",
        ),
        (
            "list_response",
            "3bd9321282acf137960e6480b523b2575f585ee0f7ea928301def363b426c6b9",
        ),
        (
            "process_response",
            "caaa50bff293aefb1d2702bf411dab49fe933225ef76a5f5591bdc8ffae1072d",
        ),
        (
            "show_request",
            "698cdcb3a9942de50bf9be2e65525e945a9ee3613d8309d2c3435b1db207852e",
        ),
        (
            "show_response",
            "eeba5938770c1ff3181e1f183a3ef9217dd44635688fd29b0940ea2ce68d7a71",
        ),
        (
            "options",
            "c615649cf6298722a03719706c4533308fa8a86e944bfeb936aad4c919ad02b7",
        ),
    ] {
        assert_eq!(
            protocol["selected_struct_hashes"][key], digest,
            "selected struct drift at {key}"
        );
    }
    assert_eq!(
        protocol["types_go_file_hashes"]["v0.33.0"],
        protocol["types_go_file_hashes"]["v0.32.15"]
    );
    assert_eq!(
        protocol["routes_go_file_hashes"]["v0.33.2"],
        protocol["routes_go_file_hashes"]["v0.32.15"]
    );
    assert_eq!(
        protocol["metrics_struct_hashes"]["v0.32.15_through_v0.33.2"],
        "1166c0471d4afd6ee12b56ae79b99e26aecfb58d347d4129961de0f26603a624"
    );
    assert_eq!(
        protocol["metrics_struct_hashes"]["v0.33.3_and_v0.34.0"],
        "bbc09305511a40e65370507477be58246ae171095d73ff35935d5f0066b38cb4"
    );
    assert_eq!(protocol["new_public_selected_operation"], false);
    assert_eq!(protocol["decoder_corpus"], "ollama-native-v0.14.0-v0.32.1");
    assert_eq!(protocol["provider_prompt_sent"], false);
    assert_eq!(protocol["attached_server_started"], false);

    assert_eq!(OLLAMA_BASELINE_VERSION, "0.14.0");
    assert_eq!(OLLAMA_LATEST_QUALIFIED_VERSION, "0.32.15");
    let claim = ollama_runtime_claim();
    assert!(matches!(
        claim.assess(&version_value("0.32.15")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Maintained
    ));
    assert!(matches!(
        claim.assess(&version_value("0.33.2")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert_eq!(
        ollama_runtime_binding("0.34.0")
            .expect("version binds")
            .axis()
            .as_str(),
        "ollama.runtime"
    );
}

#[test]
fn strict_decoder_fail_closes_on_0_33_3_cached_count_key() {
    let counterexample = protocol_record("stop_counterexample_terminal_record");
    let mut decoder = ChatDecoder::new("m:8b");
    assert!(
        decoder
            .push(format!("{counterexample}\n").as_bytes())
            .is_err(),
        "native-text-v1 must fail closed on the unmapped 0.33.3 metrics key"
    );

    let without_key = counterexample.replace("\"prompt_eval_cached_count\":4,", "");
    let mut decoder = ChatDecoder::new("m:8b");
    let events = decoder
        .push(format!("{without_key}\n").as_bytes())
        .expect("identical record without the new key decodes");
    assert_eq!(events.len(), 2);
}

fn protocol_record(key: &str) -> String {
    let protocol: Value =
        serde_json::from_str(PROTOCOL).expect("Ollama 0.34.0 protocol corpus is valid JSON");
    protocol[key]
        .as_str()
        .unwrap_or_else(|| panic!("protocol record {key} is a string"))
        .to_owned()
}

fn version_value(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}
