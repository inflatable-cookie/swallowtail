use super::AlibabaModelStudioDriver;
use super::access::AccessLeases;
use super::lifecycle::{cleanup_result, merge_cleanup};
use crate::failure::{failure, protocol, unsupported};
use crate::protocol::{ProviderEvent, ResponseStream, WireRequest};
use crate::transport::{StreamItem, Subscription};
use std::future::poll_fn;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::Poll;
use swallowtail_core::{
    CancellationScope, Capability, ExternalNetworkPolicy, ExternalSearchPolicy, PreflightPlan,
    RunRef, SafeDiagnostic,
};
use swallowtail_runtime::{
    BoxEventStream, BoxFuture, CancellationAcknowledgement, CancellationControl, CleanupOutcome,
    DeadlineObservation, DebugObservationKind, HostServices, JoinedTask, ProviderExecutionPolicy,
    ProviderObservation, ProviderRecoveryPolicy, ProviderRetentionPolicy, RequestId, RunHandle,
    RuntimeEvent, RuntimeEventKind, RuntimeFailure, RuntimeRunId, ScopeId,
    StreamReattachmentPolicy, StructuredRunDriver, StructuredRunRequest, TerminalOutcome,
    TerminalStatus, runtime_event_channel, terminal_outcome_channel,
};

const EVENT_CAPACITY: usize = 32;
const ROUTE: &str = "alibaba.conversations";

include!("run/start.rs");
include!("run/handle.rs");
include!("run/pump.rs");
