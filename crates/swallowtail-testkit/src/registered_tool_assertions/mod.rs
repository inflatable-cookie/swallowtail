//! Contract 063 provider-free conformance oracle for the registered profile.
//!
//! The oracle runs against a real mounted host registry, not isolated type
//! fixtures: every lifecycle case opens a lease through the registered port and
//! dispatches through a linked dispatcher.

mod adversarial;
mod integrity;
mod lifecycle;
mod races;
mod registration;
mod support;

pub use support::{
    ComposeRegisteredToolHost, RegisteredToolHarness, RegisteredToolHostSpec, conformance_deadline,
    conformance_host_id, conformance_instance, conformance_scope, conformance_turn,
};

use crate::registered_tool_fixture::{FIXTURE_CLEANUP_BUDGET, ScriptedRegisteredToolDispatcher};
use std::sync::Arc;

/// Runs every Contract 063 counterexample against one composed host registry.
///
/// The `compose` closure mounts a linked registered-tool dispatcher and an
/// exact bounded cleanup budget into the host registry under test.
///
/// # Panics
///
/// Panics when any registration, lifecycle, or adversarial oracle fails.
pub fn assert_registered_tool_conformance(compose: &ComposeRegisteredToolHost) {
    let harness = RegisteredToolHarness::mount(
        compose,
        Arc::new(ScriptedRegisteredToolDispatcher::echoing()),
        FIXTURE_CLEANUP_BUDGET,
    );

    registration::snapshot_binds_one_kind_per_identity();
    registration::snapshot_rejects_oversized_schema();
    registration::snapshot_requires_a_declared_transport();
    registration::selection_rejects_unknown_and_duplicate_identities();
    registration::selection_rejects_undeclared_transport_and_version();
    registration::absent_port_is_typed_unavailable();
    registration::foreign_execution_host_fails_before_open(&harness);
    registration::ready_registry_reports_every_dimension(&harness);
    registration::schema_and_snapshot_hold_no_runtime_resource();

    lifecycle::open_binds_a_lease_without_a_listener(&harness);
    lifecycle::one_call_settles_exactly_once(compose);
    lifecycle::unsupported_tools_fail_before_dispatch(compose);
    lifecycle::oversized_argument_fails_before_dispatch(compose);
    lifecycle::oversized_result_is_rejected(compose);
    lifecycle::server_failure_stays_typed_and_correlated(compose);
    lifecycle::progress_admits_only_forward_sequences(compose);
    lifecycle::progress_queue_is_positively_bounded(compose);
    lifecycle::terminal_barrier_and_close_are_joined(compose);
    lifecycle::a_closed_lease_is_not_reusable(compose);

    adversarial::revocation_before_dispatch_never_reaches_the_dispatcher(compose);
    adversarial::revocation_before_delivery_reports_honest_execution(compose);
    adversarial::a_retained_sink_fails_after_the_call_settles(compose);
    adversarial::one_outstanding_call_per_lease(compose);
    adversarial::failed_cleanup_retains_ownership(compose);

    integrity::bindings_are_minted_only_by_the_kernel(compose);
    integrity::mounted_open_enforces_typed_readiness(compose);
    integrity::unqualified_carrier_never_opens(compose);
    integrity::expired_calls_never_dispatch(compose);
    integrity::a_call_that_outruns_its_deadline_is_unknown(compose);
    integrity::progress_past_the_deadline_is_refused(compose);
    integrity::progress_rechecks_live_revocation(compose);
    integrity::revocation_between_dispatch_and_progress_is_linearized(compose);

    races::concurrent_publishes_cannot_both_take_one_slot(compose);
    races::revocation_during_a_pending_verdict_blocks_every_commit(compose);
    races::concurrent_publishes_keep_exact_ordering(compose);
}
