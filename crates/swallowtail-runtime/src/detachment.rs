use crate::{BoxFuture, RuntimeFailure};
use swallowtail_core::OperationDetachmentScope;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Immediate acknowledgement of a local detachment request.
pub enum OperationDetachmentAcknowledgement {
    /// This call recorded the first detachment request.
    Requested,
    /// Detachment had already been requested.
    AlreadyRequested,
}

/// Optional local attachment disposition for provider work which can outlive
/// one Swallowtail handle.
///
/// Acknowledgement does not establish provider completion, continued activity,
/// failure, or cancellation. The consuming handle close still owns joined
/// local cleanup.
pub trait OperationDetachmentControl: Send + Sync {
    /// Returns the provider-work shape to which detachment applies.
    fn scope(&self) -> OperationDetachmentScope;

    /// Requests local observer detachment without claiming provider state.
    fn request(&self) -> BoxFuture<'_, Result<OperationDetachmentAcknowledgement, RuntimeFailure>>;
}
