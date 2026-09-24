use serde_json::Value;
use swallowtail_adapter_ollama::{
    OLLAMA_BASELINE_VERSION, OLLAMA_LATEST_QUALIFIED_VERSION, ollama_runtime_binding,
    ollama_runtime_claim,
    protocol::{ChatDecoder, NativeEvent},
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const IDENTITY: &str = include_str!("fixtures/ollama-0.34.2/identity.json");
const PROTOCOL: &str = include_str!("fixtures/ollama-0.34.2/protocol.json");

#[test]
fn identity_freezes_0_33_3_through_0_34_2_and_names_compatible_extension() {
    let identity: Value =
        serde_json::from_str(IDENTITY).expect("Ollama 0.34.2 identity corpus is valid JSON");
    let protocol: Value =
        serde_json::from_str(PROTOCOL).expect("Ollama 0.34.2 protocol corpus is valid JSON");

    assert_eq!(identity["axis"], "ollama.runtime");
    assert_eq!(identity["not_ollama_cloud"], true);
    assert_eq!(identity["host"]["not_installed"], true);
    assert_eq!(identity["official"]["version"], "0.34.2");
    assert_eq!(identity["official"]["prerelease"], false);
    assert_eq!(
        identity["official"]["github_commit"],
        "dfabde4539e42ba1e1eab50a3a50b88aea7958a0"
    );
    assert_eq!(
        identity["official"]["github_tree"],
        "f7a55d8ed225cbddcaa585b7c5dad7fbb7c80d00"
    );
    assert_eq!(
        identity["official"]["tag_tarball_sha256"],
        "3fb06dc496f321749423792066f299205c433b815540c1f23f18368540401640"
    );
    assert_eq!(
        identity["published_stables_from_previous_ceiling"],
        serde_json::json!(["0.33.3", "0.34.0", "0.34.1", "0.34.2"])
    );
    assert_eq!(identity["previous_ceiling"], "0.33.2");
    assert_eq!(identity["unpublished_next"], "0.34.3");
    assert_eq!(
        identity["github_prerelease_plain_versions"],
        serde_json::json!(["0.32.2", "0.32.10"])
    );
    assert_eq!(
        identity["ignored_prerelease_after_official"],
        serde_json::json!(["0.34.3-rc1"])
    );
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "0.33.2"
    );

    let hops = identity["published_hops"]
        .as_array()
        .expect("published hops are an array");
    assert_eq!(hops.len(), 4);
    for (hop, commit, tree, tarball) in [
        (
            "0.33.3",
            "b79067b0db7417f20108363bc22adb97f35c966a",
            "69ea0f1f9e302e6bfabe5b00b482b9b3ff6363e1",
            "eaf9e0e7f78f1cac54dea518cbab6346890516e28c49575ee754bd778094d6d6",
        ),
        (
            "0.34.0",
            "d8ab4b4f0ca24b51d3a46b3bf4f462e58ce66b1f",
            "61d8627adc97b18b257bdb1419bf78948a052578",
            "810e7bfc2d321783f0ffd294290bca72deb292c0fd481534849dd595d59ade49",
        ),
        (
            "0.34.1",
            "38fdb5dd58c761f850cddd6ba1e78a7954646b4f",
            "0a4e45de57716a630a8275e124f64a29b370ec93",
            "e171eabbad44f0a95fbddaa40e78257b94dd7618528be47c11174be6d8d08585",
        ),
        (
            "0.34.2",
            "dfabde4539e42ba1e1eab50a3a50b88aea7958a0",
            "f7a55d8ed225cbddcaa585b7c5dad7fbb7c80d00",
            "3fb06dc496f321749423792066f299205c433b815540c1f23f18368540401640",
        ),
    ] {
        let entry = hops
            .iter()
            .find(|entry| entry["version"] == hop)
            .unwrap_or_else(|| panic!("missing published hop {hop}"));
        assert_eq!(entry["github_commit"], commit, "commit drift at {hop}");
        assert_eq!(entry["github_tree"], tree, "tree drift at {hop}");
        assert_eq!(
            entry["tag_tarball_sha256"], tarball,
            "tarball drift at {hop}"
        );
        assert_eq!(entry["prerelease"], false, "prerelease drift at {hop}");
    }

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(decision["reuse_behavior"], "ollama.native-text-v1");
    assert_eq!(decision["raise_latest_qualified_to"], "0.34.2");
    assert_eq!(decision["keep_baseline"], "0.14.0");
    assert_eq!(decision["keep_claim_id"], "ollama.native-runtime-window-2");
    assert_eq!(decision["keep_exclusion_0_32_2"], true);
    assert_eq!(decision["keep_exclusion_0_32_10"], true);
    assert_eq!(decision["keep_allow_unverified"], true);
    assert_eq!(
        decision["qualified_hops"],
        serde_json::json!(["0.33.3", "0.34.0", "0.34.1", "0.34.2"])
    );
    assert_eq!(
        decision["decoder_tolerance"]["key"],
        "prompt_eval_cached_count"
    );
    assert_eq!(decision["decoder_tolerance"]["action"], "accept-and-ignore");
    assert_eq!(decision["synthetic_unverified_newer"], "0.34.3");
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
        protocol["selected_structs_identical_0_33_2_through_0_34_2_except_options_comment"],
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
    ] {
        assert_eq!(
            protocol["selected_struct_hashes"][key], digest,
            "selected struct drift at {key}"
        );
    }
    assert_eq!(
        protocol["types_go_file_hashes"]["v0.34.2"],
        protocol["types_go_file_hashes"]["v0.34.1"]
    );
    assert_eq!(
        protocol["metrics_struct_hashes"]["v0.33.3_through_v0.34.2"],
        "bbc09305511a40e65370507477be58246ae171095d73ff35935d5f0066b38cb4"
    );
    assert_eq!(
        protocol["selected_handler_hashes"]["show_handler_0_33_2_through_0_34_2"],
        "2077b9ce5d744a7ca84fbbacb710ddc67f1f0b378fdf8607dc70b4027ab70065"
    );
    assert_eq!(
        protocol["selected_handler_hashes"]["ps_handler_0_33_2_through_0_34_2"],
        "b4deea59f7a43c03ccc61f055dfed23539617151b2654367ac825ab3899d6075"
    );
    assert_eq!(
        protocol["selected_handler_hashes"]["chat_handler"]["v0.34.1_and_v0.34.2"],
        "37e6ee8d29f977ed560d8ec0319a1bd695da3b3d105baded2a655aeeadf62f57"
    );
    assert_eq!(protocol["new_public_selected_operation"], false);
    assert_eq!(protocol["decoder_corpus"], "ollama-native-v0.14.0-v0.32.1");
    assert_eq!(protocol["provider_prompt_sent"], false);
    assert_eq!(protocol["attached_server_started"], false);

    let unused = protocol["unused_deltas"]
        .as_array()
        .expect("unused deltas are an array");
    for extra in [
        "first-run-setup-cli-desktop",
        "ollama-apps-desktop-uri",
        "mlx-speculative-decoding-memory",
        "codex-desktop-proxy-route",
        "typical_p-deprecation-adapter-never-sends",
    ] {
        let present = unused.iter().any(|value| value == extra)
            || protocol["bounded_unmapped"]
                .as_array()
                .expect("bounded unmapped is an array")
                .iter()
                .any(|value| value == extra);
        assert!(present, "missing unused or bounded delta {extra}");
    }

    assert_eq!(OLLAMA_BASELINE_VERSION, "0.14.0");
    assert_eq!(OLLAMA_LATEST_QUALIFIED_VERSION, "0.34.4");
    let claim = ollama_runtime_claim();
    for version in [
        "0.14.0", "0.33.2", "0.33.3", "0.34.0", "0.34.1", "0.34.2", "0.34.3", "0.34.4",
    ] {
        assert!(matches!(
            claim.assess(&version_value(version)),
            InterfaceCompatibilityAssessment::Qualified(matched)
                if matched.support_status() == InterfaceSupportStatus::Maintained
        ));
    }
    assert!(matches!(
        claim.assess(&version_value("0.34.5")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert_eq!(
        ollama_runtime_binding("0.34.2")
            .expect("version binds")
            .axis()
            .as_str(),
        "ollama.runtime"
    );
}

#[test]
fn decoder_accepts_and_ignores_the_0_33_3_cached_count_key() {
    let counterexample = protocol_record("tolerance_counterexample_terminal_record");
    let mut decoder = ChatDecoder::new("m:8b");
    let events = decoder
        .push(format!("{counterexample}\n").as_bytes())
        .expect("decoder-tolerance accepts the additive 0.33.3 metrics key");
    assert_eq!(events.len(), 2);
    assert!(matches!(
        &events[0],
        NativeEvent::Finished(reason) if reason == "stop"
    ));
    assert!(matches!(
        &events[1],
        NativeEvent::Usage(usage)
            if usage.input_tokens() == Some(10) && usage.output_tokens() == Some(3)
    ));

    let mut decoder = ChatDecoder::new("m:8b");
    assert!(
        decoder
            .push(
                format!(
                    "{}\n",
                    counterexample.replace(
                        "\"prompt_eval_cached_count\":4,",
                        "\"prompt_eval_cached_count\":4,\"unknown_metrics_key\":1,"
                    )
                )
                .as_bytes()
            )
            .is_err(),
        "unknown extra keys still fail closed"
    );
}

fn protocol_record(key: &str) -> String {
    let protocol: Value =
        serde_json::from_str(PROTOCOL).expect("Ollama 0.34.2 protocol corpus is valid JSON");
    protocol[key]
        .as_str()
        .unwrap_or_else(|| panic!("protocol record {key} is a string"))
        .to_owned()
}

fn version_value(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}
