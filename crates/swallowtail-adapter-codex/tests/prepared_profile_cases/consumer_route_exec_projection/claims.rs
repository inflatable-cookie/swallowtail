use std::collections::BTreeSet;
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteControlId, ConsumerRouteEvidenceStrength,
    ConsumerRouteFeatureId, ConsumerRouteLifecycle, ConsumerRouteOmissionSemantics,
    ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailureKind,
    ConsumerRouteProjectionRow, ConsumerRouteProjectionSourceKind, ConsumerRouteRowIdentity,
    ConsumerRouteSourceClass, ConsumerRouteValueDomain, ConsumerRouteValueKind,
};

use super::fixtures::*;
use super::naming::*;

#[test]
fn exec_local_controls_publish_the_exact_prepared_value_and_omission_truth() {
    let full = contribution(&maximal(), "codex.exec.values");
    let published = full
        .session_start_rows()
        .map(|row| {
            let value = row
                .control_value()
                .expect("a consumer-selectable control publishes its value");
            (semantic_id(row.identity()), value.kind(), value.omission())
        })
        .collect::<BTreeSet<_>>();
    let expected = BTreeSet::from([
        (
            "control.reasoning-selection".to_owned(),
            ConsumerRouteValueKind::BoundedEnumeration,
            ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
        ),
        (
            "control.structured-output".to_owned(),
            ConsumerRouteValueKind::StructuredDeclarations,
            ConsumerRouteOmissionSemantics::SuppliesNothing,
        ),
        (
            "control.attachments".to_owned(),
            ConsumerRouteValueKind::StructuredDeclarations,
            ConsumerRouteOmissionSemantics::SuppliesNothing,
        ),
        (
            "control.external-network-policy".to_owned(),
            ConsumerRouteValueKind::BoundedPolicy,
            ConsumerRouteOmissionSemantics::Required,
        ),
        (
            "control.external-search-policy".to_owned(),
            ConsumerRouteValueKind::BoundedPolicy,
            ConsumerRouteOmissionSemantics::Required,
        ),
        (
            "control.model-verbosity".to_owned(),
            ConsumerRouteValueKind::BoundedEnumeration,
            ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
        ),
    ]);
    assert_eq!(published, expected);

    assert_eq!(
        admitted(&full, "control.external-network-policy"),
        ["HostApproved"]
    );
    assert_eq!(
        admitted(&full, "control.external-search-policy"),
        ["Enabled"]
    );
    assert_eq!(admitted(&full, "control.model-verbosity"), ["high"]);
    assert_eq!(admitted(&full, "control.model-selection"), ["gpt-5.4-mini"]);

    let lean = contribution(&minimal(), "codex.exec.minimal-values");
    assert_eq!(
        admitted(&lean, "control.external-network-policy"),
        ["Denied"]
    );
    assert_eq!(
        admitted(&lean, "control.external-search-policy"),
        ["Disabled"]
    );
}

#[test]
fn prepared_exec_rows_claim_no_execution_acknowledgement_or_per_turn_authority() {
    let full = contribution(&maximal(), "codex.exec.authority");
    let named = full
        .sources()
        .map(|source| (source.id().as_str().to_owned(), source.kind()))
        .collect::<Vec<_>>();
    assert_eq!(
        named,
        vec![(
            "codex.exec.authority".to_owned(),
            ConsumerRouteProjectionSourceKind::AdapterContribution
        )],
        "the prepared exec contribution names one adapter-contribution source"
    );

    for row in all_rows(&full) {
        assert!(
            matches!(
                row.source_class(),
                ConsumerRouteSourceClass::PreparedOperationRecord
                    | ConsumerRouteSourceClass::CapabilityProfile
                    | ConsumerRouteSourceClass::AdapterPreparedInput
            ),
            "{:?} does not carry exact runtime or prepared authority",
            row.identity()
        );
        assert!(matches!(
            row.evidence_strength(),
            ConsumerRouteEvidenceStrength::PreparedOperation
                | ConsumerRouteEvidenceStrength::RouteValidation
        ));
        assert!(!row.state_support().provider_effective());
        assert!(!row.state_support().rejected());
        assert!(!row.state_support().pending());
        assert!(!row.mutation_authority().is_acknowledged());
        assert!(!row.mutation_authority().is_consumer_mediated_per_turn());
        assert!(row.safe_reason().is_none());
    }
    for row in full.session_start_rows() {
        assert_eq!(row.lifecycle(), ConsumerRouteLifecycle::SessionStartOnly);
        assert!(row.state_support().requested() && row.state_support().prepared());
        assert_eq!(
            row.mutation_authority().source().map(|id| id.as_str()),
            Some("codex.exec.authority")
        );
    }

    let observation = full
        .active_session_rows()
        .find(|row| {
            row.identity()
                == &ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::ActivityObservation)
        })
        .expect("activity observation stays post-open");
    assert_eq!(
        observation.lifecycle(),
        ConsumerRouteLifecycle::PostOpenObservationOnly
    );
    assert_eq!(
        observation.actor_posture(),
        ConsumerRouteActorPosture::ObservationOnly
    );
    assert!(
        observation.state_support().is_descriptor_only(),
        "prepared capability evidence proves no post-open observation"
    );

    let rendered = format!("{full:?}");
    for forbidden in ["codex-executable", "private prompt", "saved-login"] {
        assert!(
            !rendered.contains(forbidden),
            "a projected row must not carry raw target, command, or content data"
        );
    }
}

#[test]
fn a_changed_source_id_replaces_the_snapshot_rather_than_merging_into_it() {
    let run = maximal();
    let first = contribution(&run, "codex.exec.first");
    let second = contribution(&run, "codex.exec.second");
    assert_eq!(
        rows(&first),
        rows(&second),
        "both snapshots carry one exact row set"
    );
    assert_ne!(
        first, second,
        "a changed source id produces a replacement snapshot"
    );
    for row in all_rows(&second) {
        assert_eq!(row.source().id().as_str(), "codex.exec.second");
    }
    let model = second
        .selection_rows()
        .find(|row| {
            row.identity()
                == &ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::ModelSelection)
        })
        .expect("the exact model selection is published");
    assert_eq!(
        model.mutation_authority().source().map(|id| id.as_str()),
        Some("codex.exec.second"),
        "prepared authority names the replacement source, never the previous one"
    );
}

#[test]
fn a_row_from_another_exec_model_route_is_rejected_at_admission() {
    let foreign = contribution(&foreign_model(), "codex.exec.foreign");
    let borrowed = foreign
        .selection_rows()
        .next()
        .expect("the foreign run publishes rows")
        .clone();
    let local = minimal();
    let mine = contribution(&local, "codex.exec.local");
    let rejection = ConsumerRouteProjectionContribution::new(
        mine.applicability().clone(),
        mine.sources().cloned().collect::<Vec<_>>(),
        [borrowed],
        [],
        [],
    )
    .expect_err("a row bound to another model route cannot join this snapshot");
    assert_eq!(
        rejection.kind(),
        ConsumerRouteProjectionFailureKind::ApplicabilityDisagreement
    );
}

fn all_rows(
    contribution: &ConsumerRouteProjectionContribution,
) -> impl Iterator<Item = &ConsumerRouteProjectionRow> {
    contribution
        .selection_rows()
        .chain(contribution.session_start_rows())
        .chain(contribution.active_session_rows())
}

/// Returns the exactly admitted values one published control row carries.
fn admitted(contribution: &ConsumerRouteProjectionContribution, semantic: &str) -> Vec<String> {
    let row = all_rows(contribution)
        .find(|row| semantic_id(row.identity()) == semantic)
        .unwrap_or_else(|| panic!("{semantic} is published"));
    let ConsumerRouteValueDomain::Enumerated(values) = row
        .control_value()
        .unwrap_or_else(|| panic!("{semantic} carries its exact value"))
        .domain()
    else {
        panic!("{semantic} publishes an exactly admitted domain");
    };
    values
        .values()
        .map(|value| value.as_str().to_owned())
        .collect()
}
