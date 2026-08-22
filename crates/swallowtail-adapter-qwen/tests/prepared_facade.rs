#![allow(dead_code, unused_imports)]

mod support;

use futures_executor::block_on;
use std::sync::Arc;
use support::{FakeProcessService, PendingTimeService, ScriptedProcessService, host_services_for};
use swallowtail_adapter_qwen::{
    QWEN_CODE_AXIS, QwenCatalogueProfileInput, QwenHeadlessDriver, QwenModelSelection,
    QwenPreparationInput, QwenPreparationProbe, QwenRunProfileInput, QwenSessionProfileInput,
    prepare_qwen_catalogue, prepare_qwen_headless,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, ConfiguredInstanceId, CredentialMechanism,
    CredentialState, EndpointAudience, EndpointAuthorization, EntitlementMetering,
    EntitlementState, ExecutionHostId, ExtensionNamespace, HarnessConfigurationPosture,
    HarnessIsolation, InstalledExecutableCompatibility, InstanceRevision, InterfaceVersionAxis,
    ModelId, ModelRouteId, ModelRouteRevision, ObservableActivityAvailability, ProviderId,
    ReasoningMode, RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{
    CleanupOutcome, Deadline, DiscoveryCancellation, EnvironmentRef, ExecutableRef,
    InstalledExecutableTarget, MonotonicInstant, OperationContent, OperationPolicy,
    PreparedAccessEvidence, ProviderRetentionPolicy, RequestId, RuntimeTurnId, ScopeId,
    StructuredRunDriver, StructuredRunRequest, TerminalStatus, TurnRequest, WorkingResourceRef,
};
use swallowtail_testkit::assert_prepared_operation_evidence_matches_plan;

include!("prepared_facade/runs.rs");
include!("prepared_facade/sessions.rs");
include!("prepared_facade/catalogue.rs");
include!("prepared_facade/support.rs");
