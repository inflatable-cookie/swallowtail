use crate::record::{Idiom, IdiomScope, IdiomSignal};
use crate::time::MonotonicInstant;
use std::cmp::Reverse;
use std::sync::Arc;

/// Scope order used for deterministic selection: least specific first.
fn scope_rank(scope: &IdiomScope) -> u8 {
    match scope {
        IdiomScope::User => 0,
        IdiomScope::Project => 1,
        IdiomScope::Package(_) => 2,
    }
}

/// Context for one session-preparation selection (Contract 055).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdiomContext {
    scope: IdiomScope,
    at: MonotonicInstant,
    maximum: usize,
}

impl IdiomContext {
    /// Creates a selection context with a primary scope, evaluation instant,
    /// and an output bound.
    pub fn new(scope: IdiomScope, at: MonotonicInstant, maximum: usize) -> Self {
        Self { scope, at, maximum }
    }

    /// Returns the primary scope of the selection.
    #[must_use]
    pub const fn scope(&self) -> &IdiomScope {
        &self.scope
    }

    /// Returns the evaluation instant for effective confidence.
    #[must_use]
    pub const fn at(&self) -> MonotonicInstant {
        self.at
    }

    /// Returns the maximum number of idioms the selection may contain.
    #[must_use]
    pub const fn maximum(&self) -> usize {
        self.maximum
    }
}

/// One bounded selection result (Contract 055).
///
/// Idioms are ordered by scope (user, project, then package) and, within a
/// scope, by effective confidence descending with the idiom id as a
/// deterministic tie-break.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdiomSet {
    idioms: Vec<Idiom>,
    truncated: bool,
}

impl IdiomSet {
    /// Creates a selection result with an explicit truncation flag.
    #[must_use]
    pub fn new(idioms: Vec<Idiom>, truncated: bool) -> Self {
        Self { idioms, truncated }
    }

    /// Returns the selected idioms in selection order.
    #[must_use]
    pub fn idioms(&self) -> &[Idiom] {
        &self.idioms
    }

    /// Returns whether candidates were dropped by the output bound.
    #[must_use]
    pub const fn is_truncated(&self) -> bool {
        self.truncated
    }

    /// Returns the number of selected idioms.
    #[must_use]
    pub fn len(&self) -> usize {
        self.idioms.len()
    }

    /// Returns whether the selection is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.idioms.is_empty()
    }
}

impl Default for IdiomSet {
    fn default() -> Self {
        Self::new(Vec::new(), false)
    }
}

/// Orders candidate idioms by scope then effective confidence and bounds the
/// result.
///
/// A zero maximum yields an empty set with `truncated` set when candidates
/// exist. Sources may call this helper to honor the selection contract.
#[must_use]
pub fn select_bounded(candidates: Vec<Idiom>, at: MonotonicInstant, maximum: usize) -> IdiomSet {
    let mut ordered: Vec<Idiom> = candidates;
    ordered.sort_by_key(|idiom| {
        (
            scope_rank(idiom.scope()),
            Reverse(idiom.effective_confidence(at)),
            idiom.id().as_str().to_owned(),
        )
    });
    let truncated = ordered.len() > maximum;
    ordered.truncate(maximum);
    IdiomSet::new(ordered, truncated)
}

/// Pluggable idiom selection backend (Contract 055).
///
/// Implementations return at most `context.maximum()` idioms, ordered by
/// scope then effective confidence. A source never composes prompts, mutates
/// state, or enforces permissions.
pub trait IdiomSource: Send + Sync {
    /// Selects a bounded, ordered set of idioms for one context.
    fn select(&self, context: &IdiomContext) -> IdiomSet;
}

/// Fail-soft idiom signal sink (Contract 055).
///
/// Implementations never panic, block, or propagate failures; recording is
/// best-effort and carries no control over the operation.
pub trait IdiomSink: Send + Sync {
    /// Records one bounded interaction signal.
    fn record(&self, signal: &IdiomSignal);
}

/// Optional fail-soft recorder facade on the `DiagnosticObserver` model.
///
/// No registered sink means no recording and never a failure.
#[derive(Clone)]
pub struct IdiomRecorder {
    sink: Option<Arc<dyn IdiomSink>>,
}

impl IdiomRecorder {
    /// Creates a recorder with no sink: recording is a no-op.
    #[must_use]
    pub const fn none() -> Self {
        Self { sink: None }
    }

    /// Creates a recorder that forwards signals to one sink.
    #[must_use]
    pub fn with(sink: impl IdiomSink + 'static) -> Self {
        Self {
            sink: Some(Arc::new(sink)),
        }
    }

    /// Records one signal, forwarding to the registered sink when present.
    pub fn record(&self, signal: IdiomSignal) {
        if let Some(sink) = &self.sink {
            sink.record(&signal);
        }
    }
}

impl Default for IdiomRecorder {
    fn default() -> Self {
        Self::none()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::{IdiomContext, IdiomRecorder, IdiomSet, IdiomSink, IdiomSource, select_bounded};
    use crate::bounded::BoundedText;
    use crate::record::{
        Idiom, IdiomConstraint, IdiomId, IdiomScope, IdiomSignal, Provenance, SignalKind,
    };
    use crate::time::MonotonicInstant;

    fn at(ticks: u64) -> MonotonicInstant {
        MonotonicInstant::from_ticks(ticks)
    }

    fn static_source() -> Provenance {
        Provenance::Static(BoundedText::new("team rules", 256).expect("bounded source"))
    }

    fn idiom(id: &str, scope: IdiomScope, confidence: u8, at_ticks: u64) -> Idiom {
        Idiom::new(
            IdiomId::new(id).expect("id"),
            scope,
            IdiomConstraint::text("use named exports").expect("constraint"),
            confidence,
            at(at_ticks),
            static_source(),
        )
        .expect("idiom")
    }

    fn signal(sequence: u64) -> IdiomSignal {
        IdiomSignal::new(
            SignalKind::Accept,
            "read_file",
            "session-1",
            IdiomScope::Project,
            sequence,
            at(0),
        )
        .expect("signal")
    }

    #[test]
    fn selection_orders_scope_then_confidence_then_id() {
        let candidates = vec![
            idiom(
                "pkg",
                IdiomScope::Package(BoundedText::new("core", 64).expect("name")),
                90,
                0,
            ),
            idiom("user", IdiomScope::User, 10, 0),
            idiom("proj", IdiomScope::Project, 80, 0),
        ];
        let set = select_bounded(candidates, at(0), 10);
        let ids: Vec<&str> = set.idioms().iter().map(|i| i.id().as_str()).collect();
        assert_eq!(ids, vec!["user", "proj", "pkg"]);
    }

    #[test]
    fn selection_orders_confidence_descending_within_scope() {
        let candidates = vec![
            idiom("b", IdiomScope::Project, 50, 0),
            idiom("a", IdiomScope::Project, 90, 0),
            idiom("c", IdiomScope::Project, 90, 0),
        ];
        let set = select_bounded(candidates, at(0), 10);
        let ids: Vec<&str> = set.idioms().iter().map(|i| i.id().as_str()).collect();
        assert_eq!(ids, vec!["a", "c", "b"]);
    }

    #[test]
    fn selection_is_bounded_and_flags_truncation() {
        let candidates: Vec<Idiom> = (0..10)
            .map(|i| idiom(&format!("id-{i}"), IdiomScope::User, 50, 0))
            .collect();
        let set = select_bounded(candidates, at(0), 3);
        assert_eq!(set.len(), 3);
        assert!(set.is_truncated());
        assert!(!select_bounded(Vec::new(), at(0), 3).is_truncated());
    }

    #[test]
    fn zero_maximum_yields_empty_truncated_set() {
        let candidates = vec![idiom("a", IdiomScope::User, 90, 0)];
        let set = select_bounded(candidates, at(0), 0);
        assert!(set.is_empty());
        assert!(set.is_truncated());
    }

    struct FixtureSource {
        idioms: Vec<Idiom>,
    }

    impl IdiomSource for FixtureSource {
        fn select(&self, context: &IdiomContext) -> IdiomSet {
            select_bounded(self.idioms.clone(), context.at(), context.maximum())
        }
    }

    #[test]
    fn source_honors_context_bound() {
        let source = FixtureSource {
            idioms: (0..7)
                .map(|i| idiom(&format!("id-{i}"), IdiomScope::User, 50, 0))
                .collect(),
        };
        let context = IdiomContext::new(IdiomScope::Project, at(0), 4);
        let set = source.select(&context);
        assert_eq!(set.len(), 4);
        assert!(set.is_truncated());
    }

    struct CountingSink {
        calls: Arc<AtomicUsize>,
    }

    impl IdiomSink for CountingSink {
        fn record(&self, _signal: &IdiomSignal) {
            self.calls.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[test]
    fn registered_sink_receives_signals() {
        let calls = Arc::new(AtomicUsize::new(0));
        let sink = CountingSink {
            calls: Arc::clone(&calls),
        };
        let recorder = IdiomRecorder::with(sink);
        recorder.record(signal(1));
        recorder.record(signal(2));
        assert_eq!(calls.load(Ordering::Relaxed), 2);
    }
}
