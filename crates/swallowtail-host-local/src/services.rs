use crate::operation_bridge::{OperationBridgeCleanupCause, OperationBridgeRegistry};
use crate::registered_tool::{LocalRegisteredToolBridgeHostService, RegisteredToolProxyLaunch};
use crate::task::LocalTaskReaperOwner;
use crate::watcher::LocalWatcherHostService;
use crate::watcher_bridge::{LocalWatcherBridgeHostService, WatcherBridgeProofKind};
use crate::{LocalProcessHost, LocalProcessHostBuilder, LocalScopedTaskService};
use std::ffi::OsString;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{
    CleanupOutcome, Deadline, EnvironmentRef, ExecutableRef, HostServices, MonotonicInstant,
    RegisteredToolBridgeLease, RegisteredToolMountedTopology, RuntimeFailure, RuntimeTurnId,
    TimeService,
};

/// Inspectable host-owned local service composition for one execution host.
///
/// Default composition registers the portable watcher service against the
/// ordinary host-approved process service. Process-backed starts bind an owned
/// process handle and its process-group cleanup before returning watcher
/// identity. Unapproved operation data still rejects before work.
#[derive(Clone)]
pub struct LocalHostServices {
    process_host: Arc<LocalProcessHost>,
    task_service: Arc<LocalScopedTaskService>,
    task_reaper_owner: LocalTaskReaperOwner,
    watcher_bridge: Arc<LocalWatcherBridgeHostService>,
    registered_tool_bridge: Option<Arc<LocalRegisteredToolBridgeHostService>>,
    operation_bridges: Arc<OperationBridgeRegistry>,
    services: HostServices,
}

impl LocalHostServices {
    pub(crate) fn compose(
        execution_host_id: ExecutionHostId,
        process_host: LocalProcessHost,
    ) -> Self {
        let process_host = Arc::new(process_host);
        let (task_service, task_reaper_owner) = LocalScopedTaskService::with_reaper_owner(
            execution_host_id.clone(),
            process_host.task_reap_capacity,
        );
        let task_service = Arc::new(task_service);
        let watcher = Arc::new(LocalWatcherHostService::new(
            process_host.clone(),
            task_service.clone(),
            process_host.watcher_capacity,
        ));
        // One shared lease, generation, and lifecycle owner for every profile.
        let operation_bridges = Arc::new(OperationBridgeRegistry::default());
        let watcher_bridge = Arc::new(LocalWatcherBridgeHostService::new(
            execution_host_id.clone(),
            watcher.clone(),
            process_host.clone(),
            Arc::clone(&operation_bridges),
        ));
        let registered_tool_bridge =
            process_host
                .registered_tool_dispatcher
                .clone()
                .map(|dispatcher| {
                    Arc::new(
                        LocalRegisteredToolBridgeHostService::new(
                            execution_host_id.clone(),
                            dispatcher,
                            process_host
                                .registered_tool_clock
                                .clone()
                                .unwrap_or_else(|| process_host.clone()),
                            Arc::clone(&operation_bridges),
                        )
                        .with_cleanup_budget(process_host.registered_tool_cleanup_budget),
                    )
                });
        let services = HostServices::new(execution_host_id)
            .with_task(task_service.clone())
            .with_time(process_host.clone())
            .with_process(process_host.clone())
            .with_network(process_host.clone())
            .with_credential(process_host.clone())
            .with_working_resource(process_host.clone())
            .with_working_resource_io(process_host.clone())
            .with_attachment(process_host.clone())
            .with_model_artifact(process_host.clone())
            .with_serving_endpoint(process_host.clone())
            .with_schema(process_host.clone())
            .with_watcher(watcher)
            .with_watcher_bridge(watcher_bridge.clone());
        let services = match registered_tool_bridge.as_ref() {
            Some(bridge) => services.with_registered_tool_bridge(bridge.clone()),
            None => services,
        };
        if let Some(bridge) = registered_tool_bridge.as_ref() {
            // The mounted port measures every low-level open against the exact
            // topology this composition assembled.
            bridge.publish_topology(RegisteredToolMountedTopology::from_hosts(&services));
        }
        Self {
            process_host,
            task_service,
            task_reaper_owner,
            watcher_bridge,
            registered_tool_bridge,
            operation_bridges,
            services,
        }
    }

    /// Returns the complete provider-neutral host service registry.
    #[must_use]
    pub const fn services(&self) -> &HostServices {
        &self.services
    }

    /// Returns the local process and materialization host.
    #[must_use]
    pub const fn process_host(&self) -> &Arc<LocalProcessHost> {
        &self.process_host
    }

    /// Returns the scoped local task service.
    #[must_use]
    pub const fn task_service(&self) -> &Arc<LocalScopedTaskService> {
        &self.task_service
    }

    /// Stops reap reservations, settles accepted tasks, and joins their reapers.
    ///
    /// The outer selected-host lifecycle must call this after operation work
    /// and outside the task tree. It does not cancel issued or accepted work
    /// and may wait for it to settle. Calling it again is harmless.
    pub fn shutdown_task_reapers(&self) -> Result<(), swallowtail_runtime::RuntimeFailure> {
        self.task_reaper_owner.shutdown()
    }

    /// Returns reserved watcher-bridge operations observed for one turn.
    ///
    /// Names and order only. Endpoint, bearer, arguments, and response text
    /// are not retained. Facts from another turn or a prior lease generation
    /// are not included.
    #[must_use]
    pub fn watcher_bridge_proof(&self, turn: &RuntimeTurnId) -> Vec<WatcherBridgeProofKind> {
        self.watcher_bridge.proof_facts(turn)
    }

    /// Returns how many registered-tool leases this composition still owns.
    ///
    /// A failed cleanup retains its lease here; it is never detached and never
    /// inherited by a later attempt.
    #[must_use]
    pub fn registered_tool_lease_count(&self) -> usize {
        self.registered_tool_bridge
            .as_ref()
            .map_or(0, |bridge| bridge.live_lease_count())
    }

    /// Returns the approved native program path for one executable reference.
    #[must_use]
    pub fn approved_executable_path(&self, reference: &ExecutableRef) -> Option<&Path> {
        self.process_host.approved_executable_path(reference)
    }

    /// Returns the approved environment bindings for one environment reference.
    #[must_use]
    pub fn approved_environment(
        &self,
        reference: &EnvironmentRef,
    ) -> Option<&[(OsString, OsString)]> {
        self.process_host.approved_environment(reference)
    }

    /// Materializes one operation-scoped mediated-stdio courier launch.
    ///
    /// The returned launch is one-shot. Its process arguments contain only the
    /// fixed wire tag and a non-authoritative rendezvous path; endpoint,
    /// bearer, and generations remain in the private rendezvous file.
    pub fn registered_tool_proxy_launch(
        &self,
        lease: &RegisteredToolBridgeLease,
    ) -> Result<RegisteredToolProxyLaunch, RuntimeFailure> {
        self.registered_tool_bridge
            .as_ref()
            .ok_or_else(|| {
                RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                    "swallowtail.registered_tool.missing_host_service",
                    "Registered tool bridge is not mounted",
                ))
            })?
            .proxy_launch(lease)
    }

    /// Returns how many operation-bridge leases both profiles share.
    ///
    /// There is one lease owner for the whole composition, so this count spans
    /// the watcher and registered-tool profiles.
    #[must_use]
    pub fn operation_bridge_lease_count(&self) -> usize {
        self.operation_bridges.lease_count()
    }

    /// Returns how many operation-owned loopback listeners are live.
    #[must_use]
    pub fn operation_bridge_listener_count(&self) -> usize {
        self.operation_bridges.listener_count()
    }

    /// Returns every profile and shared generation owned for one turn.
    ///
    /// Both profiles draw from one monotonic generation space, so a generation
    /// is never reused across profiles or attempts.
    #[must_use]
    pub fn operation_bridge_generations(&self, turn: &RuntimeTurnId) -> Vec<(&'static str, u64)> {
        self.operation_bridges
            .generations_for_turn(turn)
            .into_iter()
            .map(|(profile, generation)| (profile.as_str(), generation))
            .collect()
    }

    /// Freezes and joins every profile lease this operation owns for one turn.
    ///
    /// Admission freezes across both profiles first, then each lease joins under
    /// the one sequence. A failed join dominates the reported outcome and keeps
    /// its lease owned here.
    pub fn close_operation_bridges(
        &self,
        turn: &RuntimeTurnId,
        cause: OperationBridgeCleanupCause,
    ) -> Result<CleanupOutcome, RuntimeFailure> {
        self.operation_bridges.close_turn(turn, cause)
    }

    /// Derives one deadline from this composition's monotonic clock and an
    /// explicit caller-selected duration.
    #[must_use]
    pub fn deadline_after(&self, duration: Duration) -> Deadline {
        deadline_after(self.process_host.now(), duration)
    }
}

fn deadline_after(now: MonotonicInstant, duration: Duration) -> Deadline {
    let duration_ticks = u64::try_from(duration.as_nanos()).unwrap_or(u64::MAX);
    Deadline::at(MonotonicInstant::from_ticks(
        now.ticks().saturating_add(duration_ticks),
    ))
}

impl LocalProcessHostBuilder {
    /// Binds and composes the local host services under one exact host identity.
    #[must_use]
    pub fn build_services(mut self, execution_host_id: ExecutionHostId) -> LocalHostServices {
        self.execution_host_id = Some(execution_host_id.clone());
        LocalHostServices::compose(execution_host_id, self.build())
    }
}

#[cfg(test)]
mod tests {
    use super::deadline_after;
    use std::time::Duration;
    use swallowtail_runtime::MonotonicInstant;

    #[test]
    fn explicit_duration_uses_nanosecond_ticks() {
        let deadline = deadline_after(MonotonicInstant::from_ticks(10), Duration::from_nanos(25));

        assert_eq!(deadline.instant().ticks(), 35);
    }

    #[test]
    fn duration_conversion_and_instant_addition_saturate() {
        let oversized = deadline_after(
            MonotonicInstant::from_ticks(10),
            Duration::new(u64::MAX, 999_999_999),
        );
        let addition = deadline_after(
            MonotonicInstant::from_ticks(u64::MAX - 5),
            Duration::from_nanos(10),
        );

        assert_eq!(oversized.instant().ticks(), u64::MAX);
        assert_eq!(addition.instant().ticks(), u64::MAX);
    }
}
