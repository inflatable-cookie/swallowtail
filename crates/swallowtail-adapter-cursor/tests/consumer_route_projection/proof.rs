use std::collections::BTreeSet;

use super::ledger::*;

#[test]
fn acp_tranche_dispositions_exactly_thirteen_rows() {
    assert_eq!(CURSOR_ACP_TRANCHE.len(), 13);
    let mut ledger_tuples = BTreeSet::new();
    let mut semantics = BTreeSet::new();
    let mut emitted_count = 0;
    let mut withheld_count = 0;

    for entry in &CURSOR_ACP_TRANCHE {
        assert_eq!(entry.route_id, CURSOR_ACP_ROUTE);
        assert!(!entry.operation_shape.is_empty());
        assert!(ledger_tuples.insert((entry.route_id, entry.operation_shape, entry.semantic_id)));
        assert!(semantics.insert(entry.semantic_id));
        if entry.emitted_by.is_empty() {
            withheld_count += 1;
            assert!(!entry.withheld_because.is_empty());
        } else {
            emitted_count += 1;
            assert!(entry.withheld_because.is_empty());
        }
    }

    let census_tuples: BTreeSet<RowTuple> = CURSOR_ACP_CENSUS_TUPLES.into_iter().collect();
    assert_eq!(
        ledger_tuples, census_tuples,
        "cursor-agent.acp ledger tuples must match census tuples bidirectionally"
    );
    assert_eq!(emitted_count, 7);
    assert_eq!(withheld_count, 6);
}

#[test]
fn catalogue_tranche_dispositions_exactly_thirteen_rows() {
    assert_eq!(CURSOR_CATALOGUE_TRANCHE.len(), 13);
    let mut ledger_tuples = BTreeSet::new();
    let mut semantics = BTreeSet::new();
    let mut emitted_count = 0;
    let mut withheld_count = 0;

    for entry in &CURSOR_CATALOGUE_TRANCHE {
        assert_eq!(entry.route_id, CURSOR_CATALOGUE_ROUTE);
        assert!(!entry.operation_shape.is_empty());
        assert!(ledger_tuples.insert((entry.route_id, entry.operation_shape, entry.semantic_id)));
        assert!(semantics.insert(entry.semantic_id));
        if entry.emitted_by.is_empty() {
            withheld_count += 1;
            assert!(!entry.withheld_because.is_empty());
        } else {
            emitted_count += 1;
            assert!(entry.withheld_because.is_empty());
        }
    }

    let census_tuples: BTreeSet<RowTuple> = CURSOR_CATALOGUE_CENSUS_TUPLES.into_iter().collect();
    assert_eq!(
        ledger_tuples, census_tuples,
        "cursor-agent.catalogue ledger tuples must match census tuples bidirectionally"
    );
    assert_eq!(emitted_count, 2);
    assert_eq!(withheld_count, 11);
}

#[test]
fn headless_tranche_dispositions_exactly_seventeen_rows() {
    assert_eq!(CURSOR_HEADLESS_TRANCHE.len(), 17);
    let mut ledger_tuples = BTreeSet::new();
    let mut semantics = BTreeSet::new();
    let mut emitted_count = 0;
    let mut withheld_count = 0;

    for entry in &CURSOR_HEADLESS_TRANCHE {
        assert_eq!(entry.route_id, CURSOR_HEADLESS_ROUTE);
        assert!(!entry.operation_shape.is_empty());
        assert!(ledger_tuples.insert((entry.route_id, entry.operation_shape, entry.semantic_id)));
        assert!(semantics.insert(entry.semantic_id));
        if entry.emitted_by.is_empty() {
            withheld_count += 1;
            assert!(!entry.withheld_because.is_empty());
        } else {
            emitted_count += 1;
            assert!(entry.withheld_because.is_empty());
        }
    }

    let census_tuples: BTreeSet<RowTuple> = CURSOR_HEADLESS_CENSUS_TUPLES.into_iter().collect();
    assert_eq!(
        ledger_tuples, census_tuples,
        "cursor-agent.headless ledger tuples must match census tuples bidirectionally"
    );
    assert_eq!(emitted_count, 14);
    assert_eq!(withheld_count, 3);
}

#[test]
fn package_dispositions_forty_three_rows_total() {
    let acp_len = CURSOR_ACP_TRANCHE.len();
    let catalogue_len = CURSOR_CATALOGUE_TRANCHE.len();
    let headless_len = CURSOR_HEADLESS_TRANCHE.len();
    assert_eq!(acp_len + catalogue_len + headless_len, 43);

    let acp_emitted = CURSOR_ACP_TRANCHE
        .iter()
        .filter(|e| !e.emitted_by.is_empty())
        .count();
    let catalogue_emitted = CURSOR_CATALOGUE_TRANCHE
        .iter()
        .filter(|e| !e.emitted_by.is_empty())
        .count();
    let headless_emitted = CURSOR_HEADLESS_TRANCHE
        .iter()
        .filter(|e| !e.emitted_by.is_empty())
        .count();
    assert_eq!(acp_emitted + catalogue_emitted + headless_emitted, 23);

    let acp_withheld = CURSOR_ACP_TRANCHE
        .iter()
        .filter(|e| e.emitted_by.is_empty())
        .count();
    let catalogue_withheld = CURSOR_CATALOGUE_TRANCHE
        .iter()
        .filter(|e| e.emitted_by.is_empty())
        .count();
    let headless_withheld = CURSOR_HEADLESS_TRANCHE
        .iter()
        .filter(|e| e.emitted_by.is_empty())
        .count();
    assert_eq!(acp_withheld + catalogue_withheld + headless_withheld, 20);
}
