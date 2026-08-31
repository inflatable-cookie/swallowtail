use crate::realtime_support;

use futures_executor::block_on;
use realtime_support::{RealtimeFixture, RealtimeScenario, TimeMode};
use swallowtail_adapter_openai::OpenAiRealtimeProjectionOpenFailure;
use swallowtail_runtime::{
    ConsumerRouteControlId, ConsumerRouteProjectionSourceKind, ConsumerRouteRowIdentity,
    ConsumerRouteValueDomain,
};

use super::ledger::*;
use super::naming::*;
use super::support::*;

#[test]
fn a_pending_request_never_claims_provider_effective_reasoning() {
    let fixture = RealtimeFixture::new(RealtimeScenario::TwoTurns, TimeMode::Pending);
    let session = prepared_session(&fixture, Some("low"));
    let prepared = session
        .consumer_route_projection_contribution(source(PREPARED_SOURCE))
        .expect("prepared contribution is admitted");
    let reasoning = prepared
        .session_start_rows()
        .find(|row| {
            row.identity()
                == &ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::ReasoningSelection)
        })
        .expect("the session-start reasoning control is published");
    assert!(reasoning.state_support().requested());
    assert!(reasoning.state_support().prepared());
    assert!(reasoning.state_support().pending());
    assert!(!reasoning.state_support().provider_effective());
    assert!(!reasoning.state_support().rejected());
    assert!(!reasoning.mutation_authority().is_acknowledged());
    assert!(prepared.active_session_rows().len() == 0);
    assert_eq!(prepared.sources().len(), 1);
    assert_eq!(
        prepared
            .sources()
            .next()
            .expect("the prepared source is named")
            .kind(),
        ConsumerRouteProjectionSourceKind::AdapterContribution
    );
}

#[test]
fn omitted_reasoning_produces_no_reasoning_state_on_a_successful_open() {
    let fixture = RealtimeFixture::new(RealtimeScenario::OmissionAckWithEffort, TimeMode::Pending);
    let session = prepared_session(&fixture, None);
    let outcome = block_on(session.open_session_with_projection(
        source(PREPARED_SOURCE),
        source(OBSERVATION_SOURCE),
        fixture.services(),
    ))
    .map_err(|failure| failure.failure().diagnostic().code())
    .expect("omission still opens when the provider returns a reasoning shape");
    assert_eq!(outcome.contribution().active_session_rows().len(), 0);
    assert!(!rows(outcome.contribution()).contains("feature.active-session-reasoning-ack"));
    assert!(!rows(outcome.contribution()).contains("control.reasoning-selection-session-start"));
    assert_eq!(outcome.contribution().sources().len(), 1);
    let (handle, _) = outcome.into_parts();
    drain_two_turns(handle, &fixture);
}

#[test]
fn only_an_exact_well_formed_different_effort_returns_a_rejected_contribution() {
    let fixture = RealtimeFixture::new(RealtimeScenario::ReasoningAckMismatch, TimeMode::Pending);
    let session = prepared_session(&fixture, Some("low"));
    let failure = block_on(session.open_session_with_projection(
        source(PREPARED_SOURCE),
        source(OBSERVATION_SOURCE),
        fixture.services(),
    ))
    .err()
    .expect("a differing acknowledgement rejects the open");
    assert_eq!(
        failure.failure().diagnostic().code(),
        "swallowtail.openai.realtime_reasoning_acknowledgement_invalid"
    );
    let contribution = failure
        .rejected_contribution()
        .expect("an exact differing effort carries a rejected contribution");
    let acknowledgement = contribution
        .active_session_rows()
        .next()
        .expect("the rejected acknowledgement row is published");
    assert!(acknowledgement.state_support().rejected());
    assert!(!acknowledgement.state_support().provider_effective());
    let ConsumerRouteValueDomain::Enumerated(values) = acknowledgement
        .control_value()
        .expect("the rejected state carries its exact effort")
        .domain()
    else {
        panic!("the acknowledged effort is enumerated");
    };
    assert_eq!(
        values
            .values()
            .map(|value| value.as_str())
            .collect::<Vec<_>>(),
        ["high"]
    );
    let (failure, contribution) = failure.into_parts();
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.openai.realtime_reasoning_acknowledgement_invalid"
    );
    assert!(contribution.is_some());
}

#[test]
fn unknown_acknowledgement_failures_carry_no_contribution() {
    for (scenario, code) in [
        (
            RealtimeScenario::ReasoningAckMissing,
            "swallowtail.openai.realtime_reasoning_acknowledgement_invalid",
        ),
        (
            RealtimeScenario::ReasoningAckMalformed,
            "swallowtail.openai.realtime_reasoning_acknowledgement_invalid",
        ),
        (
            RealtimeScenario::ReasoningAckDuplicateCreated,
            "swallowtail.openai.realtime_session_order_invalid",
        ),
        (
            RealtimeScenario::FormatDrift,
            "swallowtail.openai.realtime_format_drift",
        ),
    ] {
        let fixture = RealtimeFixture::new(scenario, TimeMode::Pending);
        let session = prepared_session(&fixture, Some("low"));
        let failure = block_on(session.open_session_with_projection(
            source(PREPARED_SOURCE),
            source(OBSERVATION_SOURCE),
            fixture.services(),
        ))
        .err()
        .expect("unknown acknowledgement evidence rejects the open");
        assert!(matches!(
            failure,
            OpenAiRealtimeProjectionOpenFailure::Runtime(_)
        ));
        assert_eq!(failure.failure().diagnostic().code(), code);
        assert!(failure.rejected_contribution().is_none());
        let (_, contribution) = failure.into_parts();
        assert!(contribution.is_none());
    }
}

#[test]
fn both_public_open_methods_report_the_same_route_failure() {
    for scenario in [
        RealtimeScenario::ReasoningAckMismatch,
        RealtimeScenario::ReasoningAckMissing,
        RealtimeScenario::ReasoningAckMalformed,
        RealtimeScenario::ReasoningAckDuplicateCreated,
    ] {
        let preserved = RealtimeFixture::new(scenario, TimeMode::Pending);
        let preserved_code =
            block_on(prepared_session(&preserved, Some("low")).open_session(preserved.services()))
                .err()
                .expect("the preserved open path rejects")
                .diagnostic()
                .code();
        let projected = RealtimeFixture::new(scenario, TimeMode::Pending);
        let projected_code = block_on(
            prepared_session(&projected, Some("low")).open_session_with_projection(
                source(PREPARED_SOURCE),
                source(OBSERVATION_SOURCE),
                projected.services(),
            ),
        )
        .err()
        .expect("the projected open path rejects")
        .failure()
        .diagnostic()
        .code();
        assert_eq!(preserved_code, projected_code);
    }
}
