//! Local Contract 063 registered-tool profile over the shared bridge kernel.
//!
//! Slice 1 carries the host-mediated callback profile only: it binds no
//! listener, starts no Longhorn process or daemon, and adds no second lease
//! manager. Lease generation, per-turn ownership, operation-private authority
//! material, and bounded joined teardown come from the same
//! [`crate::operation_bridge`] kernel the watcher profile uses.

mod failure;
mod lease;

use crate::operation_bridge::{LeaseTable, generate_operation_secret, join_within};
use failure::{already_open_failure, closed_failure, foreign_failure, identity_failure};
use lease::LiveRegisteredLease;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use swallowtail_core::{ExecutionHostId, SafeDiagnostic};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, REGISTERED_TOOL_CLEANUP_BUDGET, RegisteredToolBridgeHostService,
    RegisteredToolBridgeLease, RegisteredToolBridgeToken, RegisteredToolCleanupCause,
    RegisteredToolCompletionState, RegisteredToolDispatcher, RegisteredToolLeaseGeneration,
    RegisteredToolOpenRequest, RegisteredToolOperationKernel, RegisteredToolTransport,
    RegisteredToolTransportGeneration, RuntimeFailure, ValidatedRegisteredToolBinding,
};

pub(crate) struct LocalRegisteredToolBridgeHostService {
    execution_host_id: ExecutionHostId,
    dispatcher: Arc<dyn RegisteredToolDispatcher>,
    cleanup_budget: Duration,
    state: Arc<Mutex<LeaseTable<LiveRegisteredLease>>>,
}

impl LocalRegisteredToolBridgeHostService {
    pub(crate) fn new(
        execution_host_id: ExecutionHostId,
        dispatcher: Arc<dyn RegisteredToolDispatcher>,
    ) -> Self {
        Self {
            execution_host_id,
            dispatcher,
            cleanup_budget: REGISTERED_TOOL_CLEANUP_BUDGET,
            state: Arc::new(Mutex::new(LeaseTable::default())),
        }
    }

    /// Replaces the bounded cleanup budget for deterministic fixtures.
    #[must_use]
    pub(crate) fn with_cleanup_budget(mut self, budget: Duration) -> Self {
        self.cleanup_budget = budget;
        self
    }

    /// Returns how many live leases this service still owns.
    pub(crate) fn live_lease_count(&self) -> usize {
        self.locked().live_count()
    }

    fn locked(&self) -> std::sync::MutexGuard<'_, LeaseTable<LiveRegisteredLease>> {
        self.state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    fn open_now(
        &self,
        request: RegisteredToolOpenRequest,
    ) -> Result<RegisteredToolBridgeLease, RuntimeFailure> {
        if request.execution_host_id() != &self.execution_host_id
            || request.selection().snapshot().execution_host_id() != &self.execution_host_id
        {
            return Err(foreign_failure());
        }
        if request.selection().transport() != RegisteredToolTransport::HostMediatedCallback {
            return Err(failure::unsupported_transport_failure());
        }
        let token_secret = generate_operation_secret()?;
        let generation = {
            let mut table = self.locked();
            let reserved = table
                .reserve(request.turn())
                .ok_or_else(already_open_failure)?;
            RegisteredToolLeaseGeneration::new(reserved).ok_or_else(identity_failure)?
        };
        let transport_generation = RegisteredToolTransportGeneration::initial();
        let turn = request.turn().clone();
        let opened = self.finish_open(request, generation, transport_generation, &token_secret);
        if opened.is_err() {
            // Any opening failure releases the partial resources it created.
            self.locked().forget(&turn, generation.get());
        }
        opened
    }

    fn finish_open(
        &self,
        request: RegisteredToolOpenRequest,
        generation: RegisteredToolLeaseGeneration,
        transport_generation: RegisteredToolTransportGeneration,
        token_secret: &str,
    ) -> Result<RegisteredToolBridgeLease, RuntimeFailure> {
        let host_token =
            RegisteredToolBridgeToken::new(token_secret).map_err(|_| identity_failure())?;
        let binding = ValidatedRegisteredToolBinding::for_open(
            &request,
            generation,
            transport_generation,
            &host_token,
        );
        let kernel = Arc::new(RegisteredToolOperationKernel::new(
            binding,
            request.selection().clone(),
            Arc::clone(&self.dispatcher),
        ));
        let live = Arc::new(LiveRegisteredLease {
            turn: request.turn().clone(),
            scope: request.scope().clone(),
            execution_host_id: self.execution_host_id.clone(),
            generation,
            token: host_token,
            kernel: Arc::clone(&kernel),
            closed: AtomicBool::new(false),
        });
        self.locked().insert(generation.get(), Arc::clone(&live));
        let close_state = Arc::clone(&self.state);
        let close_live = Arc::clone(&live);
        let budget = self.cleanup_budget;
        let lease_token =
            RegisteredToolBridgeToken::new(token_secret).map_err(|_| identity_failure())?;
        Ok(
            RegisteredToolBridgeLease::new(request, generation, transport_generation).bind(
                lease_token,
                kernel.call_channel(),
                move || {
                    // Drop is defensive cleanup, never success evidence.
                    let _ = shutdown(
                        &close_state,
                        &close_live,
                        RegisteredToolCleanupCause::Cancellation,
                        budget,
                    );
                },
            ),
        )
    }

    fn live_for(
        &self,
        lease: &RegisteredToolBridgeLease,
    ) -> Result<Arc<LiveRegisteredLease>, RuntimeFailure> {
        if lease.execution_host_id() != &self.execution_host_id {
            return Err(foreign_failure());
        }
        let live = self
            .locked()
            .get(lease.generation().get())
            .ok_or_else(closed_failure)?;
        live.matches(
            lease.execution_host_id(),
            lease.scope(),
            lease.turn(),
            lease.generation(),
        )?;
        if !lease.binding_matches(&live.token) {
            return Err(foreign_failure());
        }
        Ok(live)
    }
}

fn shutdown(
    state: &Arc<Mutex<LeaseTable<LiveRegisteredLease>>>,
    live: &Arc<LiveRegisteredLease>,
    cause: RegisteredToolCleanupCause,
    budget: Duration,
) -> Result<CleanupOutcome, RuntimeFailure> {
    let _ = cause;
    if live.closed.load(Ordering::SeqCst) || live.kernel.cleanup_failed() {
        return Ok(CleanupOutcome::NotApplicable);
    }
    live.kernel.freeze();
    live.kernel.begin_close();
    let joined = join_within(budget, || live.kernel.outstanding_calls() == 0);
    if !joined {
        // The lease stays frozen and its resources remain owned here. It never
        // reports a clean close and can never be inherited by a new attempt.
        live.kernel.record_cleanup_failed();
        return Ok(CleanupOutcome::Failed(SafeDiagnostic::new(
            "swallowtail.registered_tool.teardown_failed",
            "Registered tool teardown did not join within its bounded budget",
        )));
    }
    if live.closed.swap(true, Ordering::SeqCst) {
        return Ok(CleanupOutcome::NotApplicable);
    }
    live.kernel.record_closed();
    state
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .forget(&live.turn, live.generation.get());
    Ok(CleanupOutcome::Clean)
}

impl RegisteredToolBridgeHostService for LocalRegisteredToolBridgeHostService {
    fn open(
        &self,
        request: RegisteredToolOpenRequest,
    ) -> BoxFuture<'_, Result<RegisteredToolBridgeLease, RuntimeFailure>> {
        let result = self.open_now(request);
        Box::pin(async move { result })
    }

    fn completion_gate(
        &self,
        lease: &RegisteredToolBridgeLease,
    ) -> BoxFuture<'_, Result<RegisteredToolCompletionState, RuntimeFailure>> {
        let result = self
            .live_for(lease)
            .map(|live| live.kernel.observe_and_freeze_when_clear());
        Box::pin(async move { result })
    }

    fn close(
        &self,
        lease: RegisteredToolBridgeLease,
        cause: RegisteredToolCleanupCause,
    ) -> BoxFuture<'_, Result<CleanupOutcome, RuntimeFailure>> {
        let result = self
            .live_for(&lease)
            .and_then(|live| shutdown(&self.state, &live, cause, self.cleanup_budget));
        Box::pin(async move { result })
    }
}

#[cfg(test)]
mod tests;
