use std::collections::{BTreeMap, BTreeSet};
use swallowtail_core::OperationShape;
use swallowtail_runtime::ConsumerRouteProjectionSourceKind;

use super::fixtures::{observed_dispositions, profile_contributions};
use super::ledger::*;
use super::naming::{RowIdentity, semantic_id};

fn identity(entry: &LedgerEntry) -> RowIdentity {
    (
        entry.route_id.to_owned(),
        entry.operation_shape,
        entry.semantic_id.to_owned(),
    )
}

fn claimed(profile: &str) -> BTreeSet<RowIdentity> {
    entries()
        .filter(|entry| entry.emitted_by.contains(&profile))
        .map(identity)
        .collect()
}

#[test]
fn independent_ledgers_disposition_exactly_thirty_twelve_and_eleven_rows() {
    assert_eq!(AGENT_TRANCHE.len(), 30);
    assert_eq!(CODE_TRANCHE.len(), 12);
    assert_eq!(RESPONSE_TRANCHE.len(), 11);
    let mut tuples = BTreeSet::new();
    let mut counts = BTreeMap::new();
    for entry in entries() {
        assert!(
            tuples.insert(identity(entry)),
            "duplicate census tuple {:?}",
            identity(entry)
        );
        assert_eq!(
            entry.emitted_by.is_empty(),
            !entry.withheld_because.is_empty(),
            "{} must be emitted or withheld with a reason",
            entry.semantic_id
        );
        for profile in entry.emitted_by {
            assert!(PROFILES.contains(profile), "unknown profile {profile}");
        }
        *counts.entry(entry.route_id).or_insert(0) += 1;
    }
    assert_eq!(tuples.len(), 53);
    assert_eq!(counts[AGENT_ROUTE], 30);
    assert_eq!(counts[CODE_ROUTE], 12);
    assert_eq!(counts[RESPONSE_ROUTE], 11);
}

#[test]
fn every_profile_emits_exactly_the_tuples_claimed_by_its_ledger() {
    let observed = observed_dispositions();
    assert_eq!(observed.len(), PROFILES.len());
    for profile in PROFILES {
        assert_eq!(
            observed.get(profile).expect("profile contributes"),
            &claimed(profile),
            "{profile} drifted from its exact ledger"
        );
    }
}

#[test]
fn withheld_rows_are_constructed_by_no_profile_and_no_superset_is_filtered() {
    let emitted = observed_dispositions()
        .into_values()
        .flatten()
        .collect::<BTreeSet<_>>();
    let ledger = entries().map(identity).collect::<BTreeSet<_>>();
    assert!(emitted.is_subset(&ledger));
    for entry in entries() {
        assert_eq!(
            emitted.contains(&identity(entry)),
            !entry.emitted_by.is_empty(),
            "{} disposition disagrees with construction",
            entry.semantic_id
        );
    }
    assert_eq!(
        emitted.len(),
        entries()
            .filter(|entry| !entry.emitted_by.is_empty())
            .count()
    );
}

#[test]
fn each_profile_binds_every_row_to_its_exact_prepared_applicability() {
    let expected = BTreeMap::from([
        (AGENT_RUN, OperationShape::StructuredRun),
        (AGENT_SESSION, OperationShape::InteractiveSession),
        (AGENT_DELETE, OperationShape::ProviderSessionManagement),
        (AGENT_OBSERVED, OperationShape::InteractiveSession),
        (CODE_RUN, OperationShape::StructuredRun),
        (RESPONSE_RUN, OperationShape::StructuredRun),
    ]);
    for (profile, contribution) in profile_contributions() {
        assert_eq!(
            contribution.applicability().operation_shape(),
            expected[profile],
            "{profile} uses the wrong operation"
        );
        for row in contribution
            .selection_rows()
            .chain(contribution.session_start_rows())
            .chain(contribution.active_session_rows())
        {
            assert_eq!(row.applicability(), contribution.applicability());
        }
    }
}

#[test]
fn only_the_acknowledgement_row_uses_the_active_observation_source() {
    for (profile, contribution) in profile_contributions() {
        for row in contribution
            .selection_rows()
            .chain(contribution.session_start_rows())
            .chain(contribution.active_session_rows())
        {
            let expected = if semantic_id(row.identity()) == "feature.active-session-reasoning-ack"
            {
                assert_eq!(profile, AGENT_OBSERVED);
                ConsumerRouteProjectionSourceKind::ActiveSessionObservation
            } else {
                ConsumerRouteProjectionSourceKind::AdapterContribution
            };
            assert_eq!(
                row.source().kind(),
                expected,
                "{profile}: {:?}",
                row.identity()
            );
        }
    }
}

#[test]
fn route_local_descriptors_are_bounded_to_their_exact_route_and_facade() {
    for (profile, contribution) in profile_contributions() {
        let expected_route = match profile {
            AGENT_RUN | AGENT_SESSION | AGENT_DELETE | AGENT_OBSERVED => AGENT_ROUTE,
            CODE_RUN => CODE_ROUTE,
            RESPONSE_RUN => RESPONSE_ROUTE,
            other => panic!("unknown profile {other}"),
        };
        for row in contribution
            .selection_rows()
            .chain(contribution.session_start_rows())
            .chain(contribution.active_session_rows())
        {
            if let Some(extension) = row.identity().namespaced_extension() {
                assert_eq!(extension.route(), expected_route);
                assert_eq!(
                    extension.version_segment(),
                    contribution.applicability().protocol_facade_id().as_str()
                );
            }
        }
    }
}
