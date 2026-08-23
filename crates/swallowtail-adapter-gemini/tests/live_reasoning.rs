use crate::live_support;

use futures_executor::block_on;
use live_support::{Call, LiveFixture, LiveScenario, TimeMode, config, rollover_policy};
use swallowtail_adapter_gemini::{
    GEMINI_LIVE_SUPERSEDED_FACADE_REVISION, GeminiLiveDriver, GeminiLiveSessionProfileInput,
    prepare_gemini_live,
};
use swallowtail_core::{Capability, CapabilityConstraint, PreflightPlan, ReasoningMode};
use swallowtail_runtime::{OpenRealtimeMediaSessionRequest, RealtimeMediaSessionDriver, RequestId};

const ADMITTED: [&str; 4] = ["minimal", "low", "medium", "high"];

const REJECTED: [&str; 12] = [
    "off", "none", "disabled", "on", "auto", "default", "dynamic", "xhigh", "max", "MINIMAL",
    "Low", "1024",
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
fn every_admitted_level_prepares_with_exact_capability_plan_and_request() {
    let fixture = LiveFixture::new(LiveScenario::TwoTurnsRollover, TimeMode::Pending);
    let prepared = prepare_gemini_live(fixture.preparation_input(), &fixture.services())
        .expect("Gemini Live integration prepares");
    for portable in ADMITTED {
        let operation = prepared
            .prepare_live_session(
                GeminiLiveSessionProfileInput::manual_pcm_with_one_rollover(
                    RequestId::new(format!("thinking-{portable}")).expect("request id is valid"),
                    None,
                )
                .with_reasoning_mode(mode(portable)),
            )
            .expect("Gemini Live session prepares");
        assert_eq!(
            operation.request().reasoning_mode(),
            Some(&mode(portable)),
            "request carries the exact selection"
        );
        assert_eq!(
            planned_reasoning(operation.plan()),
            Some(vec![CapabilityConstraint::ReasoningMode(mode(portable))]),
            "plan carries exactly one exact reasoning constraint"
        );
        assert_eq!(
            planned_reasoning(operation.evidence().plan()),
            planned_reasoning(operation.plan())
        );
        assert_eq!(
            operation.plan().protocol_facade_id().as_str(),
            "google.generativelanguage.v1beta.GenerativeService.BidiGenerateContent.thinking-2026-08-23"
        );
        assert_eq!(
            operation.plan().model_id().expect("model bound").as_str(),
            "gemini-3.1-flash-live-preview"
        );
        assert_eq!(
            operation
                .plan()
                .model_route_id()
                .expect("route bound")
                .as_str(),
            "gemini-3-1-flash-live-preview"
        );
        assert_eq!(operation.request().config(), &config());
        assert_eq!(
            operation.request().planned_connection_rollover(),
            rollover_policy()
        );
    }
    assert_eq!(fixture.calls.count(Call::NetworkAuthorize), 0);
    assert_eq!(fixture.calls.count(Call::CredentialAcquire), 0);
    assert!(fixture.server.frames().is_empty());
}

#[test]
fn omitted_selection_claims_no_reasoning_capability() {
    let fixture = LiveFixture::new(LiveScenario::TwoTurnsRollover, TimeMode::Pending);
    let prepared = prepare_gemini_live(fixture.preparation_input(), &fixture.services())
        .expect("Gemini Live integration prepares");
    let operation = prepared
        .prepare_live_session(GeminiLiveSessionProfileInput::manual_pcm_with_one_rollover(
            RequestId::new("thinking-omitted").expect("request id is valid"),
            None,
        ))
        .expect("Gemini Live session prepares");
    assert_eq!(operation.request().reasoning_mode(), None);
    assert_eq!(planned_reasoning(operation.plan()), None);
    assert_eq!(planned_reasoning(operation.evidence().plan()), None);
}

#[test]
fn unsupported_values_reject_before_access_or_connection() {
    let fixture = LiveFixture::new(LiveScenario::TwoTurnsRollover, TimeMode::Pending);
    let prepared = prepare_gemini_live(fixture.preparation_input(), &fixture.services())
        .expect("Gemini Live integration prepares");
    for value in REJECTED {
        let failure = prepared
            .prepare_live_session(
                GeminiLiveSessionProfileInput::manual_pcm_with_one_rollover(
                    RequestId::new("thinking-rejected").expect("request id is valid"),
                    None,
                )
                .with_reasoning_mode(mode(value)),
            )
            .expect_err("unsupported reasoning value is rejected");
        assert_eq!(
            failure.diagnostic().safe().code(),
            "swallowtail.gemini.live_preparation.reasoning_value_unsupported",
            "{value} is rejected without clamping or aliasing"
        );
    }
    assert_eq!(fixture.calls.count(Call::NetworkAuthorize), 0);
    assert_eq!(fixture.calls.count(Call::CredentialAcquire), 0);
    assert!(fixture.server.frames().is_empty());
}

#[test]
fn request_plan_and_value_drift_reject_before_endpoint_or_credential_work() {
    let fixture = LiveFixture::new(LiveScenario::TwoTurnsRollover, TimeMode::Pending);
    let selected = mode("high");
    let base = OpenRealtimeMediaSessionRequest::new(
        RequestId::new("thinking-drift").expect("request id is valid"),
        config(),
        None,
    )
    .with_planned_connection_rollover(rollover_policy());
    let drifted: [(PreflightPlan, OpenRealtimeMediaSessionRequest); 4] = [
        (
            fixture.plan(),
            base.clone().with_reasoning_mode(selected.clone()),
        ),
        (fixture.plan_with_reasoning(&selected), base.clone()),
        (
            fixture.plan_with_reasoning(&mode("low")),
            base.clone().with_reasoning_mode(selected.clone()),
        ),
        (
            fixture.plan_with_reasoning(&mode("xhigh")),
            base.with_reasoning_mode(mode("xhigh")),
        ),
    ];
    for (plan, request) in drifted {
        let failure = block_on(GeminiLiveDriver::new().open_realtime_media_session(
            plan,
            request,
            fixture.services(),
        ))
        .err()
        .expect("drifted reasoning is rejected");
        assert_eq!(
            failure.diagnostic().code(),
            "swallowtail.gemini.live_preflight_rejected"
        );
    }
    assert_eq!(fixture.calls.count(Call::NetworkAuthorize), 0);
    assert_eq!(fixture.calls.count(Call::CredentialAcquire), 0);
    assert!(fixture.server.frames().is_empty());
}

#[test]
fn the_superseded_facade_point_is_named_and_no_longer_executable() {
    assert_eq!(
        GEMINI_LIVE_SUPERSEDED_FACADE_REVISION,
        "google.generativelanguage.v1beta.GenerativeService.BidiGenerateContent",
        "the pre-thinking proof keeps its exact historical point"
    );
    let fixture = LiveFixture::new(LiveScenario::TwoTurnsRollover, TimeMode::Pending);
    let failure = block_on(
        GeminiLiveDriver::new().open_realtime_media_session(
            fixture.plan_with_facade(GEMINI_LIVE_SUPERSEDED_FACADE_REVISION),
            OpenRealtimeMediaSessionRequest::new(
                RequestId::new("superseded-facade").expect("request id is valid"),
                config(),
                None,
            )
            .with_planned_connection_rollover(rollover_policy()),
            fixture.services(),
        ),
    )
    .err()
    .expect("a plan on the superseded facade point is rejected");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.gemini.live_preflight_rejected"
    );
    assert_eq!(fixture.calls.count(Call::NetworkAuthorize), 0);
    assert_eq!(fixture.calls.count(Call::CredentialAcquire), 0);
    assert!(fixture.server.frames().is_empty());
}
