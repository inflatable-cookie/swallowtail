use std::collections::BTreeSet;
use swallowtail_core::OperationShape;

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
    DEEPAGENTS_ACP_TRANCHE
        .iter()
        .filter(|entry| entry.emitted_by.contains(&profile))
        .map(identity)
        .collect()
}

#[test]
fn the_coverage_ledger_dispositions_exactly_the_nine_deepagents_rows() {
    let mut tuples = BTreeSet::new();
    let mut semantics = BTreeSet::new();
    for entry in &DEEPAGENTS_ACP_TRANCHE {
        assert_eq!(
            entry.route_id, DEEPAGENTS_ROUTE,
            "{} does not belong to the exact census route",
            entry.semantic_id
        );
        assert!(
            DEEPAGENTS_OPERATION_SHAPES.contains(&entry.operation_shape),
            "{} names an operation shape outside the deepagents.acp census",
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
                DEEPAGENTS_PROFILES.contains(profile),
                "{} names an unknown prepared Deep Agents profile",
                entry.semantic_id
            );
        }
    }
    assert_eq!(DEEPAGENTS_ACP_TRANCHE.len(), 9);
    assert_eq!(tuples.len(), 9);
    assert_eq!(semantics.len(), 9);
}

#[test]
fn every_prepared_deepagents_profile_emits_exactly_its_ledger_identities() {
    let observed = observed_dispositions();
    assert_eq!(observed.len(), DEEPAGENTS_PROFILES.len());
    for profile in DEEPAGENTS_PROFILES {
        let published = observed.get(profile).expect("every profile contributes");
        assert_eq!(
            published,
            &claimed(profile),
            "{profile} emitted identities differ from the coverage ledger"
        );
    }
}

#[test]
fn every_emitted_row_carries_the_prepared_interactive_session_operation_shape() {
    for (profile, shape) in prepared_operation_shapes() {
        assert_eq!(
            shape,
            OperationShape::InteractiveSession,
            "{profile} publishes rows under an operation shape this route never prepares"
        );
    }
}

#[test]
fn withheld_deepagents_rows_are_emitted_by_no_prepared_profile() {
    let observed = observed_dispositions();
    let emitted = observed
        .values()
        .flatten()
        .cloned()
        .collect::<BTreeSet<_>>();
    let ledger = DEEPAGENTS_ACP_TRANCHE
        .iter()
        .map(identity)
        .collect::<BTreeSet<_>>();
    for published in &emitted {
        assert!(
            ledger.contains(published),
            "{published:?} is published without a recorded disposition"
        );
    }
    for entry in &DEEPAGENTS_ACP_TRANCHE {
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
            "{off_route} has no deepagents.acp census row and must never be constructed"
        );
        assert!(
            !DEEPAGENTS_ACP_TRANCHE
                .iter()
                .any(|entry| entry.semantic_id == off_route),
            "{off_route} must not appear in the nine-row ledger"
        );
    }
    assert_eq!(
        emitted.len(),
        DEEPAGENTS_ACP_TRANCHE
            .iter()
            .filter(|entry| !entry.emitted_by.is_empty())
            .count(),
        "the published set is exactly the ledger's emitted identities"
    );
}
