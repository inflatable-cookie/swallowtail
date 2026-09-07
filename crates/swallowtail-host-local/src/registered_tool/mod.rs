//! Local Contract 063 registered-tool profile over the shared bridge kernel.
//!
//! Slice 1 carries the host-mediated callback profile only: it binds no
//! listener, starts no Longhorn process or daemon, and owns no second lease
//! manager. Lease generation, per-turn ownership, live-lease registration, and
//! joined teardown come from the one shared [`crate::operation_bridge`]
//! registry that the watcher profile also uses.
//!
//! The mounted `open` applies the same typed readiness gate as `prepare`
//! against this host's published topology. Without a matching
//! `RegisteredToolTopologyProof` the kernel mints no binding and no lease, so a
//! direct low-level caller cannot bypass the gate.

mod failure;
mod lease;
mod proxy;
pub mod wire;

use crate::operation_bridge::{
    BridgeLease, BridgeLeaseOwner, BridgeProfile, OperationBridgeCleanupCause,
    OperationBridgeRegistry, join_within,
};
use failure::{already_open_failure, closed_failure, foreign_failure, not_ready_failure};
pub(crate) use lease::LiveRegisteredLease;
pub use proxy::RegisteredToolProxyLaunch;
use proxy::RegisteredToolProxyServer;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, OnceLock};
use std::time::Duration;
use swallowtail_core::{ExecutionHostId, SafeDiagnostic};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, REGISTERED_TOOL_CLEANUP_BUDGET, RegisteredToolBridgeHostService,
    RegisteredToolBridgeLease, RegisteredToolCleanupCause, RegisteredToolCompletionState,
    RegisteredToolDispatcher, RegisteredToolLeaseGeneration, RegisteredToolMountedTopology,
    RegisteredToolOpenRequest, RegisteredToolOperationKernel, RegisteredToolReadiness,
    RegisteredToolTransportGeneration, RuntimeFailure, TimeService,
};

pub(crate) struct LocalRegisteredToolBridgeHostService {
    execution_host_id: ExecutionHostId,
    dispatcher: Arc<dyn RegisteredToolDispatcher>,
    time: Arc<dyn TimeService>,
    cleanup_budget: Duration,
    registry: Arc<OperationBridgeRegistry>,
    topology: OnceLock<RegisteredToolMountedTopology>,
}

/// Registered-profile teardown driven by the one shared registry.
struct RegisteredLeaseOwner {
    registry: Arc<OperationBridgeRegistry>,
    live: Arc<LiveRegisteredLease>,
    budget: Duration,
}

impl BridgeLeaseOwner for RegisteredLeaseOwner {
    fn freeze(&self) {
        self.live.kernel.freeze();
    }

    fn join_and_release(
        &self,
        cause: OperationBridgeCleanupCause,
    ) -> Result<CleanupOutcome, RuntimeFailure> {
        shutdown(
            &self.registry,
            &self.live,
            registered_cause(cause),
            self.budget,
        )
    }
}

const fn registered_cause(cause: OperationBridgeCleanupCause) -> RegisteredToolCleanupCause {
    match cause {
        OperationBridgeCleanupCause::Completion => RegisteredToolCleanupCause::Completion,
        OperationBridgeCleanupCause::Cancellation => RegisteredToolCleanupCause::Cancellation,
        OperationBridgeCleanupCause::Deadline => RegisteredToolCleanupCause::Deadline,
        OperationBridgeCleanupCause::ProviderFailure => RegisteredToolCleanupCause::ProviderFailure,
        OperationBridgeCleanupCause::TransportFailure => {
            RegisteredToolCleanupCause::TransportFailure
        }
        OperationBridgeCleanupCause::ExplicitClose => RegisteredToolCleanupCause::ExplicitClose,
    }
}

impl LocalRegisteredToolBridgeHostService {
    pub(crate) fn new(
        execution_host_id: ExecutionHostId,
        dispatcher: Arc<dyn RegisteredToolDispatcher>,
        time: Arc<dyn TimeService>,
        registry: Arc<OperationBridgeRegistry>,
    ) -> Self {
        Self {
            execution_host_id,
            dispatcher,
            time,
            cleanup_budget: REGISTERED_TOOL_CLEANUP_BUDGET,
            registry,
            topology: OnceLock::new(),
        }
    }

    /// Replaces the bounded cleanup budget for deterministic fixtures.
    #[must_use]
    pub(crate) fn with_cleanup_budget(mut self, budget: Duration) -> Self {
        self.cleanup_budget = budget;
        self
    }

    /// Publishes the exact assembled topology this port validates against.
    ///
    /// Until the composition publishes it, `open` fails typed rather than
    /// admitting an unmeasured selection.
    pub(crate) fn publish_topology(&self, topology: RegisteredToolMountedTopology) {
        let _ = self.topology.set(topology);
    }

    /// Returns how many live registered-tool leases the shared registry owns.
    pub(crate) fn live_lease_count(&self) -> usize {
        self.registry.lease_count_for(BridgeProfile::RegisteredTool)
    }

    fn open_now(
        &self,
        request: RegisteredToolOpenRequest,
    ) -> Result<RegisteredToolBridgeLease, RuntimeFailure> {
        if request.execution_host_id() != &self.execution_host_id {
            return Err(foreign_failure());
        }
        // The mounted low-level open applies the same typed gate as prepare.
        let topology = self.topology.get().ok_or_else(not_ready_failure)?;
        let readiness = RegisteredToolReadiness::for_topology(topology, request.selection());
        let proof = readiness.require_ready()?;

        let reserved = self
            .registry
            .reserve(BridgeProfile::RegisteredTool, request.turn())
            .ok_or_else(already_open_failure)?;
        let turn = request.turn().clone();
        let opened = match RegisteredToolLeaseGeneration::new(reserved) {
            Some(generation) => self.finish_open(request, &proof, generation),
            None => Err(failure::identity_failure()),
        };
        if opened.is_err() {
            // Any opening failure releases the partial resources it created.
            self.registry
                .forget(BridgeProfile::RegisteredTool, &turn, reserved);
        }
        opened
    }

    fn finish_open(
        &self,
        request: RegisteredToolOpenRequest,
        proof: &swallowtail_runtime::RegisteredToolTopologyProof,
        generation: RegisteredToolLeaseGeneration,
    ) -> Result<RegisteredToolBridgeLease, RuntimeFailure> {
        let turn = request.turn().clone();
        let scope = request.scope().clone();
        let mediated_stdio = request.selection().attachment()
            == swallowtail_runtime::RegisteredToolAttachment::MediatedStdioProxy;
        let proxy_selection = request.selection().clone();
        // Only the kernel mints the binding and the lease, and only with a
        // proof issued for this exact selection.
        let (kernel, lease) = RegisteredToolOperationKernel::open(
            request,
            proof,
            Arc::clone(&self.dispatcher),
            Arc::clone(&self.time),
            generation,
            RegisteredToolTransportGeneration::initial(),
        )?;
        let proxy = if mediated_stdio {
            Some(Arc::new(RegisteredToolProxyServer::bind(
                Arc::clone(&kernel),
                proxy_selection,
                Arc::clone(&self.time),
                lease.deadline(),
            )?))
        } else {
            None
        };
        let live = Arc::new(LiveRegisteredLease {
            turn: turn.clone(),
            scope,
            execution_host_id: self.execution_host_id.clone(),
            generation,
            kernel: Arc::clone(&kernel),
            proxy,
            closed: AtomicBool::new(false),
        });
        self.registry.attach(
            generation.get(),
            BridgeProfile::RegisteredTool,
            turn,
            BridgeLease::RegisteredTool(Arc::clone(&live)),
            Arc::new(RegisteredLeaseOwner {
                registry: Arc::clone(&self.registry),
                live: Arc::clone(&live),
                budget: self.cleanup_budget,
            }),
        );
        let close_registry = Arc::clone(&self.registry);
        let close_live = Arc::clone(&live);
        let budget = self.cleanup_budget;
        Ok(lease.with_release(move || {
            // Drop is defensive cleanup, never success evidence.
            let _ = shutdown(
                &close_registry,
                &close_live,
                RegisteredToolCleanupCause::Cancellation,
                budget,
            );
        }))
    }

    fn live_for(
        &self,
        lease: &RegisteredToolBridgeLease,
    ) -> Result<Arc<LiveRegisteredLease>, RuntimeFailure> {
        if lease.execution_host_id() != &self.execution_host_id {
            return Err(foreign_failure());
        }
        let live = self
            .registry
            .registered_lease(lease.generation().get())
            .ok_or_else(closed_failure)?;
        live.matches(
            lease.execution_host_id(),
            lease.scope(),
            lease.turn(),
            lease.generation(),
        )?;
        // The lease authenticates by kernel identity: it can only have come
        // from the kernel this registry owns for that generation.
        if !lease.is_bound_to(&live.kernel) {
            return Err(foreign_failure());
        }
        Ok(live)
    }

    pub(crate) fn proxy_launch(
        &self,
        lease: &RegisteredToolBridgeLease,
    ) -> Result<RegisteredToolProxyLaunch, RuntimeFailure> {
        let live = self.live_for(lease)?;
        let proxy = live.proxy.as_ref().ok_or_else(|| {
            RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                "swallowtail.registered_tool.proxy_unavailable",
                "This registered-tool lease does not select mediated stdio",
            ))
        })?;
        let recipe = lease.selection().proxy_recipe().ok_or_else(|| {
            RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                "swallowtail.registered_tool.process_recipe_unavailable",
                "The mediated stdio lease has no resolved courier recipe",
            ))
        })?;
        let rendezvous = proxy.rendezvous(lease)?;
        let request = swallowtail_runtime::ProcessRequest::new(recipe.executable().clone())
            .with_arguments(rendezvous.courier_arguments())
            .with_environment([recipe.environment().clone()]);
        Ok(RegisteredToolProxyLaunch::new(
            request,
            rendezvous,
            Arc::clone(proxy),
        ))
    }
}

fn shutdown(
    registry: &Arc<OperationBridgeRegistry>,
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
        // The lease stays frozen and its resources remain owned by the shared
        // registry. It never reports a clean close and can never be inherited
        // by a new attempt.
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
    if let Some(proxy) = live.proxy.as_ref() {
        proxy.close();
    }
    registry.forget(
        BridgeProfile::RegisteredTool,
        &live.turn,
        live.generation.get(),
    );
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
            .and_then(|live| shutdown(&self.registry, &live, cause, self.cleanup_budget));
        Box::pin(async move { result })
    }
}

#[cfg(test)]
mod tests;
