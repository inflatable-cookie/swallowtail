use super::{
    MediaChunk, MediaTranscript, RealtimeMediaEvent, RealtimeMediaEventKind,
    RealtimeMediaFailureKind, RealtimeMediaResponseStatus, RealtimeMediaSessionState,
};
use crate::{MediaStreamId, ProviderCancellationOutcome, RuntimeSessionId, RuntimeTurnId};
use std::num::{NonZeroU16, NonZeroU32, NonZeroU64};
use swallowtail_core::{AudioEncoding, MediaDirection, MediaFormat, RealtimeMediaConfig};

fn config() -> RealtimeMediaConfig {
    let format = MediaFormat::audio(
        AudioEncoding::Pcm16LittleEndian,
        NonZeroU32::new(24_000).expect("sample rate is nonzero"),
        NonZeroU16::new(1).expect("channel count is nonzero"),
    );
    RealtimeMediaConfig::new(
        format,
        format,
        NonZeroU64::new(8).expect("chunk limit is nonzero"),
        NonZeroU32::new(2).expect("turn limit is nonzero"),
    )
}

fn session() -> RuntimeSessionId {
    RuntimeSessionId::new("private-session").expect("session id is valid")
}

fn stream(value: &str) -> MediaStreamId {
    MediaStreamId::new(value).expect("stream id is valid")
}

fn chunk(
    stream_id: MediaStreamId,
    sequence: u64,
    direction: MediaDirection,
    bytes: &[u8],
) -> MediaChunk {
    let config = config();
    let format = match direction {
        MediaDirection::Input => config.input_format(),
        MediaDirection::Output => config.output_format(),
    };
    MediaChunk::new(
        session(),
        stream_id,
        NonZeroU64::new(sequence).expect("sequence is nonzero"),
        direction,
        format,
        bytes.to_vec(),
        &config,
    )
    .expect("chunk is valid")
}

fn event(sequence: u64, turn: &str, kind: RealtimeMediaEventKind) -> RealtimeMediaEvent {
    RealtimeMediaEvent::new(
        NonZeroU64::new(sequence).expect("event sequence is nonzero"),
        RuntimeTurnId::new(turn).expect("turn id is valid"),
        kind,
    )
}

#[test]
fn chunks_are_bounded_and_redacted() {
    let config = config();
    let empty = MediaChunk::new(
        session(),
        stream("input"),
        NonZeroU64::new(1).expect("sequence is nonzero"),
        MediaDirection::Input,
        config.input_format(),
        vec![],
        &config,
    )
    .expect_err("empty chunk must fail");
    assert_eq!(empty.kind(), RealtimeMediaFailureKind::EmptyChunk);

    let private = chunk(stream("input"), 1, MediaDirection::Input, b"private");
    assert_eq!(private.bytes(), b"private");
    let rendered = format!("{private:?}");
    assert!(!rendered.contains("private"));
    assert!(!rendered.contains("private-session"));
}

#[test]
fn chunks_reject_oversized_payloads_and_format_drift() {
    let config = config();
    let oversized = MediaChunk::new(
        session(),
        stream("input"),
        NonZeroU64::new(1).expect("sequence is nonzero"),
        MediaDirection::Input,
        config.input_format(),
        vec![0; 9],
        &config,
    )
    .expect_err("oversized chunk must fail");
    assert_eq!(oversized.kind(), RealtimeMediaFailureKind::ChunkTooLarge);

    let wrong_format = MediaFormat::audio(
        AudioEncoding::Pcm16LittleEndian,
        NonZeroU32::new(16_000).expect("sample rate is nonzero"),
        NonZeroU16::new(1).expect("channel count is nonzero"),
    );
    let drift = MediaChunk::new(
        session(),
        stream("input"),
        NonZeroU64::new(1).expect("sequence is nonzero"),
        MediaDirection::Input,
        wrong_format,
        vec![0; 4],
        &config,
    )
    .expect_err("format drift must fail");
    assert_eq!(drift.kind(), RealtimeMediaFailureKind::FormatMismatch);
}

#[test]
fn append_commit_output_and_terminal_order_are_exact() {
    let mut state = RealtimeMediaSessionState::new(session(), config());
    state
        .append_input(&chunk(stream("input-1"), 1, MediaDirection::Input, b"one"))
        .expect("first input applies");
    state
        .append_input(&chunk(stream("input-1"), 2, MediaDirection::Input, b"two"))
        .expect("second input applies");
    state
        .commit_input(
            RuntimeTurnId::new("turn-1").expect("turn id is valid"),
            stream("input-1"),
        )
        .expect("commit applies");
    assert!(state.response_active());

    for item in [
        event(1, "turn-1", RealtimeMediaEventKind::ResponseStarted),
        event(
            2,
            "turn-1",
            RealtimeMediaEventKind::OutputAudio(chunk(
                stream("output-1"),
                1,
                MediaDirection::Output,
                b"audio",
            )),
        ),
        event(
            3,
            "turn-1",
            RealtimeMediaEventKind::TranscriptDelta(
                MediaTranscript::new("hel").expect("transcript is valid"),
            ),
        ),
        event(
            4,
            "turn-1",
            RealtimeMediaEventKind::TranscriptCompleted(
                MediaTranscript::new("hello").expect("transcript is valid"),
            ),
        ),
        event(
            5,
            "turn-1",
            RealtimeMediaEventKind::ResponseTerminal(RealtimeMediaResponseStatus::Completed),
        ),
    ] {
        state
            .record_response_event(&item)
            .expect("ordered response event applies");
    }
    assert!(!state.response_active());
    assert!(state.is_reusable());
}

#[test]
fn gaps_duplicates_crossings_and_parallel_response_fail() {
    let mut state = RealtimeMediaSessionState::new(session(), config());
    state
        .append_input(&chunk(stream("input-1"), 1, MediaDirection::Input, b"one"))
        .expect("first input applies");
    let gap = state
        .append_input(&chunk(stream("input-1"), 3, MediaDirection::Input, b"gap"))
        .expect_err("gap must fail");
    assert_eq!(gap.kind(), RealtimeMediaFailureKind::SequenceInvalid);
    let duplicate = state
        .append_input(&chunk(stream("input-1"), 1, MediaDirection::Input, b"dup"))
        .expect_err("duplicate must fail");
    assert_eq!(duplicate.kind(), RealtimeMediaFailureKind::SequenceInvalid);
    let crossing = state
        .append_input(&chunk(
            stream("input-2"),
            2,
            MediaDirection::Input,
            b"cross",
        ))
        .expect_err("crossing must fail");
    assert_eq!(crossing.kind(), RealtimeMediaFailureKind::StreamMismatch);

    state
        .commit_input(
            RuntimeTurnId::new("turn-1").expect("turn id is valid"),
            stream("input-1"),
        )
        .expect("commit applies");
    let parallel = state
        .commit_input(
            RuntimeTurnId::new("turn-2").expect("turn id is valid"),
            stream("input-2"),
        )
        .expect_err("parallel response must fail");
    assert_eq!(parallel.kind(), RealtimeMediaFailureKind::OrderingInvalid);
}

#[test]
fn cancellation_and_deadline_end_the_session() {
    for status in [
        RealtimeMediaResponseStatus::Cancelled(ProviderCancellationOutcome::Confirmed),
        RealtimeMediaResponseStatus::TimedOut(ProviderCancellationOutcome::Unconfirmed),
    ] {
        let mut state = RealtimeMediaSessionState::new(session(), config());
        state
            .append_input(&chunk(stream("input"), 1, MediaDirection::Input, b"one"))
            .expect("input applies");
        state
            .commit_input(
                RuntimeTurnId::new("turn").expect("turn id is valid"),
                stream("input"),
            )
            .expect("commit applies");
        state
            .record_response_event(&event(1, "turn", RealtimeMediaEventKind::ResponseStarted))
            .expect("response starts");
        state
            .record_response_event(&event(
                2,
                "turn",
                RealtimeMediaEventKind::ResponseTerminal(status),
            ))
            .expect("terminal applies");
        assert!(!state.is_reusable());
    }
}
