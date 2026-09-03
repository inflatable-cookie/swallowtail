use crate::http_support;

use futures_executor::block_on;
use futures_util::StreamExt;
use http_support::{FixtureServer, StreamFixture, ThreadServices};
use std::sync::Arc;
use std::time::Duration;
use swallowtail_adapter_opencode::{
    OpenCodeHttpDriver, opencode_http_descriptor, opencode_server_binding,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, Capability, CapabilityProfile,
    CapabilityRequirement, ConfiguredInstance, ConfiguredInstanceId, CredentialMechanism,
    CredentialState, DriverRole, EndpointAudience, EndpointAuthorization, EntitlementMetering,
    EntitlementState, ExecutionHostId, ExecutionLayer, ExtensionNamespace, InstanceOwnership,
    InstancePolicyId, InstanceRevision, InstanceTargetRef, ModelId, ModelRoute, ModelRouteId,
    ModelRouteRevision, OperationRequirements, OperationShape, PreflightContext, PreflightPlan,
    ProtocolFacadeId, ProviderId, RuntimeReadiness, SessionAccessPolicy,
    SessionProviderStatePolicy, SupportAuthority, preflight,
};
use swallowtail_host_local::{LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    BlockingWorkService, CredentialRef, CredentialService, EndpointRef, HostServices,
    InteractiveSessionDriver, LoadSessionRequest, ModelCatalogDriver, ModelCatalogRequest,
    NetworkPolicyService, OpenSessionRequest, OperationContent, OperationDetachmentAcknowledgement,
    PersistedSessionResumeBinding, ProviderObservation, ProviderSessionHistoryAgreement,
    ProviderSessionHistoryBounds, ProviderSessionHistoryDriver, ProviderSessionHistoryId,
    ProviderSessionHistoryPlan, ProviderSessionHistoryRequest, ProviderSessionHistoryTotal,
    ProviderSessionReconciliationAgreement, ProviderSessionReconciliationBounds,
    ProviderSessionReconciliationDriver, ProviderSessionReconciliationPlan,
    ProviderSessionReconciliationRequest, RequestId, ResumeSessionRequest, RuntimeEventKind,
    RuntimeTurnId, ScopedTaskService, SessionCleanupRequest, SessionPlanAgreement,
    SessionResumeBinding, SessionResumeBindingPersistenceFailureKind, TerminalStatus, TimeService,
    TurnRequest, WorkingResourceRef, WorkingResourceService,
};

fn close_session(
    session: Box<dyn swallowtail_runtime::InteractiveSessionHandle>,
    fixture: &Fixture,
) -> swallowtail_runtime::BoxFuture<'static, swallowtail_runtime::CleanupOutcome> {
    session.close(
        SessionCleanupRequest::new(fixture.thread.deadline_after(Duration::from_secs(1))),
        fixture.services(),
    )
}

fn open_session_request(id: impl Into<String>, resource: WorkingResourceRef) -> OpenSessionRequest {
    OpenSessionRequest::new(
        RequestId::new(id).expect("request id is valid"),
        resource,
        None,
        SessionPlanAgreement::explicit(
            SessionAccessPolicy::ambient_harness(swallowtail_core::ResourceAccess::Read),
            Some(SessionProviderStatePolicy::Prohibited),
            None,
        ),
    )
}

fn open_detachable_session_request(
    id: impl Into<String>,
    resource: WorkingResourceRef,
) -> OpenSessionRequest {
    OpenSessionRequest::new(
        RequestId::new(id).expect("request id is valid"),
        resource,
        None,
        SessionPlanAgreement::explicit(
            SessionAccessPolicy::ambient_harness(swallowtail_core::ResourceAccess::Read),
            Some(SessionProviderStatePolicy::DurableProviderSessionPreserved),
            None,
        ),
    )
}

include!("http_driver/success.rs");
include!("http_driver/lifecycle.rs");
include!("http_driver/failures.rs");
include!("http_driver/fixture.rs");
include!("http_driver/version_range.rs");
include!("http_driver/continuity.rs");
include!("http_driver/reconciliation.rs");
include!("http_driver/history.rs");
include!("http_driver/fixture_join.rs");
