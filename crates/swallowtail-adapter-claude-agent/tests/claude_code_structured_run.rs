mod claude_code_support;

use claude_code_support::{
    ControllableTimeService, FailingTaskService, FakeProcessService, ImmediateTimeService,
    PendingTimeService, TaskState, ThreadTaskService, fixture, host_services, local_watcher_host,
    preparation_input, preparation_probe, watcher_host_services,
};
use futures_executor::block_on;
use futures_util::{FutureExt, StreamExt};
use std::sync::Arc;
use swallowtail_adapter_claude_agent::{
    ClaudeCodeMaximumTurns, ClaudeCodeModelSelection, ClaudeCodePreparedIntegration,
    ClaudeCodePreparedRun, ClaudeCodeRunProfileInput, prepare_claude_code_headless,
};
use swallowtail_core::{
    ActivityKindClass, Capability, CapabilityConstraint, HarnessConfigurationPosture,
    HarnessIsolation, HarnessMode, ModelId, ModelRouteId, ModelRouteRevision,
    ObservableActivityAvailability, ReasoningMode,
};
use swallowtail_runtime::{
    ActivityKind, ActivityLifecyclePhase, CancellationAcknowledgement, CleanupOutcome, Deadline,
    HostServices, MonotonicInstant, OperationContent, ProcessExit, ProviderObservation,
    ProviderRetentionPolicy, RequestId, RuntimeEvent, RuntimeEventKind, StructuredRunDriver,
    TerminalOutcome, TerminalStatus, WorkingResourceRef,
};
use swallowtail_testkit::{
    ConformanceAssertion, ExecutionTopologyFixture, SyntheticProfile,
    assert_prepared_operation_evidence_matches_plan, run_one_shot_structured_cli_profile,
    run_structured_harness_native_boundary_assertions,
};

include!("claude_code_structured_run/execution_cases.rs");
include!("claude_code_structured_run/failure_cases.rs");
include!("claude_code_structured_run/control_cases.rs");
include!("claude_code_structured_run/maximum_turns_cases.rs");
include!("claude_code_structured_run/maximum_turns_rejection_cases.rs");
include!("claude_code_structured_run/profile_case.rs");
include!("claude_code_structured_run/watcher_cases.rs");
include!("claude_code_structured_run/watcher_lifecycle.rs");
include!("claude_code_structured_run/watcher_deadline.rs");
include!("claude_code_structured_run/watcher_proof.rs");
include!("claude_code_structured_run/watcher_stop_reentry.rs");
include!("claude_code_structured_run/watcher_start_failure.rs");
include!("claude_code_structured_run/support.rs");
