mod drift;
mod driver;
mod lifecycle;
mod matrix;
mod trace;

/// Runs the provider-neutral persistent-session management conformance pack.
pub fn assert_provider_session_management_contract() {
    matrix::assert_action_and_version_matrix();
    matrix::assert_topologies_and_cleanup();
    lifecycle::assert_cancellation_and_deadline_truth();
    drift::assert_request_drift_stops_before_dispatch();
}

#[cfg(test)]
mod tests {
    use swallowtail_core::CancellationScope;

    #[test]
    fn provider_session_management_contract_passes() {
        super::assert_provider_session_management_contract();
    }

    #[test]
    fn cancellation_scope_stays_management_specific() {
        assert_ne!(
            CancellationScope::ProviderSessionManagement,
            CancellationScope::InteractiveSession
        );
    }
}
