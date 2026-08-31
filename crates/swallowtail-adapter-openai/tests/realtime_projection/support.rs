use crate::realtime_reasoning_prepare::mode;
use crate::realtime_support;

use futures_executor::block_on;
use realtime_support::{RealtimeFixture, complete, start_turn};
use std::num::NonZeroU64;
use swallowtail_adapter_openai::{
    OpenAiPreparedRealtimeSession, OpenAiRealtimeSessionProfileInput, prepare_openai_realtime,
};
use swallowtail_runtime::{CleanupOutcome, RealtimeMediaSessionHandle, RequestId, TerminalStatus};

/// Completes the exact two provider turns the fixture scenario scripts.
pub(super) fn drain_two_turns(
    mut session: Box<dyn RealtimeMediaSessionHandle>,
    fixture: &RealtimeFixture,
) {
    for turn in 1..=2 {
        let response = start_turn(&mut session, fixture, &format!("projection-{turn}"), turn);
        let (response, _, outcome) = complete(response);
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(response.close()), CleanupOutcome::Clean);
    }
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

pub(super) fn prepared_session(
    fixture: &RealtimeFixture,
    effort: Option<&str>,
) -> OpenAiPreparedRealtimeSession {
    let prepared = prepare_openai_realtime(fixture.preparation_input(), &fixture.services())
        .expect("OpenAI Realtime integration prepares");
    let mut input = OpenAiRealtimeSessionProfileInput::manual_pcm_two_turns(
        RequestId::new("projection-realtime").expect("request id is valid"),
        None,
    )
    .with_maximum_output_tokens(NonZeroU64::new(1024).expect("bound is non-zero"));
    if let Some(effort) = effort {
        input = input.with_reasoning_mode(mode(effort));
    }
    prepared
        .prepare_realtime_session(input)
        .expect("realtime session prepares")
}
