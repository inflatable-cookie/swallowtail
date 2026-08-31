use crate::realtime_support;

use futures_executor::block_on;
use realtime_support::{RealtimeFixture, RealtimeScenario, TimeMode};
use std::collections::BTreeSet;
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteEvidenceStrength, ConsumerRouteLifecycle,
    ConsumerRouteSourceClass, ConsumerRouteValueDomain,
};

use super::ledger::*;
use super::naming::*;
use super::support::*;

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

    let outcome = block_on(session.open_session_with_projection(
        source(PREPARED_SOURCE),
        source(OBSERVATION_SOURCE),
        fixture.services(),
    ))
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
