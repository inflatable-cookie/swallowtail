//! One live registered-tool lease the shared operation-bridge registry owns.

use super::failure::{closed_failure, foreign_failure};
use super::proxy::RegisteredToolProxyServer;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{
    RegisteredToolLeaseGeneration, RegisteredToolOperationKernel, RuntimeFailure, RuntimeTurnId,
    ScopeId,
};

pub(crate) struct LiveRegisteredLease {
    pub(crate) turn: RuntimeTurnId,
    pub(crate) scope: ScopeId,
    pub(crate) execution_host_id: ExecutionHostId,
    pub(crate) generation: RegisteredToolLeaseGeneration,
    pub(crate) kernel: Arc<RegisteredToolOperationKernel>,
    pub(crate) proxy: Option<Arc<RegisteredToolProxyServer>>,
    pub(crate) closed: AtomicBool,
}

impl LiveRegisteredLease {
    pub(crate) fn matches(
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
