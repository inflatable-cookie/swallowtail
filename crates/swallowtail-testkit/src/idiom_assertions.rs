use std::cmp::Reverse;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use swallowtail_core::{ExecutionHostId, SafeDiagnostic};
use swallowtail_idioms::{
    BoundedText, Idiom, IdiomConstraint, IdiomContext, IdiomId, IdiomRecorder, IdiomScope,
    IdiomSet, IdiomSignal, IdiomSink, IdiomSource, MonotonicInstant, Provenance, SignalKind,
    StaticRulesSource, merge, prepare_session_idioms, select_bounded,
};
use swallowtail_runtime::{
    HostServices, IdiomSessionOption, IdiomSourceUnavailable, OperationContent, ZeroIdiomMaximum,
    append_folded_idioms, fold_idioms, fold_idioms_with_bound, resolve_idiom_instructions,
};

fn at(ticks: u64) -> MonotonicInstant {
    MonotonicInstant::from_ticks(ticks)
}

fn static_source() -> Provenance {
    Provenance::Static(BoundedText::new("fixture rules", 256).expect("bounded source"))
}

fn idiom(id: &str, scope: IdiomScope, confidence: u8, at_ticks: u64) -> Idiom {
    idiom_with_constraint(
        id,
        scope,
        IdiomConstraint::text("use named exports").expect("constraint"),
        confidence,
        at_ticks,
    )
}

fn idiom_with_constraint(
    id: &str,
    scope: IdiomScope,
    constraint: IdiomConstraint,
    confidence: u8,
    at_ticks: u64,
) -> Idiom {
    Idiom::new(
        IdiomId::new(id).expect("id"),
        scope,
        constraint,
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
        "fixture-session",
        IdiomScope::Project,
        sequence,
        at(0),
    )
    .expect("signal")
}

/// Orders candidates like the engine so a fixture can pin exact output.
fn expected_order(mut candidates: Vec<Idiom>, at_ticks: u64, maximum: usize) -> Vec<Idiom> {
    candidates.sort_by_key(|value| {
        let rank = match value.scope() {
            IdiomScope::User => 0,
            IdiomScope::Project => 1,
            IdiomScope::Package(_) => 2,
        };
        (
            rank,
            Reverse(value.effective_confidence(at(at_ticks))),
            value.id().as_str().to_owned(),
        )
    });
    candidates.truncate(maximum);
    candidates
}

/// Runs provider-free assertions for the Contract 055 engine surface.
pub fn assert_idiom_engine_contract() {
    assert_selection_ordering();
    assert_selection_bound();
    assert_merge_outcomes();
    assert_source_honors_context_bound();
    assert_recorder_noop_and_forward();
    assert_failing_sink_does_not_interfere();
}

/// Runs assertions for the Contract 056 route opt-in surface.
pub fn assert_idioms_route_opt_in_contract() {
    assert_session_option_bounds();
    assert_fold_rule_determinism_and_bounds();
    assert_fold_append_order();
    assert_host_port_registration_and_noop();
    assert_resolve_fails_closed_without_source();
    assert_resolve_folds_registered_source();
}

fn assert_session_option_bounds() {
    assert_eq!(
        IdiomSessionOption::new(IdiomScope::Project, 0).expect_err("zero maximum"),
        ZeroIdiomMaximum
    );
    assert_eq!(
        IdiomSessionOption::new(IdiomScope::Project, 8)
            .expect("option")
            .maximum(),
        8
    );
}

fn assert_fold_rule_determinism_and_bounds() {
    let set = IdiomSet::new(
        vec![
            idiom_with_constraint(
                "a",
                IdiomScope::Project,
                IdiomConstraint::text("use named exports").expect("constraint"),
                90,
                0,
            ),
            idiom_with_constraint(
                "b",
                IdiomScope::User,
                IdiomConstraint::tool("edit_*").expect("constraint"),
                80,
                0,
            ),
        ],
        false,
    );
    let first = fold_idioms(&set);
    assert_eq!(first, fold_idioms(&set), "fold must be deterministic");
    assert!(first.contains("[project static] use named exports"));
    assert!(first.contains("[user static] tool: edit_*"));

    let long_set = IdiomSet::new(
        vec![
            idiom_with_constraint(
                "a",
                IdiomScope::Project,
                IdiomConstraint::text("x".repeat(100)).expect("constraint"),
                90,
                0,
            ),
            idiom_with_constraint(
                "b",
                IdiomScope::Project,
                IdiomConstraint::text("y".repeat(100)).expect("constraint"),
                80,
                0,
            ),
        ],
        false,
    );
    let bounded = fold_idioms_with_bound(&long_set, 64);
    assert!(
        bounded.ends_with("… [idioms truncated]"),
        "overflow must carry an explicit marker"
    );
    assert!(bounded.len() < 200, "fold output must stay near the bound");
}

fn assert_resolve_fails_closed_without_source() {
    let host_id = ExecutionHostId::new("fixture.idioms.host").expect("host id is valid");
    let services = HostServices::new(host_id);
    let options = swallowtail_runtime::SessionOptions::default()
        .with_idioms(IdiomSessionOption::new(IdiomScope::Project, 8).expect("option"));
    assert_eq!(
        resolve_idiom_instructions(&services, &options)
            .expect_err("missing source must fail closed"),
        IdiomSourceUnavailable
    );
}

fn assert_resolve_folds_registered_source() {
    let host_id = ExecutionHostId::new("fixture.idioms.host").expect("host id is valid");
    let source = StaticRulesSource::new(vec![idiom("named-exports", IdiomScope::Project, 90, 0)]);
    let services = HostServices::new(host_id).with_idiom_source(Arc::new(source));
    let instructions = OperationContent::new("consumer guidance".to_owned()).expect("content");
    let options = swallowtail_runtime::SessionOptions::default()
        .with_developer_instructions(instructions)
        .with_idioms(IdiomSessionOption::new(IdiomScope::Project, 8).expect("option"));
    let resolved = resolve_idiom_instructions(&services, &options)
        .expect("registered source resolves")
        .expect("combined instructions");
    assert!(resolved.as_str().starts_with("consumer guidance"));
    assert!(
        resolved
            .as_str()
            .contains("[project static] use named exports")
    );

    let plain = swallowtail_runtime::SessionOptions::default();
    assert!(
        resolve_idiom_instructions(&services, &plain)
            .expect("no opt-in resolves")
            .is_none(),
        "no option means no idioms work"
    );
}

fn assert_fold_append_order() {
    let instructions = OperationContent::new("consumer guidance".to_owned()).expect("content");
    let set = IdiomSet::new(
        vec![idiom_with_constraint(
            "a",
            IdiomScope::Project,
            IdiomConstraint::text("use named exports").expect("constraint"),
            90,
            0,
        )],
        false,
    );
    let combined =
        append_folded_idioms(Some(instructions), fold_idioms(&set)).expect("combined content");
    assert!(
        combined.as_str().starts_with("consumer guidance"),
        "consumer instructions must stay first"
    );
    assert!(combined.as_str().contains("\n\n[idioms]\n"));

    assert!(
        append_folded_idioms(None, String::new()).is_none(),
        "empty fold with no instructions must produce nothing"
    );
}

fn assert_host_port_registration_and_noop() {
    let host_id = ExecutionHostId::new("fixture.idioms.host").expect("host id is valid");
    let services = HostServices::new(host_id.clone());
    assert!(services.idiom_source().is_none(), "no source by default");
    assert!(
        services.idiom_recorder().is_none(),
        "no recorder by default"
    );

    let calls = Arc::new(AtomicUsize::new(0));
    let sink = CountingSink {
        calls: Arc::clone(&calls),
    };
    let services = HostServices::new(host_id.clone()).with_idiom_recorder(Arc::new(sink));
    services.record_idiom_signal(signal(1));
    assert_eq!(
        calls.load(Ordering::Relaxed),
        1,
        "registered recorder receives signals"
    );

    let quiet = HostServices::new(host_id);
    quiet.record_idiom_signal(signal(2));
    assert_eq!(
        calls.load(Ordering::Relaxed),
        1,
        "missing recorder is a no-op"
    );
}

/// Runs consumer-style assertions for static-rules session delivery
/// (Contract 055).
pub fn assert_idiom_static_rules_delivery_contract() {
    let package = IdiomScope::Package(BoundedText::new("core", 64).expect("name"));
    let source = StaticRulesSource::new(vec![
        idiom("user-rule", IdiomScope::User, 90, 0),
        idiom("project-rule", IdiomScope::Project, 80, 0),
        idiom("package-rule", package, 70, 0),
    ]);
    let set = prepare_session_idioms(&source, IdiomScope::Project, at(0), 10);
    let ids: Vec<&str> = set
        .idioms()
        .iter()
        .map(|value| value.id().as_str())
        .collect();
    assert_eq!(
        ids,
        vec!["user-rule", "project-rule"],
        "delivery must include user and matching-scope rules in scope order"
    );
    assert!(!set.is_truncated());

    let bounded = prepare_session_idioms(&source, IdiomScope::Project, at(0), 1);
    assert_eq!(bounded.len(), 1);
    assert!(
        bounded.is_truncated(),
        "delivery must honor the session bound"
    );

    let headless = prepare_session_idioms(&source, IdiomScope::Project, at(0), 0);
    assert!(headless.is_empty());
    assert!(!source.records().is_empty(), "records stay unmutated");
}

fn assert_selection_ordering() {
    let candidates = vec![
        idiom(
            "pkg",
            IdiomScope::Package(BoundedText::new("core", 64).expect("name")),
            90,
            0,
        ),
        idiom("user", IdiomScope::User, 10, 0),
        idiom("proj", IdiomScope::Project, 80, 0),
        idiom("tie-b", IdiomScope::Project, 80, 0),
    ];
    let set = select_bounded(candidates.clone(), at(0), 10);
    let expected = expected_order(candidates, 0, 10);
    let got: Vec<&str> = set
        .idioms()
        .iter()
        .map(|value| value.id().as_str())
        .collect();
    let want: Vec<&str> = expected.iter().map(|value| value.id().as_str()).collect();
    assert_eq!(
        got, want,
        "selection must order scope then confidence then id"
    );
}

fn assert_selection_bound() {
    let candidates: Vec<Idiom> = (0..9)
        .map(|index| idiom(&format!("id-{index}"), IdiomScope::User, 50, 0))
        .collect();
    let set = select_bounded(candidates.clone(), at(0), 4);
    assert_eq!(set.len(), 4, "selection must honor the output bound");
    assert!(set.is_truncated(), "overflow must be flagged");
    let unbounded = select_bounded(candidates, at(0), 9);
    assert_eq!(unbounded.len(), 9);
    assert!(!unbounded.is_truncated());
}

fn assert_merge_outcomes() {
    let existing = idiom("a", IdiomScope::Project, 90, 0);
    let (_, outcome) = merge(
        Some(&existing),
        idiom("a", IdiomScope::Project, 95, 0),
        at(0),
    );
    assert_eq!(outcome, swallowtail_idioms::MergeOutcome::Raised);
    let (_, outcome) = merge(
        Some(&existing),
        idiom("a", IdiomScope::Project, 10, 0),
        at(0),
    );
    assert_eq!(outcome, swallowtail_idioms::MergeOutcome::Lowered);
    let (_, outcome) = merge(
        Some(&existing),
        idiom("a", IdiomScope::Project, 90, 0),
        at(0),
    );
    assert_eq!(outcome, swallowtail_idioms::MergeOutcome::Unchanged);
    let (_, outcome) = merge(None, idiom("b", IdiomScope::Project, 50, 0), at(0));
    assert_eq!(outcome, swallowtail_idioms::MergeOutcome::New);
}

fn assert_source_honors_context_bound() {
    let source = FixtureSource {
        idioms: (0..7)
            .map(|index| idiom(&format!("id-{index}"), IdiomScope::User, 50, 0))
            .collect(),
    };
    let context = IdiomContext::new(IdiomScope::Project, at(0), 3);
    let set = source.select(&context);
    assert_eq!(set.len(), 3, "a source must honor the context bound");
    assert!(set.is_truncated(), "overflow must be flagged");
}

struct FixtureSource {
    idioms: Vec<Idiom>,
}

impl IdiomSource for FixtureSource {
    fn select(&self, context: &IdiomContext) -> IdiomSet {
        select_bounded(self.idioms.clone(), context.at(), context.maximum())
    }
}

fn assert_recorder_noop_and_forward() {
    let recorder = IdiomRecorder::none();
    recorder.record(signal(1));

    let calls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let sink = CountingSink {
        calls: Arc::clone(&calls),
    };
    let recorder = IdiomRecorder::with(sink);
    recorder.record(signal(1));
    recorder.record(signal(2));
    assert_eq!(calls.load(Ordering::Relaxed), 2);
}

fn assert_failing_sink_does_not_interfere() {
    let failing = FailingSink {
        failed: Arc::new(AtomicBool::new(false)),
    };
    let recorder = IdiomRecorder::with(failing);
    recorder.record(signal(1));
    let set: IdiomSet = select_bounded(vec![idiom("a", IdiomScope::User, 90, 0)], at(0), 5);
    assert_eq!(set.len(), 1, "a failing sink must not affect selection");
    let diagnostic = SafeDiagnostic::new("fixture.ok", "Fixture continued");
    assert_eq!(diagnostic.code(), "fixture.ok");
}

struct CountingSink {
    calls: Arc<std::sync::atomic::AtomicUsize>,
}

impl IdiomSink for CountingSink {
    fn record(&self, _signal: &IdiomSignal) {
        self.calls.fetch_add(1, Ordering::Relaxed);
    }
}

struct FailingSink {
    failed: Arc<AtomicBool>,
}

impl IdiomSink for FailingSink {
    fn record(&self, _signal: &IdiomSignal) {
        self.failed.store(true, Ordering::Relaxed);
    }
}
