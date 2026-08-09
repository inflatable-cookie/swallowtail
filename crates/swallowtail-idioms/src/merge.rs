use crate::record::Idiom;
use crate::time::MonotonicInstant;

/// Deterministic outcome of merging one idiom into a store (Contract 055).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MergeOutcome {
    /// The incoming idiom has an id not present in the store.
    New,
    /// The incoming idiom raises effective confidence for its id.
    Raised,
    /// The incoming idiom lowers effective confidence for its id.
    Lowered,
    /// The incoming idiom leaves effective confidence unchanged.
    Unchanged,
}

/// Merges one idiom into a store and reports the deterministic outcome.
///
/// An id absent from the store is always `New`. For a present id, the
/// outcome compares effective confidence at the evaluation instant; the
/// returned record is the incoming one (fresh evidence wins). A record
/// never changes without a signal or a merge.
#[must_use]
pub fn merge(
    existing: Option<&Idiom>,
    incoming: Idiom,
    at: MonotonicInstant,
) -> (Idiom, MergeOutcome) {
    let Some(existing) = existing else {
        return (incoming, MergeOutcome::New);
    };
    if existing.id() != incoming.id() {
        return (incoming, MergeOutcome::New);
    }
    let current = existing.effective_confidence(at);
    let next = incoming.effective_confidence(at);
    let outcome = if next > current {
        MergeOutcome::Raised
    } else if next < current {
        MergeOutcome::Lowered
    } else {
        MergeOutcome::Unchanged
    };
    (incoming, outcome)
}

#[cfg(test)]
mod tests {
    use super::{MergeOutcome, merge};
    use crate::bounded::BoundedText;
    use crate::confidence::DEFAULT_DECAY_HALF_LIFE_TICKS;
    use crate::record::{Idiom, IdiomConstraint, IdiomId, IdiomScope, Provenance};
    use crate::time::MonotonicInstant;

    fn at(ticks: u64) -> MonotonicInstant {
        MonotonicInstant::from_ticks(ticks)
    }

    fn static_source() -> Provenance {
        Provenance::Static(BoundedText::new("team rules", 256).expect("bounded source"))
    }

    fn idiom(id: &str, confidence: u8, as_of_ticks: u64) -> Idiom {
        Idiom::new(
            IdiomId::new(id).expect("id"),
            IdiomScope::Project,
            IdiomConstraint::text("use named exports").expect("constraint"),
            confidence,
            at(as_of_ticks),
            static_source(),
        )
        .expect("idiom")
    }

    #[test]
    fn absent_id_is_new() {
        let (result, outcome) = merge(None, idiom("a", 50, 0), at(0));
        assert_eq!(outcome, MergeOutcome::New);
        assert_eq!(result.id().as_str(), "a");
    }

    #[test]
    fn different_id_is_new() {
        let existing = idiom("a", 90, 0);
        let incoming = idiom("b", 10, 0);
        let (result, outcome) = merge(Some(&existing), incoming, at(0));
        assert_eq!(outcome, MergeOutcome::New);
        assert_eq!(result.id().as_str(), "b");
    }

    #[test]
    fn same_id_compares_effective_confidence() {
        let existing = idiom("a", 90, 0);
        let half_life = DEFAULT_DECAY_HALF_LIFE_TICKS;

        let raised = idiom_at_half_life("a", 100, 0, half_life);
        let (result, outcome) = merge(Some(&existing), raised, at(0));
        assert_eq!(outcome, MergeOutcome::Raised);
        assert_eq!(result.confidence().value(), 100);

        let lowered = idiom_at_half_life("a", 10, 0, half_life);
        let (_, outcome) = merge(Some(&existing), lowered, at(0));
        assert_eq!(outcome, MergeOutcome::Lowered);

        let same = idiom_at_half_life("a", 90, 0, half_life);
        let (_, outcome) = merge(Some(&existing), same, at(0));
        assert_eq!(outcome, MergeOutcome::Unchanged);
    }

    #[test]
    fn decayed_existing_can_make_flat_incoming_raise() {
        let existing = idiom("a", 100, 0);
        let incoming = idiom("a", 60, DEFAULT_DECAY_HALF_LIFE_TICKS * 2);
        let (_, outcome) = merge(Some(&existing), incoming, at(DEFAULT_DECAY_HALF_LIFE_TICKS));
        assert_eq!(outcome, MergeOutcome::Raised);
    }

    fn idiom_at_half_life(id: &str, confidence: u8, as_of_ticks: u64, half_life: u64) -> Idiom {
        Idiom::with_half_life(
            IdiomId::new(id).expect("id"),
            IdiomScope::Project,
            IdiomConstraint::text("use named exports").expect("constraint"),
            confidence,
            at(as_of_ticks),
            half_life,
            static_source(),
        )
        .expect("idiom")
    }
}
