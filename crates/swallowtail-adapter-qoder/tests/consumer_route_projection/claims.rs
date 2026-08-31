use swallowtail_core::{
    CredentialState, EndpointAuthorization, EntitlementState, RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteEvidenceStrength, ConsumerRouteFeatureId,
    ConsumerRouteLifecycle, ConsumerRouteMutationAuthority, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionFailureKind, ConsumerRouteProjectionRow,
    ConsumerRouteProjectionSourceKind, ConsumerRouteRowIdentity, ConsumerRouteSourceClass,
    PreparationStage,
};

use super::fixtures::*;
use super::naming::*;

/// Proves the census no-control audit stays negative coverage.
///
/// The prepared session publishes no session-start view, no control identity,
/// and no consumer-selectable posture, so a route-specific composer control
/// cannot be read out of a common working-resource, transport, or preparation
/// input.
#[test]
fn the_no_route_specific_control_audit_produces_no_public_control_descriptor() {
    let published = contribution(&run(), "qoder.headless.audit");
    assert_eq!(
        published.session_start_rows().len(),
        0,
        "qoder.headless admits no session-start control"
    );
    for row in all_rows(&published) {
        assert!(
            matches!(row.identity(), ConsumerRouteRowIdentity::Feature(_)),
            "{:?} is a control identity this route never publishes",
            row.identity()
        );
        assert!(
            row.control_value().is_none(),
            "{:?} publishes a selectable value domain",
            row.identity()
        );
        assert_ne!(
            row.actor_posture(),
            ConsumerRouteActorPosture::ConsumerSelectable
        );
        assert_eq!(
            row.mutation_authority(),
            &ConsumerRouteMutationAuthority::Absent
        );
        assert!(row.identity().namespaced_extension().is_none());
    }
}

#[test]
fn prepared_qoder_rows_claim_no_acknowledgement_or_per_turn_authority() {
    let published = contribution(&run(), "qoder.headless.authority");
    let named = published
        .sources()
        .map(|source| (source.id().as_str().to_owned(), source.kind()))
        .collect::<Vec<_>>();
    assert_eq!(
        named,
        vec![(
            "qoder.headless.authority".to_owned(),
            ConsumerRouteProjectionSourceKind::AdapterContribution
        )],
        "session preparation names one adapter-contribution source and no observation source"
    );

    for row in all_rows(&published) {
        assert!(
            matches!(
                row.source_class(),
                ConsumerRouteSourceClass::PreparedOperationRecord
                    | ConsumerRouteSourceClass::CapabilityProfile
            ),
            "{:?} does not carry exact prepared or capability authority",
            row.identity()
        );
        assert_eq!(
            row.evidence_strength(),
            ConsumerRouteEvidenceStrength::PreparedOperation
        );
        assert!(row.state_support().is_descriptor_only());
        assert!(!row.mutation_authority().is_acknowledged());
        assert!(!row.mutation_authority().is_consumer_mediated_per_turn());
        assert!(row.safe_reason().is_none());
    }

    let observation = published
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
        !observation.state_support().observed(),
        "prepared capability evidence proves no post-open observation"
    );

    let rendered = format!("{published:?}");
    for forbidden in [
        "/fixture/bin/",
        "qoder.projection.isolated",
        "qoder.projection.workspace",
    ] {
        assert!(
            !rendered.contains(forbidden),
            "a projected row must not carry raw target, environment, or resource data"
        );
    }
}

#[test]
fn a_changed_source_id_replaces_the_snapshot_rather_than_merging_into_it() {
    let run = run();
    let first = contribution(&run, "qoder.headless.first");
    let second = contribution(&run, "qoder.headless.second");
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
        assert_eq!(row.source().id().as_str(), "qoder.headless.second");
    }
}

/// Proves a row proved under another configured revision fails closed.
#[test]
fn a_row_from_another_prepared_revision_is_rejected_at_admission() {
    let alternate = contribution(&alternate_revision(), "qoder.headless.alternate");
    let borrowed = alternate
        .selection_rows()
        .next()
        .expect("the alternate run publishes rows")
        .clone();
    let mine = contribution(&run(), "qoder.headless.local");
    assert_ne!(
        alternate.applicability().instance_revision(),
        mine.applicability().instance_revision(),
        "only the exact configured revision separates the two snapshots"
    );
    let rejection = ConsumerRouteProjectionContribution::new(
        mine.applicability().clone(),
        mine.sources().cloned().collect::<Vec<_>>(),
        [borrowed],
        [],
        [],
    )
    .expect_err("a row bound to another configured revision cannot join this snapshot");
    assert_eq!(
        rejection.kind(),
        ConsumerRouteProjectionFailureKind::ApplicabilityDisagreement
    );
}

/// Proves a drifted access dimension yields no `qoder.headless` row at all.
#[test]
fn a_drifted_access_dimension_publishes_no_qoder_row() {
    for observed in drifted_observations() {
        let failure = prepared_with("1", observed)
            .and_then(|integration| integration.prepare_run(run_input("drift")))
            .expect_err("drifted access evidence fails closed before a row exists");
        assert!(
            matches!(
                failure.stage(),
                PreparationStage::AccessEvidence | PreparationStage::Preflight
            ),
            "drifted access evidence is rejected at the access or preflight stage, not later"
        );
    }
}

/// Proves the admitted snapshot keeps all five access dimensions observable.
#[test]
fn the_admitted_snapshot_keeps_each_access_dimension_observable() {
    let admitted = contribution(&run(), "qoder.headless.dimensions");
    let applicability = admitted.applicability();
    assert_eq!(
        applicability.credential_state(),
        CredentialState::NotRequired
    );
    assert_eq!(
        applicability.entitlement_state(),
        EntitlementState::Available
    );
    assert_eq!(
        applicability.endpoint_authorization(),
        EndpointAuthorization::Allowed
    );
    assert_eq!(applicability.runtime_readiness(), RuntimeReadiness::Ready);
    assert_eq!(
        applicability.support_authority(),
        SupportAuthority::ProviderSupported
    );
    assert!(
        applicability.model().is_none(),
        "the prepared headless run binds no model route, so no model row may be published"
    );
}

pub(super) fn all_rows(
    contribution: &ConsumerRouteProjectionContribution,
) -> impl Iterator<Item = &ConsumerRouteProjectionRow> {
    contribution
        .selection_rows()
        .chain(contribution.session_start_rows())
        .chain(contribution.active_session_rows())
}
