//! One live registered-tool lease owned by the local host.

use super::failure::{closed_failure, foreign_failure};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{
    RegisteredToolBridgeToken, RegisteredToolLeaseGeneration, RegisteredToolOperationKernel,
    RuntimeFailure, RuntimeTurnId, ScopeId,
};

pub(super) struct LiveRegisteredLease {
    pub(super) turn: RuntimeTurnId,
    pub(super) scope: ScopeId,
    pub(super) execution_host_id: ExecutionHostId,
    pub(super) generation: RegisteredToolLeaseGeneration,
    pub(super) token: RegisteredToolBridgeToken,
    pub(super) kernel: Arc<RegisteredToolOperationKernel>,
    pub(super) closed: AtomicBool,
}

impl LiveRegisteredLease {
    pub(super) fn matches(
        &self,
        host: &ExecutionHostId,
        scope: &ScopeId,
        turn: &RuntimeTurnId,
        generation: RegisteredToolLeaseGeneration,
    ) -> Result<(), RuntimeFailure> {
        if self.closed.load(Ordering::SeqCst) {
            return Err(closed_failure());
        }
        if &self.execution_host_id != host
            || &self.scope != scope
            || &self.turn != turn
            || self.generation != generation
        {
            return Err(foreign_failure());
        }
        Ok(())
    }
}
