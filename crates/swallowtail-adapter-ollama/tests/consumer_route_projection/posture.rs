use std::collections::BTreeSet;
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteAvailability, ConsumerRouteLifecycle,
    ConsumerRouteProjectionContribution, ConsumerRouteSupportPosture,
};

use super::fixtures::{inference_maximal_contribution, session_contribution};
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

fn activity() -> Posture {
    (
        "feature.activity-observation".to_owned(),
        "active-session",
        ConsumerRouteLifecycle::PostOpenObservationOnly,
        ConsumerRouteActorPosture::ObservationOnly,
    )
}

fn expected_maximal_inference() -> BTreeSet<Posture> {
    BTreeSet::from([
        summary("feature.prepared-facade"),
        summary("feature.structured-run"),
        summary("feature.streaming-events"),
        summary("feature.usage-evidence"),
        summary("feature.output-token-limit"),
        summary("feature.reasoning-selection"),
        summary("feature.structured-output"),
        selectable_summary("control.model-selection"),
        session_start("control.maximum-output-tokens"),
        session_start("control.reasoning-selection"),
        session_start("control.structured-output"),
        session_start("control.context-window"),
        activity(),
    ])
}

fn expected_session() -> BTreeSet<Posture> {
    BTreeSet::from([
        summary("feature.prepared-facade"),
        summary("feature.interactive-session"),
        summary("feature.streaming-events"),
        summary("feature.usage-evidence"),
        summary("feature.output-token-limit"),
        summary("feature.cancellation-or-interruption"),
        selectable_summary("control.model-selection"),
        session_start("control.context-window"),
        activity(),
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
        observed(&inference_maximal_contribution("ollama.posture.run")),
        expected_maximal_inference()
    );
    assert_eq!(
        observed(&session_contribution("ollama.posture.session")),
        expected_session()
    );
}

#[test]
fn every_published_row_publishes_supported_and_available_separately() {
    for published in [
        inference_maximal_contribution("ollama.support.run"),
        session_contribution("ollama.support.session"),
    ] {
        for row in all_rows(&published) {
            assert_eq!(row.support(), ConsumerRouteSupportPosture::Supported);
            assert_eq!(row.availability(), ConsumerRouteAvailability::Available);
        }
    }
}
