//! Deterministic conformance support for Swallowtail adapters.
//!
//! Fixtures and assertions operate on Swallowtail public contracts. This crate
//! contains no execution, transport, or provider behavior.

#![forbid(unsafe_code)]

mod assertions;
mod callback_fixture;
mod direct_continuation_fixture;
mod fixture;
mod interface_compatibility_assertions;
mod managed_harness_fixture;
mod preflight_assertions;
mod preflight_fixture;
mod profile_acp;
mod profile_attached;
mod profile_attached_runtime;
mod profile_common;
mod profile_direct_session;
mod profile_fixture;
mod profile_harness_native;
mod profile_harness_rpc_contract;
mod profile_hosted;
mod profile_local_continuation;
mod profile_managed_harness;
mod profile_managed_harness_shape;
mod profile_network_harness;
mod profile_one_shot;
mod profile_owned;
mod profile_persistent_acp;
mod profile_persistent_acp_shape;
mod profile_provider_conversation;
mod profile_realtime_media;
mod profile_realtime_media_shape;
mod profile_realtime_rollover;
mod profile_rpc;
mod profile_session_access;
mod profile_shape;
mod profiles;
mod provider_conversation_fixture;
mod realtime_media_fixture;
mod recording_host;
mod runtime_assertions;
mod session_access_fixture;
mod topology_fixture;

pub use assertions::{
    assert_capability_rejection, assert_contract_kernel, assert_diagnostic_redaction,
    assert_extension_isolation, assert_extension_policies, assert_reference_opacity,
};
pub use callback_fixture::{CallbackExchangeFixture, successful_callback_response};
pub use fixture::ContractKernelFixture;
pub use interface_compatibility_assertions::{
    ClosedSemanticWindowCase, assert_closed_semantic_compatibility_window,
};
pub use managed_harness_fixture::{ManagedHarnessPreflightCase, ManagedHarnessPreflightFixture};
pub use preflight_assertions::{
    assert_changed_revision_invalidates_plan, assert_preflight_rejection_without_side_effects,
    assert_successful_preflight_binding,
};
pub use preflight_fixture::{PreflightFixtureCase, RuntimePreflightFixture};
pub(crate) use profile_common::assert_common_contract;
pub(crate) use profile_fixture::ProfilePreflightFixture;
pub use profile_provider_conversation::run_provider_conversation_boundary_assertions;
pub use profiles::{
    ConformanceAssertion, ConformanceReport, SyntheticProfile, run_all_synthetic_profiles,
    run_attached_network_harness_profile, run_attached_runtime_boundary_assertions,
    run_attached_self_hosted_profile, run_connection_scoped_direct_session_profile,
    run_harness_rpc_contract_assertions, run_hosted_direct_api_profile,
    run_locally_continued_direct_session_profile, run_long_lived_acp_profile,
    run_long_lived_rpc_profile, run_one_shot_structured_cli_profile, run_owned_self_hosted_profile,
    run_persistent_acp_profile, run_provider_managed_harness_profile,
    run_realtime_media_direct_session_profile, run_realtime_rollover_boundary_assertions,
    run_structured_harness_native_boundary_assertions,
};
pub use provider_conversation_fixture::{
    ProviderConversationPreflightCase, ProviderConversationPreflightFixture,
};
pub use realtime_media_fixture::{RealtimeMediaPreflightCase, RealtimeMediaPreflightFixture};
pub use recording_host::{
    RecordedHostCall, RecordingHostServices, RecordingOutcome, poll_immediate,
};
pub use runtime_assertions::{
    assert_cleanup_states_remain_distinct, assert_dynamic_role_registration_and_calls,
    assert_missing_roles_are_explicit,
};
pub use session_access_fixture::{SessionAccessFixtureCase, SessionAccessPreflightFixture};
pub use topology_fixture::ExecutionTopologyFixture;
