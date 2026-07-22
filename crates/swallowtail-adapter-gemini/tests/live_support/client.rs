use super::{LiveFixture, config, rollover_policy};
use futures_executor::block_on;
use futures_util::StreamExt;
use std::num::NonZeroU64;
use swallowtail_adapter_gemini::GeminiLiveDriver;
use swallowtail_core::MediaDirection;
use swallowtail_runtime::{
    Deadline, MediaChunk, MediaInputCommit, MediaStreamId, OpenRealtimeMediaSessionRequest,
    RealtimeMediaEvent, RealtimeMediaResponseHandle, RealtimeMediaSessionDriver,
    RealtimeMediaSessionHandle, RequestId, RuntimeSessionId, RuntimeTurnId, TerminalOutcome,
};

pub fn open(
    fixture: &LiveFixture,
    request: &str,
    deadline: Option<Deadline>,
) -> Box<dyn RealtimeMediaSessionHandle> {
    block_on(
        GeminiLiveDriver::new().open_realtime_media_session(
            fixture.plan(),
            OpenRealtimeMediaSessionRequest::new(
                RequestId::new(request).unwrap(),
                config(),
                deadline,
            )
            .with_planned_connection_rollover(rollover_policy()),
            fixture.services(),
        ),
    )
    .expect("Gemini Live session opens")
}

pub fn start_turn(
    session: &mut Box<dyn RealtimeMediaSessionHandle>,
    fixture: &LiveFixture,
    turn: u32,
) -> Box<dyn RealtimeMediaResponseHandle> {
    let stream_id = MediaStreamId::new(format!("input-{turn}")).unwrap();
    block_on(session.append_input(
        input_chunk(session.session_id(), stream_id.clone(), 1),
        fixture.services(),
    ))
    .expect("input appends");
    block_on(session.commit_input(
        MediaInputCommit::new(
            RuntimeTurnId::new(format!("turn-{turn}")).unwrap(),
            stream_id,
        ),
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
        NonZeroU64::new(sequence).unwrap(),
        MediaDirection::Input,
        config.input_format(),
        vec![1, 2, 3, 4],
        &config,
    )
    .expect("input chunk is valid")
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
