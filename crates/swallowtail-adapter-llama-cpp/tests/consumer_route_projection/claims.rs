use swallowtail_core::{
    CredentialState, EndpointAuthorization, EntitlementState, RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteEvidenceStrength, ConsumerRouteFeatureId,
    ConsumerRouteLifecycle, ConsumerRouteProjectionFailureKind, ConsumerRouteProjectionSourceKind,
    ConsumerRouteRowIdentity, ConsumerRouteSourceClass, PreparationStage,
};

use super::fixtures::*;
use super::naming::*;

#[test]
fn prepared_llama_cpp_rows_claim_no_acknowledgement_or_per_turn_authority() {
    for (published, label) in [
        (
            inference_contribution("llama-cpp.attached.authority"),
            "attached",
        ),
        (serving_contribution("llama-cpp.owned.authority"), "owned"),
    ] {
        let named = published
            .sources()
            .map(|source| (source.id().as_str().to_owned(), source.kind()))
            .collect::<Vec<_>>();
        assert_eq!(
            named.len(),
            1,
            "{label} names one adapter-contribution source"
        );
        assert_eq!(
            named[0].1,
            ConsumerRouteProjectionSourceKind::AdapterContribution
        );
        for row in all_rows(&published) {
            assert!(
                matches!(
                    row.source_class(),
                    ConsumerRouteSourceClass::PreparedOperationRecord
                        | ConsumerRouteSourceClass::CapabilityProfile
                        | ConsumerRouteSourceClass::AdapterPreparedInput
                ),
                "{label} {:?} lacks prepared authority",
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
    }

    let observation = inference_contribution("llama-cpp.attached.activity")
        .active_session_rows()
        .find(|row| {
            row.identity()
                == &ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::ActivityObservation)
        })
        .expect("activity observation stays post-open")
        .clone();
    assert_eq!(
        observation.lifecycle(),
        ConsumerRouteLifecycle::PostOpenObservationOnly
    );
    assert_eq!(
        observation.actor_posture(),
        ConsumerRouteActorPosture::ObservationOnly
    );
    assert!(observation.state_support().is_descriptor_only());
}

#[test]
fn a_changed_source_id_replaces_the_snapshot_rather_than_merging_into_it() {
    let first = inference_contribution("llama-cpp.attached.first");
    let second = inference_contribution("llama-cpp.attached.second");
    assert_eq!(rows(&first), rows(&second));
    assert_ne!(first, second);
    for row in all_rows(&second) {
        assert_eq!(row.source().id().as_str(), "llama-cpp.attached.second");
    }
}

#[test]
fn a_row_from_another_prepared_revision_is_rejected_at_admission() {
    let alternate = alternate_inference()
        .consumer_route_projection_contribution(source("llama-cpp.attached.alternate"))
        .expect("alternate contributes");
    let borrowed = alternate
        .selection_rows()
        .next()
        .expect("the alternate run publishes rows")
        .clone();
    let mine = inference_contribution("llama-cpp.attached.local");
    assert_ne!(
        alternate.applicability().instance_revision(),
        mine.applicability().instance_revision()
    );
    let rejection = swallowtail_runtime::ConsumerRouteProjectionContribution::new(
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

#[test]
fn a_drifted_access_dimension_publishes_no_attached_row() {
    for observed in drifted_observations() {
        let failure = match attached_with("host.llama-cpp", "1", observed) {
            Err(failure) => failure,
            Ok(integration) => integration
                .prepare_catalogue(
                    swallowtail_adapter_llama_cpp::LlamaCppCatalogueProfileInput::new(
                        swallowtail_runtime::RequestId::new("llama-cpp.projection.drift")
                            .expect("request"),
                    ),
                )
                .err()
                .expect("drifted access evidence fails closed before a row exists"),
        };
        assert!(
            matches!(
                failure.stage(),
                PreparationStage::AccessEvidence | PreparationStage::Preflight
            ),
            "drifted access evidence is rejected at the access or preflight stage, not later"
        );
    }
}

#[test]
fn the_admitted_snapshot_keeps_each_access_dimension_observable() {
    let admitted = inference_contribution("llama-cpp.attached.dimensions");
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
        SupportAuthority::IntegrationMaintainerSupported
    );
    assert!(applicability.model().is_some());
    let catalogue = catalogue_contribution("llama-cpp.attached.catalogue.dimensions");
    assert!(
        catalogue.applicability().model().is_none(),
        "the prepared catalogue binds no model route"
    );
}
