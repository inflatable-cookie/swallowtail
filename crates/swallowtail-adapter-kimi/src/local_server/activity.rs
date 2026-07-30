use crate::failure::failure;
use crate::local_server::protocol::{TurnEndReason, WsEvent};
use std::collections::BTreeMap;
use swallowtail_core::{ActivityContentStream, ActivityDisclosure, ProviderActivityRef};
use swallowtail_runtime::{
    ActivityAssistantPhase, ActivityContent, ActivityContentChangeKind, ActivityContentUpdate,
    ActivityId, ActivityKind, ActivityLabel, ActivityLifecyclePhase, ActivityNamespace,
    ActivityObservation, ActivityOperationId, ActivityStatus, OperationContent, RuntimeFailure,
    RuntimeTurnId,
};

pub(super) mod profile;

const MAXIMUM_ACTIVITY_CONTENT_BYTES: usize = 64 * 1024;

pub(super) struct KimiLocalActivityProjection {
    operation_id: ActivityOperationId,
    assistant: Option<OpenActivity>,
    reasoning: Option<OpenActivity>,
    steps: BTreeMap<String, OpenActivity>,
    tools: BTreeMap<String, OpenActivity>,
    commands: BTreeMap<String, OpenActivity>,
    subagents: BTreeMap<String, OpenActivity>,
    tasks: BTreeMap<String, OpenActivity>,
    compaction: Option<OpenActivity>,
    next_id: u64,
}

#[derive(Clone)]
struct OpenActivity {
    id: ActivityId,
    provider_ref: Option<ProviderActivityRef>,
    kind: ActivityKind,
    assistant_phase: Option<ActivityAssistantPhase>,
    disclosure: ActivityDisclosure,
    label: Option<ActivityLabel>,
}

include!("activity/projection.rs");

#[derive(Clone, Copy)]
enum ActivityBucket {
    Step,
    Tool,
    Command,
    Subagent,
    Task,
}

impl ActivityBucket {
    const fn label(self) -> &'static str {
        match self {
            Self::Step => "step",
            Self::Tool => "tool",
            Self::Command => "command",
            Self::Subagent => "subagent",
            Self::Task => "task",
        }
    }
}

fn key(turn_id: &u64, step: u64) -> String {
    format!("{turn_id}:{step}")
}

fn terminal_status(failed: bool) -> ActivityStatus {
    if failed {
        ActivityStatus::Failed
    } else {
        ActivityStatus::Completed
    }
}

fn namespace(value: &str) -> Result<ActivityNamespace, RuntimeFailure> {
    ActivityNamespace::new(value).map_err(|_| activity_drift())
}

fn activity_label(value: &str) -> Option<ActivityLabel> {
    ActivityLabel::new(value.trim()).ok()
}

fn content(
    value: &str,
    change: ActivityContentChangeKind,
    stream: ActivityContentStream,
) -> Result<ActivityContentUpdate, RuntimeFailure> {
    let content = OperationContent::new(value.to_owned()).map_err(|_| activity_drift())?;
    let content = ActivityContent::new(content, MAXIMUM_ACTIVITY_CONTENT_BYTES)
        .map_err(|_| activity_drift())?;
    Ok(ActivityContentUpdate::new(change, stream, content))
}

fn activity_drift() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.activity_invalid",
        "Kimi local-server activity did not match the qualified protocol",
    )
}

#[cfg(test)]
mod tests;
