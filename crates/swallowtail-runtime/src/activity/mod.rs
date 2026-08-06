#![deny(missing_docs)]

mod content;
mod error;
mod identity;
mod label;
mod lifecycle;
mod record;
mod subagent;
mod task_list;
mod validation;

pub use content::{
    ActivityContent, ActivityContentChangeKind, ActivityContentStream, ActivityContentUpdate,
};
pub use error::InvalidActivityRecord;
pub use identity::{ActivityId, ActivityNamespace};
pub use label::ActivityLabel;
pub(crate) use lifecycle::{ActivityLifecycleTracker, ActivityTransitionFailure};
pub use record::{
    ActivityAssistantPhase, ActivityCorrelation, ActivityKey, ActivityKind, ActivityLifecyclePhase,
    ActivityObservation, ActivityOperationId, ActivityStatus,
};
pub use subagent::{ActivityActor, SubagentId, SubagentParent, SubagentSnapshot, SubagentStatus};
pub use swallowtail_core::ActivityDisclosure;
pub use swallowtail_core::SubagentControlActionKind;
pub use task_list::{TaskListItem, TaskListItemPriority, TaskListItemStatus, TaskListSnapshot};

#[cfg(test)]
mod tests;
