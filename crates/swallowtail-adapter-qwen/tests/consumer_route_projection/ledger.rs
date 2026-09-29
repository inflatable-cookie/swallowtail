use super::{
    FakeProcessService, PendingTimeService, host_services_for, preparation_input, probe,
};
use futures_executor::block_on;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;
use swallowtail_adapter_qwen::{
    QwenCatalogueProfileInput, QwenModelSelection, QwenRunProfileInput, QwenSessionProfileInput,
    prepare_qwen_catalogue, prepare_qwen_headless,
};
use swallowtail_core::{
    ExecutionHostId, HarnessMode, ModelId, ModelRouteId, ModelRouteRevision, OperationShape,
    ProviderId, ReasoningMode,
};
use swallowtail_runtime::{
    ConsumerRouteControlId, ConsumerRouteFeatureId, ConsumerRouteLifecycle,
    ConsumerRouteProjectionContribution, ConsumerRouteRowIdentity, Deadline, MonotonicInstant,
    OperationContent, RequestId, SessionOptions, WorkingResourceRef,
};
use swallowtail_testkit::{
    ConsumerRouteLedgerClaim, assert_consumer_route_ledger_emitted_by_facade,
};

const ROUTE: &str = "qwen.headless";
const CATALOGUE: &str = "QwenPreparedCatalogue";
const RUN: &str = "QwenPreparedRun";
const SESSION: &str = "QwenPreparedSession";
const ALL: &[&str] = &[CATALOGUE, RUN, SESSION];
const RUN_SESSION: &[&str] = &[RUN, SESSION];

struct LedgerEntry {
    route_id: &'static str,
    operation_shape: &'static str,
    semantic_id: &'static str,
    emitted_by: &'static [&'static str],
    withheld_because: &'static str,
}

const LEDGER: [LedgerEntry; 16] = [
    LedgerEntry { route_id: ROUTE, operation_shape: "model-catalogue", semantic_id: "feature.model-catalogue", emitted_by: &[CATALOGUE, RUN], withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "structured-run", semantic_id: "feature.structured-run", emitted_by: &[RUN], withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "interactive-session", semantic_id: "feature.interactive-session", emitted_by: &[SESSION], withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "route-observation", semantic_id: "feature.streaming-events", emitted_by: RUN_SESSION, withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "route-observation", semantic_id: "feature.usage-evidence", emitted_by: &[SESSION], withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "route-capability", semantic_id: "feature.reasoning-selection", emitted_by: RUN_SESSION, withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "route-capability", semantic_id: "feature.cancellation-or-interruption", emitted_by: RUN_SESSION, withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "route-capability", semantic_id: "feature.working-resource", emitted_by: &[RUN], withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "route-capability", semantic_id: "feature.prepared-facade", emitted_by: ALL, withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "route-observation", semantic_id: "feature.activity-observation", emitted_by: RUN_SESSION, withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "structured-run", semantic_id: "control.model-selection", emitted_by: &[RUN], withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "interactive-session", semantic_id: "control.model-selection", emitted_by: &[SESSION], withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "structured-run", semantic_id: "control.reasoning-selection", emitted_by: &[RUN], withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "interactive-session", semantic_id: "control.reasoning-selection", emitted_by: &[SESSION], withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "structured-run", semantic_id: "control.harness-mode", emitted_by: &[RUN], withheld_because: "" },
    LedgerEntry { route_id: ROUTE, operation_shape: "interactive-session", semantic_id: "control.harness-mode", emitted_by: &[SESSION], withheld_because: "" },
];

fn prepared(version: &'static str) -> swallowtail_adapter_qwen::QwenPreparedIntegration {
    let host_id = ExecutionHostId::new(format!("qwen.projection.{version}.host")).unwrap();
    let version_line = format!("{version}\n");
    let (process, _) = FakeProcessService::completed(&version_line);
    let (services, _) = host_services_for(host_id.clone(), process, Arc::new(PendingTimeService));
    block_on(prepare_qwen_headless(preparation_input(host_id), probe(), services))
        .expect("Qwen prepares")
}

fn model(route: &str) -> QwenModelSelection {
    QwenModelSelection::new(
        ModelRouteId::new(route).unwrap(),
        ModelRouteRevision::new("1").unwrap(),
        ProviderId::new("alibaba-modelstudio").unwrap(),
        ModelId::new("qwen3.8-max").unwrap(),
    )
}

fn run_contribution(source: &str) -> ConsumerRouteProjectionContribution {
    let run = prepared("0.21.15")
        .prepare_run(
            QwenRunProfileInput::new(
                RequestId::new(format!("{source}.run")).unwrap(),
                model("qwen.projection.route"),
                OperationContent::new("projection prompt").unwrap(),
                WorkingResourceRef::new("qwen.projection.workspace").unwrap(),
                Deadline::at(MonotonicInstant::from_ticks(1_000)),
            )
            .with_reasoning_mode(ReasoningMode::new("high").unwrap())
            .with_harness_mode(HarnessMode::Plan),
        )
        .expect("run");
    run.consumer_route_projection_contribution(
        swallowtail_runtime::ConsumerRouteProjectionSourceId::new(source).unwrap(),
    )
    .expect("run contribution")
}

fn session_contribution(source: &str) -> ConsumerRouteProjectionContribution {
    let session = prepared("0.21.15")
        .prepare_session(
            QwenSessionProfileInput::new(
                RequestId::new(format!("{source}.session")).unwrap(),
                model("qwen.projection.route"),
                WorkingResourceRef::new("qwen.projection.workspace").unwrap(),
            )
            .with_reasoning_mode(ReasoningMode::new("high").unwrap())
            .with_harness_mode(HarnessMode::Plan),
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

fn operation_shape(
    row: &swallowtail_runtime::ConsumerRouteProjectionRow,
    contribution: &ConsumerRouteProjectionContribution,
) -> &'static str {
    match row.identity() {
        ConsumerRouteRowIdentity::Feature(feature) => match feature {
            ConsumerRouteFeatureId::ModelCatalogue => "model-catalogue",
            ConsumerRouteFeatureId::StructuredRun => "structured-run",
            ConsumerRouteFeatureId::InteractiveSession => "interactive-session",
            ConsumerRouteFeatureId::StreamingEvents
            | ConsumerRouteFeatureId::UsageEvidence
            | ConsumerRouteFeatureId::ActivityObservation => "route-observation",
            ConsumerRouteFeatureId::ReasoningSelection
            | ConsumerRouteFeatureId::CancellationOrInterruption
            | ConsumerRouteFeatureId::WorkingResource
            | ConsumerRouteFeatureId::PreparedFacade => "route-capability",
            other => panic!("unexpected Qwen feature for shape {other:?}"),
        },
        ConsumerRouteRowIdentity::Control(
            ConsumerRouteControlId::ModelSelection
            | ConsumerRouteControlId::ReasoningSelection
            | ConsumerRouteControlId::Namespaced(_),
        ) => match contribution.applicability().operation_shape() {
            OperationShape::StructuredRun => "structured-run",
            OperationShape::InteractiveSession => "interactive-session",
            other => panic!("unexpected Qwen control operation {other:?}"),
        },
        other => panic!("unexpected Qwen identity for shape {other:?}"),
    }
}

fn row_tuple(
    row: &swallowtail_runtime::ConsumerRouteProjectionRow,
    contribution: &ConsumerRouteProjectionContribution,
) -> (&'static str, &'static str, String) {
    (
        ROUTE,
        operation_shape(row, contribution),
        semantic_id(row).to_owned(),
    )
}

fn observed_tuples(
    contribution: &ConsumerRouteProjectionContribution,
) -> BTreeSet<(&'static str, &'static str, String)> {
    rows(contribution)
        .map(|row| row_tuple(row, contribution))
        .collect()
}

#[test]
fn ledger_reconciles_all_sixteen_rows_across_three_facades() {
    let prepared = prepared("0.21.15");
    let catalogue = prepare_qwen_catalogue(
        &prepared,
        QwenCatalogueProfileInput::new(RequestId::new("catalogue").unwrap()),
    )
    .expect("catalogue");
    let catalogue = catalogue
        .consumer_route_projection_contribution(swallowtail_runtime::ConsumerRouteProjectionSourceId::new("catalogue").unwrap())
        .expect("catalogue contribution");
    let run = run_contribution("run");
    let session = session_contribution("session");
    assert_eq!(rows(&catalogue).count(), 2);
    assert_eq!(rows(&run).count(), 11);
    assert_eq!(rows(&session).count(), 10);
    let tuples = LEDGER
        .iter()
        .map(|entry| (entry.route_id, entry.operation_shape, entry.semantic_id))
        .collect::<BTreeSet<_>>();
    assert_eq!(tuples.len(), LEDGER.len());
    for entry in LEDGER {
        assert_eq!(entry.route_id, ROUTE);
        assert!(entry.semantic_id.starts_with("feature.") || entry.semantic_id.starts_with("control."));
        assert!(!entry.operation_shape.is_empty());
        assert!(!entry.emitted_by.is_empty());
        assert_eq!(entry.emitted_by.is_empty(), !entry.withheld_because.is_empty());
        assert!(entry.withheld_because.is_empty());
    }
    let observed = BTreeMap::from([
        (CATALOGUE, observed_tuples(&catalogue)),
        (RUN, observed_tuples(&run)),
        (SESSION, observed_tuples(&session)),
    ]);
    assert_consumer_route_ledger_emitted_by_facade(
        &observed,
        LEDGER.iter().map(|entry| ConsumerRouteLedgerClaim {
            identity: (
                entry.route_id,
                entry.operation_shape,
                entry.semantic_id.to_owned(),
            ),
            emitted_by: entry.emitted_by,
        }),
    );
}

#[test]
fn optional_controls_are_session_start_only_and_never_provider_effective() {
    for contribution in [run_contribution("run-authority"), session_contribution("session-authority")] {
        let controls = rows(&contribution).filter(|row| {
            matches!(
                row.identity(),
                ConsumerRouteRowIdentity::Control(
                    ConsumerRouteControlId::ReasoningSelection
                        | ConsumerRouteControlId::Namespaced(_)
                )
            )
        });
        for row in controls {
            assert_eq!(row.lifecycle(), ConsumerRouteLifecycle::SessionStartOnly);
            assert!(row.mutation_authority().is_prepared_session_start());
            assert!(row.state_support().requested() && row.state_support().prepared());
            assert!(!row.state_support().provider_effective());
            assert!(!row.state_support().rejected());
        }
    }
}
