use crate::failure::{failure, unsupported};
use crate::protocol::{
    Event, OpenCodeSessionObservation, OpenCodeSessionStatus, PromptPayload, Request, Response,
    SessionDeleteResponse, abort, classify_session_delete, parse_catalog, parse_event,
    parse_session_for_version, parse_session_list, parse_session_lookup, parse_session_statuses,
    project_session_messages, prompt, require_abort_success, require_existing_session,
    require_health_matches, require_no_content, session_create, session_delete, session_get,
    session_list, session_messages, session_status,
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
    IntegrationFamilyId, InterfaceCompatibilityAssessment, ModelCatalogEntry, OperationShape,
    OwnedRemoteResourceKind, PreflightPlan, ProviderId, ProviderSessionActivityState,
    ProviderSessionDisplayContent, ProviderSessionImportAvailability,
    ProviderSessionImportUnavailableReason, ResourceAccess, ResourceRepresentation, RunRef,
    SafeDiagnostic, SessionAccessPolicy, SessionProviderStatePolicy, SessionRef,
    StructuredOutputEnforcement, TransportFamilyId,
};
use swallowtail_runtime::{
    BoxEventStream, BoxFuture, CancellationAcknowledgement, CancellationControl, CleanupOutcome,
    CredentialLease, Deadline, DeadlineObservation, DebugObservationKind, EndpointRef,
    HostServices, InteractiveSessionDriver, InteractiveSessionHandle, JoinedTask,
    LoadSessionRequest, LoadedSession, ModelCatalogDriver, ModelCatalogRequest, OpenSessionRequest,
    OperationDetachmentAcknowledgement, OperationDetachmentControl, ProviderExecutionPolicy,
    ProviderRecoveryPolicy, ProviderRetentionPolicy, ProviderSessionCandidate,
    ProviderSessionCandidateId, ProviderSessionCatalogueDriver, ProviderSessionCatalogueOutcome,
    ProviderSessionCataloguePlan, ProviderSessionCatalogueRequest, ProviderSessionHistoryDriver,
    ProviderSessionHistoryPage, ProviderSessionHistoryPlan, ProviderSessionHistoryRequest,
    ProviderSessionHistoryTotal, ProviderSessionImportDriver, ProviderSessionImportOutcome,
    ProviderSessionImportPlan, ProviderSessionImportRequest, ProviderSessionImportRevalidation,
    ProviderSessionOperationFailure, ProviderSessionOperationFailureStage,
    ProviderSessionReconciliationDriver, ProviderSessionReconciliationOutcome,
    ProviderSessionReconciliationPlan, ProviderSessionReconciliationRequest,
    RemoteResourceDeletionOutcome, RequestId, ResourceLease, ResumeSessionRequest, RunHandle,
    RuntimeEvent, RuntimeEventKind, RuntimeFailure, RuntimeRunId, RuntimeSessionId, RuntimeTurnId,
    ScopeId, SessionResumeBinding, StreamReattachmentPolicy, StructuredOutputDescriptor,
    StructuredRunDriver, StructuredRunRequest, TerminalOutcome, TerminalStatus, TurnHandle,
    TurnRequest, page_provider_session_history_window, runtime_event_channel,
    terminal_outcome_channel, validate_provider_session_catalogue_execution,
    validate_provider_session_history_execution, validate_provider_session_import_execution,
    validate_provider_session_reconciliation_execution, validate_session_resource_lease,
};

const DRIVER_ID: &str = "swallowtail.opencode.http";
const ROUTE: &str = "opencode.http";
const EVENT_CAPACITY: usize = 64;
const CONTINUITY_PAGE_LIMIT: usize = 100;
const CONTINUITY_MAXIMUM_PAGES: usize = 64;
pub(crate) const CONTINUITY_MAXIMUM_ITEMS: usize = 4096;
pub(crate) const CONTINUITY_MAXIMUM_BYTES: usize = 4 * 1024 * 1024;

pub(crate) mod callback;
pub(crate) mod input;

#[derive(Clone, Default)]
/// Low-level driver for one externally managed OpenCode HTTP server.
pub struct OpenCodeHttpDriver {
    transport: CurlTransport,
}

impl OpenCodeHttpDriver {
    /// Creates a driver using the adapter's bounded HTTP transport.
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

/// Describes the attached OpenCode server's separately authorized roles.
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
        DriverRole::ProviderSessionCatalogue,
        DriverRole::ProviderSessionImport,
        DriverRole::ProviderSessionReconciliation,
        DriverRole::ProviderSessionHistory,
    ])
    .with_interface_compatibility(opencode_http_claim())
    .with_execution_layers([ExecutionLayer::HarnessInteraction])
    .with_operation_shapes([
        OperationShape::InteractiveSession,
        OperationShape::StructuredRun,
        OperationShape::ProviderSessionManagement,
        OperationShape::ProviderSessionCatalogue,
        OperationShape::ProviderSessionImport,
        OperationShape::ProviderSessionReconciliation,
        OperationShape::ProviderSessionHistory,
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
        DriverRole::ProviderSessionCatalogue,
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
        DriverRole::ProviderSessionImport,
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
        DriverRole::ProviderSessionReconciliation,
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
        DriverRole::ProviderSessionHistory,
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
include!("driver/lifecycle_services.rs");
include!("driver/session_management.rs");
include!("driver/provider_session_import.rs");
include!("driver/provider_session_reconciliation.rs");
include!("driver/provider_session_history.rs");
include!("driver/tests.rs");
