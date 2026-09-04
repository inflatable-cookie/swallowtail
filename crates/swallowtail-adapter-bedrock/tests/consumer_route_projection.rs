//! Contract 061 disposition proof for Amazon Bedrock routes (catalogue and runtime).

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
    ConsumerRouteValueDomain, ConsumerRouteValueKind,
};

fn source(id: &str) -> ConsumerRouteProjectionSourceId {
    ConsumerRouteProjectionSourceId::new(id).expect("valid source id")
}

fn row_semantic_id(row: &ConsumerRouteProjectionRow) -> String {
    match row.identity() {
        ConsumerRouteRowIdentity::Feature(f) => match f {
            ConsumerRouteFeatureId::ModelCatalogue => "feature.model-catalogue",
            ConsumerRouteFeatureId::StructuredRun => "feature.structured-run",
            ConsumerRouteFeatureId::StreamingEvents => "feature.streaming-events",
            ConsumerRouteFeatureId::UsageEvidence => "feature.usage-evidence",
            ConsumerRouteFeatureId::OutputTokenLimit => "feature.output-token-limit",
            ConsumerRouteFeatureId::CancellationOrInterruption => {
                "feature.cancellation-or-interruption"
            }
            ConsumerRouteFeatureId::PreparedFacade => "feature.prepared-facade",
            ConsumerRouteFeatureId::ActivityObservation => "feature.activity-observation",
            other => panic!("unexpected feature {other:?}"),
        }
        .to_string(),
        ConsumerRouteRowIdentity::Control(c) => match c {
            ConsumerRouteControlId::ModelSelection => "control.model-selection".to_string(),
            ConsumerRouteControlId::MaximumOutputTokens => {
                "control.maximum-output-tokens".to_string()
            }
            other => panic!("unexpected control {other:?}"),
        },
    }
}

fn row_operation_shape(row: &ConsumerRouteProjectionRow) -> &'static str {
    match row.identity() {
        ConsumerRouteRowIdentity::Feature(f) => match f {
            ConsumerRouteFeatureId::ModelCatalogue => "model-catalogue",
            ConsumerRouteFeatureId::StructuredRun => "structured-run",
            ConsumerRouteFeatureId::StreamingEvents
            | ConsumerRouteFeatureId::UsageEvidence
            | ConsumerRouteFeatureId::ActivityObservation => "route-observation",
            ConsumerRouteFeatureId::OutputTokenLimit
            | ConsumerRouteFeatureId::CancellationOrInterruption
            | ConsumerRouteFeatureId::PreparedFacade => "route-capability",
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
fn catalogue_tranche_dispositions_exactly_nine_rows() {
    assert_eq!(BEDROCK_CATALOGUE_TRANCHE.len(), 9);
    let mut ledger_tuples = BTreeSet::new();
    let mut semantics = BTreeSet::new();
    let mut emitted_count = 0;
    let mut withheld_count = 0;

    for entry in &BEDROCK_CATALOGUE_TRANCHE {
        assert_eq!(entry.route_id, BEDROCK_CATALOGUE_ROUTE);
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

    let census_tuples: BTreeSet<RowTuple> = BEDROCK_CATALOGUE_CENSUS_TUPLES.into_iter().collect();
    assert_eq!(
        ledger_tuples, census_tuples,
        "bedrock.catalogue ledger tuples must match census tuples bidirectionally"
    );
    assert_eq!(emitted_count, 2);
    assert_eq!(withheld_count, 7);
}

#[test]
fn runtime_tranche_dispositions_exactly_ten_rows() {
    assert_eq!(BEDROCK_RUNTIME_TRANCHE.len(), 10);
    let mut ledger_tuples = BTreeSet::new();
    let mut semantics = BTreeSet::new();
    let mut emitted_count = 0;
    let mut withheld_count = 0;

    for entry in &BEDROCK_RUNTIME_TRANCHE {
        assert_eq!(entry.route_id, BEDROCK_RUNTIME_ROUTE);
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

    let census_tuples: BTreeSet<RowTuple> = BEDROCK_RUNTIME_CENSUS_TUPLES.into_iter().collect();
    assert_eq!(
        ledger_tuples, census_tuples,
        "bedrock.runtime ledger tuples must match census tuples bidirectionally"
    );
    assert_eq!(emitted_count, 8);
    assert_eq!(withheld_count, 2);
}

#[test]
fn package_dispositions_nineteen_rows_total() {
    let catalogue_len = BEDROCK_CATALOGUE_TRANCHE.len();
    let runtime_len = BEDROCK_RUNTIME_TRANCHE.len();
    assert_eq!(catalogue_len + runtime_len, 19);

    let catalogue_emitted = BEDROCK_CATALOGUE_TRANCHE
        .iter()
        .filter(|e| !e.emitted_by.is_empty())
        .count();
    let runtime_emitted = BEDROCK_RUNTIME_TRANCHE
        .iter()
        .filter(|e| !e.emitted_by.is_empty())
        .count();
    assert_eq!(catalogue_emitted + runtime_emitted, 10);

    let catalogue_withheld = BEDROCK_CATALOGUE_TRANCHE
        .iter()
        .filter(|e| e.emitted_by.is_empty())
        .count();
    let runtime_withheld = BEDROCK_RUNTIME_TRANCHE
        .iter()
        .filter(|e| e.emitted_by.is_empty())
        .count();
    assert_eq!(catalogue_withheld + runtime_withheld, 9);
}

#[test]
fn prepared_catalogue_emits_exact_two_rows() {
    let catalogue = fixtures::catalogue();
    let contribution = catalogue
        .consumer_route_projection_contribution(source("bedrock.catalogue.source"))
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

    let observed_tuples = contribution_tuples(&contribution, BEDROCK_CATALOGUE_ROUTE);
    let expected = claimed_tuples(&BEDROCK_CATALOGUE_TRANCHE, PROFILE_CATALOGUE);
    assert_eq!(
        observed_tuples, expected,
        "catalogue observed emitted tuples must match claimed ledger tuples bidirectionally"
    );

    for entry in &BEDROCK_CATALOGUE_TRANCHE {
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
        assert_eq!(row.lifecycle(), ConsumerRouteLifecycle::SelectionSummary);
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
fn prepared_runtime_emits_exact_eight_rows() {
    let attempt = fixtures::runtime_attempt();
    let contribution = attempt
        .consumer_route_projection_contribution(source("bedrock.runtime.source"))
        .expect("runtime contributes");

    let emitted = contribution_semantic_ids(&contribution);
    assert_eq!(emitted.len(), 8);

    let observed_tuples = contribution_tuples(&contribution, BEDROCK_RUNTIME_ROUTE);
    let expected = claimed_tuples(&BEDROCK_RUNTIME_TRANCHE, PROFILE_RUNTIME);
    assert_eq!(
        observed_tuples, expected,
        "runtime observed emitted tuples must match claimed ledger tuples bidirectionally"
    );

    for entry in &BEDROCK_RUNTIME_TRANCHE {
        if entry.emitted_by.is_empty() {
            assert!(
                !observed_tuples.contains(&(
                    entry.route_id,
                    entry.operation_shape,
                    entry.semantic_id.to_string()
                )),
                "withheld runtime row {:?} must not be emitted",
                entry.semantic_id
            );
        }
    }

    assert_eq!(contribution.selection_rows().len(), 6);
    assert_eq!(contribution.session_start_rows().len(), 1);
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

    let max_tokens_row = contribution
        .session_start_rows()
        .find(|r| row_semantic_id(r) == "control.maximum-output-tokens")
        .expect("maximum output tokens row");
    assert_eq!(
        max_tokens_row.lifecycle(),
        ConsumerRouteLifecycle::SessionStartOnly
    );
    assert_eq!(
        max_tokens_row.actor_posture(),
        ConsumerRouteActorPosture::ConsumerSelectable
    );
    assert_eq!(
        *max_tokens_row.mutation_authority(),
        ConsumerRouteMutationAuthority::PreparedSessionStart(source("bedrock.runtime.source"))
    );
    assert_eq!(
        max_tokens_row.evidence_strength(),
        ConsumerRouteEvidenceStrength::RouteValidation
    );
    assert_eq!(
        max_tokens_row.source_class(),
        ConsumerRouteSourceClass::AdapterPreparedInput
    );

    let max_val = max_tokens_row
        .control_value()
        .expect("maximum output tokens control value");
    assert_eq!(max_val.kind(), ConsumerRouteValueKind::BoundedInteger);
    assert_eq!(max_val.omission(), ConsumerRouteOmissionSemantics::Required);
    match max_val.domain() {
        ConsumerRouteValueDomain::Enumerated(enum_vals) => {
            let values: Vec<_> = enum_vals.values().map(|v| v.as_str()).collect();
            assert_eq!(values, ["1024"]);
        }
        other => panic!("expected enumerated domain with 1024, found {other:?}"),
    }
}

#[test]
fn negative_coverage_audits_are_withheld() {
    let catalogue = fixtures::catalogue();
    let contribution = catalogue
        .consumer_route_projection_contribution(source("bedrock.catalogue.source"))
        .expect("catalogue contributes");

    let emitted = contribution_semantic_ids(&contribution);
    assert!(!emitted.contains("audit.no-public-route-specific-selectable-control"));
}
