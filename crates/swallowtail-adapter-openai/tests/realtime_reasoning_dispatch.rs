use crate::realtime_reasoning_prepare::{ADMITTED, mode};
use crate::realtime_support;

use futures_executor::block_on;
use realtime_support::{Call, RealtimeFixture, RealtimeScenario, TimeMode, complete, start_turn};
use std::num::NonZeroU64;
use swallowtail_adapter_openai::{OpenAiRealtimeSessionProfileInput, prepare_openai_realtime};
use swallowtail_core::{Capability, CapabilityConstraint};
use swallowtail_runtime::{CleanupOutcome, RequestId, TerminalStatus};

#[test]
fn admitted_efforts_dispatch_and_require_matching_acknowledgement() {
    for portable in ADMITTED {
        let fixture = RealtimeFixture::new(RealtimeScenario::TwoTurns, TimeMode::Pending);
        let prepared = prepare_openai_realtime(fixture.preparation_input(), &fixture.services())
            .expect("OpenAI Realtime integration prepares");
        let operation = prepared
            .prepare_realtime_session(
                OpenAiRealtimeSessionProfileInput::manual_pcm_two_turns(
                    RequestId::new(format!("dispatch-{portable}")).expect("request id is valid"),
                    None,
                )
                .with_reasoning_mode(mode(portable)),
            )
            .expect("reasoning prepares");
        let mut session =
            block_on(operation.open_session(fixture.services())).expect("session opens");
        let update: serde_json::Value =
            serde_json::from_str(&fixture.server.frames()[0]).expect("session update is JSON");
        assert_eq!(update["session"]["reasoning"]["effort"], portable);
        assert!(update["session"].get("max_output_tokens").is_none());
        for turn in 1..=2 {
            let response = start_turn(
                &mut session,
                &fixture,
                &format!("reasoning-stream-{portable}-{turn}"),
                turn,
            );
            let (response, _, outcome) = complete(response);
            assert_eq!(outcome.status(), &TerminalStatus::Completed);
            assert_eq!(block_on(response.close()), CleanupOutcome::Clean);
        }
        assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
        assert_eq!(fixture.calls.count(Call::CredentialRelease), 1);
    }
}

#[test]
fn omission_keeps_historical_session_update_bytes() {
    let fixture = RealtimeFixture::new(RealtimeScenario::TwoTurns, TimeMode::Pending);
    let prepared = prepare_openai_realtime(fixture.preparation_input(), &fixture.services())
        .expect("OpenAI Realtime integration prepares");
    let operation = prepared
        .prepare_realtime_session(OpenAiRealtimeSessionProfileInput::manual_pcm_two_turns(
            RequestId::new("omission-bytes").expect("request id is valid"),
            None,
        ))
        .expect("omission prepares");
    let mut session = block_on(operation.open_session(fixture.services())).expect("session opens");
    let update: serde_json::Value =
        serde_json::from_str(&fixture.server.frames()[0]).expect("session update is JSON");
    let expected: serde_json::Value = serde_json::from_str(
        include_str!("fixtures/openai-realtime-2026-07-22/client-events.jsonl")
            .lines()
            .next()
            .expect("session update line exists"),
    )
    .expect("historical session update is JSON");
    assert_eq!(update, expected);
    assert!(update["session"].get("reasoning").is_none());
    for turn in 1..=2 {
        let response = start_turn(
            &mut session,
            &fixture,
            &format!("omission-stream-{turn}"),
            turn,
        );
        let (response, _, outcome) = complete(response);
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(response.close()), CleanupOutcome::Clean);
    }
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

#[test]
fn reasoning_composes_with_output_maximum_omission_and_bounds() {
    let fixture = RealtimeFixture::new(RealtimeScenario::TwoTurns, TimeMode::Pending);
    let prepared = prepare_openai_realtime(fixture.preparation_input(), &fixture.services())
        .expect("OpenAI Realtime integration prepares");
    let maximum = NonZeroU64::new(512).expect("maximum is non-zero");
    for portable in ADMITTED {
        let operation = prepared
            .prepare_realtime_session(
                OpenAiRealtimeSessionProfileInput::manual_pcm_two_turns(
                    RequestId::new(format!("compose-{portable}")).expect("request id is valid"),
                    None,
                )
                .with_reasoning_mode(mode(portable))
                .with_maximum_output_tokens(maximum),
            )
            .expect("composed selection prepares");
        assert_eq!(operation.request().reasoning_mode(), Some(&mode(portable)));
        assert_eq!(operation.request().maximum_output_tokens(), Some(maximum));
        assert!(
            operation
                .plan()
                .requirements()
                .capabilities()
                .any(|requirement| {
                    requirement.capability() == Capability::OutputTokenLimit
                        && requirement
                            .constraints()
                            .eq([&CapabilityConstraint::OutputTokenMaximum(512)])
                })
        );
    }
    let mut session = block_on(
        prepared
            .prepare_realtime_session(
                OpenAiRealtimeSessionProfileInput::manual_pcm_two_turns(
                    RequestId::new("compose-dispatch").expect("request id is valid"),
                    None,
                )
                .with_reasoning_mode(mode("low"))
                .with_maximum_output_tokens(maximum),
            )
            .expect("composed selection prepares")
            .open_session(fixture.services()),
    )
    .expect("session opens");
    let update: serde_json::Value =
        serde_json::from_str(&fixture.server.frames()[0]).expect("session update is JSON");
    assert_eq!(update["session"]["reasoning"]["effort"], "low");
    assert_eq!(update["session"]["max_output_tokens"], 512);
    for turn in 1..=2 {
        let response = start_turn(
            &mut session,
            &fixture,
            &format!("compose-stream-{turn}"),
            turn,
        );
        let (response, _, outcome) = complete(response);
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(response.close()), CleanupOutcome::Clean);
    }
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}
