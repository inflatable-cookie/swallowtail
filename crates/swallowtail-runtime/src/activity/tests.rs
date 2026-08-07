use super::{
    ActivityActor, ActivityAssistantPhase, ActivityContent, ActivityContentChangeKind,
    ActivityContentStream, ActivityContentUpdate, ActivityCorrelation, ActivityDisclosure,
    ActivityId, ActivityKey, ActivityKind, ActivityLabel, ActivityLifecyclePhase,
    ActivityNamespace, ActivityObservation, ActivityOperationId, ActivityStatus,
    SubagentControlActionKind, SubagentId, SubagentParent, SubagentSnapshot, SubagentStatus,
    TaskListItem, TaskListItemPriority, TaskListItemStatus, TaskListSnapshot,
};
use crate::{
    CallbackId, DirectToolCallId, EventDelivery, OperationContent, RuntimeEventKind, RuntimeRunId,
};
use swallowtail_core::{
    FailureClassification, FailureKind, FailureOrigin, FailureRecovery, ProviderActivityRef,
    ProviderRequestRef, SafeDiagnostic,
};

fn run_id() -> ActivityOperationId {
    ActivityOperationId::Run(RuntimeRunId::new("private-run").expect("run id is valid"))
}

include!("tests/identity.rs");
include!("tests/task_list.rs");
include!("tests/subagent.rs");
include!("tests/streams.rs");
include!("tests/failure.rs");
