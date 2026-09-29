//! Adapter ledger comparison: observed rows must match the emitting facade.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;

/// One Contract 061 ledger claim: the census tuple and the facades that emit it.
///
/// `identity` is the exact `(route_id, operation_shape, semantic_id)` the
/// contributing facade must publish. Empty `emitted_by` withholds the row.
#[derive(Clone, Debug)]
pub struct ConsumerRouteLedgerClaim<'a, T> {
    /// Exact census tuple the observation must publish or withhold.
    pub identity: T,
    /// Prepared facade names that claim this tuple. Empty withholds it.
    pub emitted_by: &'a [&'a str],
}

/// Asserts observed rows match the ledger per emitting facade.
///
/// `observed` maps each prepared facade to the exact census tuples it
/// published. A row that moves to another facade, or whose operation shape
/// changes, fails the comparison for that facade.
pub fn assert_consumer_route_ledger_emitted_by_facade<'a, T>(
    observed: &BTreeMap<&str, BTreeSet<T>>,
    ledger: impl IntoIterator<Item = ConsumerRouteLedgerClaim<'a, T>>,
) where
    T: Clone + Ord + Debug,
{
    let mut claimed: BTreeMap<&str, BTreeSet<T>> = BTreeMap::new();
    let mut withheld = BTreeSet::new();
    for claim in ledger {
        if claim.emitted_by.is_empty() {
            withheld.insert(claim.identity);
            continue;
        }
        for facade in claim.emitted_by {
            claimed
                .entry(*facade)
                .or_default()
                .insert(claim.identity.clone());
        }
    }

    let observed_facades = observed.keys().copied().collect::<BTreeSet<_>>();
    let claimed_facades = claimed.keys().copied().collect::<BTreeSet<_>>();
    assert_eq!(
        observed_facades, claimed_facades,
        "observed facades differ from the ledger's emitting facades"
    );

    for (facade, expected) in &claimed {
        let published = observed
            .get(facade)
            .unwrap_or_else(|| panic!("{facade} is claimed by the ledger but was not observed"));
        assert_eq!(
            published, expected,
            "{facade} emitted identities differ from the ledger"
        );
    }

    let published = observed
        .values()
        .flatten()
        .cloned()
        .collect::<BTreeSet<_>>();
    for identity in &withheld {
        assert!(
            !published.contains(identity),
            "{identity:?} is withheld but was published"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::{ConsumerRouteLedgerClaim, assert_consumer_route_ledger_emitted_by_facade};
    use std::collections::{BTreeMap, BTreeSet};

    type Tuple = (&'static str, &'static str, &'static str);

    const ROUTE: &str = "fixture.route";
    const RUN: &str = "PreparedRun";
    const SESSION: &str = "PreparedSession";
    const STREAMING: Tuple = (ROUTE, "route-observation", "feature.streaming-events");
    const STRUCTURED: Tuple = (ROUTE, "structured-run", "feature.structured-run");

    fn matching_observation() -> BTreeMap<&'static str, BTreeSet<Tuple>> {
        BTreeMap::from([
            (RUN, BTreeSet::from([STREAMING, STRUCTURED])),
            (SESSION, BTreeSet::from([STREAMING])),
        ])
    }

    fn matching_ledger() -> [ConsumerRouteLedgerClaim<'static, Tuple>; 3] {
        [
            ConsumerRouteLedgerClaim {
                identity: STREAMING,
                emitted_by: &[RUN, SESSION],
            },
            ConsumerRouteLedgerClaim {
                identity: STRUCTURED,
                emitted_by: &[RUN],
            },
            ConsumerRouteLedgerClaim {
                identity: (ROUTE, "interactive-session", "feature.interactive-session"),
                emitted_by: &[],
            },
        ]
    }

    #[test]
    fn matching_per_facade_tuples_pass() {
        assert_consumer_route_ledger_emitted_by_facade(&matching_observation(), matching_ledger());
    }

    #[test]
    #[should_panic(expected = "PreparedRun emitted identities differ from the ledger")]
    fn a_row_moving_to_another_facade_fails() {
        let mut observed = matching_observation();
        observed
            .get_mut(SESSION)
            .expect("session")
            .insert(STRUCTURED);
        observed.get_mut(RUN).expect("run").remove(&STRUCTURED);
        // BTreeMap compares PreparedRun before PreparedSession, so the moved
        // row fails on the source facade first.
        assert_consumer_route_ledger_emitted_by_facade(&observed, matching_ledger());
    }

    #[test]
    #[should_panic(expected = "PreparedRun emitted identities differ from the ledger")]
    fn a_changed_operation_shape_fails() {
        let mut observed = matching_observation();
        let run = observed.get_mut(RUN).expect("run");
        run.remove(&STREAMING);
        run.insert((ROUTE, "interactive-session", "feature.streaming-events"));
        assert_consumer_route_ledger_emitted_by_facade(&observed, matching_ledger());
    }
}
