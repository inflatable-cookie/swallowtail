use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteEvidenceStrength, ConsumerRouteFeatureId,
    ConsumerRouteLifecycle, ConsumerRouteProjectionSourceKind, ConsumerRouteRowIdentity,
    ConsumerRouteSourceClass,
};

use super::fixtures::{inference_maximal_contribution, session_contribution};
use super::naming::all_rows;

#[test]
fn prepared_ollama_rows_claim_no_acknowledgement_or_per_turn_authority() {
    for (published, label) in [
        (
            inference_maximal_contribution("ollama.attached.authority.run"),
            "inference",
        ),
        (
            session_contribution("ollama.attached.authority.session"),
            "session",
        ),
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
        let observation = published
            .active_session_rows()
            .find(|row| {
                row.identity()
                    == &ConsumerRouteRowIdentity::Feature(
                        ConsumerRouteFeatureId::ActivityObservation,
                    )
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
        assert!(observation.state_support().is_descriptor_only());
    }
}
