use super::{image, model, prepared, prepared_catalogue};
use swallowtail_adapter_oh_my_pi::{OhMyPiRunProfileInput, OhMyPiSessionProfileInput};
use swallowtail_core::{ExecutionHostId, ReasoningMode};
use swallowtail_runtime::{
    ConsumerRouteControlId, ConsumerRouteFeatureId, ConsumerRouteLifecycle,
    ConsumerRouteProjectionContribution, ConsumerRouteRowIdentity, SessionOptions,
};
use std::collections::BTreeSet;

const ROUTE: &str = "oh-my-pi.rpc";
const CATALOGUE: &str = "OhMyPiPreparedCatalogue";
const RUN: &str = "OhMyPiPreparedRun";
const SESSION: &str = "OhMyPiPreparedSession";
const ALL: &[&str] = &[CATALOGUE, RUN, SESSION];
const RUN_SESSION: &[&str] = &[RUN, SESSION];

struct LedgerEntry {
    route_id: &'static str,
    operation_shape: &'static str,
    semantic_id: &'static str,
    emitted_by: &'static [&'static str],
    withheld_because: &'static str,
}

const LEDGER: [LedgerEntry; 18] = [
    LedgerEntry { route_id: ROUTE, operation_shape: "model-catalogue", semantic_id: "feature.model-catalogue", emitted_by: &[CATALOGUE], withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "structured-run", semantic_id: "feature.structured-run", emitted_by: &[RUN], withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "interactive-session", semantic_id: "feature.interactive-session", emitted_by: &[SESSION], withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "route-observation", semantic_id: "feature.streaming-events", emitted_by: RUN_SESSION, withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "route-observation", semantic_id: "feature.usage-evidence", emitted_by: RUN_SESSION, withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "route-capability", semantic_id: "feature.reasoning-selection", emitted_by: RUN_SESSION, withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "route-capability", semantic_id: "feature.attachments", emitted_by: RUN_SESSION, withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "route-capability", semantic_id: "feature.question-exchange", emitted_by: RUN_SESSION, withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "route-capability", semantic_id: "feature.cancellation-or-interruption", emitted_by: RUN_SESSION, withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "route-capability", semantic_id: "feature.working-resource", emitted_by: RUN_SESSION, withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "route-capability", semantic_id: "feature.prepared-facade", emitted_by: ALL, withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "route-observation", semantic_id: "feature.activity-observation", emitted_by: RUN_SESSION, withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "structured-run", semantic_id: "control.model-selection", emitted_by: &[RUN], withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "interactive-session", semantic_id: "control.model-selection", emitted_by: &[SESSION], withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "structured-run", semantic_id: "control.reasoning-selection", emitted_by: &[RUN], withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "interactive-session", semantic_id: "control.reasoning-selection", emitted_by: &[SESSION], withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "structured-run", semantic_id: "control.attachments", emitted_by: &[RUN], withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "interactive-session", semantic_id: "control.attachments", emitted_by: &[SESSION], withheld_because: "" },
];

fn run_contribution(source: &str) -> ConsumerRouteProjectionContribution {
    let run = prepared()
        .prepare_run(
            OhMyPiRunProfileInput::new(
                swallowtail_runtime::RequestId::new(format!("{source}.run")).unwrap(),
                model("oh-my-pi.projection.route"),
                swallowtail_runtime::OperationContent::new("projection prompt").unwrap(),
                swallowtail_runtime::WorkingResourceRef::new("oh-my-pi.projection.workspace").unwrap(),
                swallowtail_runtime::Deadline::at(swallowtail_runtime::MonotonicInstant::from_ticks(1_000)),
            )
            .with_attachments([super::image("oh-my-pi.projection.image")])
            .with_reasoning_mode(ReasoningMode::new("low").unwrap()),
        )
        .expect("run");
    run.consumer_route_projection_contribution(
        swallowtail_runtime::ConsumerRouteProjectionSourceId::new(source).unwrap(),
    )
    .expect("run contribution")
}

fn session_contribution(source: &str) -> ConsumerRouteProjectionContribution {
    let session = prepared()
        .prepare_session(
            OhMyPiSessionProfileInput::new(
                swallowtail_runtime::RequestId::new(format!("{source}.session")).unwrap(),
                model("oh-my-pi.projection.route"),
                swallowtail_runtime::WorkingResourceRef::new("oh-my-pi.projection.workspace").unwrap(),
                SessionOptions::default().with_reasoning_mode(ReasoningMode::new("low").unwrap()),
            )
            .with_image_attachments(),
        )
        .expect("session");
    session
        .consumer_route_projection_contribution(
            swallowtail_runtime::ConsumerRouteProjectionSourceId::new(source).unwrap(),
        )
        .expect("session contribution")
}

fn semantic_id(row: &swallowtail_runtime::ConsumerRouteProjectionRow) -> &str {
    match row.identity() {
        ConsumerRouteRowIdentity::Feature(feature) => match feature {
            ConsumerRouteFeatureId::ModelCatalogue => "feature.model-catalogue",
            ConsumerRouteFeatureId::StructuredRun => "feature.structured-run",
            ConsumerRouteFeatureId::InteractiveSession => "feature.interactive-session",
            ConsumerRouteFeatureId::StreamingEvents => "feature.streaming-events",
            ConsumerRouteFeatureId::UsageEvidence => "feature.usage-evidence",
            ConsumerRouteFeatureId::ReasoningSelection => "feature.reasoning-selection",
            ConsumerRouteFeatureId::Attachments => "feature.attachments",
            ConsumerRouteFeatureId::QuestionExchange => "feature.question-exchange",
            ConsumerRouteFeatureId::CancellationOrInterruption => "feature.cancellation-or-interruption",
            ConsumerRouteFeatureId::WorkingResource => "feature.working-resource",
            ConsumerRouteFeatureId::PreparedFacade => "feature.prepared-facade",
            ConsumerRouteFeatureId::ActivityObservation => "feature.activity-observation",
            _ => "off-route-feature",
        },
        ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::ModelSelection) => "control.model-selection",
        ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::ReasoningSelection) => "control.reasoning-selection",
        ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::Namespaced(extension)) => extension.semantic_id(),
        ConsumerRouteRowIdentity::Control(_) => "off-route-control",
    }
}

fn rows(contribution: &ConsumerRouteProjectionContribution) -> impl Iterator<Item = &swallowtail_runtime::ConsumerRouteProjectionRow> {
    contribution.selection_rows().chain(contribution.session_start_rows()).chain(contribution.active_session_rows())
}

#[test]
fn ledger_reconciles_all_eighteen_rows_across_three_facades() {
    let catalogue = prepared_catalogue(ExecutionHostId::new("oh-my-pi.catalogue.host").unwrap(), None);
    let catalogue = catalogue
        .consumer_route_projection_contribution(swallowtail_runtime::ConsumerRouteProjectionSourceId::new("catalogue").unwrap())
        .expect("catalogue contribution");
    let run = run_contribution("run");
    let session = session_contribution("session");
    assert_eq!(
        rows(&catalogue).map(semantic_id).collect::<BTreeSet<_>>(),
        ["feature.model-catalogue", "feature.prepared-facade"]
            .into_iter()
            .collect(),
    );
    assert_eq!(rows(&run).count(), 13);
    assert_eq!(rows(&session).count(), 13);
    let tuples = LEDGER
        .iter()
        .map(|entry| (entry.route_id, entry.operation_shape, entry.semantic_id))
        .collect::<BTreeSet<_>>();
    assert_eq!(tuples.len(), LEDGER.len());
    let expected = LEDGER
        .iter()
        .map(|entry| entry.semantic_id)
        .collect::<BTreeSet<_>>();
    for entry in LEDGER {
        assert_eq!(entry.route_id, ROUTE);
        assert!(!entry.operation_shape.is_empty());
        assert!(entry.semantic_id.starts_with("feature.") || entry.semantic_id.starts_with("control."));
        assert!(!entry.emitted_by.is_empty());
        assert_eq!(entry.emitted_by.is_empty(), !entry.withheld_because.is_empty());
        assert!(entry.withheld_because.is_empty());
    }
    let mut observed = BTreeSet::new();
    for contribution in [&catalogue, &run, &session] {
        for row in rows(contribution) {
            observed.insert(semantic_id(row));
        }
    }
    assert_eq!(observed, expected);
}

#[test]
fn only_the_attachment_turn_row_is_consumer_mediated_per_turn() {
    let contribution = session_contribution("authority");
    let attachment = contribution
        .session_start_rows()
        .find(|row| semantic_id(row) == "control.attachments")
        .expect("attachment control");
    assert_eq!(attachment.lifecycle(), ConsumerRouteLifecycle::PerTurn);
    assert!(attachment.mutation_authority().is_consumer_mediated_per_turn());
    assert!(attachment.state_support().requested());
    assert!(!attachment.state_support().prepared());
    assert!(!attachment.state_support().provider_effective());
    assert!(!attachment.state_support().rejected());
    assert!(rows(&contribution)
        .filter(|row| row.lifecycle() == ConsumerRouteLifecycle::PerTurn)
        .all(|row| semantic_id(row) == "control.attachments"));
}
