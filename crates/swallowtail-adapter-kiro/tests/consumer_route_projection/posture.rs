use std::collections::BTreeSet;
use swallowtail_core::{
    CredentialState, EndpointAuthorization, EntitlementState, RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteAvailability, ConsumerRouteLifecycle,
    ConsumerRouteProjectionContribution, ConsumerRouteSupportPosture,
};

use super::claims::all_rows;
use super::fixtures::{contribution, ready_status, session};
use super::naming::semantic_id;

/// Projection view, lifecycle, and actor posture of one published row.
type Posture = (
    String,
    &'static str,
    ConsumerRouteLifecycle,
    ConsumerRouteActorPosture,
);

/// Exact view, lifecycle, and actor posture of every published row.
///
/// The map is written per identity rather than per view, so moving one
/// ordinary feature into another view or lifecycle fails here even when the
/// emitted identity set is unchanged.
fn expected() -> BTreeSet<Posture> {
    BTreeSet::from([
        summary("feature.prepared-facade"),
        summary("feature.interactive-session"),
        summary("feature.streaming-events"),
        summary("feature.cancellation-or-interruption"),
        summary("feature.working-resource"),
        (
            "feature.activity-observation".to_owned(),
            "active-session",
            ConsumerRouteLifecycle::PostOpenObservationOnly,
            ConsumerRouteActorPosture::ObservationOnly,
        ),
    ])
}

fn summary(semantic: &str) -> Posture {
    (
        semantic.to_owned(),
        "selection-summary",
        ConsumerRouteLifecycle::SelectionSummary,
        ConsumerRouteActorPosture::Informational,
    )
}

fn observed(contribution: &ConsumerRouteProjectionContribution) -> BTreeSet<Posture> {
    let selection = contribution
        .selection_rows()
        .map(|row| (row, "selection-summary"));
    let session_start = contribution
        .session_start_rows()
        .map(|row| (row, "session-start"));
    let active = contribution
        .active_session_rows()
        .map(|row| (row, "active-session"));
    selection
        .chain(session_start)
        .chain(active)
        .map(|(row, view)| {
            (
                semantic_id(row.identity()),
                view,
                row.lifecycle(),
                row.actor_posture(),
            )
        })
        .collect()
}

#[test]
fn every_published_row_keeps_its_exact_view_lifecycle_and_actor_posture() {
    let published = contribution(&session(), "kiro.acp.posture");
    assert_eq!(observed(&published), expected());
}

/// Proves support and availability stay exact and separately observable.
///
/// The route is host-owned and unauthenticated, so its exact ready access
/// evidence is available rather than conditional. Weakening that mapping, or
/// flattening support and availability into one dimension, fails here.
#[test]
fn every_published_row_publishes_supported_and_available_separately() {
    let ready = ready_status();
    assert_eq!(ready.credential(), CredentialState::NotRequired);
    assert_eq!(ready.entitlement(), EntitlementState::Available);
    assert_eq!(
        ready.endpoint_authorization(),
        EndpointAuthorization::Allowed
    );
    assert_eq!(ready.runtime_readiness(), RuntimeReadiness::Ready);
    assert_eq!(
        ready.support_authority(),
        SupportAuthority::ProviderSupported
    );

    let published = contribution(&session(), "kiro.acp.support");
    assert_eq!(all_rows(&published).count(), 6);
    for row in all_rows(&published) {
        assert_eq!(
            row.support(),
            ConsumerRouteSupportPosture::Supported,
            "{:?} must publish its exact route support",
            row.identity()
        );
        assert_eq!(
            row.availability(),
            ConsumerRouteAvailability::Available,
            "{:?} must publish exact availability under ready access evidence",
            row.identity()
        );
    }
}
