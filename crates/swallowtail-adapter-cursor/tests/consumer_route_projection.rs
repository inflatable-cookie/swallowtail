//! Contract 061 disposition proof for Cursor agent routes (acp, catalogue, headless).

mod consumer_route_projection {
    pub mod fixtures;
    pub mod ledger;
}

use consumer_route_projection::fixtures;
use consumer_route_projection::ledger::*;
use std::collections::BTreeSet;
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteControlId, ConsumerRouteEvidenceStrength,
    ConsumerRouteFeatureId, ConsumerRouteLifecycle, ConsumerRouteMutationAuthority,
    ConsumerRouteOmissionSemantics, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionRow, ConsumerRouteProjectionSourceId, ConsumerRouteProjectionSourceKind,
    ConsumerRouteRowIdentity, ConsumerRouteSourceClass, ConsumerRouteStateSupport,
    ConsumerRouteValueKind,
};

fn source(id: &str) -> ConsumerRouteProjectionSourceId {
    ConsumerRouteProjectionSourceId::new(id).expect("valid source id")
}

fn row_semantic_id(row: &ConsumerRouteProjectionRow) -> String {
    match row.identity() {
        ConsumerRouteRowIdentity::Feature(f) => match f {
            ConsumerRouteFeatureId::ModelCatalogue => "feature.model-catalogue",
            ConsumerRouteFeatureId::StructuredRun => "feature.structured-run",
            ConsumerRouteFeatureId::InteractiveSession => "feature.interactive-session",
            ConsumerRouteFeatureId::StreamingEvents => "feature.streaming-events",
            ConsumerRouteFeatureId::UsageEvidence => "feature.usage-evidence",
            ConsumerRouteFeatureId::ReasoningSelection => "feature.reasoning-selection",
            ConsumerRouteFeatureId::CancellationOrInterruption => {
                "feature.cancellation-or-interruption"
            }
            ConsumerRouteFeatureId::WorkingResource => "feature.working-resource",
            ConsumerRouteFeatureId::PersistentSessionPosture => {
                "feature.persistent-session-posture"
            }
            ConsumerRouteFeatureId::PreparedFacade => "feature.prepared-facade",
            ConsumerRouteFeatureId::ActivityObservation => "feature.activity-observation",
            other => panic!("unexpected feature {other:?}"),
        }
        .to_string(),
        ConsumerRouteRowIdentity::Control(c) => match c {
            ConsumerRouteControlId::ModelSelection => "control.model-selection".to_string(),
            ConsumerRouteControlId::Namespaced(ext) => ext.semantic_id().to_string(),
            other => panic!("unexpected control {other:?}"),
        },
    }
}

fn row_operation_shape(row: &ConsumerRouteProjectionRow) -> &'static str {
    match row.identity() {
        ConsumerRouteRowIdentity::Feature(f) => match f {
            ConsumerRouteFeatureId::ModelCatalogue => "model-catalogue",
            ConsumerRouteFeatureId::StructuredRun => "structured-run",
            ConsumerRouteFeatureId::InteractiveSession => "interactive-session",
            ConsumerRouteFeatureId::StreamingEvents
            | ConsumerRouteFeatureId::UsageEvidence
            | ConsumerRouteFeatureId::ActivityObservation => "route-observation",
            ConsumerRouteFeatureId::PersistentSessionPosture => "session-lifecycle",
            ConsumerRouteFeatureId::PreparedFacade
            | ConsumerRouteFeatureId::CancellationOrInterruption
            | ConsumerRouteFeatureId::WorkingResource
            | ConsumerRouteFeatureId::ReasoningSelection => "route-capability",
            other => panic!("unexpected feature for shape: {other:?}"),
        },
        ConsumerRouteRowIdentity::Control(_) => "structured-run",
    }
}

fn row_tuple(
    row: &ConsumerRouteProjectionRow,
    route_id: &'static str,
) -> (&'static str, &'static str, String) {
    (route_id, row_operation_shape(row), row_semantic_id(row))
}

fn contribution_tuples(
    contribution: &ConsumerRouteProjectionContribution,
    route_id: &'static str,
) -> BTreeSet<(&'static str, &'static str, String)> {
    contribution
        .selection_rows()
        .chain(contribution.session_start_rows())
        .chain(contribution.active_session_rows())
        .map(|row| row_tuple(row, route_id))
        .collect()
}

fn claimed_tuples(
    tranche: &[LedgerEntry],
    profile: &str,
) -> BTreeSet<(&'static str, &'static str, String)> {
    tranche
        .iter()
        .filter(|entry| entry.emitted_by.contains(&profile))
        .map(|entry| {
            (
                entry.route_id,
                entry.operation_shape,
                entry.semantic_id.to_string(),
            )
        })
        .collect()
}

fn contribution_semantic_ids(
    contribution: &ConsumerRouteProjectionContribution,
) -> BTreeSet<String> {
    contribution
        .selection_rows()
        .chain(contribution.session_start_rows())
        .chain(contribution.active_session_rows())
        .map(row_semantic_id)
        .collect()
}

#[test]
fn acp_tranche_dispositions_exactly_thirteen_rows() {
    assert_eq!(CURSOR_ACP_TRANCHE.len(), 13);
    let mut ledger_tuples = BTreeSet::new();
    let mut semantics = BTreeSet::new();
    let mut emitted_count = 0;
    let mut withheld_count = 0;

    for entry in &CURSOR_ACP_TRANCHE {
        assert_eq!(entry.route_id, CURSOR_ACP_ROUTE);
        assert!(!entry.operation_shape.is_empty());
        assert!(ledger_tuples.insert((entry.route_id, entry.operation_shape, entry.semantic_id)));
        assert!(semantics.insert(entry.semantic_id));
        if entry.emitted_by.is_empty() {
            withheld_count += 1;
            assert!(!entry.withheld_because.is_empty());
        } else {
            emitted_count += 1;
            assert!(entry.withheld_because.is_empty());
        }
    }

    let census_tuples: BTreeSet<RowTuple> = CURSOR_ACP_CENSUS_TUPLES.into_iter().collect();
    assert_eq!(
        ledger_tuples, census_tuples,
        "cursor-agent.acp ledger tuples must match census tuples bidirectionally"
    );
    assert_eq!(emitted_count, 7);
    assert_eq!(withheld_count, 6);
}

#[test]
fn catalogue_tranche_dispositions_exactly_thirteen_rows() {
    assert_eq!(CURSOR_CATALOGUE_TRANCHE.len(), 13);
    let mut ledger_tuples = BTreeSet::new();
    let mut semantics = BTreeSet::new();
    let mut emitted_count = 0;
    let mut withheld_count = 0;

    for entry in &CURSOR_CATALOGUE_TRANCHE {
        assert_eq!(entry.route_id, CURSOR_CATALOGUE_ROUTE);
        assert!(!entry.operation_shape.is_empty());
        assert!(ledger_tuples.insert((entry.route_id, entry.operation_shape, entry.semantic_id)));
        assert!(semantics.insert(entry.semantic_id));
        if entry.emitted_by.is_empty() {
            withheld_count += 1;
            assert!(!entry.withheld_because.is_empty());
        } else {
            emitted_count += 1;
            assert!(entry.withheld_because.is_empty());
        }
    }

    let census_tuples: BTreeSet<RowTuple> = CURSOR_CATALOGUE_CENSUS_TUPLES.into_iter().collect();
    assert_eq!(
        ledger_tuples, census_tuples,
        "cursor-agent.catalogue ledger tuples must match census tuples bidirectionally"
    );
    assert_eq!(emitted_count, 2);
    assert_eq!(withheld_count, 11);
}

#[test]
fn headless_tranche_dispositions_exactly_seventeen_rows() {
    assert_eq!(CURSOR_HEADLESS_TRANCHE.len(), 17);
    let mut ledger_tuples = BTreeSet::new();
    let mut semantics = BTreeSet::new();
    let mut emitted_count = 0;
    let mut withheld_count = 0;

    for entry in &CURSOR_HEADLESS_TRANCHE {
        assert_eq!(entry.route_id, CURSOR_HEADLESS_ROUTE);
        assert!(!entry.operation_shape.is_empty());
        assert!(ledger_tuples.insert((entry.route_id, entry.operation_shape, entry.semantic_id)));
        assert!(semantics.insert(entry.semantic_id));
        if entry.emitted_by.is_empty() {
            withheld_count += 1;
            assert!(!entry.withheld_because.is_empty());
        } else {
            emitted_count += 1;
            assert!(entry.withheld_because.is_empty());
        }
    }

    let census_tuples: BTreeSet<RowTuple> = CURSOR_HEADLESS_CENSUS_TUPLES.into_iter().collect();
    assert_eq!(
        ledger_tuples, census_tuples,
        "cursor-agent.headless ledger tuples must match census tuples bidirectionally"
    );
    assert_eq!(emitted_count, 14);
    assert_eq!(withheld_count, 3);
}

#[test]
fn package_dispositions_forty_three_rows_total() {
    let acp_len = CURSOR_ACP_TRANCHE.len();
    let catalogue_len = CURSOR_CATALOGUE_TRANCHE.len();
    let headless_len = CURSOR_HEADLESS_TRANCHE.len();
    assert_eq!(acp_len + catalogue_len + headless_len, 43);

    let acp_emitted = CURSOR_ACP_TRANCHE
        .iter()
        .filter(|e| !e.emitted_by.is_empty())
        .count();
    let catalogue_emitted = CURSOR_CATALOGUE_TRANCHE
        .iter()
        .filter(|e| !e.emitted_by.is_empty())
        .count();
    let headless_emitted = CURSOR_HEADLESS_TRANCHE
        .iter()
        .filter(|e| !e.emitted_by.is_empty())
        .count();
    assert_eq!(acp_emitted + catalogue_emitted + headless_emitted, 23);

    let acp_withheld = CURSOR_ACP_TRANCHE
        .iter()
        .filter(|e| e.emitted_by.is_empty())
        .count();
    let catalogue_withheld = CURSOR_CATALOGUE_TRANCHE
        .iter()
        .filter(|e| e.emitted_by.is_empty())
        .count();
    let headless_withheld = CURSOR_HEADLESS_TRANCHE
        .iter()
        .filter(|e| e.emitted_by.is_empty())
        .count();
    assert_eq!(acp_withheld + catalogue_withheld + headless_withheld, 20);
}

#[test]
fn prepared_acp_emits_exact_seven_rows() {
    let acp = fixtures::acp();
    let contribution = acp
        .consumer_route_projection_contribution(source("cursor.acp.source"))
        .expect("acp contributes");

    assert_eq!(contribution.sources().len(), 1);
    let source_identity = contribution.sources().next().unwrap();
    assert_eq!(
        source_identity.kind(),
        ConsumerRouteProjectionSourceKind::AdapterContribution
    );

    let emitted = contribution_semantic_ids(&contribution);
    assert_eq!(emitted.len(), 7);
    assert!(emitted.contains("feature.interactive-session"));
    assert!(emitted.contains("feature.streaming-events"));
    assert!(emitted.contains("feature.cancellation-or-interruption"));
    assert!(emitted.contains("feature.working-resource"));
    assert!(emitted.contains("feature.persistent-session-posture"));
    assert!(emitted.contains("feature.prepared-facade"));
    assert!(emitted.contains("feature.activity-observation"));

    let observed_tuples = contribution_tuples(&contribution, CURSOR_ACP_ROUTE);
    let expected = claimed_tuples(&CURSOR_ACP_TRANCHE, PROFILE_ACP);
    assert_eq!(
        observed_tuples, expected,
        "acp observed emitted tuples must match claimed ledger tuples bidirectionally"
    );

    for entry in &CURSOR_ACP_TRANCHE {
        if entry.emitted_by.is_empty() {
            assert!(
                !observed_tuples.contains(&(
                    entry.route_id,
                    entry.operation_shape,
                    entry.semantic_id.to_string()
                )),
                "withheld ACP row {:?} must not be emitted",
                entry.semantic_id
            );
        }
    }

    assert_eq!(contribution.selection_rows().len(), 6);
    assert_eq!(contribution.session_start_rows().len(), 0);
    assert_eq!(contribution.active_session_rows().len(), 1);

    let active_row = contribution.active_session_rows().next().unwrap();
    assert_eq!(row_semantic_id(active_row), "feature.activity-observation");
    assert_eq!(
        active_row.lifecycle(),
        ConsumerRouteLifecycle::PostOpenObservationOnly
    );
    assert_eq!(
        active_row.actor_posture(),
        ConsumerRouteActorPosture::ObservationOnly
    );
    assert_eq!(
        active_row.state_support(),
        ConsumerRouteStateSupport::descriptor_only()
    );
}

#[test]
fn prepared_catalogue_emits_exact_two_rows() {
    let catalogue = fixtures::catalogue();
    let contribution = catalogue
        .consumer_route_projection_contribution(source("cursor.catalogue.source"))
        .expect("catalogue contributes");

    assert_eq!(contribution.sources().len(), 1);
    let source_identity = contribution.sources().next().unwrap();
    assert_eq!(
        source_identity.kind(),
        ConsumerRouteProjectionSourceKind::AdapterContribution
    );

    let emitted = contribution_semantic_ids(&contribution);
    assert_eq!(emitted.len(), 2);
    assert!(emitted.contains("feature.model-catalogue"));
    assert!(emitted.contains("feature.prepared-facade"));

    let observed_tuples = contribution_tuples(&contribution, CURSOR_CATALOGUE_ROUTE);
    let expected = claimed_tuples(&CURSOR_CATALOGUE_TRANCHE, PROFILE_CATALOGUE);
    assert_eq!(
        observed_tuples, expected,
        "catalogue observed emitted tuples must match claimed ledger tuples bidirectionally"
    );

    for entry in &CURSOR_CATALOGUE_TRANCHE {
        if entry.emitted_by.is_empty() {
            assert!(
                !observed_tuples.contains(&(
                    entry.route_id,
                    entry.operation_shape,
                    entry.semantic_id.to_string()
                )),
                "withheld catalogue row {:?} must not be emitted",
                entry.semantic_id
            );
        }
    }

    assert_eq!(contribution.selection_rows().len(), 2);
    assert_eq!(contribution.session_start_rows().len(), 0);
    assert_eq!(contribution.active_session_rows().len(), 0);

    for row in contribution.selection_rows() {
        assert_eq!(
            row.actor_posture(),
            ConsumerRouteActorPosture::Informational
        );
        assert_eq!(
            row.evidence_strength(),
            ConsumerRouteEvidenceStrength::PreparedOperation
        );
    }
}

#[test]
fn prepared_headless_maximal_emits_fourteen_rows() {
    let headless = fixtures::headless_maximal();
    let contribution = headless
        .consumer_route_projection_contribution(source("cursor.headless.source"))
        .expect("headless maximal contributes");

    let emitted = contribution_semantic_ids(&contribution);
    assert_eq!(emitted.len(), 14);

    let observed_tuples = contribution_tuples(&contribution, CURSOR_HEADLESS_ROUTE);
    let expected = claimed_tuples(&CURSOR_HEADLESS_TRANCHE, PROFILE_HEADLESS_MAXIMAL);
    assert_eq!(
        observed_tuples, expected,
        "headless maximal observed emitted tuples must match claimed ledger tuples bidirectionally"
    );

    assert_eq!(contribution.selection_rows().len(), 9);
    assert_eq!(contribution.session_start_rows().len(), 4);
    assert_eq!(contribution.active_session_rows().len(), 1);

    let active_row = contribution.active_session_rows().next().unwrap();
    assert_eq!(row_semantic_id(active_row), "feature.activity-observation");
    assert_eq!(
        active_row.lifecycle(),
        ConsumerRouteLifecycle::PostOpenObservationOnly
    );
    assert_eq!(
        active_row.actor_posture(),
        ConsumerRouteActorPosture::ObservationOnly
    );
    assert_eq!(
        active_row.state_support(),
        ConsumerRouteStateSupport::descriptor_only()
    );

    let model_row = contribution
        .selection_rows()
        .find(|r| row_semantic_id(r) == "control.model-selection")
        .expect("model selection row");
    assert_eq!(
        model_row.lifecycle(),
        ConsumerRouteLifecycle::SelectionSummary
    );
    let model_val = model_row.control_value().expect("model control value");
    assert_eq!(model_val.kind(), ConsumerRouteValueKind::ExactModelRoute);
    assert_eq!(
        model_val.omission(),
        ConsumerRouteOmissionSemantics::Required
    );

    for row in contribution.session_start_rows() {
        assert_eq!(row.lifecycle(), ConsumerRouteLifecycle::SessionStartOnly);
        assert_eq!(
            row.actor_posture(),
            ConsumerRouteActorPosture::ConsumerSelectable
        );
        assert_eq!(
            *row.mutation_authority(),
            ConsumerRouteMutationAuthority::PreparedSessionStart(source("cursor.headless.source"))
        );
        assert_eq!(
            row.evidence_strength(),
            ConsumerRouteEvidenceStrength::RouteValidation
        );
        assert_eq!(
            row.source_class(),
            ConsumerRouteSourceClass::AdapterPreparedInput
        );
    }

    assert!(emitted.contains("control.fast"));
    assert!(emitted.contains("control.context-window"));
    assert!(emitted.contains("control.reasoning-effort"));
    assert!(emitted.contains("control.read-mode"));
}

#[test]
fn prepared_headless_minimal_withholds_optional_controls() {
    let headless = fixtures::headless_minimal();
    let contribution = headless
        .consumer_route_projection_contribution(source("cursor.headless.source"))
        .expect("headless minimal contributes");

    let emitted = contribution_semantic_ids(&contribution);
    assert_eq!(emitted.len(), 9);

    let observed_tuples = contribution_tuples(&contribution, CURSOR_HEADLESS_ROUTE);
    let expected = claimed_tuples(&CURSOR_HEADLESS_TRANCHE, PROFILE_HEADLESS_MINIMAL);
    assert_eq!(
        observed_tuples, expected,
        "headless minimal observed emitted tuples must match claimed ledger tuples bidirectionally"
    );

    assert!(!emitted.contains("feature.reasoning-selection"));
    assert!(!emitted.contains("control.fast"));
    assert!(!emitted.contains("control.context-window"));
    assert!(!emitted.contains("control.reasoning-effort"));
    assert!(!emitted.contains("control.read-mode"));

    assert!(emitted.contains("control.model-selection"));
    assert_eq!(contribution.session_start_rows().len(), 0);
    assert_eq!(contribution.active_session_rows().len(), 1);
    assert_eq!(contribution.selection_rows().len(), 8);
}

#[test]
fn negative_coverage_audits_are_withheld() {
    let acp = fixtures::acp();
    let acp_contribution = acp
        .consumer_route_projection_contribution(source("cursor.acp.source"))
        .expect("acp contributes");
    let acp_emitted = contribution_semantic_ids(&acp_contribution);
    assert!(!acp_emitted.contains("audit.no-public-route-specific-selectable-control"));

    let catalogue = fixtures::catalogue();
    let cat_contribution = catalogue
        .consumer_route_projection_contribution(source("cursor.catalogue.source"))
        .expect("catalogue contributes");
    let cat_emitted = contribution_semantic_ids(&cat_contribution);
    assert!(!cat_emitted.contains("audit.no-public-route-specific-selectable-control"));

    let headless = fixtures::headless_maximal();
    let head_contribution = headless
        .consumer_route_projection_contribution(source("cursor.headless.source"))
        .expect("headless contributes");
    let head_emitted = contribution_semantic_ids(&head_contribution);
    assert!(!head_emitted.contains("audit.no-public-route-specific-selectable-control"));
}
