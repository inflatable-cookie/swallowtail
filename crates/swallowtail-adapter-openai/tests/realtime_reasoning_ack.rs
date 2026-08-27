use crate::realtime_reasoning_prepare::mode;
use crate::realtime_support;

use futures_executor::block_on;
use realtime_support::{Call, RealtimeFixture, RealtimeScenario, TimeMode, complete, start_turn};
use swallowtail_adapter_openai::{OpenAiRealtimeSessionProfileInput, prepare_openai_realtime};
use swallowtail_runtime::{
    CleanupOutcome, RequestId, RuntimeTurnId, TerminalStatus, WorkingStateRestorationMethod,
    WorkingStateRestorationOutcome,
};

#[test]
fn omission_ignores_provider_returned_reasoning_shapes() {
    for scenario in [
        RealtimeScenario::OmissionAckWithEffort,
        RealtimeScenario::OmissionAckMalformed,
    ] {
        let fixture = RealtimeFixture::new(scenario, TimeMode::Pending);
        let prepared = prepare_openai_realtime(fixture.preparation_input(), &fixture.services())
            .expect("OpenAI Realtime integration prepares");
        let mut session = block_on(
            prepared
                .prepare_realtime_session(OpenAiRealtimeSessionProfileInput::manual_pcm_two_turns(
                    RequestId::new("omission-provider-shape").expect("request id is valid"),
                    None,
                ))
                .expect("omission prepares")
                .open_session(fixture.services()),
        )
        .expect("omission still opens when provider returns a reasoning shape");
        for turn in 1..=2 {
            let response = start_turn(
                &mut session,
                &fixture,
                &format!("omission-shape-{turn}"),
                turn,
            );
            let (response, _, outcome) = complete(response);
            assert_eq!(outcome.status(), &TerminalStatus::Completed);
            assert_eq!(block_on(response.close()), CleanupOutcome::Clean);
        }
        assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    }
}

#[test]
fn fresh_restoration_preserves_selected_reasoning() {
    let fixture = RealtimeFixture::new(RealtimeScenario::TwoTurns, TimeMode::Pending);
    let prepared = prepare_openai_realtime(fixture.preparation_input(), &fixture.services())
        .expect("OpenAI Realtime integration prepares");
    let session = prepared
        .prepare_realtime_session(
            OpenAiRealtimeSessionProfileInput::manual_pcm_two_turns(
                RequestId::new("reasoning-restoration").expect("request id"),
                None,
            )
            .with_reasoning_mode(mode("xhigh")),
        )
        .expect("Realtime session prepares");
    assert_eq!(session.request().reasoning_mode(), Some(&mode("xhigh")));
    let interrupted = RuntimeTurnId::new("realtime-interrupted").expect("turn id");
    let restoration = session.prepare_working_state_restoration(interrupted.clone());
    assert_eq!(
        restoration.method(),
        WorkingStateRestorationMethod::FreshRealtimeSessionReplacement
    );
    let restored = block_on(restoration.restore(fixture.services())).expect("replacement opens");
    let WorkingStateRestorationOutcome::RealtimeSessionReplaced(replacement) = restored else {
        panic!("fresh realtime replacement expected");
    };
    assert_eq!(replacement.interrupted_turn_id(), &interrupted);
    let update: serde_json::Value =
        serde_json::from_str(&fixture.server.frames()[0]).expect("session update is JSON");
    assert_eq!(update["session"]["reasoning"]["effort"], "xhigh");
    let (_, mut replacement) = replacement.into_parts();
    for turn in 1..=2 {
        let response = start_turn(
            &mut replacement,
            &fixture,
            &format!("replacement-reasoning-{turn}"),
            turn,
        );
        let (response, _, outcome) = complete(response);
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(response.close()), CleanupOutcome::Clean);
    }
    assert_eq!(block_on(replacement.close()), CleanupOutcome::Clean);
    assert_eq!(fixture.calls.count(Call::CredentialRelease), 1);
}

#[test]
fn explicit_acknowledgement_failures_join_before_credential_release() {
    for (scenario, selected, code) in [
        (
            RealtimeScenario::ReasoningAckMissing,
            "low",
            "swallowtail.openai.realtime_reasoning_acknowledgement_invalid",
        ),
        (
            RealtimeScenario::ReasoningAckMismatch,
            "low",
            "swallowtail.openai.realtime_reasoning_acknowledgement_invalid",
        ),
        (
            RealtimeScenario::ReasoningAckMalformed,
            "low",
            "swallowtail.openai.realtime_reasoning_acknowledgement_invalid",
        ),
        (
            RealtimeScenario::ReasoningAckDuplicateCreated,
            "low",
            "swallowtail.openai.realtime_session_order_invalid",
        ),
    ] {
        let fixture = RealtimeFixture::new(scenario, TimeMode::Pending);
        let prepared = prepare_openai_realtime(fixture.preparation_input(), &fixture.services())
            .expect("OpenAI Realtime integration prepares");
        let failure = block_on(
            prepared
                .prepare_realtime_session(
                    OpenAiRealtimeSessionProfileInput::manual_pcm_two_turns(
                        RequestId::new("ack-failure").expect("request id is valid"),
                        None,
                    )
                    .with_reasoning_mode(mode(selected)),
                )
                .expect("selection prepares")
                .open_session(fixture.services()),
        )
        .err()
        .expect("acknowledgement failure rejects the open");
        assert_eq!(failure.diagnostic().code(), code);
        assert_eq!(fixture.calls.count(Call::CredentialAcquire), 1);
        assert_eq!(fixture.calls.count(Call::CredentialRelease), 1);
        let calls = fixture.calls.calls();
        let blocking_join = calls
            .iter()
            .position(|call| *call == Call::BlockingJoin)
            .expect("connection work joined");
        let credential_release = calls
            .iter()
            .position(|call| *call == Call::CredentialRelease)
            .expect("credential released");
        assert!(blocking_join < credential_release);
        assert!(
            fixture
                .server
                .frames()
                .iter()
                .any(|frame| frame.contains("session.update"))
        );
    }
}
