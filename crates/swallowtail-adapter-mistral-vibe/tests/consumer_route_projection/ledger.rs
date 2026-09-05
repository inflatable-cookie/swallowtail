use super::{prepare, run_input};
use swallowtail_adapter_mistral_vibe::MistralVibeMaxTurns;
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{
    ConsumerRouteControlId, ConsumerRouteFeatureId, ConsumerRouteLifecycle,
    ConsumerRouteProjectionContribution, ConsumerRouteRowIdentity,
};
use std::collections::BTreeSet;

const ROUTE: &str = "mistral-vibe.headless";
const PROFILE: &str = "MistralVibeHeadlessPreparedRun";
const EVERY: &[&str] = &[PROFILE];

struct LedgerEntry {
    route_id: &'static str,
    operation_shape: &'static str,
    semantic_id: &'static str,
    emitted_by: &'static [&'static str],
    withheld_because: &'static str,
}

const LEDGER: [LedgerEntry; 8] = [
    LedgerEntry { route_id: ROUTE, operation_shape: "model-catalogue", semantic_id: "feature.model-catalogue", emitted_by: EVERY, withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "structured-run", semantic_id: "feature.structured-run", emitted_by: EVERY, withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "route-observation", semantic_id: "feature.streaming-events", emitted_by: EVERY, withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "route-capability", semantic_id: "feature.cancellation-or-interruption", emitted_by: EVERY, withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "route-capability", semantic_id: "feature.working-resource", emitted_by: EVERY, withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "route-capability", semantic_id: "feature.prepared-facade", emitted_by: EVERY, withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "route-observation", semantic_id: "feature.activity-observation", emitted_by: EVERY, withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "structured-run", semantic_id: "control.maximum-agentic-turns", emitted_by: EVERY, withheld_because: "" },
];

fn contribution(source: &str) -> ConsumerRouteProjectionContribution {
    let host = ExecutionHostId::new(format!("{source}.host")).expect("host");
    prepare(host)
        .prepare_run(
            run_input(source).with_max_turns(
                MistralVibeMaxTurns::try_new(4).expect("fixture bound is admitted"),
            ),
        )
        .expect("prepared contribution")
        .consumer_route_projection_contribution(
            swallowtail_runtime::ConsumerRouteProjectionSourceId::new(source)
                .expect("source"),
        )
        .expect("projection contribution")
}

fn semantic_id(row: &swallowtail_runtime::ConsumerRouteProjectionRow) -> &str {
    match row.identity() {
        ConsumerRouteRowIdentity::Feature(feature) => match feature {
            ConsumerRouteFeatureId::ModelCatalogue => "feature.model-catalogue",
            ConsumerRouteFeatureId::StructuredRun => "feature.structured-run",
            ConsumerRouteFeatureId::StreamingEvents => "feature.streaming-events",
            ConsumerRouteFeatureId::CancellationOrInterruption => {
                "feature.cancellation-or-interruption"
            }
            ConsumerRouteFeatureId::WorkingResource => "feature.working-resource",
            ConsumerRouteFeatureId::PreparedFacade => "feature.prepared-facade",
            ConsumerRouteFeatureId::ActivityObservation => "feature.activity-observation",
            _ => "off-route-feature",
        },
        ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::Namespaced(extension)) => {
            extension.semantic_id()
        }
        ConsumerRouteRowIdentity::Control(_) => "off-route-control",
    }
}

fn rows(contribution: &ConsumerRouteProjectionContribution) -> impl Iterator<Item = &swallowtail_runtime::ConsumerRouteProjectionRow> {
    contribution
        .selection_rows()
        .chain(contribution.session_start_rows())
        .chain(contribution.active_session_rows())
}

#[test]
fn ledger_emits_all_eight_exact_rows_once() {
    let contribution = contribution("mistral-vibe.ledger");
    let observed = rows(&contribution).map(semantic_id).collect::<Vec<_>>();
    let expected = LEDGER.iter().map(|entry| entry.semantic_id).collect::<Vec<_>>();
    let tuples = LEDGER
        .iter()
        .map(|entry| (entry.route_id, entry.operation_shape, entry.semantic_id))
        .collect::<BTreeSet<_>>();
    assert_eq!(observed.len(), LEDGER.len());
    assert_eq!(tuples.len(), LEDGER.len());
    assert_eq!(
        observed.iter().copied().collect::<BTreeSet<_>>(),
        expected.iter().copied().collect::<BTreeSet<_>>(),
    );
    for entry in LEDGER {
        assert_eq!(entry.route_id, ROUTE);
        assert!(entry.semantic_id.starts_with("feature.") || entry.semantic_id.starts_with("control."));
        assert_eq!(entry.emitted_by, EVERY);
        assert_eq!(entry.emitted_by.is_empty(), !entry.withheld_because.is_empty());
        assert!(entry.withheld_because.is_empty());
        assert!(expected.contains(&entry.semantic_id));
    }
}

#[test]
fn lifecycle_and_source_truth_stay_separate() {
    let contribution = contribution("mistral-vibe.lifecycle");
    let activity = contribution
        .active_session_rows()
        .next()
        .expect("activity row");
    assert_eq!(activity.lifecycle(), ConsumerRouteLifecycle::PostOpenObservationOnly);
    assert!(activity.state_support().is_descriptor_only());
    let turns = contribution
        .session_start_rows()
        .next()
        .expect("turn control");
    assert_eq!(semantic_id(turns), "control.maximum-agentic-turns");
    assert_eq!(turns.lifecycle(), ConsumerRouteLifecycle::SessionStartOnly);
    assert!(turns.state_support().requested() && turns.state_support().prepared());
    assert!(turns.mutation_authority().is_prepared_session_start());
    assert!(rows(&contribution).all(|row| !row.state_support().provider_effective()));
}
