use crate::failure::{failure, unsupported};
use crate::protocol::{
    Event, Request, abort, parse_catalog, parse_event, parse_health, parse_session, prompt,
    require_abort_success, require_no_content, session_create,
};
use crate::transport::{CurlTransport, Subscription};
use std::future::{Future, poll_fn};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::Poll;
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, CancellationScope, CredentialMechanism,
    DriverDescriptor, DriverRole, ExecutionLayer, HostServiceKind, IntegrationFamilyId,
    ModelCatalogEntry, OperationShape, PreflightPlan, ProviderId, ResourceAccess,
    ResourceRepresentation, SafeDiagnostic, SessionAccessPolicy, SessionRef, TransportFamilyId,
};
use swallowtail_runtime::{
    BoxEventStream, BoxFuture, CancellationAcknowledgement, CancellationControl, CleanupOutcome,
    CredentialLease, Deadline, DeadlineObservation, EndpointRef, HostServices,
    InteractiveSessionDriver, InteractiveSessionHandle, JoinedTask, ModelCatalogDriver,
    ModelCatalogRequest, OpenSessionRequest, RequestId, ResourceLease, ResumeSessionRequest,
    RuntimeEvent, RuntimeEventKind, RuntimeFailure, RuntimeSessionId, RuntimeTurnId, ScopeId,
    SessionResumeBinding, TerminalOutcome, TerminalStatus, TurnHandle, TurnRequest,
    runtime_event_channel, terminal_outcome_channel, validate_session_access_plan,
    validate_session_resource_lease,
};

const DRIVER_ID: &str = "swallowtail.opencode.http";
const EVENT_CAPACITY: usize = 64;

#[derive(Clone, Default)]
pub struct OpenCodeHttpDriver {
    transport: CurlTransport,
}

impl OpenCodeHttpDriver {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    fn validate_plan(plan: &PreflightPlan) -> Result<(), RuntimeFailure> {
        if plan.driver_identity().id().as_str() != DRIVER_ID {
            return Err(failure(
                "swallowtail.opencode.plan_driver_mismatch",
                "Preflight plan is bound to a different driver",
            ));
        }
        if plan.credential_reference().is_none() {
            return Err(failure(
                "swallowtail.opencode.credential_reference_missing",
                "OpenCode HTTP requires a delegated credential reference",
            ));
        }
        if matches!(
            plan.credential_mechanism(),
            CredentialMechanism::ApiKey
                | CredentialMechanism::AutomationToken
                | CredentialMechanism::WorkloadIdentity
                | CredentialMechanism::CloudProviderIdentity
                | CredentialMechanism::LocalUnauthenticated
        ) {
            return Err(failure(
                "swallowtail.opencode.credential_mechanism_rejected",
                "OpenCode HTTP requires delegated harness authentication",
            ));
        }
        Ok(())
    }
}

#[must_use]
pub fn opencode_http_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(DRIVER_ID).expect("static adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("opencode").expect("static family id is valid"),
        TransportFamilyId::new("http-sse").expect("static transport id is valid"),
    )
    .with_roles([DriverRole::ModelCatalog, DriverRole::InteractiveSession])
    .with_execution_layers([ExecutionLayer::HarnessInteraction])
    .with_operation_shapes([OperationShape::InteractiveSession])
    .with_required_host_services(
        DriverRole::ModelCatalog,
        [
            HostServiceKind::BlockingWork,
            HostServiceKind::Time,
            HostServiceKind::Network,
            HostServiceKind::Credential,
        ],
    )
    .with_required_host_services(
        DriverRole::InteractiveSession,
        [
            HostServiceKind::Task,
            HostServiceKind::BlockingWork,
            HostServiceKind::Time,
            HostServiceKind::Network,
            HostServiceKind::Credential,
            HostServiceKind::WorkingResource,
        ],
    )
}

include!("driver/roles.rs");
include!("driver/access.rs");
include!("driver/session_state.rs");
include!("driver/cancellation.rs");
include!("driver/session.rs");
include!("driver/turn.rs");
include!("driver/lifecycle.rs");
include!("driver/tests.rs");
