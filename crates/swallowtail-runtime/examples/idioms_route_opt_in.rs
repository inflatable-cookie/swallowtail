//! Compile-checked guidance for the route-path idioms opt-in (Contract 056).
//!
//! Shows the consumer flow: one host registration of an idiom source, one
//! session-option field, and the runtime folding the selected idioms after
//! consumer-supplied developer instructions.

use std::sync::Arc;

use swallowtail_core::ExecutionHostId;
use swallowtail_idioms::{
    BoundedText, Idiom, IdiomConstraint, IdiomId, IdiomScope, MonotonicInstant, Provenance,
    StaticRulesSource,
};
use swallowtail_runtime::{
    HostServices, IdiomSessionOption, OperationContent, SessionOptions, resolve_idiom_instructions,
};

fn main() {
    let rule = Idiom::new(
        IdiomId::new("named-exports").expect("id"),
        IdiomScope::Project,
        IdiomConstraint::text("use named exports").expect("constraint"),
        90,
        MonotonicInstant::from_ticks(0),
        Provenance::Static(BoundedText::new("team rules", 256).expect("bounded source")),
    )
    .expect("idiom");
    let source = StaticRulesSource::new(vec![rule]);

    let services = HostServices::new(ExecutionHostId::new("example.host").expect("host id"))
        .with_idiom_source(Arc::new(source));

    let options = SessionOptions::default()
        .with_developer_instructions(OperationContent::new("consumer guidance").expect("content"))
        .with_idioms(IdiomSessionOption::new(IdiomScope::Project, 8).expect("option"));

    let resolved = resolve_idiom_instructions(&services, &options)
        .expect("registered source resolves")
        .expect("combined instructions");
    println!("{}", resolved.as_str());
}
