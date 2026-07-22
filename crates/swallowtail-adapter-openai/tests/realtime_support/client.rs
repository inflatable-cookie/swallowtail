use super::{RealtimeFixture, config};
use futures_executor::block_on;
use futures_util::StreamExt;
use std::num::NonZeroU64;
use swallowtail_adapter_openai::OpenAiRealtimeDriver;
use swallowtail_core::MediaDirection;
use swallowtail_runtime::{
    Deadline, MediaChunk, MediaInputCommit, MediaStreamId, OpenRealtimeMediaSessionRequest,
    ProviderObservation, RealtimeMediaEvent, RealtimeMediaEventKind, RealtimeMediaResponseHandle,
    RealtimeMediaResponseStatus, RealtimeMediaSessionDriver, RealtimeMediaSessionHandle, RequestId,
    RuntimeSessionId, RuntimeTurnId, TerminalOutcome,
};

pub fn open(
    fixture: &RealtimeFixture,
    request: &str,
    deadline: Option<Deadline>,
) -> Box<dyn RealtimeMediaSessionHandle> {
    block_on(OpenAiRealtimeDriver::new().open_realtime_media_session(
        fixture.plan(),
        OpenRealtimeMediaSessionRequest::new(
            RequestId::new(request).expect("request id is valid"),
            config(),
            deadline,
        ),
        fixture.services(),
    ))
    .expect("realtime session opens")
}

pub fn start_one(
    session: &mut Box<dyn RealtimeMediaSessionHandle>,
    fixture: &RealtimeFixture,
    stream: &str,
) -> Box<dyn RealtimeMediaResponseHandle> {
    start_turn(session, fixture, stream, 1)
}

pub fn start_turn(
    session: &mut Box<dyn RealtimeMediaSessionHandle>,
    fixture: &RealtimeFixture,
    stream: &str,
    turn: u32,
) -> Box<dyn RealtimeMediaResponseHandle> {
    let stream_id = MediaStreamId::new(stream).expect("stream id is valid");
    block_on(session.append_input(
        input_chunk(session.session_id(), stream_id.clone(), 1),
        fixture.services(),
    ))
    .expect("input appends");
    block_on(session.commit_input(
        MediaInputCommit::new(turn_id(turn), stream_id),
        fixture.services(),
    ))
    .expect("input commits")
}

pub fn input_chunk(
    session_id: &RuntimeSessionId,
    stream_id: MediaStreamId,
    sequence: u64,
) -> MediaChunk {
    let config = config();
    MediaChunk::new(
        session_id.clone(),
        stream_id,
        NonZeroU64::new(sequence).expect("sequence is nonzero"),
        MediaDirection::Input,
        config.input_format(),
        vec![1, 2, 3, 4],
        &config,
    )
    .expect("input chunk is valid")
}

pub fn turn_id(turn: u32) -> RuntimeTurnId {
    RuntimeTurnId::new(format!("turn-{turn}")).expect("turn id is valid")
}

pub fn complete(
    mut response: Box<dyn RealtimeMediaResponseHandle>,
) -> (
    Box<dyn RealtimeMediaResponseHandle>,
    Vec<RealtimeMediaEvent>,
    TerminalOutcome,
) {
    let mut stream = response.take_events().expect("events exist");
    let terminal = response
        .take_terminal_outcome()
        .expect("terminal outcome exists");
    let (events, outcome) = block_on(async {
        let mut events = Vec::new();
        while let Some(event) = stream.next().await {
            events.push(event.expect("event succeeds"));
        }
        (events, terminal.await)
    });
    (response, events, outcome)
}

pub fn assert_ordered_evidence(events: &[RealtimeMediaEvent]) {
    assert!(matches!(
        events.first().expect("start exists").kind(),
        RealtimeMediaEventKind::ResponseStarted
    ));
    assert!(
        events
            .iter()
            .any(|event| matches!(event.kind(), RealtimeMediaEventKind::OutputAudio(_)))
    );
    assert!(
        events
            .iter()
            .any(|event| matches!(event.kind(), RealtimeMediaEventKind::TranscriptCompleted(_)))
    );
    for evidence in ["usage", "rate", "request"] {
        assert!(events.iter().any(|event| matches!(
            (evidence, event.kind()),
            (
                "usage",
                RealtimeMediaEventKind::ProviderObservation(ProviderObservation::Usage(_)),
            ) | (
                "rate",
                RealtimeMediaEventKind::ProviderObservation(ProviderObservation::RateLimit(_)),
            ) | (
                "request",
                RealtimeMediaEventKind::ProviderObservation(
                    ProviderObservation::RequestCorrelation(_),
                ),
            )
        )));
    }
    assert!(matches!(
        events.last().expect("terminal exists").kind(),
        RealtimeMediaEventKind::ResponseTerminal(RealtimeMediaResponseStatus::Completed)
    ));
    let rendered = format!("{events:?}");
    for private in ["Hello", "CQgHBg==", "resp_private"] {
        assert!(!rendered.contains(private));
    }
}
