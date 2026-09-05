use std::collections::{BTreeMap, BTreeSet};

use super::fixtures::{observed_rpc, observed_sidecar};
use super::ledger::{
    LedgerEntry, PI_RPC_LEDGER, PI_SIDECAR_LEDGER, RPC_PROFILES, SIDECAR_PROFILES,
};
use super::naming::RowIdentity;

fn identity(entry: &LedgerEntry) -> RowIdentity {
    (
        entry.route_id.to_owned(),
        entry.operation_shape,
        entry.semantic_id.to_owned(),
    )
}

fn claimed(ledger: &[LedgerEntry], profile: &str) -> BTreeSet<RowIdentity> {
    ledger
        .iter()
        .filter(|entry| entry.emitted_by.contains(&profile))
        .map(identity)
        .collect()
}

fn assert_ledger(
    ledger: &[LedgerEntry],
    route: &str,
    profiles: &[&str],
    emitted: usize,
    withheld: usize,
) {
    let mut identities = BTreeSet::new();
    for entry in ledger {
        assert_eq!(entry.route_id, route, "{} is off-route", entry.semantic_id);
        assert!(
            identities.insert(identity(entry)),
            "the ledger repeats {}",
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
                profiles.contains(profile),
                "{} names an unknown prepared facade profile",
                entry.semantic_id
            );
        }
    }
    assert_eq!(ledger.len(), emitted + withheld);
    assert_eq!(
        ledger
            .iter()
            .filter(|entry| !entry.emitted_by.is_empty())
            .count(),
        emitted
    );
    assert_eq!(
        ledger
            .iter()
            .filter(|entry| entry.emitted_by.is_empty())
            .count(),
        withheld
    );
}

fn assert_observed(
    observed: &BTreeMap<&'static str, BTreeSet<RowIdentity>>,
    ledger: &[LedgerEntry],
    profiles: &[&str],
) {
    assert_eq!(observed.len(), profiles.len());
    for profile in profiles {
        assert_eq!(
            observed
                .get(profile)
                .expect("every prepared facade contributes"),
            &claimed(ledger, profile),
            "{profile} emitted identities differ from its ledger claims"
        );
    }
    let published = observed
        .values()
        .flatten()
        .cloned()
        .collect::<BTreeSet<_>>();
    let ledger_identities = ledger.iter().map(identity).collect::<BTreeSet<_>>();
    for row in &published {
        assert!(
            ledger_identities.contains(row),
            "{row:?} was published without a ledger disposition"
        );
    }
    for entry in ledger {
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
fn pi_ledgers_reconcile_against_real_prepared_facade_contributions() {
    assert_ledger(&PI_RPC_LEDGER, "pi.rpc", &RPC_PROFILES, 15, 0);
    assert_ledger(
        &PI_SIDECAR_LEDGER,
        "pi.sdk-sidecar",
        &SIDECAR_PROFILES,
        16,
        3,
    );
    assert_observed(&observed_rpc(), &PI_RPC_LEDGER, &RPC_PROFILES);
    assert_observed(&observed_sidecar(), &PI_SIDECAR_LEDGER, &SIDECAR_PROFILES);
}

#[test]
fn pi_sidecar_withheld_rows_keep_anchored_plan_reasons() {
    let withheld = PI_SIDECAR_LEDGER
        .iter()
        .filter(|entry| entry.emitted_by.is_empty())
        .collect::<Vec<_>>();
    assert_eq!(withheld.len(), 3);
    assert!(withheld.iter().any(|entry| {
        entry.semantic_id == "feature.model-catalogue"
            && entry
                .withheld_because
                .contains("no catalogue role or route")
    }));
    assert!(withheld.iter().any(|entry| {
        entry.semantic_id == "feature.usage-evidence"
            && entry
                .withheld_because
                .contains("Capability::UsageReporting")
    }));
    assert!(withheld.iter().any(|entry| {
        entry.semantic_id == "feature.activity-observation"
            && entry
                .withheld_because
                .contains("ObservableActivityProfile-derived")
    }));
}
