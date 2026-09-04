use super::fixtures;
use super::ledger::*;
use super::naming::{
    claimed_tuples, contribution_semantic_ids, contribution_tuples, row_semantic_id, source,
};
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteEvidenceStrength, ConsumerRouteLifecycle,
    ConsumerRouteMutationAuthority, ConsumerRouteOmissionSemantics,
    ConsumerRouteProjectionSourceKind, ConsumerRouteSourceClass, ConsumerRouteStateSupport,
    ConsumerRouteValueKind,
};

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
