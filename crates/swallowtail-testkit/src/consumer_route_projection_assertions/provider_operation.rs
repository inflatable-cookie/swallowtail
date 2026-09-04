//! Provider-operation observation assertions for Contract 061.

mod admission;
mod composition;
mod state;
mod support;

/// Proves provider-operation observation stays outcome-backed and additive.
pub fn assert_consumer_route_provider_operation_observation_contract() {
    admission::assert_provider_operation_session_shape_is_rejected();
    admission::assert_prepared_record_cannot_masquerade_as_provider_operation_observation();
    admission::assert_provider_operation_row_maximum_is_fixed();
    state::assert_provider_operation_state_is_honest_descriptor_only_observation();
    composition::assert_provider_operation_sources_compose_without_merging();
    composition::assert_provider_operation_cross_access_and_source_disagreement_fail_closed();
}
