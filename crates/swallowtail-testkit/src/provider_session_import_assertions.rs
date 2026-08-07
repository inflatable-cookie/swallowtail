use crate::{
    ProviderSessionImportFixture, RecordingHostServices, RecordingOutcome, poll_immediate,
    provider_session_catalogue_bounds,
};
use std::sync::{Arc, Mutex};
use swallowtail_core::{
    CancellationScope, ProviderSessionActivityState, ProviderSessionBindingOrigin,
    ProviderSessionImportAvailability, ProviderSessionImportUnavailableReason, SafeDiagnostic,
    SessionRef,
};
use swallowtail_runtime::{
    BoxFuture, CancellationControl, CleanupOutcome, HostServices, ImmediateCancellation,
    InteractiveSessionDriver, InteractiveSessionHandle, LoadSessionRequest, LoadedSession,
    OpenSessionRequest, OperationContent, ProviderSessionCatalogueOutcome,
    ProviderSessionCatalogueRequest, ProviderSessionImportOutcome, ProviderSessionImportRequest,
    ProviderSessionImportRevalidation, ProviderSessionOperationFailure,
    ProviderSessionOperationFailureStage, RequestId, ResumeSessionRequest, RuntimeFailure,
    RuntimeSessionId, SessionReplayItem, SessionReplayKind, TurnHandle, TurnRequest,
    validate_provider_session_catalogue_execution, validate_provider_session_import_execution,
};

/// Runs the provider-neutral catalogue/import conformance pack.
pub fn assert_provider_session_import_contract() {
    assert_topology_and_prepared_evidence();
    assert_bounds_pagination_and_redaction();
    assert_drift_and_stale_targets_fail_closed();
    assert_lifecycle_failures_remain_distinct();
    assert_import_load_and_resume_sequence();
}

include!("provider_session_import_assertions/support.rs");
include!("provider_session_import_assertions/topology.rs");
include!("provider_session_import_assertions/bounds.rs");
include!("provider_session_import_assertions/drift.rs");
include!("provider_session_import_assertions/lifecycle.rs");
include!("provider_session_import_assertions/sequence.rs");
include!("provider_session_import_assertions/fixture.rs");
