//! Portable Contract 061 projection conformance assertions.
//!
//! Every assertion uses runtime and testkit types only. No adapter, provider,
//! transport, or live evidence takes part.

mod compound_acknowledgement;
mod counterexamples;
mod failures;
mod maxima;
mod provider_operation;
mod replacement;
mod separation;
mod support;

/// Runs the complete portable Contract 061 projection conformance suite.
pub fn assert_consumer_route_projection_contract() {
    assert_compound_acknowledgement_associates_each_half_state();
    assert_compound_acknowledgement_preserves_exact_provider_values();
    assert_compound_acknowledgement_terminal_not_dispatched_is_distinct();
    assert_compound_acknowledgement_rejects_impossible_half_combinations();
    assert_compound_acknowledgement_preserves_reasoning_first_order();
    assert_compound_acknowledgement_requires_observation_source();
    maxima::assert_fixed_maxima();
    failures::assert_failure_kinds();
    counterexamples::assert_named_counterexamples();
    counterexamples::assert_source_kind_is_part_of_identity();
    separation::assert_view_and_lifecycle_separation();
    separation::assert_exact_access_dimensions_stay_observable();
    separation::assert_consumer_mediated_per_turn_authority();
    replacement::assert_identical_row_source_replacement();
    replacement::assert_unknown_and_absent_truth_survives();
    replacement::assert_no_raw_or_presentation_data();
    assert_consumer_route_provider_operation_observation_contract();
}

pub use compound_acknowledgement::{
    assert_compound_acknowledgement_associates_each_half_state,
    assert_compound_acknowledgement_preserves_exact_provider_values,
    assert_compound_acknowledgement_preserves_reasoning_first_order,
    assert_compound_acknowledgement_rejects_impossible_half_combinations,
    assert_compound_acknowledgement_requires_observation_source,
    assert_compound_acknowledgement_terminal_not_dispatched_is_distinct,
};
pub use provider_operation::assert_consumer_route_provider_operation_observation_contract;
