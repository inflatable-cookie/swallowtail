use std::collections::BTreeSet;

use super::fixtures::observed_dispositions;
use super::ledger::*;

#[test]
fn the_coverage_ledger_dispositions_exactly_the_thirty_five_exec_rows() {
    let mut ids = BTreeSet::new();
    for entry in &CODEX_EXEC_TRANCHE {
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
        for profile in entry.emitted_by {
            assert!(
                EXEC_PROFILES.contains(profile),
                "{} names an unknown prepared exec profile",
                entry.semantic_id
            );
        }
    }
    assert_eq!(CODEX_EXEC_TRANCHE.len(), 35);
    assert_eq!(ids.len(), 35);
}

#[test]
fn every_prepared_exec_profile_emits_exactly_its_ledger_rows() {
    let observed = observed_dispositions();
    assert_eq!(observed.len(), EXEC_PROFILES.len());
    for profile in EXEC_PROFILES {
        let expected = CODEX_EXEC_TRANCHE
            .iter()
            .filter(|entry| entry.emitted_by.contains(&profile))
            .map(|entry| entry.semantic_id.to_owned())
            .collect::<BTreeSet<_>>();
        let published = observed.get(profile).expect("every profile contributes");
        assert_eq!(
            published, &expected,
            "{profile} emitted rows differ from the coverage ledger"
        );
    }
}

#[test]
fn withheld_exec_rows_are_emitted_by_no_prepared_profile() {
    let observed = observed_dispositions();
    let emitted = observed
        .values()
        .flatten()
        .cloned()
        .collect::<BTreeSet<_>>();
    let ledger = CODEX_EXEC_TRANCHE
        .iter()
        .map(|entry| entry.semantic_id.to_owned())
        .collect::<BTreeSet<_>>();
    for published in &emitted {
        assert!(
            ledger.contains(published),
            "{published} is published without a recorded disposition"
        );
    }
    for entry in &CODEX_EXEC_TRANCHE {
        if entry.emitted_by.is_empty() {
            assert!(
                !emitted.contains(entry.semantic_id),
                "{} is withheld but was published",
                entry.semantic_id
            );
        } else {
            assert!(
                emitted.contains(entry.semantic_id),
                "{} is claimed but was never published",
                entry.semantic_id
            );
        }
    }
    for borrowed in WITHHELD_APP_SERVER_ONLY {
        assert!(
            !emitted.contains(borrowed),
            "{borrowed} belongs to codex.app-server and must never be constructed here"
        );
        assert!(
            !CODEX_EXEC_TRANCHE
                .iter()
                .any(|entry| entry.semantic_id == borrowed),
            "{borrowed} must not appear in the 35-row ledger"
        );
    }
    assert_eq!(
        emitted.len(),
        CODEX_EXEC_TRANCHE
            .iter()
            .filter(|entry| !entry.emitted_by.is_empty())
            .count(),
        "the published set is exactly the ledger's emitted rows"
    );
}
