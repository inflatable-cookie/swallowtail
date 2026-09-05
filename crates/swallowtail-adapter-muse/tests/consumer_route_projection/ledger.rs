use super::common;
use swallowtail_core::ReasoningMode;
use swallowtail_runtime::{
    ConsumerRouteControlId, ConsumerRouteFeatureId, ConsumerRouteLifecycle,
    ConsumerRouteProjectionContribution, ConsumerRouteRowIdentity,
};
use std::collections::BTreeSet;

const ROUTE: &str = "muse-code.headless";
const PROFILE: &str = "MusePreparedRun";
const EVERY: &[&str] = &[PROFILE];

struct LedgerEntry {
    route_id: &'static str,
    operation_shape: &'static str,
    semantic_id: &'static str,
    emitted_by: &'static [&'static str],
    withheld_because: &'static str,
}

const LEDGER: [LedgerEntry; 10] = [
    LedgerEntry { route_id: ROUTE, operation_shape: "model-catalogue", semantic_id: "feature.model-catalogue", emitted_by: EVERY, withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "structured-run", semantic_id: "feature.structured-run", emitted_by: EVERY, withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "route-observation", semantic_id: "feature.streaming-events", emitted_by: EVERY, withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "route-capability", semantic_id: "feature.reasoning-selection", emitted_by: EVERY, withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "route-capability", semantic_id: "feature.cancellation-or-interruption", emitted_by: EVERY, withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "route-capability", semantic_id: "feature.working-resource", emitted_by: EVERY, withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "route-capability", semantic_id: "feature.prepared-facade", emitted_by: EVERY, withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "route-observation", semantic_id: "feature.activity-observation", emitted_by: EVERY, withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "structured-run", semantic_id: "control.model-selection", emitted_by: EVERY, withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "structured-run", semantic_id: "control.reasoning-selection", emitted_by: EVERY, withheld_because: "" },
];

fn contribution(source: &str) -> ConsumerRouteProjectionContribution {
    let run = common::prepare(common::host_id())
        .prepare_run(common::run_input(common::exact_model("muse.fixture.route"), "medium"))
        .expect("prepared Muse run");
    run.consumer_route_projection_contribution(
        swallowtail_runtime::ConsumerRouteProjectionSourceId::new(source).expect("source"),
    )
    .expect("projection contribution")
}

fn semantic_id(row: &swallowtail_runtime::ConsumerRouteProjectionRow) -> &str {
    match row.identity() {
        ConsumerRouteRowIdentity::Feature(feature) => match feature {
            ConsumerRouteFeatureId::ModelCatalogue => "feature.model-catalogue",
            ConsumerRouteFeatureId::StructuredRun => "feature.structured-run",
            ConsumerRouteFeatureId::StreamingEvents => "feature.streaming-events",
            ConsumerRouteFeatureId::ReasoningSelection => "feature.reasoning-selection",
            ConsumerRouteFeatureId::CancellationOrInterruption => {
                "feature.cancellation-or-interruption"
            }
            ConsumerRouteFeatureId::WorkingResource => "feature.working-resource",
            ConsumerRouteFeatureId::PreparedFacade => "feature.prepared-facade",
            ConsumerRouteFeatureId::ActivityObservation => "feature.activity-observation",
            _ => "off-route-feature",
        },
        ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::ModelSelection) => {
            "control.model-selection"
        }
        ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::ReasoningSelection) => {
            "control.reasoning-selection"
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
fn ledger_emits_all_ten_exact_rows_once() {
    let contribution = contribution("muse-code.ledger");
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
    }
}

#[test]
fn controls_use_prepared_start_authority_and_activity_is_post_open() {
    let contribution = contribution("muse-code.authority");
    let model = contribution
        .selection_rows()
        .find(|row| semantic_id(row) == "control.model-selection")
        .expect("model control");
    assert!(model.mutation_authority().is_prepared_session_start());
    assert!(model.state_support().requested() && model.state_support().prepared());
    let reasoning = contribution
        .session_start_rows()
        .find(|row| semantic_id(row) == "control.reasoning-selection")
        .expect("reasoning control");
    assert_eq!(reasoning.lifecycle(), ConsumerRouteLifecycle::SessionStartOnly);
    assert!(reasoning.mutation_authority().is_prepared_session_start());
    assert_eq!(reasoning.control_value().unwrap().domain().clone(), {
        let value = swallowtail_runtime::ConsumerRouteEnumerableValue::new("medium").unwrap();
        swallowtail_runtime::ConsumerRouteValueDomain::Enumerated(
            swallowtail_runtime::ConsumerRouteEnumeratedValues::new([value]).unwrap(),
        )
    });
    let activity = contribution.active_session_rows().next().expect("activity");
    assert_eq!(activity.lifecycle(), ConsumerRouteLifecycle::PostOpenObservationOnly);
    assert!(activity.state_support().is_descriptor_only());
    let _ = ReasoningMode::new("medium").expect("fixture reasoning is valid");
}
