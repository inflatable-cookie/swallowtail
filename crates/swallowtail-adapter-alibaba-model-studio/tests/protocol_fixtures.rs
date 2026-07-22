use serde_json::{Value, json};
use swallowtail_adapter_alibaba_model_studio::{
    DeletionKind, Method, ProviderEvent, ResponseStream, SseDecoder, TurnOptions, WireRequest,
    parse_deletion, parse_inventory, parse_provider_failure, parse_request_correlation,
};
use swallowtail_core::OwnedRemoteResourceKind;
use swallowtail_runtime::{
    CleanupOutcome, OperationContent, ProviderCancellationOutcome, RemoteResourceDeletionOutcome,
    TerminalOutcome, TerminalStatus,
};

mod support;

use support::{bytes, conversation, frames_from_chunks, json_fixture};

#[test]
fn manifest_and_requests_freeze_the_exact_workspace_route() {
    let protocol: Value = json_fixture("protocol.json");
    assert_eq!(protocol["evidence_date"], "2026-07-22");
    assert_eq!(protocol["region"], "ap-southeast-1");
    assert_eq!(protocol["deployment_scope"], "International");
    assert_eq!(protocol["model"], "qwen3.7-plus-2026-05-26");
    assert_eq!(protocol["maximum_turns"], 2);
    assert_eq!(protocol["maximum_concurrency"], 1);
    assert_eq!(protocol["native_response_cancel"], false);

    let create = WireRequest::create_conversation();
    assert_eq!(create.method(), Method::Post);
    assert_eq!(create.path(), "/compatible-mode/v1/conversations");
    assert_eq!(create.body(), Some(&json!({})));

    let conversation = conversation();
    let input = OperationContent::new("First fixture input").expect("content is valid");
    let response = WireRequest::response(&conversation, &input, &TurnOptions::frozen())
        .expect("frozen response request is valid");
    assert_eq!(response.method(), Method::Post);
    assert_eq!(response.path(), "/compatible-mode/v1/responses");
    assert_eq!(
        response.body(),
        Some(&json!({
            "model": "qwen3.7-plus-2026-05-26",
            "input": "First fixture input",
            "conversation": "conv_fixture_01",
            "stream": true,
            "store": false,
            "reasoning": {"effort": "none"}
        }))
    );
    assert!(!response.session_cache_enabled());
    let debug = format!("{response:?}");
    assert!(!debug.contains("First fixture input"));
    assert!(!debug.contains("conv_fixture_01"));
}

#[test]
fn unsupported_turn_fields_reject_before_a_wire_request_exists() {
    let variants = [
        TurnOptions::frozen().with_model("qwen3.7-plus"),
        TurnOptions::frozen().with_stream(false),
        TurnOptions::frozen().with_store(true),
        TurnOptions::frozen().with_reasoning_effort("low"),
        TurnOptions::frozen().with_tools(1),
        TurnOptions::frozen().with_session_cache(true),
        TurnOptions::frozen().with_background(true),
        TurnOptions::frozen().with_retries(1),
        TurnOptions::frozen().with_previous_response(true),
        TurnOptions::frozen().with_maximum_output_tokens(128),
        TurnOptions::frozen().with_fallback(true),
    ];
    let input = OperationContent::new("fixture input").expect("content is valid");
    for options in variants {
        let error = WireRequest::response(&conversation(), &input, &options)
            .expect_err("unsupported input must reject");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.alibaba_model_studio.unsupported_input"
        );
    }
}

#[test]
fn inventory_and_deletion_corpus_keep_item_and_conversation_truth_separate() {
    let inventory = parse_inventory(bytes("items.json")).expect("inventory is complete");
    let items: Vec<_> = inventory.items().collect();
    assert_eq!(items.len(), 4);
    assert!(format!("{:?}", items[0]).contains("<redacted>"));
    assert!(parse_inventory(bytes("items-incomplete.json")).is_err());

    let item = parse_deletion(
        bytes("delete-item.json"),
        "msg_input_01",
        DeletionKind::ConversationItem,
    )
    .expect("item deletion confirms");
    let conversation_deletion = parse_deletion(
        bytes("delete-conversation.json"),
        "conv_fixture_01",
        DeletionKind::Conversation,
    )
    .expect("conversation deletion confirms");
    assert_eq!(item.kind(), DeletionKind::ConversationItem);
    assert_eq!(conversation_deletion.kind(), DeletionKind::Conversation);
    assert!(
        parse_deletion(
            bytes("delete-conversation.json"),
            "conv_fixture_01",
            DeletionKind::ConversationItem,
        )
        .is_err()
    );

    let conversation_ref = conversation();
    let mut cleanup = vec![WireRequest::list_items(&conversation_ref)];
    cleanup.extend(
        items
            .iter()
            .map(|item| WireRequest::delete_item(&conversation_ref, item)),
    );
    cleanup.push(WireRequest::delete_conversation(&conversation_ref));
    assert!(cleanup[0].path().ends_with("/items?limit=100&order=asc"));
    assert!(
        cleanup[1..cleanup.len() - 1]
            .iter()
            .all(|request| request.path().contains("/items/msg_"))
    );
    assert_eq!(
        cleanup.last().expect("conversation deletion exists").path(),
        "/compatible-mode/v1/conversations/conv_fixture_01"
    );
}

#[test]
fn fragmented_stream_preserves_sequence_output_model_usage_and_unknowns() {
    let frames = frames_from_chunks(bytes("success.sse"), 13).expect("SSE corpus decodes");
    let mut stream = ResponseStream::default();
    let events: Vec<_> = frames
        .iter()
        .map(|frame| stream.apply(frame).expect("event is valid"))
        .collect();
    assert_eq!(events.len(), 9);
    match events.last().expect("terminal event exists") {
        ProviderEvent::Completed { output, usage, .. } => {
            assert_eq!(output.as_str(), "Hello world.");
            assert_eq!(usage.input_tokens(), Some(12));
            assert_eq!(usage.output_tokens(), Some(3));
        }
        other => panic!("unexpected terminal event: {other:?}"),
    }

    let unknown = frames_from_chunks(bytes("unknown.sse"), 5).expect("unknown corpus decodes");
    let mut stream = ResponseStream::default();
    assert!(matches!(
        stream
            .apply(&unknown[1])
            .expect_err("missing sequence zero rejects")
            .diagnostic()
            .code(),
        "swallowtail.alibaba_model_studio.protocol_invalid"
    ));
    let mut stream = ResponseStream::default();
    stream.apply(&unknown[0]).expect("created event passes");
    assert_eq!(
        stream
            .apply(&unknown[1])
            .expect("unknown event is preserved"),
        ProviderEvent::Unknown("response.provider_metadata".to_owned())
    );
}

#[test]
fn reasoning_sequence_drift_disconnect_and_provider_failure_fail_safely() {
    for name in ["reasoning.sse", "sequence-drift.sse"] {
        let frames = frames_from_chunks(bytes(name), 17).expect("SSE records decode");
        let mut stream = ResponseStream::default();
        stream.apply(&frames[0]).expect("created event passes");
        assert!(stream.apply(&frames[1]).is_err());
    }

    let mut decoder = SseDecoder::default();
    assert!(
        decoder
            .push(bytes("disconnect.sse"))
            .expect("partial data buffers")
            .is_empty()
    );
    assert!(decoder.finish().is_err());

    let provider = parse_provider_failure(bytes("provider-error.json"))
        .expect("provider envelope is recognized");
    let public = provider.to_string();
    assert!(!public.contains("workspace_fixture"));
    assert!(!public.contains("sk-fixture-secret"));
    let request =
        parse_request_correlation(bytes("headers.json")).expect("request correlation is bounded");
    assert!(!format!("{request:?}").contains("req_fixture_01"));
}

#[test]
fn cancellation_race_keeps_remote_stop_and_both_deletions_unconfirmed() {
    let race: Value = json_fixture("cleanup-race.json");
    assert_eq!(race["late_remote_mutation_possible"], true);
    assert_eq!(
        race["local_turn_statuses"],
        json!(["cancelled", "timed-out"])
    );
    for status in [TerminalStatus::Cancelled, TerminalStatus::TimedOut] {
        let outcome = TerminalOutcome::new(
            status,
            CleanupOutcome::Degraded(swallowtail_core::SafeDiagnostic::new(
                "fixture.remote_cleanup_unconfirmed",
                "Remote cleanup could not be confirmed",
            )),
        )
        .with_provider_cancellation(ProviderCancellationOutcome::Unconfirmed)
        .with_remote_resource_deletion(
            OwnedRemoteResourceKind::ConversationItems,
            RemoteResourceDeletionOutcome::Unconfirmed,
        )
        .with_remote_resource_deletion(
            OwnedRemoteResourceKind::Conversation,
            RemoteResourceDeletionOutcome::Unconfirmed,
        );
        assert_eq!(
            outcome.provider_cancellation(),
            Some(ProviderCancellationOutcome::Unconfirmed)
        );
        assert_eq!(
            outcome.remote_resource_deletion(OwnedRemoteResourceKind::ConversationItems),
            Some(RemoteResourceDeletionOutcome::Unconfirmed)
        );
        assert_eq!(
            outcome.remote_resource_deletion(OwnedRemoteResourceKind::Conversation),
            Some(RemoteResourceDeletionOutcome::Unconfirmed)
        );
    }
}
