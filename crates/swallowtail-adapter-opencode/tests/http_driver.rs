mod http_support;

use futures_executor::block_on;
use futures_util::StreamExt;
use http_support::{FixtureServer, StreamFixture, ThreadServices};
use std::sync::Arc;
use std::time::Duration;
use swallowtail_adapter_opencode::{OpenCodeHttpDriver, opencode_http_descriptor};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, Capability, CapabilityProfile,
    CapabilityRequirement, ConfiguredInstance, ConfiguredInstanceId, CredentialMechanism,
    CredentialState, DriverRole, EndpointAudience, EndpointAuthorization, EntitlementMetering,
    EntitlementState, ExecutionHostId, ExecutionLayer, ExtensionNamespace, InstanceOwnership,
    InstancePolicyId, InstanceRevision, InstanceTargetRef, ModelId, ModelRoute, ModelRouteId,
    ModelRouteRevision, OperationRequirements, OperationShape, PreflightContext, PreflightPlan,
    ProtocolFacadeId, ProviderId, RuntimeReadiness, SessionAccessPolicy, SupportAuthority,
    preflight,
};
use swallowtail_host_local::{LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    BlockingWorkService, CredentialRef, CredentialService, EndpointRef, HostServices,
    InteractiveSessionDriver, ModelCatalogDriver, ModelCatalogRequest, NetworkPolicyService,
    OpenSessionRequest, OperationContent, RequestId, RuntimeEventKind, RuntimeTurnId,
    ScopedTaskService, TerminalStatus, TimeService, TurnRequest, WorkingResourceRef,
    WorkingResourceService,
};

include!("http_driver/success.rs");
include!("http_driver/lifecycle.rs");
include!("http_driver/failures.rs");
include!("http_driver/fixture.rs");
