use swallowtail_runtime::{
    ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailureKind,
};

use super::fixtures::{inference_contribution, session_contribution};
use super::naming::all_rows;

const SHARED: &str = "ollama.shared-source";

fn assert_rejects(
    applicability: swallowtail_runtime::ConsumerRouteApplicability,
    mine: &ConsumerRouteProjectionContribution,
    row: swallowtail_runtime::ConsumerRouteProjectionRow,
) {
    let rejection = ConsumerRouteProjectionContribution::new(
        applicability,
        mine.sources().cloned().collect::<Vec<_>>(),
        [row],
        [],
        [],
    )
    .expect_err("a row proved under other evidence cannot join this snapshot");
    assert_eq!(
        rejection.kind(),
        ConsumerRouteProjectionFailureKind::ApplicabilityDisagreement
    );
}

#[test]
fn structured_run_and_interactive_session_rows_cannot_join() {
    let run = inference_contribution(SHARED);
    let session = session_contribution(SHARED);
    assert_ne!(
        run.applicability().operation_shape(),
        session.applicability().operation_shape()
    );
    assert_rejects(
        session.applicability().clone(),
        &session,
        all_rows(&run).next().expect("run publishes").clone(),
    );
    assert_rejects(
        run.applicability().clone(),
        &run,
        all_rows(&session)
            .next()
            .expect("session publishes")
            .clone(),
    );
}
