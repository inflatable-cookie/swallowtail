//! Portable Contract 061 projection conformance assertions.
//!
//! Every assertion uses runtime and testkit types only. No adapter, provider,
//! transport, or live evidence takes part.

mod counterexamples;
mod failures;
mod maxima;
mod replacement;
mod separation;
mod support;

/// Runs the complete portable Contract 061 projection conformance suite.
pub fn assert_consumer_route_projection_contract() {
    maxima::assert_fixed_maxima();
    failures::assert_failure_kinds();
    counterexamples::assert_named_counterexamples();
    separation::assert_view_and_lifecycle_separation();
    separation::assert_exact_access_dimensions_stay_observable();
    separation::assert_consumer_mediated_per_turn_authority();
    replacement::assert_identical_row_source_replacement();
    replacement::assert_unknown_and_absent_truth_survives();
    replacement::assert_no_raw_or_presentation_data();
}
