use std::collections::BTreeSet;
use swallowtail_core::OperationShape;

use super::ZCODE_APP_SERVER_ROUTE;
use super::fixtures::{observed_dispositions, prepared_operation_shapes};
use super::ledger::*;
use super::naming::RowIdentity;

/// Returns the exact `(route_id, operation_shape, semantic_id)` of one entry.
fn identity(entry: &LedgerEntry) -> RowIdentity {
    (
        entry.route_id.to_owned(),
        entry.operation_shape,
        entry.semantic_id.to_owned(),
    )
}

/// Returns the exact ledger tuples one prepared profile claims.
fn claimed(profile: &str) -> BTreeSet<RowIdentity> {
    ZCODE_APP_SERVER_TRANCHE
        .iter()
        .filter(|entry| entry.emitted_by.contains(&profile))
        .map(identity)
        .collect()
}

#[test]
fn the_coverage_ledger_dispositions_exactly_the_twelve_zcode_rows() {
    let mut tuples = BTreeSet::new();
    let mut semantics = BTreeSet::new();
    for entry in &ZCODE_APP_SERVER_TRANCHE {
        assert_eq!(
            entry.route_id, ZCODE_APP_SERVER_ROUTE,
            "{} does not belong to the exact census route",
            entry.semantic_id
        );
        assert!(
            ZCODE_OPERATION_SHAPES.contains(&entry.operation_shape),
            "{} names an operation shape outside the zcode.app-server census",
            entry.semantic_id
        );
        assert!(
            tuples.insert(identity(entry)),
            "the ledger repeats the exact identity of {}",
            entry.semantic_id
        );
        assert!(
            semantics.insert(entry.semantic_id),
            "the ledger repeats {}",
            entry.semantic_id
        );
        assert!(
            entry.semantic_id.starts_with("feature.")
                || entry.semantic_id.starts_with("control.")
                || entry.semantic_id.starts_with("audit."),
            "{} is not a census row identity",
            entry.semantic_id
        );
        assert_eq!(
            entry.emitted_by.is_empty(),
            !entry.withheld_because.is_empty(),
            "{} must be either emitted or withheld with a reason",
            entry.semantic_id
        );
        for profile in entry.emitted_by {
            assert!(
                ZCODE_PROFILES.contains(profile),
                "{} names an unknown prepared ZCode profile",
                entry.semantic_id
            );
        }
    }
    assert_eq!(ZCODE_APP_SERVER_TRANCHE.len(), 12);
    assert_eq!(tuples.len(), 12);
    assert_eq!(semantics.len(), 12);
}

#[test]
fn every_prepared_zcode_profile_emits_exactly_its_ledger_identities() {
    let observed = observed_dispositions();
    assert_eq!(observed.len(), ZCODE_PROFILES.len());
    for profile in ZCODE_PROFILES {
        let published = observed.get(profile).expect("every profile contributes");
        assert_eq!(
            published,
            &claimed(profile),
            "{profile} emitted identities differ from the coverage ledger"
        );
    }
}

#[test]
fn every_emitted_row_carries_the_prepared_structured_run_operation_shape() {
    for (profile, shape) in prepared_operation_shapes() {
        assert_eq!(
            shape,
            OperationShape::StructuredRun,
            "{profile} publishes rows under an operation shape this route never prepares"
        );
    }
}

#[test]
fn withheld_zcode_rows_are_emitted_by_no_prepared_profile() {
    let observed = observed_dispositions();
    let emitted = observed
        .values()
        .flatten()
        .cloned()
        .collect::<BTreeSet<_>>();
    let ledger = ZCODE_APP_SERVER_TRANCHE
        .iter()
        .map(identity)
        .collect::<BTreeSet<_>>();
    for published in &emitted {
        assert!(
            ledger.contains(published),
            "{published:?} is published without a recorded disposition"
        );
    }
    for entry in &ZCODE_APP_SERVER_TRANCHE {
        if entry.emitted_by.is_empty() {
            assert!(
                !emitted.contains(&identity(entry)),
                "{} is withheld but was published",
                entry.semantic_id
            );
        } else {
            assert!(
                emitted.contains(&identity(entry)),
                "{} is claimed but was never published",
                entry.semantic_id
            );
        }
    }
    let published_semantics = emitted
        .iter()
        .map(|(_, _, semantic)| semantic.as_str())
        .collect::<BTreeSet<_>>();
    for off_route in WITHHELD_OFF_ROUTE {
        assert!(
            !published_semantics.contains(off_route),
            "{off_route} has no zcode.app-server census row and must never be constructed"
        );
        assert!(
            !ZCODE_APP_SERVER_TRANCHE
                .iter()
                .any(|entry| entry.semantic_id == off_route),
            "{off_route} must not appear in the twelve-row ledger"
        );
    }
    assert_eq!(
        emitted.len(),
        ZCODE_APP_SERVER_TRANCHE
            .iter()
            .filter(|entry| !entry.emitted_by.is_empty())
            .count(),
        "the published set is exactly the ledger's emitted identities"
    );
}
