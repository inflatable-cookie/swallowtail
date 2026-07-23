macro_rules! fixture_text {
    ($name:literal) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/ollama-native-v0.14.0-v0.32.1/",
            $name
        ))
    };
}

macro_rules! fixture_bytes {
    ($name:literal) => {
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/ollama-native-v0.14.0-v0.32.1/",
            $name
        ))
    };
}

use super::*;
use serde_json::Value;
use swallowtail_core::{
    AttachedModelObservationScope, AttachedModelTag, CatalogTimestamp, ConfiguredInstanceId,
    ExecutionHostId, InterfaceVersion, ModelManifestDigest,
};

const ROOT: &str = "../../tests/fixtures/ollama-native-v0.14.0-v0.32.1";
const PROTOCOL: &str = fixture_text!("protocol.json");
const TAGS: &[u8] = fixture_bytes!("tags.json");
const PS: &[u8] = fixture_bytes!("ps.json");
const SHOW: &[u8] = fixture_bytes!("show.json");
const CHAT_SUCCESS: &[u8] = fixture_bytes!("chat-success.ndjson");
const CHAT_ERROR: &[u8] = fixture_bytes!("chat-error.ndjson");
const CHAT_MALFORMED: &[u8] = fixture_bytes!("chat-malformed.ndjson");
const CHAT_DISCONNECT: &[u8] = fixture_bytes!("chat-disconnect.ndjson");
const CHAT_UNSUPPORTED: &[u8] = fixture_bytes!("chat-unsupported.ndjson");
const TAGS_CLOUD_DRIFT: &[u8] = fixture_bytes!("tags-cloud-drift.json");
const HTTP_ERROR: &[u8] = fixture_bytes!("http-error.json");

#[test]
fn manifest_binds_every_tagged_source_to_one_text_behavior() {
    let manifest: Value = serde_json::from_str(PROTOCOL).expect("manifest parses");
    assert_eq!(manifest["fixture_schema"], 1);
    assert_eq!(
        manifest["claim_id"],
        crate::selection::ollama_runtime_claim().id().as_str()
    );
    assert_eq!(manifest["behavior_revision"], "ollama.native-text-v1");
    assert_eq!(
        manifest["qualification_points"].as_array().unwrap().len(),
        4
    );
    assert!(
        manifest["qualification_points"]
            .as_array()
            .unwrap()
            .iter()
            .all(|point| point["corpus"] == "native-text-v1")
    );
    assert!(ROOT.ends_with("ollama-native-v0.14.0-v0.32.1"));
}

#[test]
fn exact_version_codec_accepts_only_the_closed_qualified_window() {
    for version in ["0.14.0", "0.18.0", "0.30.0", "0.32.1"] {
        let name = format!("version-{version}.json");
        let body = std::fs::read(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("tests/fixtures/ollama-native-v0.14.0-v0.32.1")
                .join(name),
        )
        .expect("fixture reads");
        let binding = parse_version(&response(200, &body)).expect("version is qualified");
        assert_eq!(binding.version().as_str(), version);
    }
    for body in [
        fixture_bytes!("version-below.json").as_slice(),
        fixture_bytes!("version-above.json").as_slice(),
        fixture_bytes!("version-prerelease.json").as_slice(),
        fixture_bytes!("version-malformed.json").as_slice(),
    ] {
        let error = parse_version(&response(200, body)).expect_err("version fails closed");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.ollama.version_unsupported"
        );
    }
}

#[test]
fn inventory_scopes_preserve_runtime_identity_without_creating_routes() {
    let binding = observation_binding();
    let installed = parse_inventory(
        &response(200, TAGS),
        AttachedModelObservationScope::InstalledInventory,
        &binding,
    )
    .expect("installed inventory parses");
    let running = parse_inventory(
        &response(200, PS),
        AttachedModelObservationScope::RunningInventory,
        &binding,
    )
    .expect("running inventory parses");
    assert_eq!(installed.len(), 1);
    assert_eq!(
        installed[0].scope(),
        AttachedModelObservationScope::InstalledInventory
    );
    assert_eq!(
        running[0].scope(),
        AttachedModelObservationScope::RunningInventory
    );
    assert_eq!(installed[0].model_tag().as_str(), "fixture-model:8b");
    assert_eq!(
        installed[0].manifest_digest().unwrap().as_str(),
        digest().as_str()
    );
    assert_eq!(installed[0].instance_id(), &binding.instance_id);
    assert_eq!(installed[0].execution_host_id(), &binding.execution_host_id);
}

#[test]
fn selected_detail_uses_prior_digest_and_rejects_cloud_or_non_text_semantics() {
    let binding = observation_binding();
    let detail = parse_model_detail(&response(200, SHOW), &binding, model_tag(), digest())
        .expect("selected detail parses");
    assert_eq!(
        detail.scope(),
        AttachedModelObservationScope::SelectedModelDetail
    );
    assert_eq!(detail.manifest_digest().unwrap(), &digest());

    let error = parse_inventory(
        &response(200, TAGS_CLOUD_DRIFT),
        AttachedModelObservationScope::InstalledInventory,
        &binding,
    )
    .expect_err("cloud model is outside local attached subset");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.ollama.semantics_unsupported"
    );
}

#[test]
fn native_requests_contain_only_the_selected_text_subset() {
    assert_eq!(Request::version().path, "/api/version");
    assert_eq!(Request::installed_models().path, "/api/tags");
    assert_eq!(Request::running_models().path, "/api/ps");
    assert_eq!(Request::show("fixture-model:8b").unwrap().path, "/api/show");
    assert_json_eq(
        Request::show("fixture-model:8b").unwrap().body.unwrap(),
        fixture_text!("show-request.json"),
    );
    let chat = Request::chat("fixture-model:8b", "Fixture prompt", 8).expect("chat encodes");
    assert_eq!(chat.path, "/api/chat");
    assert_json_eq(chat.body.unwrap(), fixture_text!("chat-request.json"));
    assert!(Request::chat("fixture-model:8b", "Fixture prompt", 0).is_err());
}

#[test]
fn native_stream_preserves_output_finish_and_usage_order() {
    let events = decode(CHAT_SUCCESS).expect("success stream decodes");
    assert_eq!(
        events,
        vec![
            NativeEvent::OutputDelta("Fixture ".to_owned()),
            NativeEvent::OutputDelta("output".to_owned()),
            NativeEvent::Finished("length".to_owned()),
            NativeEvent::Usage(swallowtail_runtime::TokenUsage::new(Some(12), Some(2))),
        ]
    );
}

#[test]
fn midstream_error_is_provider_failure_after_http_success() {
    let events = decode(CHAT_ERROR).expect("provider error record decodes");
    assert_eq!(
        events,
        vec![
            NativeEvent::OutputDelta("partial".to_owned()),
            NativeEvent::ProviderFailed,
        ]
    );
}

#[test]
fn malformed_disconnect_unsupported_and_http_errors_are_safe() {
    let malformed = decode(CHAT_MALFORMED).expect_err("malformed record fails");
    assert_eq!(
        malformed.diagnostic().code(),
        "swallowtail.ollama.protocol_invalid"
    );
    let disconnect = decode(CHAT_DISCONNECT).expect_err("missing terminal fails");
    assert_eq!(
        disconnect.diagnostic().code(),
        "swallowtail.ollama.stream_disconnected"
    );
    let unsupported = decode(CHAT_UNSUPPORTED).expect_err("thinking fails closed");
    assert_eq!(
        unsupported.diagnostic().code(),
        "swallowtail.ollama.semantics_unsupported"
    );
    let provider = require_success(&response(404, HTTP_ERROR), "model detail")
        .expect_err("HTTP failure remains provider failure");
    assert_eq!(
        provider.diagnostic().code(),
        "swallowtail.ollama.provider_failed"
    );
    for error in [malformed, disconnect, unsupported, provider] {
        let diagnostic = error.to_string();
        assert!(!diagnostic.contains("synthetic provider failure"));
        assert!(!diagnostic.contains("cloud.example.invalid"));
        assert!(!diagnostic.contains("Fixture prompt"));
    }
}

fn decode(bytes: &[u8]) -> Result<Vec<NativeEvent>, swallowtail_runtime::RuntimeFailure> {
    let split = bytes.len() / 2;
    let mut decoder = ChatDecoder::new("fixture-model:8b");
    let mut events = decoder.push(&bytes[..split])?;
    events.extend(decoder.push(&bytes[split..])?);
    decoder.finish()?;
    Ok(events)
}

fn observation_binding() -> ObservationBinding {
    ObservationBinding {
        instance_id: ConfiguredInstanceId::new("fixture.ollama").unwrap(),
        execution_host_id: ExecutionHostId::new("fixture.host").unwrap(),
        runtime_version: crate::selection::ollama_runtime_binding("0.30.0"),
        observed_at: CatalogTimestamp::new(1_700_000_000, 0).unwrap(),
    }
}

fn model_tag() -> AttachedModelTag {
    AttachedModelTag::new("fixture-model:8b").unwrap()
}

fn digest() -> ModelManifestDigest {
    ModelManifestDigest::new(
        "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    )
    .unwrap()
}

fn response(status: u32, body: &[u8]) -> Response {
    Response {
        status,
        body: body.to_vec(),
    }
}

fn assert_json_eq(actual: Vec<u8>, expected: &str) {
    assert_eq!(
        serde_json::from_slice::<Value>(&actual).expect("actual JSON parses"),
        serde_json::from_str::<Value>(expected).expect("fixture JSON parses")
    );
}

#[test]
fn selected_fixture_versions_are_semantic_values() {
    for version in ["0.14.0", "0.18.0", "0.30.0", "0.32.1"] {
        assert!(!InterfaceVersion::new(version).unwrap().as_str().is_empty());
    }
}
