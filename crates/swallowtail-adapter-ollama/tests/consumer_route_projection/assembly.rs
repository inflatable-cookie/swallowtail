use swallowtail_runtime::{
    ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailureKind,
};

use super::fixtures::{inference_contribution, inventory_contribution};
use super::naming::all_rows;

#[test]
fn inventory_and_inference_rows_cannot_join_under_a_matching_source_id() {
    let inventory = inventory_contribution("ollama.assembly.shared");
    let inference = inference_contribution("ollama.assembly.shared");
    assert_ne!(
        inventory.applicability().driver_role(),
        inference.applicability().driver_role()
    );
    let rejection = ConsumerRouteProjectionContribution::new(
        inference.applicability().clone(),
        inference.sources().cloned().collect::<Vec<_>>(),
        [all_rows(&inventory)
            .next()
            .expect("inventory publishes")
            .clone()],
        [],
        [],
    )
    .expect_err("a catalogue row cannot join the structured-run snapshot");
    assert_eq!(
        rejection.kind(),
        ConsumerRouteProjectionFailureKind::ApplicabilityDisagreement
    );
}
