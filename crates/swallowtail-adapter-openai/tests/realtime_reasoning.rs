use crate::realtime_support;

use futures_executor::block_on;
use realtime_support::{
    Call, RealtimeFixture, RealtimeScenario, TimeMode, complete, config, start_turn,
};
use std::num::NonZeroU64;
use swallowtail_adapter_openai::{
    OPENAI_REALTIME_FACADE_REVISION, OPENAI_REALTIME_SUPERSEDED_FACADE_REVISION,
    OpenAiRealtimeDriver, OpenAiRealtimeSessionProfileInput, prepare_openai_realtime,
};
use swallowtail_core::{Capability, CapabilityConstraint, PreflightPlan, ReasoningMode};
use swallowtail_runtime::{
    CleanupOutcome, OpenRealtimeMediaSessionRequest, RealtimeMediaSessionDriver, RequestId,
    RuntimeTurnId, TerminalStatus, WorkingStateRestorationMethod, WorkingStateRestorationOutcome,
};

const ADMITTED: [&str; 5] = ["minimal", "low", "medium", "high", "xhigh"];

const REJECTED: [&str; 12] = [
    "none", "max", "off", "default", "on", "auto", "MINIMAL", "Low", "1024", "disabled", "dynamic",
    "x-high",
];

fn mode(value: &str) -> ReasoningMode {
    ReasoningMode::new(value).expect("fixture reasoning mode is valid")
}

fn planned_reasoning(plan: &PreflightPlan) -> Option<Vec<CapabilityConstraint>> {
    plan.requirements()
        .capabilities()
        .find(|required| required.capability() == Capability::ReasoningSelection)
        .map(|required| required.constraints().cloned().collect())
}

#[test]
fn every_admitted_effort_prepares_with_exact_capability_plan_and_request() {
    let fixture = RealtimeFixture::new(RealtimeScenario::TwoTurns, TimeMode::Pending);
    let prepared = prepare_openai_realtime(fixture.preparation_input(), &fixture.services())
        .expect("OpenAI Realtime integration prepares");
    for portable in ADMITTED {
        let operation = prepared
            .prepare_realtime_session(
                OpenAiRealtimeSessionProfileInput::manual_pcm_two_turns(
                    RequestId::new(format!("reasoning-{portable}")).expect("request id is valid"),
                    None,
                )
                .with_reasoning_mode(mode(portable)),
            )
            .expect("OpenAI Realtime session prepares");
        assert_eq!(operation.request().reasoning_mode(), Some(&mode(portable)));
        assert_eq!(
            planned_reasoning(operation.plan()),
            Some(vec![CapabilityConstraint::ReasoningMode(mode(portable))])
        );
        assert_eq!(
            planned_reasoning(operation.evidence().plan()),
            planned_reasoning(operation.plan())
        );
        assert_eq!(
            operation.plan().protocol_facade_id().as_str(),
            OPENAI_REALTIME_FACADE_REVISION
        );
        assert_eq!(
            operation.plan().model_id().expect("model bound").as_str(),
            "gpt-realtime-2.1"
        );
    }
    assert_eq!(fixture.calls.count(Call::NetworkAuthorize), 0);
    assert_eq!(fixture.calls.count(Call::CredentialAcquire), 0);
    assert!(fixture.server.frames().is_empty());
}

#[test]
fn omitted_selection_claims_no_reasoning_capability() {
    let fixture = RealtimeFixture::new(RealtimeScenario::TwoTurns, TimeMode::Pending);
    let prepared = prepare_openai_realtime(fixture.preparation_input(), &fixture.services())
        .expect("OpenAI Realtime integration prepares");
    let operation = prepared
        .prepare_realtime_session(OpenAiRealtimeSessionProfileInput::manual_pcm_two_turns(
            RequestId::new("reasoning-omitted").expect("request id is valid"),
            None,
        ))
        .expect("OpenAI Realtime session prepares");
    assert_eq!(operation.request().reasoning_mode(), None);
    assert_eq!(planned_reasoning(operation.plan()), None);
    assert_eq!(planned_reasoning(operation.evidence().plan()), None);
}

#[test]
fn unsupported_values_reject_before_access_or_connection() {
    let fixture = RealtimeFixture::new(RealtimeScenario::TwoTurns, TimeMode::Pending);
    let prepared = prepare_openai_realtime(fixture.preparation_input(), &fixture.services())
        .expect("OpenAI Realtime integration prepares");
    for value in REJECTED {
        let failure = prepared
            .prepare_realtime_session(
                OpenAiRealtimeSessionProfileInput::manual_pcm_two_turns(
                    RequestId::new("reasoning-rejected").expect("request id is valid"),
                    None,
                )
                .with_reasoning_mode(mode(value)),
            )
            .expect_err("unsupported reasoning value is rejected");
        assert_eq!(
            failure.diagnostic().safe().code(),
            "swallowtail.openai.realtime_preparation.reasoning_value_unsupported",
            "{value} is rejected without clamping or aliasing"
        );
    }
    assert_eq!(fixture.calls.count(Call::NetworkAuthorize), 0);
    assert_eq!(fixture.calls.count(Call::CredentialAcquire), 0);
    assert!(fixture.server.frames().is_empty());
}

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
            "swallowtail.openai.realtime_protocol_malformed",
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

#[test]
fn the_superseded_facade_point_is_named_and_no_longer_executable() {
    assert_eq!(
        OPENAI_REALTIME_SUPERSEDED_FACADE_REVISION, "openai-realtime-2026-07-22",
        "the pre-reasoning proof keeps its exact historical point"
    );
    let fixture = RealtimeFixture::new(RealtimeScenario::TwoTurns, TimeMode::Pending);
    let failure = block_on(OpenAiRealtimeDriver::new().open_realtime_media_session(
        fixture.plan_with_facade(OPENAI_REALTIME_SUPERSEDED_FACADE_REVISION),
        OpenRealtimeMediaSessionRequest::new(
            RequestId::new("superseded-facade").expect("request id is valid"),
            config(),
            None,
        ),
        fixture.services(),
    ))
    .err()
    .expect("a plan on the superseded facade point is rejected");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.openai.realtime_preflight_rejected"
    );
    assert_eq!(fixture.calls.count(Call::NetworkAuthorize), 0);
    assert_eq!(fixture.calls.count(Call::CredentialAcquire), 0);
    assert!(fixture.server.frames().is_empty());
}
