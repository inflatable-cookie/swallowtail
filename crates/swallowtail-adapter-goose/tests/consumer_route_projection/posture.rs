use std::collections::BTreeSet;
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteAvailability, ConsumerRouteEvidenceStrength,
    ConsumerRouteLifecycle, ConsumerRouteProjectionContribution, ConsumerRouteProjectionSourceKind,
    ConsumerRouteSourceClass, ConsumerRouteStateSupport, ConsumerRouteSupportPosture,
};

use super::{identity_name, prepared_session, rows, source};

pub(super) fn observed_tuples(
    contribution: &ConsumerRouteProjectionContribution,
) -> BTreeSet<(String, String, String)> {
    super::identities(contribution)
        .into_iter()
        .map(|semantic| {
            let shape = match semantic {
                "feature.interactive-session" => "interactive-session",
                "feature.streaming-events" | "feature.activity-observation" => "route-observation",
                "feature.cancellation-or-interruption"
                | "feature.working-resource"
                | "feature.prepared-facade" => "route-capability",
                _ => panic!("unexpected observed Goose tuple {semantic}"),
            };
            (
                "goose.acp".to_owned(),
                shape.to_owned(),
                semantic.to_owned(),
            )
        })
        .collect()
}

type Posture = (
    &'static str,
    &'static str,
    ConsumerRouteLifecycle,
    ConsumerRouteActorPosture,
    ConsumerRouteStateSupport,
    ConsumerRouteSourceClass,
    ConsumerRouteEvidenceStrength,
);

fn expected() -> BTreeSet<Posture> {
    BTreeSet::from([
        summary(
            "feature.interactive-session",
            ConsumerRouteSourceClass::CapabilityProfile,
        ),
        summary(
            "feature.streaming-events",
            ConsumerRouteSourceClass::CapabilityProfile,
        ),
        summary(
            "feature.cancellation-or-interruption",
            ConsumerRouteSourceClass::CapabilityProfile,
        ),
        summary(
            "feature.working-resource",
            ConsumerRouteSourceClass::CapabilityProfile,
        ),
        summary(
            "feature.prepared-facade",
            ConsumerRouteSourceClass::PreparedOperationRecord,
        ),
        (
            "feature.activity-observation",
            "active-session",
            ConsumerRouteLifecycle::PostOpenObservationOnly,
            ConsumerRouteActorPosture::ObservationOnly,
            ConsumerRouteStateSupport::descriptor_only(),
            ConsumerRouteSourceClass::PreparedOperationRecord,
            ConsumerRouteEvidenceStrength::PreparedOperation,
        ),
    ])
}

fn summary(semantic: &'static str, source_class: ConsumerRouteSourceClass) -> Posture {
    (
        semantic,
        "selection-summary",
        ConsumerRouteLifecycle::SelectionSummary,
        ConsumerRouteActorPosture::Informational,
        ConsumerRouteStateSupport::descriptor_only(),
        source_class,
        ConsumerRouteEvidenceStrength::PreparedOperation,
    )
}

#[test]
fn every_identity_keeps_exact_posture_and_source() {
    let source_id = source("goose.projection.posture");
    let contribution = prepared_session()
        .consumer_route_projection_contribution(source_id.clone())
        .expect("session contributes");
    assert_eq!(observed(&contribution), expected());
    assert_eq!(rows(&contribution).count(), 6);
    for row in rows(&contribution) {
        assert_eq!(row.source().id(), &source_id);
        assert_eq!(
            row.source().kind(),
            ConsumerRouteProjectionSourceKind::AdapterContribution
        );
        assert_eq!(row.support(), ConsumerRouteSupportPosture::Supported);
        assert_eq!(row.availability(), ConsumerRouteAvailability::Available);
        assert!(row.mutation_authority().source().is_none());
        assert!(row.control_value().is_none());
    }
}

fn observed(contribution: &ConsumerRouteProjectionContribution) -> BTreeSet<Posture> {
    contribution
        .selection_rows()
        .map(|row| (row, "selection-summary"))
        .chain(
            contribution
                .active_session_rows()
                .map(|row| (row, "active-session")),
        )
        .map(|(row, view)| {
            (
                identity_name(row),
                view,
                row.lifecycle(),
                row.actor_posture(),
                row.state_support(),
                row.source_class(),
                row.evidence_strength(),
            )
        })
        .collect()
}
