mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use std::sync::Arc;
use swallowtail_adapter_cursor::{
    CURSOR_AGENT_RELEASE_AXIS, CursorAcpSessionProfileInput, CursorCatalogueProfileInput,
    CursorHeadlessContext, CursorHeadlessFast, CursorHeadlessModelSelection,
    CursorHeadlessReadMode, CursorHeadlessRunProfileInput, CursorPreparationInput,
    CursorPreparationProbe, CursorPreparedDriver, CursorPreparedIntegration,
    cursor_subscription_access_profile, prepare_cursor,
};
use swallowtail_core::{
    AccessProfileId, AccessStatus, CredentialState, EndpointAuthorization, EntitlementState,
    ExecutionHostId, InterfaceVersionAxis, ModelId, ModelRouteId, ModelRouteRevision, ProviderId,
    ReasoningMode, ResourceAccess, ResourceRepresentation, RuntimeReadiness, SessionRef,
    SupportAuthority,
};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, Deadline, DiscoveryCancellation, EnvironmentRef, ExecutableRef,
    HostServices, InstalledExecutableTarget, MaterializedResourceRef, MonotonicInstant,
    OperationContent, PreparationFailure, PreparedAccessEvidence, ProcessOutputChunk,
    ProcessOutputStream, RequestId, ResourceLease, RuntimeFailure, RuntimeTurnId, ScopeId,
    SessionResumeBinding, TerminalStatus, WorkingResourceIoService, WorkingResourceReadRequest,
    WorkingResourceRef, WorkingResourceService, WorkingResourceText, WorkingResourceWriteRequest,
    WorkingStateRestorationMethod,
};

const VERSION: &str = "2026.07.01-41b2de7\n";
const CATALOGUE: &str =
    "Available models\n\nauto - Auto (current, default)\nfixture-model - Fixture Model\n";
const HEADLESS: &str =
    include_str!("fixtures/cursor-agent-2026.07.01-41b2de7/headless-success.jsonl");
const QUALIFIED_RELEASES: [&str; 4] = [
    "2026.07.01-41b2de7",
    "2026.07.23-e383d2b",
    "2026.08.04-aaa8809",
    "2026.08.11-e8db854",
];

include!("prepared_suite/plans.rs");
include!("prepared_suite/model_parameters.rs");
include!("prepared_suite/rejections.rs");
include!("prepared_suite/support.rs");
