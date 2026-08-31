use super::ledger::*;
use super::naming::*;
use super::support::*;
use crate::realtime_support;

use futures_executor::block_on;
use realtime_support::{RealtimeFixture, RealtimeScenario, TimeMode};
use swallowtail_runtime::ConsumerRouteProjectionSourceKind;

/// Proves prepared and active-observation evidence keep separate identities.
#[test]
fn prepared_and_active_observation_sources_stay_distinct() {
    let fixture = RealtimeFixture::new(RealtimeScenario::TwoTurns, TimeMode::Pending);
    let session = prepared_session(&fixture, Some("xhigh"));
    let outcome = block_on(session.open_session_with_projection(
        source(PREPARED_SOURCE),
        source(OBSERVATION_SOURCE),
        fixture.services(),
    ))
    .map_err(|failure| failure.failure().diagnostic().code())
    .expect("a matching acknowledgement opens the projected session");
    let contribution = outcome.contribution();

    let named = contribution
        .sources()
        .map(|source| (source.id().as_str().to_owned(), source.kind()))
        .collect::<Vec<_>>();
    assert_eq!(
        named,
        vec![
            (
                PREPARED_SOURCE.to_owned(),
                ConsumerRouteProjectionSourceKind::AdapterContribution
            ),
            (
                OBSERVATION_SOURCE.to_owned(),
                ConsumerRouteProjectionSourceKind::ActiveSessionObservation
            ),
        ],
        "the two evidence sources are independently retained"
    );

    for row in contribution
        .selection_rows()
        .chain(contribution.session_start_rows())
    {
        assert_eq!(
            row.source().id().as_str(),
            PREPARED_SOURCE,
            "{:?} is prepared truth and keeps the prepared source",
            row.identity()
        );
        assert!(
            !row.state_support().provider_effective() && !row.state_support().rejected(),
            "{:?} is prepared truth and claims no acknowledgement",
            row.identity()
        );
    }
    for row in contribution.active_session_rows() {
        assert_eq!(
            row.source().id().as_str(),
            OBSERVATION_SOURCE,
            "{:?} is post-open truth and keeps the observation source",
            row.identity()
        );
        assert_eq!(
            row.mutation_authority().source().map(|id| id.as_str()),
            Some(OBSERVATION_SOURCE),
            "acknowledgement authority names the observation source"
        );
    }

    let (handle, _) = outcome.into_parts();
    drain_two_turns(handle, &fixture);
}

/// Proves one id may not serve as both prepared and active-observation evidence.
#[test]
fn equal_projection_source_ids_are_rejected_before_provider_work() {
    let fixture = RealtimeFixture::new(RealtimeScenario::TwoTurns, TimeMode::Pending);
    let session = prepared_session(&fixture, Some("low"));
    let failure = block_on(session.open_session_with_projection(
        source(PREPARED_SOURCE),
        source(PREPARED_SOURCE),
        fixture.services(),
    ))
    .err()
    .expect("one collapsed evidence source rejects the open");
    assert_eq!(
        failure.failure().diagnostic().code(),
        "swallowtail.openai.realtime_projection_source_identity_invalid"
    );
    assert!(
        failure.rejected_contribution().is_none(),
        "an invalid source pair carries no contribution"
    );
}
