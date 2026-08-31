use std::collections::BTreeSet;
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteEvidenceStrength, ConsumerRouteFeatureId,
    ConsumerRouteLifecycle, ConsumerRouteOmissionSemantics, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionFailureKind, ConsumerRouteProjectionRow,
    ConsumerRouteProjectionSourceKind, ConsumerRouteRowIdentity, ConsumerRouteSourceClass,
    ConsumerRouteValueDomain, ConsumerRouteValueKind,
};

use super::fixtures::*;
use super::naming::*;

#[test]
fn background_controls_publish_the_exact_prepared_value_and_omission_truth() {
    let tiered = contribution(&tiered(), "openai.background.values");
    let published = tiered
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
            "control.maximum-output-tokens".to_owned(),
            ConsumerRouteValueKind::BoundedInteger,
            ConsumerRouteOmissionSemantics::Required,
        ),
        (
            "control.structured-output".to_owned(),
            ConsumerRouteValueKind::StructuredDeclarations,
            ConsumerRouteOmissionSemantics::SuppliesNothing,
        ),
        (
            "control.provider-execution-policy".to_owned(),
            ConsumerRouteValueKind::BoundedPolicy,
            ConsumerRouteOmissionSemantics::Required,
        ),
        (
            "control.provider-retention-policy".to_owned(),
            ConsumerRouteValueKind::BoundedPolicy,
            ConsumerRouteOmissionSemantics::Required,
        ),
        (
            "control.stream-reattachment".to_owned(),
            ConsumerRouteValueKind::BoundedPolicy,
            ConsumerRouteOmissionSemantics::Required,
        ),
        (
            "control.service-tier".to_owned(),
            ConsumerRouteValueKind::BoundedEnumeration,
            ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
        ),
    ]);
    assert_eq!(published, expected);

    assert_eq!(admitted(&tiered, "control.model-selection"), ["gpt-5.6"]);
    assert_eq!(admitted(&tiered, "control.reasoning-selection"), ["high"]);
    assert_eq!(admitted(&tiered, "control.maximum-output-tokens"), ["64"]);
    assert_eq!(
        admitted(&tiered, "control.provider-execution-policy"),
        ["Background"]
    );
    assert_eq!(
        admitted(&tiered, "control.provider-retention-policy"),
        ["TemporaryAllowed"]
    );
    assert_eq!(
        admitted(&tiered, "control.stream-reattachment"),
        ["Bounded(1)"]
    );
    assert_eq!(admitted(&tiered, "control.service-tier"), ["default"]);

    let detached = contribution(&detached(), "openai.background.detached-values");
    assert_eq!(
        admitted(&detached, "control.active-run-detachment"),
        ["structured-run active-run detachment"]
    );
}

#[test]
fn prepared_background_rows_claim_no_acknowledgement_recovery_or_per_turn_authority() {
    let tiered = contribution(&tiered(), "openai.background.authority");
    let named = tiered
        .sources()
        .map(|source| (source.id().as_str().to_owned(), source.kind()))
        .collect::<Vec<_>>();
    assert_eq!(
        named,
        vec![(
            "openai.background.authority".to_owned(),
            ConsumerRouteProjectionSourceKind::AdapterContribution
        )],
        "background preparation names one adapter-contribution source and no observation source"
    );

    for row in all_rows(&tiered) {
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
        assert!(
            !row.state_support().provider_effective(),
            "{:?} must not claim provider-effective state",
            row.identity()
        );
        assert!(!row.state_support().rejected());
        assert!(
            !row.state_support().pending(),
            "{:?} must not claim a pending acknowledgement the route never observes",
            row.identity()
        );
        assert!(!row.mutation_authority().is_acknowledged());
        assert!(!row.mutation_authority().is_consumer_mediated_per_turn());
        assert!(row.safe_reason().is_none());
    }
    for row in tiered.session_start_rows() {
        assert_eq!(row.lifecycle(), ConsumerRouteLifecycle::SessionStartOnly);
        assert!(row.state_support().requested() && row.state_support().prepared());
        assert!(row.mutation_authority().is_prepared_session_start());
    }

    let observation = tiered
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
    assert!(observation.state_support().observed());
    assert!(!observation.state_support().prepared());

    let rendered = format!("{tiered:?}");
    for forbidden in [
        "openai-fixture-key",
        "fixture-secret",
        "https://api.openai.com",
        "Say hello",
    ] {
        assert!(
            !rendered.contains(forbidden),
            "a projected row must not carry raw target, credential, or content data"
        );
    }
}

#[test]
fn a_changed_source_id_replaces_the_background_snapshot_rather_than_merging_into_it() {
    let run = tiered();
    let first = contribution(&run, "openai.background.first");
    let second = contribution(&run, "openai.background.second");
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
        assert_eq!(row.source().id().as_str(), "openai.background.second");
        if let Some(authority) = row.mutation_authority().source() {
            assert_eq!(
                authority.as_str(),
                "openai.background.second",
                "prepared authority names the replacement source"
            );
        }
    }
}

#[test]
fn a_row_from_another_background_route_revision_is_rejected_at_admission() {
    let alternate = contribution(&alternate_revision(), "openai.background.alternate");
    let borrowed = alternate
        .selection_rows()
        .next()
        .expect("the alternate run publishes rows")
        .clone();
    let mine = contribution(&minimal(), "openai.background.local");
    let rejection = ConsumerRouteProjectionContribution::new(
        mine.applicability().clone(),
        mine.sources().cloned().collect::<Vec<_>>(),
        [borrowed],
        [],
        [],
    )
    .expect_err("a row bound to another route revision cannot join this snapshot");
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
