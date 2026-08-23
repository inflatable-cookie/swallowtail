use crate::live_support;

use futures_executor::block_on;
use live_support::{Call, LiveFixture, LiveScenario, TimeMode, complete, start_turn};
use serde_json::{Value, json};
use std::num::NonZeroU64;
use swallowtail_adapter_gemini::{
    GEMINI_LIVE_FACADE_REVISION, GeminiLiveContextWindowCompression, GeminiLiveSessionProfileInput,
    prepare_gemini_live,
};
use swallowtail_core::ReasoningMode;
use swallowtail_runtime::{
    CleanupOutcome, RequestId, RuntimeTurnId, TerminalStatus, WorkingStateRestorationOutcome,
};

fn compression() -> GeminiLiveContextWindowCompression {
    GeminiLiveContextWindowCompression::sliding_window()
}

fn maximum(value: u64) -> NonZeroU64 {
    NonZeroU64::new(value).expect("fixture maximum is non-zero")
}

fn mode(value: &str) -> ReasoningMode {
    ReasoningMode::new(value).expect("fixture reasoning mode is valid")
}

fn setups(frames: &[String]) -> Vec<Value> {
    frames
        .iter()
        .filter_map(|frame| {
            let value: Value = serde_json::from_str(frame).expect("fixture frame is JSON");
            value.get("setup").cloned()
        })
        .collect()
}

fn compression_values(frames: &[String]) -> Vec<Option<Value>> {
    setups(frames)
        .iter()
        .map(|setup| setup.get("contextWindowCompression").cloned())
        .collect()
}

fn handles(frames: &[String]) -> Vec<Option<String>> {
    setups(frames)
        .iter()
        .map(|setup| {
            setup
                .get("sessionResumption")
                .and_then(|resumption| resumption.get("handle"))
                .and_then(Value::as_str)
                .map(str::to_owned)
        })
        .collect()
}

fn levels(frames: &[String]) -> Vec<Option<String>> {
    setups(frames)
        .iter()
        .map(|setup| {
            setup
                .get("generationConfig")
                .and_then(|generation| generation.get("thinkingConfig"))
                .and_then(|thinking| thinking.get("thinkingLevel"))
                .and_then(Value::as_str)
                .map(str::to_owned)
        })
        .collect()
}

fn maxima(frames: &[String]) -> Vec<Option<u64>> {
    setups(frames)
        .iter()
        .map(|setup| {
            setup
                .get("generationConfig")
                .and_then(|generation| generation.get("maxOutputTokens"))
                .and_then(Value::as_u64)
        })
        .collect()
}

#[test]
fn selected_default_sliding_window_survives_one_planned_rollover() {
    let fixture = LiveFixture::new(LiveScenario::TwoTurnsRollover, TimeMode::Pending);
    let prepared = prepare_gemini_live(fixture.preparation_input(), &fixture.services())
        .expect("Gemini Live integration prepares");
    let selected = compression();
    let operation = prepared
        .prepare_live_session(
            GeminiLiveSessionProfileInput::manual_pcm_with_one_rollover(
                RequestId::new("context-compression-rollover").expect("request id is valid"),
                None,
            )
            .with_context_window_compression(selected),
        )
        .expect("selected compression prepares");
    assert_eq!(
        operation.evidence().context_window_compression(),
        Some(selected)
    );
    assert_eq!(
        operation.plan().protocol_facade_id().as_str(),
        GEMINI_LIVE_FACADE_REVISION
    );
    assert_eq!(
        operation.plan().model_id().expect("model bound").as_str(),
        "gemini-3.1-flash-live-preview"
    );

    let mut session = block_on(operation.open_session(fixture.services())).expect("session opens");
    for turn in 1..=2 {
        let response = start_turn(&mut session, &fixture, turn);
        let (response, _, outcome) = complete(response);
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(response.close()), CleanupOutcome::Clean);
    }
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);

    let frames = fixture.server.frames();
    let setup_frames = raw_setup_frames(&frames);
    assert_eq!(
        setup_frames[0],
        include_str!("fixtures/gemini-live-2026-07-22/client-setup-compression-initial.json")
            .trim()
    );
    assert_eq!(
        setup_frames[1],
        include_str!("fixtures/gemini-live-2026-07-22/client-setup-compression-resume.json").trim()
    );
    assert_eq!(
        compression_values(&frames),
        vec![Some(json!({"slidingWindow": {}})); 2]
    );
    assert_eq!(handles(&frames)[0], None);
    assert_eq!(
        handles(&frames)[1],
        Some("fixture-private-handle-2".to_owned())
    );
    assert_eq!(fixture.calls.count(Call::CredentialRelease), 1);
}

#[test]
fn selected_default_sliding_window_survives_fresh_restoration_and_composes() {
    let fixture = LiveFixture::new(LiveScenario::TwoTurnsRollover, TimeMode::Pending);
    let prepared = prepare_gemini_live(fixture.preparation_input(), &fixture.services())
        .expect("Gemini Live integration prepares");
    let operation = prepared
        .prepare_live_session(
            GeminiLiveSessionProfileInput::manual_pcm_with_one_rollover(
                RequestId::new("context-compression-restoration").expect("request id is valid"),
                None,
            )
            .with_reasoning_mode(mode("high"))
            .with_maximum_output_tokens(maximum(1_024))
            .with_context_window_compression(compression()),
        )
        .expect("composed compression prepares");
    let interrupted = RuntimeTurnId::new("context-compression-interrupted").expect("turn id");
    let restoration = operation.prepare_working_state_restoration(interrupted.clone());
    let restored = block_on(restoration.restore(fixture.services())).expect("replacement opens");
    let WorkingStateRestorationOutcome::RealtimeSessionReplaced(replacement) = restored else {
        panic!("fresh realtime replacement expected");
    };
    assert_eq!(replacement.interrupted_turn_id(), &interrupted);
    let (_, mut replacement) = replacement.into_parts();
    for turn in 1..=2 {
        let response = start_turn(&mut replacement, &fixture, turn);
        let (response, _, outcome) = complete(response);
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(response.close()), CleanupOutcome::Clean);
    }
    assert_eq!(block_on(replacement.close()), CleanupOutcome::Clean);

    let frames = fixture.server.frames();
    assert_eq!(
        compression_values(&frames),
        vec![Some(json!({"slidingWindow": {}})); 2]
    );
    assert_eq!(
        levels(&frames),
        vec![Some("HIGH".to_owned()), Some("HIGH".to_owned())]
    );
    assert_eq!(maxima(&frames), vec![Some(1_024), Some(1_024)]);
    assert_eq!(handles(&frames)[0], None);
    assert!(handles(&frames)[1].is_some());
}

#[test]
fn omitted_compression_keeps_prior_initial_and_resume_setup_bytes() {
    let fixture = LiveFixture::new(LiveScenario::TwoTurnsRollover, TimeMode::Pending);
    let prepared = prepare_gemini_live(fixture.preparation_input(), &fixture.services())
        .expect("Gemini Live integration prepares");
    let operation = prepared
        .prepare_live_session(GeminiLiveSessionProfileInput::manual_pcm_with_one_rollover(
            RequestId::new("context-compression-omitted").expect("request id is valid"),
            None,
        ))
        .expect("omitted compression prepares");
    assert_eq!(operation.evidence().context_window_compression(), None);

    let mut session = block_on(operation.open_session(fixture.services())).expect("session opens");
    for turn in 1..=2 {
        let response = start_turn(&mut session, &fixture, turn);
        let (response, _, outcome) = complete(response);
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(response.close()), CleanupOutcome::Clean);
    }
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);

    let frames = fixture.server.frames();
    let setup_frames = raw_setup_frames(&frames);
    assert_eq!(compression_values(&frames), vec![None, None]);
    assert_eq!(
        setup_frames[0],
        include_str!("fixtures/gemini-live-2026-07-22/client-setup-initial.json").trim()
    );
    assert_eq!(
        setup_frames[1],
        include_str!("fixtures/gemini-live-2026-07-22/client-setup-resume.json").trim()
    );
}

fn raw_setup_frames(frames: &[String]) -> Vec<&str> {
    frames
        .iter()
        .filter(|frame| frame.contains("\"setup\""))
        .map(String::as_str)
        .collect()
}

#[test]
fn selected_compression_composes_with_every_admitted_thinking_and_output_selection() {
    let fixture = LiveFixture::new(LiveScenario::TwoTurnsRollover, TimeMode::Pending);
    let prepared = prepare_gemini_live(fixture.preparation_input(), &fixture.services())
        .expect("Gemini Live integration prepares");
    for (thinking_index, thinking) in [
        None,
        Some("minimal"),
        Some("low"),
        Some("medium"),
        Some("high"),
    ]
    .into_iter()
    .enumerate()
    {
        for (maximum_index, maximum_value) in [None, Some(1), Some(1_024), Some(65_536)]
            .into_iter()
            .enumerate()
        {
            let mut input = GeminiLiveSessionProfileInput::manual_pcm_with_one_rollover(
                RequestId::new(format!(
                    "context-compression-compose-{thinking_index}-{maximum_index}"
                ))
                .expect("request id is valid"),
                None,
            )
            .with_context_window_compression(compression());
            if let Some(thinking) = thinking {
                input = input.with_reasoning_mode(mode(thinking));
            }
            if let Some(maximum_value) = maximum_value {
                input = input.with_maximum_output_tokens(maximum(maximum_value));
            }
            let operation = prepared
                .prepare_live_session(input)
                .expect("admitted composition prepares");
            assert_eq!(
                operation.evidence().context_window_compression(),
                Some(compression())
            );
            assert_eq!(
                operation.plan().protocol_facade_id().as_str(),
                GEMINI_LIVE_FACADE_REVISION
            );
            assert_eq!(
                operation.request().maximum_output_tokens(),
                maximum_value.map(maximum)
            );
            let expected_mode = thinking.map(mode);
            assert_eq!(operation.request().reasoning_mode(), expected_mode.as_ref());
        }
    }
    assert_eq!(fixture.calls.count(Call::NetworkAuthorize), 0);
    assert_eq!(fixture.calls.count(Call::CredentialAcquire), 0);
    assert!(fixture.server.frames().is_empty());
}
