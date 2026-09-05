#![allow(dead_code, unused_imports)]

mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use support::{FixtureHost, Scenario, close_session};
use swallowtail_adapter_pi::{
    PI_PACKAGE_AXIS, PiCatalogueProfileInput, PiModelSelection, PiPreparationInput,
    PiPreparationProbe, PiRunProfileInput, PiSessionProfileInput, prepare_pi_rpc,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, Capability, ConfiguredInstanceId,
    CredentialMechanism, CredentialRef, CredentialState, EndpointAudience, EndpointAuthorization,
    EntitlementMetering, EntitlementState, ExecutionHostId, ExtensionNamespace,
    HarnessConfigurationPosture, HarnessIsolation, InstalledExecutableCompatibility,
    InstanceRevision, InterfaceVersionAxis, ModelId, ModelRouteId, ModelRouteRevision,
    ObservableActivityAvailability, ProviderId, ResourceAccess, RuntimeReadiness,
    SessionAccessPolicy, SupportAuthority,
};
use swallowtail_runtime::{
    AttachmentDescriptor, AttachmentRef, AttachmentRole, CleanupOutcome, Deadline,
    DiscoveryCancellation, EnvironmentRef, ExecutableRef, InstalledExecutableTarget,
    MonotonicInstant, OperationContent, PreparedAccessEvidence, ProviderRetentionPolicy, RequestId,
    RuntimeTurnId, ScopeId, SessionOptions, TerminalStatus, TurnRequest, WorkingResourceRef,
};
use swallowtail_testkit::assert_prepared_operation_evidence_matches_plan;

include!("prepared_facade/catalogue.rs");
include!("prepared_facade/operations.rs");
include!("prepared_facade/support.rs");

#[path = "prepared_facade/consumer_route_projection.rs"]
mod consumer_route_projection;
