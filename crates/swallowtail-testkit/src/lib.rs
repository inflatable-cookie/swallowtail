//! Deterministic conformance support for Swallowtail adapters.
//!
//! Fixtures and assertions operate on Swallowtail public contracts. This crate
//! contains no execution, transport, or provider behavior.

#![forbid(unsafe_code)]

mod assertions;
mod callback_fixture;
mod fixture;
mod preflight_assertions;
mod preflight_fixture;
mod profile_attached;
mod profile_common;
mod profile_fixture;
mod profile_hosted;
mod profile_one_shot;
mod profile_owned;
mod profile_rpc;
mod profile_shape;
mod profiles;
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
pub use preflight_assertions::{
    assert_changed_revision_invalidates_plan, assert_preflight_rejection_without_side_effects,
    assert_successful_preflight_binding,
};
pub use preflight_fixture::{PreflightFixtureCase, RuntimePreflightFixture};
pub(crate) use profile_common::assert_common_contract;
pub(crate) use profile_fixture::ProfilePreflightFixture;
pub use profiles::{
    ConformanceAssertion, ConformanceReport, SyntheticProfile, run_all_synthetic_profiles,
    run_attached_self_hosted_profile, run_hosted_direct_api_profile, run_long_lived_rpc_profile,
    run_one_shot_structured_cli_profile, run_owned_self_hosted_profile,
};
pub use recording_host::{
    RecordedHostCall, RecordingHostServices, RecordingOutcome, poll_immediate,
};
pub use runtime_assertions::{
    assert_cleanup_states_remain_distinct, assert_dynamic_role_registration_and_calls,
    assert_missing_roles_are_explicit,
};
pub use session_access_fixture::{SessionAccessFixtureCase, SessionAccessPreflightFixture};
pub use topology_fixture::ExecutionTopologyFixture;
