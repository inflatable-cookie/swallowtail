use crate::{BoxFuture, RuntimeFailure};
use std::sync::atomic::{AtomicBool, Ordering};
use swallowtail_core::CancellationScope;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CancellationAcknowledgement {
    Requested,
    AlreadyRequested,
}

pub trait CancellationControl: Send + Sync {
    fn scope(&self) -> CancellationScope;
    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>>;
}

pub struct ImmediateCancellation {
    scope: CancellationScope,
    requested: AtomicBool,
}

impl ImmediateCancellation {
    #[must_use]
    pub const fn new(scope: CancellationScope) -> Self {
        Self {
            scope,
            requested: AtomicBool::new(false),
        }
    }

    #[must_use]
    pub fn is_requested(&self) -> bool {
        self.requested.load(Ordering::SeqCst)
    }
}

impl CancellationControl for ImmediateCancellation {
    fn scope(&self) -> CancellationScope {
        self.scope
    }

    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        let acknowledgement = if self.requested.swap(true, Ordering::SeqCst) {
            CancellationAcknowledgement::AlreadyRequested
        } else {
            CancellationAcknowledgement::Requested
        };
        Box::pin(async move { Ok(acknowledgement) })
    }
}
