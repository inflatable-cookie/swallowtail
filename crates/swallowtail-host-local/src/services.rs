use crate::{LocalProcessHost, LocalProcessHostBuilder, LocalScopedTaskService};
use std::sync::Arc;
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::HostServices;

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
            .with_schema(process_host.clone());
        Self {
            process_host,
            task_service,
            services,
        }
    }

    #[must_use]
    pub const fn services(&self) -> &HostServices {
        &self.services
    }

    #[must_use]
    pub const fn process_host(&self) -> &Arc<LocalProcessHost> {
        &self.process_host
    }

    #[must_use]
    pub const fn task_service(&self) -> &Arc<LocalScopedTaskService> {
        &self.task_service
    }
}

impl LocalProcessHostBuilder {
    /// Binds and composes the local host services under one exact host identity.
    #[must_use]
    pub fn build_services(mut self, execution_host_id: ExecutionHostId) -> LocalHostServices {
        self.execution_host_id = Some(execution_host_id.clone());
        LocalHostServices::compose(execution_host_id, self.build())
    }
}
