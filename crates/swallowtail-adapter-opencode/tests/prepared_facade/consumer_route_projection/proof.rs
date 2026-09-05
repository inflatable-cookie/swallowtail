use std::collections::{BTreeMap, BTreeSet};

use super::fixtures::observed;
use super::ledger::{LEDGER, LedgerEntry, PROFILES, ROUTE};
use super::naming::RowIdentity;

fn identity(entry: &LedgerEntry) -> RowIdentity {
    (ROUTE.to_owned(), entry.shape, entry.semantic_id.to_owned())
}

fn claimed(profile: &str) -> BTreeSet<RowIdentity> {
    LEDGER
        .iter()
        .filter(|entry| entry.emitted_by.contains(&profile))
        .map(identity)
        .collect()
}

fn assert_ledger() {
    let mut identities = BTreeSet::new();
    for entry in &LEDGER {
        assert!(
            identities.insert(identity(entry)),
            "ledger repeats {}",
            entry.semantic_id
        );
        assert_eq!(
            entry.emitted_by.is_empty(),
            !entry.withheld_because.is_empty(),
            "{} must be emitted or withheld with a reason",
            entry.semantic_id
        );
        for profile in entry.emitted_by {
            assert!(
                PROFILES.contains(profile),
                "{} names an unknown prepared facade profile",
                entry.semantic_id
            );
        }
    }
    assert_eq!(LEDGER.len(), 35);
    assert_eq!(
        LEDGER
            .iter()
            .filter(|entry| !entry.emitted_by.is_empty())
            .count(),
        33
    );
    assert_eq!(
        LEDGER
            .iter()
            .filter(|entry| entry.emitted_by.is_empty())
            .count(),
        2
    );
    assert!(
        LEDGER
            .iter()
            .filter(|entry| entry.emitted_by.is_empty())
            .all(|entry| entry.withheld_because.contains("matrix-descriptor-only"))
    );
}

fn assert_observed(observed: &BTreeMap<&'static str, BTreeSet<RowIdentity>>) {
    assert_eq!(observed.len(), PROFILES.len());
    for profile in PROFILES {
        assert_eq!(
            observed.get(profile).expect("profile contributes"),
            &claimed(profile),
            "{profile} emitted identities differ from its ledger claims"
        );
    }
    let published = observed
        .values()
        .flatten()
        .cloned()
        .collect::<BTreeSet<_>>();
    let dispositions = LEDGER.iter().map(identity).collect::<BTreeSet<_>>();
    for row in &published {
        assert!(
            dispositions.contains(row),
            "{row:?} was published without a ledger disposition"
        );
    }
    for entry in &LEDGER {
        if entry.emitted_by.is_empty() {
            assert!(
                !published.contains(&identity(entry)),
                "{} is withheld but was published",
                entry.semantic_id
            );
        } else {
            assert!(
                published.contains(&identity(entry)),
                "{} is claimed but was never published",
                entry.semantic_id
            );
        }
    }
}

#[test]
fn opencode_ledger_reconciles_against_real_prepared_facade_contributions() {
    assert_ledger();
    assert_observed(&observed());
}

#[test]
fn opencode_matrix_only_rows_remain_withheld() {
    let withheld = LEDGER
        .iter()
        .filter(|entry| entry.emitted_by.is_empty())
        .collect::<Vec<_>>();
    assert_eq!(withheld.len(), 2);
    assert!(withheld.iter().any(|entry| {
        entry.semantic_id == "control.reasoning-selection"
            && entry
                .withheld_because
                .contains("no retained interactive-session owner")
    }));
    assert!(withheld.iter().any(|entry| {
        entry.semantic_id == "control.provider-turn-reference"
            && entry
                .withheld_because
                .contains("reconciliation rejects provider turn references")
    }));
}
