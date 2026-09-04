use std::collections::BTreeSet;

use super::fixtures::observed_dispositions;
use super::ledger::*;
use super::naming::RowIdentity;

fn identity(entry: &LedgerEntry) -> RowIdentity {
    (
        entry.route_id.to_owned(),
        entry.operation_shape,
        entry.semantic_id.to_owned(),
    )
}

fn claimed(profile: &str) -> BTreeSet<RowIdentity> {
    OLLAMA_ATTACHED_TRANCHE
        .iter()
        .filter(|entry| entry.emitted_by.contains(&profile))
        .map(identity)
        .collect()
}

#[test]
fn the_coverage_ledger_dispositions_exactly_the_nineteen_ollama_rows() {
    let mut tuples = BTreeSet::new();
    for entry in &OLLAMA_ATTACHED_TRANCHE {
        assert_eq!(entry.route_id, OLLAMA_ROUTE);
        assert!(
            OLLAMA_SHAPES.contains(&entry.operation_shape),
            "{} names an operation shape outside the ollama.attached census",
            entry.semantic_id
        );
        assert!(
            tuples.insert(identity(entry)),
            "the ledger repeats the exact identity of {}/{}",
            entry.operation_shape,
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
        for profile in entry.emitted_by {
            assert!(
                OLLAMA_PROFILES.contains(profile),
                "{} names an unknown prepared Ollama profile",
                entry.semantic_id
            );
        }
    }
    assert_eq!(OLLAMA_ATTACHED_TRANCHE.len(), 19);
    assert_eq!(tuples.len(), 19);
    assert_eq!(
        OLLAMA_ATTACHED_TRANCHE
            .iter()
            .filter(|entry| entry.emitted_by.is_empty())
            .count(),
        1
    );
}

#[test]
fn every_prepared_ollama_profile_emits_exactly_its_ledger_identities() {
    let observed = observed_dispositions();
    assert_eq!(observed.len(), OLLAMA_PROFILES.len());
    for profile in OLLAMA_PROFILES {
        assert_eq!(
            observed.get(profile).expect("every profile contributes"),
            &claimed(profile),
            "{profile} emitted identities differ from the coverage ledger"
        );
    }
}

#[test]
fn withheld_ollama_rows_are_emitted_by_no_prepared_profile() {
    let observed = observed_dispositions();
    let emitted = observed
        .values()
        .flatten()
        .cloned()
        .collect::<BTreeSet<_>>();
    let ledger = OLLAMA_ATTACHED_TRANCHE
        .iter()
        .map(identity)
        .collect::<BTreeSet<_>>();
    for published in &emitted {
        assert!(
            ledger.contains(published),
            "{published:?} is published without a recorded disposition"
        );
    }
    for entry in &OLLAMA_ATTACHED_TRANCHE {
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
            "{off_route} has no ollama.attached census row and must never be constructed"
        );
        assert!(
            !OLLAMA_ATTACHED_TRANCHE
                .iter()
                .any(|entry| entry.semantic_id == off_route),
            "{off_route} must not appear in the nineteen-row ledger"
        );
    }
    assert_eq!(
        emitted.len(),
        OLLAMA_ATTACHED_TRANCHE
            .iter()
            .filter(|entry| !entry.emitted_by.is_empty())
            .count(),
        "the published set is exactly the ledger's emitted identities"
    );
}
