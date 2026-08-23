use crate::live_support;

use futures_executor::block_on;
use live_support::{Call, LiveFixture, LiveScenario, TimeMode, complete, start_turn};
use serde_json::Value;
use swallowtail_adapter_gemini::{GeminiLiveSessionProfileInput, prepare_gemini_live};
use swallowtail_core::ReasoningMode;
use swallowtail_runtime::{
    CleanupOutcome, RequestId, RuntimeTurnId, TerminalStatus, WorkingStateRestorationOutcome,
};

fn mode(value: &str) -> ReasoningMode {
    ReasoningMode::new(value).expect("fixture reasoning mode is valid")
}

fn setup_levels(frames: &[String]) -> Vec<Option<String>> {
    frames
        .iter()
        .filter_map(|frame| {
            let value: Value = serde_json::from_str(frame).expect("fixture frame is JSON");
            let setup = value.get("setup")?.clone();
            Some(
                setup
                    .get("generationConfig")
                    .and_then(|config| config.get("thinkingConfig"))
                    .and_then(|thinking| thinking.get("thinkingLevel"))
                    .and_then(Value::as_str)
                    .map(str::to_owned),
            )
        })
        .collect()
}

fn resumption_handles(frames: &[String]) -> Vec<Option<String>> {
    frames
        .iter()
        .filter_map(|frame| {
            let value: Value = serde_json::from_str(frame).expect("fixture frame is JSON");
            let setup = value.get("setup")?.clone();
            Some(
                setup
                    .get("sessionResumption")
                    .and_then(|resumption| resumption.get("handle"))
                    .and_then(Value::as_str)
                    .map(str::to_owned),
            )
        })
        .collect()
}

#[test]
fn the_selected_level_survives_one_planned_rollover_on_the_wire() {
    let fixture = LiveFixture::new(LiveScenario::TwoTurnsRollover, TimeMode::Pending);
    let prepared = prepare_gemini_live(fixture.preparation_input(), &fixture.services())
        .expect("Gemini Live integration prepares");
    let operation = prepared
        .prepare_live_session(
            GeminiLiveSessionProfileInput::manual_pcm_with_one_rollover(
                RequestId::new("thinking-rollover").expect("request id is valid"),
                None,
            )
            .with_reasoning_mode(mode("high")),
        )
        .expect("Gemini Live session prepares");
    let mut session = block_on(operation.open_session(fixture.services())).expect("session opens");
    for turn in 1..=2 {
        let response = start_turn(&mut session, &fixture, turn);
        let (response, _, outcome) = complete(response);
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(response.close()), CleanupOutcome::Clean);
    }
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    assert_eq!(fixture.server.handshakes().len(), 2);
    let frames = fixture.server.frames();
    assert_eq!(
        setup_levels(&frames),
        vec![Some("HIGH".to_owned()), Some("HIGH".to_owned())],
        "both connections dispatch the same selected level"
    );
    let handles = resumption_handles(&frames);
    assert_eq!(handles.len(), 2);
    assert_eq!(handles[0], None, "the first setup carries no handle");
    assert!(
        handles[1].is_some(),
        "the rollover setup carries the private resumable handle"
    );
    assert_eq!(fixture.calls.count(Call::CredentialRelease), 1);
}

#[test]
fn omission_dispatches_the_current_minimal_bytes_on_both_connections() {
    let fixture = LiveFixture::new(LiveScenario::TwoTurnsRollover, TimeMode::Pending);
    let prepared = prepare_gemini_live(fixture.preparation_input(), &fixture.services())
        .expect("Gemini Live integration prepares");
    let operation = prepared
        .prepare_live_session(GeminiLiveSessionProfileInput::manual_pcm_with_one_rollover(
            RequestId::new("thinking-omitted-wire").expect("request id is valid"),
            None,
        ))
        .expect("Gemini Live session prepares");
    let mut session = block_on(operation.open_session(fixture.services())).expect("session opens");
    for turn in 1..=2 {
        let response = start_turn(&mut session, &fixture, turn);
        let (response, _, outcome) = complete(response);
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(response.close()), CleanupOutcome::Clean);
    }
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    assert_eq!(
        setup_levels(&fixture.server.frames()),
        vec![Some("MINIMAL".to_owned()), Some("MINIMAL".to_owned())]
    );
}

#[test]
fn fresh_restoration_reuses_the_same_selected_level() {
    let fixture = LiveFixture::new(LiveScenario::TwoTurnsRollover, TimeMode::Pending);
    let prepared = prepare_gemini_live(fixture.preparation_input(), &fixture.services())
        .expect("Gemini Live integration prepares");
    let operation = prepared
        .prepare_live_session(
            GeminiLiveSessionProfileInput::manual_pcm_with_one_rollover(
                RequestId::new("thinking-restoration").expect("request id is valid"),
                None,
            )
            .with_reasoning_mode(mode("medium")),
        )
        .expect("Gemini Live session prepares");
    let interrupted = RuntimeTurnId::new("thinking-interrupted").expect("turn id is valid");
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
    assert_eq!(
        setup_levels(&fixture.server.frames()),
        vec![Some("MEDIUM".to_owned()), Some("MEDIUM".to_owned())],
        "restoration and its rollover keep the prepared selection"
    );
}
