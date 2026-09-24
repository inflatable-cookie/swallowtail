use serde_json::Value;
use swallowtail_adapter_ollama::{
    OLLAMA_BASELINE_VERSION, OLLAMA_LATEST_QUALIFIED_VERSION, ollama_runtime_binding,
    ollama_runtime_claim,
    protocol::{ObservationBinding, OllamaModelCapability, Response, parse_model_detail},
};
use swallowtail_core::{
    AttachedModelTag, CatalogTimestamp, ConfiguredInstanceId, ExecutionHostId,
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
    ModelManifestDigest,
};

const IDENTITY: &str = include_str!("fixtures/ollama-0.34.4/identity.json");
const PROTOCOL: &str = include_str!("fixtures/ollama-0.34.4/protocol.json");

#[test]
fn identity_freezes_0_34_3_and_0_34_4_and_names_compatible_extension() {
    let identity: Value =
        serde_json::from_str(IDENTITY).expect("Ollama 0.34.4 identity corpus is valid JSON");
    let protocol: Value =
        serde_json::from_str(PROTOCOL).expect("Ollama 0.34.4 protocol corpus is valid JSON");

    assert_eq!(identity["axis"], "ollama.runtime");
    assert_eq!(identity["not_ollama_cloud"], true);
    assert_eq!(identity["host"]["client_version"], "0.33.3");
    assert_eq!(identity["host"]["no_runtime_reachable"], true);
    assert_eq!(identity["host"]["host_not_mutated"], true);
    assert_eq!(identity["official"]["version"], "0.34.4");
    assert_eq!(identity["official"]["prerelease"], false);
    assert_eq!(
        identity["official"]["github_commit"],
        "b2da9e468af2479058ae18c6d908ed29de410684"
    );
    assert_eq!(
        identity["official"]["github_tree"],
        "525c37066242cc8feb9f7d909927270b168c6a7c"
    );
    assert_eq!(
        identity["official"]["tag_tarball_sha256"],
        "c265dfff78cd2fe909f9f9fa0bb21b1e34dfcd354e1bfa13ed8eac2cfce90940"
    );
    assert_eq!(
        identity["published_stables_from_previous_ceiling"],
        serde_json::json!(["0.34.3", "0.34.4"])
    );
    assert_eq!(identity["previous_ceiling"], "0.34.2");
    assert_eq!(identity["unpublished_next"], "0.34.5");
    assert_eq!(identity["research_342_reproduced"], true);
    assert_eq!(
        identity["github_prerelease_plain_versions"],
        serde_json::json!(["0.32.2", "0.32.10"])
    );
    assert_eq!(
        identity["ignored_prerelease_after_previous_ceiling"],
        serde_json::json!(["0.34.3-rc1"])
    );
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "0.34.2"
    );

    let hops = identity["published_hops"]
        .as_array()
        .expect("published hops are an array");
    assert_eq!(hops.len(), 2);
    for (hop, commit, tree, tarball) in [
        (
            "0.34.3",
            "6383a0fa9cbf97494b847226e189f6e36b401a08",
            "45bb0e750cc7db55ee25b73e039af3026e683a3d",
            "89aecca954feb03ff39c781ff321b0055349b82720b0c3832a2336e0ea9da029",
        ),
        (
            "0.34.4",
            "b2da9e468af2479058ae18c6d908ed29de410684",
            "525c37066242cc8feb9f7d909927270b168c6a7c",
            "c265dfff78cd2fe909f9f9fa0bb21b1e34dfcd354e1bfa13ed8eac2cfce90940",
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

    let previous = &identity["previous_ceiling_identity"];
    assert_eq!(
        previous["tag_tarball_sha256"],
        "3fb06dc496f321749423792066f299205c433b815540c1f23f18368540401640"
    );
    assert_eq!(
        previous["types_go_sha256"],
        protocol["types_go_file_hashes"]["v0.34.2"]
    );
    assert_eq!(
        previous["routes_go_sha256"],
        protocol["routes_go_file_hashes"]["v0.34.2"]
    );

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(decision["reuse_behavior"], "ollama.native-text-v1");
    assert_eq!(decision["raise_latest_qualified_to"], "0.34.4");
    assert_eq!(decision["keep_baseline"], "0.14.0");
    assert_eq!(decision["keep_claim_id"], "ollama.native-runtime-window-2");
    assert_eq!(decision["keep_exclusion_0_32_2"], true);
    assert_eq!(decision["keep_exclusion_0_32_10"], true);
    assert_eq!(decision["keep_allow_unverified"], true);
    assert_eq!(
        decision["qualified_hops"],
        serde_json::json!(["0.34.3", "0.34.4"])
    );
    assert_eq!(decision["synthetic_unverified_newer"], "0.34.5");
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
    assert_eq!(protocol["research_342_0_34_2_hashes_reproduced"], true);
    assert_eq!(
        protocol["types_go_byte_identical_0_34_3_through_0_34_4"],
        true
    );
    assert_eq!(
        protocol["types_go_file_hashes"]["v0.34.4"],
        protocol["types_go_file_hashes"]["v0.34.3"]
    );
    assert_eq!(
        protocol["selected_struct_hashes"]["chat_response_0_34_2_through_0_34_4"],
        "f9deac910407655aa7ecbded4820f8de5d404bb2f49e261b1a64b7db56f2b936"
    );
    assert_eq!(
        protocol["selected_struct_hashes"]["show_response"]["v0.34.3_and_v0.34.4"],
        "7e6f60b93930ae251348000e61e6f3b7ea650826759323687292ad05fa58271c"
    );
    assert_eq!(
        protocol["selected_handler_hashes"]["show_handler_0_34_2_through_0_34_4"],
        "2077b9ce5d744a7ca84fbbacb710ddc67f1f0b378fdf8607dc70b4027ab70065"
    );
    assert_eq!(
        protocol["selected_handler_hashes"]["chat_handler"]["v0.34.4"],
        "12314badb6cb4bc0d91459dc48d0a065d32d636c8724d750d811ee51b713adf4"
    );
    assert_eq!(protocol["new_public_selected_operation"], false);
    assert_eq!(protocol["decoder_corpus"], "ollama-native-v0.14.0-v0.32.1");
    assert_eq!(protocol["provider_prompt_sent"], false);
    assert_eq!(protocol["attached_server_started"], false);

    let unused = protocol["unused_deltas"]
        .as_array()
        .expect("unused deltas are an array");
    let bounded = protocol["bounded_unmapped"]
        .as_array()
        .expect("bounded unmapped is an array");
    for extra in [
        "show-thinking-advertisement-catalog-decoder-ignores-unknown-keys",
        "chat-request-think-comment-only-json-tag-unchanged",
        "chat-handler-single-pass-format-on-thinking-models",
        "getexistingname-case-insensitive-prefix-match",
        "mlx-llamacpp-xgrammar-updates",
        "huggingface-pull-fix",
    ] {
        let present =
            unused.iter().any(|value| value == extra) || bounded.iter().any(|value| value == extra);
        assert!(present, "missing unused or bounded delta {extra}");
    }

    assert_eq!(OLLAMA_BASELINE_VERSION, "0.14.0");
    assert_eq!(OLLAMA_LATEST_QUALIFIED_VERSION, "0.34.4");
    let claim = ollama_runtime_claim();
    for version in ["0.14.0", "0.34.2", "0.34.3", "0.34.4"] {
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
        ollama_runtime_binding("0.34.4")
            .expect("version binds")
            .axis()
            .as_str(),
        "ollama.runtime"
    );
}

#[test]
fn catalog_decoder_ignores_the_0_34_3_show_thinking_advertisement() {
    let body = protocol_record("show_thinking_counterexample");
    let detail = parse_model_detail(
        &Response {
            status: 200,
            body: body.into_bytes(),
        },
        &observation_binding(),
        AttachedModelTag::new("fixture-model:8b").expect("tag is valid"),
        ModelManifestDigest::new(
            "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        )
        .expect("digest is valid"),
    )
    .expect("catalog decoder accepts additive show thinking advertisement");
    assert!(detail.supports(OllamaModelCapability::Thinking));
    assert!(detail.supports(OllamaModelCapability::Completion));
}

fn protocol_record(key: &str) -> String {
    let protocol: Value =
        serde_json::from_str(PROTOCOL).expect("Ollama 0.34.4 protocol corpus is valid JSON");
    protocol[key]
        .as_str()
        .unwrap_or_else(|| panic!("protocol record {key} is a string"))
        .to_owned()
}

fn version_value(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}

fn observation_binding() -> ObservationBinding {
    ObservationBinding {
        instance_id: ConfiguredInstanceId::new("fixture.ollama").unwrap(),
        execution_host_id: ExecutionHostId::new("fixture.host").unwrap(),
        runtime_version: ollama_runtime_binding("0.34.4").expect("fixture Ollama version is valid"),
        observed_at: CatalogTimestamp::new(1_700_000_000, 0).unwrap(),
    }
}
