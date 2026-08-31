use crate::ZcodeAppServerMode;
use crate::prepared::projection_fixture;
use swallowtail_core::{
    CredentialState, EndpointAuthorization, EntitlementState, RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteEvidenceStrength, ConsumerRouteFeatureId,
    ConsumerRouteLifecycle, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionFailureKind, ConsumerRouteProjectionSourceKind,
    ConsumerRouteRowIdentity, ConsumerRouteSourceClass, PreparationStage,
};

use super::fixtures::{contribution, profile};
use super::ledger::PLAN;
use super::naming::{all_rows, rows};

#[test]
fn prepared_zcode_rows_claim_no_acknowledgement_or_per_turn_authority() {
    let published = contribution(&profile(PLAN), "zcode.app-server.authority");
    let named = published
        .sources()
        .map(|source| (source.id().as_str().to_owned(), source.kind()))
        .collect::<Vec<_>>();
    assert_eq!(
        named,
        vec![(
            "zcode.app-server.authority".to_owned(),
            ConsumerRouteProjectionSourceKind::AdapterContribution
        )],
        "run preparation names one adapter-contribution source and no observation source"
    );

    for row in all_rows(&published) {
        assert!(
            matches!(
                row.source_class(),
                ConsumerRouteSourceClass::PreparedOperationRecord
                    | ConsumerRouteSourceClass::CapabilityProfile
                    | ConsumerRouteSourceClass::AdapterPreparedInput
            ),
            "{:?} does not carry exact prepared or route-validation authority",
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
        assert!(!row.state_support().observed());
        assert!(!row.mutation_authority().is_acknowledged());
        assert!(!row.mutation_authority().is_consumer_mediated_per_turn());
        assert!(row.safe_reason().is_none());
    }
    for row in published.session_start_rows() {
        assert_eq!(row.lifecycle(), ConsumerRouteLifecycle::SessionStartOnly);
        assert!(row.state_support().requested() && row.state_support().prepared());
        assert!(row.mutation_authority().is_prepared_session_start());
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
        observation.state_support().is_descriptor_only(),
        "prepared capability evidence proves no post-open observation"
    );

    let rendered = format!("{published:?}");
    for forbidden in [
        "/fixture/bin/node",
        "/fixture/vendor/",
        "/fixture/settings.json",
        "private projection prompt",
    ] {
        assert!(
            !rendered.contains(forbidden),
            "a projected row must not carry raw target, command, or content data"
        );
    }
}

#[test]
fn a_changed_source_id_replaces_the_snapshot_rather_than_merging_into_it() {
    let run = profile(PLAN);
    let first = contribution(&run, "zcode.app-server.first");
    let second = contribution(&run, "zcode.app-server.second");
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
        assert_eq!(row.source().id().as_str(), "zcode.app-server.second");
        if let Some(authority) = row.mutation_authority().source() {
            assert_eq!(
                authority.as_str(),
                "zcode.app-server.second",
                "prepared authority names the replacement source"
            );
        }
    }
}

/// Proves a row proved under another configured revision fails closed.
#[test]
fn a_row_from_another_prepared_revision_is_rejected_at_admission() {
    let alternate = contribution(
        &projection_fixture::alternate_revision(),
        "zcode.app-server.alternate",
    );
    let borrowed = alternate
        .selection_rows()
        .next()
        .expect("the alternate run publishes rows")
        .clone();
    let mine = contribution(&profile(PLAN), "zcode.app-server.local");
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

/// Proves a drifted access dimension yields no `zcode.app-server` row at all.
#[test]
fn a_drifted_access_dimension_publishes_no_zcode_row() {
    for observed in projection_fixture::drifted_observations() {
        let failure = projection_fixture::prepared_with("prepared-1", observed)
            .and_then(|integration| {
                integration.prepare_run(projection_fixture::run_input(
                    "drift",
                    ZcodeAppServerMode::plan(),
                ))
            })
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
    let admitted = contribution(&profile(PLAN), "zcode.app-server.dimensions");
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
}
