use super::DeepSeekDirectDriver;
use super::access::AccessLeases;
use super::catalogue::{operation_scope, require_services};
use super::lifecycle::{cleanup_result, merge_cleanup};
use crate::DeepSeekThinkingMode;
use crate::failure::{failure, protocol, unsupported};
use crate::protocol::{
    FinalStreamParser, FinalStreamUpdate, HttpRequest, Usage, encode_structured,
};
use crate::selection::{
    DEEPSEEK_MODEL_ID, deepseek_plan_supports_reasoning, deepseek_reasoning_mode_is_supported,
    deepseek_v4_config,
};
use crate::transport::{StreamItem, Subscription};
use std::collections::BTreeMap;
use std::future::poll_fn;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::Poll;
use swallowtail_core::{
    CancellationScope, Capability, ExternalNetworkPolicy, ExternalSearchPolicy, PreflightPlan,
    ProviderRequestRef, RunRef, SafeDiagnostic,
};
use swallowtail_runtime::{
    BoxEventStream, BoxFuture, CancellationAcknowledgement, CancellationControl, CleanupOutcome,
    DeadlineObservation, DebugObservationKind, HostServices, JoinedTask, OperationContent,
    ProviderExecutionPolicy, ProviderObservation, ProviderRecoveryPolicy, ProviderRetentionPolicy,
    RequestId, RunHandle, RuntimeEvent, RuntimeEventKind, RuntimeFailure, RuntimeRunId,
    StreamReattachmentPolicy, StructuredRunDriver, StructuredRunRequest, TerminalOutcome,
    TerminalStatus, TokenUsage, runtime_event_channel, terminal_outcome_channel,
};

const EVENT_CAPACITY: usize = 32;
const ROUTE: &str = "deepseek.continuation";

include!("run/start.rs");
include!("run/handle.rs");
include!("run/pump.rs");
