use super::{ClientEvent, RealtimeServerEvent, RealtimeServerStream, parse_server_event};
use serde_json::Value;
use std::num::{NonZeroU16, NonZeroU32, NonZeroU64};
use swallowtail_core::{AudioEncoding, MediaDirection, MediaFormat, RealtimeMediaConfig};
use swallowtail_runtime::{
    MediaChunk, MediaStreamId, ProviderCancellationOutcome, RuntimeSessionId,
};

#[test]
fn client_events_match_the_frozen_manual_audio_subset() {
    let expected = json_lines(include_bytes!(concat!(
        "../../tests/fixtures/openai-realtime-2026-07-22/",
        "client-events.jsonl"
    )));
    let config = config();
    let chunk = MediaChunk::new(
        RuntimeSessionId::new("session-private").expect("session id is valid"),
        MediaStreamId::new("stream-private").expect("stream id is valid"),
        NonZeroU64::new(1).expect("sequence is nonzero"),
        MediaDirection::Input,
        config.input_format(),
        vec![1, 2, 3, 4],
        &config,
    )
    .expect("chunk is valid");
    let actual = [
        ClientEvent::SessionUpdate.to_json(),
        ClientEvent::InputAudioAppend {
            event_id: "input-append-1",
            chunk: &chunk,
        }
        .to_json(),
        ClientEvent::InputAudioCommit {
            event_id: "input-commit-1",
        }
        .to_json(),
        ClientEvent::ResponseCreate {
            event_id: "response-create-1",
        }
        .to_json(),
        ClientEvent::ResponseCancel {
            event_id: "response-cancel-1",
            response_id: None,
        }
        .to_json(),
    ];
    assert_eq!(actual.as_slice(), expected.as_slice());
}

#[test]
fn success_corpus_preserves_order_audio_transcript_usage_and_rate_truth() {
    let events = parse_lines(include_bytes!(concat!(
        "../../tests/fixtures/openai-realtime-2026-07-22/",
        "success-events.jsonl"
    )));
    let mut stream = RealtimeServerStream::new();
    for event in &events {
        stream.apply(event).expect("ordered event applies");
    }
    stream
        .disconnected()
        .expect("terminal response permits close");

    assert!(matches!(events[0], RealtimeServerEvent::SessionConfigured));
    assert!(matches!(events[2], RealtimeServerEvent::InputCommitted));
    assert!(matches!(events[3], RealtimeServerEvent::ResponseStarted(_)));
    let RealtimeServerEvent::RateLimits(rate) = &events[4] else {
        panic!("rate event is retained");
    };
    assert_eq!(rate.len(), 2);
    let RealtimeServerEvent::AudioDelta { audio, .. } = &events[5] else {
        panic!("audio event is retained");
    };
    assert_eq!(audio.bytes(), [9, 8, 7, 6]);
    let RealtimeServerEvent::TranscriptDelta { transcript, .. } = &events[6] else {
        panic!("transcript delta is retained");
    };
    assert_eq!(transcript, "Hello");
    assert!(matches!(
        events[7],
        RealtimeServerEvent::AudioCompleted { .. }
    ));
    let RealtimeServerEvent::TranscriptCompleted { transcript, .. } = &events[8] else {
        panic!("completed transcript is retained");
    };
    assert_eq!(transcript, "Hello");
    let RealtimeServerEvent::Usage {
        usage, cancelled, ..
    } = &events[9]
    else {
        panic!("terminal usage is retained");
    };
    assert_eq!(usage.input_tokens(), Some(4));
    assert_eq!(usage.output_tokens(), Some(3));
    assert_eq!(*cancelled, None);

    let rendered = format!("{events:?}");
    for private in ["resp_fixture_1", "Hello", "CQgHBg=="] {
        assert!(!rendered.contains(private));
    }
}

#[test]
fn cancellation_is_native_terminal_truth() {
    let events = parse_lines(include_bytes!(concat!(
        "../../tests/fixtures/openai-realtime-2026-07-22/",
        "cancel-events.jsonl"
    )));
    let mut stream = RealtimeServerStream::new();
    for event in &events {
        stream.apply(event).expect("cancel event applies");
    }
    let RealtimeServerEvent::Usage { cancelled, .. } = &events[1] else {
        panic!("cancel terminal is retained");
    };
    assert_eq!(*cancelled, Some(ProviderCancellationOutcome::Confirmed));
}

#[test]
fn malformed_unknown_format_drift_error_and_disconnect_fail_safely() {
    for (name, bytes, code) in [
        (
            "malformed-event.json",
            include_bytes!(concat!(
                "../../tests/fixtures/openai-realtime-2026-07-22/",
                "malformed-event.json"
            ))
            .as_slice(),
            "swallowtail.openai.realtime_protocol_malformed",
        ),
        (
            "unknown-event.json",
            include_bytes!(concat!(
                "../../tests/fixtures/openai-realtime-2026-07-22/",
                "unknown-event.json"
            ))
            .as_slice(),
            "swallowtail.openai.realtime_event_unknown",
        ),
        (
            "format-drift.json",
            include_bytes!(concat!(
                "../../tests/fixtures/openai-realtime-2026-07-22/",
                "format-drift.json"
            ))
            .as_slice(),
            "swallowtail.openai.realtime_format_drift",
        ),
    ] {
        let failure = parse_server_event(bytes).expect_err("fixture must fail");
        assert_eq!(failure.diagnostic().code(), code, "wrong code for {name}");
        let rendered = format!("{failure:?}");
        assert!(!rendered.contains("resp_secret"));
        assert!(!rendered.contains("sess_secret"));
    }

    let provider = parse_server_event(include_bytes!(concat!(
        "../../tests/fixtures/openai-realtime-2026-07-22/",
        "provider-error.json"
    )))
    .expect("provider error parses");
    assert!(matches!(provider, RealtimeServerEvent::ProviderFailed));
    assert!(!format!("{provider:?}").contains("synthetic private provider detail"));

    let started =
        parse_server_event(br#"{"type":"response.created","response":{"id":"resp_secret"}}"#)
            .expect("response start parses");
    let mut stream = RealtimeServerStream::new();
    stream.apply(&started).expect("response starts");
    let disconnected = stream
        .disconnected()
        .expect_err("unterminated disconnect fails");
    assert_eq!(
        disconnected.diagnostic().code(),
        "swallowtail.openai.realtime_disconnected"
    );
    assert!(!format!("{disconnected:?}").contains("resp_secret"));
}

#[test]
fn protocol_fixture_pins_access_format_and_absent_recovery() {
    let protocol: Value = serde_json::from_slice(include_bytes!(concat!(
        "../../tests/fixtures/openai-realtime-2026-07-22/",
        "protocol.json"
    )))
    .expect("protocol fixture is JSON");
    assert_eq!(protocol["endpoint"], "wss://api.openai.com/v1/realtime");
    assert_eq!(protocol["credential_mechanism"], "public_api_key");
    assert_eq!(protocol["model"], "gpt-realtime-2.1");
    assert_eq!(protocol["input_format"]["rate"], 24_000);
    assert_eq!(protocol["output_format"]["channels"], 1);
    assert_eq!(protocol["maximum_successful_turns"], 2);
    assert_eq!(protocol["reconnect"], false);
    assert_eq!(protocol["resume"], false);
    assert_eq!(protocol["storage"], false);

    let handshake: Value = serde_json::from_slice(include_bytes!(concat!(
        "../../tests/fixtures/openai-realtime-2026-07-22/",
        "handshake-headers.json"
    )))
    .expect("handshake fixture is JSON");
    assert_eq!(
        handshake["request"]["authorization"],
        "Bearer <credential-lease>"
    );
    assert_eq!(
        handshake["response"]["x-request-id"],
        "req_realtime_fixture"
    );
}

fn config() -> RealtimeMediaConfig {
    let format = MediaFormat::audio(
        AudioEncoding::Pcm16LittleEndian,
        NonZeroU32::new(24_000).expect("sample rate is nonzero"),
        NonZeroU16::new(1).expect("channel count is nonzero"),
    );
    RealtimeMediaConfig::new(
        format,
        format,
        NonZeroU64::new(32_768).expect("chunk bound is nonzero"),
        NonZeroU32::new(2).expect("turn bound is nonzero"),
    )
}

fn parse_lines(bytes: &[u8]) -> Vec<RealtimeServerEvent> {
    bytes
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| parse_server_event(line).expect("fixture event parses"))
        .collect()
}

fn json_lines(bytes: &[u8]) -> Vec<Value> {
    bytes
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_slice(line).expect("fixture line is JSON"))
        .collect()
}
