mod admission;
mod state;
mod support;

pub use admission::assert_compound_acknowledgement_requires_observation_source;
pub use state::{
    assert_compound_acknowledgement_associates_each_half_state,
    assert_compound_acknowledgement_preserves_exact_provider_values,
    assert_compound_acknowledgement_preserves_reasoning_first_order,
    assert_compound_acknowledgement_rejects_impossible_half_combinations,
    assert_compound_acknowledgement_terminal_not_dispatched_is_distinct,
};
