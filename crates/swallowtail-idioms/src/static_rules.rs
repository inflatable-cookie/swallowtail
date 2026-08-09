use crate::engine::{IdiomContext, IdiomSet, IdiomSource, select_bounded};
use crate::record::{Idiom, IdiomScope};
use crate::time::MonotonicInstant;

/// Whether a record with `record_scope` applies to a context scope.
///
/// User-scope records apply to every context; otherwise the record must
/// match the context scope exactly. Hosts that want project-plus-package
/// layering compose one source per scope.
fn scope_applies(record_scope: &IdiomScope, context_scope: &IdiomScope) -> bool {
    matches!(record_scope, IdiomScope::User) || record_scope == context_scope
}

/// Static-rules backend implementing `IdiomSource` over portable records
/// (Contract 055).
///
/// Selection keeps records whose scope matches the context scope (plus
/// user-scope records) and delegates ordering and bounding to
/// `select_bounded`. It never mutates its records, composes prompts, or
/// enforces permissions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StaticRulesSource {
    records: Vec<Idiom>,
}

impl StaticRulesSource {
    /// Creates a static-rules source from validated idiom records.
    #[must_use]
    pub fn new(records: Vec<Idiom>) -> Self {
        Self { records }
    }

    /// Creates a static-rules source from a rule store slice.
    #[must_use]
    pub fn from_store(records: &[Idiom]) -> Self {
        Self::new(records.to_vec())
    }

    /// Returns the records this source holds.
    #[must_use]
    pub fn records(&self) -> &[Idiom] {
        &self.records
    }
}

impl IdiomSource for StaticRulesSource {
    fn select(&self, context: &IdiomContext) -> IdiomSet {
        let applicable: Vec<Idiom> = self
            .records
            .iter()
            .filter(|record| scope_applies(record.scope(), context.scope()))
            .cloned()
            .collect();
        select_bounded(applicable, context.at(), context.maximum())
    }
}

/// Resolves one bounded, ordered idiom set for session preparation.
///
/// This is the host-facing delivery seam: a consumer calls it when preparing
/// a session and receives a bounded `IdiomSet` to map into its own prompt
/// layer. The mechanism never composes or mutates prompt text.
///
/// Headless posture: headless routes have no accept/reject loop, so they
/// receive static rules only; any learned layer stays opt-in where a
/// consumer supplies signals.
#[must_use]
pub fn prepare_session_idioms(
    source: &dyn IdiomSource,
    scope: IdiomScope,
    at: MonotonicInstant,
    maximum: usize,
) -> IdiomSet {
    let context = IdiomContext::new(scope, at, maximum);
    source.select(&context)
}

#[cfg(test)]
mod tests {
    use super::{StaticRulesSource, prepare_session_idioms, scope_applies};
    use crate::bounded::BoundedText;
    use crate::engine::{IdiomContext, IdiomSource};
    use crate::record::{Idiom, IdiomConstraint, IdiomId, IdiomScope, Provenance};
    use crate::time::MonotonicInstant;

    fn at(ticks: u64) -> MonotonicInstant {
        MonotonicInstant::from_ticks(ticks)
    }

    fn static_source() -> Provenance {
        Provenance::Static(BoundedText::new("team rules", 256).expect("bounded source"))
    }

    fn idiom(id: &str, scope: IdiomScope, confidence: u8) -> Idiom {
        Idiom::new(
            IdiomId::new(id).expect("id"),
            scope,
            IdiomConstraint::text("use named exports").expect("constraint"),
            confidence,
            at(0),
            static_source(),
        )
        .expect("idiom")
    }

    #[test]
    fn user_records_apply_to_every_context() {
        assert!(scope_applies(&IdiomScope::User, &IdiomScope::User));
        assert!(scope_applies(&IdiomScope::User, &IdiomScope::Project));
        let package = IdiomScope::Package(BoundedText::new("core", 64).expect("name"));
        assert!(scope_applies(&IdiomScope::User, &package));
    }

    #[test]
    fn non_user_records_require_exact_scope_match() {
        assert!(scope_applies(&IdiomScope::Project, &IdiomScope::Project));
        assert!(!scope_applies(&IdiomScope::Project, &IdiomScope::User));
        let package = IdiomScope::Package(BoundedText::new("core", 64).expect("name"));
        assert!(!scope_applies(&IdiomScope::Project, &package));
        assert!(scope_applies(&package, &package));
    }

    #[test]
    fn static_source_selects_user_and_matching_scope() {
        let package = IdiomScope::Package(BoundedText::new("core", 64).expect("name"));
        let source = StaticRulesSource::new(vec![
            idiom("user", IdiomScope::User, 90),
            idiom("proj", IdiomScope::Project, 80),
            idiom("pkg", package.clone(), 70),
        ]);
        let context = IdiomContext::new(IdiomScope::Project, at(0), 10);
        let set = source.select(&context);
        let ids: Vec<&str> = set
            .idioms()
            .iter()
            .map(|value| value.id().as_str())
            .collect();
        assert_eq!(ids, vec!["user", "proj"]);
    }

    #[test]
    fn delivery_seam_returns_bounded_set_without_prompt_text() {
        let source = StaticRulesSource::new(
            (0..6)
                .map(|index| idiom(&format!("id-{index}"), IdiomScope::User, 50))
                .collect(),
        );
        let set = prepare_session_idioms(&source, IdiomScope::Project, at(0), 3);
        assert_eq!(set.len(), 3);
        assert!(set.is_truncated());
        assert!(
            !set.idioms()
                .iter()
                .any(|value| value.id().as_str() == "prompt")
        );
    }
}
