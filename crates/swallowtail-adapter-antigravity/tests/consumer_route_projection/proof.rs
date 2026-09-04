use std::collections::BTreeSet;

use super::ledger::*;

#[test]
fn catalogue_tranche_dispositions_exactly_fourteen_rows() {
    assert_eq!(ANTIGRAVITY_CATALOGUE_TRANCHE.len(), 14);
    let mut emitted = 0;
    let mut withheld = 0;
    let mut ledger_tuples: BTreeSet<RowTuple> = BTreeSet::new();

    for entry in &ANTIGRAVITY_CATALOGUE_TRANCHE {
        assert_eq!(entry.route_id, ANTIGRAVITY_CATALOGUE_ROUTE);
        assert!(!entry.operation_shape.is_empty());
        assert!(
            ledger_tuples.insert((entry.route_id, entry.operation_shape, entry.semantic_id)),
            "duplicate tuple in ledger: ({}, {}, {})",
            entry.route_id,
            entry.operation_shape,
            entry.semantic_id
        );
        if entry.emitted_by.is_empty() {
            withheld += 1;
            assert!(
                !entry.withheld_because.is_empty(),
                "withheld row {} must specify reason",
                entry.semantic_id
            );
        } else {
            emitted += 1;
            assert!(entry.withheld_because.is_empty());
        }
    }

    assert_eq!(emitted, 2);
    assert_eq!(withheld, 12);

    let census_tuples: BTreeSet<RowTuple> =
        ANTIGRAVITY_CATALOGUE_CENSUS_TUPLES.into_iter().collect();
    assert_eq!(
        ledger_tuples, census_tuples,
        "antigravity.catalogue ledger tuples must match census tuples bidirectionally"
    );
}

#[test]
fn headless_tranche_dispositions_exactly_eighteen_rows() {
    assert_eq!(ANTIGRAVITY_HEADLESS_TRANCHE.len(), 18);
    let mut emitted = 0;
    let mut withheld = 0;
    let mut ledger_tuples: BTreeSet<RowTuple> = BTreeSet::new();

    for entry in &ANTIGRAVITY_HEADLESS_TRANCHE {
        assert_eq!(entry.route_id, ANTIGRAVITY_HEADLESS_ROUTE);
        assert!(!entry.operation_shape.is_empty());
        assert!(
            ledger_tuples.insert((entry.route_id, entry.operation_shape, entry.semantic_id)),
            "duplicate tuple in ledger: ({}, {}, {})",
            entry.route_id,
            entry.operation_shape,
            entry.semantic_id
        );
        if entry.emitted_by.is_empty() {
            withheld += 1;
            assert!(
                !entry.withheld_because.is_empty(),
                "withheld row {} must specify reason",
                entry.semantic_id
            );
        } else {
            emitted += 1;
            assert!(entry.withheld_because.is_empty());
        }
    }

    assert_eq!(emitted, 16);
    assert_eq!(withheld, 2);

    let census_tuples: BTreeSet<RowTuple> =
        ANTIGRAVITY_HEADLESS_CENSUS_TUPLES.into_iter().collect();
    assert_eq!(
        ledger_tuples, census_tuples,
        "antigravity.headless ledger tuples must match census tuples bidirectionally"
    );
}
