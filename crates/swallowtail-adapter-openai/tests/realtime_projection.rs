use crate::realtime_reasoning_prepare::mode;
use crate::realtime_support;

use futures_executor::block_on;
use realtime_support::{RealtimeFixture, RealtimeScenario, TimeMode, complete, start_turn};
use std::collections::BTreeSet;
use std::num::NonZeroU64;
use swallowtail_adapter_openai::{
    OpenAiPreparedRealtimeSession, OpenAiRealtimeProjectionOpenFailure,
    OpenAiRealtimeSessionProfileInput, prepare_openai_realtime,
};
use swallowtail_runtime::{
    CleanupOutcome, ConsumerRouteActorPosture, ConsumerRouteControlId,
    ConsumerRouteEvidenceStrength, ConsumerRouteFeatureId, ConsumerRouteLifecycle,
    ConsumerRouteProjectionContribution, ConsumerRouteProjectionSourceId,
    ConsumerRouteProjectionSourceKind, ConsumerRouteRowIdentity, ConsumerRouteSourceClass,
    ConsumerRouteValueDomain, RealtimeMediaSessionHandle, RequestId, TerminalStatus,
};

const PREPARED_SOURCE: &str = "openai.realtime.prepared";
const OBSERVATION_SOURCE: &str = "openai.realtime.active-session";

const PREPARED_FACADE: &str = "OpenAiPreparedRealtimeSession";
const PROJECTION_OPEN: &str = "open_session_with_projection";

const MATRIX_ONLY: &str = "matrix or route-wide posture only; no exact prepared realtime authority";

/// One exact `openai.realtime` census row and its adapter disposition.
struct LedgerEntry {
    semantic_id: &'static str,
    emitted_by: &'static [&'static str],
    withheld_because: &'static str,
}

/// Deterministic disposition of exactly the 15 `openai.realtime` census rows.
///
/// The ledger claims nothing about the remaining 716 census rows.
const REALTIME_FIRST_TRANCHE: [LedgerEntry; 15] = [
    LedgerEntry {
        semantic_id: "feature.model-catalogue",
        emitted_by: &[],
        withheld_because: "no prepared realtime plan carries model-catalogue authority",
    },
    LedgerEntry {
        semantic_id: "feature.realtime-media-session",
        emitted_by: &[PREPARED_FACADE, PROJECTION_OPEN],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.streaming-events",
        emitted_by: &[PREPARED_FACADE, PROJECTION_OPEN],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.usage-evidence",
        emitted_by: &[PREPARED_FACADE, PROJECTION_OPEN],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.output-token-limit",
        emitted_by: &[PREPARED_FACADE, PROJECTION_OPEN],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.reasoning-selection",
        emitted_by: &[PREPARED_FACADE, PROJECTION_OPEN],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.cancellation-or-interruption",
        emitted_by: &[PREPARED_FACADE, PROJECTION_OPEN],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.persistent-session-posture",
        emitted_by: &[],
        withheld_because: MATRIX_ONLY,
    },
    LedgerEntry {
        semantic_id: "feature.prepared-facade",
        emitted_by: &[PREPARED_FACADE, PROJECTION_OPEN],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.activity-observation",
        emitted_by: &[],
        withheld_because: "no prepared realtime plan requires the observable-activity capability",
    },
    LedgerEntry {
        semantic_id: "feature.active-session-reasoning-ack",
        emitted_by: &[PROJECTION_OPEN],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "control.reasoning-selection-session-start",
        emitted_by: &[PREPARED_FACADE, PROJECTION_OPEN],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "control.maximum-output-tokens",
        emitted_by: &[PREPARED_FACADE, PROJECTION_OPEN],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "control.realtime-media-config",
        emitted_by: &[PREPARED_FACADE, PROJECTION_OPEN],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "control.planned-connection-rollover",
        emitted_by: &[PREPARED_FACADE, PROJECTION_OPEN],
        withheld_because: "",
    },
];

fn source(id: &str) -> ConsumerRouteProjectionSourceId {
    ConsumerRouteProjectionSourceId::new(id).expect("fixture source id is valid")
}

fn semantic_id(identity: &ConsumerRouteRowIdentity) -> &'static str {
    match identity {
        ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::ModelCatalogue) => {
            "feature.model-catalogue"
        }
        ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::RealtimeMediaSession) => {
            "feature.realtime-media-session"
        }
        ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::StreamingEvents) => {
            "feature.streaming-events"
        }
        ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::UsageEvidence) => {
            "feature.usage-evidence"
        }
        ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::OutputTokenLimit) => {
            "feature.output-token-limit"
        }
        ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::ReasoningSelection) => {
            "feature.reasoning-selection"
        }
        ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::CancellationOrInterruption) => {
            "feature.cancellation-or-interruption"
        }
        ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::PersistentSessionPosture) => {
            "feature.persistent-session-posture"
        }
        ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::PreparedFacade) => {
            "feature.prepared-facade"
        }
        ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::ActivityObservation) => {
            "feature.activity-observation"
        }
        ConsumerRouteRowIdentity::Feature(
            ConsumerRouteFeatureId::ActiveSessionReasoningAcknowledgement,
        ) => "feature.active-session-reasoning-ack",
        ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::ReasoningSelection) => {
            "control.reasoning-selection-session-start"
        }
        ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::MaximumOutputTokens) => {
            "control.maximum-output-tokens"
        }
        ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::RealtimeMediaConfig) => {
            "control.realtime-media-config"
        }
        ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::PlannedConnectionRollover) => {
            "control.planned-connection-rollover"
        }
        other => panic!("unexpected realtime projection row {other:?}"),
    }
}

fn rows(contribution: &ConsumerRouteProjectionContribution) -> BTreeSet<&'static str> {
    contribution
        .selection_rows()
        .chain(contribution.session_start_rows())
        .chain(contribution.active_session_rows())
        .map(|row| semantic_id(row.identity()))
        .collect()
}

/// Completes the exact two provider turns the fixture scenario scripts.
fn drain_two_turns(mut session: Box<dyn RealtimeMediaSessionHandle>, fixture: &RealtimeFixture) {
    for turn in 1..=2 {
        let response = start_turn(&mut session, fixture, &format!("projection-{turn}"), turn);
        let (response, _, outcome) = complete(response);
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(response.close()), CleanupOutcome::Clean);
    }
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

fn prepared_session(
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

#[test]
fn the_coverage_ledger_dispositions_exactly_the_fifteen_realtime_rows() {
    let mut ids = BTreeSet::new();
    for entry in &REALTIME_FIRST_TRANCHE {
        assert!(
            ids.insert(entry.semantic_id),
            "the ledger repeats {}",
            entry.semantic_id
        );
        assert!(
            entry.semantic_id.starts_with("feature.") || entry.semantic_id.starts_with("control."),
            "{} is not a census row identity",
            entry.semantic_id
        );
        assert_eq!(
            entry.emitted_by.is_empty(),
            !entry.withheld_because.is_empty(),
            "{} must be either emitted or withheld with a reason",
            entry.semantic_id
        );
    }
    assert_eq!(REALTIME_FIRST_TRANCHE.len(), 15);
}

#[test]
fn prepared_and_acknowledged_contributions_match_the_coverage_ledger() {
    let fixture = RealtimeFixture::new(RealtimeScenario::TwoTurns, TimeMode::Pending);
    let session = prepared_session(&fixture, Some("xhigh"));
    let prepared = session
        .consumer_route_projection_contribution(source(PREPARED_SOURCE))
        .expect("prepared contribution is admitted");
    let expected_prepared = REALTIME_FIRST_TRANCHE
        .iter()
        .filter(|entry| entry.emitted_by.contains(&PREPARED_FACADE))
        .map(|entry| entry.semantic_id)
        .collect::<BTreeSet<_>>();
    assert_eq!(rows(&prepared), expected_prepared);

    let outcome = block_on(
        session.open_session_with_projection(source(OBSERVATION_SOURCE), fixture.services()),
    )
    .map_err(|failure| failure.failure().diagnostic().code())
    .expect("a matching acknowledgement opens the projected session");
    let expected_open = REALTIME_FIRST_TRANCHE
        .iter()
        .filter(|entry| entry.emitted_by.contains(&PROJECTION_OPEN))
        .map(|entry| entry.semantic_id)
        .collect::<BTreeSet<_>>();
    assert_eq!(rows(outcome.contribution()), expected_open);

    let acknowledgement = outcome
        .contribution()
        .active_session_rows()
        .next()
        .expect("the acknowledgement row is published");
    assert_eq!(
        acknowledgement.lifecycle(),
        ConsumerRouteLifecycle::PostOpenObservationOnly
    );
    assert_eq!(
        acknowledgement.actor_posture(),
        ConsumerRouteActorPosture::ObservationOnly
    );
    assert_eq!(
        acknowledgement.source_class(),
        ConsumerRouteSourceClass::RouteAcknowledgementEvidence
    );
    assert_eq!(
        acknowledgement.evidence_strength(),
        ConsumerRouteEvidenceStrength::WireAcknowledgement
    );
    assert!(acknowledgement.state_support().provider_effective());
    assert!(!acknowledgement.state_support().rejected());
    assert!(acknowledgement.mutation_authority().is_acknowledged());
    let ConsumerRouteValueDomain::Enumerated(values) = acknowledgement
        .control_value()
        .expect("the acknowledgement carries its exact state")
        .domain()
    else {
        panic!("the acknowledged effort is enumerated");
    };
    assert_eq!(
        values
            .values()
            .map(|value| value.as_str())
            .collect::<Vec<_>>(),
        ["xhigh"]
    );

    let (session_handle, _) = outcome.into_parts();
    drain_two_turns(session_handle, &fixture);
}

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
    let outcome = block_on(
        session.open_session_with_projection(source(OBSERVATION_SOURCE), fixture.services()),
    )
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
    let failure = block_on(
        session.open_session_with_projection(source(OBSERVATION_SOURCE), fixture.services()),
    )
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
        let failure = block_on(
            session.open_session_with_projection(source(OBSERVATION_SOURCE), fixture.services()),
        )
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
            prepared_session(&projected, Some("low"))
                .open_session_with_projection(source(OBSERVATION_SOURCE), projected.services()),
        )
        .err()
        .expect("the projected open path rejects")
        .failure()
        .diagnostic()
        .code();
        assert_eq!(preserved_code, projected_code);
    }
}

#[test]
fn no_projected_realtime_row_carries_raw_endpoint_or_credential_data() {
    let fixture = RealtimeFixture::new(RealtimeScenario::TwoTurns, TimeMode::Pending);
    let session = prepared_session(&fixture, Some("high"));
    let contribution = session
        .consumer_route_projection_contribution(source(PREPARED_SOURCE))
        .expect("prepared contribution is admitted");
    let rendered = format!("{contribution:?}");
    for forbidden in [
        "openai-realtime-fixture-endpoint",
        "openai-realtime-fixture-key",
        "fixture-secret",
        "ws://",
    ] {
        assert!(
            !rendered.contains(forbidden),
            "a projected row must not carry raw target, endpoint, or credential data"
        );
    }
}
