#![allow(dead_code, unused_imports)]

mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use support::{FixtureHost, Scenario, close_session};
use swallowtail_adapter_oh_my_pi::{
    OH_MY_PI_PACKAGE_AXIS, OhMyPiCatalogueProfileInput, OhMyPiModelSelection,
    OhMyPiPreparationInput, OhMyPiPreparationProbe, OhMyPiRunProfileInput,
    OhMyPiSessionProfileInput, prepare_oh_my_pi_rpc,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, Capability, ConfiguredInstanceId,
    CredentialMechanism, CredentialState, EndpointAudience, EndpointAuthorization,
    EntitlementMetering, EntitlementState, ExecutionHostId, HarnessConfigurationPosture,
    HarnessIsolation, InstalledExecutableCompatibility, InstanceRevision, InterfaceVersionAxis,
    ModelId, ModelRouteId, ModelRouteRevision, ObservableActivityAvailability, ProviderId,
    ReasoningMode, ResourceAccess, RuntimeReadiness, SessionAccessPolicy, SupportAuthority,
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

include!("consumer_route_projection/mod.rs");

fn prepared() -> swallowtail_adapter_oh_my_pi::OhMyPiPreparedIntegration {
    let host_id = ExecutionHostId::new("oh-my-pi.projection.host").expect("host");
    let discovery = FixtureHost::version_probe("17.2.9");
    block_on(prepare_oh_my_pi_rpc(
        preparation_input(host_id.clone()),
        probe(),
        discovery.services(host_id),
    ))
    .expect("Oh My Pi prepares")
}
