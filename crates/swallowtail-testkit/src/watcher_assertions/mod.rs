//! Contract 059 portable watcher lifecycle assertions.

mod activity;
mod control;
mod identity;
mod lifecycle;

pub(crate) const WATCHER_RULE: &str = "Contract 059 portable watcher lifecycle";

pub use activity::assert_watcher_activity_projection;
pub use control::{assert_watcher_model_operator_roles, assert_watcher_stale_id_fails_closed};
pub use identity::{
    assert_watcher_byte_bounds, assert_watcher_capacity_bound, assert_watcher_identity_redaction,
    assert_watcher_ownership_rejection,
};
pub use lifecycle::{
    assert_watcher_cleanup_rejects_completed, assert_watcher_completion_stop_race,
    assert_watcher_lifecycle_transitions, assert_watcher_wait_representation,
};

/// Runs the Contract 059 portable watcher assertion pack.
pub fn assert_portable_watcher_lifecycle_contract() {
    use swallowtail_core::{WatcherId, WatcherSummary};
    use swallowtail_runtime::{RuntimeTurnId, WatcherRegistry};

    assert_watcher_identity_redaction(
        &WatcherId::new("watcher-opaque").expect("id"),
        &WatcherSummary::new("bounded progress").expect("summary"),
    );
    assert_watcher_byte_bounds();

    let turn = RuntimeTurnId::new("turn-pack").expect("turn is valid");
    let registry = WatcherRegistry::new(turn.clone(), 2).expect("registry");
    assert_watcher_ownership_rejection(&registry);
    assert_watcher_capacity_bound(WatcherRegistry::new(turn, 1).expect("capacity registry"));
    assert_watcher_lifecycle_transitions();
    assert_watcher_completion_stop_race();
    assert_watcher_wait_representation();
    assert_watcher_model_operator_roles();
    assert_watcher_stale_id_fails_closed();
    assert_watcher_cleanup_rejects_completed();
    assert_watcher_activity_projection();
}

#[cfg(test)]
mod tests {
    use super::assert_portable_watcher_lifecycle_contract;

    #[test]
    fn portable_watcher_lifecycle_contract_holds() {
        assert_portable_watcher_lifecycle_contract();
    }
}
