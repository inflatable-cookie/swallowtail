use crate::failure::{failure, unsupported};
use crate::protocol::{
    Event, PromptPayload, Request, Response, SessionDeleteResponse, abort, classify_session_delete,
    parse_catalog, parse_event, parse_session_for_version, prompt, require_abort_success,
    require_health_matches, require_no_content, session_create, session_delete,
};
use crate::selection::{OpenCodePlanVersion, classify_plan, opencode_http_claim};
use crate::transport::{CurlTransport, Subscription};
use std::collections::BTreeSet;
use std::future::{Future, poll_fn};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::Poll;
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, CancellationScope, Capability,
    CapabilityConstraint, CredentialMechanism, DriverDescriptor, DriverRole, ExecutionLayer,
    HarnessConfigurationPosture, HarnessIsolation, HostServiceKind, InstanceOwnership,
    IntegrationFamilyId, ModelCatalogEntry, OperationShape, OwnedRemoteResourceKind, PreflightPlan,
    ProviderId, ResourceAccess, ResourceRepresentation, RunRef, SafeDiagnostic,
    SessionAccessPolicy, SessionRef, StructuredOutputEnforcement, TransportFamilyId,
};
use swallowtail_runtime::{
    BoxEventStream, BoxFuture, CancellationAcknowledgement, CancellationControl, CleanupOutcome,
    CredentialLease, Deadline, DeadlineObservation, EndpointRef, HostServices,
    InteractiveSessionDriver, InteractiveSessionHandle, JoinedTask, ModelCatalogDriver,
    ModelCatalogRequest, OpenSessionRequest, ProviderExecutionPolicy, ProviderRecoveryPolicy,
    ProviderRetentionPolicy, RemoteResourceDeletionOutcome, RequestId, ResourceLease,
    ResumeSessionRequest, RunHandle, RuntimeEvent, RuntimeEventKind, RuntimeFailure, RuntimeRunId,
    RuntimeSessionId, RuntimeTurnId, ScopeId, SessionResumeBinding, StreamReattachmentPolicy,
    StructuredOutputDescriptor, StructuredRunDriver, StructuredRunRequest, TerminalOutcome,
    TerminalStatus, TurnHandle, TurnRequest, runtime_event_channel, terminal_outcome_channel,
    validate_session_resource_lease,
};

const DRIVER_ID: &str = "swallowtail.opencode.http";
const EVENT_CAPACITY: usize = 64;

pub(crate) mod callback;
pub(crate) mod input;

#[derive(Clone, Default)]
pub struct OpenCodeHttpDriver {
    transport: CurlTransport,
}

impl OpenCodeHttpDriver {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    fn validate_plan(plan: &PreflightPlan) -> Result<OpenCodePlanVersion, RuntimeFailure> {
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
        classify_plan(plan)
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
    .with_roles([
        DriverRole::ModelCatalog,
        DriverRole::InteractiveSession,
        DriverRole::StructuredRun,
        DriverRole::ProviderSessionManagement,
    ])
    .with_interface_compatibility(opencode_http_claim())
    .with_execution_layers([ExecutionLayer::HarnessInteraction])
    .with_operation_shapes([
        OperationShape::InteractiveSession,
        OperationShape::StructuredRun,
        OperationShape::ProviderSessionManagement,
    ])
    .with_extension_namespaces([
        callback::permission_namespace(),
        callback::question_namespace(),
    ])
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
    .with_required_host_services(
        DriverRole::ProviderSessionManagement,
        [
            HostServiceKind::Task,
            HostServiceKind::BlockingWork,
            HostServiceKind::Time,
            HostServiceKind::Network,
            HostServiceKind::Credential,
            HostServiceKind::WorkingResource,
        ],
    )
    .with_required_host_services(
        DriverRole::StructuredRun,
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
include!("driver/run.rs");
include!("driver/lifecycle.rs");
include!("driver/session_management.rs");
include!("driver/tests.rs");
