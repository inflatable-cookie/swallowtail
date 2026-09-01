#![allow(dead_code)]

#[path = "support/discovery.rs"]
mod discovery_support;
#[path = "support/headless.rs"]
mod headless_support;
mod support;

use discovery_support::DiscoveryHost;
use futures_executor::block_on;
use std::collections::BTreeSet;
use support::{FixtureHost, Scenario};
use swallowtail_adapter_cline::{
    CLINE_EXECUTABLE_NAME, CLINE_PACKAGE_AXIS, CLINE_PACKAGE_VERSION,
    ClineHeadlessPreparationInput, ClineHeadlessPreparationProbe, ClineHeadlessRunProfileInput,
    ClinePreparationInput, ClinePreparationProbe, ClinePreparedSession, ClineProjectionOpenFailure,
    ClineSessionProfileInput, cline_local_account_access_profile, prepare_cline_acp,
    prepare_cline_headless,
};
use swallowtail_core::{
    AccessProfileId, AccessStatus, ConfiguredInstanceId, CredentialState, EndpointAuthorization,
    EntitlementState, ExecutionHostId, HarnessMode, InstanceRevision, InterfaceVersionAxis,
    RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{
    CleanupOutcome, ConsumerRouteProjectionContribution, ConsumerRouteProjectionSourceId,
    ConsumerRouteProjectionSourceKind, ConsumerRouteRowIdentity, Deadline, DiscoveryCancellation,
    EnvironmentRef, ExecutableRef, InstalledExecutableTarget, MonotonicInstant, OperationContent,
    PreparationFailure, PreparedAccessEvidence, RequestId, ScopeId, WorkingResourceRef,
};

const HEADLESS_SUCCESS: &str = include_str!("fixtures/cline-headless-3.0.55/success.jsonl");

include!("consumer_route_projection/cases.rs");
include!("consumer_route_projection/ledger.rs");
include!("consumer_route_projection/mixture.rs");
include!("consumer_route_projection/headless_mixture.rs");
include!("consumer_route_projection/fixtures.rs");
include!("consumer_route_projection/posture.rs");
