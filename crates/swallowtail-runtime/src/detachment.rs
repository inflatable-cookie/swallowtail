use crate::{BoxFuture, RuntimeFailure};
use swallowtail_core::OperationDetachmentScope;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OperationDetachmentAcknowledgement {
    Requested,
    AlreadyRequested,
}

/// Optional local attachment disposition for provider work which can outlive
/// one Swallowtail handle.
///
/// Acknowledgement does not establish provider completion, continued activity,
/// failure, or cancellation. The consuming handle close still owns joined
/// local cleanup.
pub trait OperationDetachmentControl: Send + Sync {
    fn scope(&self) -> OperationDetachmentScope;

    fn request(&self) -> BoxFuture<'_, Result<OperationDetachmentAcknowledgement, RuntimeFailure>>;
}
