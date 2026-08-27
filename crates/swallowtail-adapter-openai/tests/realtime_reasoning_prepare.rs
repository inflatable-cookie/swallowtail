use crate::realtime_support;

use realtime_support::{Call, RealtimeFixture, RealtimeScenario, TimeMode};
use swallowtail_adapter_openai::{
    OPENAI_REALTIME_FACADE_REVISION, OPENAI_REALTIME_SUPERSEDED_FACADE_REVISION,
    OpenAiRealtimeDriver, OpenAiRealtimeSessionProfileInput, prepare_openai_realtime,
};
use swallowtail_core::{Capability, CapabilityConstraint, PreflightPlan, ReasoningMode};
use swallowtail_runtime::{OpenRealtimeMediaSessionRequest, RealtimeMediaSessionDriver, RequestId};

pub(crate) const ADMITTED: [&str; 5] = ["minimal", "low", "medium", "high", "xhigh"];

const REJECTED: [&str; 12] = [
    "none", "max", "off", "default", "on", "auto", "MINIMAL", "Low", "1024", "disabled", "dynamic",
    "x-high",
];

pub(crate) fn mode(value: &str) -> ReasoningMode {
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
fn the_superseded_facade_point_is_named_and_no_longer_executable() {
    assert_eq!(
        OPENAI_REALTIME_SUPERSEDED_FACADE_REVISION, "openai-realtime-2026-07-22",
        "the pre-reasoning proof keeps its exact historical point"
    );
    let fixture = RealtimeFixture::new(RealtimeScenario::TwoTurns, TimeMode::Pending);
    let failure =
        futures_executor::block_on(OpenAiRealtimeDriver::new().open_realtime_media_session(
            fixture.plan_with_facade(OPENAI_REALTIME_SUPERSEDED_FACADE_REVISION),
            OpenRealtimeMediaSessionRequest::new(
                RequestId::new("superseded-facade").expect("request id is valid"),
                realtime_support::config(),
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
