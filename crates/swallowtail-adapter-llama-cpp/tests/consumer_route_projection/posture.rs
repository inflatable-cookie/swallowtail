use std::collections::BTreeSet;
use swallowtail_core::{
    CredentialState, EndpointAuthorization, EntitlementState, RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteAvailability, ConsumerRouteLifecycle,
    ConsumerRouteProjectionContribution, ConsumerRouteSupportPosture,
};

use super::fixtures::{inference_contribution, ready_status, serving_contribution};
use super::naming::{all_rows, semantic_id};

type Posture = (
    String,
    &'static str,
    ConsumerRouteLifecycle,
    ConsumerRouteActorPosture,
);

fn summary(semantic: &str) -> Posture {
    (
        semantic.to_owned(),
        "selection-summary",
        ConsumerRouteLifecycle::SelectionSummary,
        ConsumerRouteActorPosture::Informational,
    )
}

fn selectable_summary(semantic: &str) -> Posture {
    (
        semantic.to_owned(),
        "selection-summary",
        ConsumerRouteLifecycle::SelectionSummary,
        ConsumerRouteActorPosture::ConsumerSelectable,
    )
}

fn session_start(semantic: &str) -> Posture {
    (
        semantic.to_owned(),
        "session-start",
        ConsumerRouteLifecycle::SessionStartOnly,
        ConsumerRouteActorPosture::ConsumerSelectable,
    )
}

fn expected_inference() -> BTreeSet<Posture> {
    BTreeSet::from([
        summary("feature.prepared-facade"),
        summary("feature.structured-run"),
        summary("feature.streaming-events"),
        summary("feature.usage-evidence"),
        summary("feature.output-token-limit"),
        selectable_summary("control.model-selection"),
        session_start("control.maximum-output-tokens"),
        (
            "feature.activity-observation".to_owned(),
            "active-session",
            ConsumerRouteLifecycle::PostOpenObservationOnly,
            ConsumerRouteActorPosture::ObservationOnly,
        ),
    ])
}

fn expected_serving() -> BTreeSet<Posture> {
    BTreeSet::from([
        summary("feature.prepared-facade"),
        summary("feature.owned-runtime-lifecycle"),
        session_start("control.serving-model-artifact"),
        session_start("control.serving-context-size"),
        session_start("control.serving-reasoning"),
    ])
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
    assert_eq!(
        observed(&inference_contribution("llama-cpp.attached.posture")),
        expected_inference()
    );
    assert_eq!(
        observed(&serving_contribution("llama-cpp.owned.posture")),
        expected_serving()
    );
}

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
        SupportAuthority::IntegrationMaintainerSupported
    );
    for published in [
        inference_contribution("llama-cpp.attached.support"),
        serving_contribution("llama-cpp.owned.support"),
    ] {
        for row in all_rows(&published) {
            assert_eq!(row.support(), ConsumerRouteSupportPosture::Supported);
            assert_eq!(row.availability(), ConsumerRouteAvailability::Available);
        }
    }
}
