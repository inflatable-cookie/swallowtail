//! Compile-checked guidance for the pluggable idioms mechanism (Contract 055).
//!
//! Shows the consumer path: build a static-rules source from portable
//! records, resolve a bounded session-preparation delivery, and merge a
//! registry package into a local store — all without prompt composition,
//! permission authority, or transport.

use swallowtail_idioms::{
    BoundedText, Idiom, IdiomConstraint, IdiomId, IdiomRecorder, IdiomScope, MonotonicInstant,
    Provenance, RegistryNamespace, RegistryPackage, RegistryPackageRef, SignalKind,
    StaticRulesSource, prepare_session_idioms, pull_package,
};

fn at(ticks: u64) -> MonotonicInstant {
    MonotonicInstant::from_ticks(ticks)
}

fn static_rule(id: &str, scope: IdiomScope, confidence: u8) -> Idiom {
    Idiom::new(
        IdiomId::new(id).expect("id"),
        scope,
        IdiomConstraint::text("use named exports").expect("constraint"),
        confidence,
        at(0),
        Provenance::Static(BoundedText::new("team rules", 256).expect("bounded source")),
    )
    .expect("idiom")
}

fn main() {
    let source = StaticRulesSource::new(vec![
        static_rule("named-exports", IdiomScope::User, 90),
        static_rule("strict-mode", IdiomScope::Project, 80),
    ]);

    // Session preparation: bounded delivery into the host's own prompt layer.
    let delivery = prepare_session_idioms(&source, IdiomScope::Project, at(0), 8);
    println!(
        "delivered {} idioms (truncated: {})",
        delivery.len(),
        delivery.is_truncated()
    );

    // Fail-soft signal recording: no sink means no recording, never a failure.
    let recorder = IdiomRecorder::none();
    let signal = swallowtail_idioms::IdiomSignal::new(
        SignalKind::Accept,
        "read_file",
        "session-1",
        IdiomScope::Project,
        1,
        at(1),
    )
    .expect("signal");
    recorder.record(signal);

    // Registry pull: remote learnings merge into the local store.
    let package = RegistryPackage::new(
        RegistryPackageRef::new(RegistryNamespace::new("myorg").expect("namespace"), "cli")
            .expect("reference"),
        vec![static_rule("lowercase-flags", IdiomScope::Project, 85)],
    )
    .expect("package");
    let local = source.records().to_vec();
    let outcome = pull_package(&local, &package, at(0));
    println!("pull summary: {:?}", outcome.summary());
}
