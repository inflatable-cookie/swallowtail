use crate::headless_support;

use futures_executor::block_on;
use headless_support::{
    FakeProcessService, PendingTimeService, ScriptedProcessService, assert_redacted,
    assert_status_code, cancelled, completed, driver, fixture, host_services_for, plan_for,
    request_for, timed_out,
};
use std::sync::Arc;
use swallowtail_adapter_gemini::{
    GeminiCliPreparedDriver, GeminiCliPreparedIntegration, GeminiHeadlessModelSelection,
    GeminiHeadlessRunProfileInput, prepare_gemini_cli,
};
use swallowtail_core::{
    DriverRole, HarnessConfigurationPosture, HarnessIsolation, ModelId, ModelRouteId,
    ModelRouteRevision, ObservableActivityAvailability, OperationShape, OwnedRemoteResourceKind,
    ProviderId,
};
use swallowtail_runtime::{
    CleanupOutcome, Deadline, MonotonicInstant, OperationContent, ProcessExit, ProviderObservation,
    ProviderRetentionPolicy, RemoteResourceDeletionOutcome, RequestId, RuntimeEventKind,
    StructuredRunDriver, TerminalStatus, WorkingResourceRef,
};
use swallowtail_testkit::{
    ConformanceAssertion, ExecutionTopologyFixture, SyntheticProfile,
    assert_prepared_operation_evidence_matches_plan, run_one_shot_structured_cli_profile,
    run_structured_harness_native_boundary_assertions,
};

include!("headless_structured_run/production.rs");
include!("headless_structured_run/input_failures.rs");
include!("headless_structured_run/prepared.rs");
include!("headless_structured_run/retention.rs");
include!("headless_structured_run/profile.rs");
