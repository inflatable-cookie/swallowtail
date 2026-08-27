mod claude_code_support;

use claude_code_support::{
    FakeProcessService, ImmediateTimeService, PendingTimeService, fixture, host_services,
    preparation_input, preparation_probe,
};
use futures_executor::block_on;
use futures_util::StreamExt;
use std::sync::Arc;
use swallowtail_adapter_claude_agent::{
    ClaudeCodeMaximumTurns, ClaudeCodeModelSelection, ClaudeCodePreparedIntegration,
    ClaudeCodePreparedRun, ClaudeCodeRunProfileInput, prepare_claude_code_headless,
};
use swallowtail_core::{
    Capability, CapabilityConstraint, HarnessConfigurationPosture, HarnessIsolation, HarnessMode,
    ModelId, ModelRouteId, ModelRouteRevision, ObservableActivityAvailability, ReasoningMode,
};
use swallowtail_runtime::{
    CancellationAcknowledgement, CleanupOutcome, Deadline, MonotonicInstant, OperationContent,
    ProcessExit, ProviderObservation, ProviderRetentionPolicy, RequestId, RuntimeEvent,
    RuntimeEventKind, StructuredRunDriver, TerminalOutcome, TerminalStatus, WorkingResourceRef,
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
include!("claude_code_structured_run/support.rs");
