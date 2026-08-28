use crate::watcher::LocalWatcherHostService;
use crate::{LocalProcessHost, LocalProcessHostBuilder, LocalScopedTaskService};
use std::sync::Arc;
use std::time::Duration;
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{Deadline, HostServices, MonotonicInstant, TimeService};

/// Inspectable host-owned local service composition for one execution host.
#[derive(Clone)]
pub struct LocalHostServices {
    process_host: Arc<LocalProcessHost>,
    task_service: Arc<LocalScopedTaskService>,
    services: HostServices,
}

impl LocalHostServices {
    pub(crate) fn compose(
        execution_host_id: ExecutionHostId,
        process_host: LocalProcessHost,
    ) -> Self {
        let process_host = Arc::new(process_host);
        let task_service = Arc::new(LocalScopedTaskService::new(execution_host_id.clone()));
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
            .with_watcher(Arc::new(LocalWatcherHostService::new(
                process_host.clone(),
                task_service.clone(),
                process_host.watcher_capacity,
            )));
        Self {
            process_host,
            task_service,
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
