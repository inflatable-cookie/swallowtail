#![allow(dead_code)]

#[path = "support/discovery.rs"]
mod discovery_support;
#[path = "support/headless.rs"]
mod headless_support;

use discovery_support::DiscoveryHost;
use futures_executor::block_on;
use futures_util::StreamExt;
use headless_support::{FIXTURE_CWD, FixtureHost};
use swallowtail_adapter_cline::{
    CLINE_EXECUTABLE_NAME, CLINE_LOCAL_ACCOUNT_AUDIENCE, CLINE_PACKAGE_AXIS, CLINE_PACKAGE_VERSION,
    ClineHeadlessPreparationInput, ClineHeadlessPreparationProbe, ClineHeadlessRunProfileInput,
    cline_local_account_access_profile, prepare_cline_headless,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, Capability, CapabilityConstraint,
    ConfiguredInstanceId, CredentialMechanism, CredentialState, EndpointAudience,
    EndpointAuthorization, EntitlementMetering, EntitlementState, ExecutionHostId, HarnessMode,
    InstanceRevision, InterfaceVersionAxis, RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{
    CleanupOutcome, Deadline, DiscoveryCancellation, EnvironmentRef, ExecutableRef,
    InstalledExecutableTarget, MonotonicInstant, OperationContent, PreparedAccessEvidence,
    RequestId, ScopeId, TerminalStatus, WorkingResourceRef,
};
use swallowtail_testkit::assert_prepared_operation_evidence_matches_plan;

const SUCCESS: &str = include_str!("fixtures/cline-headless-3.0.55/success.jsonl");

include!("prepared_headless_facade/default_run.rs");
include!("prepared_headless_facade/plan.rs");
include!("prepared_headless_facade/rejections.rs");
include!("prepared_headless_facade/support.rs");
