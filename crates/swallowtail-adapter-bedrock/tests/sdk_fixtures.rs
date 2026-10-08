use aws_credential_types::Credentials;
use aws_sdk_bedrockruntime::config::{BehaviorVersion, Region};
use aws_sdk_bedrockruntime::operation::converse_stream::ConverseStreamError;
use aws_sdk_bedrockruntime::types::error::{ModelStreamErrorException, ThrottlingException};
use aws_sdk_bedrockruntime::types::{
    ContentBlockDelta, ContentBlockDeltaEvent, ContentBlockStopEvent, ConversationRole,
    ConverseStreamMetadataEvent, ConverseStreamMetrics, ConverseStreamOutput, MessageStartEvent,
    MessageStopEvent, StopReason, TokenUsage as AwsTokenUsage,
};
use aws_smithy_http_client::test_util::capture_request;
use serde_json::{Value, json};
use swallowtail_adapter_bedrock::{
    DecodeFailure, ProviderFailureKind, SDK_CRATE, SDK_VERSION, SERVICE_API, StopKind,
    StreamDecoder, StreamUpdate, TokenUsage, classify_converse_failure,
};

const PROTOCOL: &str = include_str!("fixtures/bedrock-runtime-1.148.0/protocol.json");

#[test]
fn manifest_binds_sdk_access_attempt_and_exclusions() {
    let fixture: Value = serde_json::from_str(PROTOCOL).expect("fixture manifest is valid JSON");
    assert_eq!(fixture["sdk_crate"], SDK_CRATE);
    assert_eq!(fixture["sdk_version"], SDK_VERSION);
    assert_eq!(fixture["service_api"], SERVICE_API);
    assert_eq!(
        fixture["transport"],
        "in_process_rust_sdk_typed_eventstream"
    );
    assert_eq!(fixture["endpoint_audience"], "bedrock-runtime");
    assert_eq!(fixture["authentication"]["default_chain_allowed"], false);
    assert_eq!(
        fixture["authentication"]["secret_exposed_by_runtime_lease"],
        false
    );
    assert_eq!(fixture["metering"], "cloud_account_billing");
    assert_eq!(fixture["support_authority"], "provider_supported");
    assert_eq!(fixture["client_configuration"]["retry_max_attempts"], 1);
    assert_eq!(fixture["failure"]["automatic_retry"], false);
    assert_eq!(fixture["stream"]["bounded_projection"], true);
    assert!(
        fixture["excluded"]
            .as_array()
            .expect("exclusions are an array")
            .contains(&json!("model_catalogue"))
    );
}

#[test]
fn typed_sdk_success_stream_preserves_text_stop_and_usage_order() {
    let mut decoder = StreamDecoder::default();
    let events = [
        ConverseStreamOutput::MessageStart(
            MessageStartEvent::builder()
                .role(ConversationRole::Assistant)
                .build()
                .expect("message start fixture builds"),
        ),
        ConverseStreamOutput::ContentBlockDelta(
            ContentBlockDeltaEvent::builder()
                .content_block_index(0)
                .delta(ContentBlockDelta::Text("hello ".to_owned()))
                .build()
                .expect("delta fixture builds"),
        ),
        ConverseStreamOutput::ContentBlockDelta(
            ContentBlockDeltaEvent::builder()
                .content_block_index(0)
                .delta(ContentBlockDelta::Text("world".to_owned()))
                .build()
                .expect("delta fixture builds"),
        ),
        ConverseStreamOutput::ContentBlockStop(
            ContentBlockStopEvent::builder()
                .content_block_index(0)
                .build()
                .expect("stop fixture builds"),
        ),
        ConverseStreamOutput::MessageStop(
            MessageStopEvent::builder()
                .stop_reason(StopReason::EndTurn)
                .build()
                .expect("message stop fixture builds"),
        ),
        ConverseStreamOutput::Metadata(
            ConverseStreamMetadataEvent::builder()
                .usage(
                    AwsTokenUsage::builder()
                        .input_tokens(7)
                        .output_tokens(2)
                        .total_tokens(9)
                        .build()
                        .expect("usage fixture builds"),
                )
                .metrics(
                    ConverseStreamMetrics::builder()
                        .latency_ms(12)
                        .build()
                        .expect("metrics fixture builds"),
                )
                .build(),
        ),
    ];
    let updates = events
        .into_iter()
        .map(|event| decoder.push(event).expect("fixture event is accepted"))
        .collect::<Vec<_>>();
    assert_eq!(
        updates,
        [
            StreamUpdate::MessageStarted,
            StreamUpdate::TextDelta("hello ".to_owned()),
            StreamUpdate::TextDelta("world".to_owned()),
            StreamUpdate::ContentBlockStopped,
            StreamUpdate::MessageStopped(StopKind::EndTurn),
            StreamUpdate::Usage(TokenUsage {
                input: 7,
                output: 2,
                total: 9,
            }),
        ]
    );
    assert!(decoder.is_complete());
}

#[test]
fn unknown_and_unsupported_sdk_semantics_fail_closed() {
    let mut decoder = StreamDecoder::default();
    decoder
        .push(ConverseStreamOutput::MessageStart(
            MessageStartEvent::builder()
                .role(ConversationRole::Assistant)
                .build()
                .expect("message start fixture builds"),
        ))
        .expect("start is accepted");
    decoder
        .push(ConverseStreamOutput::ContentBlockStop(
            ContentBlockStopEvent::builder()
                .content_block_index(0)
                .build()
                .expect("content stop fixture builds"),
        ))
        .expect("content stop is accepted");
    assert_eq!(
        decoder.push(ConverseStreamOutput::MessageStop(
            MessageStopEvent::builder()
                .stop_reason(StopReason::from("future_stop_reason"))
                .build()
                .expect("unknown stop reason fixture builds"),
        )),
        Err(DecodeFailure::UnknownSdkVariant)
    );

    let mut decoder = started_and_content_stopped();
    assert_eq!(
        decoder.push(ConverseStreamOutput::MessageStop(
            MessageStopEvent::builder()
                .stop_reason(StopReason::ToolUse)
                .build()
                .expect("unsupported stop reason fixture builds"),
        )),
        Err(DecodeFailure::UnsupportedSemanticEvent)
    );
}

#[test]
fn missing_fields_and_order_drift_fail_closed() {
    let mut decoder = StreamDecoder::default();
    let delta = ContentBlockDeltaEvent::builder()
        .content_block_index(0)
        .build()
        .expect("missing optional union fixture builds");
    assert_eq!(
        decoder.push(ConverseStreamOutput::ContentBlockDelta(delta)),
        Err(DecodeFailure::EventOutOfOrder)
    );

    decoder
        .push(ConverseStreamOutput::MessageStart(
            MessageStartEvent::builder()
                .role(ConversationRole::Assistant)
                .build()
                .expect("message start fixture builds"),
        ))
        .expect("start is accepted");
    let delta = ContentBlockDeltaEvent::builder()
        .content_block_index(0)
        .build()
        .expect("missing optional union fixture builds");
    assert_eq!(
        decoder.push(ConverseStreamOutput::ContentBlockDelta(delta)),
        Err(DecodeFailure::MissingRequiredField)
    );
}

fn started_and_content_stopped() -> StreamDecoder {
    let mut decoder = StreamDecoder::default();
    decoder
        .push(ConverseStreamOutput::MessageStart(
            MessageStartEvent::builder()
                .role(ConversationRole::Assistant)
                .build()
                .expect("message start fixture builds"),
        ))
        .expect("start is accepted");
    decoder
        .push(ConverseStreamOutput::ContentBlockStop(
            ContentBlockStopEvent::builder()
                .content_block_index(0)
                .build()
                .expect("content stop fixture builds"),
        ))
        .expect("content stop is accepted");
    decoder
}

#[test]
fn provider_failures_are_classified_without_preserving_raw_messages() {
    let throttled = ConverseStreamError::ThrottlingException(
        ThrottlingException::builder()
            .message("raw throttle detail must stay private")
            .build(),
    );
    let stream = ConverseStreamError::ModelStreamErrorException(
        ModelStreamErrorException::builder()
            .message("raw stream detail must stay private")
            .build(),
    );
    assert_eq!(
        classify_converse_failure(&throttled),
        ProviderFailureKind::RateLimited
    );
    assert_eq!(
        classify_converse_failure(&stream),
        ProviderFailureKind::ProviderFailed
    );
    assert!(!format!("{:?}", classify_converse_failure(&throttled)).contains("raw throttle"));
    assert!(!format!("{:?}", classify_converse_failure(&stream)).contains("raw stream"));
}

#[tokio::test]
async fn frozen_converse_stream_request_is_serialized_by_the_qualified_sdk() {
    let (capture_client, captured_request) = capture_request(None);
    let config = aws_sdk_bedrockruntime::Config::builder()
        .behavior_version(BehaviorVersion::latest())
        .region(Region::new("us-east-1"))
        .endpoint_url("https://bedrock-runtime.us-east-1.amazonaws.com")
        .credentials_provider(Credentials::new(
            "test-access-key",
            "test-secret-key",
            None,
            None,
            "swallowtail-test",
        ))
        .retry_config(
            aws_sdk_bedrockruntime::config::retry::RetryConfig::standard().with_max_attempts(1),
        )
        .http_client(capture_client)
        .build();
    let client = aws_sdk_bedrockruntime::Client::from_conf(config);
    let message = aws_sdk_bedrockruntime::types::Message::builder()
        .role(ConversationRole::User)
        .content(aws_sdk_bedrockruntime::types::ContentBlock::Text(
            "hello".to_owned(),
        ))
        .build()
        .expect("user message fixture builds");
    let inference = aws_sdk_bedrockruntime::types::InferenceConfiguration::builder()
        .max_tokens(7)
        .build();

    let _ = client
        .converse_stream()
        .model_id("test-model")
        .messages(message)
        .inference_config(inference)
        .send()
        .await;

    let request = captured_request.expect_request();
    let fixture: Value = serde_json::from_str(PROTOCOL).expect("protocol fixture is valid JSON");
    let request_fixture = &fixture["request"];
    assert_eq!(
        request.method(),
        request_fixture["method"]
            .as_str()
            .expect("fixture method is text")
    );
    let expected_path = request_fixture["path"]
        .as_str()
        .expect("fixture path is text")
        .replace("{modelId}", "test-model");
    assert!(
        request.uri().ends_with(&expected_path),
        "captured URI did not end in the frozen path"
    );
    let body = request
        .body()
        .bytes()
        .expect("captured request body is in memory");
    let body: Value = serde_json::from_slice(body).expect("request body is JSON");
    assert_eq!(body, request_fixture["body"]);
}
