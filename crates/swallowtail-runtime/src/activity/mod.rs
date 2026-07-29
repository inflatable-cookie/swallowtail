mod content;
mod error;
mod identity;
mod lifecycle;
mod record;
mod validation;

pub use content::{
    ActivityContent, ActivityContentChangeKind, ActivityContentStream, ActivityContentUpdate,
};
pub use error::InvalidActivityRecord;
pub use identity::{ActivityId, ActivityNamespace};
pub(crate) use lifecycle::{ActivityLifecycleTracker, ActivityTransitionFailure};
pub use record::{
    ActivityAssistantPhase, ActivityCorrelation, ActivityKind, ActivityLifecyclePhase,
    ActivityObservation, ActivityOperationId, ActivityStatus,
};
pub use swallowtail_core::ActivityDisclosure;

#[cfg(test)]
mod tests;
